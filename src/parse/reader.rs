use byteorder::{ReadBytesExt, BE};

use super::*;

pub trait ReadType<'a, R> {
    type Output;
    type Context;

    fn read(reader: &mut Reader<'_, R>, context: &'a Self::Context) -> Result<Self::Output>;
}

pub struct ReadContext<'a> {
    pub constants: &'a [Constant],
}

#[derive(Copy, Clone)]
pub struct NullContext;

pub struct Reader<'a, R> {
    source: &'a mut R,
    pos: usize,
}

impl<'a, R: Read> Reader<'a, R> {
    pub fn new(source: &'a mut R, pos: usize) -> Self {
        Self { source, pos }
    }
}

impl<'a, R> From<&'a mut R> for Reader<'a, R>
where
    R: Read,
{
    fn from(read: &'a mut R) -> Self {
        Self::new(read, 0)
    }
}

impl<'a, R: Read> Reader<'a, R> {
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn read_exact(&mut self, buf: &mut [u8], msg: impl std::fmt::Display) -> Result<()> {
        self.source
            .read_exact(buf)
            .map(|_| self.pos += buf.len())
            .map_err(|err| Error::Io {
                msg: msg.to_string(),
                error: err,
            })
    }

    #[inline]
    pub fn read_u64(&mut self, msg: impl std::fmt::Display) -> Result<u64> {
        self.source
            .read_u64::<BE>()
            .map(|n| {
                self.pos += 8;
                n
            })
            .map_err(|err| Error::Io {
                msg: msg.to_string(),
                error: err,
            })
    }

    #[inline]
    pub fn read_u32(&mut self, msg: impl std::fmt::Display) -> Result<u32> {
        self.source
            .read_u32::<BE>()
            .map(|n| {
                self.pos += 4;
                n
            })
            .map_err(|err| Error::Io {
                msg: msg.to_string(),
                error: err,
            })
    }

    #[inline]
    pub fn read_u16(&mut self, msg: impl std::fmt::Display) -> Result<u16> {
        self.source
            .read_u16::<BE>()
            .map(|n| {
                self.pos += 2;
                n
            })
            .map_err(|err| Error::Io {
                msg: msg.to_string(),
                error: err,
            })
    }

    #[inline]
    pub fn read_u8(&mut self, msg: impl std::fmt::Display) -> Result<u8> {
        self.source
            .read_u8()
            .map(|n| {
                self.pos += 1;
                n
            })
            .map_err(|err| Error::Io {
                msg: msg.to_string(),
                error: err,
            })
    }

    #[inline]
    pub fn read_f32(&mut self, msg: impl std::fmt::Display) -> Result<f32> {
        self.source
            .read_f32::<BE>()
            .map(|n| {
                self.pos += 4;
                n
            })
            .map_err(|err| Error::Io {
                msg: msg.to_string(),
                error: err,
            })
    }

    #[inline]
    pub fn read_f64(&mut self, msg: impl std::fmt::Display) -> Result<f64> {
        self.source
            .read_f64::<BE>()
            .map(|n| {
                self.pos += 8;
                n
            })
            .map_err(|err| Error::Io {
                msg: msg.to_string(),
                error: err,
            })
    }

    pub fn read_many<Length, Step, Index, Element>(
        &mut self,
        len: Length,
        step: Step,
    ) -> Result<Vec<Element>>
    where
        Length: Fn(&mut Self) -> Result<Index>,
        Step: Fn(&mut Self) -> Result<Element>,
        Index: Into<usize>,
    {
        let mut vec = Vec::with_capacity(len(self)?.into());
        for _ in 0..vec.capacity() {
            vec.push(step(self)?);
        }
        Ok(vec)
    }

    // fn read_struct<T>(&mut self) -> Result<T> {
    //     let size = std::mem::size_of::<T>();
    //     unsafe {
    //         let mut out = std::mem::zeroed();
    //         self.read_exact(std::slice::from_raw_parts_mut(
    //             &mut out as *mut _ as *mut u8,
    //             size,
    //         ))?;
    //         Ok(out)
    //     }
    // }
}
