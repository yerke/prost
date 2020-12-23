use crate::{DecodeError, Message};
use std::marker::PhantomData;

pub struct ExtFieldOptional<M, F> {
    pub field_number: u32,
    pub phantom: PhantomData<(M, F)>,
}

impl<M, F> ExtFieldOptional<M, F> {
    pub fn get(&self, m: &M) -> Option<Result<F, DecodeError>>
    where
        F: Message + Default + Sized,
        M: Message,
    {
        m.get_unknown_fields()
            .and_then(|m1| m1.get(&self.field_number))
            .map(F::decode_from_unknown)
    }
}
