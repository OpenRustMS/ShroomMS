use futures::future::try_join_all;

use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

use crate::helper::{stats::with_char_stats, stats::with_equip_stats, *};

#[derive(Iden)]
enum Account {
    Table,
    Id,
    Username,
    PasswordHash,
    Gender,
    AcceptedTos,
    LastLoginAt,
    CreatedAt,
    Pin,
    Pic,
    Country,
    GmLevel,
    LastSelectedWorld,
    CharacterSlots,
    NxCredit,
    NxPrepaid,
    ShroomPoints,
    Tester,
}

#[derive(Iden)]
enum Ban {
    Table,
    Id,
    Reason,
    Time,
    AccId,
}

#[derive(Iden)]
enum Character {
    Table,
    Id,
    AccId,
    Name,
    CreatedAt,
    LastLoginAt,
    Gender,
    SkillPoints,
    PlayTime,
}

#[derive(Iden)]
enum ItemStack {
    Table,
    Id,
    ItemId,
    CashId,
    ExpiresAt,
    Quantity,
    Flags,
}

#[derive(Iden)]
enum EquipItem {
    Table,
    Id,
    ItemId,
    CashId,
    ExpiresAt,
    Flags,
    OwnerTag,
    ItemLevel,
    ItemExp,
    ViciousHammers,
}

#[derive(Iden)]
enum PetItem {
    Table,
    Id,
    ItemId,
    CashId,
    ExpiresAt,
    Flags,
    Name,
    Level,
    Tameness,
    Fullness,
    Skill,
    RemainingLife,
    Summoned,
}

#[derive(Iden)]
enum InventorySlot {
    Table,
    Id,
    InvType,
    Slot,
    CharId,
    EquipItemId,
    StackItemId,
    PetItemId,
}

#[derive(Iden)]
enum Skill {
    Table,
    Id,
    CharId,
    GameId,
    Level,
    MasterLevel,
    ExpiresAt,
    Cooldown,
}

#[derive(DeriveMigrationName)]
pub struct Migration {
    acc_table: ShroomTbl,
    char_table: ShroomTbl,
    ban_table: ShroomTbl,
    eq_table: ShroomTbl,
    stack_item_table: ShroomTbl,
    pet_item_table: ShroomTbl,
    inv_slot_table: ShroomTbl,
    skill_table: ShroomTbl,
}

impl Default for Migration {
    fn default() -> Self {
        let acc_table = ShroomTbl::new(
            Account::Table,
            Account::Id,
            [
                ColumnDef::new(Account::Username)
                    .string()
                    .not_null()
                    .unique_key()
                    .to_owned(),
                ColumnDef::new(Account::PasswordHash)
                    .string()
                    .not_null()
                    .to_owned(),
                shroom_bool(Account::AcceptedTos),
                shroom_gender_col(Account::Gender).null().to_owned(),
                date_time(Account::LastLoginAt),
                created_at(Account::CreatedAt),
                shroom_small_str(Account::Pin),
                shroom_small_str(Account::Pic),
                shroom_id(Account::Country),
                shroom_int(Account::GmLevel),
                shroom_id(Account::LastSelectedWorld),
                shroom_size(Account::CharacterSlots),
                shroom_size(Account::NxCredit),
                shroom_size(Account::NxPrepaid),
                shroom_size(Account::ShroomPoints),
                shroom_bool(Account::Tester),
            ],
            [],
        );

        let char_table = ShroomTbl::new(
            Character::Table,
            Character::Id,
            with_char_stats([
                shroom_name(Character::Name),
                created_at(Character::CreatedAt),
                date_time(Character::LastLoginAt),
                shroom_gender_col(Character::Gender).not_null().to_owned(),
                shroom_skill_points(Character::SkillPoints),
                shroom_int(Character::PlayTime),
            ]),
            [Ref::ownership(Character::AccId, &acc_table)],
        );

        let ban_table = ShroomTbl::new(
            Ban::Table,
            Ban::Id,
            [shroom_str(Ban::Reason), date_time(Ban::Time)],
            [Ref::ownership(Ban::AccId, &acc_table)],
        );

        let item_stack_table = ShroomTbl::new(
            ItemStack::Table,
            ItemStack::Id,
            [
                date_time(ItemStack::ExpiresAt),
                mopple_cash_id(ItemStack::CashId),
                shroom_id(ItemStack::ItemId),
                shroom_int(ItemStack::Flags),
                shroom_size(ItemStack::Quantity),
            ],
            [],
        );

        let item_equip_table = ShroomTbl::new(
            EquipItem::Table,
            EquipItem::Id,
            with_equip_stats([
                date_time(EquipItem::ExpiresAt),
                mopple_cash_id(EquipItem::CashId),
                shroom_id(EquipItem::ItemId),
                shroom_int(EquipItem::Flags),
                shroom_size(EquipItem::ItemLevel),
                shroom_size(EquipItem::ItemExp),
                shroom_size(EquipItem::ViciousHammers),
                shroom_name(EquipItem::OwnerTag),
            ]),
            [],
        );

        let item_pet_table = ShroomTbl::new(
            PetItem::Table,
            PetItem::Id,
            [
                date_time(PetItem::ExpiresAt),
                mopple_cash_id(PetItem::CashId),
                shroom_id(PetItem::ItemId),
                shroom_int(PetItem::Flags),
                shroom_name(PetItem::Name),
                shroom_stat(PetItem::Level),
                shroom_stat(PetItem::Tameness),
                shroom_stat(PetItem::Fullness),
                shroom_stat(PetItem::Skill),
                shroom_stat(PetItem::RemainingLife),
                shroom_bool(PetItem::Summoned),
            ],
            [],
        );

        let inv_slot_table = ShroomTbl::new(
            InventorySlot::Table,
            InventorySlot::Id,
            [
                shroom_int(InventorySlot::InvType),
                shroom_int(InventorySlot::Slot),
            ],
            [
                Ref::ownership(InventorySlot::CharId, &char_table),
                Ref::opt(InventorySlot::EquipItemId, &item_equip_table),
                Ref::opt(InventorySlot::StackItemId, &item_stack_table),
                Ref::opt(InventorySlot::PetItemId, &item_pet_table),
            ],
        );

        let skill_table = ShroomTbl::new(
            Skill::Table,
            Skill::Id,
            [
                shroom_id(Skill::GameId),
                shroom_int(Skill::Level),
                shroom_int(Skill::MasterLevel),
                date_time(Skill::ExpiresAt),
                date_time(Skill::Cooldown),
            ],
            [Ref::ownership(Skill::CharId, &char_table)],
        );

        Self {
            acc_table,
            char_table,
            ban_table,
            eq_table: item_equip_table,
            stack_item_table: item_stack_table,
            pet_item_table: item_pet_table,
            inv_slot_table,
            skill_table,
        }
    }
}

impl Migration {
    fn table_iter(&self) -> impl Iterator<Item = &'_ ShroomTbl> {
        [
            &self.acc_table,
            &self.char_table,
            &self.ban_table,
            &self.eq_table,
            &self.pet_item_table,
            &self.stack_item_table,
            &self.inv_slot_table,
            &self.skill_table,
        ]
        .into_iter()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(shroom_gender_ty()).await?;

        for tbl in self.table_iter() {
            tbl.create_table(manager).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        try_join_all(self.table_iter().map(|tbl| tbl.drop_fk(manager))).await?;
        for tbl in self.table_iter() {
            tbl.drop_table(manager).await?;
        }

        manager
            .drop_type(Type::drop().name(Gender::GenderTy).to_owned())
            .await?;

        Ok(())
    }
}
