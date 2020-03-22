use {super::super::types::*, std::net::SocketAddr};

type ServerErrResult<T> = Result<T, ServerErr>;

#[inline]
fn serialize<T>(value: &ServerErrResult<T>) -> Vec<u8>
where
    T: serde::Serialize,
{
    bincode::serialize::<ServerErrResult<T>>(value).unwrap()
}

#[derive(Default)]
pub struct Handler {
    pub(super) messages: Vec<WhatsUp>,
    pub(super) participants: Vec<Participant>,
}

impl Handler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn handler(&mut self, data: Vec<u8>, addr: SocketAddr) -> Vec<u8> {
        let req = bincode::deserialize::<ReqType>(&data[..]);
        let authorized_req = bincode::deserialize::<AuthorizedReq>(&data[..]);

        if req.is_err() && authorized_req.is_err() {
            return serialize::<()>(&Err(ServerErr::ErrBadRequest400));
        }

        if req.is_ok() {
            if let Ok(ReqType::AddParticipant(user_name)) = req {
                println!("{} connected as {}", addr, user_name);
                self.participants.push(Participant {
                    name: user_name.clone(),
                    ip: addr,
                });
                self.messages.push(WhatsUp::NewParticipant(user_name));

                let participants = &self
                    .participants
                    .iter()
                    .map(|el| el.name.clone())
                    .collect::<Vec<_>>();

                return serialize(&Ok(participants));
            }

            return serialize::<()>(&Err(ServerErr::PermissionDenied));
        }

        let (user_name_auth, req_type) = authorized_req.unwrap();
        let user_match_to_ip = self.participants.iter().find(|user| user.ip == addr);

        if user_match_to_ip.is_none() || user_match_to_ip.unwrap().name != user_name_auth {
            return serialize::<()>(&Err(ServerErr::BadUser));
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
                serialize(&Ok(()))
            }
            ReqType::SendMessage(msg) => {
                let news = WhatsUp::NewMessage((name.clone(), msg));
                println!("{}", news);
                self.messages.push(news);
                serialize(&Ok(()))
            }
        }
    }
}
