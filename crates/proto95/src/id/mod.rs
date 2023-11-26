//TODO find a way to auto generate this from wz files or verify this with files during build time

pub mod item_id;
pub mod job_id;
pub mod map_id;

use shroom_pkt::shroom_enum_code;

pub use self::item_id::ItemId;
pub use self::job_id::JobClass;
pub use self::map_id::FieldId;

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
        matches!(
            self.0,
            1120004
                | 1120003
                | 1121000
                | 1121001
                | 1121002
                | 1121008
                | 1121010
                | 1121011
                | 1121006
                | 1220005
                | 1220006
                | 1221000
                | 1221002
                | 1221012
                | 1220010
                | 1221011
                | 1221007
                | 1221009
                | 1221004
                | 1320008
                | 1321010
                | 1320009
                | 1321002
                | 1321007
                | 1320005
                | 1320006
                | 1321001
                | 1321000
                | 1321003
                | 21120002
                | 21120001
                | 21120005
                | 21121003
                | 21120004
                | 21120006
                | 21120007
                | 21121008
                | 21120009
                | 21120010
                | 21121000
                | 2121001
                | 2121002
                | 2121004
                | 2121005
                | 2121008
                | 2121000
                | 2121003
                | 2121006
                | 2121007
                | 22111001
                | 22140000
                | 22141002
                | 22171000
                | 22170001
                | 22171002
                | 22171003
                | 22171004
                | 22181000
                | 22181001
                | 22181002
                | 22181003
                | 2221001
                | 2221002
                | 2221004
                | 2221005
                | 2221008
                | 2221000
                | 2221003
                | 2221006
                | 2221007
                | 2321000
                | 2321001
                | 2321002
                | 2321004
                | 2321007
                | 2321003
                | 2321006
                | 2321008
                | 2321009
                | 2321005
                | 3121000
                | 3121002
                | 3121004
                | 3121007
                | 3121006
                | 3121009
                | 3120005
                | 3121008
                | 3121003
                | 32121002
                | 32121003
                | 32121004
                | 32120000
                | 32120001
                | 32121005
                | 32121006
                | 32121007
                | 32121008
                | 3220004
                | 3221000
                | 3221002
                | 3221001
                | 3221006
                | 3221005
                | 3221007
                | 3221008
                | 3221003
                | 33121009
                | 33121002
                | 33121001
                | 33120000
                | 33121004
                | 33121005
                | 33121006
                | 33121007
                | 33121008
                | 35120000
                | 35121005
                | 35121012
                | 35121006
                | 35121003
                | 35121009
                | 35121010
                | 35121011
                | 35120001
                | 35121007
                | 35121008
                | 35121013
                | 4121000
                | 4121003
                | 4121007
                | 4121006
                | 4121009
                | 4120002
                | 4120005
                | 4121004
                | 4121008
                | 4221003
                | 4221008
                | 4220002
                | 4221004
                | 4221007
                | 4221006
                | 4220005
                | 4221000
                | 4221001
                | 4311003
                | 4321000
                | 4331002
                | 4331005
                | 4341000
                | 4340001
                | 4341002
                | 4341003
                | 4341004
                | 4341005
                | 4341006
                | 4341007
                | 4341008
                | 5121002
                | 5121003
                | 5121009
                | 5121010
                | 5121008
                | 5121004
                | 5121005
                | 5121000
                | 5121001
                | 5121007
                | 5220001
                | 5220011
                | 5220002
                | 5221000
                | 5221003
                | 5221004
                | 5221009
                | 5221006
                | 5221007
                | 5221008
                | 5221010
        )
        /*let job = self.0 / 10000;
        let job = JobId::try_from(job as u16).unwrap();

        job.job_level() == 4*/
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
