use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ReqType {
    GetParticipants,
    AddParticipant(String),
    SendMessage(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerErr {
    ErrBadRequest400,
    UnknownUser,
}

pub type Participants = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub enum WhatsUp {
    NewParticipant(String),
    NewMessage((String, String)),
    ParticipantDisconected(String),
}
