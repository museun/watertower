use super::*;

#[derive(Default)]
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

        if reader.read_u32("magic")? != 0xCAFE_BABE {
            return Err(Error::InvalidClassFile);
        }

        let minor_version = reader.read_u16("minor_version")?;
        let major_version = reader.read_u16("major_version")?;

        let constant_pool = reader.read_many(
            |reader| reader.read_u16("constant_pool_count").map(|d| d - 1),
            |reader| Constant::read(reader, &NullContext),
        )?;

        let flags = reader
            .read_u16("flags")
            .map(ClassFlags::from_bits)?
            .expect("valid flags");

        let this_class = reader.read_u16("this_class").map(ConstantIndex)?;
        let super_class = reader.read_u16("super_class").map(ConstantIndex)?;

        let interfaces = reader.read_many(
            |reader| reader.read_u16("interfaces_count"), //
            |reader| ConstantIndex::read(reader, &NullContext),
        )?;

        let ctx = ReadContext {
            constants: constant_pool.as_slice(),
        };

        let fields = reader.read_many(
            |reader| reader.read_u16("fields_count"), //
            |reader| Field::read(reader, &ctx),
        )?;

        let methods = reader.read_many(
            |reader| reader.read_u16("methods_count"), //
            |reader| Method::read(reader, &ctx),
        )?;

        let attributes = reader.read_many(
            |reader| reader.read_u16("attributes_count"), //
            |reader| Attribute::read(reader, &ctx),
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

#[derive(PartialEq, Debug, Clone)]
pub struct InnerClassInfo {
    inner_class: ConstantIndex,
    outer_class: ConstantIndex,
    inner_class_name: ConstantIndex,
    flags: InnerClassFlags,
}

bitflags! {
    #[derive(Default)]
    pub struct ClassFlags: u16 {
        const PUBLIC     = 0x0001;
        const FINAL      = 0x0010;
        const SUPER      = 0x0020;
        const ABSTRACT   = 0x0040;
        const INTERFACE  = 0x0200;
        const SYNTHETIC  = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM       = 0x4000;
    }
}

bitflags! {
    pub struct InnerClassFlags: u16 {
        const PUBLIC     = 0x0001;
        const PRIVATE    = 0x0002;
        const PROTECTED  = 0x0004;
        const STATIC     = 0x0008;
        const FINAL      = 0x0010;
        const ABSTRACT   = 0x0040;
        const INTERFACE  = 0x0200;
        const SYNTHETIC  = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM       = 0x4000;
    }
}
