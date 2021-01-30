#[derive(Copy, Clone, Debug)]
pub struct Byte<'l> {
    bit: u8,
    alias: [&'l str; 8],
}

impl<'l> Byte<'l> {
    pub fn default() -> Byte<'l> {
        Byte {
            bit: 0b0000_0000,
            alias: ["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"],
        }
    }
    pub fn new(alias: [&'l str; 8]) -> Byte<'l> {
        Byte {
            bit: 0b0000_0000,
            alias,
        }
    }

    pub fn number(&self) -> u8 {
        self.bit
    }

    pub fn change(&mut self, alias: &str, value: bool) -> &mut Byte<'l> {
        match self.alias.iter().position(|&s| s == alias) {
            Some(i) => match i {
                0..=7 => match value {
                    true => self.bit |= 0b0000_0001 << i,
                    false => self.bit &= !0b0000_0001 << i,
                },
                _ => self.bit = 0b0000_0000,
            },
            None => {}
        };

        self
    }

    pub fn turn_on(&mut self, alias: &str) -> &mut Byte<'l> {
        self.change(alias, true)
    }

    pub fn turn_off(&mut self, alias: &str) -> &mut Byte<'l> {
        self.change(alias, false)
    }

    pub fn all(&mut self) -> &mut Byte<'l> {
        self.bit = 0b1111_1111;
        self
    }

    pub fn clean(&mut self) -> &mut Byte<'l> {
        self.bit = 0b0000_0000;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Byte;

    #[test]
    fn it_works() {
        let mut byte = Byte::default();

        // Inicial
        assert_eq!(byte.number(), 0);

        // On
        assert_eq!(byte.turn_on("b1").number(), 1);
        assert_eq!(byte.turn_on("b2").number(), 3);
        assert_eq!(byte.turn_on("b3").number(), 7);
        assert_eq!(byte.turn_on("b4").number(), 15);
        assert_eq!(byte.turn_on("b5").number(), 31);
        assert_eq!(byte.turn_on("b6").number(), 63);
        assert_eq!(byte.turn_on("b7").number(), 127);
        assert_eq!(byte.turn_on("b8").number(), 255);

        // Off
        assert_eq!(byte.turn_off("b1").number(), 254);
        assert_eq!(byte.turn_off("b2").number(), 252);
        assert_eq!(byte.turn_off("b3").number(), 248);
        assert_eq!(byte.turn_off("b4").number(), 240);
        assert_eq!(byte.turn_off("b5").number(), 224);
        assert_eq!(byte.turn_off("b6").number(), 192);
        assert_eq!(byte.turn_off("b7").number(), 128);
        assert_eq!(byte.turn_off("b8").number(), 0);

        // Todos / Nenhum
        assert_eq!(byte.all().number(), 255);
        assert_eq!(byte.clean().number(), 0);
    }
}
