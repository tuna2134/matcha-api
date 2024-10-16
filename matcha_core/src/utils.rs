pub fn intersperse(slice: Vec<i64>, item: i64) -> Vec<i64> {
    let mut result = Vec::with_capacity(slice.len() * 2 + 1);
    for val in slice.iter() {
        result.push(item);
        result.push(*val);
    }
    result.push(item);
    result
}
