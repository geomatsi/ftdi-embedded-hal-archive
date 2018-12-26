/* MPSSE commands */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MPSSECmd {
    MSB_RISING_EDGE_CLK_BYTE_OUT,
    MSB_FALLING_EDGE_CLK_BYTE_OUT,
    MSB_RISING_EDGE_CLK_BIT_OUT,
    MSB_FALLING_EDGE_CLK_BIT_OUT,

    MSB_RISING_EDGE_CLK_BYTE_IN,
    MSB_FALLING_EDGE_CLK_BYTE_IN,
    MSB_RISING_EDGE_CLK_BIT_IN,
    MSB_FALLING_EDGE_CLK_BIT_IN,

    SET_BITS_LOW,
    SET_BITS_HIGH,
    GET_BITS_LOW,
    GET_BITS_HIGH,

    LOOPBACK_ENABLE,
    LOOPBACK_DISABLE,
    TCK_DIVISOR,
    SEND_IMMEDIATE_RESP,
}

impl Into<u8> for MPSSECmd {
    fn into(self) -> u8 {
        let cmd = match self {
            MPSSECmd::MSB_RISING_EDGE_CLK_BYTE_OUT => 0x10,
            MPSSECmd::MSB_FALLING_EDGE_CLK_BYTE_OUT => 0x11,
            MPSSECmd::MSB_RISING_EDGE_CLK_BIT_OUT => 0x12,
            MPSSECmd::MSB_FALLING_EDGE_CLK_BIT_OUT => 0x13,

            MPSSECmd::MSB_RISING_EDGE_CLK_BYTE_IN => 0x20,
            MPSSECmd::MSB_FALLING_EDGE_CLK_BYTE_IN => 0x24,
            MPSSECmd::MSB_RISING_EDGE_CLK_BIT_IN => 0x22,
            MPSSECmd::MSB_FALLING_EDGE_CLK_BIT_IN => 0x26,

            MPSSECmd::SET_BITS_LOW => 0x80,
            MPSSECmd::SET_BITS_HIGH => 0x82,
            MPSSECmd::GET_BITS_LOW => 0x81,
            MPSSECmd::GET_BITS_HIGH => 0x83,

            MPSSECmd::LOOPBACK_ENABLE => 0x84,
            MPSSECmd::LOOPBACK_DISABLE => 0x85,
            MPSSECmd::TCK_DIVISOR => 0x86,
            MPSSECmd::SEND_IMMEDIATE_RESP => 0x87,
        };

        cmd as u8
    }
}

/* H Type specific MPSSE commands */
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum MPSSECmd_H {
    DISABLE_DIV_5_CLK,
    ENABLE_DIV_5_CLK,
    ENABLE_3_PHASE_CLK,
    DISABLE_3_PHASE_CLK,
    CLK_BITS,
    CLK_BYTES,
    CLK_WAIT_HIGH,
    CLK_WAIT_LOW,
    ENABLE_ADAPTIVE_CLK,
    DISABLE_ADAPTIVE_CLK,
    CLK_BYTES_OR_HIGH,
    CLK_BYTES_OR_LOW,
}

impl Into<u8> for MPSSECmd_H {
    fn into(self) -> u8 {
        let cmd = match self {
            MPSSECmd_H::DISABLE_DIV_5_CLK => 0x8a,
            MPSSECmd_H::ENABLE_DIV_5_CLK => 0x8b,
            MPSSECmd_H::ENABLE_3_PHASE_CLK => 0x8c,
            MPSSECmd_H::DISABLE_3_PHASE_CLK => 0x8d,
            MPSSECmd_H::CLK_BITS => 0x8e,
            MPSSECmd_H::CLK_BYTES => 0x8f,
            MPSSECmd_H::CLK_WAIT_HIGH => 0x94,
            MPSSECmd_H::CLK_WAIT_LOW => 0x95,
            MPSSECmd_H::ENABLE_ADAPTIVE_CLK => 0x96,
            MPSSECmd_H::DISABLE_ADAPTIVE_CLK => 0x97,
            MPSSECmd_H::CLK_BYTES_OR_HIGH => 0x9c,
            MPSSECmd_H::CLK_BYTES_OR_LOW => 0x9d,
        };

        cmd as u8
    }
}
