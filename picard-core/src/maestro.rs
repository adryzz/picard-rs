#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MaestroOpcode {
    MAESTRO_ADD = 0x00,
    MAESTRO_DP3,
    MAESTRO_DP4,
    MAESTRO_DPH,
    MAESTRO_DST,
    MAESTRO_EX2,
    MAESTRO_LG2,
    MAESTRO_LITP,
    MAESTRO_MUL,
    MAESTRO_SGE,
    MAESTRO_SLT,
    MAESTRO_FLR,
    MAESTRO_MAX,
    MAESTRO_MIN,
    MAESTRO_RCP,
    MAESTRO_RSQ,

    MAESTRO_UNK10 = 0x10,
    MAESTRO_UNK11,
    MAESTRO_MOVA,
    MAESTRO_MOV,
    MAESTRO_UNK14,
    MAESTRO_UNK15,
    MAESTRO_UNK16,
    MAESTRO_UNK17,
    MAESTRO_DPHI,
    MAESTRO_DSTI,
    MAESTRO_SGEI,
    MAESTRO_SLTI,
    MAESTRO_UNK1C,
    MAESTRO_UNK1D,
    MAESTRO_UNK1E,
    MAESTRO_UNK1F,

    MAESTRO_BREAK = 0x20,
    MAESTRO_NOP,
    MAESTRO_END,
    MAESTRO_BREAKC,
    MAESTRO_CALL,
    MAESTRO_CALLC,
    MAESTRO_CALLU,
    MAESTRO_IFU,
    MAESTRO_IFC,
    MAESTRO_FOR,
    MAESTRO_EMIT,      // Geometry shader related
    MAESTRO_SETEMIT,   // Geometry shader related
    MAESTRO_JMPC,
    MAESTRO_JMPU,
    MAESTRO_CMP = 0x38,  // only the upper 5 bits are used for the opcode

    // Only the upper 3 bits are used for the following opcodes
    MAESTRO_MADI = 0x30,
    MAESTRO_MAD,
}