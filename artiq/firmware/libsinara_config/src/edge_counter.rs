use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Config: i32 {
    const CountRising = 1;
    const CountFalling = 2;
    const SendCountEvent = 4;
    const ResetToZero = 8;
    }
}
