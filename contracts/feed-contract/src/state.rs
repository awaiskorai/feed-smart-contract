use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, UniqueIndex, IndexList, Index, IndexedMap, MultiIndex};
//use cw_controllers::Admin;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Feed {
    pub id: u128,
    pub address: Addr,
    pub message: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State{
   pub admin_address: Addr,
}
pub struct FeedIndexes<'a>{
    pub feed_id: UniqueIndex<'a, u128, Feed,u128 >,
    pub address: MultiIndex<'a, Addr, Feed,u128>,
    pub add_feed_id:MultiIndex<'a,(Addr, u128),Feed,u128>
}

impl<'a> IndexList<Feed> for FeedIndexes<'a>{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn cw_storage_plus::Index<Feed>> + '_> {
        let v: Vec<&dyn Index<Feed>> = vec![&self.feed_id,&self.address,&self.add_feed_id];
         Box::new(v.into_iter())
    }
}

pub fn feeds<'a>() -> IndexedMap<'a, &'a [u8], Feed, FeedIndexes<'a>> {
    let indexes = FeedIndexes {
      feed_id: UniqueIndex::new(|d| d.id, "feed_id"),
      address:MultiIndex::new(|f|  f.address.clone(),FEED_NAMESPACE,"feed_address"),
      add_feed_id:MultiIndex::new(|f| (f.address.clone(),f.id),FEED_NAMESPACE,"feed_address_id"),
    };
    IndexedMap::new(FEED_NAMESPACE, indexes)
  }

   

pub const STATE: Item<State> = Item::new("state");
pub const FEEDCOUNTER: Item<u128>= Item::new("feed-counter");
const FEED_NAMESPACE: &str = "threads";
//pub const FEED: Map<(&Addr,u128),Feed> = Map::new("feed");
//pub const ADMIN: Admin = Admin::new("admin");
