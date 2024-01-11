use crate::speed_daemon::server::{Mile, Road};

#[derive(Debug)]
pub struct Ticket {
    pub plate: String,
    pub road: Road,
    pub mile1: Mile,
    pub timestamp1: u32,
    pub mile2: Mile,
    pub timestamp2: u32,
    pub speed: u16,
    pub day_1: u32,
    pub day_2: u32,
}
