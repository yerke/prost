//! Protocol Buffers well-known wrapper types.
//!
//! This module provides implementations of `Message` for Rust standard library types which
//! correspond to a Protobuf well-known wrapper type. The remaining well-known types are defined in
//! the `prost-types` crate in order to avoid a cyclic dependency between `prost` and
//! `prost-build`.

use alloc::string::String;
use alloc::vec::Vec;

use ::bytes::{Buf, BufMut, Bytes};

use crate::{
    encoding::{
        bool, bytes, double, float, int32, int64, skip_field, string, uint32, uint64,
        DecodeContext, WireType,
    },
    DecodeError, Message, UnknownField, UnknownFieldData,
};

/// `google.protobuf.BoolValue`
impl Message for bool {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self {
            bool::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            bool::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self {
            2
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = false;
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        match f.data {
            UnknownFieldData::Varint(b) => Ok(b != 0),
            ref ufd => Err(DecodeError::new(format!(
                "cannot decode bool from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.UInt32Value`
impl Message for u32 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            uint32::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            uint32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            uint32::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        match f.data {
            UnknownFieldData::Varint(u) => Ok(u as u32),
            UnknownFieldData::ThirtyTwoBit(u) => Ok(u),
            ref ufd => Err(DecodeError::new(format!(
                "cannot decode u32 from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.UInt64Value`
impl Message for u64 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            uint64::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            uint64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            uint64::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        match f.data {
            UnknownFieldData::SixtyFourBit(u) | UnknownFieldData::Varint(u) => Ok(u),
            ref ufd => Err(DecodeError::new(format!(
                "cannot decode u64 from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.Int32Value`
impl Message for i32 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            int32::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            int32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            int32::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        println!("i32 tag: {}, data: {:?}", f.tag, f.data);
        match f.data {
            UnknownFieldData::ThirtyTwoBit(u) => Ok(u as i32),
            UnknownFieldData::Varint(u) => Ok(u as i32),
            ref ufd => Err(DecodeError::new(format!(
                "cannot decode i32 from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.Int64Value`
impl Message for i64 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0 {
            int64::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            int64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            int64::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0;
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        println!("i64 tag: {}, data: {:?}", f.tag, f.data);
        match f.data {
            UnknownFieldData::SixtyFourBit(u) => Ok(u as i64),
            UnknownFieldData::Varint(u) => Ok(u as i64),
            ref ufd => Err(DecodeError::new(format!(
                "cannot decode i64 from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.FloatValue`
impl Message for f32 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0.0 {
            float::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            float::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0.0 {
            float::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0.0;
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        match f.data {
            UnknownFieldData::ThirtyTwoBit(u) => Ok(f32::from_bits(u)),
            ref ufd => Err(DecodeError::new(format!(
                "cannot decode f32 from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.DoubleValue`
impl Message for f64 {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if *self != 0.0 {
            double::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            double::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0.0 {
            double::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        *self = 0.0;
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        match f.data {
            UnknownFieldData::SixtyFourBit(u) => Ok(f64::from_bits(u)),
            ref ufd => Err(DecodeError::new(format!(
                "cannot decode f64 from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.StringValue`
impl Message for String {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if !self.is_empty() {
            string::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            string::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            string::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.clear();
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        match &f.data {
            UnknownFieldData::LengthDelimited(bytes) => std::str::from_utf8(bytes.as_slice())
                .map(|s| s.to_string())
                .map_err(|e| DecodeError::new(e.to_string())),
            ufd => Err(DecodeError::new(format!(
                "cannot decode String from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.BytesValue`
impl Message for Vec<u8> {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            bytes::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            bytes::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.clear();
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError>
    {
        match &f.data {
            UnknownFieldData::LengthDelimited(bytes) => Ok(bytes.clone()),
            ufd => Err(DecodeError::new(format!(
                "cannot decode Vec<u8> from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.BytesValue`
impl Message for Bytes {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            bytes::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            bytes::encoded_len(1, self)
        } else {
            0
        }
    }
    fn clear(&mut self) {
        self.clear();
    }

    fn decode_from_unknown(f: &UnknownField) -> Result<Self, DecodeError> {
        match &f.data {
            UnknownFieldData::LengthDelimited(bytes) => Ok(Bytes::from(bytes.clone())),
            ufd => Err(DecodeError::new(format!(
                "cannot decode Bytes from {:?}",
                ufd
            ))),
        }
    }
}

/// `google.protobuf.Empty`
impl Message for () {
    fn encode_raw<B>(&self, _buf: &mut B)
    where
        B: BufMut,
    {
    }
    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        skip_field(wire_type, tag, buf, ctx)
    }
    fn encoded_len(&self) -> usize {
        0
    }
    fn clear(&mut self) {}

    fn decode_from_unknown(_f: &UnknownField) -> Result<Self, DecodeError>
    {
        Ok(())
    }
}
