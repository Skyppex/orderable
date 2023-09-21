pub trait Orderable {
    fn order(&self) -> u64;
    fn cmp_order(&self, other: &Self) -> std::cmp::Ordering {
        self.order().cmp(&other.order())
    }
}

pub trait BigOrderable {
    fn order(&self) -> u128;
    fn cmp_order(&self, other: &Self) -> std::cmp::Ordering {
        self.order().cmp(&other.order())
    }
}