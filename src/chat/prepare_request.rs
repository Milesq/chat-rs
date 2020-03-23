const MAX: usize = std::u8::MAX as usize;

#[derive(Default, Debug)]
pub struct PreparePacketConfig {
    pub packet: Vec<u8>,
    pub length: usize,
    pub is_length_calculated: bool,
}

impl PreparePacketConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

pub fn prepare_to_receive(part: Vec<u8>, config: &mut PreparePacketConfig) -> Option<Vec<u8>> {
    if !config.is_length_calculated {
        // Calculating length

        if let Some(n) = part.iter().position(|&el| el == 0) {
            let tail = &part[(n + 1)..];
            config.length += sum(&Vec::from(&part[..n]));
            config.packet.extend(tail);

            if tail.len() > config.length {
                assert!(config.packet.len() > config.length);
                config.packet.resize(config.length, 0);
                return Some(config.packet.clone());
            } else {
                config.length -= tail.len();
                config.is_length_calculated = true;
            }
        } else {
            config.length += sum(&part);
        }
    }

    if config.length > 0 {
        if part.len() > config.length {
            config.packet.extend(Vec::from(&part[0..config.length]));
            Some(config.packet.clone())
        } else {
            config.length -= part.len();
            config.packet.extend(part);
            None
        }
    } else {
        Some(config.packet.clone())
    }
}

pub fn prepare_to_send(packet: Vec<u8>) -> Vec<Vec<u8>> {
    let mut ret = Vec::new();
    let mut len = packet.len();

    while len > MAX {
        len -= MAX;
        ret.push(MAX as u8);
    }

    ret.push(len as u8);
    ret.push(0);

    ret.extend(packet);

    let mut ret = ret
        .chunks(crate::PACKET_SIZE)
        .map(|el| Vec::from(el))
        .collect::<Vec<_>>();

    for el in &mut ret {
        el.resize(crate::PACKET_SIZE, 0);
    }

    ret
}

fn sum(v: &Vec<u8>) -> usize {
    v.iter().fold(0usize, |acc, x| acc + *x as usize)
}
