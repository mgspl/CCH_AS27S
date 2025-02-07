mod decoder;

use crate::decoder::{Decoder, RemoteDecoder};

fn main() {
    let password = &"senhaSuperSegura".to_string();
    let tfa = &"SIM".to_string();

    let mut remote = RemoteDecoder::new();

    let (status, body) = remote.secure_decoder(password, "NAO");
    println!("Acesso {}", if status { "Permitido" } else { "Negado" },);
    println!("{}\n", body);

    let (status, body) = remote.secure_decoder(password, tfa);
    println!("Acesso {}", if status { "Permitido" } else { "Negado" },);
    println!("{}\n", body);

    let (status, body) = remote.secure_decoder(password, tfa);
    println!("Acesso {}", if status { "Permitido" } else { "Negado" },);
    println!("{}\n", body);
}
