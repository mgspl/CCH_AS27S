mod app;
mod remote_decoder;

pub use remote_decoder::RemoteDecoder;

pub trait Decoder {
    fn secure_decoder(&mut self, pass: &str, tfa: &str) -> (bool, String);
}
