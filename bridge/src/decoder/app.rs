use super::Decoder;

pub struct App;

impl Decoder for App {
    fn secure_decoder(&mut self, pass: &str, tfa: &str) -> (bool, String) {
        if pass == "senhaSuperSegura" && tfa == "SIM" {
            return (true, "Senha e 2FA Corretos".into());
        }

        return (false, "Senha ou 2FA Incorretos".into());
    }
}
