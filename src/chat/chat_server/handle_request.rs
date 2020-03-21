use super::super::types::*;

pub fn handler(data: Vec<u8>) -> Vec<u8> {
    let req = bincode::deserialize::<ReqType>(&data[..]);
    println!("{:?}", req);

    bincode::serialize(&true).unwrap()
}
