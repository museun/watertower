pub trait JvmInstruction {
    fn description(&self) -> &'static str;
    fn opcode(&self) -> u8;
}

macro_rules! instruction {
    (@one $($x:tt)*) => (());
    (@count $($args:ty),*) => (<[()]>::len(&[$(instruction!(@one $args)),*]));

    (@impl_ $inst:ident, $opcode:expr, $doc:expr) => {
        impl JvmInstruction for $inst {
            fn description(&self) -> &'static str {
                $doc
            }
            #[inline]
            fn opcode(&self) -> u8 {
                $opcode
            }
        }
    };

    (@create $opcode:expr, $inst:ident, (), $doc:expr) => {
        instruction!(@empty $opcode, $inst, $doc);
    };

    (@create $opcode:expr, $inst:ident, (Vec<$a:tt>), $doc:expr) => {
        #[derive(Default, Debug, PartialEq, Clone)]
        #[allow(non_camel_case_types)]
        #[doc = $doc]
        pub struct $inst (pub Vec<$a>);
        instruction!(@impl_ $inst, $opcode, $doc);
    };

    (@create $opcode:expr, $inst:ident, ($a:tt), $doc:expr) => {
        #[derive(Default, Debug, PartialEq, Clone)]
        #[allow(non_camel_case_types)]
        #[doc = $doc]
        pub struct $inst (pub $a);
        instruction!(@impl_ $inst, $opcode, $doc);
    };

    (@create $opcode:expr, $inst:ident, ($a:tt, $b:tt), $doc:expr) => {
        #[derive(Default, Debug, PartialEq, Clone)]
        #[allow(non_camel_case_types)]
        #[doc = $doc]
        pub struct $inst (pub $a, pub $b);
        instruction!(@impl_ $inst, $opcode, $doc);
    };

    (@create $opcode:expr, $inst:ident, ($a:tt, $b:tt, $c:tt), $doc:expr) => {
        #[derive(Default, Debug, PartialEq, Clone)]
        #[allow(non_camel_case_types)]
        #[doc = $doc]
        pub struct $inst (pub $a, pub $b, pub $c);
        instruction!(@impl_ $inst, $opcode, $doc);
    };

    (@create $opcode:expr, $inst:ident, ($a:tt, $b:tt, $c:tt, $d:tt), $doc:expr) => {
        #[derive(Default, Debug, PartialEq, Clone)]
        #[allow(non_camel_case_types)]
        #[doc = $doc]
        pub struct $inst (pub $a, pub $b, pub $c, pub $d);
        instruction!(@impl_ $inst, $opcode, $doc);
    };

    (@create $opcode:expr, $inst:ident, ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt), $doc:expr) => {
        #[derive(Default, Debug, PartialEq, Clone)]
        #[allow(non_camel_case_types)]
        #[doc = $doc]
        pub struct $inst (pub $a, pub $b, pub $c, pub $d, pub $e);
        instruction!(@impl_ $inst, $opcode, $doc);
    };

    (@empty $opcode:expr, $inst:ident, $doc:expr) => {
        #[derive(Default, Debug, PartialEq, Clone)]
        #[allow(non_camel_case_types)]
        #[doc = $doc]
        pub struct $inst;
        instruction!(@impl_ $inst, $opcode, $doc);
    };

    ($($opcode:expr => $inst:ident >> $ty:tt => $doc:expr);* $(;)?) => {
        $( instruction!(@create $opcode, $inst, $ty, $doc); )*

        #[derive(Debug, Clone, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum Instruction {
            $(#[doc = $doc] $inst($inst),)*
        }

        impl Instruction {
            /// Lookup the instructions for an Opcode
            pub fn lookup(opcode: u8) -> Option<Self> {
                match opcode {
                    $($opcode => Some(Instruction::$inst($inst::default())),)*
                    _ => None
                }
            }

            /// Is this instruction a variable-length isntruction?
            pub fn is_varargs(&self) -> bool {
                match self {
                    Instruction::LOOKUPSWITCH(..) | Instruction::TABLESWITCH(..) => true,
                    _ => false
                }
            }

            /// Does this instruction involve a wide set of arguments?
            pub fn is_wide(&self) -> bool {
                if let Instruction::WIDE(..) = self {
                    true
                } else {
                    false
                }
            }

            pub fn description(&self) -> &'static str {
                match self {
                    $(Instruction::$inst(d) => d.description(),)*
                }
            }
        }

        impl std::fmt::Display for Instruction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Instruction::$inst(..) => write!(f, "{}", stringify!($inst)),)*
                }
            }
        }
    };
}

// TODO make this documentation look less like garbage
instruction! {
    0x00 => NOP            >> ()               => "perform no operation";
    0x01 => ACONST_NULL    >> ()               => "push a null reference onto the stack";
    0x02 => ICONST_M1      >> ()               => "load the int value −1 onto the stack";
    0x03 => ICONST_0       >> ()               => "load the int value 0 onto the stack";
    0x04 => ICONST_1       >> ()               => "load the int value 1 onto the stack";
    0x05 => ICONST_2       >> ()               => "load the int value 2 onto the stack";
    0x06 => ICONST_3       >> ()               => "load the int value 3 onto the stack";
    0x07 => ICONST_4       >> ()               => "load the int value 4 onto the stack";
    0x08 => ICONST_5       >> ()               => "load the int value 5 onto the stack";
    0x09 => LCONST_0       >> ()               => "push 0L (the number zero with type long) onto the stack";
    0x0A => LCONST_1       >> ()               => "push 1L (the number one with type long) onto the stack";
    0x0B => FCONST_0       >> ()               => "push 0.0f on the stack";
    0x0C => FCONST_1       >> ()               => "push 1.0f on the stack";
    0x0D => FCONST_2       >> ()               => "push 2.0f on the stack";
    0x0E => DCONST_0       >> ()               => "push the constant 0.0 (a double) onto the stack";
    0x0F => DCONST_1       >> ()               => "push the constant 1.0 (a double) onto the stack";
    0x10 => BIPUSH         >> (u8)             => "push a byte onto the stack as an integer value";
    0x11 => SIPUSH         >> (u8,u8)          => "push a short onto the stack as an integer value";
    0x12 => LDC            >> (u8)             => "push a constant #index from a constant pool (`String`, `int`, `float`, `Class`, `java.lang.invoke.MethodType`, or `java.lang.invoke.MethodHandle`) onto the stack";
    0x13 => LDC_W          >> (u8,u8)          => "push a constant #index from a constant pool (`String`, `int`, `float`, `Class`, `java.lang.invoke.MethodType`, or `java.lang.invoke.MethodHandle`) onto the stack (wide index is constructed as `indexbyte1 << 8 + indexbyte2`)";
    0x14 => LDC2_W         >> (u8,u8)          => "push a constant #index from a constant pool (`double` or `long`) onto the stack (wide index is constructed as `indexbyte1 << 8 + indexbyte2`)";
    0x15 => ILOAD          >> (u8)             => "load an int value from a local variable #index";
    0x16 => LLOAD          >> (u8)             => "load a long value from a local variable #index";
    0x17 => FLOAD          >> (u8)             => "load a float value from a local variable #index";
    0x18 => DLOAD          >> (u8)             => "load a double value from a local variable #index";
    0x19 => ALOAD          >> ()               => "load a reference onto the stack from a local variable #index";
    0x1A => ILOAD_0        >> ()               => "load an int value from local variable 0";
    0x1B => ILOAD_1        >> ()               => "load an int value from local variable 1";
    0x1C => ILOAD_2        >> ()               => "load an int value from local variable 2";
    0x1D => ILOAD_3        >> ()               => "load an int value from local variable 3";
    0x1E => LLOAD_0        >> ()               => "load a long value from a local variable 0";
    0x1F => LLOAD_1        >> ()               => "load a long value from a local variable 1";
    0x20 => LLOAD_2        >> ()               => "load a long value from a local variable 2";
    0x21 => LLOAD_3        >> ()               => "load a long value from a local variable 3";
    0x22 => FLOAD_0        >> ()               => "load a float value from local variable 0";
    0x23 => FLOAD_1        >> ()               => "load a float value from local variable 1";
    0x24 => FLOAD_2        >> ()               => "load a float value from local variable 2";
    0x25 => FLOAD_3        >> ()               => "load a float value from local variable 3";
    0x26 => DLOAD_0        >> ()               => "load a double from local variable 0";
    0x27 => DLOAD_1        >> ()               => "load a double from local variable 1";
    0x28 => DLOAD_2        >> ()               => "load a double from local variable 2";
    0x29 => DLOAD_3        >> ()               => "load a double from local variable 3";
    0x2A => ALOAD_0        >> (u8)             => "load a reference onto the stack from local variable 0";
    0x2B => ALOAD_1        >> ()               => "load a reference onto the stack from local variable 1";
    0x2C => ALOAD_2        >> ()               => "load a reference onto the stack from local variable 2";
    0x2D => ALOAD_3        >> ()               => "load a reference onto the stack from local variable 3";
    0x2E => IALOAD         >> ()               => "load an int from an array";
    0x2F => LALOAD         >> ()               => "load a long from an array";
    0x30 => FALOAD         >> ()               => "load a float from an array";
    0x31 => DALOAD         >> ()               => "load a double from an array";
    0x32 => AALOAD         >> ()               => "load onto the stack a reference from an array";
    0x33 => BALOAD         >> ()               => "load a byte or Boolean value from an array";
    0x34 => CALOAD         >> ()               => "load a char from an array";
    0x35 => SALOAD         >> ()               => "load short from array";
    0x36 => ISTORE         >> (u8)             => "store int value into variable #index";
    0x37 => LSTORE         >> (u8)             => "store a long value in a local variable #index";
    0x38 => FSTORE         >> (u8)             => "store a float value into a local variable #index";
    0x39 => DSTORE         >> (u8)             => "store a double value into a local variable #index";
    0x3A => ASTORE         >> (u8)             => "store a reference into a local variable #index";
    0x3B => ISTORE_0       >> ()               => "store int value into variable 0";
    0x3C => ISTORE_1       >> ()               => "store int value into variable 1";
    0x3D => ISTORE_2       >> ()               => "store int value into variable 2";
    0x3E => ISTORE_3       >> ()               => "store int value into variable 3";
    0x3F => LSTORE_0       >> ()               => "store a long value in a local variable 0";
    0x40 => LSTORE_1       >> ()               => "store a long value in a local variable 1";
    0x41 => LSTORE_2       >> ()               => "store a long value in a local variable 2";
    0x42 => LSTORE_3       >> ()               => "store a long value in a local variable 3";
    0x43 => FSTORE_0       >> ()               => "store a float value into local variable 0";
    0x44 => FSTORE_1       >> ()               => "store a float value into local variable 1";
    0x45 => FSTORE_2       >> ()               => "store a float value into local variable 2";
    0x46 => FSTORE_3       >> ()               => "store a float value into local variable 3";
    0x47 => DSTORE_0       >> ()               => "store a double into local variable 0";
    0x48 => DSTORE_1       >> ()               => "store a double into local variable 1";
    0x49 => DSTORE_2       >> ()               => "store a double into local variable 2";
    0x4A => DSTORE_3       >> ()               => "store a double into local variable 3";
    0x4B => ASTORE_0       >> ()               => "store a reference into local variable 0";
    0x4C => ASTORE_1       >> ()               => "store a reference into local variable 1";
    0x4D => ASTORE_2       >> ()               => "store a reference into local variable 2";
    0x4E => ASTORE_3       >> ()               => "store a reference into local variable 3";
    0x4F => IASTORE        >> ()               => "store an int into an array";
    0x50 => LASTORE        >> ()               => "store a long to an array";
    0x51 => FASTORE        >> ()               => "store a float in an array";
    0x52 => DASTORE        >> ()               => "store a double into an array";
    0x53 => AASTORE        >> ()               => "store into a reference in an array";
    0x54 => BASTORE        >> ()               => "store a byte or Boolean value into an array";
    0x55 => CASTORE        >> ()               => "store a char into an array";
    0x56 => SASTORE        >> ()               => "store short to array";
    0x57 => POP            >> ()               => "discard the top value on the stack";
    0x58 => POP2           >> ()               => "discard the top two values on the stack (or one value, if it is a double or long)";
    0x59 => DUP            >> ()               => "duplicate the value on top of the stack";
    0x5A => DUP_X1         >> ()               => "insert a copy of the top value into the stack two values from the top. value1 and value2 must not be of the type double or long.";
    0x5B => DUP_X2         >> ()               => "insert a copy of the top value into the stack two (if value2 is double or long it takes up the entry of value3, too) or three values (if value2 is neither double nor long) from the top";
    0x5C => DUP2           >> ()               => "duplicate top two stack words (two values, if value1 is not double nor long; a single value, if value1 is double or long)";
    0x5D => DUP2_X1        >> ()               => "duplicate two words and insert beneath third word (see explanation above)";
    0x5E => DUP2_X2        >> ()               => "duplicate two words and insert beneath fourth word";
    0x5F => SWAP           >> ()               => "swaps two top words on the stack (note that value1 and value2 must not be double or long)";
    0x60 => IADD           >> ()               => "add two ints";
    0x61 => LADD           >> ()               => "add two longs";
    0x62 => FADD           >> ()               => "add two floats";
    0x63 => DADD           >> ()               => "add two doubles";
    0x64 => ISUB           >> ()               => "int subtract";
    0x65 => LSUB           >> ()               => "subtract two longs";
    0x66 => FSUB           >> ()               => "subtract two floats";
    0x67 => DSUB           >> ()               => "subtract a double from another";
    0x68 => IMUL           >> ()               => "multiply two integers";
    0x69 => LMUL           >> ()               => "multiply two longs";
    0x6A => FMUL           >> ()               => "multiply two floats";
    0x6B => DMUL           >> ()               => "multiply two doubles";
    0x6C => IDIV           >> ()               => "divide two integers";
    0x6D => LDIV           >> ()               => "divide two longs";
    0x6E => FDIV           >> ()               => "divide two floats";
    0x6F => DDIV           >> ()               => "divide two doubles";
    0x70 => IREM           >> ()               => "logical int remainder";
    0x71 => LREM           >> ()               => "remainder of division of two longs";
    0x72 => FREM           >> ()               => "get the remainder from a division between two floats";
    0x73 => DREM           >> ()               => "get the remainder from a division between two doubles";
    0x74 => INEG           >> ()               => "negate int";
    0x75 => LNEG           >> ()               => "negate a long";
    0x76 => FNEG           >> ()               => "negate a float";
    0x77 => DNEG           >> ()               => "negate a double";
    0x78 => ISHL           >> ()               => "int shift left";
    0x79 => LSHL           >> ()               => "bitwise shift left of a long value1 by int value2 positions";
    0x7A => ISHR           >> ()               => "int arithmetic shift right";
    0x7B => LSHR           >> ()               => "bitwise shift right of a long value1 by int value2 positions";
    0x7C => IUSHR          >> ()               => "int logical shift right";
    0x7D => LUSHR          >> ()               => "bitwise shift right of a long value1 by int value2 positions, unsigned";
    0x7E => IAND           >> ()               => "perform a bitwise AND on two integers";
    0x7F => LAND           >> ()               => "bitwise AND of two longs";
    0x80 => IOR            >> ()               => "bitwise int OR";
    0x81 => LOR            >> ()               => "bitwise OR of two longs";
    0x82 => IXOR           >> ()               => "int xor";
    0x83 => LXOR           >> ()               => "bitwise XOR of two longs";
    0x84 => IINC           >> (u8,u8)          => "increment local variable #index by signed byte const";
    0x85 => I2L            >> ()               => "convert an int into a long";
    0x86 => I2F            >> ()               => "convert an int into a float";
    0x87 => I2D            >> ()               => "convert an int into a double";
    0x88 => L2I            >> ()               => "convert a long to a int";
    0x89 => L2F            >> ()               => "convert a long to a float";
    0x8A => L2D            >> ()               => "convert a long to a double";
    0x8B => F2I            >> ()               => "convert a float to an int";
    0x8C => F2L            >> ()               => "convert a float to a long";
    0x8D => F2D            >> ()               => "convert a float to a double";
    0x8E => D2I            >> ()               => "convert a double to an int";
    0x8F => D2L            >> ()               => "convert a double to a long";
    0x90 => D2F            >> ()               => "convert a double to a float";
    0x91 => I2B            >> ()               => "convert an int into a byte";
    0x92 => I2C            >> ()               => "convert an int into a character";
    0x93 => I2S            >> ()               => "convert an int into a short";
    0x94 => LCMP           >> ()               => "push 0 if the two longs are the same, 1 if value1 is greater than value2, -1 otherwise";
    0x95 => FCMPL          >> ()               => "compare two floats";
    0x96 => FCMPG          >> ()               => "compare two floats";
    0x97 => DCMPL          >> ()               => "compare two doubles";
    0x98 => DCMPG          >> ()               => "compare two doubles";
    0x99 => IFEQ           >> (u8,u8)          => "if value is 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9A => IFNE           >> (u8,u8)          => "if value is not 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9B => IFLT           >> (u8,u8)          => "if value is less than 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9C => IFGE           >> (u8,u8)          => "if value is greater than or equal to 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9D => IFGT           >> (u8,u8)          => "if value is greater than 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9E => IFLE           >> (u8,u8)          => "if value is less than or equal to 0, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0x9F => IF_ICMPEQ      >> (u8,u8)          => "if ints are equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA0 => IF_ICMPNE      >> (u8,u8)          => "if ints are not equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA1 => IF_ICMPLT      >> (u8,u8)          => "if value1 is less than value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA2 => IF_ICMPGE      >> (u8,u8)          => "if value1 is greater than or equal to value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA3 => IF_ICMPGT      >> (u8,u8)          => "if value1 is greater than value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA4 => IF_ICMPLE      >> (u8,u8)          => "if value1 is less than or equal to value2, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA5 => IF_ACMPEQ      >> (u8,u8)          => "if references are equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA6 => IF_ACMPNE      >> (u8,u8)          => "if references are not equal, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA7 => GOTO           >> (u8,u8)          => "goes to another instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xA8 => JSR            >> (u8,u8)          => "jump to subroutine at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2) and place the return address on the stack";
    0xA9 => RET            >> (u8)             => "continue execution from address taken from a local variable #index (the asymmetry with jsr is intentional)";
    0xAA => TABLESWITCH    >> (Vec<u8>)        => "continue execution from an address in the table at offset index";
    0xAB => LOOKUPSWITCH   >> (Vec<u8>)        => "a target address is looked up from a table using a key and execution continues from the instruction at that address";
    0xAC => IRETURN        >> ()               => "return an integer from a method";
    0xAD => LRETURN        >> ()               => "return a long value";
    0xAE => FRETURN        >> ()               => "return a float";
    0xAF => DRETURN        >> ()               => "return a double from a method";
    0xB0 => ARETURN        >> ()               => "return a reference from a method";
    0xB1 => RETURN         >> ()               => "return void from method";
    0xB2 => GETSTATIC      >> (u8,u8)          => "get a static field value of a class, where the field is identified by field reference in the constant pool index (indexbyte1 << 8 + indexbyte2)";
    0xB3 => PUTSTATIC      >> (u8,u8)          => "set static field to value in a class, where the field is identified by a field reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB4 => GETFIELD       >> (u8,u8)          => "get a field value of an object objectref, where the field is identified by field reference in the constant pool index (indexbyte1 << 8 + indexbyte2)";
    0xB5 => PUTFIELD       >> (u8,u8)          => "set field to value in an object objectref, where the field is identified by a field reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB6 => INVOKEVIRTUAL  >> (u8,u8)          => "invoke virtual method on object objectref and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB7 => INVOKESPECIAL  >> (u8,u8)          => "invoke instance method on object objectref and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB8 => INVOKESTATIC   >> (u8,u8)          => "invoke a static method and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xB9 => INVOKEINTERFACE>> (u8,u8,u8,u8)    => "invokes an interface method on object objectref and puts the result on the stack (might be void); the interface method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xBA => INVOKEDYNAMIC  >> (u8,u8,u8,u8)    => "invokes a dynamic method and puts the result on the stack (might be void); the method is identified by method reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xBB => NEW            >> (u8,u8)          => "create new object of type identified by class reference in constant pool index (indexbyte1 << 8 + indexbyte2)";
    0xBC => NEWARRAY       >> (u8)             => "create new array with count elements of primitive type identified by atype";
    0xBD => ANEWARRAY      >> (u8,u8)          => "create a new array of references of length count and component type identified by the class reference index (indexbyte1 << 8 + indexbyte2) in the constant pool";
    0xBE => ARRAYLENGTH    >> ()               => "get the length of an array";
    0xBF => ATHROW         >> ()               => "throws an error or exception (notice that the rest of the stack is cleared, leaving only a reference to the Throwable)";
    0xC0 => CHECKCAST      >> (u8,u8)          => "checks whether an objectref is of a certain type, the class reference of which is in the constant pool at index (indexbyte1 << 8 + indexbyte2)";
    0xC1 => INSTANCEOF     >> (u8,u8)          => "determines if an object objectref is of a given type, identified by class reference index in constant pool (indexbyte1 << 8 + indexbyte2)";
    0xC2 => MONITORENTER   >> ()               => "enter monitor for object (\"grab the lock\" – start of synchronized() section)";
    0xC3 => MONITOREXIT    >> ()               => "exit monitor for object (\"release the lock\" – end of synchronized() section)";
    0xC4 => WIDE           >> (u8,u8,u8,u8,u8) => "execute opcode, where opcode is either iload, fload, aload, lload, dload, istore, fstore, astore, lstore, dstore, or ret, but assume the index is 16 bit; or execute iinc, where the index is 16 bits and the constant to increment by is a signed 16 bit short";
    0xC5 => MULTIANEWARRAY >> (u8,u8,u8)       => "create a new array of dimensions dimensions of type identified by class reference in constant pool index (indexbyte1 << 8 + indexbyte2); the sizes of each dimension is identified by count1, [count2, etc.]";
    0xC6 => IFNULL         >> (u8,u8)          => "if value is null, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xC7 => IFNONNULL      >> (u8,u8)          => "if value is not null, branch to instruction at branchoffset (signed short constructed from unsigned bytes branchbyte1 << 8 + branchbyte2)";
    0xC8 => GOTO_W         >> (u8,u8,u8,u8)    => "goes to another instruction at branchoffset (signed int constructed from unsigned bytes branchbyte1 << 24 + branchbyte2 << 16 + branchbyte3 << 8 + branchbyte4)";
    0xC9 => JSR_W          >> (u8,u8,u8,u8)    => "jump to subroutine at branchoffset (signed int constructed from unsigned bytes branchbyte1 << 24 + branchbyte2 << 16 + branchbyte3 << 8 + branchbyte4) and place the return address on the stack";
    0xCA => BREAKPOINT     >> ()               => "reserved for breakpoints in Java debuggers; should not appear in any class file";
    0xFE => IMPDEP1        >> ()               => "reserved for implementation-dependent operations within debuggers; should not appear in any class file";
    0xFF => IMPDEP2        >> ()               => "reserved for implementation-dependent operations within debuggers; should not appear in any class file";
}
