use std::io::Read;

use super::*;

pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Constant>,
    pub flags: ClassFlags,
    pub this_class: ConstantIndex,
    pub super_class: ConstantIndex,
    pub interfaces: Vec<ConstantIndex>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl std::fmt::Debug for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassFile")
            .field("minor_version", &self.minor_version)
            .field("major_version", &self.major_version)
            .field("constant_pool_len", &self.constant_pool.len())
            .field("flags", &self.flags)
            .field("this_class", &self.this_class)
            .field("super_class", &self.super_class)
            .field("interfaces", &self.interfaces)
            .field("fields", &self.fields)
            .field("methods", &self.methods)
            .field("attributes", &self.attributes)
            .finish()
    }
}

impl ClassFile {
    pub fn read<'a, R, I>(reader: I) -> Result<Self>
    where
        R: Read + 'a,
        I: Into<Reader<'a, R>>,
    {
        let mut reader = reader.into();

        const MAGIC: u32 = 0xCAFE_BABE;
        let magic = reader.read_u32("magic")?;
        if magic != MAGIC {
            return Err(Error::Expected(
                format!("{:#X?}", magic),
                format!("{:#X?}", MAGIC),
            ));
        }

        let minor_version = reader.read_u16("minor_version")?;
        let major_version = reader.read_u16("major_version")?;

        let constant_pool = reader.read_many(
            |reader| reader.read_u16("constant_pool_count").map(|d| d - 1),
            Constant::read,
        )?;

        let flags = reader
            .read_u16("flags")
            .map(ClassFlags::from_bits)?
            .expect("valid flags");

        let this_class = reader.read_u16("this_class").map(ConstantIndex)?;
        let super_class = reader.read_u16("super_class").map(ConstantIndex)?;

        let interfaces = reader.read_many(
            |reader| reader.read_u16("interfaces_count"), //
            ConstantIndex::read,
        )?;

        let fields = reader.read_many(
            |reader| reader.read_u16("fields_count"), //
            |reader| Field::read(reader, constant_pool.as_slice()),
        )?;

        let methods = reader.read_many(
            |reader| reader.read_u16("methods_count"), //
            |reader| Method::read(reader, constant_pool.as_slice()),
        )?;

        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes_count"), //
            |reader| Attribute::read(reader, constant_pool.as_slice()),
        )?;

        Ok(Self {
            minor_version,
            major_version,
            constant_pool,
            flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}
