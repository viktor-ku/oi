use lib_store::Timer;

#[derive(Debug)]
pub enum OidMessage {
    StartTimer(Timer),
    Save(Vec<u8>),
}
