use std::{mem::transmute, convert};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    Integer,
    Float,
    Bool,
    Byte,
    Pointer,
    Null,
}

#[repr(packed(1))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Constant {
    raw: u64,
    _type: Type,
}

impl Constant {
    pub fn null() -> Self {
        Self {
            raw: 0,
            _type: Type::Null,
        }
    }


    //unwrap a constant as T where T can be converted from Constant
    pub fn unwrap<'a, T : convert::From<&'a mut Constant>>(&'a mut self) -> T {
        Into::<T>::into(self)
    }
}

impl From<i64> for Constant {
    fn from(value: i64) -> Self {
        Self {
            raw: value as u64,
            _type: Type::Integer,
        }
    }
}

impl From<i32> for Constant {
    fn from(value: i32) -> Self {
        Self {
            raw: value as u64,
            _type: Type::Integer,
        }
    }
}

impl From<i16> for Constant {
    fn from(value: i16) -> Self {
        Self {
            raw: value as u64,
            _type: Type::Integer,
        }
    }
}

impl From<i8> for Constant {
    fn from(value: i8) -> Self {
        Self {
            raw: value as u64,
            _type: Type::Integer,
        }
    }
}

impl From<usize> for Constant {
    fn from(value: usize) -> Self {
        Self {
            raw: value as u64,
            _type: Type::Pointer,
        }
    }
}

impl From<u64> for Constant {
    fn from(value: u64) -> Self {
        Self {
            raw: value,
            _type: Type::Pointer,
        }
    }
}

impl From<u8> for Constant {
    fn from(value: u8) -> Self {
        Self {
            raw: value as u64,
            _type: Type::Byte,
        }
    }
}

impl From<bool> for Constant {
    fn from(value: bool) -> Self {
        if value {
            Self {
                raw: 1,
                _type: Type::Bool,
            }
        }else{
            Self {
                raw: 0,
                _type: Type::Bool,
            }
        }
    }
}

impl From<f64> for Constant {
    fn from(value: f64) -> Self {
        Self {
            raw: unsafe {
                transmute(value)
            },
            _type: Type::Float,
        }
    }
}

impl From<f32> for Constant {
    fn from(value: f32) -> Self {
        Self::from(value as f64)
    }
}

impl Into<i64> for Constant {
    fn into(self) -> i64 {
        self.raw as i64
    }
}

impl Into<f64> for Constant {
    fn into(self) -> f64 {
        unsafe {
            transmute(self.raw)
        }
    }
}

impl Into<bool> for Constant {
    fn into(self) -> bool {
        self.raw > 0
    }
}

impl Into<u8> for Constant {
    fn into(self) -> u8 {
        self.raw as u8
    }
}

impl Into<u64> for Constant {
    fn into(self) -> u64 {
        self.raw as u64
    }
}

impl Into<usize> for Constant {
    fn into(self) -> usize {
        self.raw as usize
    }
}

impl std::ops::Add for Constant {
    type Output = Result<Constant, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs + rhs).into())
            },
            (Type::Float, Type::Float) => {
                let lhs: f64 = self.into();
                let rhs: f64 = rhs.into();
                Ok((lhs + rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw + rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs + rhs).into())
            },
            _ => Err(format!("Cannot add types {:?} and {:?}", self._type, rhs._type)), 
        }
    }
}

impl std::ops::Sub for Constant {
    type Output = Result<Constant, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs - rhs).into())
            },
            (Type::Float, Type::Float) => {
                let lhs: f64 = self.into();
                let rhs: f64 = rhs.into();
                Ok((lhs - rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw - rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs - rhs).into())
            },
            _ => Err(format!("Cannot subtract types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}

impl std::ops::Mul for Constant {
    type Output = Result<Constant, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs * rhs).into())
            },
            (Type::Float, Type::Float) => {
                let lhs: f64 = self.into();
                let rhs: f64 = rhs.into();
                Ok((lhs * rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw * rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs * rhs).into())
            },
            _ => Err(format!("Cannot multiply types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}

impl std::ops::Div for Constant {
    type Output = Result<Constant, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs / rhs).into())
            },
            (Type::Float, Type::Float) => {
                let lhs: f64 = self.into();
                let rhs: f64 = rhs.into();
                Ok((lhs / rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw / rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs / rhs).into())
            },
            _ => Err(format!("Cannot divide types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}

impl std::ops::Rem for Constant {
    type Output = Result<Constant, String>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs % rhs).into())
            },
            (Type::Float, Type::Float) => {
                let lhs: f64 = self.into();
                let rhs: f64 = rhs.into();
                Ok((lhs % rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw % rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs % rhs).into())
            },
            _ => Err(format!("Cannot modulo types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}


impl std::ops::BitAnd for Constant {
    type Output = Result<Constant, String>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs & rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw & rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs & rhs).into())
            },
            (Type::Bool, Type::Bool) => {
                let lhs: bool = self.into();
                let rhs: bool = rhs.into();
                Ok((lhs & rhs).into())
            },
            _ => Err(format!("Cannot bitwise and types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}

impl std::ops::BitOr for Constant {
    type Output = Result<Constant, String>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs | rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw | rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs | rhs).into())
            },
            (Type::Bool, Type::Bool) => {
                let lhs: bool = self.into();
                let rhs: bool = rhs.into();
                Ok((lhs | rhs).into())
            },
            _ => Err(format!("Cannot bitwise or types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}

impl std::ops::BitXor for Constant {
    type Output = Result<Constant, String>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs ^ rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw ^ rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs ^ rhs).into())
            },
            (Type::Bool, Type::Bool) => {
                let lhs: bool = self.into();
                let rhs: bool = rhs.into();
                Ok((lhs ^ rhs).into())
            },
            _ => Err(format!("Cannot bitwise xor types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}

impl std::ops::Shl for Constant {
    type Output = Result<Constant, String>;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs << rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw << rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs << rhs).into())
            },
            _ => Err(format!("Cannot shift left types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}

impl std::ops::Shr for Constant {
    type Output = Result<Constant, String>;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self._type, rhs._type) {
            (Type::Integer, Type::Integer) => {
                let lhs: i64 = self.into();
                let rhs: i64 = rhs.into();
                Ok((lhs >> rhs).into())
            },
            (Type::Pointer, Type::Pointer) => {
                Ok((self.raw >> rhs.raw).into())
            },
            (Type::Byte, Type::Byte) => {
                let lhs: u8 = self.into();
                let rhs: u8 = rhs.into();
                Ok((lhs >> rhs).into())
            },
            _ => Err(format!("Cannot shift right types {:?} and {:?}", self._type, rhs._type)),
        }
    }
}
