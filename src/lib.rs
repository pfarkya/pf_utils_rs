/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg1 = 5;
/// let arg2 = 1;
/// let answer = pf_utils_rs::add(arg1, arg2);
///
/// assert_eq!(6, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
