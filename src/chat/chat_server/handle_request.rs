use {super::super::types::*, std::net::SocketAddr};

#[derive(Default)]
pub struct Handler {
    pub(super) messages: Vec<WhatsUp>,
    pub(super) participants: Vec<Participant>,
}

impl Handler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn handler(&mut self, data: Vec<u8>, addr: SocketAddr) -> Response {
        let req = bincode::deserialize::<Request>(&data[..]);

        if req.is_err() {
            return Err(ServerErr::ErrBadRequest400);
        }

        let Request {
            user_name,
            req_type,
        } = req.unwrap();

        let user_match_to_ip = self.participants.iter().find(|user| user.ip == addr);

        if req_type == ReqType::AddParticipant && user_match_to_ip.is_none() {
            self.participants.push(Participant {
                name: user_name.clone(),
                ip: addr,
            });
            let news = WhatsUp::NewParticipant(user_name);
            println!("{}", news);
            self.messages.push(news);

            let participants = &self
                .participants
                .iter()
                .map(|el| el.name.clone())
                .collect::<Vec<_>>();
            return Ok(WhatsUp::ParticipantsList(participants.to_vec()));
        }

        if user_match_to_ip.is_none() {
            return Err(ServerErr::PermissionDenied);
        }

        if user_match_to_ip.unwrap().name != user_name {
            return Err(ServerErr::BadUser);
        }
        // ██████╗  █████╗ ██████╗ ███████╗██╗███╗   ██╗ ██████╗      ██████╗ ██████╗ ██████╗ ██████╗ ███████╗ ██████╗████████╗    ██████╗ ███████╗ ██████╗ ██╗   ██╗███████╗███████╗████████╗
        // ██╔══██╗██╔══██╗██╔══██╗██╔════╝██║████╗  ██║██╔════╝     ██╔════╝██╔═══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝╚══██╔══╝    ██╔══██╗██╔════╝██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
        // ██████╔╝███████║██████╔╝███████╗██║██╔██╗ ██║██║  ███╗    ██║     ██║   ██║██████╔╝██████╔╝█████╗  ██║        ██║       ██████╔╝█████╗  ██║   ██║██║   ██║█████╗  ███████╗   ██║
        // ██╔═══╝ ██╔══██║██╔══██╗╚════██║██║██║╚██╗██║██║   ██║    ██║     ██║   ██║██╔══██╗██╔══██╗██╔══╝  ██║        ██║       ██╔══██╗██╔══╝  ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║
        // ██║     ██║  ██║██║  ██║███████║██║██║ ╚████║╚██████╔╝    ╚██████╗╚██████╔╝██║  ██║██║  ██║███████╗╚██████╗   ██║       ██║  ██║███████╗╚██████╔╝╚██████╔╝███████╗███████║   ██║
        // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝      ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ ╚═════╝   ╚═╝       ╚═╝  ╚═╝╚══════╝ ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝

        match req_type {
            ReqType::SendMessage(msg) => {
                let news = WhatsUp::NewMessage((user_name.clone(), msg));
                println!("{}", news);

                self.messages.push(news.clone());
                Ok(WhatsUp::Nothing)
            }
            ReqType::WhatsUp(ptr) => {
                if self.messages.len() < ptr {
                    Ok(WhatsUp::Nothing)
                } else {
                    Ok(WhatsUp::News(self.messages[ptr..].to_vec()))
                }
            }
            _ => Ok(WhatsUp::Nothing),
        }
    }
}
