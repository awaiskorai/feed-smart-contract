#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Order};
use cw2::set_contract_version;
use cw_storage_plus::Bound;

// use schemars::_serde_json::Error;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg,FeedResponse, FeedListResponse};
use crate::state::{Feed, feeds,
    //FEED, 
    State, STATE, FEEDCOUNTER};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:feed-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


//temporary size for default limits -- will be changed
const MAX_LIMIT: u64 = 30;
const DEFAULT_LIMIT: u64 = 10;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //Instantiate contract admin
    let admin_address = deps.api.addr_validate(&msg.admin_address)?;

    //Instantiating Variable ->Set Feed Counter to 0 using feedcounter varial
    let feedcounter: u128 = 0;

    //Save admin address as in the state constant
    let state = State{
        admin_address: admin_address.clone()
    };
    //Set contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
   
    //Save State or return an error
    STATE.save(deps.storage, &state)?;
    //Save Feed Counter or return an error
    FEEDCOUNTER.save(deps.storage,&feedcounter)?;
   

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", admin_address)
        .add_attribute("feed_counter", feedcounter.to_string())
        )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateFeed { address, message } => create_feed(deps,info,address,message),
        ExecuteMsg::UpdateFeed { id,message } => update_feed(deps,info,id,message)
    }
}


pub fn create_feed(deps: DepsMut, info: MessageInfo, address: String, message: String) -> Result<Response, ContractError> {
   //Validating the  Address for use in Feed struct
   let address = deps.api.addr_validate(&address)?;
   //Load latest Feed Counter, if not available then set a value of 0
   let feed_counter = match FEEDCOUNTER.may_load(deps.storage)?{
    Some(counter)=>counter,
    None => 0u128,
   };

   //Load latest feed, if not deal with the error using a match case
   let feed =  feeds().may_load(deps.storage, &feed_counter.to_be_bytes().to_vec())?;
    
   //Match case for a success in creating or failure case
   let feed = match feed{
        None =>{
                    Feed{
                        id:feed_counter,
                        address:address.clone(),
                        message:message,
                    }
        },
        Some(_feed_found)=>{
            
                        Feed{
                            id:feed_counter.checked_add(1u128).unwrap(),
                            address:address.clone(),
                            message:message
                        }
        }
   };
   
   if info.sender!=feed.address{
     return Err(ContractError::Unauthorized { });
    }
    feeds().save(deps.storage,&feed.id.to_be_bytes().to_vec(), &feed)?;
    FEEDCOUNTER.save(deps.storage,&feed.id)?;
    Ok(Response::new().add_attribute("method", "feedcreated"))
}


pub fn update_feed(deps:DepsMut, info:MessageInfo, id:u128, message:String)->Result<Response, ContractError>{
    // let mut feed =match FEED.may_load(deps.storage,(&info.sender, id) ){
    //     Ok(feed_option)=> match feed_option {
    //         Some(feed)=>feed,
    //         None=>return Err(ContractError::CustomError { val: "No feed found inserted by the user".to_string() })
    //     }, 
    //     _=> return Err(ContractError::CustomError { val: "No feed found inserted by the user".to_string() })
    
    // };

    // if info.sender!=feed.address {
    //     return Err( ContractError::Unauthorized {  });
    // }

    let feed = feeds().update(deps.storage,&id.to_be_bytes().to_vec() , |feed| match feed{
        Some(feed)=>{
            if info.sender!=feed.address {
                return Err(ContractError::Unauthorized {  })
            }else{
                let feed_update = Feed{
                    message,
                    ..feed
                };
                Ok(feed_update)
            }
        },
        None=>{
            return Err(ContractError::CustomError { val: "Feed not found!".to_string() })
        }
    })?;

    Ok(Response::new().add_attribute("action", "updated_feed").add_attribute("update_by", info.sender).add_attribute("message", feed.message))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFeed { address,feed_id } => to_binary(&get_feed(deps,address,feed_id)?),
        QueryMsg::GetAllFeeds {offset, limit  }=> to_binary(&get_all_feeds(deps, offset, limit )?),
        QueryMsg::GetAllFeedsByOwner {offset, limit , address }=> to_binary(&get_all_feeds_by_owner(deps, offset, limit, address )?),
       
    }
}

fn get_feed(deps:Deps, address: String, feed_id: u128)->StdResult<FeedResponse>{
    let address = deps.api.addr_validate(&address)?;
    //Check if there is any feed
    match FEEDCOUNTER.may_load(deps.storage)?{
        Some(counter)=>counter,
        None => return Err(cosmwasm_std::StdError::not_found("There are no feeds to get".to_string())),
       };
    println!("Counter:: {:?}", FEEDCOUNTER.may_load(deps.storage).unwrap());
    
    // let feed =  FEED.may_load(deps.storage, (&address,feed_counter))?;

    //Check if the feed with the given id and address exists
    // match feeds().may_load(deps.storage, (&address.clone(),feed_id))?{
    //     Some(feed)=>Ok(FeedResponse { feed:feed }),
    //     None => return Err(cosmwasm_std::StdError::not_found("Feed not found".to_string())),
    //    }

    
    let res:StdResult<Vec<_>>= feeds()
    .idx
    .add_feed_id
    .prefix((address,feed_id))
    .range_raw(deps.storage, None, None, Order::Ascending)
    .map(|data| {
     
      data
    }).collect();

    let (_,value) = &res?[0];
      
      
    //println!("Value: {:?}", value);


    Ok(FeedResponse {
        feed:  value.to_owned(),
    })    
       
}

fn get_all_feeds(deps: Deps,offset:Option<u128>, limit:Option<u64>)-> StdResult<FeedListResponse>{
     let num_items = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
     println!("Limti: {:?}", num_items);
     let total_count: u128 = FEEDCOUNTER.may_load(deps.storage)?.unwrap_or_default();
    //  println!("Total: {}  ----  Offset: {}", total_count, offset.unwrap());
     let finish= offset.map(|offset| Bound::inclusive(total_count -offset));
     println!("{:?}", finish.clone().unwrap());


    // Query by latest post
     let list: StdResult<Vec<_>>  = feeds()
     .idx.feed_id
     .prefix(())
     .range_raw(deps.storage, None, finish, Order::Descending)
     .take(num_items)
     .map(|item| item.map(|(_, t)| t))
     .collect();
    
     println!("{:?}", list);
     let response = FeedListResponse {
         feeds: list?,
     };
    
     Ok(response)    
}

fn get_all_feeds_by_owner(deps: Deps,offset:Option<u128>, limit:Option<u64>, address: String)-> StdResult<FeedListResponse>{
    //Address check
    let addr = deps.api.addr_validate(&address)?;
    // pagination logic
    let num_items = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
   // println!("Limti: {:?}", num_items);
    let total_count: u128 = FEEDCOUNTER.may_load(deps.storage)?.unwrap_or_default();
   // println!("Total: {}  ----  Offset: {}", total_count, offset.unwrap());
    let finish= offset.map(|offset| Bound::inclusive(total_count -offset));
   //  println!("{:?}", finish.clone().unwrap());


//Query by owner latest post first
    let list: StdResult<Vec<_>>  = feeds()
    .idx.address
    .prefix(addr)
    .range_raw(deps.storage, None, finish, Order::Descending)
    .take(num_items)
    .map(|item| item.map(|(_, t)| t))
    .collect();
   
    //println!("{:?}", list);
    let response = FeedListResponse {
        feeds: list?,
    };
   
    Ok(response)    
}

#[cfg(test)]
mod tests {
   // use std::ops::Add;
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {admin_address:"admin".to_string() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

   

    #[test]
    fn create_feed_test() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { admin_address:"admin".to_string()};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // no one else can create the feed for now
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::CreateFeed { address: "someone".to_string(), message: "Testing Message Functionality".to_string() } ;
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
             Err(ContractError::Unauthorized {}) => {}
             _ => panic!("Must return unauthorized error"),
         }

        // only the original creator can create the feed
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();
        // let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();


        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();
        let msg = ExecuteMsg::CreateFeed { address: auth_info.sender.clone().into_string(), message: "Testing Message Functionality".to_string() } ;
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();

        // checking the feed
     
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetFeed { address: auth_info.sender.clone().into_string(), feed_id:1 } ).unwrap();
        let value: FeedResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.feed.id);

       // query(deps.as_ref(), mock_env(), QueryMsg::GetAllFeeds { offset: Some(0u128), limit: Some(5u64) });
       
    }
}


// FEED.update(deps.storage, |mut feed| -> Result<_, ContractError> {
    //     if info.sender != feed.address {
    //         return Err(ContractError::Unauthorized {});
    //     }
    //     feed.id = count;
    //     Ok(feed)
    // })?;

// QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
// fn query_count(deps: Deps) -> StdResult<CountResponse> {
//     let state = FEED.load(deps.storage)?;
//     // Ok(CountResponse { count: state.id })
// }


// let feed = Feed {
    //     id: 0,
    //     address: info.sender.clone(),
    //     message:"".to_string()
    // };
    // FEED.save(deps.storage, &feed)?;


// #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

    //     let msg = InstantiateMsg { };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     // let msg = ExecuteMsg::Increment {};
    //     // let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     // let value: CountResponse = from_binary(&res).unwrap();
    //     // assert_eq!(18, value.count);
    // }