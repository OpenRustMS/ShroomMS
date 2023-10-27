use proto95::id::{job_id::JobGroup, ItemId};

use crate::services::data::character::check_contains;

#[derive(Debug, Clone)]
pub struct ItemStarterSet {
    pub bottom: ItemId,
    pub shoes: ItemId,
    pub top: ItemId,
    pub weapon: ItemId,
    pub guide: ItemId,
}

impl ItemStarterSet {
    pub fn validate(&self, job: JobGroup) -> anyhow::Result<()> {
        //TODO: update to v95
        let _bottom = check_contains(job.get_starter_bottoms(), self.bottom, "Bottom ID")?;
        let _shoes = check_contains(job.get_starter_shoes(), self.shoes, "Shoes ID")?;
        let _top = check_contains(job.get_starter_tops(), self.top, "Top ID")?;
        let _weapon = check_contains(job.get_starter_weapons(), self.weapon, "Weapon ID")?;
        if self.guide != job.get_guide_item() {
            anyhow::bail!("Invalid starter guide");
        }

        Ok(())
    }

    pub fn default_starter_set(job: JobGroup) -> Self {
        Self {
            shoes: ItemId::LEATHER_SANDALS,
            bottom: ItemId::BLUE_JEAN_SHORTS,
            top: ItemId::WHITE_UNDERSHIRT,
            weapon: ItemId::SWORD,
            guide: job.get_guide_item(),
        }
    }
}