use proto95::shared::char::InventorySize;

use crate::entities::character;

pub const TOTAL_SKILL_PAGES: usize = 10;

pub type SkillPages = [u16; TOTAL_SKILL_PAGES];

impl character::Model {
    pub fn get_skill_pages(&self) -> &SkillPages {
        bytemuck::try_from_bytes(self.skill_points.as_slice()).expect("skill pages")
    }

    pub fn get_skill_pages_mut(&mut self) -> &mut SkillPages {
        bytemuck::try_from_bytes_mut(self.skill_points.as_mut_slice()).expect("skill pages mut")
    }

    pub fn get_skill_points(&self) -> u16 {
        self.get_skill_pages()[0]
    }

    pub fn set_skill_points_mut(&mut self) -> &mut u16 {
        &mut self.get_skill_pages_mut()[0]
    }

    pub fn get_inventory_size(&self) -> InventorySize {
        [
            self.equip_slots as u8,
            self.use_slots as u8,
            self.setup_slots as u8,
            self.etc_slots as u8,
            self.cash_slots as u8,
        ]
    }
}
