use cosmwasm_std::{Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage, to_binary};

use crate::msg::{HandleMsg, InitMsg, QueryAnswer, QueryMsg};
use crate::state::{load, save, State};

use sha2::{Digest};
use x25519_dalek::{PublicKey, StaticSecret};
use std::convert::TryInto;


pub const STATE_KEY: &[u8] = b"state";



pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {


    //Takes admin seed and turns into 32 bit array entropy
    let hashvalue = sha2::Sha256::digest(msg.adminseed.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");
    
    

    let state = State {

        seed: hash,

    };


    //Save State
    save(&mut deps.storage, STATE_KEY, &state)?;


    Ok(InitResponse::default())
}





//HANDLE LIST
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Entropy {entropy } => gather_entropy(deps, env, entropy),
    }
}



//---------------------KEY FUNCTIONS------------------------------------------------------------------------------------------------


pub fn gather_entropy<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    entropy: String
) -> StdResult<HandleResponse> {


    //Load state
    let mut state: State = load(&mut deps.storage, STATE_KEY)?;

    //Converts new entropy and old key into a new hash
    let new_data: String = format!("{:?}+{}", state.seed, entropy);

    let hashvalue = sha2::Sha256::digest(new_data.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");

    state.seed = hash;

    //Save State
    save(&mut deps.storage, STATE_KEY, &state)?;


    Ok(HandleResponse::default())


}






//-------------------------------------------QUERIES----------------------------------------------------------------------


pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
   match msg {
    QueryMsg::Info { } => get_info(deps),
    QueryMsg::Keypair { entropy} => get_key(deps, entropy)
   }
    
}



pub fn get_info<S: Storage, A: Api, Q: Querier>(
    _deps: &Extern<S, A, Q>,
) -> StdResult<Binary> {

    let mut data = String::new();
    data.push_str("Made by: Lumi @ Trivium");
    
    to_binary(&QueryAnswer::Info{ info: data})

    
}


pub fn get_key<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    entropy: String
) -> StdResult<Binary> {

    if entropy.len() < 1 {
        return Err(StdError::generic_err("YOU MUST ENTER SOME ENTROPY"));
    }

    //Load state
    let state: State = load(&deps.storage, STATE_KEY)?;


    //Converts new entropy and old key into a new hash
    let new_data: String = format!("{:?}+{}", state.seed, entropy);

    let hashvalue = sha2::Sha256::digest(new_data.as_bytes());
    let hash: [u8; 32] = hashvalue.as_slice().try_into().expect("Wrong length");

     //Generate pub and priv key
     let con_priv_key = StaticSecret::from(hash);
     let con_pub_key = PublicKey::from(&con_priv_key);

    
    to_binary(&QueryAnswer::Keypair{ pubkey: con_pub_key.to_bytes(), privkey: con_priv_key.to_bytes()})

    
}