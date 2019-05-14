use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::Path;

pub struct LogThis {
    file: File,
    pos: usize,
}

impl LogThis {
    pub fn new(name: impl AsRef<Path>) -> Self {
        Self {
            file: File::create(name).unwrap(),
            pos: 0,
        }
    }

    pub fn tee<R: Read>(self, other: R) -> tee::Reader<R, Self> {
        tee::Reader::new(other, self, true)
    }
}
impl Write for LogThis {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        for ch in buf {
            if self.pos % 16 == 0 {
                if self.pos > 0 {
                    self.file.write_all(&[b'\n'])?;
                }
                self.file.write_fmt(format_args!("{:0>4X} ", self.pos))?;
            }
            self.file.write_fmt(format_args!("{:02x} ", ch))?;
            self.pos += 1;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

// usage
// let data = std::fs::File::open("./etc/hello.class").unwrap();
// let mut reader = LogThis::new("out.log").tee(data);
// let class_file = parse::types::ClassFile::read(&mut reader).unwrap();
