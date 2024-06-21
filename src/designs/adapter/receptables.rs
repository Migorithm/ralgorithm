// Receptacle is what the adapter will adapt to. It is the interface that the adapter will implement.

pub trait TReceptacle {
    const VOLT: u8;
    fn receive(&self, volt: u8) -> Result<(), &'static str> {
        if volt != Self::VOLT {
            return Err("Receptacle cannot receive the plug.");
        }
        println!("Receptacle received the plug.");
        Ok(())
    }
    fn bolt_to_accept(&self) -> u8 {
        Self::VOLT
    }
}

pub struct JapaneseReceptacle;

pub struct KoreanReceptacle;

impl TReceptacle for JapaneseReceptacle {
    const VOLT: u8 = 110;
}
impl TReceptacle for KoreanReceptacle {
    const VOLT: u8 = 220;
}
