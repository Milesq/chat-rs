use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ReqType {
    GetParticipants,
    AddParticipant(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerErr {
    ErrBadRequest400,
}

pub type Participants = Vec<String>;
