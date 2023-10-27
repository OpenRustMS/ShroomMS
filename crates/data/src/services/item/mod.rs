pub mod stats;
pub mod model;
pub mod starter_set;

use std::sync::Arc;

use proto95::id::ItemId;


pub struct EquipItem {

}

pub struct DropItem {
    pub item_id: ItemId,
    pub quantity: usize,
    pub item_ref: Option<Arc<EquipItem>>
}
