pub use self::SVMCell::*;
pub use self::Atom::*;

use super::List;

use std::{fmt,ops};
use std::mem::transmute;

#[cfg(test)]
mod tests;

#[macro_export]
#[cfg_attr(feature = "unstable",
    stable(feature = "list", since = "0.1.1") )]
macro_rules! list_cell {
    [ $first:expr, $($rest:expr),+ ] => {
        ListCell(Box::new( list!( $first, $( $rest),+ ) ))
    };
    [ $e:expr ] => {
        ListCell(Box::new( list!($e) ))
    };
    [] => { ListCell(Box::new(Nil)) };

}

#[derive(PartialEq,Clone)]
#[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
pub enum SVMCell {
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    AtomCell(Atom),
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    ListCell(Box<List<SVMCell>>),
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    InstCell(Inst)
}

#[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
impl fmt::Display for SVMCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
impl fmt::Debug for SVMCell {
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AtomCell(atom) => write!(f, "{:?}", atom),
            &ListCell(ref list) => write!(f, "{:?}", list),
            &InstCell(inst) => write!(f, "{:?}", inst)
        }
    }
}

/// SVM atom types.
///
/// A VM atom can be either an unsigned int, signed int, float, or char.
#[derive(PartialEq,PartialOrd,Copy,Clone)]
#[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
pub enum Atom {
    /// Unsigned integer atom (machine 64)
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    UInt(u64),
    /// Signed integer atom (machine 64)
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    SInt(i64),
    /// Floating point number atom (64-bits)
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    Float(f64),
    /// UTF-8 character atom
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    Char(char)
}
#[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
impl fmt::Display for Atom {
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Atom::UInt(value) => write!(f, "{}", value),
            &Atom::SInt(value) => write!(f, "{}", value),
            &Atom::Float(value) => write!(f, "{}", value),
            &Atom::Char(value) => write!(f, "'{}'", value),
        }
    }
}

#[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
impl fmt::Debug for Atom {
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Atom::UInt(value) => write!(f, "{:?}u", value),
            &Atom::SInt(value) => write!(f, "{:?}", value),
            &Atom::Float(value) => write!(f, "{:?}f", value),
            &Atom::Char(value) => write!(f, "'{}'", value),
        }
    }
}

macro_rules! e {
    ($e:expr) => { $e }
}
macro_rules! impl_arith_ops {
    ($name:ident, $path:path, $symbol:tt) => {
        impl $path for Atom {
            type Output = Atom;
            fn $name(self, other: Atom) -> Atom {
                match (self, other) {
                    // same type: no coercion
                    (SInt(a), SInt(b))      => SInt(e!(a $symbol b)),
                    (UInt(a), UInt(b))      => UInt(e!(a $symbol b)),
                    (Float(a), Float(b))    => Float(e!(a $symbol b)),
                    (Char(a), Char(b))      => Char(e!(a as u8 $symbol b as u8) as char),
                    // float & int: coerce to float
                    (Float(a), SInt(b))     => Float(e!(a $symbol b as f64)),
                    (Float(a), UInt(b))     => Float(e!(a $symbol b as f64) ),
                    (SInt(a), Float(b))     => Float(e!(a as f64 $symbol b)),
                    (UInt(a), Float(b))     => Float(e!(a as f64 $symbol b)),
                    // uint & sint: coerce to sint
                    (UInt(a), SInt(b))      => SInt(e!(a as i64 $symbol b)),
                    (SInt(a), UInt(b))      => SInt(e!(a $symbol b as i64)),
                    // char & any: coerce to char
                    // because of the supported operations on Rust chars,
                    // everything has to be cast to u8 (byte) to allow
                    // arithmetic ops and then cast back to char.
                    (Char(a), UInt(b))      => Char(e!(a as u8 $symbol b as u8) as char),
                    (Char(a), SInt(b))      => Char(e!(a as u8 $symbol b as u8) as char),
                    (Char(a), Float(b))     => Char(e!(a as u8 $symbol b as u8) as char),
                    (UInt(a), Char(b))      => Char(e!(a as u8 $symbol b as u8) as char),
                    (SInt(a), Char(b))      => Char(e!(a as u8 $symbol b as u8) as char),
                    (Float(a), Char(b))     => Char(e!(a as u8 $symbol b as u8) as char)
                }
            }
        }
    }
}

impl_arith_ops!(add, ops::Add, +);
impl_arith_ops!(sub, ops::Sub, -);
impl_arith_ops!(div, ops::Div, /);
impl_arith_ops!(mul, ops::Mul, *);
impl_arith_ops!(rem, ops::Rem, %);

macro_rules! impl_bit_ops {
    ($name:ident, $path:path, $symbol:tt) => {
        impl $path for Atom {
            type Output = Atom;
            fn $name(self, other: Atom) -> Atom {
                unsafe {
                    match (self, other) {
                        // same type: no coercion
                        (SInt(a), SInt(b))      => SInt(e!(a $symbol b)),
                        (UInt(a), UInt(b))      => UInt(e!(a $symbol b)),
                        (Char(a), Char(b))      => Char(e!(a as u8 $symbol b as u8) as char),
                        // floats do not support bit ops, so type error
                        // (SVM should check this and provide more appropriate error
                        //  messages, so these panics should never actually happen)
                        (Float(_), _)     => panic!("floats do not support bit ops"),
                        (_, Float(_))     => panic!("floats do not support bit ops"),
                        (UInt(a), SInt(b))      => SInt(e!(transmute::<u64,i64>(a) $symbol b) ),
                        (SInt(a), UInt(b))      => SInt(e!(a $symbol transmute::<u64,i64>(b)) ),
                        _ => unimplemented!() // todo: figure out truncating to char
                    }
                }
            }
        }
    }
}

impl_bit_ops!(bitand, ops::BitAnd, &);
impl_bit_ops!(bitor, ops::BitOr, |);
impl_bit_ops!(bitxor, ops::BitXor, ^);

/// SVM instruction types.
///
/// Each SVM instruction will be described using operational
/// semantics through the use of the following notation:
///
///  + a state is written `(s, e, c, d)`
///  + `(x.y)` is `Cons(x, y)`. The empty list is `nil`.
///  + each instruction is described as a state transition `(s, e, c, d) → (s´, e´, c´, d´)`
#[derive(Debug,Copy,Clone,PartialEq)]
#[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
pub enum Inst {
    /// `nil`
    ///
    /// Pushes an empty list (nil) onto the stack.
    ///
    /// __Operational semantics__: `(s, e, (NIL.c), d) → ( (nil.s), e, c, d )`
    ///
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    NIL = 0x00,
    /// `ldc`: `L`oa`d` `C`onstant. Loads a constant (atom)
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    LDC = 0x1C,
    /// `ld`: `L`oa`d`. Pushes a variable onto the stack.
    ///
    /// The variable is indicated by the argument, a pair.
    /// The pair's `car` specifies the level, the `cdr` the position.
    /// So `(1 . 3)` gives the current function's (level 1) third
    /// parameter.
    ///
    /// __Operational semantics__: `(s, e, LDC.v.c, d) → (v.s, e, c, d)`
    ///
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    LD = 0x01,
    /// `ldf`: `L`oa`d` `F`unction.
    ///
    ///  Takes one list argument representing a function and constructs
    ///  a closure (a pair containing the function and the current
    ///  environment) and pushes that onto the stack.
    ///
    /// _Operational semantics_: `(s, e, (LDF f.c), d) → ( ([f e].s), e, c, d)`
    ///
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    LDF = 0x02,
    /// `join`
    ///
    /// Pops a list reference from the dump and makes thi64 the new value
    /// of `C`. This instruction occurs at the end of both alternatives of
    ///  a `sel`.
    ///
    /// __Operational semantics__: `(s, e, JOIN.c, c´.d) → (s, e, c´, d)`
    ///
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    JOIN = 0x05,
    /// `ap`: `Ap`ply.
    ///
    /// Pops a closure and a list of parameter values from the stack.
    /// The closure is applied to the parameters by installing its
    /// environment as the current one, pushing the parameter list
    /// in front of that, clearing the stack, and setting `c` to the
    /// closure's function pointer. The previous values of `s`, `e`,
    ///  and the next value of `c` are saved on the dump.
    ///
    /// __Operational semantics__: `(([f e´] v.s), e, (AP.c), d) → (nil, (v.e&prime), f, (s e c.d))`
    ///
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    AP = 0x03,
    /// `ret`: `Ret`urn.
    ///
    /// Pops one return value from the stack, restores
    /// `$s`, `$e`, and `$c` from the dump, and pushes
    /// the return value onto the now-current stack.
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    RET = 0x07,
    /// `dum`: `Dum`my.
    ///
    /// Pops a dummy environment (an empty list) onto the `$e` stack.
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    DUM = 0x08,
    /// `rap`: `R`ecursive `Ap`ply.
    /// Works like `ap`, only that it replaces an occurrence of a
    /// dummy environment with the current one, thus making recursive
    ///  functions possible.
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    RAP = 0x06,
    /// `sel`: `Sel`ect branch
    ///
    /// Expects two list arguments on the control stack, and pops a value
    /// from the stack. The first list is executed if the popped value
    /// was non-nil, the second list otherwise. Before one of these list
    /// pointers is made the new `$c`, a pointer to the instruction
    /// following `sel` is saved on the dump.
    ///
    /// __Operational semantics__: `(v.s, e, SEL.true.false.c, d) → (s, e, (if v then true else false), c.d)`
    ///
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    SEL = 0x09,
    /// `add`
    ///
    /// Pops two numbers off of the stack and adds them, pu64hing the
    /// result onto the stack. This will up-convert integers to floating
    /// point if necessary.
    ///
    /// TODO: figure out what happens when you try to add things that aren't
    /// numbers (maybe the compiler won't let this happen?).
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    ADD = 0x0A,
    /// `sub`: `Sub`tract
    ///
    /// Pops two numbers off of the stack and subtracts the first from the
    /// second, pu64hing the result onto the stack. This will up-convert
    /// integers to floating point if necessary.
    ///
    /// TODO: figure out what happens when you try to subtract things that
    /// aren't numbers (maybe the compiler won't let thi64 happen?).
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    SUB = 0x0B,
    /// `mul`: `Mul`tiply
    ///
    /// Pops two numbers off of the stack and multiplies them, pu64hing the
    /// result onto the stack. This will up-convert integers to floating
    /// point if necessary.
    ///
    /// TODO: figure out what happens when you try to multiply things that
    /// aren't numbers (maybe the compiler won't let thi64 happen?).
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    MUL = 0x0C,
    /// `div`: `Div`ide
    ///
    /// Pops two numbers off of the stack and divides the first by the second,
    /// pushing the result onto the stack. This performs integer divi64ion.
    ///
    /// TODO: figure out what happens when you try to divide things that
    /// aren't numbers (maybe the compiler won't let thi64 happen?).
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    DIV = 0x0D,
    /// `fdiv`: `F`loating-point `div`ide
    ///
    /// Pops two numbers off of the stack and divides the first by the second,
    /// pu64hing the result onto the stack. This performs float divi64ion.
    ///
    /// TODO: figure out what happens when you try to divide things that
    /// aren't numbers (maybe the compiler won't let this happen?).
    ///
    /// TODO: Not sure if there should be separate float and int divide words
    /// I guess the compiler can figure this out
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    FDIV = 0x0F,
    /// `mod`: `Mod`ulo
    ///
    /// Pops two numbers off of the stack and divides the first by the second,
    /// pushing the remainder onto the stack.
    ///
    /// TODO: figure out what happens when you try to modulo things that
    /// aren't numbers (maybe the compiler won't let this happen?).
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    MOD = 0x0E,
    /// `eq`: `Eq`uality of atoms
    #[cfg_attr(feature = "unstable",
    stable(feature="vm_core", since="0.1.0") )]
    EQ = 0x10,
    /// `gt`: `G`reater `t`han
    ///
    /// Pops two numbers on the stack and puts a 'true' on the stack
    /// if the first atom i64 greater than the other atom, false otherwi64e.
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    GT = 0x11,
    /// `gte`: `G`reater `t`han or `e`qual
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    GTE = 0x12,
    /// `lt`: `L`ess `t`han
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    LT = 0x13,
    /// `lte`: `L`ess `t`han or `e`qual
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    LTE = 0x14,
    /// `atom`: test if `atom`
    ///
    /// Pops an item from the stack and returns true if it's an atom, false
    /// otherwise.
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    ATOM = 0x15,
    /// `car`: `C`ontents of `A`ddress `R`egister
    ///
    /// Pops a list from the stack and returns the list's `car` (head)
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    CAR = 0x1A,
    /// `cdr`: `C`ontents of `D`ecrement `R`egister
    ///
    /// Pops a list from the stack and returns the list's `cdr` (tail)
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    CDR = 0x1B,
    /// `cons`: `Cons`truct
    ///
    /// Pops an item and a list from the stack and returns the list, with
    /// the item prepended.
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    CONS = 0x19,
    /// `null`: test if `null`
    ///
    ///  Pops an item from the stack and returns true if it is `nil`, false
    ///  otherwise.
    #[cfg_attr(feature = "unstable",
        stable(feature="vm_core", since="0.1.0") )]
    NULL = 0x16,
    /// `stop`: `stop` execution
    ///
    /// Terminates program execution. The `eval_program()` function will return
    /// the last state of the VM.
    #[cfg_attr(feature = "unstable",
        stable(feature = "vm_core", since="0.1.0") )]
    STOP = 0x1D,
    /// `readc`: `read` `c`haracter
    ///
    /// Reads a character from the machine's input stream and places it
    /// on top of the stack
    #[cfg_attr(feature = "unstable",
        stable(feature = "vm_core", since="0.1.0") )]
    READC = 0x17,
    /// `writec`: `write` `c`haracter
    ///
    /// Writes a character from the top of the stack to the machine's
    /// output stream.
    #[cfg_attr(feature = "unstable",
        stable(feature = "vm_core", since="0.1.0") )]
    WRITEC = 0x18,
    /// `apcc`: `ap`ply with `c`urrent `c`ontinuation
    ///
    /// Applies a closure and captures the continuation that can
    /// then be applied with `ap`.
    #[cfg_attr(feature = "unstable",
        unstable(feature = "callcc", issue = "69"))]
    APCC = 0x04,
}
