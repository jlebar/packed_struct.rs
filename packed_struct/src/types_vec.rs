//! Helpers for vectors of slice-packable structures

use internal_prelude::v1::*;

use crate::{PackedStructSlice, PackingError};

/// This can only be used as a vector of structures that have a statically known size
impl<T> PackedStructSlice for Vec<T> where T: PackedStructSlice {
    fn pack_to_slice(&self, output: &mut [u8]) -> Result<(), PackingError> {
        let expected_size = Self::packed_bytes_size(Some(self))?;
        if output.len() != expected_size {
            return Err(crate::PackingError::BufferSizeMismatch { expected: expected_size, actual: output.len() });
        }

        let size = T::packed_bytes_size(None)?;

        for (i, item) in self.iter().enumerate() {
            item.pack_to_slice(&mut output[(i * size)..((i+1)*size)])?;
        }

        Ok(())
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, PackingError> {
        let size = T::packed_bytes_size(None)?;
        let modulo = src.len() % size;
        if modulo != 0 {
            return Err(crate::PackingError::BufferModMismatch { actual_size: src.len(), modulo_required: size });
        }
        let n = size / modulo;

        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            let item = T::unpack_from_slice(&src[(i*size)..((i+1)*size)])?;
            vec.push(item);
        }        

        Ok(vec)
    }

    fn packed_bytes_size(opt_self: Option<&Self>) -> Result<usize, PackingError> {
        match opt_self {
            None => Err(PackingError::InstanceRequiredForSize),
            Some(s) => {
                Ok(s.len() * T::packed_bytes_size(None)?)
            }
        }
    }
}