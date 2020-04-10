const MAX: usize = std::u8::MAX as usize;

#[derive(Default, Debug)]
pub struct PreparePacketConfig {
    pub packet: Vec<u8>,
    pub length: Option<usize>,
}

impl PreparePacketConfig {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn prepare_to_receive(&mut self, part: Vec<u8>) -> Option<Vec<u8>> {
        match self.length {
            None => {
                self.length = Some(sum(&part));
                None
            }
            Some(_) => {
                for num in part {
                    if self.length.unwrap() > 0 {
                        self.packet.push(num);
                        self.length = Some(self.length.unwrap() - 1);
                    } else {
                        return Some(self.packet.clone());
                    }
                }

                None
            }
        }
    }
}

pub fn prepare_to_send(packet: Vec<u8>) -> Vec<Vec<u8>> {
    let mut len = packet.len();
    assert!(len <= MAX * crate::PACKET_SIZE);

    let mut ret = Vec::new();

    while len > MAX {
        len -= MAX;
        ret.push(MAX as u8);
    }

    ret.push(len as u8);
    ret.resize(crate::PACKET_SIZE, 0);

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
