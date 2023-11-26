use std::ops::RangeInclusive;

use crate::shroom_id;

shroom_id!(FieldId, u32);

impl FieldId {
    pub fn is_shroom_island(&self) -> bool {
        Self::MAPLE_ISLAND_RANGE.contains(self)
    }

    // Aran tutorial / burning intro / godly stat
    pub fn is_aran_tutorial_map(&self) -> bool {
        matches!(
            *self,
            Self::BURNING_FOREST_1 | Self::BURNING_FOREST_2 | Self::BURNING_FOREST_3
        )
    }

    pub fn is_cygnus_intro(&self) -> bool {
        Self::CYGNUS_INTRO_LOCATION_RANGE.contains(self)
    }

    pub fn is_physical_fitness(&self) -> bool {
        Self::PHYSICAL_FITNESS_RANGE.contains(self)
    }

    pub fn is_solo_dojo(&self) -> bool {
        Self::DOJO_RANGE.contains(self)
    }

    pub fn is_party_dojo(&self) -> bool {
        Self::DOJO_PARTY_RANGE.contains(self)
    }

    //TODO what's that?
    pub fn is_self_lootable_only(&self) -> bool {
        Self::HAPPYVILLE_TREE_RANGE.contains(self) || Self::GPQ_FOUNTAIN_RANGE.contains(self)
    }

    pub fn is_ola_ola(&self) -> bool {
        Self::OLA_OLA_RANGE.contains(self)
    }

    pub fn is_boss_rush(&self) -> bool {
        Self::BOSS_RUSH_RANGE.contains(self)
    }

    pub fn is_netts_pyramid(&self) -> bool {
        Self::NETTS_PYRAMID_RANGE.contains(self)
    }

    pub fn is_fishing_area(&self) -> bool {
        matches!(
            *self,
            Self::ON_THE_WAY_TO_THE_HARBOR | Self::PIER_ON_THE_BEACH | Self::PEACEFUL_SHIP
        )
    }

    pub fn is_none(&self) -> bool {
        *self == Self::NONE
    }
}

impl FieldId {
    // Special
    pub const NONE: FieldId = FieldId(999999999);
    pub const GM_MAP: FieldId = FieldId(180000000);
    pub const JAIL: FieldId = FieldId(300000012); // "Cellar: Camp Conference Room"
    pub const DEVELOPERS_HQ: FieldId = FieldId(777777777);

    // Misc
    pub const ORBIS_TOWER_BOTTOM: FieldId = FieldId(200082300);
    pub const INTERNET_CAFE: FieldId = FieldId(193000000);
    pub const CRIMSONWOOD_VALLEY_1: FieldId = FieldId(610020000);
    pub const CRIMSONWOOD_VALLEY_2: FieldId = FieldId(610020001);
    pub const HENESYS_PQ: FieldId = FieldId(910010000);
    pub const ORIGIN_OF_CLOCKTOWER: FieldId = FieldId(220080001);
    pub const CAVE_OF_PIANUS: FieldId = FieldId(230040420);
    pub const GUILD_HQ: FieldId = FieldId(200000301);
    pub const FM_ENTRANCE: FieldId = FieldId(910000000);

    // Beginner
    pub const MUSHROOM_TOWN: FieldId = FieldId(10000);

    // Town
    pub const SOUTHPERRY: FieldId = FieldId(2000000);
    pub const AMHERST: FieldId = FieldId(1000000);
    pub const HENESYS: FieldId = FieldId(100000000);
    pub const ELLINIA: FieldId = FieldId(101000000);
    pub const PERION: FieldId = FieldId(102000000);
    pub const KERNING_CITY: FieldId = FieldId(103000000);
    pub const LITH_HARBOUR: FieldId = FieldId(104000000);
    pub const SLEEPYWOOD: FieldId = FieldId(105040300);
    pub const MUSHROOM_KINGDOM: FieldId = FieldId(106020000);
    pub const FLORINA_BEACH: FieldId = FieldId(110000000);
    pub const EREVE: FieldId = FieldId(130000000);
    pub const KERNING_SQUARE: FieldId = FieldId(103040000);
    pub const RIEN: FieldId = FieldId(140000000);
    pub const ORBIS: FieldId = FieldId(200000000);
    pub const EL_NATH: FieldId = FieldId(211000000);
    pub const LUDIBRIUM: FieldId = FieldId(220000000);
    pub const AQUARIUM: FieldId = FieldId(230000000);
    pub const LEAFRE: FieldId = FieldId(240000000);
    pub const NEO_CITY: FieldId = FieldId(240070000);
    pub const MU_LUNG: FieldId = FieldId(250000000);
    pub const HERB_TOWN: FieldId = FieldId(251000000);
    pub const OMEGA_SECTOR: FieldId = FieldId(221000000);
    pub const KOREAN_FOLK_TOWN: FieldId = FieldId(222000000);
    pub const ARIANT: FieldId = FieldId(260000000);
    pub const MAGATIA: FieldId = FieldId(261000000);
    pub const TEMPLE_OF_TIME: FieldId = FieldId(270000100);
    pub const ELLIN_FOREST: FieldId = FieldId(300000000);
    pub const SINGAPORE: FieldId = FieldId(540000000);
    pub const BOAT_QUAY_TOWN: FieldId = FieldId(541000000);
    pub const KAMPUNG_VILLAGE: FieldId = FieldId(551000000);
    pub const NEW_LEAF_CITY: FieldId = FieldId(600000000);
    pub const MUSHROOM_SHRINE: FieldId = FieldId(800000000);
    pub const SHOWA_TOWN: FieldId = FieldId(801000000);
    pub const NAUTILUS_HARBOR: FieldId = FieldId(120000000);
    pub const HAPPYVILLE: FieldId = FieldId(209000000);

    pub const SHOWA_SPA_M: FieldId = FieldId(809000101);
    pub const SHOWA_SPA_F: FieldId = FieldId(809000201);

    pub(crate) const MAPLE_ISLAND_MIN: FieldId = FieldId(0);
    pub(crate) const MAPLE_ISLAND_MAX: FieldId = FieldId(2000001);
    pub(crate) const MAPLE_ISLAND_RANGE: RangeInclusive<FieldId> =
        (Self::MAPLE_ISLAND_MIN..=Self::MAPLE_ISLAND_MAX);

    // Travel
    // There are 10 of each of these travel maps in the files
    pub const FROM_LITH_TO_RIEN: FieldId = FieldId(200090060);
    pub const FROM_RIEN_TO_LITH: FieldId = FieldId(200090070);
    pub const DANGEROUS_FOREST: FieldId = FieldId(140020300); // Rien docks
    pub const FROM_ELLINIA_TO_EREVE: FieldId = FieldId(200090030);
    pub const SKY_FERRY: FieldId = FieldId(130000210); // Ereve platform
    pub const FROM_EREVE_TO_ELLINIA: FieldId = FieldId(200090031);
    pub const ELLINIA_SKY_FERRY: FieldId = FieldId(101000400);
    pub const FROM_EREVE_TO_ORBIS: FieldId = FieldId(200090021);
    pub const ORBIS_STATION: FieldId = FieldId(200000161);
    pub const FROM_ORBIS_TO_EREVE: FieldId = FieldId(200090020);

    // Aran
    pub const ARAN_TUTORIAL_START: FieldId = FieldId(914000000);
    pub const ARAN_TUTORIAL_MAX: FieldId = FieldId(914000500);
    pub const ARAN_INTRO: FieldId = FieldId(140090000);
    pub(crate) const BURNING_FOREST_1: FieldId = FieldId(914000200);
    pub(crate) const BURNING_FOREST_2: FieldId = FieldId(914000210);
    pub(crate) const BURNING_FOREST_3: FieldId = FieldId(914000220);

    // Aran intro
    pub const ARAN_TUTO_1: FieldId = FieldId(914090010);
    pub const ARAN_TUTO_2: FieldId = FieldId(914090011);
    pub const ARAN_TUTO_3: FieldId = FieldId(914090012);
    pub const ARAN_TUTO_4: FieldId = FieldId(914090013);
    pub const ARAN_POLEARM: FieldId = FieldId(914090100);
    pub const ARAN_MAHA: FieldId = FieldId(914090200); // Black screen when warped to

    // Starting map Evan
    pub const STARTING_MAP_EVAN: FieldId = FieldId(100030100);

    // Starting map
    pub const STARTING_MAP_NOBLESSE: FieldId = FieldId(130030000);

    // Edelstein Starting map
    pub const STARTING_MAP_RESISTANCE: FieldId = FieldId(310010000);

    // Cygnus intro
    // These are the actual maps
    pub(crate) const CYGNUS_INTRO_LOCATION_MIN: FieldId = FieldId(913040000);
    pub(crate) const CYGNUS_INTRO_LOCATION_MAX: FieldId = FieldId(913040006);
    pub(crate) const CYGNUS_INTRO_LOCATION_RANGE: RangeInclusive<FieldId> =
        (Self::CYGNUS_INTRO_LOCATION_MIN..=Self::CYGNUS_INTRO_LOCATION_MAX);

    // Cygnus intro video
    pub const CYGNUS_INTRO_LEAD: FieldId = FieldId(913040100);
    pub const CYGNUS_INTRO_WARRIOR: FieldId = FieldId(913040101);
    pub const CYGNUS_INTRO_BOWMAN: FieldId = FieldId(913040102);
    pub const CYGNUS_INTRO_MAGE: FieldId = FieldId(913040103);
    pub const CYGNUS_INTRO_PIRATE: FieldId = FieldId(913040104);
    pub const CYGNUS_INTRO_THIEF: FieldId = FieldId(913040105);
    pub const CYGNUS_INTRO_CONCLUSION: FieldId = FieldId(913040106);

    // Event
    pub const EVENT_COCONUT_HARVEST: FieldId = FieldId(109080000);
    pub const EVENT_OX_QUIZ: FieldId = FieldId(109020001);
    pub const EVENT_PHYSICAL_FITNESS: FieldId = FieldId(109040000);
    pub const EVENT_OLA_OLA_0: FieldId = FieldId(109030001);
    pub const EVENT_OLA_OLA_1: FieldId = FieldId(109030101);
    pub const EVENT_OLA_OLA_2: FieldId = FieldId(109030201);
    pub const EVENT_OLA_OLA_3: FieldId = FieldId(109030301);
    pub const EVENT_OLA_OLA_4: FieldId = FieldId(109030401);
    pub const EVENT_SNOWBALL: FieldId = FieldId(109060000);
    pub const EVENT_FIND_THE_JEWEL: FieldId = FieldId(109010000);
    pub const FITNESS_EVENT_LAST: FieldId = FieldId(109040004);
    pub const OLA_EVENT_LAST_1: FieldId = FieldId(109030003);
    pub const OLA_EVENT_LAST_2: FieldId = FieldId(109030103);
    pub const WITCH_TOWER_ENTRANCE: FieldId = FieldId(980040000);
    pub const EVENT_WINNER: FieldId = FieldId(109050000);
    pub const EVENT_EXIT: FieldId = FieldId(109050001);
    pub const EVENT_SNOWBALL_ENTRANCE: FieldId = FieldId(109060001);

    pub(crate) const PHYSICAL_FITNESS_MIN: FieldId = Self::EVENT_PHYSICAL_FITNESS;
    pub(crate) const PHYSICAL_FITNESS_MAX: FieldId = Self::FITNESS_EVENT_LAST;
    pub(crate) const PHYSICAL_FITNESS_RANGE: RangeInclusive<FieldId> =
        (Self::PHYSICAL_FITNESS_MIN..=Self::PHYSICAL_FITNESS_MAX);

    pub(crate) const OLA_OLA_MIN: FieldId = Self::EVENT_OLA_OLA_0;
    pub(crate) const OLA_OLA_MAX: FieldId = FieldId(109030403); // OLA_OLA_4 level 3
    pub(crate) const OLA_OLA_RANGE: RangeInclusive<FieldId> = (Self::OLA_OLA_MIN..=Self::OLA_OLA_MAX);

    // Self lootable maps
    pub(crate) const HAPPYVILLE_TREE_MIN: FieldId = FieldId(209000001);
    pub(crate) const HAPPYVILLE_TREE_MAX: FieldId = FieldId(209000015);
    pub(crate) const HAPPYVILLE_TREE_RANGE: RangeInclusive<FieldId> =
        (Self::HAPPYVILLE_TREE_MIN..=Self::HAPPYVILLE_TREE_MAX);

    pub(crate) const GPQ_FOUNTAIN_MIN: FieldId = FieldId(990000500);
    pub(crate) const GPQ_FOUNTAIN_MAX: FieldId = FieldId(990000502);
    pub(crate) const GPQ_FOUNTAIN_RANGE: RangeInclusive<FieldId> =
        (Self::GPQ_FOUNTAIN_MIN..=Self::GPQ_FOUNTAIN_MAX);

    // Dojo
    pub const DOJO_SOLO_BASE: FieldId = FieldId(925020000);
    pub const DOJO_PARTY_BASE: FieldId = FieldId(925030000);
    pub const DOJO_EXIT: FieldId = FieldId(925020002);

    pub(crate) const DOJO_MIN: FieldId = Self::DOJO_SOLO_BASE;
    pub(crate) const DOJO_MAX: FieldId = FieldId(925033804);
    pub(crate) const DOJO_RANGE: RangeInclusive<FieldId> = (Self::DOJO_MIN..=Self::DOJO_MAX);

    pub(crate) const DOJO_PARTY_MIN: FieldId = FieldId(925030100);
    pub const DOJO_PARTY_MAX: FieldId = Self::DOJO_MAX;
    pub(crate) const DOJO_PARTY_RANGE: RangeInclusive<FieldId> =
        (Self::DOJO_PARTY_MIN..=Self::DOJO_PARTY_MAX);

    // Mini dungeon
    pub const ANT_TUNNEL_2: FieldId = FieldId(105050100);
    pub const CAVE_OF_MUSHROOMS_BASE: FieldId = FieldId(105050101);
    pub const SLEEPY_DUNGEON_4: FieldId = FieldId(105040304);
    pub const GOLEMS_CASTLE_RUINS_BASE: FieldId = FieldId(105040320);
    pub const SAHEL_2: FieldId = FieldId(260020600);
    pub const HILL_OF_SANDSTORMS_BASE: FieldId = FieldId(260020630);
    pub const RAIN_FOREST_EAST_OF_HENESYS: FieldId = FieldId(100020000);
    pub const HENESYS_PIG_FARM_BASE: FieldId = FieldId(100020100);
    pub const COLD_CRADLE: FieldId = FieldId(105090311);
    pub const DRAKES_BLUE_CAVE_BASE: FieldId = FieldId(105090320);
    pub const EOS_TOWER_76TH_TO_90TH_FLOOR: FieldId = FieldId(221023400);
    pub const DRUMMER_BUNNYS_LAIR_BASE: FieldId = FieldId(221023401);
    pub const BATTLEFIELD_OF_FIRE_AND_WATER: FieldId = FieldId(240020500);
    pub const ROUND_TABLE_OF_KENTAURUS_BASE: FieldId = FieldId(240020512);
    pub const RESTORING_MEMORY_BASE: FieldId = FieldId(240040800);
    pub const DESTROYED_DRAGON_NEST: FieldId = FieldId(240040520);
    pub const NEWT_SECURED_ZONE_BASE: FieldId = FieldId(240040900);
    pub const RED_NOSE_PIRATE_DEN_2: FieldId = FieldId(251010402);
    pub const PILLAGE_OF_TREASURE_ISLAND_BASE: FieldId = FieldId(251010410);
    pub const LAB_AREA_C1: FieldId = FieldId(261020300);
    pub const CRITICAL_ERROR_BASE: FieldId = FieldId(261020301);
    pub const FANTASY_THEME_PARK_3: FieldId = FieldId(551030000);
    pub const LONGEST_RIDE_ON_BYEBYE_STATION: FieldId = FieldId(551030001);

    // Boss rush
    pub(crate) const BOSS_RUSH_MIN: FieldId = FieldId(970030100);
    pub(crate) const BOSS_RUSH_MAX: FieldId = FieldId(970042711);
    pub(crate) const BOSS_RUSH_RANGE: RangeInclusive<FieldId> =
        (Self::BOSS_RUSH_MIN..=Self::BOSS_RUSH_MAX);

    // ARPQ
    pub const ARPQ_LOBBY: FieldId = FieldId(980010000);
    pub const ARPQ_ARENA_1: FieldId = FieldId(980010101);
    pub const ARPQ_ARENA_2: FieldId = FieldId(980010201);
    pub const ARPQ_ARENA_3: FieldId = FieldId(980010301);
    pub const ARPQ_KINGS_ROOM: FieldId = FieldId(980010010);

    // Nett's pyramid
    pub const NETTS_PYRAMID: FieldId = FieldId(926010001);
    pub const NETTS_PYRAMID_SOLO_BASE: FieldId = FieldId(926010100);
    pub const NETTS_PYRAMID_PARTY_BASE: FieldId = FieldId(926020100);
    pub(crate) const NETTS_PYRAMID_MIN: FieldId = Self::NETTS_PYRAMID_SOLO_BASE;
    pub(crate) const NETTS_PYRAMID_MAX: FieldId = FieldId(926023500);
    pub(crate) const NETTS_PYRAMID_RANGE: RangeInclusive<FieldId> =
        (Self::NETTS_PYRAMID_MIN..=Self::NETTS_PYRAMID_MAX);

    // Fishing
    pub(crate) const ON_THE_WAY_TO_THE_HARBOR: FieldId = FieldId(120010000);
    pub(crate) const PIER_ON_THE_BEACH: FieldId = FieldId(251000100);
    pub(crate) const PEACEFUL_SHIP: FieldId = FieldId(541010110);

    // Wedding
    pub const AMORIA: FieldId = FieldId(680000000);
    pub const CHAPEL_WEDDING_ALTAR: FieldId = FieldId(680000110);
    pub const CATHEDRAL_WEDDING_ALTAR: FieldId = FieldId(680000210);
    pub const WEDDING_PHOTO: FieldId = FieldId(680000300);
    pub const WEDDING_EXIT: FieldId = FieldId(680000500);

    // Statue
    pub const HALL_OF_WARRIORS: FieldId = FieldId(102000004); // Explorer
    pub const HALL_OF_MAGICIANS: FieldId = FieldId(101000004);
    pub const HALL_OF_BOWMEN: FieldId = FieldId(100000204);
    pub const HALL_OF_THIEVES: FieldId = FieldId(103000008);
    pub const NAUTILUS_TRAINING_ROOM: FieldId = FieldId(120000105);
    pub const KNIGHTS_CHAMBER: FieldId = FieldId(130000100); // Cygnus
    pub const KNIGHTS_CHAMBER_2: FieldId = FieldId(130000110);
    pub const KNIGHTS_CHAMBER_3: FieldId = FieldId(130000120);
    pub const KNIGHTS_CHAMBER_LARGE: FieldId = FieldId(130000101);
    pub const PALACE_OF_THE_MASTER: FieldId = FieldId(140010110); // Aran

    // gm-goto
    pub const EXCAVATION_SITE: FieldId = FieldId(990000000);
    pub const SOMEONE_ELSES_HOUSE: FieldId = FieldId(100000005);
    pub const GRIFFEY_FOREST: FieldId = FieldId(240020101);
    pub const MANONS_FOREST: FieldId = FieldId(240020401);
    pub const HOLLOWED_GROUND: FieldId = FieldId(682000001);
    pub const CURSED_SANCTUARY: FieldId = FieldId(105090900);
    pub const DOOR_TO_ZAKUM: FieldId = FieldId(211042300);
    pub const DRAGON_NEST_LEFT_BEHIND: FieldId = FieldId(240040511);
    pub const HENESYS_PARK: FieldId = FieldId(100000200);
    pub const ENTRANCE_TO_HORNTAILS_CAVE: FieldId = FieldId(240050400);
    pub const FORGOTTEN_TWILIGHT: FieldId = FieldId(270050000);
    pub const CRIMSONWOOD_KEEP: FieldId = FieldId(610020006);
    pub const MU_LUNG_DOJO_HALL: FieldId = FieldId(925020001);
    pub const EXCLUSIVE_TRAINING_CENTER: FieldId = FieldId(970030000);
}
