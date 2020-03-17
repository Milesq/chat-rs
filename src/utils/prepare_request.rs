fn ceil(value: f64, scale: u8) -> f64 {
    let multiplier = 10i64.pow(scale as u32) as f64;
    (value * multiplier).ceil() / multiplier
}

// Max is maximal value in one cell
pub fn prepare_request(req: &[u8], max: Option<u8>) -> Vec<u8> {
    let max = max.unwrap_or(std::u8::MAX);

    let len = req.len();
    let length_offset = ceil(len as f64 / max as f64, 0) as u8;

    let mut ret = Vec::from(&[length_offset][..]);

    ret.push(max);

    ret.push((len % max as usize) as u8);

    ret.extend_from_slice(req);

    ret
}

#[cfg(tests)]
mod tests {
    #[test]
    pub fn test() {
        assert_eq!(
            prepare_request(&[1, 2, 3, 4, 5], 4),
            &[2, 4, 1, /**/ 1, 2, 3, 4, 5]
        );
    }
}
