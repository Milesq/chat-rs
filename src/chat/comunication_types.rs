use serde::{Deserialize, Serialize};
use std::{fmt, net::SocketAddr};

pub type AuthorizedReq = (String, ReqType);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ReqType {
    AddParticipant(String),
    SendMessage(String),
    WhatsUp(usize),
}

pub type ServerResponse = Result<WhatsUp, ServerErr>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WhatsUp {
    NewParticipant(String),
    NewMessage((String, String)),
    ParticipantDisconected(String),
    ParticipantsList(Vec<String>),
    News(Vec<WhatsUp>),
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
            NewParticipant(name) => Ok(format!("{} has joined!", name)),
            NewMessage((user, msg)) => Ok(format!("[{}]: {}", user, msg)),
            ParticipantDisconected(name) => Ok(format!("{} has disconnected", name)),

            ParticipantsList(_) => Err(fmt::Error),
            Nothing => Err(fmt::Error),
            News(_) => Err(fmt::Error),
        };
        write!(f, "{}", string?)
    }
}
