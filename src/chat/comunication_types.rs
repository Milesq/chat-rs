use serde::{Deserialize, Serialize};
use std::{fmt, net::SocketAddr};

pub type AuthorizedReq = (String, ReqType);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ReqType {
    AddParticipant(String),
    SendMessage(String),
}

pub type ServerResponse = Result<WhatsUp, ServerErr>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WhatsUp {
    NewParticipant(String),
    NewMessage((String, String)),
    ParticipantDisconected(String),
    ParticipantsList(Vec<String>),
    Nothing,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerErr {
    ErrBadRequest400,
    BadUser,
    UserNameAlreadyTaken,
    PermissionDenied,
}

#[derive(Clone)]
pub struct Participant {
    pub name: String,
    pub ip: SocketAddr,
}

impl fmt::Display for WhatsUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use WhatsUp::*;

        let string = match self {
            NewParticipant(name) => format!("{} has joined!", name),
            NewMessage((user, msg)) => format!("[{}]: {}", user, msg),
            ParticipantDisconected(name) => format!("{} has disconnected", name),

            ParticipantsList(_) => "".into(),
            Nothing => "".into(),
        };
        write!(f, "{}", string)
    }
}
