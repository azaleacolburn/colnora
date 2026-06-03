use crate::reg::Reg;

#[derive(Debug, Clone)]
pub enum Value {
    Lit(i32),
    Reg(Reg),
    Data(String),
    Deref(Box<Value>),
}

impl TryFrom<&str> for Value {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = if value.starts_with("[") {
            let inner = Value::try_from(&value[1..value.len() - 1])?;
            assert!(value.chars().last().unwrap() == ']');
            Value::Deref(Box::new(inner))
        } else if let Ok(reg) = Reg::try_from(value) {
            Value::Reg(reg)
        } else if let Ok(num) = value.parse::<i32>() {
            Value::Lit(num)
        } else {
            Value::Data(value.to_string())
        };

        Ok(value)
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        self.try_into().unwrap()
    }
}

impl Into<String> for &Value {
    fn into(self) -> String {
        self.try_into().unwrap()
    }
}
