use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub enum MsgType {
    GetParticipants,
    AddParticipant(String),
}

pub type Participants = Vec<String>;
