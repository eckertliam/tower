use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::I8(i) => write!(f, "{}", i),
            Value::I16(i) => write!(f, "{}", i),
            Value::I32(i) => write!(f, "{}", i),
            Value::I64(i) => write!(f, "{}", i),
            Value::U8(u) => write!(f, "{}", u),
            Value::U16(u) => write!(f, "{}", u),
            Value::U32(u) => write!(f, "{}", u),
            Value::U64(u) => write!(f, "{}", u),
            Value::F32(fl) => write!(f, "{}", fl),
            Value::F64(fl) => write!(f, "{}", fl),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (&self, &other) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a + b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a + b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a + b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a + b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a + b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a + b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a + b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a + b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a + b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a + b),
            _ => panic!("Cannot add types {} and {}", self, other),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a - b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a - b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a - b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a - b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a - b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a - b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a - b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a - b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a - b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a - b),
            _ => panic!("Cannot subtract types {} and {}", self, rhs),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a * b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a * b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a * b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a * b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a * b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a * b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a * b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a * b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a * b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a * b),
            _ => panic!("Cannot multiply types {} and {}", self, rhs),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a / b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a / b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a / b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a / b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a / b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a / b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a / b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a / b),
            (Value::F32(a), Value::F32(b)) => Value::F32(a / b),
            (Value::F64(a), Value::F64(b)) => Value::F64(a / b),
            _ => panic!("Cannot divide types {} and {}", self, rhs),
        }
    }
}

impl std::ops::BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a & b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a & b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a & b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a & b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a & b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a & b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a & b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a & b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a & b),
            _ => panic!("Cannot bitand types {} and {}", self, rhs),
        }
    }
}

impl std::ops::BitOr for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a | b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a | b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a | b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a | b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a | b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a | b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a | b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a | b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a | b),
            _ => panic!("Cannot bitor types {} and {}", self, rhs),
        }
    }
}

impl std::ops::BitXor for Value {
    type Output = Value;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a ^ b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a ^ b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a ^ b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a ^ b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a ^ b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a ^ b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a ^ b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a ^ b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a ^ b),
            _ => panic!("Cannot bitxor types {} and {}", self, rhs),
        }
    }
}

impl std::ops::Shr for Value {
    type Output = Value;

    fn shr(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a >> b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a >> b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a >> b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a >> b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a >> b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a >> b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a >> b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a >> b),
            _ => panic!("Cannot bitshift types {} and {}", self, rhs),
        }
    }
}

impl std::ops::Shl for Value {
    type Output = Value;

    fn shl(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::I8(a), Value::I8(b)) => Value::I8(a << b),
            (Value::I16(a), Value::I16(b)) => Value::I16(a << b),
            (Value::I32(a), Value::I32(b)) => Value::I32(a << b),
            (Value::I64(a), Value::I64(b)) => Value::I64(a << b),
            (Value::U8(a), Value::U8(b)) => Value::U8(a << b),
            (Value::U16(a), Value::U16(b)) => Value::U16(a << b),
            (Value::U32(a), Value::U32(b)) => Value::U32(a << b),
            (Value::U64(a), Value::U64(b)) => Value::U64(a << b),
            _ => panic!("Cannot bitshift types {} and {}", self, rhs),
        }
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        match self {
            Value::U8(u) => u,
            _ => panic!("Cannot convert {} into u8", self),
        }
    }
}

impl Into<u16> for Value {
    fn into(self) -> u16 {
        match self {
            Value::U16(u) => u,
            _ => panic!("Cannot convert {} into u16", self),
        }
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        match self {
            Value::U32(u) => u,
            _ => panic!("Cannot convert {} into u32", self),
        }
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        match self {
            Value::U64(u) => u,
            _ => panic!("Cannot convert {} into u64", self),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Bool(b)
    }
}

impl From<i8> for Value {
    fn from(i: i8) -> Value {
        Value::I8(i)
    }
}

impl From<i16> for Value {
    fn from(i: i16) -> Value {
        Value::I16(i)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Value {
        Value::I32(i)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Value {
        Value::I64(i)
    }
}

impl From<u8> for Value {
    fn from(u: u8) -> Value {
        Value::U8(u)
    }
}

impl From<u16> for Value {
    fn from(u: u16) -> Value {
        Value::U16(u)
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Value {
        Value::U32(u)
    }
}

impl From<u64> for Value {
    fn from(u: u64) -> Value {
        Value::U64(u)
    }
}


impl From<f32> for Value {
    fn from(f: f32) -> Value {
        Value::F32(f)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Value {
        Value::F64(f)
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i)= s.parse::<i8>() {
            Ok(Value::I8(i))
        } else if let Ok(i) = s.parse::<i16>() {
            Ok(Value::I16(i))
        } else if let Ok(i) = s.parse::<i32>() {
            Ok(Value::I32(i))
        } else if let Ok(i) = s.parse::<i64>() {
            Ok(Value::I64(i))
        } else if let Ok(u) = s.parse::<u8>() {
            Ok(Value::U8(u))
        } else if let Ok(u) = s.parse::<u16>() {
            Ok(Value::U16(u))
        } else if let Ok(u) = s.parse::<u32>() {
            Ok(Value::U32(u))
        } else if let Ok(u) = s.parse::<u64>() {
            Ok(Value::U64(u))
        } else if let Ok(f) = s.parse::<f32>() {
            Ok(Value::F32(f))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(Value::F64(f))
        } else if let Ok(b) = s.parse::<bool>() {
            Ok(Value::Bool(b))
        } else {
            Err(format!("Cannot parse {} into Value", s))
        }
    }
}