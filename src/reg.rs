#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,

    Cmp,
    Link,
    StackPtr,
}

impl TryFrom<&str> for Reg {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = match value {
            "%a" => Self::A,
            "%b" => Self::B,
            "%c" => Self::C,
            "%d" => Self::D,
            "%e" => Self::E,
            "%f" => Self::F,
            "%g" => Self::G,
            "%h" => Self::H,
            "%i" => Self::I,
            "%j" => Self::J,
            "%cmp" => Self::Cmp,
            "%lnk" => Self::Link,
            "%sp" => Self::StackPtr,
            _ => return Err(format!("Invalid Register Name {}", value)),
        };

        return Ok(value);
    }
}

impl TryFrom<i32> for Reg {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value = match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::F,
            7 => Self::G,
            8 => Self::H,
            9 => Self::I,
            10 => Self::J,
            _ => return Err(format!("Invalid Register Number {}", value)),
        };

        return Ok(value);
    }
}

impl Into<String> for Reg {
    fn into(self) -> String {
        self.try_into().unwrap()
    }
}

impl Into<String> for &Reg {
    fn into(self) -> String {
        self.try_into().unwrap()
    }
}
