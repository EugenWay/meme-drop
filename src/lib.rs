#![no_std]
use gstd::{collections::HashMap, msg, prelude::*, ActorId};
use io::*;
static mut CONTRACT: Option<ClaimContract> = None;
use fungible_token_io::FTAction;
#[derive(Default)]
pub struct ClaimContract {
    admins: Vec<ActorId>,
    claimers: HashMap<ActorId, u128>,
    ft_contract_id: ActorId,
}
#[no_mangle]
extern "C" fn handle() {
    let action: Action = msg::load().expect("Unable to decode input msg");
    let contract = unsafe { CONTRACT.as_mut().expect("Contract is not initialized") };
    match action {
        Action::AddAdmin { account_id } => {
            assert!(contract.admins.contains(&msg::source()), "Not admin");
            contract.admins.push(account_id);
        }
        Action::AddClaimers {
            account_ids,
            amounts,
        } => {
            assert!(contract.admins.contains(&msg::source()), "Not admin");
            assert_eq!(account_ids.len(), amounts.len(), "Wrong len");
            for (i, account_id) in account_ids.iter().enumerate() {
                contract.claimers.insert(*account_id, amounts[i]);
            }
        }
        Action::Claim => {
            let claimer = msg::source();
            let amount = if let Some(amount) = contract.claimers.remove(&claimer) {
                amount
            } else {
                panic!("Not claimer")
            };
            msg::send_with_gas(
                contract.ft_contract_id,
                FTAction::Mint {
                    amount,
                    to: claimer,
                },
                10_000_000_000,
                0,
            )
            .expect("Error in sending msg");
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let init: Init = msg::load().expect("Unable to load the msg");
    unsafe {
        CONTRACT = Some(ClaimContract {
            admins: vec![msg::source()],
            claimers: HashMap::new(),
            ft_contract_id: init.ft_contract_id,
        })
    };
}

#[no_mangle]
extern "C" fn state() {
    let query: Query = msg::load().expect("Unable to load the query");
    let contract = unsafe { CONTRACT.as_ref().expect("Contract is not initialized") };
    let reply = match query {
        Query::GetAdmins => {
            QueryReply::Admins(contract.admins.clone())
        },
        Query::GetClaimers => {
            QueryReply::Claimers(contract.claimers.clone().into_iter().collect())
        },
        Query::GetClaimerAmount(account_id) => {
            let amount = if let Some(amount) = contract.claimers.get(&account_id) {
                *amount
            } else {
                0
            };
            QueryReply::ClaimerAmount(amount)
        },
        Query::GetFtContractId => {
            QueryReply::FtContractId(contract.ft_contract_id)
        }
    };
    msg::reply(reply, 0).expect("Error in sending a query reply");
}