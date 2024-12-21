use std::fmt;

// Operations
pub(crate) enum Op {
    Mov,
    Add,
    Sub,
    Cmp,
    Je,
    Jl,
    Jle,
    Jb,
    Jbe,
    Jp,
    Jo,
    Js,
    Jne,
    Jnl,
    Jg,
    Jnb,
    Ja,
    Jnp,
    Jno,
    Jns,
    Loop,
    Loopz,
    Loopnz,
    Jcxz,
    Invalid,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Op::Mov => "mov",
            Op::Add => "add",
            Op::Sub => "sub",
            Op::Cmp => "cmp",
            Op::Je => "je",
            Op::Jl => "jl",
            Op::Jle => "jle",
            Op::Jb => "jb",
            Op::Jbe => "jbe",
            Op::Jp => "jp",
            Op::Jo => "jo",
            Op::Js => "js",
            Op::Jne => "jne",
            Op::Jnl => "jnl",
            Op::Jg => "jg",
            Op::Jnb => "jnb",
            Op::Ja => "ja",
            Op::Jnp => "jnp",
            Op::Jno => "jno",
            Op::Jns => "jns",
            Op::Loop => "loop",
            Op::Loopz => "loopz",
            Op::Loopnz => "loopnz",
            Op::Jcxz => "jcxz",
            Op::Invalid => "",
        };

        write!(f, "{}", value)
    }
}

// Instructions
pub(crate) enum Instruction {
    // MOV Instructions
    MovRegisterMemoryToFromRegister,
    MovImmediateToRegisterMemory,
    MovImmediateToRegister,
    MovMemoryToAccumulator,
    MovAccumulatorToMemory,
    MovRegisterMemoryToSegmentRegister,
    MovSegmentRegisterToRegisterMemory,

    // ADD Instructions
    AddRegisterMemoryWithRegisterToEither,
    AddImmediateToRegisterMemory,
    AddImmediateToAccumulator,

    // SUB Instructions
    SubRegisterMemoryAndRegisterToEither,
    SubImmediateFromRegisterMemory,
    SubImmediateFromAccumulator,

    // CMP Instructions
    CmpRegisterMemoryAndRegister,
    CmpImmediateWithRegisterMemory,
    CmpImmediateWithAccumulator,

    // Jump Instructions
    JumpOnEqual,
    JumpOnLess,
    JumpOnLessOrEqual,
    JumpOnBelow,
    JumpOnBelowOrEqual,
    JumpOnParity,
    JumpOnOverflow,
    JumpOnSign,
    JumpOnNotEqual,
    JumpOnNotLess,
    JumpOnGreater,
    JumpOnNotBelow,
    JumpOnAbove,
    JumpOnNotPar,
    JumpOnNotOverflow,
    JumpOnNotSign,
    JumpOnCxZero,

    // Loop Instructions
    LoopCxTimes,
    LoopWhileZero,
    LoopWhileNotZero,

    // _
    Invalid,
}

// Addresses
pub(crate) enum EffectiveAddressCalculation {
    AL,   // 000
    CL,   // 001
    DL,   // 010
    BL,   // 011
    AH,   // 100
    CH,   // 101
    DH,   // 110
    BH,   // 111
    AX,   // 000
    CX,   // 001
    DX,   // 010
    BX,   // 011
    SP,   // 100
    BP,   // 101
    SI,   // 110
    DI,   // 111
    BxSi, // 000
    BxDi, // 001
    BpSi, // 010
    BpDi, // 011
    Si,   // 100
    Di,   // 101
    Bp,   // 110
    Bx,   // 111
}

impl fmt::Display for EffectiveAddressCalculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            EffectiveAddressCalculation::AL => "al",
            EffectiveAddressCalculation::CL => "cl",
            EffectiveAddressCalculation::DL => "dl",
            EffectiveAddressCalculation::BL => "bl",
            EffectiveAddressCalculation::AH => "ah",
            EffectiveAddressCalculation::CH => "ch",
            EffectiveAddressCalculation::DH => "dh",
            EffectiveAddressCalculation::BH => "bh",
            EffectiveAddressCalculation::AX => "ax",
            EffectiveAddressCalculation::CX => "cx",
            EffectiveAddressCalculation::DX => "dx",
            EffectiveAddressCalculation::BX => "bx",
            EffectiveAddressCalculation::SP => "sp",
            EffectiveAddressCalculation::BP => "bp",
            EffectiveAddressCalculation::SI => "si",
            EffectiveAddressCalculation::DI => "di",
            EffectiveAddressCalculation::BxSi => "bx + si",
            EffectiveAddressCalculation::BxDi => "bx + di",
            EffectiveAddressCalculation::BpSi => "bp + si",
            EffectiveAddressCalculation::BpDi => "bp + di",
            EffectiveAddressCalculation::Si => "si",
            EffectiveAddressCalculation::Di => "di",
            EffectiveAddressCalculation::Bp => "bp",
            EffectiveAddressCalculation::Bx => "bx",
        };

        write!(f, "{}", value)
    }
}
