
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Feed;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
   pub admin_address:String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
   CreateFeed{
    address:String,
    message: String,
   },

   UpdateFeed{
    id:u128,
    message:String, 
   },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    //This Variant Returns a single feed based on an id and an address
   GetFeed{address:String, feed_id:u128},
   GetAllFeeds{offset: Option<u128>,limit: Option<u64>},
   GetAllFeedsByOwner{offset: Option<u128>,limit: Option<u64>,address:String}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FeedResponse {
    pub feed: Feed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FeedListResponse {
   pub feeds: Vec<Feed>,
}

