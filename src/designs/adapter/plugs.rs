// plug is what we want to put to the receptacle. It is the interface that the adapter will take as dependency.
pub trait TPlug {
    fn plug(&self) -> u8;
}

pub struct JapanesePlug;

impl TPlug for JapanesePlug {
    fn plug(&self) -> u8 {
        110
    }
}

pub struct KoreanPlug;

impl TPlug for KoreanPlug {
    fn plug(&self) -> u8 {
        220
    }
}
