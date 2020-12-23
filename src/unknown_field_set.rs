//! Runtime library code for storing unknown fields.

use crate::encoding::{
    bytes as bytes1, decode_varint, encode_key, encode_varint, fixed32, fixed64,
    skip_field, uint64, DecodeContext, WireType,
};
use crate::DecodeError;
use bytes::{Buf, BufMut};
use std::collections::BTreeMap;
#[cfg(feature = "sq")]
use serde::{Deserialize, Serialize};

/// A set of Protobuf fields that were not recognized during decoding.
///
/// Every Message struct should have an UnknownFieldSet member. This is how
/// messages make sure to not discard unknown data in a decode/encode cycle,
/// which is required by the Protobuf spec.
#[derive(Clone, Debug, PartialEq, Eq, Default, PartialOrd, Ord, Deserialize, Serialize)]
pub struct UnknownFieldSet {
    // The actual data of this struct is wrapped in a Box to ensure that
    // this struct uses only one machine word of memory unless there are
    // unknown fields to store.
    //
    // If the Option is non-empty, the BTreeMap is also non-empty.
    pub data: Option<Box<BTreeMap<u32, UnknownField>>>,
}

impl UnknownFieldSet {
    /// Adds a field to the UnknownFieldSet. Takes the tag, the wire type and
    /// a buffer that points to where the field itself (excluding the key is).
    ///
    /// Mutates the provided buffer to point to after the unknown field ends.
    #[doc(hidden)] // Not for external use.
    pub fn skip_unknown_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        _ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        UnknownField::parse(tag, wire_type, buf, _ctx)
            .map_or(Ok(()), |res| Ok(self.insert(tag, res?)))
    }

    fn insert(&mut self, tag: u32, field: UnknownField) {
        match self.data {
            Some(ref mut m) => {
                m.insert(tag, field);
            }
            None => {
                let mut m = BTreeMap::new();
                m.insert(tag, field);
                self.data = Some(Box::new(m));
            }
        }
    }

    #[doc(hidden)] // Not for external use.
    pub fn encode<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        match self.data {
            Some(ref map) => {
                for (_, field) in map.iter() {
                    field.encode(buf);
                }
            }
            None => {}
        }
    }

    #[doc(hidden)] // Not for external use.
    pub fn encoded_len(&self) -> usize {
        match self.data {
            Some(ref map) => map
                .iter()
                .fold(0, |len, (_, field)| len + field.encoded_len()),
            None => 0,
        }
    }
}

//impl Hash

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct UnknownField {
    pub tag: u32,
    pub data: UnknownFieldData,
}

impl UnknownField {
    /// Parses an unknown field. Takes the tag, the wire type and a buffer that
    /// points to where the field itself (excluding the key is). Returns the
    /// parsed UnknownField.
    fn parse<B>(
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        _ctx: DecodeContext,
    ) -> Option<Result<UnknownField, DecodeError>>
    where
        B: Buf,
    {
        let f = |wire_type: WireType| {
            match wire_type {
                WireType::Varint => Ok(Some(
                    decode_varint(buf).map(|val| UnknownFieldData::Varint(val))?,
                )),
                WireType::ThirtyTwoBit => {
                    if buf.remaining() < 4 {
                        return Err(DecodeError::new("buffer underflow"));
                    }
                    Ok(Some(UnknownFieldData::ThirtyTwoBit(buf.get_u32_le())))
                }
                WireType::SixtyFourBit => {
                    if buf.remaining() < 8 {
                        return Err(DecodeError::new("buffer underflow"));
                    }
                    Ok(Some(UnknownFieldData::SixtyFourBit(buf.get_u64_le())))
                }
                WireType::LengthDelimited => {
                    let mut field_buf = Vec::new();
                    crate::encoding::bytes::merge(wire_type, &mut field_buf, buf, _ctx)?;
                    Ok(Some(UnknownFieldData::LengthDelimited(field_buf)))
                }
                WireType::StartGroup => {
                    //TODO(amilkov3) skipping groups for now
                    skip_field(WireType::StartGroup, tag, buf, _ctx)?;
                    Ok(None)
                }
                WireType::EndGroup => return Err(DecodeError::new("unexpected end group tag")),
            }
        };
        f(wire_type).map_or_else(
            |e| Some(Err(e)),
            |opt| opt.map(|data| Ok(UnknownField { tag, data })),
        )
    }

    fn encode<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        match &self.data {
            UnknownFieldData::Varint(value) => {
                encode_key(self.tag, WireType::Varint, buf);
                encode_varint(*value, buf);
            }
            UnknownFieldData::SixtyFourBit(value) => {
                encode_key(self.tag, WireType::SixtyFourBit, buf);
                buf.put_u64_le(*value);
            }
            UnknownFieldData::LengthDelimited(value) => {
                encode_key(self.tag, WireType::LengthDelimited, buf);
                encode_varint(value.len() as u64, buf);
                buf.put_slice(value);
            }
            UnknownFieldData::ThirtyTwoBit(value) => {
                encode_key(self.tag, WireType::ThirtyTwoBit, buf);
                buf.put_u32_le(*value);
            }
        }
    }

    fn encoded_len(&self) -> usize {
        match &self.data {
            UnknownFieldData::Varint(value) => uint64::encoded_len(self.tag, value),
            UnknownFieldData::SixtyFourBit(value) => fixed64::encoded_len(self.tag, value),
            UnknownFieldData::LengthDelimited(value) => bytes1::encoded_len(self.tag, value),
            UnknownFieldData::ThirtyTwoBit(value) => fixed32::encoded_len(self.tag, value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum UnknownFieldData {
    Varint(u64),
    SixtyFourBit(u64),
    LengthDelimited(Vec<u8>),
    ThirtyTwoBit(u32),
}
