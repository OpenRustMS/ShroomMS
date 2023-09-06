//TODO find a way to auto generate this from wz files or verify this with files during build time

pub mod item_id;
pub mod job_id;
pub mod map_id;

use shroom_net::shroom_enum_code;

pub use self::item_id::ItemId;
pub use self::job_id::JobClass;
use self::job_id::JobId;
pub use self::map_id::MapId;

#[macro_export]
macro_rules! shroom_id {
    ($name:ident, $ty:ty) => {
        #[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
        pub struct $name(pub $ty);

        impl shroom_net::packet::proto::PacketWrapped for $name {
            type Inner = $ty;

            fn packet_into_inner(&self) -> Self::Inner {
                self.0
            }

            fn packet_from(v: Self::Inner) -> Self {
                Self(v)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.0)
            }
        }
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
        let mut cond = false;
        if (self.0 <= 4111001) {
            if (self.0 == 0x3eba99) {
                return true;
            }
            if (self.0 <= 2121000) {
                if (self.0 == 0x205d28) {
                    return true;
                }
                if (self.0 <= 0x12a188) {
                    if (self.0 == 0x12a188) {
                        return true;
                    }
                    if (self.0 > 0x111ae8) {
                        if (self.0 == 0x12536e) {
                            return true;
                        }
                        if (self.0 <= 0x127a80) {
                            return false;
                        }
                        if (self.0 <= 0x127a82) {
                            return true;
                        }
                        return false;
                    }
                    if (self.0 == 0x111ae8) {
                        return true;
                    }
                    if (self.0 == 0xf462b) {
                        return true;
                    }
                    if (self.0 == 0x10ccce) {
                        return true;
                    }
                    cond = self.0 == 0x10f3df;
                } else if (self.0 > 0x142828) {
                    if (self.0 == 0x200f09) {
                        return true;
                    }
                    cond = self.0 == 0x200f0b;
                } else {
                    if (self.0 == 0x142828) {
                        return true;
                    }
                    if (self.0 < 0x13da0e) {
                        return false;
                    }
                    if (self.0 <= 0x13da0f) {
                        return true;
                    }
                    cond = self.0 == 0x14011f;
                }
            } else if (self.0 <= 0x236a68) {
                if (self.0 == 0x236a68) {
                    return true;
                }
                if (self.0 > 0x231c4c) {
                    if (self.0 == 0x234359) {
                        return true;
                    }
                    cond = self.0 == 0x23435b;
                } else {
                    if (self.0 == 0x231c4c) {
                        return true;
                    }
                    if (self.0 == 0x2195a9) {
                        return true;
                    }
                    if (self.0 == 0x2195ab) {
                        return true;
                    }
                    cond = self.0 == 0x21e3c8;
                }
            } else if (self.0 > 0x2f9f6a) {
                if (self.0 == 0x312608) {
                    return true;
                }
                cond = self.0 == 0x3e938c;
            } else {
                if (self.0 == 0x2f9f6a) {
                    return true;
                }
                if (self.0 == 0x236a6d) {
                    return true;
                }
                cond = self.0 == 0x2f9f68;
            }
        } else if (self.0 <= 0xa9634b) {
            if (self.0 == 0xa9634b) {
                return true;
            }
            if (self.0 <= 0x4dfcdf) {
                if (self.0 == 0x4dfcdf) {
                    return true;
                }
                if (self.0 > 0x41c7d9) {
                    if (self.0 == 0x423d08) {
                        return true;
                    }
                    cond = self.0 == 0x423d0f;
                } else {
                    if (self.0 == 0x41c7d9) {
                        return true;
                    }
                    if (self.0 == 0x3ee1a8) {
                        return true;
                    }
                    if (self.0 == 0x401a2b) {
                        return true;
                    }
                    cond = self.0 == 0x406848;
                }
            } else if (self.0 > 0x4f837f) {
                if self.0 == 0x4faa88 {
                    return true;
                }
                cond = self.0 == 0xa7dca9;
            } else {
                if self.0 == 0x4f837f {
                    return true;
                }
                if self.0 == 0x4e23e8 {
                    return true;
                }
                cond = self.0 == 0x4e23f1;
            }
        } else if self.0 <= 0x1524d78 {
            if self.0 == 0x1524d78 {
                return true;
            }
            if self.0 > 0xe6935d {
                if self.0 == 0x14247e8 {
                    return true;
                }
                cond = self.0 == 0x151d84b;
            } else {
                if self.0 == 0xe6935d {
                    return true;
                }
                if self.0 < 0xb8a588 {
                    return false;
                }
                if self.0 <= 0xb8a589 {
                    return true;
                }
                cond = self.0 == 0xd72a0b;
            }
        } else if self.0 > 0x1ea20af {
            if self.0 == 0x1f962ef {
                return true;
            }
            cond = self.0 == 0x217c065;
        } else {
            if self.0 == 0x1ea20af {
                return true;
            }
            if self.0 == 0x1527488 {
                return true;
            }
            cond = self.0 == 0x1e9f99c;
        }
        if (!cond) {
            return false;
        }
        return true;
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
