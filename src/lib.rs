pub trait Orderable {
    fn get_order(&self) -> u64;
    fn cmp_order(&self, other: &Self) -> std::cmp::Ordering {
        self.get_order().cmp(&other.get_order())
    }
}

pub trait BigOrderable {
    fn get_order(&self) -> u128;
    fn cmp_order(&self, other: &Self) -> std::cmp::Ordering {
        self.get_order().cmp(&other.get_order())
    }
}

const I8_SHIFT: i8 = 2i8.pow(6) as i8;
const I16_SHIFT: i16 = 2i16.pow(14) as i16;
const I32_SHIFT: i32 = 2i32.pow(30) as i32;
const I64_SHIFT: i64 = 2i64.pow(62) as i64;
const I128_SHIFT: i128 = 2i128.pow(126) as i128;

impl Orderable for u8 {
    fn get_order(&self) -> u64 {
        *self as u64
    }
}

impl BigOrderable for u8 {
    fn get_order(&self) -> u128 {
        *self as u128
    }
}

impl Orderable for u16 {
    fn get_order(&self) -> u64 {
        *self as u64
    }
}

impl BigOrderable for u16 {
    fn get_order(&self) -> u128 {
        *self as u128
    }
}

impl Orderable for u32 {
    fn get_order(&self) -> u64 {
        *self as u64
    }
}

impl BigOrderable for u32 {
    fn get_order(&self) -> u128 {
        *self as u128
    }
}

impl Orderable for u64 {
    fn get_order(&self) -> u64 {
        *self
    }
}

impl BigOrderable for u64 {
    fn get_order(&self) -> u128 {
        *self as u128
    }
}

impl BigOrderable for u128 {
    fn get_order(&self) -> u128 {
        *self
    }
}

impl Orderable for i8 {
    fn get_order(&self) -> u64 {
        self.wrapping_add(I8_SHIFT).wrapping_add(I8_SHIFT) as u64
    }
}

impl BigOrderable for i8 {
    fn get_order(&self) -> u128 {
        self.wrapping_add(I8_SHIFT).wrapping_add(I8_SHIFT) as u128
    }
}

impl Orderable for i16 {
    fn get_order(&self) -> u64 {
        self.wrapping_add(I16_SHIFT).wrapping_add(I16_SHIFT) as u64
    }
}

impl BigOrderable for i16 {
    fn get_order(&self) -> u128 {
        self.wrapping_add(I16_SHIFT).wrapping_add(I16_SHIFT) as u128
    }
}

impl Orderable for i32 {
    fn get_order(&self) -> u64 {
        self.wrapping_add(I32_SHIFT).wrapping_add(I32_SHIFT) as u64
    }
}

impl BigOrderable for i32 {
    fn get_order(&self) -> u128 {
        self.wrapping_add(I32_SHIFT).wrapping_add(I32_SHIFT) as u128
    }
}

impl Orderable for i64 {
    fn get_order(&self) -> u64 {
        self.wrapping_add(I64_SHIFT).wrapping_add(I64_SHIFT) as u64
    }
}

impl BigOrderable for i64 {
    fn get_order(&self) -> u128 {
        self.wrapping_add(I64_SHIFT).wrapping_add(I64_SHIFT) as u128
    }
}

impl BigOrderable for i128 {
    fn get_order(&self) -> u128 {
        self.wrapping_add(I128_SHIFT).wrapping_add(I128_SHIFT) as u128
    }
}
