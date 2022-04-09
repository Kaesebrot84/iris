/// Return the mean value for a `Iterator<u8>`.
///
/// # Examples
/// 
/// ```
/// let data: Vec<u8> = vec![33, 13, 255, 0, 42];
/// let result = mean(data.into_iter());
/// assert_eq!(68, result);
/// ```
/// 
pub fn mean(iter: impl Iterator<Item = u8> + Clone) -> u8 {
    (iter.clone().map(|x| x as u64).sum::<u64>() / iter.count() as u64) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_ut() {
        let data: Vec<u8> = vec![33, 13, 255, 0, 42];
        let result = mean(data.into_iter());
        assert_eq!(68, result);
    }
}
