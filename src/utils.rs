#[cfg(test)]
pub fn is_same_variant<T>(v1: &T, v2: &T) -> bool {
    use std::mem;

    mem::discriminant(v1) == mem::discriminant(v2)
}
