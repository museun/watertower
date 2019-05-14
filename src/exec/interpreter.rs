use std::collections::HashMap;
use std::rc::Rc;

use super::*;

#[derive(Debug, Default)]
pub struct Interpreter {
    pc: usize,
    stack: Vec<Value>,
    sp: usize,
    offset: usize,

    class_index_map: HashMap<String, HashMap<usize, String>>,
    classes: HashMap<String, Rc<ty::ClassFile>>,
    // code_cache: Cache<attr::Code>,
}

impl Interpreter {
    pub fn load_class(&mut self, class: Rc<ty::ClassFile>) {
        let map = Self::build_class_map(&class);
        let name = class.get_class_name();
        self.class_index_map.insert(name.to_string(), map);
        self.classes.insert(name.to_string(), Rc::clone(&class));
    }

    pub fn load_class_from_reader<'a, R, Reader>(&mut self, reader: Reader) -> Result<()>
    where
        R: std::io::Read + 'a,
        Reader: Into<crate::parse::Reader<'a, R>>,
    {
        ty::ClassFile::read(reader)
            .map_err(Into::into)
            .map(|class| self.load_class(Rc::new(class)))
    }

    pub fn run(self) {}

    fn build_class_map(class: &ty::ClassFile) -> HashMap<usize, String> {
        class
            .constant_pool
            .iter()
            .filter_map(|k| match k {
                ty::Constant::MethodRef(ty::constant::MethodRef { class, .. }) => Some(class),
                _ => None,
            })
            .filter_map(|index| match index.lookup(&class.constant_pool) {
                Ok(&ty::Constant::ClassRef(class)) => Some((index, class)),
                _ => None,
            })
            .filter_map(|(index, name)| match name.lookup(&class.constant_pool) {
                Ok(ty::Constant::Utf8(s)) => Some((index.0, s)),
                _ => None,
            })
            .map(|(index, name)| (usize::from(index), name.to_string()))
            .collect()
    }

    fn push(&mut self, val: impl Into<Value>) {
        self.sp += 1;
        self.stack.push(val.into());
    }

    fn pop(&mut self) -> Value {
        self.sp -= 1;
        self.stack.pop().expect("stack must not be empty")
    }

    fn step(&mut self) {
        // eprintln!("offset: {}", self.offset);
        // let code = self.code_cache.get(self.offset);
        // for code in &code.code {
        //     self.execute(*code);
        // }
        // eprintln!();
        // self.offset += 1;
    }

    fn execute(&mut self, op: u8) {
        if let Some(instruction) = Instruction::lookup(op) {
            eprintln!("0x{:02X} : {}", instruction.opcode(), instruction);
        }

        /*
        use Instruction::*;
        match Instruction::lookup(op) {
            Some(inst) => match (inst.arity(), inst) {
                (d, AALOAD) => unimplemented!(),
                (d, AASTORE) => unimplemented!(),
                (d, ACONST_NULL) => unimplemented!(),
                (d, ALOAD) => unimplemented!(),
                (d, ALOAD_0) => unimplemented!(),
                (d, ALOAD_1) => unimplemented!(),
                (d, ALOAD_2) => unimplemented!(),
                (d, ALOAD_3) => unimplemented!(),
                (d, ANEWARRAY) => unimplemented!(),
                (d, ARETURN) => unimplemented!(),
                (d, ARRAYLENGTH) => unimplemented!(),
                (d, ASTORE) => unimplemented!(),
                (d, ASTORE_0) => unimplemented!(),
                (d, ASTORE_1) => unimplemented!(),
                (d, ASTORE_2) => unimplemented!(),
                (d, ASTORE_3) => unimplemented!(),
                (d, ATHROW) => unimplemented!(),
                (d, BALOAD) => unimplemented!(),
                (d, BASTORE) => unimplemented!(),
                (d, BIPUSH) => unimplemented!(),
                (d, BREAKPOINT) => unimplemented!(),
                (d, CALOAD) => unimplemented!(),
                (d, CASTORE) => unimplemented!(),
                (d, CHECKCAST) => unimplemented!(),
                (d, D2F) => unimplemented!(),
                (d, D2I) => unimplemented!(),
                (d, D2L) => unimplemented!(),
                (d, DADD) => unimplemented!(),
                (d, DALOAD) => unimplemented!(),
                (d, DASTORE) => unimplemented!(),
                (d, DCMPG) => unimplemented!(),
                (d, DCMPL) => unimplemented!(),
                (d, DCONST_0) => unimplemented!(),
                (d, DCONST_1) => unimplemented!(),
                (d, DDIV) => unimplemented!(),
                (d, DLOAD) => unimplemented!(),
                (d, DLOAD_0) => unimplemented!(),
                (d, DLOAD_1) => unimplemented!(),
                (d, DLOAD_2) => unimplemented!(),
                (d, DLOAD_3) => unimplemented!(),
                (d, DMUL) => unimplemented!(),
                (d, DNEG) => unimplemented!(),
                (d, DREM) => unimplemented!(),
                (d, DRETURN) => unimplemented!(),
                (d, DSTORE) => unimplemented!(),
                (d, DSTORE_0) => unimplemented!(),
                (d, DSTORE_1) => unimplemented!(),
                (d, DSTORE_2) => unimplemented!(),
                (d, DSTORE_3) => unimplemented!(),
                (d, DSUB) => unimplemented!(),
                (d, DUP) => unimplemented!(),
                (d, DUP_X1) => unimplemented!(),
                (d, DUP_X2) => unimplemented!(),
                (d, DUP2) => unimplemented!(),
                (d, DUP2_X1) => unimplemented!(),
                (d, DUP2_X2) => unimplemented!(),
                (d, F2D) => unimplemented!(),
                (d, F2I) => unimplemented!(),
                (d, F2L) => unimplemented!(),
                (d, FADD) => unimplemented!(),
                (d, FALOAD) => unimplemented!(),
                (d, FASTORE) => unimplemented!(),
                (d, FCMPG) => unimplemented!(),
                (d, FCMPL) => unimplemented!(),
                (d, FCONST_0) => unimplemented!(),
                (d, FCONST_1) => unimplemented!(),
                (d, FCONST_2) => unimplemented!(),
                (d, FDIV) => unimplemented!(),
                (d, FLOAD) => unimplemented!(),
                (d, FLOAD_0) => unimplemented!(),
                (d, FLOAD_1) => unimplemented!(),
                (d, FLOAD_2) => unimplemented!(),
                (d, FLOAD_3) => unimplemented!(),
                (d, FMUL) => unimplemented!(),
                (d, FNEG) => unimplemented!(),
                (d, FREM) => unimplemented!(),
                (d, FRETURN) => unimplemented!(),
                (d, FSTORE) => unimplemented!(),
                (d, FSTORE_0) => unimplemented!(),
                (d, FSTORE_1) => unimplemented!(),
                (d, FSTORE_2) => unimplemented!(),
                (d, FSTORE_3) => unimplemented!(),
                (d, FSUB) => unimplemented!(),
                (d, GETFIELD) => unimplemented!(),
                (d, GETSTATIC) => unimplemented!(),
                (d, GOTO) => unimplemented!(),
                (d, GOTO_W) => unimplemented!(),
                (d, I2B) => unimplemented!(),
                (d, I2C) => unimplemented!(),
                (d, I2D) => unimplemented!(),
                (d, I2F) => unimplemented!(),
                (d, I2L) => unimplemented!(),
                (d, I2S) => unimplemented!(),
                (d, IADD) => unimplemented!(),
                (d, IALOAD) => unimplemented!(),
                (d, IAND) => unimplemented!(),
                (d, IASTORE) => unimplemented!(),
                (d, ICONST_M1) => unimplemented!(),
                (d, ICONST_0) => unimplemented!(),
                (d, ICONST_1) => unimplemented!(),
                (d, ICONST_2) => unimplemented!(),
                (d, ICONST_3) => unimplemented!(),
                (d, ICONST_4) => unimplemented!(),
                (d, ICONST_5) => unimplemented!(),
                (d, IDIV) => unimplemented!(),
                (d, IF_ACMPEQ) => unimplemented!(),
                (d, IF_ACMPNE) => unimplemented!(),
                (d, IF_ICMPEQ) => unimplemented!(),
                (d, IF_ICMPGE) => unimplemented!(),
                (d, IF_ICMPGT) => unimplemented!(),
                (d, IF_ICMPLE) => unimplemented!(),
                (d, IF_ICMPLT) => unimplemented!(),
                (d, IF_ICMPNE) => unimplemented!(),
                (d, IFEQ) => unimplemented!(),
                (d, IFGE) => unimplemented!(),
                (d, IFGT) => unimplemented!(),
                (d, IFLE) => unimplemented!(),
                (d, IFLT) => unimplemented!(),
                (d, IFNE) => unimplemented!(),
                (d, IFNONNULL) => unimplemented!(),
                (d, IFNULL) => unimplemented!(),
                (d, IINC) => unimplemented!(),
                (d, ILOAD) => unimplemented!(),
                (d, ILOAD_0) => unimplemented!(),
                (d, ILOAD_1) => unimplemented!(),
                (d, ILOAD_2) => unimplemented!(),
                (d, ILOAD_3) => unimplemented!(),
                (d, IMPDEP1) => unimplemented!(),
                (d, IMPDEP2) => unimplemented!(),
                (d, IMUL) => unimplemented!(),
                (d, INEG) => unimplemented!(),
                (d, INSTANCEOF) => unimplemented!(),
                (d, INVOKEDYNAMIC) => unimplemented!(),
                (d, INVOKEINTERFACE) => unimplemented!(),
                (d, INVOKESPECIAL) => unimplemented!(),
                (d, INVOKESTATIC) => unimplemented!(),
                (d, INVOKEVIRTUAL) => unimplemented!(),
                (d, IOR) => unimplemented!(),
                (d, IREM) => unimplemented!(),
                (d, IRETURN) => unimplemented!(),
                (d, ISHL) => unimplemented!(),
                (d, ISHR) => unimplemented!(),
                (d, ISTORE) => unimplemented!(),
                (d, ISTORE_0) => unimplemented!(),
                (d, ISTORE_1) => unimplemented!(),
                (d, ISTORE_2) => unimplemented!(),
                (d, ISTORE_3) => unimplemented!(),
                (d, ISUB) => unimplemented!(),
                (d, IUSHR) => unimplemented!(),
                (d, IXOR) => unimplemented!(),
                (d, JSR) => unimplemented!(),
                (d, JSR_W) => unimplemented!(),
                (d, L2D) => unimplemented!(),
                (d, L2F) => unimplemented!(),
                (d, L2I) => unimplemented!(),
                (d, LADD) => unimplemented!(),
                (d, LALOAD) => unimplemented!(),
                (d, LAND) => unimplemented!(),
                (d, LASTORE) => unimplemented!(),
                (d, LCMP) => unimplemented!(),
                (d, LCONST_0) => unimplemented!(),
                (d, LCONST_1) => unimplemented!(),
                (d, LDC) => unimplemented!(),
                (d, LDC_W) => unimplemented!(),
                (d, LDC2_W) => unimplemented!(),
                (d, LDIV) => unimplemented!(),
                (d, LLOAD) => unimplemented!(),
                (d, LLOAD_0) => unimplemented!(),
                (d, LLOAD_1) => unimplemented!(),
                (d, LLOAD_2) => unimplemented!(),
                (d, LLOAD_3) => unimplemented!(),
                (d, LMUL) => unimplemented!(),
                (d, LNEG) => unimplemented!(),
                (d, LOOKUPSWITCH) => unimplemented!(),
                (d, LOR) => unimplemented!(),
                (d, LREM) => unimplemented!(),
                (d, LRETURN) => unimplemented!(),
                (d, LSHL) => unimplemented!(),
                (d, LSHR) => unimplemented!(),
                (d, LSTORE) => unimplemented!(),
                (d, LSTORE_0) => unimplemented!(),
                (d, LSTORE_1) => unimplemented!(),
                (d, LSTORE_2) => unimplemented!(),
                (d, LSTORE_3) => unimplemented!(),
                (d, LSUB) => unimplemented!(),
                (d, LUSHR) => unimplemented!(),
                (d, LXOR) => unimplemented!(),
                (d, MONITORENTER) => unimplemented!(),
                (d, MONITOREXIT) => unimplemented!(),
                (d, MULTIANEWARRAY) => unimplemented!(),
                (d, NEW) => unimplemented!(),
                (d, NEWARRAY) => unimplemented!(),
                (d, NOP) => unimplemented!(),
                (d, POP) => unimplemented!(),
                (d, POP2) => unimplemented!(),
                (d, PUTFIELD) => unimplemented!(),
                (d, PUTSTATIC) => unimplemented!(),
                (d, RET) => unimplemented!(),
                (d, RETURN) => unimplemented!(),
                (d, SALOAD) => unimplemented!(),
                (d, SASTORE) => unimplemented!(),
                (d, SIPUSH) => unimplemented!(),
                (d, SWAP) => unimplemented!(),
                (d, TABLESWITCH) => unimplemented!(),
                (d, WIDE) => unimplemented!(),
            },
            _ => unimplemented!("bad instruction: 0x{:02X}", op),
        }
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn something() {
        let fi = std::fs::read("./etc/hello.class").unwrap();

        let mut interpreter = Interpreter::default();
        interpreter
            .load_class_from_reader(&mut fi.as_slice())
            .unwrap();

        eprintln!("{:#?}", interpreter);
    }
}
