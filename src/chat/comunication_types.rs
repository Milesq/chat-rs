use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ReqType {
    AddParticipant(String),
    SendMessage(String),
}

pub type AuthorizedReq = (String, ReqType);

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerErr {
    ErrBadRequest400,
    UnknownUser,
    PermissionDenied,
}

pub type Participants = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub enum WhatsUp {
    NewParticipant(String),
    NewMessage((String, String)),
    ParticipantDisconected(String),
}

impl fmt::Display for WhatsUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use WhatsUp::*;

        let string = match self {
            NewParticipant(name) => format!("{} has joined!", name),
            NewMessage((user, msg)) => format!("[{}]: {}", user, msg),
            ParticipantDisconected(name) => format!("{} has disconnected", name),
        };
        write!(f, "{}", string)
    }
}
