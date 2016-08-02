use int::Int;

pub fn popcnt<T: Int>(x: T) -> T {
    x.count_ones() // TODO: software emulation
}

pub fn lzcnt<T: Int>(x: T) -> T {
    x.leading_zeros() // TODO: software emulation
}
