use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum KasliPort {
    Eem0 = 7,
    Eem1 = 5,
    Eem2 = 4,
    Eem3 = 3,
    Eem4 = 2,
    Eem5 = 1,
    Eem6 = 0,
    Eem7 = 6,
    Eem8 = 12,
    Eem9 = 13,
    Eem10 = 15,
    Eem11 = 14,
    Sfp0 = 8,
    Sfp1 = 9,
    Sfp2 = 10,
    Loc0 = 11,
}

impl<'a> TryFrom<&'a str> for KasliPort {
    type Error = InvalidKasliPort<'a>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "EEM0" => Ok(Self::Eem0),
            "EEM1" => Ok(Self::Eem1),
            "EEM2" => Ok(Self::Eem2),
            "EEM3" => Ok(Self::Eem3),
            "EEM4" => Ok(Self::Eem4),
            "EEM5" => Ok(Self::Eem5),
            "EEM6" => Ok(Self::Eem6),
            "EEM7" => Ok(Self::Eem7),
            "EEM8" => Ok(Self::Eem8),
            "EEM9" => Ok(Self::Eem9),
            "EEM10" => Ok(Self::Eem10),
            "EEM11" => Ok(Self::Eem11),
            "SFP0" => Ok(Self::Sfp0),
            "SFP1" => Ok(Self::Sfp1),
            "SFP2" => Ok(Self::Sfp2),
            "LOC0" => Ok(Self::Loc0),
            _ => Err(InvalidKasliPort(s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvalidKasliPort<'a>(&'a str);

impl<'a> fmt::Display for InvalidKasliPort<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid Kasli I2C port: {}", self.0)
    }
}
