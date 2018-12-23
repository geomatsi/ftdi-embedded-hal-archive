/* MPSSE commands */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MPSSECmd {
    MSB_BYTES_RW_CPOL_0_CPHA_0,
    MSB_BYTES_RW_CPOL_1_CPHA_0,
    MSB_BYTES_R_CPOL_0_CPHA_0,
    MSB_BYTES_R_CPOL_1_CPHA_0,
    MSB_BYTES_W_CPOL_0_CPHA_0,
    MSB_BYTES_W_CPOL_1_CPHA_0,

    MSB_BYTES_W_FALLING,
    MSB_BYTES_R_FALLING,

    MSB_BITS_W_RISING,
    MSB_BITS_W_FALLING,
    MSB_BITS_R_RISING,
    MSB_BITS_R_FALLING,

    SET_BITS_LOW,
    SET_BITS_HIGH,
    GET_BITS_LOW,
    GET_BITS_HIGH,

    LOOPBACK_START,
    LOOPBACK_END,

    TCK_DIVISOR,

    SEND_BACK_NOW,
}

impl Into<u8> for MPSSECmd {
    fn into(self) -> u8 {
        let cmd = match self {
            MPSSECmd::MSB_BYTES_RW_CPOL_0_CPHA_0 => 0x31,
            MPSSECmd::MSB_BYTES_RW_CPOL_1_CPHA_0 => 0x32,
            MPSSECmd::MSB_BYTES_R_CPOL_0_CPHA_0 => 0x20,
            MPSSECmd::MSB_BYTES_R_CPOL_1_CPHA_0 => 0x24,
            MPSSECmd::MSB_BYTES_W_CPOL_0_CPHA_0 => 0x11,
            MPSSECmd::MSB_BYTES_W_CPOL_1_CPHA_0 => 0x10,

            MPSSECmd::MSB_BYTES_W_FALLING => 0x11,
            MPSSECmd::MSB_BYTES_R_FALLING => 0x24,

            MPSSECmd::MSB_BITS_W_RISING => 0x12,
            MPSSECmd::MSB_BITS_W_FALLING => 0x13,
            MPSSECmd::MSB_BITS_R_RISING => 0x22,
            MPSSECmd::MSB_BITS_R_FALLING => 0x26,

            MPSSECmd::SET_BITS_LOW => 0x80,
            MPSSECmd::SET_BITS_HIGH => 0x82,
            MPSSECmd::GET_BITS_LOW => 0x81,
            MPSSECmd::GET_BITS_HIGH => 0x83,

            MPSSECmd::LOOPBACK_START => 0x84,
            MPSSECmd::LOOPBACK_END => 0x85,

            MPSSECmd::TCK_DIVISOR => 0x86,

            MPSSECmd::SEND_BACK_NOW => 0x87,
        };

        cmd as u8
    }
}

/* H Type specific MPSSE commands */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MPSSECmd_H {
    DIS_DIV_5,
    EN_DIV_5,
    EN_3_PHASE,
    DIS_3_PHASE,
    CLK_BITS,
    CLK_BYTES,
    CLK_WAIT_HIGH,
    CLK_WAIT_LOW,
    EN_ADAPTIVE,
    DIS_ADAPTIVE,
    CLK_BYTES_OR_HIGH,
    CLK_BYTES_OR_LOW,
}

impl Into<u8> for MPSSECmd_H {
    fn into(self) -> u8 {
        let cmd = match self {
            MPSSECmd_H::DIS_DIV_5 => 0x8a,
            MPSSECmd_H::EN_DIV_5 => 0x8b,
            MPSSECmd_H::EN_3_PHASE => 0x8c,
            MPSSECmd_H::DIS_3_PHASE => 0x8d,
            MPSSECmd_H::CLK_BITS => 0x8e,
            MPSSECmd_H::CLK_BYTES => 0x8f,
            MPSSECmd_H::CLK_WAIT_HIGH => 0x94,
            MPSSECmd_H::CLK_WAIT_LOW => 0x95,
            MPSSECmd_H::EN_ADAPTIVE => 0x96,
            MPSSECmd_H::DIS_ADAPTIVE => 0x97,
            MPSSECmd_H::CLK_BYTES_OR_HIGH => 0x9c,
            MPSSECmd_H::CLK_BYTES_OR_LOW => 0x9d,
        };

        cmd as u8
    }
}
