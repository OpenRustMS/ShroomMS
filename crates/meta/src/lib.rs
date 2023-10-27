pub mod drops;
pub mod shared;
pub mod skill;
pub mod svc;
pub mod wz2;

pub mod schemas {
    //#[allow(unused)]
    //pub mod skill;
    pub mod field_mapper;
    pub mod shroom_schemas;
    #[allow(unused)]
    pub mod skill_mapper;

    impl From<shroom_schemas::Bool> for bool {
        fn from(val: shroom_schemas::Bool) -> Self {
            match val {
                shroom_schemas::Bool::Str(v) => v.as_str() == "1",
                shroom_schemas::Bool::Int(v) => v != 0,
            }
        }
    }

    impl From<&shroom_schemas::Bool> for bool {
        fn from(val: &shroom_schemas::Bool) -> Self {
            match val {
                shroom_schemas::Bool::Str(v) => v.as_str() == "1",
                shroom_schemas::Bool::Int(v) => *v != 0,
            }
        }
    }

    impl From<shroom_schemas::StrOrNum> for i64 {
        fn from(val: shroom_schemas::StrOrNum) -> Self {
            match val {
                shroom_schemas::StrOrNum::NumStr(s) => s.parse().unwrap(),
                shroom_schemas::StrOrNum::Int(v) => v,
            }
        }
    }

    impl From<&shroom_schemas::StrOrNum> for i64 {
        fn from(val: &shroom_schemas::StrOrNum) -> Self {
            match val {
                shroom_schemas::StrOrNum::NumStr(s) => s.parse().unwrap(),
                shroom_schemas::StrOrNum::Int(v) => *v,
            }
        }
    }

    impl From<&shroom_schemas::StrOrNum> for u32 {
        fn from(val: &shroom_schemas::StrOrNum) -> Self {
            match val {
                shroom_schemas::StrOrNum::NumStr(s) => s.parse().unwrap(),
                shroom_schemas::StrOrNum::Int(v) => *v as u32,
            }
        }
    }

    impl From<shroom_schemas::Vec2> for crate::shared::Vec2 {
        fn from(val: shroom_schemas::Vec2) -> crate::shared::Vec2 {
            Self {
                x: val.x as i32,
                y: val.y as i32,
            }
        }
    }

    impl From<(shroom_schemas::Vec2, shroom_schemas::Vec2)> for crate::shared::Rect {
        fn from(val: (shroom_schemas::Vec2, shroom_schemas::Vec2)) -> crate::shared::Rect {
            Self {
                lt: val.0.into(),
                rb: val.1.into(),
            }
        }
    }

    impl From<(shroom_schemas::Vec2, u32)> for crate::shared::Circ {
        fn from(val: (shroom_schemas::Vec2, u32)) -> crate::shared::Circ {
            Self {
                sp: val.0.into(),
                radius: val.1,
            }
        }
    }
}

pub mod field {
    pub mod fh_tree;
    pub use fh_tree::FhTree;
}

pub use schemas::field_mapper::*;
pub use svc::*;

pub const FIELD_REGIONS: [u8; 9] = [0, 1, 2, 3, 5, 6, 7, 8, 9];
