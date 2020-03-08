bitflags::bitflags! {
    #[derive(Default)]
    pub struct InputFlag : u64{
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;

        const DOWN = 1 << 4;
        const UP = 1 << 5;
        const RIGHT = 1 << 6;
        const LEFT = 1 << 7;

        const RIGHT_DOWN = 1 << 8;
        const LEFT_DOWN = 1 << 9;
        const RIGHT_UP = 1 << 10;
        const LEFT_UP = 1 << 11;
    }
}

impl std::fmt::Display for InputFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.contains(Self::UP) {
            write!(f, "8")?;
        }
        if self.contains(Self::DOWN) {
            write!(f, "2")?;
        }
        if self.contains(Self::RIGHT) {
            write!(f, "6")?;
        }
        if self.contains(Self::LEFT) {
            write!(f, "4")?;
        }
        if self.contains(Self::RIGHT_UP) {
            write!(f, "9")?;
        }
        if self.contains(Self::RIGHT_DOWN) {
            write!(f, "3")?;
        }
        if self.contains(Self::LEFT_UP) {
            write!(f, "7")?;
        }
        if self.contains(Self::LEFT_DOWN) {
            write!(f, "1")?;
        }
        if self.contains(Self::A) {
            write!(f, "A")?;
        }
        if self.contains(Self::B) {
            write!(f, "B")?;
        }
        if self.contains(Self::C) {
            write!(f, "C")?;
        }
        if self.contains(Self::D) {
            write!(f, "D")?;
        }
        write!(f, "")
    }
}
