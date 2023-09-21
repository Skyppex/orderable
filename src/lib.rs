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