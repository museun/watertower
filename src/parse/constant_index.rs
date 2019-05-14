use super::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ConstantIndex(pub u16);

impl ConstantIndex {
    pub(super) fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        reader.read_u16("constant index").map(Self)
    }

    // TODO impl this as Index on &'a [T] where T: Constant
    pub fn lookup(self, pool: &[Constant]) -> Result<&Constant> {
        if self.0 == 0 {
            return Err(Error::ZeroIndex);
        } else if pool.len() < self.0 as usize {
            return Err(Error::OutOfRange(self.0));
        }
        let constant = &pool[(&self.0 - 1) as usize];
        match *constant {
            Constant::Padding => Err(Error::IndexInsideDoubleWidthConstant(self.0)),
            _ => Ok(constant),
        }
    }
}
