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

    pub fn handler(
        &mut self,
        data: Vec<u8>,
        addr: SocketAddr,
        on_new_msg: &dyn Fn(WhatsUp),
    ) -> ServerResponse {
        let req = bincode::deserialize::<ReqType>(&data[..]);
        let authorized_req = bincode::deserialize::<AuthorizedReq>(&data[..]);

        if req.is_err() && authorized_req.is_err() {
            return Err(ServerErr::ErrBadRequest400);
        }

        if req.is_ok() {
            if let Ok(ReqType::AddParticipant(user_name)) = req {
                self.participants.push(Participant {
                    name: user_name.clone(),
                    ip: addr,
                });
                let news = WhatsUp::NewParticipant(user_name);
                self.messages.push(news.clone());
                on_new_msg(news);

                let participants = &self
                    .participants
                    .iter()
                    .map(|el| el.name.clone())
                    .collect::<Vec<_>>();

                return Ok(WhatsUp::ParticipantsList(participants.to_vec()));
            }

            return Err(ServerErr::PermissionDenied);
        }

        let (user_name_auth, req_type) = authorized_req.unwrap();
        let user_match_to_ip = self.participants.iter().find(|user| user.ip == addr);

        if user_match_to_ip.is_none() || user_match_to_ip.unwrap().name != user_name_auth {
            return Err(ServerErr::BadUser);
        }

        let Participant { name, .. } = user_match_to_ip.unwrap();

        // ██████╗  █████╗ ██████╗ ███████╗██╗███╗   ██╗ ██████╗      ██████╗ ██████╗ ██████╗ ██████╗ ███████╗ ██████╗████████╗    ██████╗ ███████╗ ██████╗ ██╗   ██╗███████╗███████╗████████╗
        // ██╔══██╗██╔══██╗██╔══██╗██╔════╝██║████╗  ██║██╔════╝     ██╔════╝██╔═══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝╚══██╔══╝    ██╔══██╗██╔════╝██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
        // ██████╔╝███████║██████╔╝███████╗██║██╔██╗ ██║██║  ███╗    ██║     ██║   ██║██████╔╝██████╔╝█████╗  ██║        ██║       ██████╔╝█████╗  ██║   ██║██║   ██║█████╗  ███████╗   ██║
        // ██╔═══╝ ██╔══██║██╔══██╗╚════██║██║██║╚██╗██║██║   ██║    ██║     ██║   ██║██╔══██╗██╔══██╗██╔══╝  ██║        ██║       ██╔══██╗██╔══╝  ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║
        // ██║     ██║  ██║██║  ██║███████║██║██║ ╚████║╚██████╔╝    ╚██████╗╚██████╔╝██║  ██║██║  ██║███████╗╚██████╗   ██║       ██║  ██║███████╗╚██████╔╝╚██████╔╝███████╗███████║   ██║
        // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝      ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ ╚═════╝   ╚═╝       ╚═╝  ╚═╝╚══════╝ ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝

        match req_type {
            ReqType::AddParticipant(_) => Err(ServerErr::PermissionDenied),
            ReqType::SendMessage(msg) => {
                let news = WhatsUp::NewMessage((name.clone(), msg));
                println!("{}", news);

                self.messages.push(news.clone());
                on_new_msg(news);
                Ok(WhatsUp::Nothing)
            }
        }
    }
}
