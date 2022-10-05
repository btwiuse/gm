use crate::state;
use codec::Decode;
use codec::Encode;
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, TypeInfo, Decode)]
pub enum Init {
    Payload(String),
}

#[derive(Debug, TypeInfo, Decode)]
pub enum Input {
    Payload(String),
}

#[derive(Debug, TypeInfo, Encode)]
pub enum Output {
    Payload(ActorId),
}

#[derive(Debug, TypeInfo, Decode)]
pub enum Query {
    Last,
    All,
}

#[derive(Debug, TypeInfo, Encode)]
pub enum State {
    AccountId(ActorId),
    All(state::State),
}
