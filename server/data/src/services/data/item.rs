use crate::{
    entities::{equip_item, inventory_slot, item_stack},
    services::{
        character::inventory_set::{InventorySet, INV_ITEM_CAP},
        helper::intentory::data::{EquipInventory, EquippedInventory, ShroomStackInventory},
        item::{
            model::{EquipItem, StackItem},
            stats::EquipStat,
        },
        meta::meta_service::MetaService,
    },
};
use anyhow::anyhow;
use itertools::Itertools;
use num_enum::TryFromPrimitive;
use proto95::{
    id::{item_id::InventoryType, ItemId},
    shared::inventory::CharEquipSlot,
};
use sea_orm::{
    ActiveValue::NotSet, ColumnTrait, DatabaseConnection, DeriveColumn, EntityTrait, EnumIter,
    QueryFilter, QuerySelect, Set,
};

use super::character::{CharacterID, ItemStarterSet};

#[derive(Debug, Clone, Default)]
pub struct CharacterEquippedItemIds {
    pub equipped: Vec<(CharEquipSlot, ItemId)>,
    pub masked: Vec<(CharEquipSlot, ItemId)>,
}

pub type DbItemId = i32;
pub type DbSlotId = i32;

#[derive(Debug, Clone)]
pub struct ItemService {
    db: DatabaseConnection,
    meta: &'static MetaService,
}

fn map_equip_to_active_model(item: &EquipItem) -> equip_item::ActiveModel {
    let stats = &item.stats;
    let lvl = item.level_info.as_ref();
    let id = item.db_id.map(Set).unwrap_or(NotSet);

    equip_item::ActiveModel {
        id,
        expires_at: Set(item.expiration),
        cash_id: Set(item.cash_id.map(|i| i as i64)),
        item_id: Set(item.item_id.0 as i32),
        flags: Set(item.flags.bits() as i32),
        //TODO ioptional
        item_level: Set(lvl.map(|lvl| lvl.level as i32).unwrap_or(0)),
        //TODO Optional
        item_exp: Set(lvl.map(|lvl| lvl.exp as i32).unwrap_or(0)),
        vicious_hammers: Set(item.hammers_used as i32),
        //TODO optional
        owner_tag: Set(item.owner.clone().unwrap_or(String::new())),
        level: Set(0),
        upgrade_slots: Set(item.slots as i32),
        str: Set(stats[EquipStat::Str].0 as i32),
        dex: Set(stats[EquipStat::Dex].0 as i32),
        luk: Set(stats[EquipStat::Luk].0 as i32),
        int: Set(stats[EquipStat::Int].0 as i32),
        hp: Set(stats[EquipStat::Hp].0 as i32),
        mp: Set(stats[EquipStat::Mp].0 as i32),
        weapon_atk: Set(stats[EquipStat::WeaponAtk].0 as i32),
        weapon_def: Set(stats[EquipStat::WeaponDef].0 as i32),
        magic_atk: Set(stats[EquipStat::MagicAtk].0 as i32),
        magic_def: Set(stats[EquipStat::MagicDef].0 as i32),
        accuracy: Set(stats[EquipStat::Accuracy].0 as i32),
        avoid: Set(stats[EquipStat::Avoid].0 as i32),
        speed: Set(stats[EquipStat::Speed].0 as i32),
        jump: Set(stats[EquipStat::Jump].0 as i32),
        craft: Set(stats[EquipStat::Craft].0 as i32),
    }
}

fn map_stack_to_active_model(item: &StackItem) -> item_stack::ActiveModel {
    let id = item.db_id.map(Set).unwrap_or(NotSet);

    item_stack::ActiveModel {
        id,
        expires_at: Set(item.expiration),
        cash_id: Set(item.cash_id.map(|i| i as i64)),
        item_id: Set(item.item_id.0 as i32),
        flags: Set(item.flags.bits() as i32),
        quantity: Set(item.quantity as i32),
    }
}

impl ItemService {
    pub fn new(db: DatabaseConnection, meta: &'static MetaService) -> Self {
        Self { db, meta }
    }

    pub fn get_eq_item_from_id(&self, item_id: ItemId) -> anyhow::Result<EquipItem> {
        let item_meta = self
            .meta
            .get_eq_data(item_id)
            .ok_or_else(|| anyhow!("Invalid item: {item_id:?}"))?;

        Ok(EquipItem::from_item_id(item_id, item_meta))
    }

    pub fn get_stack_item_from_id(
        &self,
        item_id: ItemId,
        quantity: usize,
    ) -> anyhow::Result<StackItem> {
        let item = StackItem::from_item_id(item_id, quantity as u16);
        Ok(item)
    }

    pub async fn create_equip(&self, item: &mut EquipItem) -> anyhow::Result<()> {
        if item.db_id.is_some() {
            anyhow::bail!("DB id already set");
        }
        let res = equip_item::Entity::insert(map_equip_to_active_model(item))
            .exec(&self.db)
            .await?;

        item.db_id = Some(res.last_insert_id);
        Ok(())
    }

    pub async fn update_equip(&self, item: &mut EquipItem) -> anyhow::Result<()> {
        if item.db_id.is_none() {
            anyhow::bail!("DB id not set");
        }
        equip_item::Entity::update(map_equip_to_active_model(item))
            .exec(&self.db)
            .await?;
        item.last_update = 0;

        Ok(())
    }

    pub async fn create_stack(&self, item: &mut StackItem) -> anyhow::Result<()> {
        if item.db_id.is_some() {
            anyhow::bail!("DB id already set");
        }

        let res = item_stack::Entity::insert(map_stack_to_active_model(item))
            .exec(&self.db)
            .await?;
        item.db_id = Some(res.last_insert_id);
        Ok(())
    }

    pub async fn update_stack(&self, item: &mut StackItem) -> anyhow::Result<()> {
        if item.db_id.is_none() {
            anyhow::bail!("DB id not set");
        }
        item_stack::Entity::update(map_stack_to_active_model(item))
            .exec(&self.db)
            .await?;
        item.last_update = 0;

        Ok(())
    }

    pub async fn create_starter_set(
        &self,
        char_id: i32,
        starter_set: ItemStarterSet,
    ) -> anyhow::Result<()> {
        let slots = [
            CharEquipSlot::Bottom,
            CharEquipSlot::Shoes,
            CharEquipSlot::Top,
            CharEquipSlot::Weapon,
        ];
        let items = [
            starter_set.bottom,
            starter_set.shoes,
            starter_set.top,
            starter_set.weapon,
        ]
        .iter()
        .map(|id| self.get_eq_item_from_id(*id))
        .collect::<anyhow::Result<Vec<_>>>()?;

        let mut inv = InventorySet::with_default_slots();
        for (mut item, slot) in items.into_iter().zip(slots) {
            self.create_equip(&mut item).await?;
            inv.equipped.set(slot, item.into())?;
        }

        inv.etc
            .set(0, self.get_stack_item_from_id(starter_set.guide, 1)?)?;

        self.save_inventory(&mut inv, char_id).await?;

        Ok(())
    }

    pub async fn clear_inventory(&self, char_id: i32) -> anyhow::Result<()> {
        inventory_slot::Entity::delete_many()
            .filter(inventory_slot::Column::CharId.eq(char_id))
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn save_eq_inventory_type<'a, const CAP: usize>(
        &self,
        inv_type: InventoryType,
        char_id: i32,
        inv: &mut EquipInventory<CAP>,
    ) -> anyhow::Result<()> {
        if inv.is_empty() {
            return Ok(());
        }

        // Update items
        for item_slot in inv.items_mut() {
            let item = &mut item_slot.item;
            if item.db_id.is_none() {
                self.create_equip(item).await?;
            } else if item.last_update > 0 {
                self.update_equip(item).await?;
            }
        }

        let slots = inv
            .item_with_slots()
            .map(|(slot, item)| inventory_slot::ActiveModel {
                id: NotSet,
                equip_item_id: Set(Some(item.item.db_id.unwrap())),
                char_id: Set(char_id),
                slot: Set(slot as u8 as i32),
                inv_type: Set(inv_type as i32),
                stack_item_id: Set(None),
                pet_item_id: Set(None),
            });

        let slots = slots.collect_vec();
        inventory_slot::Entity::insert_many(slots)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn save_eqd_inventory_type<'a, const CAP: usize>(
        &self,
        inv_type: InventoryType,
        char_id: i32,
        inv: &mut EquippedInventory<CAP>,
    ) -> anyhow::Result<()> {
        if inv.is_empty() {
            return Ok(());
        }

        // Update items
        for item_slot in inv.items_mut() {
            let item = &mut item_slot.0.item;
            if item.db_id.is_none() {
                self.create_equip(item).await?;
            } else if item.last_update > 0 {
                self.update_equip(item).await?;
            }
        }

        let slots = inv
            .item_with_slots()
            .map(|(slot, item)| inventory_slot::ActiveModel {
                id: NotSet,
                equip_item_id: Set(Some(item.0.item.db_id.unwrap())),
                char_id: Set(char_id),
                slot: Set(slot as u8 as i32),
                inv_type: Set(inv_type as i32),
                stack_item_id: Set(None),
                pet_item_id: Set(None),
            });

        let slots = slots.collect_vec();
        inventory_slot::Entity::insert_many(slots)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    async fn save_stack_inventory_type<'a>(
        &self,
        inv_type: InventoryType,
        char_id: i32,
        inv: &mut ShroomStackInventory<INV_ITEM_CAP>,
    ) -> anyhow::Result<()> {
        if inv.is_empty() {
            return Ok(());
        }
        // Update items
        // TODO optimize this + use transaction
        for item in inv.items_mut() {
            if item.db_id.is_none() {
                self.create_stack(item).await?;
            } else if item.last_update > 0 {
                self.update_stack(item).await?;
            }
        }

        let slots = inv
            .item_with_slots()
            .map(|(slot, item)| inventory_slot::ActiveModel {
                id: NotSet,
                equip_item_id: Set(None),
                char_id: Set(char_id),
                slot: Set(slot as i32),
                inv_type: Set(inv_type as i32),
                stack_item_id: Set(Some(item.db_id.unwrap())),
                pet_item_id: Set(None),
            });

        inventory_slot::Entity::insert_many(slots)
            .exec(&self.db)
            .await?;

        Ok(())
    }

    pub async fn save_inventory(
        &self,
        invs: &mut InventorySet,
        char_id: CharacterID,
    ) -> anyhow::Result<()> {
        inventory_slot::Entity::delete_many()
            .filter(inventory_slot::Column::CharId.eq(char_id))
            .exec(&self.db)
            .await?;

        self.save_eqd_inventory_type(InventoryType::Equipped, char_id, &mut invs.equipped)
            .await?;

        self.save_eqd_inventory_type(InventoryType::Special, char_id, &mut invs.masked_equipped)
            .await?;

        self.save_eq_inventory_type(InventoryType::Equip, char_id, &mut invs.equip)
            .await?;

        self.save_stack_inventory_type(InventoryType::Consume, char_id, &mut invs.use_)
            .await?;
        self.save_stack_inventory_type(InventoryType::Install, char_id, &mut invs.misc)
            .await?;
        self.save_stack_inventory_type(InventoryType::Etc, char_id, &mut invs.etc)
            .await?;
        self.save_stack_inventory_type(InventoryType::Cash, char_id, &mut invs.cash)
            .await?;
        Ok(())
    }

    pub async fn load_inventory_for_character(&self, char_id: i32) -> anyhow::Result<InventorySet> {
        let equip_item_slots = inventory_slot::Entity::find()
            .filter(inventory_slot::Column::CharId.eq(char_id))
            .inner_join(equip_item::Entity)
            .select_also(equip_item::Entity)
            .all(&self.db)
            .await?;

        let item_stack_slots = inventory_slot::Entity::find()
            .filter(inventory_slot::Column::CharId.eq(char_id))
            .inner_join(item_stack::Entity)
            .select_also(item_stack::Entity)
            .all(&self.db)
            .await?;

        let mut inv = InventorySet::with_default_slots();

        // Load equips
        for (slot_info, equip_item) in equip_item_slots {
            let Some(equip_item) = equip_item else {
                anyhow::bail!("Invalid no equip item");
            };
            let inv_type = InventoryType::try_from_primitive(slot_info.inv_type as u8)?;
            match inv_type {
                InventoryType::Equipped => {
                    let slot = CharEquipSlot::try_from_primitive(slot_info.slot as u8)?;
                    let equip_item: EquipItem = equip_item.into();
                    inv.equipped.set(slot, equip_item.into())?;
                }
                InventoryType::Special => {
                    let slot = CharEquipSlot::try_from_primitive(slot_info.slot as u8)?;
                    let equip_item: EquipItem = equip_item.into();
                    inv.masked_equipped.set(slot, equip_item.into())?;
                }
                InventoryType::Equip => {
                    let slot = slot_info.slot as usize;
                    let equip_item: EquipItem = equip_item.into();
                    inv.equip.set(slot, equip_item.into())?
                }
                _ => anyhow::bail!(
                    "Inventory Item({} - {}) with invalid inventory type found: {inv_type:?}",
                    equip_item.id,
                    equip_item.item_id
                ),
            }
        }

        // Load slots
        for (slot_info, stack_item) in item_stack_slots {
            let Some(stack_item) = stack_item else {
                anyhow::bail!("Invalid no stack item");
            };
            let inv_type = InventoryType::try_from_primitive(slot_info.inv_type as u8)?;
            let slot = slot_info.slot as usize;

            inv.get_stack_inventory_mut(inv_type)?
                .set(slot, stack_item.into())?;
        }

        Ok(inv)
    }

    pub async fn load_equipped_items(
        &self,
        char_id: CharacterID,
    ) -> anyhow::Result<CharacterEquippedItemIds> {
        #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
        enum QueryAs {
            InvType,
            ItemId,
            Slot,
        }

        let equip_items: Vec<(i32, i32, i32)> = equip_item::Entity::find()
            .select_only()
            .column_as(inventory_slot::Column::InvType, QueryAs::InvType)
            .column_as(equip_item::Column::ItemId, QueryAs::ItemId)
            .column_as(inventory_slot::Column::Slot, QueryAs::Slot)
            .inner_join(inventory_slot::Entity)
            .filter(inventory_slot::Column::InvType.is_in([
                InventoryType::Equipped as i32,
                InventoryType::Special as i32,
            ]))
            .filter(inventory_slot::Column::CharId.eq(char_id))
            .into_values::<_, QueryAs>()
            .all(&self.db)
            .await?;

        equip_items.iter().try_fold(
            CharacterEquippedItemIds::default(),
            |mut acc, &(inv_ty, item_id, slot)| {
                let item = (CharEquipSlot::try_from(slot as u8)?, ItemId(item_id as u32));
                // Inv type has to be either equipped or maskedequipped
                match InventoryType::try_from_primitive(inv_ty as u8).unwrap() {
                    InventoryType::Equipped => acc.equipped.push(item),
                    InventoryType::Special => acc.masked.push(item),
                    _ => unreachable!(),
                };
                Ok(acc)
            },
        )
    }
}

#[cfg(test)]
mod tests {

    use proto95::{
        id::{job_id::JobGroup, FaceId, HairId, Skin},
        shared::{inventory::CharEquipSlot, Gender},
    };

    use crate::{
        gen_sqlite,
        services::{
            data::{
                account::{AccountId, Region},
                character::{CharacterCreateDTO, CharacterID, ItemStarterSet},
                AccountService, CharacterService,
            },
            helper::inv::stack::InvStackItem,
            meta::meta_service::MetaService,
        },
    };

    use super::ItemService;

    fn get_mock_meta() -> &'static MetaService {
        Box::leak(Box::new(
            MetaService::load_from_dir("../../game_data/rbin").expect("Meta"),
        ))
    }

    async fn get_svc() -> anyhow::Result<(ItemService, AccountId, CharacterID)> {
        let db = gen_sqlite(crate::SQL_OPT_MEMORY).await?;

        let acc = AccountService::new(db.clone());
        let acc_id = acc
            .create("test", "hunter3", Region::Europe, true, None)
            .await?;

        let meta = get_mock_meta();

        let char = CharacterService::new(db.clone(), meta);
        let item_svc = ItemService::new(db.clone(), meta);
        let job = JobGroup::Legend;
        let char_id = char
            .create_character(
                acc_id,
                CharacterCreateDTO {
                    name: "Aran".to_string(),
                    job_group: JobGroup::Legend,
                    face: FaceId::FEARFUL_STARE_F,
                    skin: Skin::White,
                    hair: HairId::BLACK_TOBEN,
                    starter_set: ItemStarterSet {
                        bottom: job.get_starter_bottoms().next().unwrap(),
                        shoes: job.get_starter_shoes().next().unwrap(),
                        top: job.get_starter_tops().next().unwrap(),
                        weapon: job.get_starter_weapons().next().unwrap(),
                        guide: job.get_guide_item(),
                    },
                    gender: Gender::Male,
                },
                &item_svc,
            )
            .await?;

        Ok((item_svc, acc_id, char_id))
    }

    #[tokio::test]
    async fn load_save_inventory() {
        let (svc, _acc_id, char_id) = get_svc().await.unwrap();
        let _inv = svc.load_inventory_for_character(char_id).await.unwrap();
        svc.create_starter_set(
            char_id,
            ItemStarterSet::default_starter_set(JobGroup::Adventurer),
        )
        .await
        .unwrap();

        let mut inv = svc.load_inventory_for_character(char_id).await.unwrap();
        assert_eq!(inv.equipped.len(), 4);
        assert_eq!(inv.etc.len(), 1);
        assert!(inv.equipped.remove(CharEquipSlot::Top).unwrap().is_some());
        let stack_1 = inv.etc.get_mut(0).unwrap();
        stack_1.set_quantity(stack_1.quantity() + 5);

        svc.save_inventory(&mut inv, char_id).await.unwrap();
        let inv = svc.load_inventory_for_character(char_id).await.unwrap();
        assert_eq!(inv.equipped.len(), 3);
        assert_eq!(inv.etc.get(0).unwrap().quantity(), 1 + 5);

        let _eq = svc.load_equipped_items(char_id).await.unwrap();
    }
}
