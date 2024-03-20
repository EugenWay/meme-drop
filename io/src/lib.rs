//! Data types for the contract input/output.

#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {  
    type Init = In<Init>;
    type Handle = In<Action>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<Query, QueryReply>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Init {
    pub ft_contract_id: ActorId,
}


/// The main type used as an input message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    AddAdmin {
        account_id: ActorId,
    },
    AddClaimers {
        account_ids: Vec<ActorId>,
        amounts: Vec<u128>,
    },
    Claim,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Query {
    GetClaimers,
    GetClaimerAmount(ActorId),
    GetFtContractId,
    GetAdmins,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum QueryReply {
    Claimers (
        Vec<(ActorId, u128)>,
    ),
    ClaimerAmount(u128),
    Admins(Vec<ActorId>),
    FtContractId(ActorId),
}
