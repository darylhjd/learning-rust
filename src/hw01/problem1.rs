/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    return slice.iter().fold(0, |acc, i| acc + i);
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    for i in vs {
        if !new_vec.contains(i) {
            new_vec.push(*i);
        }
    }
    return new_vec;
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut new_vec = vec![];
    for i in vs {
        if pred(*i) {
            new_vec.push(*i);
        }
    }
    return new_vec;
}