use super::*;

#[derive(PartialEq, Debug, Clone)]
pub enum StackMapFrame {
    SameFrame {
        offset: u8,
    },
    SameFrameExtended {
        offset: u16,
    },
    SameLocalsOneStackItemFrame {
        offset: u8,
        stack_item: VerificationType,
    },
    SameLocalsOneStackItemFrameExtended {
        offset: u16,
        stack_item: VerificationType,
    },
    ChopFrame {
        offset: u16,
        absent_locals: u8,
    },
    AppendFrame {
        offset: u16,
        new_locals: Vec<VerificationType>,
    },
    FullFrame {
        offset: u16,
        locals: Vec<VerificationType>,
        stack_items: Vec<VerificationType>,
    },
}

impl StackMapFrame {
    pub(super) fn read<R: Read>(reader: &mut Reader<'_, R>) -> Result<Self> {
        let ty = reader.read_u8("stack_map_frame type")?;
        match ty {
            0...63 => Ok(StackMapFrame::SameFrame { offset: ty }),
            64...127 => Ok(StackMapFrame::SameLocalsOneStackItemFrame {
                offset: ty - 64,
                stack_item: VerificationType::read(reader)?,
            }),
            247 => Ok(StackMapFrame::SameLocalsOneStackItemFrameExtended {
                offset: reader.read_u16("same_locals_one_stack_item_frame_extended")?,
                stack_item: VerificationType::read(reader)?,
            }),
            248...250 => Ok(StackMapFrame::ChopFrame {
                offset: reader.read_u16("chop_frame")?,
                absent_locals: (251 - ty),
            }),
            251 => Ok(StackMapFrame::SameFrameExtended {
                offset: reader.read_u16("same_frame_extended")?,
            }),
            252...254 => {
                let offset = reader.read_u16("append_frame")?;
                let new_locals = reader.read_many(
                    |reader| reader.read_u16("num_locals"),
                    VerificationType::read,
                )?;
                Ok(StackMapFrame::AppendFrame { offset, new_locals })
            }
            255 => {
                let offset = reader.read_u16("full_frame")?;
                let locals = reader.read_many(
                    |reader| reader.read_u16("num_locals"),
                    VerificationType::read,
                )?;
                let stack_items = reader.read_many(
                    |reader| reader.read_u16("num_stack_items"),
                    VerificationType::read,
                )?;
                Ok(StackMapFrame::FullFrame {
                    offset,
                    locals,
                    stack_items,
                })
            }
            _ => Err(Error::InvalidStackFrameType(ty)),
        }
    }
}
