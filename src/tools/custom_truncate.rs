pub fn truncate_and_return<T>(vec: &mut Vec<T>, len: usize) -> Vec<T> {
    vec.split_off(len)
}
