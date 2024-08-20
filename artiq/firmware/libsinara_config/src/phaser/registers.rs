use num_enum::IntoPrimitive;

/// Compatible Phaser board identifier.
pub const BOARD_ID: u8 = 19;

#[derive(Debug, IntoPrimitive)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Address {
    BoardId = 0x00,
    HwRev = 0x01,
    GwRev = 0x02,
    Cfg = 0x03,
    Sta = 0x04,
    CrcErr = 0x05,
    Led = 0x06,
    Fan = 0x07,
    DucStb = 0x08,
    AdcCfg = 0x09,
    SpiCfg = 0x0a,
    SpiDivLen = 0x0b,
    SpiSel = 0x0c,
    SpiDatW = 0x0d,
    SpiDatR = 0x0e,
    SyncDly = 0x0f,
    Duc0Cfg = 0x10,
    // 0x11: reserved
    Duc0F = 0x12,
    Duc0P = 0x16,
    Dac0Data = 0x18,
    Dac0Test = 0x1c,
    Duc1Cfg = 0x20,
    // 0x21: reserved
    Duc1F = 0x22,
    Duc1P = 0x26,
    Dac1Data = 0x28,
    Dac1Test = 0x2c,
    ServoCfg0 = 0x30,
    ServoCfg1 = 0x31,
    ServoDataBase = 0x32, // servo coefficients + offset data: 0x32 - 0x71
    MiqroMemAddr = 0x72,  // miqro channel profile/window memories: 0x72 - 0x78
    MiqroMemData = 0x74,
}
