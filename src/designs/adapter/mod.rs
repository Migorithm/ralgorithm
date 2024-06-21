pub mod plugs;
use plugs::*;
pub mod receptables;
use receptables::*;

struct PowerAdapter<T: TPlug> {
    port: T,
}

impl<T> PowerAdapter<T>
where
    T: TPlug,
{
    fn new(port: T) -> Self {
        PowerAdapter { port }
    }
    fn convert<U: TPlug>(self, port: U) -> PowerAdapter<U> {
        PowerAdapter { port }
    }
    fn convert_bolt(&self) -> u8 {
        if self.port.plug() == 110 {
            220
        } else {
            110
        }
    }

    fn connect(&self, receptacle: impl TReceptacle) -> Result<(), &'static str> {
        let volt = self.port.plug();
        if volt == receptacle.bolt_to_accept() {
            receptacle.receive(volt)
        } else {
            let volt = self.convert_bolt();
            receptacle.receive(volt)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_japanese_plug_to_korean_receptacle() {
        let adapter = PowerAdapter::new(JapanesePlug);
        let result = adapter.connect(KoreanReceptacle);
        assert!(result.is_ok());
    }

    #[test]
    fn test_korean_plug_to_japanese_receptacle() {
        let adapter = PowerAdapter::new(KoreanPlug);
        let result = adapter.connect(JapaneseReceptacle);
        assert!(result.is_ok());
    }

    #[test]
    fn test_korean_plug_to_korean_receptacle() {
        let adapter = PowerAdapter::new(KoreanPlug);
        let result = adapter.connect(KoreanReceptacle);
        assert!(result.is_ok());
    }
}
