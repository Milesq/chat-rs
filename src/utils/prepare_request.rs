// Max is maximal value in one cell
pub fn prepare_metadata_request(len: usize, max: Option<u8>) -> [u8; 3] {
    let max = max.unwrap_or(std::u8::MAX) as usize;

    let multiplier = len / max;
    let modulo = len % max;

    [multiplier as u8, max as u8, modulo as u8]
}

#[cfg(tests)]
mod tests {
    #[test]
    pub fn test() {
        let req = &[1, 2, 3, 4, 5];
        assert_eq!(prepare_request(req.len(), 4), [1, 4, 1]);
    }
}
