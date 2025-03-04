//! Traits for encoding and decoding network message types.

use super::VarInt;

use std::io;

/// A trait for unifying encoding and decoding.
pub trait Codec {
    /// Encodes the payload into the supplied buffer.
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()>;

    /// Decodes the bytes and returns the payload.
    fn decode(bytes: &mut io::Cursor<&[u8]>) -> io::Result<Self>
    where
        Self: Sized;
}

impl<T: Codec> Codec for Vec<T> {
    fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        VarInt(self.len()).encode(buffer)?;
        for element in self {
            element.encode(buffer)?;
        }

        Ok(())
    }

    fn decode(bytes: &mut io::Cursor<&[u8]>) -> io::Result<Self>
    where
        Self: Sized,
    {
        let length = *VarInt::decode(bytes)?;
        (0..length)
            .map(|_| T::decode(bytes))
            .collect::<io::Result<Self>>()
    }
}
