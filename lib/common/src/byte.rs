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

    pub fn change(&mut self, alias: &str, value: bool) -> &Byte<'l> {
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
        assert_eq!(byte.number(), 0);

        byte.change("b1", true);
        assert_eq!(byte.number(), 1);

        byte.change("b2", true);
        assert_eq!(byte.number(), 3);

        byte.change("b3", true);
        assert_eq!(byte.number(), 7);

        byte.change("b4", true);
        assert_eq!(byte.number(), 15);

        byte.change("b5", true);
        assert_eq!(byte.number(), 31);

        byte.change("b6", true);
        assert_eq!(byte.number(), 63);

        byte.change("b7", true);
        assert_eq!(byte.number(), 127);

        byte.change("b8", true);
        assert_eq!(byte.number(), 255);

        byte.change("b1", false);
        assert_eq!(byte.number(), 254);

        byte.change("b2", false);
        assert_eq!(byte.number(), 252);

        byte.change("b3", false);
        assert_eq!(byte.number(), 248);

        byte.change("b4", false);
        assert_eq!(byte.number(), 240);

        byte.change("b5", false);
        assert_eq!(byte.number(), 224);

        byte.change("b6", false);
        assert_eq!(byte.number(), 192);

        byte.change("b7", false);
        assert_eq!(byte.number(), 128);

        byte.change("b8", false);
        assert_eq!(byte.number(), 0);

        byte.all().change("b4", false);
        assert_eq!(byte.number(), 240);

        byte.clean().change("b4", true);
        assert_eq!(byte.number(), 8);
    }
}
