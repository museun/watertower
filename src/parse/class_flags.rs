use super::*;

bitflags! {
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
