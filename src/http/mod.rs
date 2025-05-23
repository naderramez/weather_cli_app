// static mut CLIENT: Option<Client> = None;

// pub fn get_client() -> Client {
//     match CLIENT {
//         Some(client) => client,
//         None => {
//             CLIENT = Some(reqwest::blocking::Client::new());
//             return CLIENT;
//         }
//     }
// }

pub struct Client {
    pub client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::blocking::Client::new(),
        }
    }
    pub fn get_client(&self) -> &reqwest::blocking::Client {
        &self.client
    }
}
