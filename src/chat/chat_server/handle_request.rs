use {super::super::types::*, std::net::SocketAddr};

#[derive(Default)]
pub struct Handler {
    messages: Vec<WhatsUp>,
    participants: Vec<Participant>,
}

impl Handler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn handler(&mut self, data: Vec<u8>, addr: SocketAddr) -> Vec<u8> {
        let req = bincode::deserialize::<ReqType>(&data[..]);
        let authorized_req = bincode::deserialize::<AuthorizedReq>(&data[..]);

        if req.is_err() && authorized_req.is_err() {
            return bincode::serialize(&ServerErr::ErrBadRequest400).unwrap();
        }

        if req.is_ok() {
            if let Ok(ReqType::AddParticipant(user_name)) = req {
                println!("{} connected as {}", addr, user_name);
                self.participants.push(Participant {
                    name: user_name.clone(),
                    ip: addr.ip(),
                });
                self.messages.push(WhatsUp::NewParticipant(user_name));

                return bincode::serialize(
                    &self
                        .participants
                        .iter()
                        .map(|el| el.name.clone())
                        .collect::<Vec<_>>(),
                )
                .unwrap();
            }

            return bincode::serialize(&ServerErr::PermissionDenied).unwrap();
        }

        let (user_name_auth, req_type) = authorized_req.unwrap();
        let user_match_to_ip = self.participants.iter().find(|user| user.ip == addr.ip());

        if user_match_to_ip.is_none() || user_match_to_ip.unwrap().name != user_name_auth {
            return bincode::serialize(&ServerErr::BadUser).unwrap();
        }

        let Participant { name, .. } = user_match_to_ip.unwrap();

        // ██████╗  █████╗ ██████╗ ███████╗██╗███╗   ██╗ ██████╗      ██████╗ ██████╗ ██████╗ ██████╗ ███████╗ ██████╗████████╗    ██████╗ ███████╗ ██████╗ ██╗   ██╗███████╗███████╗████████╗
        // ██╔══██╗██╔══██╗██╔══██╗██╔════╝██║████╗  ██║██╔════╝     ██╔════╝██╔═══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝╚══██╔══╝    ██╔══██╗██╔════╝██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
        // ██████╔╝███████║██████╔╝███████╗██║██╔██╗ ██║██║  ███╗    ██║     ██║   ██║██████╔╝██████╔╝█████╗  ██║        ██║       ██████╔╝█████╗  ██║   ██║██║   ██║█████╗  ███████╗   ██║
        // ██╔═══╝ ██╔══██║██╔══██╗╚════██║██║██║╚██╗██║██║   ██║    ██║     ██║   ██║██╔══██╗██╔══██╗██╔══╝  ██║        ██║       ██╔══██╗██╔══╝  ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║
        // ██║     ██║  ██║██║  ██║███████║██║██║ ╚████║╚██████╔╝    ╚██████╗╚██████╔╝██║  ██║██║  ██║███████╗╚██████╗   ██║       ██║  ██║███████╗╚██████╔╝╚██████╔╝███████╗███████║   ██║
        // ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝      ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝ ╚═════╝   ╚═╝       ╚═╝  ╚═╝╚══════╝ ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝

        match req_type {
            ReqType::AddParticipant(name) => {
                println!("New Participant: {}", name);
                bincode::serialize(&true)
            }
            ReqType::SendMessage(msg) => {
                let news = WhatsUp::NewMessage((name.clone(), msg));
                println!("{}", news);
                self.messages.push(news);
                bincode::serialize(&true)
            }
        }
        .unwrap()
    }
}
