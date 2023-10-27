//TODO find a way to auto generate this from wz files or verify this with files during build time

pub mod item_id;
pub mod job_id;
pub mod map_id;

use shroom_pkt::shroom_enum_code;

pub use self::item_id::ItemId;
pub use self::job_id::JobClass;
use self::job_id::JobId;
pub use self::map_id::MapId;

#[macro_export]
macro_rules! shroom_id {
    ($name:ident, $ty:ty) => {
        #[derive(
            Default,
            Debug,
            PartialEq,
            Eq,
            Clone,
            Copy,
            Hash,
            Ord,
            PartialOrd,
            serde::Serialize,
            serde::Deserialize,
        )]
        pub struct $name(pub $ty);

        impl shroom_pkt::PacketWrapped for $name {
            type Inner = $ty;

            fn packet_into_inner(&self) -> Self::Inner {
                self.0
            }

            fn packet_from(v: Self::Inner) -> Self {
                Self(v)
            }
        }
        /*
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }*/
    };
}

shroom_id!(FaceId, u32);

impl FaceId {
    pub const MOTIVATED_LOOK_M: FaceId = FaceId(20000); // Face
    pub const PERPLEXED_STARE: FaceId = FaceId(20001);
    pub const LEISURE_LOOK_M: FaceId = FaceId(20002);
    pub const MOTIVATED_LOOK_F: FaceId = FaceId(21000);
    pub const FEARFUL_STARE_M: FaceId = FaceId(21001);
    pub const LEISURE_LOOK_F: FaceId = FaceId(21002);
    pub const FEARFUL_STARE_F: FaceId = FaceId(21201);
    pub const PERPLEXED_STARE_HAZEL: FaceId = FaceId(20401);
    pub const LEISURE_LOOK_HAZEL: FaceId = FaceId(20402);
    pub const MOTIVATED_LOOK_AMETHYST: FaceId = FaceId(21700);
    pub const MOTIVATED_LOOK_BLUE: FaceId = FaceId(20100);
}

shroom_id!(HairId, u32);

impl HairId {
    pub const BLACK_TOBEN: HairId = HairId(30000); // Hair
    pub const ZETA: HairId = HairId(30010);
    pub const BLACK_REBEL: HairId = HairId(30020);
    pub const BLACK_BUZZ: HairId = HairId(30030);
    pub const BLACK_SAMMY: HairId = HairId(31000);
    pub const BLACK_EDGY: HairId = HairId(31040);
    pub const BLACK_CONNIE: HairId = HairId(31050);
}

shroom_id!(SkillId, u32);

impl SkillId {
    pub fn is_anti_repeat_buff_skill(&self) -> bool {
        matches!(
            self.0,
            1001003
                | 1101006
                | 1111007
                | 1121000
                | 1201006
                | 1211009
                | 1211010
                | 1221000
                | 1301006
                | 1301007
                | 1311007
                | 1321000
                | 2101001
                | 2101003
                | 2121000
                | 2201001
                | 2201003
                | 2221000
                | 2301004
                | 2311001
                | 2311003
                | 2321000
                | 2321005
                | 3121000
                | 3121002
                | 3221000
                | 4101004
                | 4111001
                | 4121000
                | 4201003
                | 4221000
                | 4311001
                | 4341000
                | 4341007
                | 5111007
                | 5121000
                | 5121009
                | 5211007
                | 5221000
                | 11001001
                | 11101003
                | 12101000
                | 12101001
                | 14101003
                | 15111005
                | 21121000
                | 22141003
                | 22171000
                | 22181000
                | 32111004
                | 32121007
                | 33121007
                | 35111013
        )
    }

    pub fn is_dispel(&self) -> bool {
        self.0 == 2311001
    }

    pub fn is_spirit_javelin(&self) -> bool {
        self.0 == 4121006
    }

    pub fn is_monster_magnet(&self) -> bool {
        self.0 % 10000000 == 1004
    }

    pub fn is_charge_skill(&self) -> bool {
        //TODO
        [
            33101005, 33121009, 35001001, 35101009, 22121000, 22151001, 14111006, 15101003,
            3221001, 5201002, 5221004, 2321001, 3121004, 2121001, 4341003,
        ]
        .contains(&self.0)
    }

    pub fn has_targets(&self) -> bool {
        self.0 == 0
    }

    pub fn is_grenade_skill(&self) -> bool {
        [14111006].contains(&self.0)
    }

    pub fn has_master_level(&self) -> bool {
        let job = self.0 / 10000;
        let job = JobId::try_from(job as u16).unwrap();

        job.job_level() == 4
    }
}

shroom_enum_code!(
    Skin,
    u8,
    Normal = 0,
    Dark = 1,
    Black = 2,
    Pale = 3,
    Blue = 4,
    Green = 5,
    White = 9,
    Pink = 10
);
