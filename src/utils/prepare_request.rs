// Max is maximal value in one cell
fn prepare_request_metadata(len: usize, max: Option<u8>) -> [u8; 3] {
    let max = max.unwrap_or(std::u8::MAX) as usize;

    let multiplier = len / max;
    let modulo = len % max;

    [multiplier as u8, max as u8, modulo as u8]
}

pub fn prepare_request(body: &[u8]) -> Vec<u8> {
    let head = prepare_request_metadata(body.len(), None);
    let mut req = Vec::from(&head[..]);
    req.append(&mut Vec::from(body));
    req
}

#[cfg(tests)]
mod tests {
    #[test]
    pub fn test() {
        let req = &[1, 2, 3, 4, 5];
        assert_eq!(prepare_request(req.len(), 4), [1, 4, 1]);
    }
}
