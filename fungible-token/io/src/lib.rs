#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

pub struct FungibleTokenMetadata;

impl Metadata for FungibleTokenMetadata {
    type Init = In<InitConfig>;
    type Handle = InOut<FTAction, FTEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<IoFungibleToken>;
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub admin: ActorId,
    pub decimals: u8,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Mint {
        amount: u128,
        to: ActorId,
    },
    Burn {
        amount: u128,
        from: ActorId,
    },
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
    AddAdmin {
        admin: ActorId,
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTEvent {
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    TotalSupply(u128),
    Balance(u128),
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoFungibleToken {
    pub name: String,
    pub symbol: String,
    pub admins: Vec<ActorId>,
    pub total_supply: u128,
    pub balances: Vec<(ActorId, u128)>,
    pub allowances: Vec<(ActorId, Vec<(ActorId, u128)>)>,
    pub decimals: u8,
}
