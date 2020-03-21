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

                return bincode::serialize(&true).unwrap();
            }
            return bincode::serialize(&ServerErr::PermissionDenied).unwrap();
        }

        let (user_name_auth, req_type) = authorized_req.unwrap();
        let user = self.participants.iter().find(|user| user.ip == addr.ip());

        if user.is_none() || user.unwrap().name == user_name_auth {
            return bincode::serialize(&ServerErr::BadUser).unwrap();
        }

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
                // println!("{}", msg);
                bincode::serialize(&true)
            }
        }
        .unwrap()
    }
}

// if self.participants.iter().any(|el| el.name.clone() == name) {
//     bincode::serialize(&false)
// } else {
//     println!("User connected: {}", name);

//     self.participants.push(Participant { name, ip: user_ip });

//     bincode::serialize(&true)
// }

/* match user {
    None => bincode::serialize(&ServerErr::UnknownUser),
    Some(user) => {
        let Participant { name, .. } = user;

        println!("{}: {}", name, msg);

        self.messages.push((name.clone(), msg));
        bincode::serialize(&true)
    }
}, */
