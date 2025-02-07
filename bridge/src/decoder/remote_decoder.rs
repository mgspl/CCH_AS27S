use std::collections::HashMap;

use super::{app::App, Decoder};

pub struct RemoteDecoder {
    app: App,
    max_allowed_tries: u32,
    try_limiter: HashMap<String, u32>,
}

impl RemoteDecoder {
    pub fn new() -> Self {
        Self {
            app: App,
            max_allowed_tries: 2,
            try_limiter: HashMap::default(),
        }
    }

    pub fn check_tries(&mut self, pass: &str) -> bool {
        let try_req = self.try_limiter.entry(pass.to_string()).or_insert(1);

        if *try_req > self.max_allowed_tries {
            return false;
        }

        *try_req += 1;
        true
    }
}

impl Decoder for RemoteDecoder {
    fn secure_decoder(&mut self, pass: &str, tfa: &str) -> (bool, String) {
        if !self.check_tries(pass) {
            return (false, "Número de Tentativas excedidos!".into());
        }

        self.app.secure_decoder(pass, tfa)
    }
}
