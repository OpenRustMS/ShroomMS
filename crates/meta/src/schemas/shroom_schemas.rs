#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Bool {
    Str(Str),
    Int(i64),
}
impl From<&Bool> for Bool {
    fn from(value: &Bool) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Bool {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Str(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Int(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for Bool {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Bool {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Bool {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for Bool {
    fn to_string(&self) -> String {
        match self {
            Self::Str(x) => x.to_string(),
            Self::Int(x) => x.to_string(),
        }
    }
}
impl From<Str> for Bool {
    fn from(value: Str) -> Self {
        Self::Str(value)
    }
}
impl From<i64> for Bool {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Canvas {
    #[doc = "height"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[doc = "url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub: Option<serde_json::Map<String, serde_json::Value>>,
    #[doc = "width"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}
impl From<&Canvas> for Canvas {
    fn from(value: &Canvas) -> Self {
        value.clone()
    }
}
impl Canvas {
    pub fn builder() -> builder::Canvas {
        builder::Canvas::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fh {
    #[serde(
        rename = "cantThrough",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cant_through: Option<Bool>,
    #[serde(
        rename = "forbidFallDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forbid_fall_down: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<i64>,
    pub next: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub piece: Option<i64>,
    pub prev: i64,
    pub x1: i64,
    pub x2: i64,
    pub y1: i64,
    pub y2: i64,
}
impl From<&Fh> for Fh {
    fn from(value: &Fh) -> Self {
        value.clone()
    }
}
impl Fh {
    pub fn builder() -> builder::Fh {
        builder::Fh::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Field {
    #[serde(
        rename = "0",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "3",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _3: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "4",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _4: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "5",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _5: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "6",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _6: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "7",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _7: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub area: std::collections::HashMap<String, FieldAreaValue>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "battleField",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub battle_field: Option<FieldBattleField>,
    #[serde(
        rename = "BuffZone",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub buff_zone: std::collections::HashMap<String, FieldBuffZoneValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<FieldClock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coconut: Option<FieldCoconut>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub foothold: std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashMap<String, Fh>>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healer: Option<FieldHealer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<FieldInfo>,
    #[serde(
        rename = "ladderRope",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub ladder_rope: std::collections::HashMap<String, FieldLadderRopeValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub life: std::collections::HashMap<String, FieldLifeValue>,
    #[serde(
        rename = "miniMap",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub mini_map: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "mobMassacre",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mob_massacre: Option<FieldMobMassacre>,
    #[serde(
        rename = "monsterCarnival",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monster_carnival: Option<FieldMonsterCarnival>,
    #[serde(rename = "noSkill", default, skip_serializing_if = "Option::is_none")]
    pub no_skill: Option<FieldNoSkill>,
    #[serde(
        rename = "nodeInfo",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub node_info: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub portal: std::collections::HashMap<String, FieldPortalValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulley: Option<FieldPulley>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub reactor: std::collections::HashMap<String, FieldReactorValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub seat: std::collections::HashMap<String, FieldSeatValue>,
    #[serde(rename = "shipObj", default, skip_serializing_if = "Option::is_none")]
    pub ship_obj: Option<FieldShipObj>,
    #[serde(rename = "snowBall", default, skip_serializing_if = "Option::is_none")]
    pub snow_ball: Option<FieldSnowBall>,
    #[serde(rename = "snowMan", default, skip_serializing_if = "Option::is_none")]
    pub snow_man: Option<FieldSnowMan>,
    #[serde(
        rename = "swimArea",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub swim_area: std::collections::HashMap<String, FieldSwimAreaValue>,
    #[serde(
        rename = "ToolTip",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub tool_tip: std::collections::HashMap<String, FieldToolTipValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub user: std::collections::HashMap<String, FieldUserValue>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub weather: std::collections::HashMap<String, FieldWeatherValue>,
}
impl From<&Field> for Field {
    fn from(value: &Field) -> Self {
        value.clone()
    }
}
impl Field {
    pub fn builder() -> builder::Field {
        builder::Field::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldAreaValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldAreaValue> for FieldAreaValue {
    fn from(value: &FieldAreaValue) -> Self {
        value.clone()
    }
}
impl FieldAreaValue {
    pub fn builder() -> builder::FieldAreaValue {
        builder::FieldAreaValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldBattleField {
    #[serde(
        rename = "effectLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_lose: Option<String>,
    #[serde(rename = "effectWin", default, skip_serializing_if = "Option::is_none")]
    pub effect_win: Option<String>,
    #[serde(
        rename = "rewardMapLoseSheep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_lose_sheep: Option<i64>,
    #[serde(
        rename = "rewardMapLoseWolf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_lose_wolf: Option<i64>,
    #[serde(
        rename = "rewardMapWinSheep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_win_sheep: Option<i64>,
    #[serde(
        rename = "rewardMapWinWolf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_win_wolf: Option<i64>,
    #[serde(
        rename = "timeDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_default: Option<i64>,
    #[serde(
        rename = "timeFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_finish: Option<i64>,
}
impl From<&FieldBattleField> for FieldBattleField {
    fn from(value: &FieldBattleField) -> Self {
        value.clone()
    }
}
impl FieldBattleField {
    pub fn builder() -> builder::FieldBattleField {
        builder::FieldBattleField::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldBuffZoneValue {
    #[serde(rename = "Duration", default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "Interval", default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "ItemID", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldBuffZoneValue> for FieldBuffZoneValue {
    fn from(value: &FieldBuffZoneValue) -> Self {
        value.clone()
    }
}
impl FieldBuffZoneValue {
    pub fn builder() -> builder::FieldBuffZoneValue {
        builder::FieldBuffZoneValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldClock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldClock> for FieldClock {
    fn from(value: &FieldClock) -> Self {
        value.clone()
    }
}
impl FieldClock {
    pub fn builder() -> builder::FieldClock {
        builder::FieldClock::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldCoconut {
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub avatar: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "countBombing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_bombing: Option<i64>,
    #[serde(
        rename = "countFalling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_falling: Option<i64>,
    #[serde(rename = "countHit", default, skip_serializing_if = "Option::is_none")]
    pub count_hit: Option<i64>,
    #[serde(
        rename = "countStopped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_stopped: Option<i64>,
    #[serde(
        rename = "effectLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_lose: Option<String>,
    #[serde(rename = "effectWin", default, skip_serializing_if = "Option::is_none")]
    pub effect_win: Option<String>,
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(
        rename = "eventObjectName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub event_object_name: Option<String>,
    #[serde(rename = "soundLose", default, skip_serializing_if = "Option::is_none")]
    pub sound_lose: Option<String>,
    #[serde(rename = "soundWin", default, skip_serializing_if = "Option::is_none")]
    pub sound_win: Option<String>,
    #[serde(
        rename = "timeDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_default: Option<i64>,
    #[serde(
        rename = "timeExpand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_expand: Option<i64>,
    #[serde(
        rename = "timeFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_finish: Option<i64>,
    #[serde(
        rename = "timeMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_message: Option<i64>,
}
impl From<&FieldCoconut> for FieldCoconut {
    fn from(value: &FieldCoconut) -> Self {
        value.clone()
    }
}
impl FieldCoconut {
    pub fn builder() -> builder::FieldCoconut {
        builder::FieldCoconut::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldHealer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fall: Option<i64>,
    #[serde(rename = "healMax", default, skip_serializing_if = "Option::is_none")]
    pub heal_max: Option<i64>,
    #[serde(rename = "healMin", default, skip_serializing_if = "Option::is_none")]
    pub heal_min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rise: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(rename = "yMax", default, skip_serializing_if = "Option::is_none")]
    pub y_max: Option<i64>,
    #[serde(rename = "yMin", default, skip_serializing_if = "Option::is_none")]
    pub y_min: Option<i64>,
}
impl From<&FieldHealer> for FieldHealer {
    fn from(value: &FieldHealer) -> Self {
        value.clone()
    }
}
impl FieldHealer {
    pub fn builder() -> builder::FieldHealer {
        builder::FieldHealer::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfo {
    #[serde(
        rename = "allMoveCheck",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_move_check: Option<Bool>,
    #[serde(
        rename = "allowedItem",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub allowed_item: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "autoLieDetector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_lie_detector: Option<FieldInfoAutoLieDetector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bgm: Option<String>,
    #[serde(
        rename = "blockPBossChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_p_boss_change: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<Bool>,
    #[serde(
        rename = "consumeItemCoolTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub consume_item_cool_time: Option<i64>,
    #[serde(
        rename = "createMobInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_mob_interval: Option<i64>,
    #[serde(
        rename = "damageCheckFree",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_check_free: Option<Bool>,
    #[serde(rename = "decHP", default, skip_serializing_if = "Option::is_none")]
    pub dec_hp: Option<i64>,
    #[serde(
        rename = "decInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dec_interval: Option<i64>,
    #[serde(rename = "decMP", default, skip_serializing_if = "Option::is_none")]
    pub dec_mp: Option<i64>,
    #[serde(rename = "decRate", default, skip_serializing_if = "Option::is_none")]
    pub dec_rate: Option<i64>,
    #[serde(
        rename = "dropExpire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drop_expire: Option<i64>,
    #[serde(rename = "dropRate", default, skip_serializing_if = "Option::is_none")]
    pub drop_rate: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(
        rename = "entrustedShop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entrusted_shop: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escort: Option<FieldInfoEscort>,
    #[serde(
        rename = "EscortMinTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub escort_min_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everlast: Option<Bool>,
    #[serde(
        rename = "expeditionOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expedition_only: Option<Bool>,
    #[serde(
        rename = "fieldLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_limit: Option<i64>,
    #[serde(
        rename = "fieldSubType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub field_sub_type: Option<i64>,
    #[serde(rename = "fieldType", default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<StrOrNum>,
    #[serde(
        rename = "fixedMobCapacity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_mob_capacity: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fly: Option<Bool>,
    #[serde(
        rename = "forcedReturn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub forced_return: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(
        rename = "hideMinimap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_minimap: Option<Bool>,
    #[serde(rename = "LBBottom", default, skip_serializing_if = "Option::is_none")]
    pub lb_bottom: Option<i64>,
    #[serde(rename = "LBSide", default, skip_serializing_if = "Option::is_none")]
    pub lb_side: Option<i64>,
    #[serde(rename = "LBTop", default, skip_serializing_if = "Option::is_none")]
    pub lb_top: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(
        rename = "lvForceMove",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lv_force_move: Option<i64>,
    #[serde(rename = "lvLimit", default, skip_serializing_if = "Option::is_none")]
    pub lv_limit: Option<i64>,
    #[serde(rename = "mapDesc", default, skip_serializing_if = "Option::is_none")]
    pub map_desc: Option<String>,
    #[serde(rename = "mapMark", default, skip_serializing_if = "Option::is_none")]
    pub map_mark: Option<String>,
    #[serde(rename = "mapName", default, skip_serializing_if = "Option::is_none")]
    pub map_name: Option<String>,
    #[serde(
        rename = "miniMapOnOff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mini_map_on_off: Option<Bool>,
    #[serde(rename = "mobRate", default, skip_serializing_if = "Option::is_none")]
    pub mob_rate: Option<f64>,
    #[serde(rename = "moveLimit", default, skip_serializing_if = "Option::is_none")]
    pub move_limit: Option<Bool>,
    #[serde(
        rename = "needSkillForFly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub need_skill_for_fly: Option<Bool>,
    #[serde(rename = "noMapCmd", default, skip_serializing_if = "Option::is_none")]
    pub no_map_cmd: Option<Bool>,
    #[serde(
        rename = "noRegenMap",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_regen_map: Option<Bool>,
    #[serde(
        rename = "onFirstUserEnter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_first_user_enter: Option<String>,
    #[serde(
        rename = "onUserEnter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub on_user_enter: Option<String>,
    #[serde(rename = "partyOnly", default, skip_serializing_if = "Option::is_none")]
    pub party_only: Option<Bool>,
    #[serde(
        rename = "personalShop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub personal_shop: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<i64>,
    #[serde(
        rename = "phaseAlpha",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phase_alpha: Option<i64>,
    #[serde(
        rename = "phaseBG",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub phase_bg: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "protectItem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_item: Option<i64>,
    #[serde(
        rename = "protectSetKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protect_set_key: Option<i64>,
    #[serde(
        rename = "reactorShuffle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_shuffle: Option<Bool>,
    #[serde(
        rename = "reactorShuffleName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_shuffle_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<f64>,
    #[serde(rename = "returnMap", default, skip_serializing_if = "Option::is_none")]
    pub return_map: Option<i64>,
    #[serde(
        rename = "scrollDisable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scroll_disable: Option<Bool>,
    #[serde(
        rename = "streetName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swim: Option<Bool>,
    #[serde(rename = "timeLimit", default, skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<i64>,
    #[serde(rename = "timeMob", default, skip_serializing_if = "Option::is_none")]
    pub time_mob: Option<FieldInfoTimeMob>,
    #[serde(rename = "timeOut", default, skip_serializing_if = "Option::is_none")]
    pub time_out: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub town: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "VRBottom", default, skip_serializing_if = "Option::is_none")]
    pub vr_bottom: Option<i64>,
    #[serde(rename = "VRLeft", default, skip_serializing_if = "Option::is_none")]
    pub vr_left: Option<i64>,
    #[serde(rename = "VRLimit", default, skip_serializing_if = "Option::is_none")]
    pub vr_limit: Option<Bool>,
    #[serde(rename = "VRRight", default, skip_serializing_if = "Option::is_none")]
    pub vr_right: Option<i64>,
    #[serde(rename = "VRTop", default, skip_serializing_if = "Option::is_none")]
    pub vr_top: Option<i64>,
    #[serde(
        rename = "zakum2Hack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zakum2_hack: Option<Bool>,
}
impl From<&FieldInfo> for FieldInfo {
    fn from(value: &FieldInfo) -> Self {
        value.clone()
    }
}
impl FieldInfo {
    pub fn builder() -> builder::FieldInfo {
        builder::FieldInfo::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfoAutoLieDetector {
    #[serde(rename = "endHour", default, skip_serializing_if = "Option::is_none")]
    pub end_hour: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<i64>,
    #[serde(rename = "startHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i64>,
}
impl From<&FieldInfoAutoLieDetector> for FieldInfoAutoLieDetector {
    fn from(value: &FieldInfoAutoLieDetector) -> Self {
        value.clone()
    }
}
impl FieldInfoAutoLieDetector {
    pub fn builder() -> builder::FieldInfoAutoLieDetector {
        builder::FieldInfoAutoLieDetector::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfoEscort {
    #[serde(
        rename = "checkDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_distance: Option<Bool>,
    #[serde(
        rename = "failMessageOnDie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_message_on_die: Option<String>,
    #[serde(
        rename = "failMessageOnDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_message_on_distance: Option<String>,
    #[serde(rename = "mobID", default, skip_serializing_if = "Option::is_none")]
    pub mob_id: Option<i64>,
    #[serde(
        rename = "timeOutLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_out_limit: Option<i64>,
    #[serde(
        rename = "timeOutWarningTerm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_out_warning_term: Option<i64>,
    #[serde(
        rename = "warningDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_distance: Option<i64>,
    #[serde(
        rename = "warningMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warning_message: Option<String>,
    #[serde(
        rename = "weatherItemID",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub weather_item_id: Option<i64>,
}
impl From<&FieldInfoEscort> for FieldInfoEscort {
    fn from(value: &FieldInfoEscort) -> Self {
        value.clone()
    }
}
impl FieldInfoEscort {
    pub fn builder() -> builder::FieldInfoEscort {
        builder::FieldInfoEscort::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldInfoTimeMob {
    #[serde(rename = "endHour", default, skip_serializing_if = "Option::is_none")]
    pub end_hour: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "startHour", default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<i64>,
}
impl From<&FieldInfoTimeMob> for FieldInfoTimeMob {
    fn from(value: &FieldInfoTimeMob) -> Self {
        value.clone()
    }
}
impl FieldInfoTimeMob {
    pub fn builder() -> builder::FieldInfoTimeMob {
        builder::FieldInfoTimeMob::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldLadderRopeValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub l: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uf: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldLadderRopeValue> for FieldLadderRopeValue {
    fn from(value: &FieldLadderRopeValue) -> Self {
        value.clone()
    }
}
impl FieldLadderRopeValue {
    pub fn builder() -> builder::FieldLadderRopeValue {
        builder::FieldLadderRopeValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldLifeValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cy: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fh: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limitedname: Option<String>,
    #[serde(rename = "mobTime", default, skip_serializing_if = "Option::is_none")]
    pub mob_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<i64>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldLifeValue> for FieldLifeValue {
    fn from(value: &FieldLifeValue) -> Self {
        value.clone()
    }
}
impl FieldLifeValue {
    pub fn builder() -> builder::FieldLifeValue {
        builder::FieldLifeValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMobMassacre {
    #[serde(
        rename = "countEffect",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub count_effect: std::collections::HashMap<String, FieldMobMassacreCountEffectValue>,
    #[serde(
        rename = "disableSkill",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_skill: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gauge: Option<FieldMobMassacreGauge>,
    #[serde(
        rename = "mapDistance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_distance: Option<i64>,
}
impl From<&FieldMobMassacre> for FieldMobMassacre {
    fn from(value: &FieldMobMassacre) -> Self {
        value.clone()
    }
}
impl FieldMobMassacre {
    pub fn builder() -> builder::FieldMobMassacre {
        builder::FieldMobMassacre::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMobMassacreCountEffectValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buff: Option<i64>,
    #[serde(rename = "skillUse", default, skip_serializing_if = "Option::is_none")]
    pub skill_use: Option<Bool>,
}
impl From<&FieldMobMassacreCountEffectValue> for FieldMobMassacreCountEffectValue {
    fn from(value: &FieldMobMassacreCountEffectValue) -> Self {
        value.clone()
    }
}
impl FieldMobMassacreCountEffectValue {
    pub fn builder() -> builder::FieldMobMassacreCountEffectValue {
        builder::FieldMobMassacreCountEffectValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMobMassacreGauge {
    #[serde(rename = "coolAdd", default, skip_serializing_if = "Option::is_none")]
    pub cool_add: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decrease: Option<i64>,
    #[serde(rename = "hitAdd", default, skip_serializing_if = "Option::is_none")]
    pub hit_add: Option<i64>,
    #[serde(rename = "missSub", default, skip_serializing_if = "Option::is_none")]
    pub miss_sub: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
impl From<&FieldMobMassacreGauge> for FieldMobMassacreGauge {
    fn from(value: &FieldMobMassacreGauge) -> Self {
        value.clone()
    }
}
impl FieldMobMassacreGauge {
    pub fn builder() -> builder::FieldMobMassacreGauge {
        builder::FieldMobMassacreGauge::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnival {
    #[serde(rename = "deathCP", default, skip_serializing_if = "Option::is_none")]
    pub death_cp: Option<i64>,
    #[serde(
        rename = "effectLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_lose: Option<String>,
    #[serde(rename = "effectWin", default, skip_serializing_if = "Option::is_none")]
    pub effect_win: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub guardian: std::collections::HashMap<String, f64>,
    #[serde(
        rename = "guardianGenMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub guardian_gen_max: Option<i64>,
    #[serde(
        rename = "guardianGenPos",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub guardian_gen_pos:
        std::collections::HashMap<String, FieldMonsterCarnivalGuardianGenPosValue>,
    #[serde(
        rename = "mapDivided",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_divided: Option<Bool>,
    #[serde(rename = "mapType", default, skip_serializing_if = "Option::is_none")]
    pub map_type: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub mob: std::collections::HashMap<String, FieldMonsterCarnivalMobValue>,
    #[serde(rename = "mobGenMax", default, skip_serializing_if = "Option::is_none")]
    pub mob_gen_max: Option<i64>,
    #[serde(
        rename = "mobGenPos",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub mob_gen_pos: std::collections::HashMap<String, FieldMonsterCarnivalMobGenPosValue>,
    #[serde(
        rename = "reactorBlue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_blue: Option<i64>,
    #[serde(
        rename = "reactorRed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_red: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reward: Option<FieldMonsterCarnivalReward>,
    #[serde(
        rename = "rewardMapLose",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_lose: Option<i64>,
    #[serde(
        rename = "rewardMapWin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reward_map_win: Option<i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, f64>,
    #[serde(rename = "soundLose", default, skip_serializing_if = "Option::is_none")]
    pub sound_lose: Option<String>,
    #[serde(rename = "soundWin", default, skip_serializing_if = "Option::is_none")]
    pub sound_win: Option<String>,
    #[serde(
        rename = "timeDefault",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_default: Option<i64>,
    #[serde(
        rename = "timeExpand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_expand: Option<i64>,
    #[serde(
        rename = "timeFinish",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_finish: Option<i64>,
    #[serde(
        rename = "timeMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_message: Option<i64>,
}
impl From<&FieldMonsterCarnival> for FieldMonsterCarnival {
    fn from(value: &FieldMonsterCarnival) -> Self {
        value.clone()
    }
}
impl FieldMonsterCarnival {
    pub fn builder() -> builder::FieldMonsterCarnival {
        builder::FieldMonsterCarnival::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalGuardianGenPosValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldMonsterCarnivalGuardianGenPosValue> for FieldMonsterCarnivalGuardianGenPosValue {
    fn from(value: &FieldMonsterCarnivalGuardianGenPosValue) -> Self {
        value.clone()
    }
}
impl FieldMonsterCarnivalGuardianGenPosValue {
    pub fn builder() -> builder::FieldMonsterCarnivalGuardianGenPosValue {
        builder::FieldMonsterCarnivalGuardianGenPosValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalMobGenPosValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cy: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fh: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldMonsterCarnivalMobGenPosValue> for FieldMonsterCarnivalMobGenPosValue {
    fn from(value: &FieldMonsterCarnivalMobGenPosValue) -> Self {
        value.clone()
    }
}
impl FieldMonsterCarnivalMobGenPosValue {
    pub fn builder() -> builder::FieldMonsterCarnivalMobGenPosValue {
        builder::FieldMonsterCarnivalMobGenPosValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalMobValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(rename = "mobTime", default, skip_serializing_if = "Option::is_none")]
    pub mob_time: Option<i64>,
    #[serde(rename = "spendCP", default, skip_serializing_if = "Option::is_none")]
    pub spend_cp: Option<i64>,
}
impl From<&FieldMonsterCarnivalMobValue> for FieldMonsterCarnivalMobValue {
    fn from(value: &FieldMonsterCarnivalMobValue) -> Self {
        value.clone()
    }
}
impl FieldMonsterCarnivalMobValue {
    pub fn builder() -> builder::FieldMonsterCarnivalMobValue {
        builder::FieldMonsterCarnivalMobValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalReward {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub climax: Option<f64>,
    #[serde(
        rename = "cpDiff",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub cp_diff: std::collections::HashMap<String, i64>,
    #[serde(
        rename = "probChange",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub prob_change: std::collections::HashMap<String, FieldMonsterCarnivalRewardProbChangeValue>,
}
impl From<&FieldMonsterCarnivalReward> for FieldMonsterCarnivalReward {
    fn from(value: &FieldMonsterCarnivalReward) -> Self {
        value.clone()
    }
}
impl FieldMonsterCarnivalReward {
    pub fn builder() -> builder::FieldMonsterCarnivalReward {
        builder::FieldMonsterCarnivalReward::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldMonsterCarnivalRewardProbChangeValue {
    #[serde(rename = "loseCoin", default, skip_serializing_if = "Option::is_none")]
    pub lose_coin: Option<f64>,
    #[serde(rename = "loseCP", default, skip_serializing_if = "Option::is_none")]
    pub lose_cp: Option<f64>,
    #[serde(rename = "loseNuff", default, skip_serializing_if = "Option::is_none")]
    pub lose_nuff: Option<f64>,
    #[serde(
        rename = "loseRecovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lose_recovery: Option<f64>,
    #[serde(rename = "wInCoin", default, skip_serializing_if = "Option::is_none")]
    pub w_in_coin: Option<f64>,
    #[serde(rename = "winCP", default, skip_serializing_if = "Option::is_none")]
    pub win_cp: Option<f64>,
    #[serde(rename = "winNuff", default, skip_serializing_if = "Option::is_none")]
    pub win_nuff: Option<f64>,
    #[serde(
        rename = "winRecovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub win_recovery: Option<f64>,
}
impl From<&FieldMonsterCarnivalRewardProbChangeValue>
    for FieldMonsterCarnivalRewardProbChangeValue
{
    fn from(value: &FieldMonsterCarnivalRewardProbChangeValue) -> Self {
        value.clone()
    }
}
impl FieldMonsterCarnivalRewardProbChangeValue {
    pub fn builder() -> builder::FieldMonsterCarnivalRewardProbChangeValue {
        builder::FieldMonsterCarnivalRewardProbChangeValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldNoSkill {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub class: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, i64>,
}
impl From<&FieldNoSkill> for FieldNoSkill {
    fn from(value: &FieldNoSkill) -> Self {
        value.clone()
    }
}
impl FieldNoSkill {
    pub fn builder() -> builder::FieldNoSkill {
        builder::FieldNoSkill::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldPortalValue {
    #[serde(
        rename = "2",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<f64>,
    #[serde(rename = "hRange", default, skip_serializing_if = "Option::is_none")]
    pub h_range: Option<i64>,
    #[serde(
        rename = "hideTooltip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_tooltip: Option<Bool>,
    #[serde(
        rename = "horizontalImpact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_impact: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "onlyOnce", default, skip_serializing_if = "Option::is_none")]
    pub only_once: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pt: Option<i64>,
    #[serde(
        rename = "reactorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(
        rename = "sessionValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_value: Option<String>,
    #[serde(
        rename = "sessionValueKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_value_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teleport: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tm: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tn: Option<String>,
    #[serde(rename = "vRange", default, skip_serializing_if = "Option::is_none")]
    pub v_range: Option<i64>,
    #[serde(
        rename = "verticalImpact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_impact: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldPortalValue> for FieldPortalValue {
    fn from(value: &FieldPortalValue) -> Self {
        value.clone()
    }
}
impl FieldPortalValue {
    pub fn builder() -> builder::FieldPortalValue {
        builder::FieldPortalValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldPulley {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulley: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldPulley> for FieldPulley {
    fn from(value: &FieldPulley) -> Self {
        value.clone()
    }
}
impl FieldPulley {
    pub fn builder() -> builder::FieldPulley {
        builder::FieldPulley::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldReactorValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "reactorTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reactor_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldReactorValue> for FieldReactorValue {
    fn from(value: &FieldReactorValue) -> Self {
        value.clone()
    }
}
impl FieldReactorValue {
    pub fn builder() -> builder::FieldReactorValue {
        builder::FieldReactorValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSeatValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}
impl From<&FieldSeatValue> for FieldSeatValue {
    fn from(value: &FieldSeatValue) -> Self {
        value.clone()
    }
}
impl FieldSeatValue {
    pub fn builder() -> builder::FieldSeatValue {
        builder::FieldSeatValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldShipObj {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub f: Option<i64>,
    #[serde(rename = "shipKind", default, skip_serializing_if = "Option::is_none")]
    pub ship_kind: Option<i64>,
    #[serde(rename = "shipObj", default, skip_serializing_if = "Option::is_none")]
    pub ship_obj: Option<String>,
    #[serde(rename = "tMove", default, skip_serializing_if = "Option::is_none")]
    pub t_move: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x0: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<i64>,
}
impl From<&FieldShipObj> for FieldShipObj {
    fn from(value: &FieldShipObj) -> Self {
        value.clone()
    }
}
impl FieldShipObj {
    pub fn builder() -> builder::FieldShipObj {
        builder::FieldShipObj::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSnowBall {
    #[serde(
        rename = "0",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "1",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub _1: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "damageSnowBall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_snow_ball: Option<StrOrNum>,
    #[serde(
        rename = "damageSnowMan0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_snow_man0: Option<i64>,
    #[serde(
        rename = "damageSnowMan1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub damage_snow_man1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<i64>,
    #[serde(
        rename = "recoveryAmount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section3: Option<i64>,
    #[serde(rename = "snowManHP", default, skip_serializing_if = "Option::is_none")]
    pub snow_man_hp: Option<i64>,
    #[serde(
        rename = "snowManWait",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snow_man_wait: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x0: Option<i64>,
    #[serde(rename = "xMax", default, skip_serializing_if = "Option::is_none")]
    pub x_max: Option<i64>,
    #[serde(rename = "xMin", default, skip_serializing_if = "Option::is_none")]
    pub x_min: Option<i64>,
}
impl From<&FieldSnowBall> for FieldSnowBall {
    fn from(value: &FieldSnowBall) -> Self {
        value.clone()
    }
}
impl FieldSnowBall {
    pub fn builder() -> builder::FieldSnowBall {
        builder::FieldSnowBall::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSnowMan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl From<&FieldSnowMan> for FieldSnowMan {
    fn from(value: &FieldSnowMan) -> Self {
        value.clone()
    }
}
impl FieldSnowMan {
    pub fn builder() -> builder::FieldSnowMan {
        builder::FieldSnowMan::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSwimAreaValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldSwimAreaValue> for FieldSwimAreaValue {
    fn from(value: &FieldSwimAreaValue) -> Self {
        value.clone()
    }
}
impl FieldSwimAreaValue {
    pub fn builder() -> builder::FieldSwimAreaValue {
        builder::FieldSwimAreaValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldToolTipValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x2: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y2: Option<i64>,
}
impl From<&FieldToolTipValue> for FieldToolTipValue {
    fn from(value: &FieldToolTipValue) -> Self {
        value.clone()
    }
}
impl FieldToolTipValue {
    pub fn builder() -> builder::FieldToolTipValue {
        builder::FieldToolTipValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cond: Option<FieldUserValueCond>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub look: Option<FieldUserValueLook>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub noitem: std::collections::HashMap<String, i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stat: Option<FieldUserValueStat>,
}
impl From<&FieldUserValue> for FieldUserValue {
    fn from(value: &FieldUserValue) -> Self {
        value.clone()
    }
}
impl FieldUserValue {
    pub fn builder() -> builder::FieldUserValue {
        builder::FieldUserValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValueCond {
    #[serde(
        rename = "battleFieldTeam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub battle_field_team: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<StrOrNum>,
    #[serde(rename = "itemCount", default, skip_serializing_if = "Option::is_none")]
    pub item_count: Option<StrOrNum>,
    #[serde(rename = "itemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<StrOrNum>,
    #[serde(
        rename = "jobCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub job_category: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<StrOrNum>,
}
impl From<&FieldUserValueCond> for FieldUserValueCond {
    fn from(value: &FieldUserValueCond) -> Self {
        value.clone()
    }
}
impl FieldUserValueCond {
    pub fn builder() -> builder::FieldUserValueCond {
        builder::FieldUserValueCond::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValueLook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cape: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clothes: Option<StrOrNum>,
    #[serde(rename = "earAcc", default, skip_serializing_if = "Option::is_none")]
    pub ear_acc: Option<StrOrNum>,
    #[serde(rename = "faceAcc", default, skip_serializing_if = "Option::is_none")]
    pub face_acc: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gloves: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pants: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shield: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shoes: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<StrOrNum>,
}
impl From<&FieldUserValueLook> for FieldUserValueLook {
    fn from(value: &FieldUserValueLook) -> Self {
        value.clone()
    }
}
impl FieldUserValueLook {
    pub fn builder() -> builder::FieldUserValueLook {
        builder::FieldUserValueLook::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldUserValueStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dex: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub luk: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speedmax: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub str: Option<StrOrNum>,
}
impl From<&FieldUserValueStat> for FieldUserValueStat {
    fn from(value: &FieldUserValueStat) -> Self {
        value.clone()
    }
}
impl FieldUserValueStat {
    pub fn builder() -> builder::FieldUserValueStat {
        builder::FieldUserValueStat::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FieldWeatherValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}
impl From<&FieldWeatherValue> for FieldWeatherValue {
    fn from(value: &FieldWeatherValue) -> Self {
        value.clone()
    }
}
impl FieldWeatherValue {
    pub fn builder() -> builder::FieldWeatherValue {
        builder::FieldWeatherValue::default()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IntStr(String);
impl std::ops::Deref for IntStr {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<IntStr> for String {
    fn from(value: IntStr) -> Self {
        value.0
    }
}
impl From<&IntStr> for IntStr {
    fn from(value: &IntStr) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IntStr {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^(-)?\\d+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(-)?\\d+$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for IntStr {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntStr {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntStr {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for IntStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NumStr(String);
impl std::ops::Deref for NumStr {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<NumStr> for String {
    fn from(value: NumStr) -> Self {
        value.0
    }
}
impl From<&NumStr> for NumStr {
    fn from(value: &NumStr) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for NumStr {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^\\d+(\\.\\d+)?$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^\\d+(\\.\\d+)?$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for NumStr {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NumStr {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NumStr {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for NumStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "Shroom basic schema"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Shroom(pub serde_json::Value);
impl std::ops::Deref for Shroom {
    type Target = serde_json::Value;
    fn deref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl From<Shroom> for serde_json::Value {
    fn from(value: Shroom) -> Self {
        value.0
    }
}
impl From<&Shroom> for Shroom {
    fn from(value: &Shroom) -> Self {
        value.clone()
    }
}
impl From<serde_json::Value> for Shroom {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Skill {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<SkillInfo>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub skill: std::collections::HashMap<String, SkillSkillValue>,
}
impl From<&Skill> for Skill {
    fn from(value: &Skill) -> Self {
        value.clone()
    }
}
impl Skill {
    pub fn builder() -> builder::Skill {
        builder::Skill::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillCommonInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Abnormal status resistance"]
    #[serde(rename = "asrR", default, skip_serializing_if = "Option::is_none")]
    pub asr_r: Option<SkillExpr>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<SkillExpr>,
    #[serde(
        rename = "bulletConsume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_consume: Option<StrOrNum>,
    #[serde(
        rename = "bulletCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_count: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooltime: Option<SkillExpr>,
    #[doc = "Craft or hands"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cr: Option<SkillExpr>,
    #[serde(
        rename = "criticaldamageMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub criticaldamage_max: Option<SkillExpr>,
    #[serde(
        rename = "criticaldamageMin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub criticaldamage_min: Option<SkillExpr>,
    #[serde(rename = "damR", default, skip_serializing_if = "Option::is_none")]
    pub dam_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot: Option<SkillExpr>,
    #[serde(
        rename = "dotInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dot_interval: Option<SkillExpr>,
    #[serde(rename = "dotTime", default, skip_serializing_if = "Option::is_none")]
    pub dot_time: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emdd: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emhp: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emmp: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epad: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epdd: Option<SkillExpr>,
    #[doc = "Total Evasion/Dodge chance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub er: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<SkillExpr>,
    #[serde(rename = "expR", default, skip_serializing_if = "Option::is_none")]
    pub exp_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hp: Option<SkillExpr>,
    #[serde(rename = "hpCon", default, skip_serializing_if = "Option::is_none")]
    pub hp_con: Option<SkillExpr>,
    #[serde(
        rename = "ignoreMobpdpR",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_mobpdp_r: Option<SkillExpr>,
    #[serde(rename = "itemCon", default, skip_serializing_if = "Option::is_none")]
    pub item_con: Option<StrOrNum>,
    #[serde(rename = "itemConNo", default, skip_serializing_if = "Option::is_none")]
    pub item_con_no: Option<StrOrNum>,
    #[serde(
        rename = "itemConsume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub item_consume: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<SkillExpr>,
    #[serde(rename = "madX", default, skip_serializing_if = "Option::is_none")]
    pub mad_x: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mastery: Option<SkillExpr>,
    #[doc = "Either txt number or number"]
    #[serde(rename = "maxLevel", default, skip_serializing_if = "Option::is_none")]
    pub max_level: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd: Option<SkillExpr>,
    #[serde(rename = "mddR", default, skip_serializing_if = "Option::is_none")]
    pub mdd_r: Option<SkillExpr>,
    #[serde(rename = "mesoR", default, skip_serializing_if = "Option::is_none")]
    pub meso_r: Option<SkillExpr>,
    #[serde(rename = "mhpR", default, skip_serializing_if = "Option::is_none")]
    pub mhp_r: Option<SkillExpr>,
    #[serde(rename = "mmpR", default, skip_serializing_if = "Option::is_none")]
    pub mmp_r: Option<SkillExpr>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<SkillExpr>,
    #[serde(rename = "moneyCon", default, skip_serializing_if = "Option::is_none")]
    pub money_con: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub morph: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mp: Option<SkillExpr>,
    #[serde(rename = "mpCon", default, skip_serializing_if = "Option::is_none")]
    pub mp_con: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<SkillExpr>,
    #[serde(rename = "padX", default, skip_serializing_if = "Option::is_none")]
    pub pad_x: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd: Option<SkillExpr>,
    #[serde(rename = "pddR", default, skip_serializing_if = "Option::is_none")]
    pub pdd_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(
        rename = "selfDestruction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_destruction: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<SkillExpr>,
    #[serde(rename = "subProp", default, skip_serializing_if = "Option::is_none")]
    pub sub_prop: Option<SkillExpr>,
    #[serde(rename = "subTime", default, skip_serializing_if = "Option::is_none")]
    pub sub_time: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<SkillExpr>,
    #[doc = "Attribute Attack Status Resistance"]
    #[serde(rename = "terR", default, skip_serializing_if = "Option::is_none")]
    pub ter_r: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub u: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<SkillExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<SkillExpr>,
}
impl From<&SkillCommonInfo> for SkillCommonInfo {
    fn from(value: &SkillCommonInfo) -> Self {
        value.clone()
    }
}
impl SkillCommonInfo {
    pub fn builder() -> builder::SkillCommonInfo {
        builder::SkillCommonInfo::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SkillExpr {
    Expr(String),
    Int(i64),
}
impl From<&SkillExpr> for SkillExpr {
    fn from(value: &SkillExpr) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SkillExpr {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Expr(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Int(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for SkillExpr {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SkillExpr {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SkillExpr {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for SkillExpr {
    fn to_string(&self) -> String {
        match self {
            Self::Expr(x) => x.to_string(),
            Self::Int(x) => x.to_string(),
        }
    }
}
impl From<i64> for SkillExpr {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Canvas>,
}
impl From<&SkillInfo> for SkillInfo {
    fn from(value: &SkillInfo) -> Self {
        value.clone()
    }
}
impl SkillInfo {
    pub fn builder() -> builder::SkillInfo {
        builder::SkillInfo::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<SkillSkillValueAction>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub affected: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub afterimage: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back_effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back_effect0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub back_finish: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "cDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub c_door: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "CharLevel",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub char_level: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "combatOrders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub combat_orders: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<SkillCommonInfo>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub damage: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<Bool>,
    #[serde(
        rename = "eDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub e_door: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect3: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect_ship: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "elemAttr", default, skip_serializing_if = "Option::is_none")]
    pub elem_attr: Option<String>,
    #[doc = "List of final attack"]
    #[serde(
        rename = "finalAttack",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub final_attack: std::collections::HashMap<String, std::collections::HashMap<String, i64>>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub finish: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub finish0: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "flipBall", default, skip_serializing_if = "Option::is_none")]
    pub flip_ball: Option<serde_json::Value>,
    #[serde(
        rename = "Frame",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub frame: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon1: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon2: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon3: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon4: Option<Canvas>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon5: Option<Canvas>,
    #[serde(
        rename = "iconDisabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_disabled: Option<Canvas>,
    #[serde(
        rename = "iconMouseOver",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_mouse_over: Option<Canvas>,
    #[doc = "name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invisible: Option<Bool>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub keydown: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub keydown0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub keydownend: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub level: std::collections::HashMap<String, SkillSkillValueLevelValue>,
    #[serde(
        rename = "mDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub m_door: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "masterLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub master_level: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub mob: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub mob0: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "mobCode", default, skip_serializing_if = "Option::is_none")]
    pub mob_code: Option<StrOrNum>,
    #[serde(
        rename = "oDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub o_door: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub prepare: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub psd: Option<Bool>,
    #[doc = "List of psd skill id"]
    #[serde(
        rename = "psdSkill",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub psd_skill: std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub repeat: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub req: std::collections::HashMap<String, StrOrNum>,
    #[serde(
        rename = "sDoor",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub s_door: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub screen: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "skillType", default, skip_serializing_if = "Option::is_none")]
    pub skill_type: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub special: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub special0: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "specialAction",
        default,
        skip_serializing_if = "std::collections::HashMap::is_empty"
    )]
    pub special_action: std::collections::HashMap<String, String>,
    #[serde(
        rename = "specialActionFrame",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_action_frame: Option<SkillSkillValueSpecialActionFrame>,
    #[serde(
        rename = "specialAffected",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub special_affected: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub state: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "stopEffect",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub stop_effect: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "subWeapon", default, skip_serializing_if = "Option::is_none")]
    pub sub_weapon: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summon: Option<SkillSkillValueSummon>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub tile: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "timeLimited",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_limited: Option<Bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weapon: Option<StrOrNum>,
}
impl From<&SkillSkillValue> for SkillSkillValue {
    fn from(value: &SkillSkillValue) -> Self {
        value.clone()
    }
}
impl SkillSkillValue {
    pub fn builder() -> builder::SkillSkillValue {
        builder::SkillSkillValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SkillSkillValueAction {
    Variant0(String),
    Variant1(std::collections::HashMap<String, String>),
}
impl From<&SkillSkillValueAction> for SkillSkillValueAction {
    fn from(value: &SkillSkillValueAction) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, String>> for SkillSkillValueAction {
    fn from(value: std::collections::HashMap<String, String>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueLevelValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acc: Option<i64>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooltime: Option<i64>,
    #[serde(
        rename = "criticaldamageMax",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub criticaldamage_max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damagepc: Option<i64>,
    #[serde(
        rename = "dateExpire",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_expire: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot: Option<StrOrNum>,
    #[serde(
        rename = "dotInterval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dot_interval: Option<StrOrNum>,
    #[serde(rename = "dotTime", default, skip_serializing_if = "Option::is_none")]
    pub dot_time: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eva: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixdamage: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "hpCon", default, skip_serializing_if = "Option::is_none")]
    pub hp_con: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hs: Option<String>,
    #[serde(rename = "itemCon", default, skip_serializing_if = "Option::is_none")]
    pub item_con: Option<i64>,
    #[serde(rename = "itemConNo", default, skip_serializing_if = "Option::is_none")]
    pub item_con_no: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mad: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mastery: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdd: Option<i64>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<i64>,
    #[serde(rename = "mpCon", default, skip_serializing_if = "Option::is_none")]
    pub mp_con: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pad: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdd: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prop: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z: Option<i64>,
}
impl From<&SkillSkillValueLevelValue> for SkillSkillValueLevelValue {
    fn from(value: &SkillSkillValueLevelValue) -> Self {
        value.clone()
    }
}
impl SkillSkillValueLevelValue {
    pub fn builder() -> builder::SkillSkillValueLevelValue {
        builder::SkillSkillValueLevelValue::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSpecialActionFrame {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
}
impl From<&SkillSkillValueSpecialActionFrame> for SkillSkillValueSpecialActionFrame {
    fn from(value: &SkillSkillValueSpecialActionFrame) -> Self {
        value.clone()
    }
}
impl SkillSkillValueSpecialActionFrame {
    pub fn builder() -> builder::SkillSkillValueSpecialActionFrame {
        builder::SkillSkillValueSpecialActionFrame::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSummon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attack1: Option<SkillSkillValueSummonAttack1>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub attack2: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "attackTriangle",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub attack_triangle: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub die: Option<SkillSkillValueSummonDie>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub die1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub fly: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub heal: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "move",
        default,
        skip_serializing_if = "serde_json::Map::is_empty"
    )]
    pub move_: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub prepare: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub repeat: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub repeat0: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub say: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill1: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill2: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill3: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill4: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill5: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub skill6: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stand: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub subsummon: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub summoned: serde_json::Map<String, serde_json::Value>,
}
impl From<&SkillSkillValueSummon> for SkillSkillValueSummon {
    fn from(value: &SkillSkillValueSummon) -> Self {
        value.clone()
    }
}
impl SkillSkillValueSummon {
    pub fn builder() -> builder::SkillSkillValueSummon {
        builder::SkillSkillValueSummon::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillSkillValueSummonAttack1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<SkillSkillValueSummonAttack1Info>,
}
impl From<&SkillSkillValueSummonAttack1> for SkillSkillValueSummonAttack1 {
    fn from(value: &SkillSkillValueSummonAttack1) -> Self {
        value.clone()
    }
}
impl SkillSkillValueSummonAttack1 {
    pub fn builder() -> builder::SkillSkillValueSummonAttack1 {
        builder::SkillSkillValueSummonAttack1::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSummonAttack1Info {
    #[serde(
        rename = "attackAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_after: Option<i64>,
    #[serde(
        rename = "attackCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_count: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub ball: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "bulletSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bullet_speed: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub effect: serde_json::Map<String, serde_json::Value>,
    #[serde(
        rename = "effectAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effect_after: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub hit: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<StrOrNum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<SummonRange>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i64>,
}
impl From<&SkillSkillValueSummonAttack1Info> for SkillSkillValueSummonAttack1Info {
    fn from(value: &SkillSkillValueSummonAttack1Info) -> Self {
        value.clone()
    }
}
impl SkillSkillValueSummonAttack1Info {
    pub fn builder() -> builder::SkillSkillValueSummonAttack1Info {
        builder::SkillSkillValueSummonAttack1Info::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SkillSkillValueSummonDie {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<SkillSkillValueSummonDieInfo>,
}
impl From<&SkillSkillValueSummonDie> for SkillSkillValueSummonDie {
    fn from(value: &SkillSkillValueSummonDie) -> Self {
        value.clone()
    }
}
impl SkillSkillValueSummonDie {
    pub fn builder() -> builder::SkillSkillValueSummonDie {
        builder::SkillSkillValueSummonDie::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SkillSkillValueSummonDieInfo {
    #[serde(
        rename = "attackAfter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attack_after: Option<i64>,
    #[serde(rename = "mobCount", default, skip_serializing_if = "Option::is_none")]
    pub mob_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<SummonRange>,
}
impl From<&SkillSkillValueSummonDieInfo> for SkillSkillValueSummonDieInfo {
    fn from(value: &SkillSkillValueSummonDieInfo) -> Self {
        value.clone()
    }
}
impl SkillSkillValueSummonDieInfo {
    pub fn builder() -> builder::SkillSkillValueSummonDieInfo {
        builder::SkillSkillValueSummonDieInfo::default()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Str(String);
impl std::ops::Deref for Str {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Str> for String {
    fn from(value: Str) -> Self {
        value.0
    }
}
impl From<&Str> for Str {
    fn from(value: &Str) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Str {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^1|0$").unwrap().find(value).is_none() {
            return Err("doesn't match pattern \"^1|0$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Str {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Str {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Str {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StrOrInt {
    Variant0(IntStr),
    Variant1(i64),
}
impl From<&StrOrInt> for StrOrInt {
    fn from(value: &StrOrInt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for StrOrInt {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for StrOrInt {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StrOrInt {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StrOrInt {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for StrOrInt {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<IntStr> for StrOrInt {
    fn from(value: IntStr) -> Self {
        Self::Variant0(value)
    }
}
impl From<i64> for StrOrInt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StrOrNum {
    NumStr(NumStr),
    Int(i64),
}
impl From<&StrOrNum> for StrOrNum {
    fn from(value: &StrOrNum) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for StrOrNum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::NumStr(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Int(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for StrOrNum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StrOrNum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StrOrNum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for StrOrNum {
    fn to_string(&self) -> String {
        match self {
            Self::NumStr(x) => x.to_string(),
            Self::Int(x) => x.to_string(),
        }
    }
}
impl From<NumStr> for StrOrNum {
    fn from(value: NumStr) -> Self {
        Self::NumStr(value)
    }
}
impl From<i64> for StrOrNum {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SummonRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sp: Option<Vec2>,
}
impl From<&SummonRange> for SummonRange {
    fn from(value: &SummonRange) -> Self {
        value.clone()
    }
}
impl SummonRange {
    pub fn builder() -> builder::SummonRange {
        builder::SummonRange::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vec2 {
    #[doc = "x"]
    pub x: i64,
    #[doc = "y"]
    pub y: i64,
}
impl From<&Vec2> for Vec2 {
    fn from(value: &Vec2) -> Self {
        value.clone()
    }
}
impl Vec2 {
    pub fn builder() -> builder::Vec2 {
        builder::Vec2::default()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Canvas {
        height: Result<Option<i64>, String>,
        scale: Result<Option<i64>, String>,
        sub: Result<Option<serde_json::Map<String, serde_json::Value>>, String>,
        width: Result<Option<i64>, String>,
    }
    impl Default for Canvas {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                scale: Ok(Default::default()),
                sub: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl Canvas {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {}", e));
            self
        }
        pub fn scale<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.scale = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scale: {}", e));
            self
        }
        pub fn sub<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Map<String, serde_json::Value>>>,
            T::Error: std::fmt::Display,
        {
            self.sub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub: {}", e));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Canvas> for super::Canvas {
        type Error = String;
        fn try_from(value: Canvas) -> Result<Self, String> {
            Ok(Self {
                height: value.height?,
                scale: value.scale?,
                sub: value.sub?,
                width: value.width?,
            })
        }
    }
    impl From<super::Canvas> for Canvas {
        fn from(value: super::Canvas) -> Self {
            Self {
                height: Ok(value.height),
                scale: Ok(value.scale),
                sub: Ok(value.sub),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Fh {
        cant_through: Result<Option<super::Bool>, String>,
        forbid_fall_down: Result<Option<super::Bool>, String>,
        force: Result<Option<i64>, String>,
        next: Result<i64, String>,
        piece: Result<Option<i64>, String>,
        prev: Result<i64, String>,
        x1: Result<i64, String>,
        x2: Result<i64, String>,
        y1: Result<i64, String>,
        y2: Result<i64, String>,
    }
    impl Default for Fh {
        fn default() -> Self {
            Self {
                cant_through: Ok(Default::default()),
                forbid_fall_down: Ok(Default::default()),
                force: Ok(Default::default()),
                next: Err("no value supplied for next".to_string()),
                piece: Ok(Default::default()),
                prev: Err("no value supplied for prev".to_string()),
                x1: Err("no value supplied for x1".to_string()),
                x2: Err("no value supplied for x2".to_string()),
                y1: Err("no value supplied for y1".to_string()),
                y2: Err("no value supplied for y2".to_string()),
            }
        }
    }
    impl Fh {
        pub fn cant_through<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.cant_through = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cant_through: {}", e));
            self
        }
        pub fn forbid_fall_down<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.forbid_fall_down = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for forbid_fall_down: {}",
                    e
                )
            });
            self
        }
        pub fn force<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.force = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for force: {}", e));
            self
        }
        pub fn next<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.next = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next: {}", e));
            self
        }
        pub fn piece<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.piece = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for piece: {}", e));
            self
        }
        pub fn prev<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.prev = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prev: {}", e));
            self
        }
        pub fn x1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.x1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x1: {}", e));
            self
        }
        pub fn x2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.x2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x2: {}", e));
            self
        }
        pub fn y1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.y1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y1: {}", e));
            self
        }
        pub fn y2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.y2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y2: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Fh> for super::Fh {
        type Error = String;
        fn try_from(value: Fh) -> Result<Self, String> {
            Ok(Self {
                cant_through: value.cant_through?,
                forbid_fall_down: value.forbid_fall_down?,
                force: value.force?,
                next: value.next?,
                piece: value.piece?,
                prev: value.prev?,
                x1: value.x1?,
                x2: value.x2?,
                y1: value.y1?,
                y2: value.y2?,
            })
        }
    }
    impl From<super::Fh> for Fh {
        fn from(value: super::Fh) -> Self {
            Self {
                cant_through: Ok(value.cant_through),
                forbid_fall_down: Ok(value.forbid_fall_down),
                force: Ok(value.force),
                next: Ok(value.next),
                piece: Ok(value.piece),
                prev: Ok(value.prev),
                x1: Ok(value.x1),
                x2: Ok(value.x2),
                y1: Ok(value.y1),
                y2: Ok(value.y2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Field {
        _0: Result<serde_json::Map<String, serde_json::Value>, String>,
        _1: Result<serde_json::Map<String, serde_json::Value>, String>,
        _2: Result<serde_json::Map<String, serde_json::Value>, String>,
        _3: Result<serde_json::Map<String, serde_json::Value>, String>,
        _4: Result<serde_json::Map<String, serde_json::Value>, String>,
        _5: Result<serde_json::Map<String, serde_json::Value>, String>,
        _6: Result<serde_json::Map<String, serde_json::Value>, String>,
        _7: Result<serde_json::Map<String, serde_json::Value>, String>,
        area: Result<std::collections::HashMap<String, super::FieldAreaValue>, String>,
        back: Result<serde_json::Map<String, serde_json::Value>, String>,
        battle_field: Result<Option<super::FieldBattleField>, String>,
        buff_zone: Result<std::collections::HashMap<String, super::FieldBuffZoneValue>, String>,
        clock: Result<Option<super::FieldClock>, String>,
        coconut: Result<Option<super::FieldCoconut>, String>,
        foothold: Result<
            std::collections::HashMap<
                String,
                std::collections::HashMap<String, std::collections::HashMap<String, super::Fh>>,
            >,
            String,
        >,
        healer: Result<Option<super::FieldHealer>, String>,
        info: Result<Option<super::FieldInfo>, String>,
        ladder_rope: Result<std::collections::HashMap<String, super::FieldLadderRopeValue>, String>,
        life: Result<std::collections::HashMap<String, super::FieldLifeValue>, String>,
        mini_map: Result<serde_json::Map<String, serde_json::Value>, String>,
        mob_massacre: Result<Option<super::FieldMobMassacre>, String>,
        monster_carnival: Result<Option<super::FieldMonsterCarnival>, String>,
        no_skill: Result<Option<super::FieldNoSkill>, String>,
        node_info: Result<serde_json::Map<String, serde_json::Value>, String>,
        portal: Result<std::collections::HashMap<String, super::FieldPortalValue>, String>,
        pulley: Result<Option<super::FieldPulley>, String>,
        reactor: Result<std::collections::HashMap<String, super::FieldReactorValue>, String>,
        seat: Result<std::collections::HashMap<String, super::FieldSeatValue>, String>,
        ship_obj: Result<Option<super::FieldShipObj>, String>,
        snow_ball: Result<Option<super::FieldSnowBall>, String>,
        snow_man: Result<Option<super::FieldSnowMan>, String>,
        swim_area: Result<std::collections::HashMap<String, super::FieldSwimAreaValue>, String>,
        tool_tip: Result<std::collections::HashMap<String, super::FieldToolTipValue>, String>,
        user: Result<std::collections::HashMap<String, super::FieldUserValue>, String>,
        weather: Result<std::collections::HashMap<String, super::FieldWeatherValue>, String>,
    }
    impl Default for Field {
        fn default() -> Self {
            Self {
                _0: Ok(Default::default()),
                _1: Ok(Default::default()),
                _2: Ok(Default::default()),
                _3: Ok(Default::default()),
                _4: Ok(Default::default()),
                _5: Ok(Default::default()),
                _6: Ok(Default::default()),
                _7: Ok(Default::default()),
                area: Ok(Default::default()),
                back: Ok(Default::default()),
                battle_field: Ok(Default::default()),
                buff_zone: Ok(Default::default()),
                clock: Ok(Default::default()),
                coconut: Ok(Default::default()),
                foothold: Ok(Default::default()),
                healer: Ok(Default::default()),
                info: Ok(Default::default()),
                ladder_rope: Ok(Default::default()),
                life: Ok(Default::default()),
                mini_map: Ok(Default::default()),
                mob_massacre: Ok(Default::default()),
                monster_carnival: Ok(Default::default()),
                no_skill: Ok(Default::default()),
                node_info: Ok(Default::default()),
                portal: Ok(Default::default()),
                pulley: Ok(Default::default()),
                reactor: Ok(Default::default()),
                seat: Ok(Default::default()),
                ship_obj: Ok(Default::default()),
                snow_ball: Ok(Default::default()),
                snow_man: Ok(Default::default()),
                swim_area: Ok(Default::default()),
                tool_tip: Ok(Default::default()),
                user: Ok(Default::default()),
                weather: Ok(Default::default()),
            }
        }
    }
    impl Field {
        pub fn _0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _0: {}", e));
            self
        }
        pub fn _1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _1: {}", e));
            self
        }
        pub fn _2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _2: {}", e));
            self
        }
        pub fn _3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _3: {}", e));
            self
        }
        pub fn _4<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _4: {}", e));
            self
        }
        pub fn _5<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _5: {}", e));
            self
        }
        pub fn _6<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _6: {}", e));
            self
        }
        pub fn _7<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _7: {}", e));
            self
        }
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldAreaValue>>,
            T::Error: std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn back<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.back = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for back: {}", e));
            self
        }
        pub fn battle_field<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldBattleField>>,
            T::Error: std::fmt::Display,
        {
            self.battle_field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for battle_field: {}", e));
            self
        }
        pub fn buff_zone<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldBuffZoneValue>>,
            T::Error: std::fmt::Display,
        {
            self.buff_zone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for buff_zone: {}", e));
            self
        }
        pub fn clock<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldClock>>,
            T::Error: std::fmt::Display,
        {
            self.clock = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clock: {}", e));
            self
        }
        pub fn coconut<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldCoconut>>,
            T::Error: std::fmt::Display,
        {
            self.coconut = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coconut: {}", e));
            self
        }
        pub fn foothold<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<
                    String,
                    std::collections::HashMap<String, std::collections::HashMap<String, super::Fh>>,
                >,
            >,
            T::Error: std::fmt::Display,
        {
            self.foothold = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for foothold: {}", e));
            self
        }
        pub fn healer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldHealer>>,
            T::Error: std::fmt::Display,
        {
            self.healer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for healer: {}", e));
            self
        }
        pub fn info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldInfo>>,
            T::Error: std::fmt::Display,
        {
            self.info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info: {}", e));
            self
        }
        pub fn ladder_rope<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, super::FieldLadderRopeValue>,
            >,
            T::Error: std::fmt::Display,
        {
            self.ladder_rope = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ladder_rope: {}", e));
            self
        }
        pub fn life<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldLifeValue>>,
            T::Error: std::fmt::Display,
        {
            self.life = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for life: {}", e));
            self
        }
        pub fn mini_map<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.mini_map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mini_map: {}", e));
            self
        }
        pub fn mob_massacre<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldMobMassacre>>,
            T::Error: std::fmt::Display,
        {
            self.mob_massacre = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_massacre: {}", e));
            self
        }
        pub fn monster_carnival<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldMonsterCarnival>>,
            T::Error: std::fmt::Display,
        {
            self.monster_carnival = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for monster_carnival: {}",
                    e
                )
            });
            self
        }
        pub fn no_skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldNoSkill>>,
            T::Error: std::fmt::Display,
        {
            self.no_skill = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for no_skill: {}", e));
            self
        }
        pub fn node_info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.node_info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for node_info: {}", e));
            self
        }
        pub fn portal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldPortalValue>>,
            T::Error: std::fmt::Display,
        {
            self.portal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for portal: {}", e));
            self
        }
        pub fn pulley<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldPulley>>,
            T::Error: std::fmt::Display,
        {
            self.pulley = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pulley: {}", e));
            self
        }
        pub fn reactor<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldReactorValue>>,
            T::Error: std::fmt::Display,
        {
            self.reactor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reactor: {}", e));
            self
        }
        pub fn seat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldSeatValue>>,
            T::Error: std::fmt::Display,
        {
            self.seat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seat: {}", e));
            self
        }
        pub fn ship_obj<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldShipObj>>,
            T::Error: std::fmt::Display,
        {
            self.ship_obj = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ship_obj: {}", e));
            self
        }
        pub fn snow_ball<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldSnowBall>>,
            T::Error: std::fmt::Display,
        {
            self.snow_ball = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for snow_ball: {}", e));
            self
        }
        pub fn snow_man<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldSnowMan>>,
            T::Error: std::fmt::Display,
        {
            self.snow_man = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for snow_man: {}", e));
            self
        }
        pub fn swim_area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldSwimAreaValue>>,
            T::Error: std::fmt::Display,
        {
            self.swim_area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for swim_area: {}", e));
            self
        }
        pub fn tool_tip<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldToolTipValue>>,
            T::Error: std::fmt::Display,
        {
            self.tool_tip = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_tip: {}", e));
            self
        }
        pub fn user<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldUserValue>>,
            T::Error: std::fmt::Display,
        {
            self.user = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user: {}", e));
            self
        }
        pub fn weather<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::FieldWeatherValue>>,
            T::Error: std::fmt::Display,
        {
            self.weather = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weather: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Field> for super::Field {
        type Error = String;
        fn try_from(value: Field) -> Result<Self, String> {
            Ok(Self {
                _0: value._0?,
                _1: value._1?,
                _2: value._2?,
                _3: value._3?,
                _4: value._4?,
                _5: value._5?,
                _6: value._6?,
                _7: value._7?,
                area: value.area?,
                back: value.back?,
                battle_field: value.battle_field?,
                buff_zone: value.buff_zone?,
                clock: value.clock?,
                coconut: value.coconut?,
                foothold: value.foothold?,
                healer: value.healer?,
                info: value.info?,
                ladder_rope: value.ladder_rope?,
                life: value.life?,
                mini_map: value.mini_map?,
                mob_massacre: value.mob_massacre?,
                monster_carnival: value.monster_carnival?,
                no_skill: value.no_skill?,
                node_info: value.node_info?,
                portal: value.portal?,
                pulley: value.pulley?,
                reactor: value.reactor?,
                seat: value.seat?,
                ship_obj: value.ship_obj?,
                snow_ball: value.snow_ball?,
                snow_man: value.snow_man?,
                swim_area: value.swim_area?,
                tool_tip: value.tool_tip?,
                user: value.user?,
                weather: value.weather?,
            })
        }
    }
    impl From<super::Field> for Field {
        fn from(value: super::Field) -> Self {
            Self {
                _0: Ok(value._0),
                _1: Ok(value._1),
                _2: Ok(value._2),
                _3: Ok(value._3),
                _4: Ok(value._4),
                _5: Ok(value._5),
                _6: Ok(value._6),
                _7: Ok(value._7),
                area: Ok(value.area),
                back: Ok(value.back),
                battle_field: Ok(value.battle_field),
                buff_zone: Ok(value.buff_zone),
                clock: Ok(value.clock),
                coconut: Ok(value.coconut),
                foothold: Ok(value.foothold),
                healer: Ok(value.healer),
                info: Ok(value.info),
                ladder_rope: Ok(value.ladder_rope),
                life: Ok(value.life),
                mini_map: Ok(value.mini_map),
                mob_massacre: Ok(value.mob_massacre),
                monster_carnival: Ok(value.monster_carnival),
                no_skill: Ok(value.no_skill),
                node_info: Ok(value.node_info),
                portal: Ok(value.portal),
                pulley: Ok(value.pulley),
                reactor: Ok(value.reactor),
                seat: Ok(value.seat),
                ship_obj: Ok(value.ship_obj),
                snow_ball: Ok(value.snow_ball),
                snow_man: Ok(value.snow_man),
                swim_area: Ok(value.swim_area),
                tool_tip: Ok(value.tool_tip),
                user: Ok(value.user),
                weather: Ok(value.weather),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldAreaValue {
        x1: Result<Option<i64>, String>,
        x2: Result<Option<i64>, String>,
        y1: Result<Option<i64>, String>,
        y2: Result<Option<i64>, String>,
    }
    impl Default for FieldAreaValue {
        fn default() -> Self {
            Self {
                x1: Ok(Default::default()),
                x2: Ok(Default::default()),
                y1: Ok(Default::default()),
                y2: Ok(Default::default()),
            }
        }
    }
    impl FieldAreaValue {
        pub fn x1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x1: {}", e));
            self
        }
        pub fn x2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x2: {}", e));
            self
        }
        pub fn y1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y1: {}", e));
            self
        }
        pub fn y2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y2: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldAreaValue> for super::FieldAreaValue {
        type Error = String;
        fn try_from(value: FieldAreaValue) -> Result<Self, String> {
            Ok(Self {
                x1: value.x1?,
                x2: value.x2?,
                y1: value.y1?,
                y2: value.y2?,
            })
        }
    }
    impl From<super::FieldAreaValue> for FieldAreaValue {
        fn from(value: super::FieldAreaValue) -> Self {
            Self {
                x1: Ok(value.x1),
                x2: Ok(value.x2),
                y1: Ok(value.y1),
                y2: Ok(value.y2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldBattleField {
        effect_lose: Result<Option<String>, String>,
        effect_win: Result<Option<String>, String>,
        reward_map_lose_sheep: Result<Option<i64>, String>,
        reward_map_lose_wolf: Result<Option<i64>, String>,
        reward_map_win_sheep: Result<Option<i64>, String>,
        reward_map_win_wolf: Result<Option<i64>, String>,
        time_default: Result<Option<i64>, String>,
        time_finish: Result<Option<i64>, String>,
    }
    impl Default for FieldBattleField {
        fn default() -> Self {
            Self {
                effect_lose: Ok(Default::default()),
                effect_win: Ok(Default::default()),
                reward_map_lose_sheep: Ok(Default::default()),
                reward_map_lose_wolf: Ok(Default::default()),
                reward_map_win_sheep: Ok(Default::default()),
                reward_map_win_wolf: Ok(Default::default()),
                time_default: Ok(Default::default()),
                time_finish: Ok(Default::default()),
            }
        }
    }
    impl FieldBattleField {
        pub fn effect_lose<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.effect_lose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_lose: {}", e));
            self
        }
        pub fn effect_win<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.effect_win = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_win: {}", e));
            self
        }
        pub fn reward_map_lose_sheep<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reward_map_lose_sheep = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reward_map_lose_sheep: {}",
                    e
                )
            });
            self
        }
        pub fn reward_map_lose_wolf<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reward_map_lose_wolf = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reward_map_lose_wolf: {}",
                    e
                )
            });
            self
        }
        pub fn reward_map_win_sheep<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reward_map_win_sheep = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reward_map_win_sheep: {}",
                    e
                )
            });
            self
        }
        pub fn reward_map_win_wolf<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reward_map_win_wolf = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reward_map_win_wolf: {}",
                    e
                )
            });
            self
        }
        pub fn time_default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_default: {}", e));
            self
        }
        pub fn time_finish<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_finish = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_finish: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldBattleField> for super::FieldBattleField {
        type Error = String;
        fn try_from(value: FieldBattleField) -> Result<Self, String> {
            Ok(Self {
                effect_lose: value.effect_lose?,
                effect_win: value.effect_win?,
                reward_map_lose_sheep: value.reward_map_lose_sheep?,
                reward_map_lose_wolf: value.reward_map_lose_wolf?,
                reward_map_win_sheep: value.reward_map_win_sheep?,
                reward_map_win_wolf: value.reward_map_win_wolf?,
                time_default: value.time_default?,
                time_finish: value.time_finish?,
            })
        }
    }
    impl From<super::FieldBattleField> for FieldBattleField {
        fn from(value: super::FieldBattleField) -> Self {
            Self {
                effect_lose: Ok(value.effect_lose),
                effect_win: Ok(value.effect_win),
                reward_map_lose_sheep: Ok(value.reward_map_lose_sheep),
                reward_map_lose_wolf: Ok(value.reward_map_lose_wolf),
                reward_map_win_sheep: Ok(value.reward_map_win_sheep),
                reward_map_win_wolf: Ok(value.reward_map_win_wolf),
                time_default: Ok(value.time_default),
                time_finish: Ok(value.time_finish),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldBuffZoneValue {
        duration: Result<Option<i64>, String>,
        interval: Result<Option<i64>, String>,
        item_id: Result<Option<i64>, String>,
        msg: Result<Option<String>, String>,
        x1: Result<Option<i64>, String>,
        x2: Result<Option<i64>, String>,
        y1: Result<Option<i64>, String>,
        y2: Result<Option<i64>, String>,
    }
    impl Default for FieldBuffZoneValue {
        fn default() -> Self {
            Self {
                duration: Ok(Default::default()),
                interval: Ok(Default::default()),
                item_id: Ok(Default::default()),
                msg: Ok(Default::default()),
                x1: Ok(Default::default()),
                x2: Ok(Default::default()),
                y1: Ok(Default::default()),
                y2: Ok(Default::default()),
            }
        }
    }
    impl FieldBuffZoneValue {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {}", e));
            self
        }
        pub fn item_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.item_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_id: {}", e));
            self
        }
        pub fn msg<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.msg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for msg: {}", e));
            self
        }
        pub fn x1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x1: {}", e));
            self
        }
        pub fn x2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x2: {}", e));
            self
        }
        pub fn y1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y1: {}", e));
            self
        }
        pub fn y2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y2: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldBuffZoneValue> for super::FieldBuffZoneValue {
        type Error = String;
        fn try_from(value: FieldBuffZoneValue) -> Result<Self, String> {
            Ok(Self {
                duration: value.duration?,
                interval: value.interval?,
                item_id: value.item_id?,
                msg: value.msg?,
                x1: value.x1?,
                x2: value.x2?,
                y1: value.y1?,
                y2: value.y2?,
            })
        }
    }
    impl From<super::FieldBuffZoneValue> for FieldBuffZoneValue {
        fn from(value: super::FieldBuffZoneValue) -> Self {
            Self {
                duration: Ok(value.duration),
                interval: Ok(value.interval),
                item_id: Ok(value.item_id),
                msg: Ok(value.msg),
                x1: Ok(value.x1),
                x2: Ok(value.x2),
                y1: Ok(value.y1),
                y2: Ok(value.y2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldClock {
        height: Result<Option<i64>, String>,
        width: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldClock {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                width: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldClock {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {}", e));
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for width: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldClock> for super::FieldClock {
        type Error = String;
        fn try_from(value: FieldClock) -> Result<Self, String> {
            Ok(Self {
                height: value.height?,
                width: value.width?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldClock> for FieldClock {
        fn from(value: super::FieldClock) -> Self {
            Self {
                height: Ok(value.height),
                width: Ok(value.width),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldCoconut {
        avatar: Result<serde_json::Map<String, serde_json::Value>, String>,
        count_bombing: Result<Option<i64>, String>,
        count_falling: Result<Option<i64>, String>,
        count_hit: Result<Option<i64>, String>,
        count_stopped: Result<Option<i64>, String>,
        effect_lose: Result<Option<String>, String>,
        effect_win: Result<Option<String>, String>,
        event_name: Result<Option<String>, String>,
        event_object_name: Result<Option<String>, String>,
        sound_lose: Result<Option<String>, String>,
        sound_win: Result<Option<String>, String>,
        time_default: Result<Option<i64>, String>,
        time_expand: Result<Option<i64>, String>,
        time_finish: Result<Option<i64>, String>,
        time_message: Result<Option<i64>, String>,
    }
    impl Default for FieldCoconut {
        fn default() -> Self {
            Self {
                avatar: Ok(Default::default()),
                count_bombing: Ok(Default::default()),
                count_falling: Ok(Default::default()),
                count_hit: Ok(Default::default()),
                count_stopped: Ok(Default::default()),
                effect_lose: Ok(Default::default()),
                effect_win: Ok(Default::default()),
                event_name: Ok(Default::default()),
                event_object_name: Ok(Default::default()),
                sound_lose: Ok(Default::default()),
                sound_win: Ok(Default::default()),
                time_default: Ok(Default::default()),
                time_expand: Ok(Default::default()),
                time_finish: Ok(Default::default()),
                time_message: Ok(Default::default()),
            }
        }
    }
    impl FieldCoconut {
        pub fn avatar<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.avatar = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for avatar: {}", e));
            self
        }
        pub fn count_bombing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.count_bombing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count_bombing: {}", e));
            self
        }
        pub fn count_falling<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.count_falling = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count_falling: {}", e));
            self
        }
        pub fn count_hit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.count_hit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count_hit: {}", e));
            self
        }
        pub fn count_stopped<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.count_stopped = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count_stopped: {}", e));
            self
        }
        pub fn effect_lose<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.effect_lose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_lose: {}", e));
            self
        }
        pub fn effect_win<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.effect_win = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_win: {}", e));
            self
        }
        pub fn event_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.event_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_name: {}", e));
            self
        }
        pub fn event_object_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.event_object_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for event_object_name: {}",
                    e
                )
            });
            self
        }
        pub fn sound_lose<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.sound_lose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sound_lose: {}", e));
            self
        }
        pub fn sound_win<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.sound_win = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sound_win: {}", e));
            self
        }
        pub fn time_default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_default: {}", e));
            self
        }
        pub fn time_expand<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_expand = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_expand: {}", e));
            self
        }
        pub fn time_finish<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_finish = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_finish: {}", e));
            self
        }
        pub fn time_message<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_message: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldCoconut> for super::FieldCoconut {
        type Error = String;
        fn try_from(value: FieldCoconut) -> Result<Self, String> {
            Ok(Self {
                avatar: value.avatar?,
                count_bombing: value.count_bombing?,
                count_falling: value.count_falling?,
                count_hit: value.count_hit?,
                count_stopped: value.count_stopped?,
                effect_lose: value.effect_lose?,
                effect_win: value.effect_win?,
                event_name: value.event_name?,
                event_object_name: value.event_object_name?,
                sound_lose: value.sound_lose?,
                sound_win: value.sound_win?,
                time_default: value.time_default?,
                time_expand: value.time_expand?,
                time_finish: value.time_finish?,
                time_message: value.time_message?,
            })
        }
    }
    impl From<super::FieldCoconut> for FieldCoconut {
        fn from(value: super::FieldCoconut) -> Self {
            Self {
                avatar: Ok(value.avatar),
                count_bombing: Ok(value.count_bombing),
                count_falling: Ok(value.count_falling),
                count_hit: Ok(value.count_hit),
                count_stopped: Ok(value.count_stopped),
                effect_lose: Ok(value.effect_lose),
                effect_win: Ok(value.effect_win),
                event_name: Ok(value.event_name),
                event_object_name: Ok(value.event_object_name),
                sound_lose: Ok(value.sound_lose),
                sound_win: Ok(value.sound_win),
                time_default: Ok(value.time_default),
                time_expand: Ok(value.time_expand),
                time_finish: Ok(value.time_finish),
                time_message: Ok(value.time_message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldHealer {
        fall: Result<Option<i64>, String>,
        heal_max: Result<Option<i64>, String>,
        heal_min: Result<Option<i64>, String>,
        healer: Result<Option<String>, String>,
        rise: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        y_max: Result<Option<i64>, String>,
        y_min: Result<Option<i64>, String>,
    }
    impl Default for FieldHealer {
        fn default() -> Self {
            Self {
                fall: Ok(Default::default()),
                heal_max: Ok(Default::default()),
                heal_min: Ok(Default::default()),
                healer: Ok(Default::default()),
                rise: Ok(Default::default()),
                x: Ok(Default::default()),
                y_max: Ok(Default::default()),
                y_min: Ok(Default::default()),
            }
        }
    }
    impl FieldHealer {
        pub fn fall<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.fall = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fall: {}", e));
            self
        }
        pub fn heal_max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.heal_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for heal_max: {}", e));
            self
        }
        pub fn heal_min<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.heal_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for heal_min: {}", e));
            self
        }
        pub fn healer<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.healer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for healer: {}", e));
            self
        }
        pub fn rise<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rise = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rise: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y_max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y_max: {}", e));
            self
        }
        pub fn y_min<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y_min: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldHealer> for super::FieldHealer {
        type Error = String;
        fn try_from(value: FieldHealer) -> Result<Self, String> {
            Ok(Self {
                fall: value.fall?,
                heal_max: value.heal_max?,
                heal_min: value.heal_min?,
                healer: value.healer?,
                rise: value.rise?,
                x: value.x?,
                y_max: value.y_max?,
                y_min: value.y_min?,
            })
        }
    }
    impl From<super::FieldHealer> for FieldHealer {
        fn from(value: super::FieldHealer) -> Self {
            Self {
                fall: Ok(value.fall),
                heal_max: Ok(value.heal_max),
                heal_min: Ok(value.heal_min),
                healer: Ok(value.healer),
                rise: Ok(value.rise),
                x: Ok(value.x),
                y_max: Ok(value.y_max),
                y_min: Ok(value.y_min),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldInfo {
        all_move_check: Result<Option<super::Bool>, String>,
        allowed_item: Result<std::collections::HashMap<String, i64>, String>,
        auto_lie_detector: Result<Option<super::FieldInfoAutoLieDetector>, String>,
        bgm: Result<Option<String>, String>,
        block_p_boss_change: Result<Option<super::Bool>, String>,
        cloud: Result<Option<super::Bool>, String>,
        consume_item_cool_time: Result<Option<i64>, String>,
        create_mob_interval: Result<Option<i64>, String>,
        damage_check_free: Result<Option<super::Bool>, String>,
        dec_hp: Result<Option<i64>, String>,
        dec_interval: Result<Option<i64>, String>,
        dec_mp: Result<Option<i64>, String>,
        dec_rate: Result<Option<i64>, String>,
        drop_expire: Result<Option<i64>, String>,
        drop_rate: Result<Option<f64>, String>,
        effect: Result<Option<String>, String>,
        entrusted_shop: Result<Option<super::Bool>, String>,
        escort: Result<Option<super::FieldInfoEscort>, String>,
        escort_min_time: Result<Option<i64>, String>,
        everlast: Result<Option<super::Bool>, String>,
        expedition_only: Result<Option<super::Bool>, String>,
        field_limit: Result<Option<i64>, String>,
        field_sub_type: Result<Option<i64>, String>,
        field_type: Result<Option<super::StrOrNum>, String>,
        fixed_mob_capacity: Result<Option<i64>, String>,
        fly: Result<Option<super::Bool>, String>,
        forced_return: Result<Option<i64>, String>,
        fs: Result<Option<f64>, String>,
        help: Result<Option<String>, String>,
        hide_minimap: Result<Option<super::Bool>, String>,
        lb_bottom: Result<Option<i64>, String>,
        lb_side: Result<Option<i64>, String>,
        lb_top: Result<Option<i64>, String>,
        link: Result<Option<String>, String>,
        lv_force_move: Result<Option<i64>, String>,
        lv_limit: Result<Option<i64>, String>,
        map_desc: Result<Option<String>, String>,
        map_mark: Result<Option<String>, String>,
        map_name: Result<Option<String>, String>,
        mini_map_on_off: Result<Option<super::Bool>, String>,
        mob_rate: Result<Option<f64>, String>,
        move_limit: Result<Option<super::Bool>, String>,
        need_skill_for_fly: Result<Option<super::Bool>, String>,
        no_map_cmd: Result<Option<super::Bool>, String>,
        no_regen_map: Result<Option<super::Bool>, String>,
        on_first_user_enter: Result<Option<String>, String>,
        on_user_enter: Result<Option<String>, String>,
        party_only: Result<Option<super::Bool>, String>,
        personal_shop: Result<Option<super::Bool>, String>,
        phase: Result<Option<i64>, String>,
        phase_alpha: Result<Option<i64>, String>,
        phase_bg: Result<std::collections::HashMap<String, i64>, String>,
        protect_item: Result<Option<i64>, String>,
        protect_set_key: Result<Option<i64>, String>,
        reactor_shuffle: Result<Option<super::Bool>, String>,
        reactor_shuffle_name: Result<Option<String>, String>,
        recovery: Result<Option<f64>, String>,
        return_map: Result<Option<i64>, String>,
        scroll_disable: Result<Option<super::Bool>, String>,
        street_name: Result<Option<String>, String>,
        swim: Result<Option<super::Bool>, String>,
        time_limit: Result<Option<i64>, String>,
        time_mob: Result<Option<super::FieldInfoTimeMob>, String>,
        time_out: Result<Option<i64>, String>,
        town: Result<Option<super::Bool>, String>,
        version: Result<Option<i64>, String>,
        vr_bottom: Result<Option<i64>, String>,
        vr_left: Result<Option<i64>, String>,
        vr_limit: Result<Option<super::Bool>, String>,
        vr_right: Result<Option<i64>, String>,
        vr_top: Result<Option<i64>, String>,
        zakum2_hack: Result<Option<super::Bool>, String>,
    }
    impl Default for FieldInfo {
        fn default() -> Self {
            Self {
                all_move_check: Ok(Default::default()),
                allowed_item: Ok(Default::default()),
                auto_lie_detector: Ok(Default::default()),
                bgm: Ok(Default::default()),
                block_p_boss_change: Ok(Default::default()),
                cloud: Ok(Default::default()),
                consume_item_cool_time: Ok(Default::default()),
                create_mob_interval: Ok(Default::default()),
                damage_check_free: Ok(Default::default()),
                dec_hp: Ok(Default::default()),
                dec_interval: Ok(Default::default()),
                dec_mp: Ok(Default::default()),
                dec_rate: Ok(Default::default()),
                drop_expire: Ok(Default::default()),
                drop_rate: Ok(Default::default()),
                effect: Ok(Default::default()),
                entrusted_shop: Ok(Default::default()),
                escort: Ok(Default::default()),
                escort_min_time: Ok(Default::default()),
                everlast: Ok(Default::default()),
                expedition_only: Ok(Default::default()),
                field_limit: Ok(Default::default()),
                field_sub_type: Ok(Default::default()),
                field_type: Ok(Default::default()),
                fixed_mob_capacity: Ok(Default::default()),
                fly: Ok(Default::default()),
                forced_return: Ok(Default::default()),
                fs: Ok(Default::default()),
                help: Ok(Default::default()),
                hide_minimap: Ok(Default::default()),
                lb_bottom: Ok(Default::default()),
                lb_side: Ok(Default::default()),
                lb_top: Ok(Default::default()),
                link: Ok(Default::default()),
                lv_force_move: Ok(Default::default()),
                lv_limit: Ok(Default::default()),
                map_desc: Ok(Default::default()),
                map_mark: Ok(Default::default()),
                map_name: Ok(Default::default()),
                mini_map_on_off: Ok(Default::default()),
                mob_rate: Ok(Default::default()),
                move_limit: Ok(Default::default()),
                need_skill_for_fly: Ok(Default::default()),
                no_map_cmd: Ok(Default::default()),
                no_regen_map: Ok(Default::default()),
                on_first_user_enter: Ok(Default::default()),
                on_user_enter: Ok(Default::default()),
                party_only: Ok(Default::default()),
                personal_shop: Ok(Default::default()),
                phase: Ok(Default::default()),
                phase_alpha: Ok(Default::default()),
                phase_bg: Ok(Default::default()),
                protect_item: Ok(Default::default()),
                protect_set_key: Ok(Default::default()),
                reactor_shuffle: Ok(Default::default()),
                reactor_shuffle_name: Ok(Default::default()),
                recovery: Ok(Default::default()),
                return_map: Ok(Default::default()),
                scroll_disable: Ok(Default::default()),
                street_name: Ok(Default::default()),
                swim: Ok(Default::default()),
                time_limit: Ok(Default::default()),
                time_mob: Ok(Default::default()),
                time_out: Ok(Default::default()),
                town: Ok(Default::default()),
                version: Ok(Default::default()),
                vr_bottom: Ok(Default::default()),
                vr_left: Ok(Default::default()),
                vr_limit: Ok(Default::default()),
                vr_right: Ok(Default::default()),
                vr_top: Ok(Default::default()),
                zakum2_hack: Ok(Default::default()),
            }
        }
    }
    impl FieldInfo {
        pub fn all_move_check<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.all_move_check = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for all_move_check: {}", e));
            self
        }
        pub fn allowed_item<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, i64>>,
            T::Error: std::fmt::Display,
        {
            self.allowed_item = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_item: {}", e));
            self
        }
        pub fn auto_lie_detector<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldInfoAutoLieDetector>>,
            T::Error: std::fmt::Display,
        {
            self.auto_lie_detector = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for auto_lie_detector: {}",
                    e
                )
            });
            self
        }
        pub fn bgm<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.bgm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bgm: {}", e));
            self
        }
        pub fn block_p_boss_change<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.block_p_boss_change = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for block_p_boss_change: {}",
                    e
                )
            });
            self
        }
        pub fn cloud<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.cloud = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cloud: {}", e));
            self
        }
        pub fn consume_item_cool_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.consume_item_cool_time = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for consume_item_cool_time: {}",
                    e
                )
            });
            self
        }
        pub fn create_mob_interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.create_mob_interval = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for create_mob_interval: {}",
                    e
                )
            });
            self
        }
        pub fn damage_check_free<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.damage_check_free = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for damage_check_free: {}",
                    e
                )
            });
            self
        }
        pub fn dec_hp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dec_hp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dec_hp: {}", e));
            self
        }
        pub fn dec_interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dec_interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dec_interval: {}", e));
            self
        }
        pub fn dec_mp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dec_mp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dec_mp: {}", e));
            self
        }
        pub fn dec_rate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dec_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dec_rate: {}", e));
            self
        }
        pub fn drop_expire<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.drop_expire = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for drop_expire: {}", e));
            self
        }
        pub fn drop_rate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.drop_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for drop_rate: {}", e));
            self
        }
        pub fn effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect: {}", e));
            self
        }
        pub fn entrusted_shop<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.entrusted_shop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entrusted_shop: {}", e));
            self
        }
        pub fn escort<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldInfoEscort>>,
            T::Error: std::fmt::Display,
        {
            self.escort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for escort: {}", e));
            self
        }
        pub fn escort_min_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.escort_min_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for escort_min_time: {}", e));
            self
        }
        pub fn everlast<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.everlast = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for everlast: {}", e));
            self
        }
        pub fn expedition_only<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.expedition_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expedition_only: {}", e));
            self
        }
        pub fn field_limit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.field_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for field_limit: {}", e));
            self
        }
        pub fn field_sub_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.field_sub_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for field_sub_type: {}", e));
            self
        }
        pub fn field_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.field_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for field_type: {}", e));
            self
        }
        pub fn fixed_mob_capacity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.fixed_mob_capacity = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_mob_capacity: {}",
                    e
                )
            });
            self
        }
        pub fn fly<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.fly = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fly: {}", e));
            self
        }
        pub fn forced_return<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.forced_return = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for forced_return: {}", e));
            self
        }
        pub fn fs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.fs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fs: {}", e));
            self
        }
        pub fn help<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.help = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for help: {}", e));
            self
        }
        pub fn hide_minimap<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.hide_minimap = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hide_minimap: {}", e));
            self
        }
        pub fn lb_bottom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.lb_bottom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lb_bottom: {}", e));
            self
        }
        pub fn lb_side<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.lb_side = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lb_side: {}", e));
            self
        }
        pub fn lb_top<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.lb_top = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lb_top: {}", e));
            self
        }
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
        pub fn lv_force_move<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.lv_force_move = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lv_force_move: {}", e));
            self
        }
        pub fn lv_limit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.lv_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lv_limit: {}", e));
            self
        }
        pub fn map_desc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.map_desc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map_desc: {}", e));
            self
        }
        pub fn map_mark<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.map_mark = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map_mark: {}", e));
            self
        }
        pub fn map_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.map_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map_name: {}", e));
            self
        }
        pub fn mini_map_on_off<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.mini_map_on_off = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mini_map_on_off: {}", e));
            self
        }
        pub fn mob_rate<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_rate: {}", e));
            self
        }
        pub fn move_limit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.move_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for move_limit: {}", e));
            self
        }
        pub fn need_skill_for_fly<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.need_skill_for_fly = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for need_skill_for_fly: {}",
                    e
                )
            });
            self
        }
        pub fn no_map_cmd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.no_map_cmd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for no_map_cmd: {}", e));
            self
        }
        pub fn no_regen_map<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.no_regen_map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for no_regen_map: {}", e));
            self
        }
        pub fn on_first_user_enter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.on_first_user_enter = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for on_first_user_enter: {}",
                    e
                )
            });
            self
        }
        pub fn on_user_enter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.on_user_enter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for on_user_enter: {}", e));
            self
        }
        pub fn party_only<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.party_only = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for party_only: {}", e));
            self
        }
        pub fn personal_shop<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.personal_shop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for personal_shop: {}", e));
            self
        }
        pub fn phase<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.phase = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase: {}", e));
            self
        }
        pub fn phase_alpha<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.phase_alpha = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase_alpha: {}", e));
            self
        }
        pub fn phase_bg<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, i64>>,
            T::Error: std::fmt::Display,
        {
            self.phase_bg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase_bg: {}", e));
            self
        }
        pub fn protect_item<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.protect_item = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect_item: {}", e));
            self
        }
        pub fn protect_set_key<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.protect_set_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protect_set_key: {}", e));
            self
        }
        pub fn reactor_shuffle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.reactor_shuffle = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reactor_shuffle: {}", e));
            self
        }
        pub fn reactor_shuffle_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.reactor_shuffle_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reactor_shuffle_name: {}",
                    e
                )
            });
            self
        }
        pub fn recovery<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.recovery = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recovery: {}", e));
            self
        }
        pub fn return_map<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.return_map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for return_map: {}", e));
            self
        }
        pub fn scroll_disable<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.scroll_disable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scroll_disable: {}", e));
            self
        }
        pub fn street_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.street_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for street_name: {}", e));
            self
        }
        pub fn swim<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.swim = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for swim: {}", e));
            self
        }
        pub fn time_limit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_limit: {}", e));
            self
        }
        pub fn time_mob<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldInfoTimeMob>>,
            T::Error: std::fmt::Display,
        {
            self.time_mob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_mob: {}", e));
            self
        }
        pub fn time_out<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_out = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_out: {}", e));
            self
        }
        pub fn town<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.town = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for town: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
        pub fn vr_bottom<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.vr_bottom = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vr_bottom: {}", e));
            self
        }
        pub fn vr_left<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.vr_left = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vr_left: {}", e));
            self
        }
        pub fn vr_limit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.vr_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vr_limit: {}", e));
            self
        }
        pub fn vr_right<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.vr_right = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vr_right: {}", e));
            self
        }
        pub fn vr_top<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.vr_top = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vr_top: {}", e));
            self
        }
        pub fn zakum2_hack<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.zakum2_hack = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for zakum2_hack: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldInfo> for super::FieldInfo {
        type Error = String;
        fn try_from(value: FieldInfo) -> Result<Self, String> {
            Ok(Self {
                all_move_check: value.all_move_check?,
                allowed_item: value.allowed_item?,
                auto_lie_detector: value.auto_lie_detector?,
                bgm: value.bgm?,
                block_p_boss_change: value.block_p_boss_change?,
                cloud: value.cloud?,
                consume_item_cool_time: value.consume_item_cool_time?,
                create_mob_interval: value.create_mob_interval?,
                damage_check_free: value.damage_check_free?,
                dec_hp: value.dec_hp?,
                dec_interval: value.dec_interval?,
                dec_mp: value.dec_mp?,
                dec_rate: value.dec_rate?,
                drop_expire: value.drop_expire?,
                drop_rate: value.drop_rate?,
                effect: value.effect?,
                entrusted_shop: value.entrusted_shop?,
                escort: value.escort?,
                escort_min_time: value.escort_min_time?,
                everlast: value.everlast?,
                expedition_only: value.expedition_only?,
                field_limit: value.field_limit?,
                field_sub_type: value.field_sub_type?,
                field_type: value.field_type?,
                fixed_mob_capacity: value.fixed_mob_capacity?,
                fly: value.fly?,
                forced_return: value.forced_return?,
                fs: value.fs?,
                help: value.help?,
                hide_minimap: value.hide_minimap?,
                lb_bottom: value.lb_bottom?,
                lb_side: value.lb_side?,
                lb_top: value.lb_top?,
                link: value.link?,
                lv_force_move: value.lv_force_move?,
                lv_limit: value.lv_limit?,
                map_desc: value.map_desc?,
                map_mark: value.map_mark?,
                map_name: value.map_name?,
                mini_map_on_off: value.mini_map_on_off?,
                mob_rate: value.mob_rate?,
                move_limit: value.move_limit?,
                need_skill_for_fly: value.need_skill_for_fly?,
                no_map_cmd: value.no_map_cmd?,
                no_regen_map: value.no_regen_map?,
                on_first_user_enter: value.on_first_user_enter?,
                on_user_enter: value.on_user_enter?,
                party_only: value.party_only?,
                personal_shop: value.personal_shop?,
                phase: value.phase?,
                phase_alpha: value.phase_alpha?,
                phase_bg: value.phase_bg?,
                protect_item: value.protect_item?,
                protect_set_key: value.protect_set_key?,
                reactor_shuffle: value.reactor_shuffle?,
                reactor_shuffle_name: value.reactor_shuffle_name?,
                recovery: value.recovery?,
                return_map: value.return_map?,
                scroll_disable: value.scroll_disable?,
                street_name: value.street_name?,
                swim: value.swim?,
                time_limit: value.time_limit?,
                time_mob: value.time_mob?,
                time_out: value.time_out?,
                town: value.town?,
                version: value.version?,
                vr_bottom: value.vr_bottom?,
                vr_left: value.vr_left?,
                vr_limit: value.vr_limit?,
                vr_right: value.vr_right?,
                vr_top: value.vr_top?,
                zakum2_hack: value.zakum2_hack?,
            })
        }
    }
    impl From<super::FieldInfo> for FieldInfo {
        fn from(value: super::FieldInfo) -> Self {
            Self {
                all_move_check: Ok(value.all_move_check),
                allowed_item: Ok(value.allowed_item),
                auto_lie_detector: Ok(value.auto_lie_detector),
                bgm: Ok(value.bgm),
                block_p_boss_change: Ok(value.block_p_boss_change),
                cloud: Ok(value.cloud),
                consume_item_cool_time: Ok(value.consume_item_cool_time),
                create_mob_interval: Ok(value.create_mob_interval),
                damage_check_free: Ok(value.damage_check_free),
                dec_hp: Ok(value.dec_hp),
                dec_interval: Ok(value.dec_interval),
                dec_mp: Ok(value.dec_mp),
                dec_rate: Ok(value.dec_rate),
                drop_expire: Ok(value.drop_expire),
                drop_rate: Ok(value.drop_rate),
                effect: Ok(value.effect),
                entrusted_shop: Ok(value.entrusted_shop),
                escort: Ok(value.escort),
                escort_min_time: Ok(value.escort_min_time),
                everlast: Ok(value.everlast),
                expedition_only: Ok(value.expedition_only),
                field_limit: Ok(value.field_limit),
                field_sub_type: Ok(value.field_sub_type),
                field_type: Ok(value.field_type),
                fixed_mob_capacity: Ok(value.fixed_mob_capacity),
                fly: Ok(value.fly),
                forced_return: Ok(value.forced_return),
                fs: Ok(value.fs),
                help: Ok(value.help),
                hide_minimap: Ok(value.hide_minimap),
                lb_bottom: Ok(value.lb_bottom),
                lb_side: Ok(value.lb_side),
                lb_top: Ok(value.lb_top),
                link: Ok(value.link),
                lv_force_move: Ok(value.lv_force_move),
                lv_limit: Ok(value.lv_limit),
                map_desc: Ok(value.map_desc),
                map_mark: Ok(value.map_mark),
                map_name: Ok(value.map_name),
                mini_map_on_off: Ok(value.mini_map_on_off),
                mob_rate: Ok(value.mob_rate),
                move_limit: Ok(value.move_limit),
                need_skill_for_fly: Ok(value.need_skill_for_fly),
                no_map_cmd: Ok(value.no_map_cmd),
                no_regen_map: Ok(value.no_regen_map),
                on_first_user_enter: Ok(value.on_first_user_enter),
                on_user_enter: Ok(value.on_user_enter),
                party_only: Ok(value.party_only),
                personal_shop: Ok(value.personal_shop),
                phase: Ok(value.phase),
                phase_alpha: Ok(value.phase_alpha),
                phase_bg: Ok(value.phase_bg),
                protect_item: Ok(value.protect_item),
                protect_set_key: Ok(value.protect_set_key),
                reactor_shuffle: Ok(value.reactor_shuffle),
                reactor_shuffle_name: Ok(value.reactor_shuffle_name),
                recovery: Ok(value.recovery),
                return_map: Ok(value.return_map),
                scroll_disable: Ok(value.scroll_disable),
                street_name: Ok(value.street_name),
                swim: Ok(value.swim),
                time_limit: Ok(value.time_limit),
                time_mob: Ok(value.time_mob),
                time_out: Ok(value.time_out),
                town: Ok(value.town),
                version: Ok(value.version),
                vr_bottom: Ok(value.vr_bottom),
                vr_left: Ok(value.vr_left),
                vr_limit: Ok(value.vr_limit),
                vr_right: Ok(value.vr_right),
                vr_top: Ok(value.vr_top),
                zakum2_hack: Ok(value.zakum2_hack),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldInfoAutoLieDetector {
        end_hour: Result<Option<i64>, String>,
        interval: Result<Option<i64>, String>,
        prop: Result<Option<i64>, String>,
        start_hour: Result<Option<i64>, String>,
    }
    impl Default for FieldInfoAutoLieDetector {
        fn default() -> Self {
            Self {
                end_hour: Ok(Default::default()),
                interval: Ok(Default::default()),
                prop: Ok(Default::default()),
                start_hour: Ok(Default::default()),
            }
        }
    }
    impl FieldInfoAutoLieDetector {
        pub fn end_hour<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.end_hour = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_hour: {}", e));
            self
        }
        pub fn interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interval: {}", e));
            self
        }
        pub fn prop<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.prop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prop: {}", e));
            self
        }
        pub fn start_hour<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.start_hour = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_hour: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldInfoAutoLieDetector> for super::FieldInfoAutoLieDetector {
        type Error = String;
        fn try_from(value: FieldInfoAutoLieDetector) -> Result<Self, String> {
            Ok(Self {
                end_hour: value.end_hour?,
                interval: value.interval?,
                prop: value.prop?,
                start_hour: value.start_hour?,
            })
        }
    }
    impl From<super::FieldInfoAutoLieDetector> for FieldInfoAutoLieDetector {
        fn from(value: super::FieldInfoAutoLieDetector) -> Self {
            Self {
                end_hour: Ok(value.end_hour),
                interval: Ok(value.interval),
                prop: Ok(value.prop),
                start_hour: Ok(value.start_hour),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldInfoEscort {
        check_distance: Result<Option<super::Bool>, String>,
        fail_message_on_die: Result<Option<String>, String>,
        fail_message_on_distance: Result<Option<String>, String>,
        mob_id: Result<Option<i64>, String>,
        time_out_limit: Result<Option<i64>, String>,
        time_out_warning_term: Result<Option<i64>, String>,
        warning_distance: Result<Option<i64>, String>,
        warning_message: Result<Option<String>, String>,
        weather_item_id: Result<Option<i64>, String>,
    }
    impl Default for FieldInfoEscort {
        fn default() -> Self {
            Self {
                check_distance: Ok(Default::default()),
                fail_message_on_die: Ok(Default::default()),
                fail_message_on_distance: Ok(Default::default()),
                mob_id: Ok(Default::default()),
                time_out_limit: Ok(Default::default()),
                time_out_warning_term: Ok(Default::default()),
                warning_distance: Ok(Default::default()),
                warning_message: Ok(Default::default()),
                weather_item_id: Ok(Default::default()),
            }
        }
    }
    impl FieldInfoEscort {
        pub fn check_distance<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.check_distance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for check_distance: {}", e));
            self
        }
        pub fn fail_message_on_die<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.fail_message_on_die = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fail_message_on_die: {}",
                    e
                )
            });
            self
        }
        pub fn fail_message_on_distance<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.fail_message_on_distance = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fail_message_on_distance: {}",
                    e
                )
            });
            self
        }
        pub fn mob_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_id: {}", e));
            self
        }
        pub fn time_out_limit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_out_limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_out_limit: {}", e));
            self
        }
        pub fn time_out_warning_term<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_out_warning_term = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for time_out_warning_term: {}",
                    e
                )
            });
            self
        }
        pub fn warning_distance<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.warning_distance = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for warning_distance: {}",
                    e
                )
            });
            self
        }
        pub fn warning_message<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.warning_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for warning_message: {}", e));
            self
        }
        pub fn weather_item_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.weather_item_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weather_item_id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldInfoEscort> for super::FieldInfoEscort {
        type Error = String;
        fn try_from(value: FieldInfoEscort) -> Result<Self, String> {
            Ok(Self {
                check_distance: value.check_distance?,
                fail_message_on_die: value.fail_message_on_die?,
                fail_message_on_distance: value.fail_message_on_distance?,
                mob_id: value.mob_id?,
                time_out_limit: value.time_out_limit?,
                time_out_warning_term: value.time_out_warning_term?,
                warning_distance: value.warning_distance?,
                warning_message: value.warning_message?,
                weather_item_id: value.weather_item_id?,
            })
        }
    }
    impl From<super::FieldInfoEscort> for FieldInfoEscort {
        fn from(value: super::FieldInfoEscort) -> Self {
            Self {
                check_distance: Ok(value.check_distance),
                fail_message_on_die: Ok(value.fail_message_on_die),
                fail_message_on_distance: Ok(value.fail_message_on_distance),
                mob_id: Ok(value.mob_id),
                time_out_limit: Ok(value.time_out_limit),
                time_out_warning_term: Ok(value.time_out_warning_term),
                warning_distance: Ok(value.warning_distance),
                warning_message: Ok(value.warning_message),
                weather_item_id: Ok(value.weather_item_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldInfoTimeMob {
        end_hour: Result<Option<i64>, String>,
        id: Result<Option<super::StrOrNum>, String>,
        message: Result<Option<String>, String>,
        start_hour: Result<Option<i64>, String>,
    }
    impl Default for FieldInfoTimeMob {
        fn default() -> Self {
            Self {
                end_hour: Ok(Default::default()),
                id: Ok(Default::default()),
                message: Ok(Default::default()),
                start_hour: Ok(Default::default()),
            }
        }
    }
    impl FieldInfoTimeMob {
        pub fn end_hour<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.end_hour = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for end_hour: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
        pub fn start_hour<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.start_hour = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start_hour: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldInfoTimeMob> for super::FieldInfoTimeMob {
        type Error = String;
        fn try_from(value: FieldInfoTimeMob) -> Result<Self, String> {
            Ok(Self {
                end_hour: value.end_hour?,
                id: value.id?,
                message: value.message?,
                start_hour: value.start_hour?,
            })
        }
    }
    impl From<super::FieldInfoTimeMob> for FieldInfoTimeMob {
        fn from(value: super::FieldInfoTimeMob) -> Self {
            Self {
                end_hour: Ok(value.end_hour),
                id: Ok(value.id),
                message: Ok(value.message),
                start_hour: Ok(value.start_hour),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldLadderRopeValue {
        l: Result<Option<super::Bool>, String>,
        page: Result<Option<i64>, String>,
        uf: Result<Option<super::Bool>, String>,
        x: Result<Option<i64>, String>,
        y1: Result<Option<i64>, String>,
        y2: Result<Option<i64>, String>,
    }
    impl Default for FieldLadderRopeValue {
        fn default() -> Self {
            Self {
                l: Ok(Default::default()),
                page: Ok(Default::default()),
                uf: Ok(Default::default()),
                x: Ok(Default::default()),
                y1: Ok(Default::default()),
                y2: Ok(Default::default()),
            }
        }
    }
    impl FieldLadderRopeValue {
        pub fn l<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.l = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for l: {}", e));
            self
        }
        pub fn page<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.page = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page: {}", e));
            self
        }
        pub fn uf<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.uf = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uf: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y1: {}", e));
            self
        }
        pub fn y2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y2: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldLadderRopeValue> for super::FieldLadderRopeValue {
        type Error = String;
        fn try_from(value: FieldLadderRopeValue) -> Result<Self, String> {
            Ok(Self {
                l: value.l?,
                page: value.page?,
                uf: value.uf?,
                x: value.x?,
                y1: value.y1?,
                y2: value.y2?,
            })
        }
    }
    impl From<super::FieldLadderRopeValue> for FieldLadderRopeValue {
        fn from(value: super::FieldLadderRopeValue) -> Self {
            Self {
                l: Ok(value.l),
                page: Ok(value.page),
                uf: Ok(value.uf),
                x: Ok(value.x),
                y1: Ok(value.y1),
                y2: Ok(value.y2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldLifeValue {
        cy: Result<Option<i64>, String>,
        f: Result<Option<i64>, String>,
        fh: Result<Option<i64>, String>,
        hide: Result<Option<super::Bool>, String>,
        id: Result<Option<super::StrOrNum>, String>,
        limitedname: Result<Option<String>, String>,
        mob_time: Result<Option<i64>, String>,
        rx0: Result<Option<i64>, String>,
        rx1: Result<Option<i64>, String>,
        team: Result<Option<i64>, String>,
        type_: Result<Option<String>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldLifeValue {
        fn default() -> Self {
            Self {
                cy: Ok(Default::default()),
                f: Ok(Default::default()),
                fh: Ok(Default::default()),
                hide: Ok(Default::default()),
                id: Ok(Default::default()),
                limitedname: Ok(Default::default()),
                mob_time: Ok(Default::default()),
                rx0: Ok(Default::default()),
                rx1: Ok(Default::default()),
                team: Ok(Default::default()),
                type_: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldLifeValue {
        pub fn cy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.cy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cy: {}", e));
            self
        }
        pub fn f<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.f = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for f: {}", e));
            self
        }
        pub fn fh<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.fh = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fh: {}", e));
            self
        }
        pub fn hide<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.hide = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hide: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn limitedname<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.limitedname = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limitedname: {}", e));
            self
        }
        pub fn mob_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_time: {}", e));
            self
        }
        pub fn rx0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rx0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rx0: {}", e));
            self
        }
        pub fn rx1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.rx1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rx1: {}", e));
            self
        }
        pub fn team<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.team = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for team: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldLifeValue> for super::FieldLifeValue {
        type Error = String;
        fn try_from(value: FieldLifeValue) -> Result<Self, String> {
            Ok(Self {
                cy: value.cy?,
                f: value.f?,
                fh: value.fh?,
                hide: value.hide?,
                id: value.id?,
                limitedname: value.limitedname?,
                mob_time: value.mob_time?,
                rx0: value.rx0?,
                rx1: value.rx1?,
                team: value.team?,
                type_: value.type_?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldLifeValue> for FieldLifeValue {
        fn from(value: super::FieldLifeValue) -> Self {
            Self {
                cy: Ok(value.cy),
                f: Ok(value.f),
                fh: Ok(value.fh),
                hide: Ok(value.hide),
                id: Ok(value.id),
                limitedname: Ok(value.limitedname),
                mob_time: Ok(value.mob_time),
                rx0: Ok(value.rx0),
                rx1: Ok(value.rx1),
                team: Ok(value.team),
                type_: Ok(value.type_),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMobMassacre {
        count_effect: Result<
            std::collections::HashMap<String, super::FieldMobMassacreCountEffectValue>,
            String,
        >,
        disable_skill: Result<Option<super::Bool>, String>,
        gauge: Result<Option<super::FieldMobMassacreGauge>, String>,
        map_distance: Result<Option<i64>, String>,
    }
    impl Default for FieldMobMassacre {
        fn default() -> Self {
            Self {
                count_effect: Ok(Default::default()),
                disable_skill: Ok(Default::default()),
                gauge: Ok(Default::default()),
                map_distance: Ok(Default::default()),
            }
        }
    }
    impl FieldMobMassacre {
        pub fn count_effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, super::FieldMobMassacreCountEffectValue>,
            >,
            T::Error: std::fmt::Display,
        {
            self.count_effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for count_effect: {}", e));
            self
        }
        pub fn disable_skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.disable_skill = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for disable_skill: {}", e));
            self
        }
        pub fn gauge<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldMobMassacreGauge>>,
            T::Error: std::fmt::Display,
        {
            self.gauge = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gauge: {}", e));
            self
        }
        pub fn map_distance<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.map_distance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map_distance: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMobMassacre> for super::FieldMobMassacre {
        type Error = String;
        fn try_from(value: FieldMobMassacre) -> Result<Self, String> {
            Ok(Self {
                count_effect: value.count_effect?,
                disable_skill: value.disable_skill?,
                gauge: value.gauge?,
                map_distance: value.map_distance?,
            })
        }
    }
    impl From<super::FieldMobMassacre> for FieldMobMassacre {
        fn from(value: super::FieldMobMassacre) -> Self {
            Self {
                count_effect: Ok(value.count_effect),
                disable_skill: Ok(value.disable_skill),
                gauge: Ok(value.gauge),
                map_distance: Ok(value.map_distance),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMobMassacreCountEffectValue {
        buff: Result<Option<i64>, String>,
        skill_use: Result<Option<super::Bool>, String>,
    }
    impl Default for FieldMobMassacreCountEffectValue {
        fn default() -> Self {
            Self {
                buff: Ok(Default::default()),
                skill_use: Ok(Default::default()),
            }
        }
    }
    impl FieldMobMassacreCountEffectValue {
        pub fn buff<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.buff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for buff: {}", e));
            self
        }
        pub fn skill_use<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.skill_use = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_use: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMobMassacreCountEffectValue>
        for super::FieldMobMassacreCountEffectValue
    {
        type Error = String;
        fn try_from(value: FieldMobMassacreCountEffectValue) -> Result<Self, String> {
            Ok(Self {
                buff: value.buff?,
                skill_use: value.skill_use?,
            })
        }
    }
    impl From<super::FieldMobMassacreCountEffectValue> for FieldMobMassacreCountEffectValue {
        fn from(value: super::FieldMobMassacreCountEffectValue) -> Self {
            Self {
                buff: Ok(value.buff),
                skill_use: Ok(value.skill_use),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMobMassacreGauge {
        cool_add: Result<Option<i64>, String>,
        decrease: Result<Option<i64>, String>,
        hit_add: Result<Option<i64>, String>,
        miss_sub: Result<Option<i64>, String>,
        total: Result<Option<i64>, String>,
    }
    impl Default for FieldMobMassacreGauge {
        fn default() -> Self {
            Self {
                cool_add: Ok(Default::default()),
                decrease: Ok(Default::default()),
                hit_add: Ok(Default::default()),
                miss_sub: Ok(Default::default()),
                total: Ok(Default::default()),
            }
        }
    }
    impl FieldMobMassacreGauge {
        pub fn cool_add<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.cool_add = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cool_add: {}", e));
            self
        }
        pub fn decrease<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.decrease = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for decrease: {}", e));
            self
        }
        pub fn hit_add<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.hit_add = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit_add: {}", e));
            self
        }
        pub fn miss_sub<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.miss_sub = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for miss_sub: {}", e));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMobMassacreGauge> for super::FieldMobMassacreGauge {
        type Error = String;
        fn try_from(value: FieldMobMassacreGauge) -> Result<Self, String> {
            Ok(Self {
                cool_add: value.cool_add?,
                decrease: value.decrease?,
                hit_add: value.hit_add?,
                miss_sub: value.miss_sub?,
                total: value.total?,
            })
        }
    }
    impl From<super::FieldMobMassacreGauge> for FieldMobMassacreGauge {
        fn from(value: super::FieldMobMassacreGauge) -> Self {
            Self {
                cool_add: Ok(value.cool_add),
                decrease: Ok(value.decrease),
                hit_add: Ok(value.hit_add),
                miss_sub: Ok(value.miss_sub),
                total: Ok(value.total),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMonsterCarnival {
        death_cp: Result<Option<i64>, String>,
        effect_lose: Result<Option<String>, String>,
        effect_win: Result<Option<String>, String>,
        guardian: Result<std::collections::HashMap<String, f64>, String>,
        guardian_gen_max: Result<Option<i64>, String>,
        guardian_gen_pos: Result<
            std::collections::HashMap<String, super::FieldMonsterCarnivalGuardianGenPosValue>,
            String,
        >,
        map_divided: Result<Option<super::Bool>, String>,
        map_type: Result<Option<i64>, String>,
        mob: Result<std::collections::HashMap<String, super::FieldMonsterCarnivalMobValue>, String>,
        mob_gen_max: Result<Option<i64>, String>,
        mob_gen_pos: Result<
            std::collections::HashMap<String, super::FieldMonsterCarnivalMobGenPosValue>,
            String,
        >,
        reactor_blue: Result<Option<i64>, String>,
        reactor_red: Result<Option<i64>, String>,
        reward: Result<Option<super::FieldMonsterCarnivalReward>, String>,
        reward_map_lose: Result<Option<i64>, String>,
        reward_map_win: Result<Option<i64>, String>,
        skill: Result<std::collections::HashMap<String, f64>, String>,
        sound_lose: Result<Option<String>, String>,
        sound_win: Result<Option<String>, String>,
        time_default: Result<Option<i64>, String>,
        time_expand: Result<Option<i64>, String>,
        time_finish: Result<Option<i64>, String>,
        time_message: Result<Option<i64>, String>,
    }
    impl Default for FieldMonsterCarnival {
        fn default() -> Self {
            Self {
                death_cp: Ok(Default::default()),
                effect_lose: Ok(Default::default()),
                effect_win: Ok(Default::default()),
                guardian: Ok(Default::default()),
                guardian_gen_max: Ok(Default::default()),
                guardian_gen_pos: Ok(Default::default()),
                map_divided: Ok(Default::default()),
                map_type: Ok(Default::default()),
                mob: Ok(Default::default()),
                mob_gen_max: Ok(Default::default()),
                mob_gen_pos: Ok(Default::default()),
                reactor_blue: Ok(Default::default()),
                reactor_red: Ok(Default::default()),
                reward: Ok(Default::default()),
                reward_map_lose: Ok(Default::default()),
                reward_map_win: Ok(Default::default()),
                skill: Ok(Default::default()),
                sound_lose: Ok(Default::default()),
                sound_win: Ok(Default::default()),
                time_default: Ok(Default::default()),
                time_expand: Ok(Default::default()),
                time_finish: Ok(Default::default()),
                time_message: Ok(Default::default()),
            }
        }
    }
    impl FieldMonsterCarnival {
        pub fn death_cp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.death_cp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for death_cp: {}", e));
            self
        }
        pub fn effect_lose<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.effect_lose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_lose: {}", e));
            self
        }
        pub fn effect_win<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.effect_win = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_win: {}", e));
            self
        }
        pub fn guardian<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, f64>>,
            T::Error: std::fmt::Display,
        {
            self.guardian = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for guardian: {}", e));
            self
        }
        pub fn guardian_gen_max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.guardian_gen_max = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for guardian_gen_max: {}",
                    e
                )
            });
            self
        }
        pub fn guardian_gen_pos<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, super::FieldMonsterCarnivalGuardianGenPosValue>,
            >,
            T::Error: std::fmt::Display,
        {
            self.guardian_gen_pos = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for guardian_gen_pos: {}",
                    e
                )
            });
            self
        }
        pub fn map_divided<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.map_divided = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map_divided: {}", e));
            self
        }
        pub fn map_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.map_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for map_type: {}", e));
            self
        }
        pub fn mob<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, super::FieldMonsterCarnivalMobValue>,
            >,
            T::Error: std::fmt::Display,
        {
            self.mob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob: {}", e));
            self
        }
        pub fn mob_gen_max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_gen_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_gen_max: {}", e));
            self
        }
        pub fn mob_gen_pos<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, super::FieldMonsterCarnivalMobGenPosValue>,
            >,
            T::Error: std::fmt::Display,
        {
            self.mob_gen_pos = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_gen_pos: {}", e));
            self
        }
        pub fn reactor_blue<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reactor_blue = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reactor_blue: {}", e));
            self
        }
        pub fn reactor_red<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reactor_red = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reactor_red: {}", e));
            self
        }
        pub fn reward<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldMonsterCarnivalReward>>,
            T::Error: std::fmt::Display,
        {
            self.reward = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reward: {}", e));
            self
        }
        pub fn reward_map_lose<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reward_map_lose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reward_map_lose: {}", e));
            self
        }
        pub fn reward_map_win<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reward_map_win = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reward_map_win: {}", e));
            self
        }
        pub fn skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, f64>>,
            T::Error: std::fmt::Display,
        {
            self.skill = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill: {}", e));
            self
        }
        pub fn sound_lose<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.sound_lose = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sound_lose: {}", e));
            self
        }
        pub fn sound_win<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.sound_win = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sound_win: {}", e));
            self
        }
        pub fn time_default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_default: {}", e));
            self
        }
        pub fn time_expand<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_expand = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_expand: {}", e));
            self
        }
        pub fn time_finish<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_finish = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_finish: {}", e));
            self
        }
        pub fn time_message<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_message: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMonsterCarnival> for super::FieldMonsterCarnival {
        type Error = String;
        fn try_from(value: FieldMonsterCarnival) -> Result<Self, String> {
            Ok(Self {
                death_cp: value.death_cp?,
                effect_lose: value.effect_lose?,
                effect_win: value.effect_win?,
                guardian: value.guardian?,
                guardian_gen_max: value.guardian_gen_max?,
                guardian_gen_pos: value.guardian_gen_pos?,
                map_divided: value.map_divided?,
                map_type: value.map_type?,
                mob: value.mob?,
                mob_gen_max: value.mob_gen_max?,
                mob_gen_pos: value.mob_gen_pos?,
                reactor_blue: value.reactor_blue?,
                reactor_red: value.reactor_red?,
                reward: value.reward?,
                reward_map_lose: value.reward_map_lose?,
                reward_map_win: value.reward_map_win?,
                skill: value.skill?,
                sound_lose: value.sound_lose?,
                sound_win: value.sound_win?,
                time_default: value.time_default?,
                time_expand: value.time_expand?,
                time_finish: value.time_finish?,
                time_message: value.time_message?,
            })
        }
    }
    impl From<super::FieldMonsterCarnival> for FieldMonsterCarnival {
        fn from(value: super::FieldMonsterCarnival) -> Self {
            Self {
                death_cp: Ok(value.death_cp),
                effect_lose: Ok(value.effect_lose),
                effect_win: Ok(value.effect_win),
                guardian: Ok(value.guardian),
                guardian_gen_max: Ok(value.guardian_gen_max),
                guardian_gen_pos: Ok(value.guardian_gen_pos),
                map_divided: Ok(value.map_divided),
                map_type: Ok(value.map_type),
                mob: Ok(value.mob),
                mob_gen_max: Ok(value.mob_gen_max),
                mob_gen_pos: Ok(value.mob_gen_pos),
                reactor_blue: Ok(value.reactor_blue),
                reactor_red: Ok(value.reactor_red),
                reward: Ok(value.reward),
                reward_map_lose: Ok(value.reward_map_lose),
                reward_map_win: Ok(value.reward_map_win),
                skill: Ok(value.skill),
                sound_lose: Ok(value.sound_lose),
                sound_win: Ok(value.sound_win),
                time_default: Ok(value.time_default),
                time_expand: Ok(value.time_expand),
                time_finish: Ok(value.time_finish),
                time_message: Ok(value.time_message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMonsterCarnivalGuardianGenPosValue {
        f: Result<Option<i64>, String>,
        team: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldMonsterCarnivalGuardianGenPosValue {
        fn default() -> Self {
            Self {
                f: Ok(Default::default()),
                team: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldMonsterCarnivalGuardianGenPosValue {
        pub fn f<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.f = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for f: {}", e));
            self
        }
        pub fn team<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.team = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for team: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMonsterCarnivalGuardianGenPosValue>
        for super::FieldMonsterCarnivalGuardianGenPosValue
    {
        type Error = String;
        fn try_from(value: FieldMonsterCarnivalGuardianGenPosValue) -> Result<Self, String> {
            Ok(Self {
                f: value.f?,
                team: value.team?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldMonsterCarnivalGuardianGenPosValue>
        for FieldMonsterCarnivalGuardianGenPosValue
    {
        fn from(value: super::FieldMonsterCarnivalGuardianGenPosValue) -> Self {
            Self {
                f: Ok(value.f),
                team: Ok(value.team),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMonsterCarnivalMobGenPosValue {
        cy: Result<Option<i64>, String>,
        fh: Result<Option<i64>, String>,
        team: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldMonsterCarnivalMobGenPosValue {
        fn default() -> Self {
            Self {
                cy: Ok(Default::default()),
                fh: Ok(Default::default()),
                team: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldMonsterCarnivalMobGenPosValue {
        pub fn cy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.cy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cy: {}", e));
            self
        }
        pub fn fh<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.fh = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fh: {}", e));
            self
        }
        pub fn team<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.team = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for team: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMonsterCarnivalMobGenPosValue>
        for super::FieldMonsterCarnivalMobGenPosValue
    {
        type Error = String;
        fn try_from(value: FieldMonsterCarnivalMobGenPosValue) -> Result<Self, String> {
            Ok(Self {
                cy: value.cy?,
                fh: value.fh?,
                team: value.team?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldMonsterCarnivalMobGenPosValue> for FieldMonsterCarnivalMobGenPosValue {
        fn from(value: super::FieldMonsterCarnivalMobGenPosValue) -> Self {
            Self {
                cy: Ok(value.cy),
                fh: Ok(value.fh),
                team: Ok(value.team),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMonsterCarnivalMobValue {
        id: Result<Option<super::StrOrNum>, String>,
        mob_time: Result<Option<i64>, String>,
        spend_cp: Result<Option<i64>, String>,
    }
    impl Default for FieldMonsterCarnivalMobValue {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                mob_time: Ok(Default::default()),
                spend_cp: Ok(Default::default()),
            }
        }
    }
    impl FieldMonsterCarnivalMobValue {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn mob_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_time: {}", e));
            self
        }
        pub fn spend_cp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.spend_cp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spend_cp: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMonsterCarnivalMobValue> for super::FieldMonsterCarnivalMobValue {
        type Error = String;
        fn try_from(value: FieldMonsterCarnivalMobValue) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                mob_time: value.mob_time?,
                spend_cp: value.spend_cp?,
            })
        }
    }
    impl From<super::FieldMonsterCarnivalMobValue> for FieldMonsterCarnivalMobValue {
        fn from(value: super::FieldMonsterCarnivalMobValue) -> Self {
            Self {
                id: Ok(value.id),
                mob_time: Ok(value.mob_time),
                spend_cp: Ok(value.spend_cp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMonsterCarnivalReward {
        climax: Result<Option<f64>, String>,
        cp_diff: Result<std::collections::HashMap<String, i64>, String>,
        prob_change: Result<
            std::collections::HashMap<String, super::FieldMonsterCarnivalRewardProbChangeValue>,
            String,
        >,
    }
    impl Default for FieldMonsterCarnivalReward {
        fn default() -> Self {
            Self {
                climax: Ok(Default::default()),
                cp_diff: Ok(Default::default()),
                prob_change: Ok(Default::default()),
            }
        }
    }
    impl FieldMonsterCarnivalReward {
        pub fn climax<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.climax = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for climax: {}", e));
            self
        }
        pub fn cp_diff<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, i64>>,
            T::Error: std::fmt::Display,
        {
            self.cp_diff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cp_diff: {}", e));
            self
        }
        pub fn prob_change<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, super::FieldMonsterCarnivalRewardProbChangeValue>,
            >,
            T::Error: std::fmt::Display,
        {
            self.prob_change = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prob_change: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMonsterCarnivalReward> for super::FieldMonsterCarnivalReward {
        type Error = String;
        fn try_from(value: FieldMonsterCarnivalReward) -> Result<Self, String> {
            Ok(Self {
                climax: value.climax?,
                cp_diff: value.cp_diff?,
                prob_change: value.prob_change?,
            })
        }
    }
    impl From<super::FieldMonsterCarnivalReward> for FieldMonsterCarnivalReward {
        fn from(value: super::FieldMonsterCarnivalReward) -> Self {
            Self {
                climax: Ok(value.climax),
                cp_diff: Ok(value.cp_diff),
                prob_change: Ok(value.prob_change),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldMonsterCarnivalRewardProbChangeValue {
        lose_coin: Result<Option<f64>, String>,
        lose_cp: Result<Option<f64>, String>,
        lose_nuff: Result<Option<f64>, String>,
        lose_recovery: Result<Option<f64>, String>,
        w_in_coin: Result<Option<f64>, String>,
        win_cp: Result<Option<f64>, String>,
        win_nuff: Result<Option<f64>, String>,
        win_recovery: Result<Option<f64>, String>,
    }
    impl Default for FieldMonsterCarnivalRewardProbChangeValue {
        fn default() -> Self {
            Self {
                lose_coin: Ok(Default::default()),
                lose_cp: Ok(Default::default()),
                lose_nuff: Ok(Default::default()),
                lose_recovery: Ok(Default::default()),
                w_in_coin: Ok(Default::default()),
                win_cp: Ok(Default::default()),
                win_nuff: Ok(Default::default()),
                win_recovery: Ok(Default::default()),
            }
        }
    }
    impl FieldMonsterCarnivalRewardProbChangeValue {
        pub fn lose_coin<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.lose_coin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lose_coin: {}", e));
            self
        }
        pub fn lose_cp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.lose_cp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lose_cp: {}", e));
            self
        }
        pub fn lose_nuff<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.lose_nuff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lose_nuff: {}", e));
            self
        }
        pub fn lose_recovery<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.lose_recovery = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lose_recovery: {}", e));
            self
        }
        pub fn w_in_coin<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.w_in_coin = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for w_in_coin: {}", e));
            self
        }
        pub fn win_cp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.win_cp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for win_cp: {}", e));
            self
        }
        pub fn win_nuff<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.win_nuff = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for win_nuff: {}", e));
            self
        }
        pub fn win_recovery<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.win_recovery = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for win_recovery: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldMonsterCarnivalRewardProbChangeValue>
        for super::FieldMonsterCarnivalRewardProbChangeValue
    {
        type Error = String;
        fn try_from(value: FieldMonsterCarnivalRewardProbChangeValue) -> Result<Self, String> {
            Ok(Self {
                lose_coin: value.lose_coin?,
                lose_cp: value.lose_cp?,
                lose_nuff: value.lose_nuff?,
                lose_recovery: value.lose_recovery?,
                w_in_coin: value.w_in_coin?,
                win_cp: value.win_cp?,
                win_nuff: value.win_nuff?,
                win_recovery: value.win_recovery?,
            })
        }
    }
    impl From<super::FieldMonsterCarnivalRewardProbChangeValue>
        for FieldMonsterCarnivalRewardProbChangeValue
    {
        fn from(value: super::FieldMonsterCarnivalRewardProbChangeValue) -> Self {
            Self {
                lose_coin: Ok(value.lose_coin),
                lose_cp: Ok(value.lose_cp),
                lose_nuff: Ok(value.lose_nuff),
                lose_recovery: Ok(value.lose_recovery),
                w_in_coin: Ok(value.w_in_coin),
                win_cp: Ok(value.win_cp),
                win_nuff: Ok(value.win_nuff),
                win_recovery: Ok(value.win_recovery),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldNoSkill {
        class: Result<std::collections::HashMap<String, i64>, String>,
        skill: Result<std::collections::HashMap<String, i64>, String>,
    }
    impl Default for FieldNoSkill {
        fn default() -> Self {
            Self {
                class: Ok(Default::default()),
                skill: Ok(Default::default()),
            }
        }
    }
    impl FieldNoSkill {
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, i64>>,
            T::Error: std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for class: {}", e));
            self
        }
        pub fn skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, i64>>,
            T::Error: std::fmt::Display,
        {
            self.skill = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldNoSkill> for super::FieldNoSkill {
        type Error = String;
        fn try_from(value: FieldNoSkill) -> Result<Self, String> {
            Ok(Self {
                class: value.class?,
                skill: value.skill?,
            })
        }
    }
    impl From<super::FieldNoSkill> for FieldNoSkill {
        fn from(value: super::FieldNoSkill) -> Self {
            Self {
                class: Ok(value.class),
                skill: Ok(value.skill),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldPortalValue {
        _2: Result<serde_json::Map<String, serde_json::Value>, String>,
        delay: Result<Option<super::Bool>, String>,
        f: Result<Option<f64>, String>,
        h_range: Result<Option<i64>, String>,
        hide_tooltip: Result<Option<super::Bool>, String>,
        horizontal_impact: Result<Option<i64>, String>,
        image: Result<Option<String>, String>,
        only_once: Result<Option<super::Bool>, String>,
        pn: Result<Option<String>, String>,
        pt: Result<Option<i64>, String>,
        reactor_name: Result<Option<String>, String>,
        script: Result<Option<String>, String>,
        session_value: Result<Option<String>, String>,
        session_value_key: Result<Option<String>, String>,
        teleport: Result<Option<super::Bool>, String>,
        tm: Result<Option<i64>, String>,
        tn: Result<Option<String>, String>,
        v_range: Result<Option<i64>, String>,
        vertical_impact: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldPortalValue {
        fn default() -> Self {
            Self {
                _2: Ok(Default::default()),
                delay: Ok(Default::default()),
                f: Ok(Default::default()),
                h_range: Ok(Default::default()),
                hide_tooltip: Ok(Default::default()),
                horizontal_impact: Ok(Default::default()),
                image: Ok(Default::default()),
                only_once: Ok(Default::default()),
                pn: Ok(Default::default()),
                pt: Ok(Default::default()),
                reactor_name: Ok(Default::default()),
                script: Ok(Default::default()),
                session_value: Ok(Default::default()),
                session_value_key: Ok(Default::default()),
                teleport: Ok(Default::default()),
                tm: Ok(Default::default()),
                tn: Ok(Default::default()),
                v_range: Ok(Default::default()),
                vertical_impact: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldPortalValue {
        pub fn _2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _2: {}", e));
            self
        }
        pub fn delay<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.delay = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delay: {}", e));
            self
        }
        pub fn f<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.f = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for f: {}", e));
            self
        }
        pub fn h_range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.h_range = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for h_range: {}", e));
            self
        }
        pub fn hide_tooltip<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.hide_tooltip = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hide_tooltip: {}", e));
            self
        }
        pub fn horizontal_impact<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.horizontal_impact = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for horizontal_impact: {}",
                    e
                )
            });
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn only_once<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.only_once = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for only_once: {}", e));
            self
        }
        pub fn pn<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.pn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pn: {}", e));
            self
        }
        pub fn pt<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.pt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pt: {}", e));
            self
        }
        pub fn reactor_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.reactor_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reactor_name: {}", e));
            self
        }
        pub fn script<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.script = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for script: {}", e));
            self
        }
        pub fn session_value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.session_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session_value: {}", e));
            self
        }
        pub fn session_value_key<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.session_value_key = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for session_value_key: {}",
                    e
                )
            });
            self
        }
        pub fn teleport<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.teleport = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for teleport: {}", e));
            self
        }
        pub fn tm<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.tm = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tm: {}", e));
            self
        }
        pub fn tn<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.tn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tn: {}", e));
            self
        }
        pub fn v_range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.v_range = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for v_range: {}", e));
            self
        }
        pub fn vertical_impact<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.vertical_impact = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vertical_impact: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldPortalValue> for super::FieldPortalValue {
        type Error = String;
        fn try_from(value: FieldPortalValue) -> Result<Self, String> {
            Ok(Self {
                _2: value._2?,
                delay: value.delay?,
                f: value.f?,
                h_range: value.h_range?,
                hide_tooltip: value.hide_tooltip?,
                horizontal_impact: value.horizontal_impact?,
                image: value.image?,
                only_once: value.only_once?,
                pn: value.pn?,
                pt: value.pt?,
                reactor_name: value.reactor_name?,
                script: value.script?,
                session_value: value.session_value?,
                session_value_key: value.session_value_key?,
                teleport: value.teleport?,
                tm: value.tm?,
                tn: value.tn?,
                v_range: value.v_range?,
                vertical_impact: value.vertical_impact?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldPortalValue> for FieldPortalValue {
        fn from(value: super::FieldPortalValue) -> Self {
            Self {
                _2: Ok(value._2),
                delay: Ok(value.delay),
                f: Ok(value.f),
                h_range: Ok(value.h_range),
                hide_tooltip: Ok(value.hide_tooltip),
                horizontal_impact: Ok(value.horizontal_impact),
                image: Ok(value.image),
                only_once: Ok(value.only_once),
                pn: Ok(value.pn),
                pt: Ok(value.pt),
                reactor_name: Ok(value.reactor_name),
                script: Ok(value.script),
                session_value: Ok(value.session_value),
                session_value_key: Ok(value.session_value_key),
                teleport: Ok(value.teleport),
                tm: Ok(value.tm),
                tn: Ok(value.tn),
                v_range: Ok(value.v_range),
                vertical_impact: Ok(value.vertical_impact),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldPulley {
        pulley: Result<Option<String>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldPulley {
        fn default() -> Self {
            Self {
                pulley: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldPulley {
        pub fn pulley<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.pulley = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pulley: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldPulley> for super::FieldPulley {
        type Error = String;
        fn try_from(value: FieldPulley) -> Result<Self, String> {
            Ok(Self {
                pulley: value.pulley?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldPulley> for FieldPulley {
        fn from(value: super::FieldPulley) -> Self {
            Self {
                pulley: Ok(value.pulley),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldReactorValue {
        f: Result<Option<i64>, String>,
        id: Result<Option<super::StrOrNum>, String>,
        name: Result<Option<String>, String>,
        reactor_time: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldReactorValue {
        fn default() -> Self {
            Self {
                f: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                reactor_time: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldReactorValue {
        pub fn f<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.f = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for f: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn reactor_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.reactor_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reactor_time: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldReactorValue> for super::FieldReactorValue {
        type Error = String;
        fn try_from(value: FieldReactorValue) -> Result<Self, String> {
            Ok(Self {
                f: value.f?,
                id: value.id?,
                name: value.name?,
                reactor_time: value.reactor_time?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldReactorValue> for FieldReactorValue {
        fn from(value: super::FieldReactorValue) -> Self {
            Self {
                f: Ok(value.f),
                id: Ok(value.id),
                name: Ok(value.name),
                reactor_time: Ok(value.reactor_time),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldSeatValue {
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
    }
    impl Default for FieldSeatValue {
        fn default() -> Self {
            Self {
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl FieldSeatValue {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldSeatValue> for super::FieldSeatValue {
        type Error = String;
        fn try_from(value: FieldSeatValue) -> Result<Self, String> {
            Ok(Self {
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::FieldSeatValue> for FieldSeatValue {
        fn from(value: super::FieldSeatValue) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldShipObj {
        f: Result<Option<i64>, String>,
        ship_kind: Result<Option<i64>, String>,
        ship_obj: Result<Option<String>, String>,
        t_move: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        x0: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
        z: Result<Option<i64>, String>,
    }
    impl Default for FieldShipObj {
        fn default() -> Self {
            Self {
                f: Ok(Default::default()),
                ship_kind: Ok(Default::default()),
                ship_obj: Ok(Default::default()),
                t_move: Ok(Default::default()),
                x: Ok(Default::default()),
                x0: Ok(Default::default()),
                y: Ok(Default::default()),
                z: Ok(Default::default()),
            }
        }
    }
    impl FieldShipObj {
        pub fn f<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.f = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for f: {}", e));
            self
        }
        pub fn ship_kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.ship_kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ship_kind: {}", e));
            self
        }
        pub fn ship_obj<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.ship_obj = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ship_obj: {}", e));
            self
        }
        pub fn t_move<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.t_move = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for t_move: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn x0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x0: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
        pub fn z<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.z = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for z: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldShipObj> for super::FieldShipObj {
        type Error = String;
        fn try_from(value: FieldShipObj) -> Result<Self, String> {
            Ok(Self {
                f: value.f?,
                ship_kind: value.ship_kind?,
                ship_obj: value.ship_obj?,
                t_move: value.t_move?,
                x: value.x?,
                x0: value.x0?,
                y: value.y?,
                z: value.z?,
            })
        }
    }
    impl From<super::FieldShipObj> for FieldShipObj {
        fn from(value: super::FieldShipObj) -> Self {
            Self {
                f: Ok(value.f),
                ship_kind: Ok(value.ship_kind),
                ship_obj: Ok(value.ship_obj),
                t_move: Ok(value.t_move),
                x: Ok(value.x),
                x0: Ok(value.x0),
                y: Ok(value.y),
                z: Ok(value.z),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldSnowBall {
        _0: Result<serde_json::Map<String, serde_json::Value>, String>,
        _1: Result<serde_json::Map<String, serde_json::Value>, String>,
        damage_snow_ball: Result<Option<super::StrOrNum>, String>,
        damage_snow_man0: Result<Option<i64>, String>,
        damage_snow_man1: Result<Option<i64>, String>,
        dx: Result<Option<i64>, String>,
        recovery_amount: Result<Option<i64>, String>,
        section1: Result<Option<i64>, String>,
        section2: Result<Option<i64>, String>,
        section3: Result<Option<i64>, String>,
        snow_man_hp: Result<Option<i64>, String>,
        snow_man_wait: Result<Option<i64>, String>,
        speed: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        x0: Result<Option<i64>, String>,
        x_max: Result<Option<i64>, String>,
        x_min: Result<Option<i64>, String>,
    }
    impl Default for FieldSnowBall {
        fn default() -> Self {
            Self {
                _0: Ok(Default::default()),
                _1: Ok(Default::default()),
                damage_snow_ball: Ok(Default::default()),
                damage_snow_man0: Ok(Default::default()),
                damage_snow_man1: Ok(Default::default()),
                dx: Ok(Default::default()),
                recovery_amount: Ok(Default::default()),
                section1: Ok(Default::default()),
                section2: Ok(Default::default()),
                section3: Ok(Default::default()),
                snow_man_hp: Ok(Default::default()),
                snow_man_wait: Ok(Default::default()),
                speed: Ok(Default::default()),
                x: Ok(Default::default()),
                x0: Ok(Default::default()),
                x_max: Ok(Default::default()),
                x_min: Ok(Default::default()),
            }
        }
    }
    impl FieldSnowBall {
        pub fn _0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _0: {}", e));
            self
        }
        pub fn _1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self._1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for _1: {}", e));
            self
        }
        pub fn damage_snow_ball<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.damage_snow_ball = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for damage_snow_ball: {}",
                    e
                )
            });
            self
        }
        pub fn damage_snow_man0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.damage_snow_man0 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for damage_snow_man0: {}",
                    e
                )
            });
            self
        }
        pub fn damage_snow_man1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.damage_snow_man1 = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for damage_snow_man1: {}",
                    e
                )
            });
            self
        }
        pub fn dx<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.dx = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dx: {}", e));
            self
        }
        pub fn recovery_amount<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.recovery_amount = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recovery_amount: {}", e));
            self
        }
        pub fn section1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.section1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for section1: {}", e));
            self
        }
        pub fn section2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.section2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for section2: {}", e));
            self
        }
        pub fn section3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.section3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for section3: {}", e));
            self
        }
        pub fn snow_man_hp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.snow_man_hp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for snow_man_hp: {}", e));
            self
        }
        pub fn snow_man_wait<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.snow_man_wait = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for snow_man_wait: {}", e));
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn x0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x0: {}", e));
            self
        }
        pub fn x_max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x_max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x_max: {}", e));
            self
        }
        pub fn x_min<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x_min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x_min: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldSnowBall> for super::FieldSnowBall {
        type Error = String;
        fn try_from(value: FieldSnowBall) -> Result<Self, String> {
            Ok(Self {
                _0: value._0?,
                _1: value._1?,
                damage_snow_ball: value.damage_snow_ball?,
                damage_snow_man0: value.damage_snow_man0?,
                damage_snow_man1: value.damage_snow_man1?,
                dx: value.dx?,
                recovery_amount: value.recovery_amount?,
                section1: value.section1?,
                section2: value.section2?,
                section3: value.section3?,
                snow_man_hp: value.snow_man_hp?,
                snow_man_wait: value.snow_man_wait?,
                speed: value.speed?,
                x: value.x?,
                x0: value.x0?,
                x_max: value.x_max?,
                x_min: value.x_min?,
            })
        }
    }
    impl From<super::FieldSnowBall> for FieldSnowBall {
        fn from(value: super::FieldSnowBall) -> Self {
            Self {
                _0: Ok(value._0),
                _1: Ok(value._1),
                damage_snow_ball: Ok(value.damage_snow_ball),
                damage_snow_man0: Ok(value.damage_snow_man0),
                damage_snow_man1: Ok(value.damage_snow_man1),
                dx: Ok(value.dx),
                recovery_amount: Ok(value.recovery_amount),
                section1: Ok(value.section1),
                section2: Ok(value.section2),
                section3: Ok(value.section3),
                snow_man_hp: Ok(value.snow_man_hp),
                snow_man_wait: Ok(value.snow_man_wait),
                speed: Ok(value.speed),
                x: Ok(value.x),
                x0: Ok(value.x0),
                x_max: Ok(value.x_max),
                x_min: Ok(value.x_min),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldSnowMan {
        path: Result<Option<String>, String>,
    }
    impl Default for FieldSnowMan {
        fn default() -> Self {
            Self {
                path: Ok(Default::default()),
            }
        }
    }
    impl FieldSnowMan {
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldSnowMan> for super::FieldSnowMan {
        type Error = String;
        fn try_from(value: FieldSnowMan) -> Result<Self, String> {
            Ok(Self { path: value.path? })
        }
    }
    impl From<super::FieldSnowMan> for FieldSnowMan {
        fn from(value: super::FieldSnowMan) -> Self {
            Self {
                path: Ok(value.path),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldSwimAreaValue {
        x1: Result<Option<i64>, String>,
        x2: Result<Option<i64>, String>,
        y1: Result<Option<i64>, String>,
        y2: Result<Option<i64>, String>,
    }
    impl Default for FieldSwimAreaValue {
        fn default() -> Self {
            Self {
                x1: Ok(Default::default()),
                x2: Ok(Default::default()),
                y1: Ok(Default::default()),
                y2: Ok(Default::default()),
            }
        }
    }
    impl FieldSwimAreaValue {
        pub fn x1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x1: {}", e));
            self
        }
        pub fn x2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x2: {}", e));
            self
        }
        pub fn y1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y1: {}", e));
            self
        }
        pub fn y2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y2: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldSwimAreaValue> for super::FieldSwimAreaValue {
        type Error = String;
        fn try_from(value: FieldSwimAreaValue) -> Result<Self, String> {
            Ok(Self {
                x1: value.x1?,
                x2: value.x2?,
                y1: value.y1?,
                y2: value.y2?,
            })
        }
    }
    impl From<super::FieldSwimAreaValue> for FieldSwimAreaValue {
        fn from(value: super::FieldSwimAreaValue) -> Self {
            Self {
                x1: Ok(value.x1),
                x2: Ok(value.x2),
                y1: Ok(value.y1),
                y2: Ok(value.y2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldToolTipValue {
        x1: Result<Option<i64>, String>,
        x2: Result<Option<i64>, String>,
        y1: Result<Option<i64>, String>,
        y2: Result<Option<i64>, String>,
    }
    impl Default for FieldToolTipValue {
        fn default() -> Self {
            Self {
                x1: Ok(Default::default()),
                x2: Ok(Default::default()),
                y1: Ok(Default::default()),
                y2: Ok(Default::default()),
            }
        }
    }
    impl FieldToolTipValue {
        pub fn x1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x1: {}", e));
            self
        }
        pub fn x2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x2: {}", e));
            self
        }
        pub fn y1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y1: {}", e));
            self
        }
        pub fn y2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y2: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldToolTipValue> for super::FieldToolTipValue {
        type Error = String;
        fn try_from(value: FieldToolTipValue) -> Result<Self, String> {
            Ok(Self {
                x1: value.x1?,
                x2: value.x2?,
                y1: value.y1?,
                y2: value.y2?,
            })
        }
    }
    impl From<super::FieldToolTipValue> for FieldToolTipValue {
        fn from(value: super::FieldToolTipValue) -> Self {
            Self {
                x1: Ok(value.x1),
                x2: Ok(value.x2),
                y1: Ok(value.y1),
                y2: Ok(value.y2),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldUserValue {
        cond: Result<Option<super::FieldUserValueCond>, String>,
        look: Result<Option<super::FieldUserValueLook>, String>,
        noitem: Result<std::collections::HashMap<String, i64>, String>,
        stat: Result<Option<super::FieldUserValueStat>, String>,
    }
    impl Default for FieldUserValue {
        fn default() -> Self {
            Self {
                cond: Ok(Default::default()),
                look: Ok(Default::default()),
                noitem: Ok(Default::default()),
                stat: Ok(Default::default()),
            }
        }
    }
    impl FieldUserValue {
        pub fn cond<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldUserValueCond>>,
            T::Error: std::fmt::Display,
        {
            self.cond = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cond: {}", e));
            self
        }
        pub fn look<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldUserValueLook>>,
            T::Error: std::fmt::Display,
        {
            self.look = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for look: {}", e));
            self
        }
        pub fn noitem<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, i64>>,
            T::Error: std::fmt::Display,
        {
            self.noitem = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for noitem: {}", e));
            self
        }
        pub fn stat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FieldUserValueStat>>,
            T::Error: std::fmt::Display,
        {
            self.stat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stat: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldUserValue> for super::FieldUserValue {
        type Error = String;
        fn try_from(value: FieldUserValue) -> Result<Self, String> {
            Ok(Self {
                cond: value.cond?,
                look: value.look?,
                noitem: value.noitem?,
                stat: value.stat?,
            })
        }
    }
    impl From<super::FieldUserValue> for FieldUserValue {
        fn from(value: super::FieldUserValue) -> Self {
            Self {
                cond: Ok(value.cond),
                look: Ok(value.look),
                noitem: Ok(value.noitem),
                stat: Ok(value.stat),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldUserValueCond {
        battle_field_team: Result<Option<super::StrOrNum>, String>,
        compare: Result<Option<super::Bool>, String>,
        gender: Result<Option<super::StrOrNum>, String>,
        item_count: Result<Option<super::StrOrNum>, String>,
        item_id: Result<Option<super::StrOrNum>, String>,
        job: Result<Option<super::StrOrNum>, String>,
        job_category: Result<Option<super::StrOrNum>, String>,
        level: Result<Option<super::StrOrNum>, String>,
        target: Result<Option<super::StrOrNum>, String>,
    }
    impl Default for FieldUserValueCond {
        fn default() -> Self {
            Self {
                battle_field_team: Ok(Default::default()),
                compare: Ok(Default::default()),
                gender: Ok(Default::default()),
                item_count: Ok(Default::default()),
                item_id: Ok(Default::default()),
                job: Ok(Default::default()),
                job_category: Ok(Default::default()),
                level: Ok(Default::default()),
                target: Ok(Default::default()),
            }
        }
    }
    impl FieldUserValueCond {
        pub fn battle_field_team<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.battle_field_team = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for battle_field_team: {}",
                    e
                )
            });
            self
        }
        pub fn compare<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.compare = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for compare: {}", e));
            self
        }
        pub fn gender<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.gender = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gender: {}", e));
            self
        }
        pub fn item_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.item_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_count: {}", e));
            self
        }
        pub fn item_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.item_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_id: {}", e));
            self
        }
        pub fn job<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.job = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job: {}", e));
            self
        }
        pub fn job_category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.job_category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for job_category: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {}", e));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldUserValueCond> for super::FieldUserValueCond {
        type Error = String;
        fn try_from(value: FieldUserValueCond) -> Result<Self, String> {
            Ok(Self {
                battle_field_team: value.battle_field_team?,
                compare: value.compare?,
                gender: value.gender?,
                item_count: value.item_count?,
                item_id: value.item_id?,
                job: value.job?,
                job_category: value.job_category?,
                level: value.level?,
                target: value.target?,
            })
        }
    }
    impl From<super::FieldUserValueCond> for FieldUserValueCond {
        fn from(value: super::FieldUserValueCond) -> Self {
            Self {
                battle_field_team: Ok(value.battle_field_team),
                compare: Ok(value.compare),
                gender: Ok(value.gender),
                item_count: Ok(value.item_count),
                item_id: Ok(value.item_id),
                job: Ok(value.job),
                job_category: Ok(value.job_category),
                level: Ok(value.level),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldUserValueLook {
        cap: Result<Option<super::StrOrNum>, String>,
        cape: Result<Option<super::StrOrNum>, String>,
        clothes: Result<Option<super::StrOrNum>, String>,
        ear_acc: Result<Option<super::StrOrNum>, String>,
        face_acc: Result<Option<super::StrOrNum>, String>,
        gloves: Result<Option<super::StrOrNum>, String>,
        pants: Result<Option<super::StrOrNum>, String>,
        shield: Result<Option<super::StrOrNum>, String>,
        shoes: Result<Option<super::StrOrNum>, String>,
        weapon: Result<Option<super::StrOrNum>, String>,
    }
    impl Default for FieldUserValueLook {
        fn default() -> Self {
            Self {
                cap: Ok(Default::default()),
                cape: Ok(Default::default()),
                clothes: Ok(Default::default()),
                ear_acc: Ok(Default::default()),
                face_acc: Ok(Default::default()),
                gloves: Ok(Default::default()),
                pants: Ok(Default::default()),
                shield: Ok(Default::default()),
                shoes: Ok(Default::default()),
                weapon: Ok(Default::default()),
            }
        }
    }
    impl FieldUserValueLook {
        pub fn cap<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.cap = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cap: {}", e));
            self
        }
        pub fn cape<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.cape = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cape: {}", e));
            self
        }
        pub fn clothes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.clothes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clothes: {}", e));
            self
        }
        pub fn ear_acc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.ear_acc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ear_acc: {}", e));
            self
        }
        pub fn face_acc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.face_acc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for face_acc: {}", e));
            self
        }
        pub fn gloves<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.gloves = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gloves: {}", e));
            self
        }
        pub fn pants<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.pants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pants: {}", e));
            self
        }
        pub fn shield<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.shield = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shield: {}", e));
            self
        }
        pub fn shoes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.shoes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shoes: {}", e));
            self
        }
        pub fn weapon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.weapon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weapon: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldUserValueLook> for super::FieldUserValueLook {
        type Error = String;
        fn try_from(value: FieldUserValueLook) -> Result<Self, String> {
            Ok(Self {
                cap: value.cap?,
                cape: value.cape?,
                clothes: value.clothes?,
                ear_acc: value.ear_acc?,
                face_acc: value.face_acc?,
                gloves: value.gloves?,
                pants: value.pants?,
                shield: value.shield?,
                shoes: value.shoes?,
                weapon: value.weapon?,
            })
        }
    }
    impl From<super::FieldUserValueLook> for FieldUserValueLook {
        fn from(value: super::FieldUserValueLook) -> Self {
            Self {
                cap: Ok(value.cap),
                cape: Ok(value.cape),
                clothes: Ok(value.clothes),
                ear_acc: Ok(value.ear_acc),
                face_acc: Ok(value.face_acc),
                gloves: Ok(value.gloves),
                pants: Ok(value.pants),
                shield: Ok(value.shield),
                shoes: Ok(value.shoes),
                weapon: Ok(value.weapon),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldUserValueStat {
        acc: Result<Option<super::StrOrNum>, String>,
        dex: Result<Option<super::StrOrNum>, String>,
        eva: Result<Option<super::StrOrNum>, String>,
        int: Result<Option<super::StrOrNum>, String>,
        jump: Result<Option<super::StrOrNum>, String>,
        luk: Result<Option<super::StrOrNum>, String>,
        mad: Result<Option<super::StrOrNum>, String>,
        pad: Result<Option<super::StrOrNum>, String>,
        speed: Result<Option<super::StrOrNum>, String>,
        speedmax: Result<Option<super::StrOrNum>, String>,
        str: Result<Option<super::StrOrNum>, String>,
    }
    impl Default for FieldUserValueStat {
        fn default() -> Self {
            Self {
                acc: Ok(Default::default()),
                dex: Ok(Default::default()),
                eva: Ok(Default::default()),
                int: Ok(Default::default()),
                jump: Ok(Default::default()),
                luk: Ok(Default::default()),
                mad: Ok(Default::default()),
                pad: Ok(Default::default()),
                speed: Ok(Default::default()),
                speedmax: Ok(Default::default()),
                str: Ok(Default::default()),
            }
        }
    }
    impl FieldUserValueStat {
        pub fn acc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.acc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for acc: {}", e));
            self
        }
        pub fn dex<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.dex = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dex: {}", e));
            self
        }
        pub fn eva<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.eva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eva: {}", e));
            self
        }
        pub fn int<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.int = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for int: {}", e));
            self
        }
        pub fn jump<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.jump = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jump: {}", e));
            self
        }
        pub fn luk<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.luk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for luk: {}", e));
            self
        }
        pub fn mad<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.mad = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mad: {}", e));
            self
        }
        pub fn pad<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.pad = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pad: {}", e));
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed: {}", e));
            self
        }
        pub fn speedmax<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.speedmax = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speedmax: {}", e));
            self
        }
        pub fn str<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for str: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldUserValueStat> for super::FieldUserValueStat {
        type Error = String;
        fn try_from(value: FieldUserValueStat) -> Result<Self, String> {
            Ok(Self {
                acc: value.acc?,
                dex: value.dex?,
                eva: value.eva?,
                int: value.int?,
                jump: value.jump?,
                luk: value.luk?,
                mad: value.mad?,
                pad: value.pad?,
                speed: value.speed?,
                speedmax: value.speedmax?,
                str: value.str?,
            })
        }
    }
    impl From<super::FieldUserValueStat> for FieldUserValueStat {
        fn from(value: super::FieldUserValueStat) -> Self {
            Self {
                acc: Ok(value.acc),
                dex: Ok(value.dex),
                eva: Ok(value.eva),
                int: Ok(value.int),
                jump: Ok(value.jump),
                luk: Ok(value.luk),
                mad: Ok(value.mad),
                pad: Ok(value.pad),
                speed: Ok(value.speed),
                speedmax: Ok(value.speedmax),
                str: Ok(value.str),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FieldWeatherValue {
        item: Result<Option<super::StrOrNum>, String>,
        msg: Result<Option<String>, String>,
        option: Result<Option<super::Bool>, String>,
        time: Result<Option<i64>, String>,
    }
    impl Default for FieldWeatherValue {
        fn default() -> Self {
            Self {
                item: Ok(Default::default()),
                msg: Ok(Default::default()),
                option: Ok(Default::default()),
                time: Ok(Default::default()),
            }
        }
    }
    impl FieldWeatherValue {
        pub fn item<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.item = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item: {}", e));
            self
        }
        pub fn msg<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.msg = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for msg: {}", e));
            self
        }
        pub fn option<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.option = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for option: {}", e));
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FieldWeatherValue> for super::FieldWeatherValue {
        type Error = String;
        fn try_from(value: FieldWeatherValue) -> Result<Self, String> {
            Ok(Self {
                item: value.item?,
                msg: value.msg?,
                option: value.option?,
                time: value.time?,
            })
        }
    }
    impl From<super::FieldWeatherValue> for FieldWeatherValue {
        fn from(value: super::FieldWeatherValue) -> Self {
            Self {
                item: Ok(value.item),
                msg: Ok(value.msg),
                option: Ok(value.option),
                time: Ok(value.time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Skill {
        info: Result<Option<super::SkillInfo>, String>,
        skill: Result<std::collections::HashMap<String, super::SkillSkillValue>, String>,
    }
    impl Default for Skill {
        fn default() -> Self {
            Self {
                info: Ok(Default::default()),
                skill: Ok(Default::default()),
            }
        }
    }
    impl Skill {
        pub fn info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillInfo>>,
            T::Error: std::fmt::Display,
        {
            self.info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info: {}", e));
            self
        }
        pub fn skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::SkillSkillValue>>,
            T::Error: std::fmt::Display,
        {
            self.skill = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Skill> for super::Skill {
        type Error = String;
        fn try_from(value: Skill) -> Result<Self, String> {
            Ok(Self {
                info: value.info?,
                skill: value.skill?,
            })
        }
    }
    impl From<super::Skill> for Skill {
        fn from(value: super::Skill) -> Self {
            Self {
                info: Ok(value.info),
                skill: Ok(value.skill),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillCommonInfo {
        acc: Result<Option<super::SkillExpr>, String>,
        action: Result<Option<String>, String>,
        asr_r: Result<Option<super::SkillExpr>, String>,
        attack_count: Result<Option<super::SkillExpr>, String>,
        bullet_consume: Result<Option<super::StrOrNum>, String>,
        bullet_count: Result<Option<super::StrOrNum>, String>,
        cooltime: Result<Option<super::SkillExpr>, String>,
        cr: Result<Option<super::SkillExpr>, String>,
        criticaldamage_max: Result<Option<super::SkillExpr>, String>,
        criticaldamage_min: Result<Option<super::SkillExpr>, String>,
        dam_r: Result<Option<super::SkillExpr>, String>,
        damage: Result<Option<super::SkillExpr>, String>,
        dot: Result<Option<super::SkillExpr>, String>,
        dot_interval: Result<Option<super::SkillExpr>, String>,
        dot_time: Result<Option<super::SkillExpr>, String>,
        emdd: Result<Option<super::SkillExpr>, String>,
        emhp: Result<Option<super::SkillExpr>, String>,
        emmp: Result<Option<super::SkillExpr>, String>,
        epad: Result<Option<super::SkillExpr>, String>,
        epdd: Result<Option<super::SkillExpr>, String>,
        er: Result<Option<super::SkillExpr>, String>,
        eva: Result<Option<super::SkillExpr>, String>,
        exp_r: Result<Option<super::SkillExpr>, String>,
        hp: Result<Option<super::SkillExpr>, String>,
        hp_con: Result<Option<super::SkillExpr>, String>,
        ignore_mobpdp_r: Result<Option<super::SkillExpr>, String>,
        item_con: Result<Option<super::StrOrNum>, String>,
        item_con_no: Result<Option<super::StrOrNum>, String>,
        item_consume: Result<Option<super::StrOrNum>, String>,
        jump: Result<Option<super::SkillExpr>, String>,
        lt: Result<Option<super::Vec2>, String>,
        mad: Result<Option<super::SkillExpr>, String>,
        mad_x: Result<Option<super::SkillExpr>, String>,
        mastery: Result<Option<super::SkillExpr>, String>,
        max_level: Result<Option<super::StrOrNum>, String>,
        mdd: Result<Option<super::SkillExpr>, String>,
        mdd_r: Result<Option<super::SkillExpr>, String>,
        meso_r: Result<Option<super::SkillExpr>, String>,
        mhp_r: Result<Option<super::SkillExpr>, String>,
        mmp_r: Result<Option<super::SkillExpr>, String>,
        mob_count: Result<Option<super::SkillExpr>, String>,
        money_con: Result<Option<super::SkillExpr>, String>,
        morph: Result<Option<super::StrOrNum>, String>,
        mp: Result<Option<super::SkillExpr>, String>,
        mp_con: Result<Option<super::SkillExpr>, String>,
        pad: Result<Option<super::SkillExpr>, String>,
        pad_x: Result<Option<super::SkillExpr>, String>,
        pdd: Result<Option<super::SkillExpr>, String>,
        pdd_r: Result<Option<super::SkillExpr>, String>,
        prop: Result<Option<super::SkillExpr>, String>,
        range: Result<Option<super::SkillExpr>, String>,
        rb: Result<Option<super::Vec2>, String>,
        self_destruction: Result<Option<super::SkillExpr>, String>,
        speed: Result<Option<super::SkillExpr>, String>,
        sub_prop: Result<Option<super::SkillExpr>, String>,
        sub_time: Result<Option<super::SkillExpr>, String>,
        t: Result<Option<super::SkillExpr>, String>,
        ter_r: Result<Option<super::SkillExpr>, String>,
        time: Result<Option<super::SkillExpr>, String>,
        u: Result<Option<super::SkillExpr>, String>,
        v: Result<Option<super::SkillExpr>, String>,
        w: Result<Option<super::SkillExpr>, String>,
        x: Result<Option<super::SkillExpr>, String>,
        y: Result<Option<super::SkillExpr>, String>,
        z: Result<Option<super::SkillExpr>, String>,
    }
    impl Default for SkillCommonInfo {
        fn default() -> Self {
            Self {
                acc: Ok(Default::default()),
                action: Ok(Default::default()),
                asr_r: Ok(Default::default()),
                attack_count: Ok(Default::default()),
                bullet_consume: Ok(Default::default()),
                bullet_count: Ok(Default::default()),
                cooltime: Ok(Default::default()),
                cr: Ok(Default::default()),
                criticaldamage_max: Ok(Default::default()),
                criticaldamage_min: Ok(Default::default()),
                dam_r: Ok(Default::default()),
                damage: Ok(Default::default()),
                dot: Ok(Default::default()),
                dot_interval: Ok(Default::default()),
                dot_time: Ok(Default::default()),
                emdd: Ok(Default::default()),
                emhp: Ok(Default::default()),
                emmp: Ok(Default::default()),
                epad: Ok(Default::default()),
                epdd: Ok(Default::default()),
                er: Ok(Default::default()),
                eva: Ok(Default::default()),
                exp_r: Ok(Default::default()),
                hp: Ok(Default::default()),
                hp_con: Ok(Default::default()),
                ignore_mobpdp_r: Ok(Default::default()),
                item_con: Ok(Default::default()),
                item_con_no: Ok(Default::default()),
                item_consume: Ok(Default::default()),
                jump: Ok(Default::default()),
                lt: Ok(Default::default()),
                mad: Ok(Default::default()),
                mad_x: Ok(Default::default()),
                mastery: Ok(Default::default()),
                max_level: Ok(Default::default()),
                mdd: Ok(Default::default()),
                mdd_r: Ok(Default::default()),
                meso_r: Ok(Default::default()),
                mhp_r: Ok(Default::default()),
                mmp_r: Ok(Default::default()),
                mob_count: Ok(Default::default()),
                money_con: Ok(Default::default()),
                morph: Ok(Default::default()),
                mp: Ok(Default::default()),
                mp_con: Ok(Default::default()),
                pad: Ok(Default::default()),
                pad_x: Ok(Default::default()),
                pdd: Ok(Default::default()),
                pdd_r: Ok(Default::default()),
                prop: Ok(Default::default()),
                range: Ok(Default::default()),
                rb: Ok(Default::default()),
                self_destruction: Ok(Default::default()),
                speed: Ok(Default::default()),
                sub_prop: Ok(Default::default()),
                sub_time: Ok(Default::default()),
                t: Ok(Default::default()),
                ter_r: Ok(Default::default()),
                time: Ok(Default::default()),
                u: Ok(Default::default()),
                v: Ok(Default::default()),
                w: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
                z: Ok(Default::default()),
            }
        }
    }
    impl SkillCommonInfo {
        pub fn acc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.acc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for acc: {}", e));
            self
        }
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {}", e));
            self
        }
        pub fn asr_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.asr_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asr_r: {}", e));
            self
        }
        pub fn attack_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.attack_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack_count: {}", e));
            self
        }
        pub fn bullet_consume<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.bullet_consume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bullet_consume: {}", e));
            self
        }
        pub fn bullet_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.bullet_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bullet_count: {}", e));
            self
        }
        pub fn cooltime<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.cooltime = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cooltime: {}", e));
            self
        }
        pub fn cr<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.cr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cr: {}", e));
            self
        }
        pub fn criticaldamage_max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.criticaldamage_max = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for criticaldamage_max: {}",
                    e
                )
            });
            self
        }
        pub fn criticaldamage_min<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.criticaldamage_min = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for criticaldamage_min: {}",
                    e
                )
            });
            self
        }
        pub fn dam_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.dam_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dam_r: {}", e));
            self
        }
        pub fn damage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.damage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for damage: {}", e));
            self
        }
        pub fn dot<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.dot = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dot: {}", e));
            self
        }
        pub fn dot_interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.dot_interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dot_interval: {}", e));
            self
        }
        pub fn dot_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.dot_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dot_time: {}", e));
            self
        }
        pub fn emdd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.emdd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emdd: {}", e));
            self
        }
        pub fn emhp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.emhp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emhp: {}", e));
            self
        }
        pub fn emmp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.emmp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emmp: {}", e));
            self
        }
        pub fn epad<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.epad = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for epad: {}", e));
            self
        }
        pub fn epdd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.epdd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for epdd: {}", e));
            self
        }
        pub fn er<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.er = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for er: {}", e));
            self
        }
        pub fn eva<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.eva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eva: {}", e));
            self
        }
        pub fn exp_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.exp_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for exp_r: {}", e));
            self
        }
        pub fn hp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.hp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hp: {}", e));
            self
        }
        pub fn hp_con<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.hp_con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hp_con: {}", e));
            self
        }
        pub fn ignore_mobpdp_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.ignore_mobpdp_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ignore_mobpdp_r: {}", e));
            self
        }
        pub fn item_con<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.item_con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_con: {}", e));
            self
        }
        pub fn item_con_no<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.item_con_no = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_con_no: {}", e));
            self
        }
        pub fn item_consume<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.item_consume = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_consume: {}", e));
            self
        }
        pub fn jump<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.jump = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jump: {}", e));
            self
        }
        pub fn lt<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Vec2>>,
            T::Error: std::fmt::Display,
        {
            self.lt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lt: {}", e));
            self
        }
        pub fn mad<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mad = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mad: {}", e));
            self
        }
        pub fn mad_x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mad_x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mad_x: {}", e));
            self
        }
        pub fn mastery<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mastery = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mastery: {}", e));
            self
        }
        pub fn max_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.max_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_level: {}", e));
            self
        }
        pub fn mdd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mdd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mdd: {}", e));
            self
        }
        pub fn mdd_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mdd_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mdd_r: {}", e));
            self
        }
        pub fn meso_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.meso_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meso_r: {}", e));
            self
        }
        pub fn mhp_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mhp_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mhp_r: {}", e));
            self
        }
        pub fn mmp_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mmp_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mmp_r: {}", e));
            self
        }
        pub fn mob_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mob_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_count: {}", e));
            self
        }
        pub fn money_con<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.money_con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for money_con: {}", e));
            self
        }
        pub fn morph<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.morph = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for morph: {}", e));
            self
        }
        pub fn mp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mp: {}", e));
            self
        }
        pub fn mp_con<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.mp_con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mp_con: {}", e));
            self
        }
        pub fn pad<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.pad = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pad: {}", e));
            self
        }
        pub fn pad_x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.pad_x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pad_x: {}", e));
            self
        }
        pub fn pdd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.pdd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdd: {}", e));
            self
        }
        pub fn pdd_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.pdd_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdd_r: {}", e));
            self
        }
        pub fn prop<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.prop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prop: {}", e));
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.range = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range: {}", e));
            self
        }
        pub fn rb<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Vec2>>,
            T::Error: std::fmt::Display,
        {
            self.rb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rb: {}", e));
            self
        }
        pub fn self_destruction<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.self_destruction = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for self_destruction: {}",
                    e
                )
            });
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed: {}", e));
            self
        }
        pub fn sub_prop<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.sub_prop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_prop: {}", e));
            self
        }
        pub fn sub_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.sub_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_time: {}", e));
            self
        }
        pub fn t<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.t = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for t: {}", e));
            self
        }
        pub fn ter_r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.ter_r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ter_r: {}", e));
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {}", e));
            self
        }
        pub fn u<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.u = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for u: {}", e));
            self
        }
        pub fn v<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.v = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for v: {}", e));
            self
        }
        pub fn w<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.w = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for w: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
        pub fn z<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillExpr>>,
            T::Error: std::fmt::Display,
        {
            self.z = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for z: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillCommonInfo> for super::SkillCommonInfo {
        type Error = String;
        fn try_from(value: SkillCommonInfo) -> Result<Self, String> {
            Ok(Self {
                acc: value.acc?,
                action: value.action?,
                asr_r: value.asr_r?,
                attack_count: value.attack_count?,
                bullet_consume: value.bullet_consume?,
                bullet_count: value.bullet_count?,
                cooltime: value.cooltime?,
                cr: value.cr?,
                criticaldamage_max: value.criticaldamage_max?,
                criticaldamage_min: value.criticaldamage_min?,
                dam_r: value.dam_r?,
                damage: value.damage?,
                dot: value.dot?,
                dot_interval: value.dot_interval?,
                dot_time: value.dot_time?,
                emdd: value.emdd?,
                emhp: value.emhp?,
                emmp: value.emmp?,
                epad: value.epad?,
                epdd: value.epdd?,
                er: value.er?,
                eva: value.eva?,
                exp_r: value.exp_r?,
                hp: value.hp?,
                hp_con: value.hp_con?,
                ignore_mobpdp_r: value.ignore_mobpdp_r?,
                item_con: value.item_con?,
                item_con_no: value.item_con_no?,
                item_consume: value.item_consume?,
                jump: value.jump?,
                lt: value.lt?,
                mad: value.mad?,
                mad_x: value.mad_x?,
                mastery: value.mastery?,
                max_level: value.max_level?,
                mdd: value.mdd?,
                mdd_r: value.mdd_r?,
                meso_r: value.meso_r?,
                mhp_r: value.mhp_r?,
                mmp_r: value.mmp_r?,
                mob_count: value.mob_count?,
                money_con: value.money_con?,
                morph: value.morph?,
                mp: value.mp?,
                mp_con: value.mp_con?,
                pad: value.pad?,
                pad_x: value.pad_x?,
                pdd: value.pdd?,
                pdd_r: value.pdd_r?,
                prop: value.prop?,
                range: value.range?,
                rb: value.rb?,
                self_destruction: value.self_destruction?,
                speed: value.speed?,
                sub_prop: value.sub_prop?,
                sub_time: value.sub_time?,
                t: value.t?,
                ter_r: value.ter_r?,
                time: value.time?,
                u: value.u?,
                v: value.v?,
                w: value.w?,
                x: value.x?,
                y: value.y?,
                z: value.z?,
            })
        }
    }
    impl From<super::SkillCommonInfo> for SkillCommonInfo {
        fn from(value: super::SkillCommonInfo) -> Self {
            Self {
                acc: Ok(value.acc),
                action: Ok(value.action),
                asr_r: Ok(value.asr_r),
                attack_count: Ok(value.attack_count),
                bullet_consume: Ok(value.bullet_consume),
                bullet_count: Ok(value.bullet_count),
                cooltime: Ok(value.cooltime),
                cr: Ok(value.cr),
                criticaldamage_max: Ok(value.criticaldamage_max),
                criticaldamage_min: Ok(value.criticaldamage_min),
                dam_r: Ok(value.dam_r),
                damage: Ok(value.damage),
                dot: Ok(value.dot),
                dot_interval: Ok(value.dot_interval),
                dot_time: Ok(value.dot_time),
                emdd: Ok(value.emdd),
                emhp: Ok(value.emhp),
                emmp: Ok(value.emmp),
                epad: Ok(value.epad),
                epdd: Ok(value.epdd),
                er: Ok(value.er),
                eva: Ok(value.eva),
                exp_r: Ok(value.exp_r),
                hp: Ok(value.hp),
                hp_con: Ok(value.hp_con),
                ignore_mobpdp_r: Ok(value.ignore_mobpdp_r),
                item_con: Ok(value.item_con),
                item_con_no: Ok(value.item_con_no),
                item_consume: Ok(value.item_consume),
                jump: Ok(value.jump),
                lt: Ok(value.lt),
                mad: Ok(value.mad),
                mad_x: Ok(value.mad_x),
                mastery: Ok(value.mastery),
                max_level: Ok(value.max_level),
                mdd: Ok(value.mdd),
                mdd_r: Ok(value.mdd_r),
                meso_r: Ok(value.meso_r),
                mhp_r: Ok(value.mhp_r),
                mmp_r: Ok(value.mmp_r),
                mob_count: Ok(value.mob_count),
                money_con: Ok(value.money_con),
                morph: Ok(value.morph),
                mp: Ok(value.mp),
                mp_con: Ok(value.mp_con),
                pad: Ok(value.pad),
                pad_x: Ok(value.pad_x),
                pdd: Ok(value.pdd),
                pdd_r: Ok(value.pdd_r),
                prop: Ok(value.prop),
                range: Ok(value.range),
                rb: Ok(value.rb),
                self_destruction: Ok(value.self_destruction),
                speed: Ok(value.speed),
                sub_prop: Ok(value.sub_prop),
                sub_time: Ok(value.sub_time),
                t: Ok(value.t),
                ter_r: Ok(value.ter_r),
                time: Ok(value.time),
                u: Ok(value.u),
                v: Ok(value.v),
                w: Ok(value.w),
                x: Ok(value.x),
                y: Ok(value.y),
                z: Ok(value.z),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillInfo {
        icon: Result<Option<super::Canvas>, String>,
    }
    impl Default for SkillInfo {
        fn default() -> Self {
            Self {
                icon: Ok(Default::default()),
            }
        }
    }
    impl SkillInfo {
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillInfo> for super::SkillInfo {
        type Error = String;
        fn try_from(value: SkillInfo) -> Result<Self, String> {
            Ok(Self { icon: value.icon? })
        }
    }
    impl From<super::SkillInfo> for SkillInfo {
        fn from(value: super::SkillInfo) -> Self {
            Self {
                icon: Ok(value.icon),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValue {
        action: Result<Option<super::SkillSkillValueAction>, String>,
        affected: Result<serde_json::Map<String, serde_json::Value>, String>,
        afterimage: Result<serde_json::Map<String, serde_json::Value>, String>,
        back: Result<serde_json::Map<String, serde_json::Value>, String>,
        back_effect: Result<serde_json::Map<String, serde_json::Value>, String>,
        back_effect0: Result<serde_json::Map<String, serde_json::Value>, String>,
        back_finish: Result<serde_json::Map<String, serde_json::Value>, String>,
        ball: Result<serde_json::Map<String, serde_json::Value>, String>,
        ball0: Result<serde_json::Map<String, serde_json::Value>, String>,
        c_door: Result<serde_json::Map<String, serde_json::Value>, String>,
        char_level: Result<serde_json::Map<String, serde_json::Value>, String>,
        combat_orders: Result<Option<super::StrOrNum>, String>,
        common: Result<Option<super::SkillCommonInfo>, String>,
        damage: Result<serde_json::Map<String, serde_json::Value>, String>,
        disable: Result<Option<super::Bool>, String>,
        e_door: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect0: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect1: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect2: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect3: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect_ship: Result<serde_json::Map<String, serde_json::Value>, String>,
        elem_attr: Result<Option<String>, String>,
        final_attack: Result<
            std::collections::HashMap<String, std::collections::HashMap<String, i64>>,
            String,
        >,
        finish: Result<serde_json::Map<String, serde_json::Value>, String>,
        finish0: Result<serde_json::Map<String, serde_json::Value>, String>,
        flip_ball: Result<Option<serde_json::Value>, String>,
        frame: Result<serde_json::Map<String, serde_json::Value>, String>,
        hit: Result<serde_json::Map<String, serde_json::Value>, String>,
        hit0: Result<serde_json::Map<String, serde_json::Value>, String>,
        hit1: Result<serde_json::Map<String, serde_json::Value>, String>,
        icon: Result<Option<super::Canvas>, String>,
        icon1: Result<Option<super::Canvas>, String>,
        icon2: Result<Option<super::Canvas>, String>,
        icon3: Result<Option<super::Canvas>, String>,
        icon4: Result<Option<super::Canvas>, String>,
        icon5: Result<Option<super::Canvas>, String>,
        icon_disabled: Result<Option<super::Canvas>, String>,
        icon_mouse_over: Result<Option<super::Canvas>, String>,
        info: Result<Option<String>, String>,
        invisible: Result<Option<super::Bool>, String>,
        keydown: Result<serde_json::Map<String, serde_json::Value>, String>,
        keydown0: Result<serde_json::Map<String, serde_json::Value>, String>,
        keydownend: Result<serde_json::Map<String, serde_json::Value>, String>,
        level: Result<std::collections::HashMap<String, super::SkillSkillValueLevelValue>, String>,
        m_door: Result<serde_json::Map<String, serde_json::Value>, String>,
        master_level: Result<Option<super::StrOrNum>, String>,
        mob: Result<serde_json::Map<String, serde_json::Value>, String>,
        mob0: Result<serde_json::Map<String, serde_json::Value>, String>,
        mob_code: Result<Option<super::StrOrNum>, String>,
        o_door: Result<serde_json::Map<String, serde_json::Value>, String>,
        prepare: Result<serde_json::Map<String, serde_json::Value>, String>,
        psd: Result<Option<super::Bool>, String>,
        psd_skill: Result<
            std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
            String,
        >,
        repeat: Result<serde_json::Map<String, serde_json::Value>, String>,
        req: Result<std::collections::HashMap<String, super::StrOrNum>, String>,
        s_door: Result<serde_json::Map<String, serde_json::Value>, String>,
        screen: Result<serde_json::Map<String, serde_json::Value>, String>,
        skill_type: Result<Option<i64>, String>,
        special: Result<serde_json::Map<String, serde_json::Value>, String>,
        special0: Result<serde_json::Map<String, serde_json::Value>, String>,
        special_action: Result<std::collections::HashMap<String, String>, String>,
        special_action_frame: Result<Option<super::SkillSkillValueSpecialActionFrame>, String>,
        special_affected: Result<serde_json::Map<String, serde_json::Value>, String>,
        state: Result<serde_json::Map<String, serde_json::Value>, String>,
        stop_effect: Result<serde_json::Map<String, serde_json::Value>, String>,
        sub_weapon: Result<Option<super::StrOrNum>, String>,
        summon: Result<Option<super::SkillSkillValueSummon>, String>,
        tile: Result<serde_json::Map<String, serde_json::Value>, String>,
        time_limited: Result<Option<super::Bool>, String>,
        weapon: Result<Option<super::StrOrNum>, String>,
    }
    impl Default for SkillSkillValue {
        fn default() -> Self {
            Self {
                action: Ok(Default::default()),
                affected: Ok(Default::default()),
                afterimage: Ok(Default::default()),
                back: Ok(Default::default()),
                back_effect: Ok(Default::default()),
                back_effect0: Ok(Default::default()),
                back_finish: Ok(Default::default()),
                ball: Ok(Default::default()),
                ball0: Ok(Default::default()),
                c_door: Ok(Default::default()),
                char_level: Ok(Default::default()),
                combat_orders: Ok(Default::default()),
                common: Ok(Default::default()),
                damage: Ok(Default::default()),
                disable: Ok(Default::default()),
                e_door: Ok(Default::default()),
                effect: Ok(Default::default()),
                effect0: Ok(Default::default()),
                effect1: Ok(Default::default()),
                effect2: Ok(Default::default()),
                effect3: Ok(Default::default()),
                effect_ship: Ok(Default::default()),
                elem_attr: Ok(Default::default()),
                final_attack: Ok(Default::default()),
                finish: Ok(Default::default()),
                finish0: Ok(Default::default()),
                flip_ball: Ok(Default::default()),
                frame: Ok(Default::default()),
                hit: Ok(Default::default()),
                hit0: Ok(Default::default()),
                hit1: Ok(Default::default()),
                icon: Ok(Default::default()),
                icon1: Ok(Default::default()),
                icon2: Ok(Default::default()),
                icon3: Ok(Default::default()),
                icon4: Ok(Default::default()),
                icon5: Ok(Default::default()),
                icon_disabled: Ok(Default::default()),
                icon_mouse_over: Ok(Default::default()),
                info: Ok(Default::default()),
                invisible: Ok(Default::default()),
                keydown: Ok(Default::default()),
                keydown0: Ok(Default::default()),
                keydownend: Ok(Default::default()),
                level: Ok(Default::default()),
                m_door: Ok(Default::default()),
                master_level: Ok(Default::default()),
                mob: Ok(Default::default()),
                mob0: Ok(Default::default()),
                mob_code: Ok(Default::default()),
                o_door: Ok(Default::default()),
                prepare: Ok(Default::default()),
                psd: Ok(Default::default()),
                psd_skill: Ok(Default::default()),
                repeat: Ok(Default::default()),
                req: Ok(Default::default()),
                s_door: Ok(Default::default()),
                screen: Ok(Default::default()),
                skill_type: Ok(Default::default()),
                special: Ok(Default::default()),
                special0: Ok(Default::default()),
                special_action: Ok(Default::default()),
                special_action_frame: Ok(Default::default()),
                special_affected: Ok(Default::default()),
                state: Ok(Default::default()),
                stop_effect: Ok(Default::default()),
                sub_weapon: Ok(Default::default()),
                summon: Ok(Default::default()),
                tile: Ok(Default::default()),
                time_limited: Ok(Default::default()),
                weapon: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValue {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillSkillValueAction>>,
            T::Error: std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {}", e));
            self
        }
        pub fn affected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.affected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for affected: {}", e));
            self
        }
        pub fn afterimage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.afterimage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for afterimage: {}", e));
            self
        }
        pub fn back<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.back = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for back: {}", e));
            self
        }
        pub fn back_effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.back_effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for back_effect: {}", e));
            self
        }
        pub fn back_effect0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.back_effect0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for back_effect0: {}", e));
            self
        }
        pub fn back_finish<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.back_finish = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for back_finish: {}", e));
            self
        }
        pub fn ball<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.ball = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ball: {}", e));
            self
        }
        pub fn ball0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.ball0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ball0: {}", e));
            self
        }
        pub fn c_door<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.c_door = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for c_door: {}", e));
            self
        }
        pub fn char_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.char_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for char_level: {}", e));
            self
        }
        pub fn combat_orders<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.combat_orders = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for combat_orders: {}", e));
            self
        }
        pub fn common<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillCommonInfo>>,
            T::Error: std::fmt::Display,
        {
            self.common = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for common: {}", e));
            self
        }
        pub fn damage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.damage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for damage: {}", e));
            self
        }
        pub fn disable<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.disable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for disable: {}", e));
            self
        }
        pub fn e_door<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.e_door = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for e_door: {}", e));
            self
        }
        pub fn effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect: {}", e));
            self
        }
        pub fn effect0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect0: {}", e));
            self
        }
        pub fn effect1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect1: {}", e));
            self
        }
        pub fn effect2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect2: {}", e));
            self
        }
        pub fn effect3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect3: {}", e));
            self
        }
        pub fn effect_ship<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect_ship = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_ship: {}", e));
            self
        }
        pub fn elem_attr<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.elem_attr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for elem_attr: {}", e));
            self
        }
        pub fn final_attack<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, std::collections::HashMap<String, i64>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.final_attack = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for final_attack: {}", e));
            self
        }
        pub fn finish<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.finish = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finish: {}", e));
            self
        }
        pub fn finish0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.finish0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for finish0: {}", e));
            self
        }
        pub fn flip_ball<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.flip_ball = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flip_ball: {}", e));
            self
        }
        pub fn frame<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.frame = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for frame: {}", e));
            self
        }
        pub fn hit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.hit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit: {}", e));
            self
        }
        pub fn hit0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.hit0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit0: {}", e));
            self
        }
        pub fn hit1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.hit1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit1: {}", e));
            self
        }
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {}", e));
            self
        }
        pub fn icon1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon1: {}", e));
            self
        }
        pub fn icon2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon2: {}", e));
            self
        }
        pub fn icon3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon3: {}", e));
            self
        }
        pub fn icon4<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon4: {}", e));
            self
        }
        pub fn icon5<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon5: {}", e));
            self
        }
        pub fn icon_disabled<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon_disabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_disabled: {}", e));
            self
        }
        pub fn icon_mouse_over<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Canvas>>,
            T::Error: std::fmt::Display,
        {
            self.icon_mouse_over = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_mouse_over: {}", e));
            self
        }
        pub fn info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info: {}", e));
            self
        }
        pub fn invisible<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.invisible = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for invisible: {}", e));
            self
        }
        pub fn keydown<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.keydown = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keydown: {}", e));
            self
        }
        pub fn keydown0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.keydown0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keydown0: {}", e));
            self
        }
        pub fn keydownend<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.keydownend = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keydownend: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, super::SkillSkillValueLevelValue>,
            >,
            T::Error: std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {}", e));
            self
        }
        pub fn m_door<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.m_door = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for m_door: {}", e));
            self
        }
        pub fn master_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.master_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for master_level: {}", e));
            self
        }
        pub fn mob<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.mob = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob: {}", e));
            self
        }
        pub fn mob0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.mob0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob0: {}", e));
            self
        }
        pub fn mob_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.mob_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_code: {}", e));
            self
        }
        pub fn o_door<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.o_door = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for o_door: {}", e));
            self
        }
        pub fn prepare<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.prepare = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prepare: {}", e));
            self
        }
        pub fn psd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.psd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for psd: {}", e));
            self
        }
        pub fn psd_skill<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<
                std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
            >,
            T::Error: std::fmt::Display,
        {
            self.psd_skill = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for psd_skill: {}", e));
            self
        }
        pub fn repeat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.repeat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat: {}", e));
            self
        }
        pub fn req<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.req = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for req: {}", e));
            self
        }
        pub fn s_door<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.s_door = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for s_door: {}", e));
            self
        }
        pub fn screen<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.screen = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for screen: {}", e));
            self
        }
        pub fn skill_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.skill_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill_type: {}", e));
            self
        }
        pub fn special<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.special = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for special: {}", e));
            self
        }
        pub fn special0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.special0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for special0: {}", e));
            self
        }
        pub fn special_action<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, String>>,
            T::Error: std::fmt::Display,
        {
            self.special_action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for special_action: {}", e));
            self
        }
        pub fn special_action_frame<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillSkillValueSpecialActionFrame>>,
            T::Error: std::fmt::Display,
        {
            self.special_action_frame = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for special_action_frame: {}",
                    e
                )
            });
            self
        }
        pub fn special_affected<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.special_affected = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for special_affected: {}",
                    e
                )
            });
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {}", e));
            self
        }
        pub fn stop_effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.stop_effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop_effect: {}", e));
            self
        }
        pub fn sub_weapon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.sub_weapon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sub_weapon: {}", e));
            self
        }
        pub fn summon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillSkillValueSummon>>,
            T::Error: std::fmt::Display,
        {
            self.summon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summon: {}", e));
            self
        }
        pub fn tile<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.tile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tile: {}", e));
            self
        }
        pub fn time_limited<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Bool>>,
            T::Error: std::fmt::Display,
        {
            self.time_limited = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time_limited: {}", e));
            self
        }
        pub fn weapon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.weapon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weapon: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValue> for super::SkillSkillValue {
        type Error = String;
        fn try_from(value: SkillSkillValue) -> Result<Self, String> {
            Ok(Self {
                action: value.action?,
                affected: value.affected?,
                afterimage: value.afterimage?,
                back: value.back?,
                back_effect: value.back_effect?,
                back_effect0: value.back_effect0?,
                back_finish: value.back_finish?,
                ball: value.ball?,
                ball0: value.ball0?,
                c_door: value.c_door?,
                char_level: value.char_level?,
                combat_orders: value.combat_orders?,
                common: value.common?,
                damage: value.damage?,
                disable: value.disable?,
                e_door: value.e_door?,
                effect: value.effect?,
                effect0: value.effect0?,
                effect1: value.effect1?,
                effect2: value.effect2?,
                effect3: value.effect3?,
                effect_ship: value.effect_ship?,
                elem_attr: value.elem_attr?,
                final_attack: value.final_attack?,
                finish: value.finish?,
                finish0: value.finish0?,
                flip_ball: value.flip_ball?,
                frame: value.frame?,
                hit: value.hit?,
                hit0: value.hit0?,
                hit1: value.hit1?,
                icon: value.icon?,
                icon1: value.icon1?,
                icon2: value.icon2?,
                icon3: value.icon3?,
                icon4: value.icon4?,
                icon5: value.icon5?,
                icon_disabled: value.icon_disabled?,
                icon_mouse_over: value.icon_mouse_over?,
                info: value.info?,
                invisible: value.invisible?,
                keydown: value.keydown?,
                keydown0: value.keydown0?,
                keydownend: value.keydownend?,
                level: value.level?,
                m_door: value.m_door?,
                master_level: value.master_level?,
                mob: value.mob?,
                mob0: value.mob0?,
                mob_code: value.mob_code?,
                o_door: value.o_door?,
                prepare: value.prepare?,
                psd: value.psd?,
                psd_skill: value.psd_skill?,
                repeat: value.repeat?,
                req: value.req?,
                s_door: value.s_door?,
                screen: value.screen?,
                skill_type: value.skill_type?,
                special: value.special?,
                special0: value.special0?,
                special_action: value.special_action?,
                special_action_frame: value.special_action_frame?,
                special_affected: value.special_affected?,
                state: value.state?,
                stop_effect: value.stop_effect?,
                sub_weapon: value.sub_weapon?,
                summon: value.summon?,
                tile: value.tile?,
                time_limited: value.time_limited?,
                weapon: value.weapon?,
            })
        }
    }
    impl From<super::SkillSkillValue> for SkillSkillValue {
        fn from(value: super::SkillSkillValue) -> Self {
            Self {
                action: Ok(value.action),
                affected: Ok(value.affected),
                afterimage: Ok(value.afterimage),
                back: Ok(value.back),
                back_effect: Ok(value.back_effect),
                back_effect0: Ok(value.back_effect0),
                back_finish: Ok(value.back_finish),
                ball: Ok(value.ball),
                ball0: Ok(value.ball0),
                c_door: Ok(value.c_door),
                char_level: Ok(value.char_level),
                combat_orders: Ok(value.combat_orders),
                common: Ok(value.common),
                damage: Ok(value.damage),
                disable: Ok(value.disable),
                e_door: Ok(value.e_door),
                effect: Ok(value.effect),
                effect0: Ok(value.effect0),
                effect1: Ok(value.effect1),
                effect2: Ok(value.effect2),
                effect3: Ok(value.effect3),
                effect_ship: Ok(value.effect_ship),
                elem_attr: Ok(value.elem_attr),
                final_attack: Ok(value.final_attack),
                finish: Ok(value.finish),
                finish0: Ok(value.finish0),
                flip_ball: Ok(value.flip_ball),
                frame: Ok(value.frame),
                hit: Ok(value.hit),
                hit0: Ok(value.hit0),
                hit1: Ok(value.hit1),
                icon: Ok(value.icon),
                icon1: Ok(value.icon1),
                icon2: Ok(value.icon2),
                icon3: Ok(value.icon3),
                icon4: Ok(value.icon4),
                icon5: Ok(value.icon5),
                icon_disabled: Ok(value.icon_disabled),
                icon_mouse_over: Ok(value.icon_mouse_over),
                info: Ok(value.info),
                invisible: Ok(value.invisible),
                keydown: Ok(value.keydown),
                keydown0: Ok(value.keydown0),
                keydownend: Ok(value.keydownend),
                level: Ok(value.level),
                m_door: Ok(value.m_door),
                master_level: Ok(value.master_level),
                mob: Ok(value.mob),
                mob0: Ok(value.mob0),
                mob_code: Ok(value.mob_code),
                o_door: Ok(value.o_door),
                prepare: Ok(value.prepare),
                psd: Ok(value.psd),
                psd_skill: Ok(value.psd_skill),
                repeat: Ok(value.repeat),
                req: Ok(value.req),
                s_door: Ok(value.s_door),
                screen: Ok(value.screen),
                skill_type: Ok(value.skill_type),
                special: Ok(value.special),
                special0: Ok(value.special0),
                special_action: Ok(value.special_action),
                special_action_frame: Ok(value.special_action_frame),
                special_affected: Ok(value.special_affected),
                state: Ok(value.state),
                stop_effect: Ok(value.stop_effect),
                sub_weapon: Ok(value.sub_weapon),
                summon: Ok(value.summon),
                tile: Ok(value.tile),
                time_limited: Ok(value.time_limited),
                weapon: Ok(value.weapon),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValueLevelValue {
        acc: Result<Option<i64>, String>,
        attack_count: Result<Option<i64>, String>,
        ball: Result<serde_json::Map<String, serde_json::Value>, String>,
        cooltime: Result<Option<i64>, String>,
        criticaldamage_max: Result<Option<i64>, String>,
        damage: Result<Option<super::StrOrNum>, String>,
        damagepc: Result<Option<i64>, String>,
        date_expire: Result<Option<String>, String>,
        dot: Result<Option<super::StrOrNum>, String>,
        dot_interval: Result<Option<super::StrOrNum>, String>,
        dot_time: Result<Option<super::StrOrNum>, String>,
        eva: Result<Option<i64>, String>,
        fixdamage: Result<Option<super::StrOrNum>, String>,
        hit: Result<serde_json::Map<String, serde_json::Value>, String>,
        hp_con: Result<Option<i64>, String>,
        hs: Result<Option<String>, String>,
        item_con: Result<Option<i64>, String>,
        item_con_no: Result<Option<i64>, String>,
        jump: Result<Option<i64>, String>,
        lt: Result<Option<super::Vec2>, String>,
        mad: Result<Option<i64>, String>,
        mastery: Result<Option<i64>, String>,
        mdd: Result<Option<i64>, String>,
        mob_count: Result<Option<i64>, String>,
        mp_con: Result<Option<i64>, String>,
        pad: Result<Option<i64>, String>,
        pdd: Result<Option<i64>, String>,
        prop: Result<Option<i64>, String>,
        range: Result<Option<i64>, String>,
        rb: Result<Option<super::Vec2>, String>,
        speed: Result<Option<i64>, String>,
        time: Result<Option<i64>, String>,
        x: Result<Option<i64>, String>,
        y: Result<Option<i64>, String>,
        z: Result<Option<i64>, String>,
    }
    impl Default for SkillSkillValueLevelValue {
        fn default() -> Self {
            Self {
                acc: Ok(Default::default()),
                attack_count: Ok(Default::default()),
                ball: Ok(Default::default()),
                cooltime: Ok(Default::default()),
                criticaldamage_max: Ok(Default::default()),
                damage: Ok(Default::default()),
                damagepc: Ok(Default::default()),
                date_expire: Ok(Default::default()),
                dot: Ok(Default::default()),
                dot_interval: Ok(Default::default()),
                dot_time: Ok(Default::default()),
                eva: Ok(Default::default()),
                fixdamage: Ok(Default::default()),
                hit: Ok(Default::default()),
                hp_con: Ok(Default::default()),
                hs: Ok(Default::default()),
                item_con: Ok(Default::default()),
                item_con_no: Ok(Default::default()),
                jump: Ok(Default::default()),
                lt: Ok(Default::default()),
                mad: Ok(Default::default()),
                mastery: Ok(Default::default()),
                mdd: Ok(Default::default()),
                mob_count: Ok(Default::default()),
                mp_con: Ok(Default::default()),
                pad: Ok(Default::default()),
                pdd: Ok(Default::default()),
                prop: Ok(Default::default()),
                range: Ok(Default::default()),
                rb: Ok(Default::default()),
                speed: Ok(Default::default()),
                time: Ok(Default::default()),
                x: Ok(Default::default()),
                y: Ok(Default::default()),
                z: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValueLevelValue {
        pub fn acc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.acc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for acc: {}", e));
            self
        }
        pub fn attack_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.attack_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack_count: {}", e));
            self
        }
        pub fn ball<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.ball = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ball: {}", e));
            self
        }
        pub fn cooltime<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.cooltime = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cooltime: {}", e));
            self
        }
        pub fn criticaldamage_max<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.criticaldamage_max = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for criticaldamage_max: {}",
                    e
                )
            });
            self
        }
        pub fn damage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.damage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for damage: {}", e));
            self
        }
        pub fn damagepc<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.damagepc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for damagepc: {}", e));
            self
        }
        pub fn date_expire<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.date_expire = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for date_expire: {}", e));
            self
        }
        pub fn dot<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.dot = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dot: {}", e));
            self
        }
        pub fn dot_interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.dot_interval = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dot_interval: {}", e));
            self
        }
        pub fn dot_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.dot_time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dot_time: {}", e));
            self
        }
        pub fn eva<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.eva = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eva: {}", e));
            self
        }
        pub fn fixdamage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.fixdamage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fixdamage: {}", e));
            self
        }
        pub fn hit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.hit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit: {}", e));
            self
        }
        pub fn hp_con<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.hp_con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hp_con: {}", e));
            self
        }
        pub fn hs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.hs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hs: {}", e));
            self
        }
        pub fn item_con<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.item_con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_con: {}", e));
            self
        }
        pub fn item_con_no<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.item_con_no = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for item_con_no: {}", e));
            self
        }
        pub fn jump<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.jump = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jump: {}", e));
            self
        }
        pub fn lt<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Vec2>>,
            T::Error: std::fmt::Display,
        {
            self.lt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lt: {}", e));
            self
        }
        pub fn mad<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mad = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mad: {}", e));
            self
        }
        pub fn mastery<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mastery = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mastery: {}", e));
            self
        }
        pub fn mdd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mdd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mdd: {}", e));
            self
        }
        pub fn mob_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_count: {}", e));
            self
        }
        pub fn mp_con<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mp_con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mp_con: {}", e));
            self
        }
        pub fn pad<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.pad = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pad: {}", e));
            self
        }
        pub fn pdd<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.pdd = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdd: {}", e));
            self
        }
        pub fn prop<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.prop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prop: {}", e));
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.range = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range: {}", e));
            self
        }
        pub fn rb<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Vec2>>,
            T::Error: std::fmt::Display,
        {
            self.rb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rb: {}", e));
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for speed: {}", e));
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
        pub fn z<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.z = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for z: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValueLevelValue> for super::SkillSkillValueLevelValue {
        type Error = String;
        fn try_from(value: SkillSkillValueLevelValue) -> Result<Self, String> {
            Ok(Self {
                acc: value.acc?,
                attack_count: value.attack_count?,
                ball: value.ball?,
                cooltime: value.cooltime?,
                criticaldamage_max: value.criticaldamage_max?,
                damage: value.damage?,
                damagepc: value.damagepc?,
                date_expire: value.date_expire?,
                dot: value.dot?,
                dot_interval: value.dot_interval?,
                dot_time: value.dot_time?,
                eva: value.eva?,
                fixdamage: value.fixdamage?,
                hit: value.hit?,
                hp_con: value.hp_con?,
                hs: value.hs?,
                item_con: value.item_con?,
                item_con_no: value.item_con_no?,
                jump: value.jump?,
                lt: value.lt?,
                mad: value.mad?,
                mastery: value.mastery?,
                mdd: value.mdd?,
                mob_count: value.mob_count?,
                mp_con: value.mp_con?,
                pad: value.pad?,
                pdd: value.pdd?,
                prop: value.prop?,
                range: value.range?,
                rb: value.rb?,
                speed: value.speed?,
                time: value.time?,
                x: value.x?,
                y: value.y?,
                z: value.z?,
            })
        }
    }
    impl From<super::SkillSkillValueLevelValue> for SkillSkillValueLevelValue {
        fn from(value: super::SkillSkillValueLevelValue) -> Self {
            Self {
                acc: Ok(value.acc),
                attack_count: Ok(value.attack_count),
                ball: Ok(value.ball),
                cooltime: Ok(value.cooltime),
                criticaldamage_max: Ok(value.criticaldamage_max),
                damage: Ok(value.damage),
                damagepc: Ok(value.damagepc),
                date_expire: Ok(value.date_expire),
                dot: Ok(value.dot),
                dot_interval: Ok(value.dot_interval),
                dot_time: Ok(value.dot_time),
                eva: Ok(value.eva),
                fixdamage: Ok(value.fixdamage),
                hit: Ok(value.hit),
                hp_con: Ok(value.hp_con),
                hs: Ok(value.hs),
                item_con: Ok(value.item_con),
                item_con_no: Ok(value.item_con_no),
                jump: Ok(value.jump),
                lt: Ok(value.lt),
                mad: Ok(value.mad),
                mastery: Ok(value.mastery),
                mdd: Ok(value.mdd),
                mob_count: Ok(value.mob_count),
                mp_con: Ok(value.mp_con),
                pad: Ok(value.pad),
                pdd: Ok(value.pdd),
                prop: Ok(value.prop),
                range: Ok(value.range),
                rb: Ok(value.rb),
                speed: Ok(value.speed),
                time: Ok(value.time),
                x: Ok(value.x),
                y: Ok(value.y),
                z: Ok(value.z),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValueSpecialActionFrame {
        delay: Result<Option<i64>, String>,
    }
    impl Default for SkillSkillValueSpecialActionFrame {
        fn default() -> Self {
            Self {
                delay: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValueSpecialActionFrame {
        pub fn delay<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.delay = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delay: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValueSpecialActionFrame>
        for super::SkillSkillValueSpecialActionFrame
    {
        type Error = String;
        fn try_from(value: SkillSkillValueSpecialActionFrame) -> Result<Self, String> {
            Ok(Self {
                delay: value.delay?,
            })
        }
    }
    impl From<super::SkillSkillValueSpecialActionFrame> for SkillSkillValueSpecialActionFrame {
        fn from(value: super::SkillSkillValueSpecialActionFrame) -> Self {
            Self {
                delay: Ok(value.delay),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValueSummon {
        attack1: Result<Option<super::SkillSkillValueSummonAttack1>, String>,
        attack2: Result<serde_json::Map<String, serde_json::Value>, String>,
        attack_triangle: Result<serde_json::Map<String, serde_json::Value>, String>,
        die: Result<Option<super::SkillSkillValueSummonDie>, String>,
        die1: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect0: Result<serde_json::Map<String, serde_json::Value>, String>,
        fly: Result<serde_json::Map<String, serde_json::Value>, String>,
        heal: Result<serde_json::Map<String, serde_json::Value>, String>,
        height: Result<Option<i64>, String>,
        hit: Result<serde_json::Map<String, serde_json::Value>, String>,
        move_: Result<serde_json::Map<String, serde_json::Value>, String>,
        prepare: Result<serde_json::Map<String, serde_json::Value>, String>,
        repeat: Result<serde_json::Map<String, serde_json::Value>, String>,
        repeat0: Result<serde_json::Map<String, serde_json::Value>, String>,
        say: Result<serde_json::Map<String, serde_json::Value>, String>,
        skill1: Result<serde_json::Map<String, serde_json::Value>, String>,
        skill2: Result<serde_json::Map<String, serde_json::Value>, String>,
        skill3: Result<serde_json::Map<String, serde_json::Value>, String>,
        skill4: Result<serde_json::Map<String, serde_json::Value>, String>,
        skill5: Result<serde_json::Map<String, serde_json::Value>, String>,
        skill6: Result<serde_json::Map<String, serde_json::Value>, String>,
        stand: Result<serde_json::Map<String, serde_json::Value>, String>,
        subsummon: Result<serde_json::Map<String, serde_json::Value>, String>,
        summoned: Result<serde_json::Map<String, serde_json::Value>, String>,
    }
    impl Default for SkillSkillValueSummon {
        fn default() -> Self {
            Self {
                attack1: Ok(Default::default()),
                attack2: Ok(Default::default()),
                attack_triangle: Ok(Default::default()),
                die: Ok(Default::default()),
                die1: Ok(Default::default()),
                effect: Ok(Default::default()),
                effect0: Ok(Default::default()),
                fly: Ok(Default::default()),
                heal: Ok(Default::default()),
                height: Ok(Default::default()),
                hit: Ok(Default::default()),
                move_: Ok(Default::default()),
                prepare: Ok(Default::default()),
                repeat: Ok(Default::default()),
                repeat0: Ok(Default::default()),
                say: Ok(Default::default()),
                skill1: Ok(Default::default()),
                skill2: Ok(Default::default()),
                skill3: Ok(Default::default()),
                skill4: Ok(Default::default()),
                skill5: Ok(Default::default()),
                skill6: Ok(Default::default()),
                stand: Ok(Default::default()),
                subsummon: Ok(Default::default()),
                summoned: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValueSummon {
        pub fn attack1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillSkillValueSummonAttack1>>,
            T::Error: std::fmt::Display,
        {
            self.attack1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack1: {}", e));
            self
        }
        pub fn attack2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.attack2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack2: {}", e));
            self
        }
        pub fn attack_triangle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.attack_triangle = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack_triangle: {}", e));
            self
        }
        pub fn die<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillSkillValueSummonDie>>,
            T::Error: std::fmt::Display,
        {
            self.die = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for die: {}", e));
            self
        }
        pub fn die1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.die1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for die1: {}", e));
            self
        }
        pub fn effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect: {}", e));
            self
        }
        pub fn effect0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect0: {}", e));
            self
        }
        pub fn fly<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.fly = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fly: {}", e));
            self
        }
        pub fn heal<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.heal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for heal: {}", e));
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for height: {}", e));
            self
        }
        pub fn hit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.hit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit: {}", e));
            self
        }
        pub fn move_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.move_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for move_: {}", e));
            self
        }
        pub fn prepare<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.prepare = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prepare: {}", e));
            self
        }
        pub fn repeat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.repeat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat: {}", e));
            self
        }
        pub fn repeat0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.repeat0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repeat0: {}", e));
            self
        }
        pub fn say<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.say = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for say: {}", e));
            self
        }
        pub fn skill1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.skill1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill1: {}", e));
            self
        }
        pub fn skill2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.skill2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill2: {}", e));
            self
        }
        pub fn skill3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.skill3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill3: {}", e));
            self
        }
        pub fn skill4<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.skill4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill4: {}", e));
            self
        }
        pub fn skill5<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.skill5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill5: {}", e));
            self
        }
        pub fn skill6<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.skill6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skill6: {}", e));
            self
        }
        pub fn stand<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.stand = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stand: {}", e));
            self
        }
        pub fn subsummon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.subsummon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subsummon: {}", e));
            self
        }
        pub fn summoned<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.summoned = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summoned: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValueSummon> for super::SkillSkillValueSummon {
        type Error = String;
        fn try_from(value: SkillSkillValueSummon) -> Result<Self, String> {
            Ok(Self {
                attack1: value.attack1?,
                attack2: value.attack2?,
                attack_triangle: value.attack_triangle?,
                die: value.die?,
                die1: value.die1?,
                effect: value.effect?,
                effect0: value.effect0?,
                fly: value.fly?,
                heal: value.heal?,
                height: value.height?,
                hit: value.hit?,
                move_: value.move_?,
                prepare: value.prepare?,
                repeat: value.repeat?,
                repeat0: value.repeat0?,
                say: value.say?,
                skill1: value.skill1?,
                skill2: value.skill2?,
                skill3: value.skill3?,
                skill4: value.skill4?,
                skill5: value.skill5?,
                skill6: value.skill6?,
                stand: value.stand?,
                subsummon: value.subsummon?,
                summoned: value.summoned?,
            })
        }
    }
    impl From<super::SkillSkillValueSummon> for SkillSkillValueSummon {
        fn from(value: super::SkillSkillValueSummon) -> Self {
            Self {
                attack1: Ok(value.attack1),
                attack2: Ok(value.attack2),
                attack_triangle: Ok(value.attack_triangle),
                die: Ok(value.die),
                die1: Ok(value.die1),
                effect: Ok(value.effect),
                effect0: Ok(value.effect0),
                fly: Ok(value.fly),
                heal: Ok(value.heal),
                height: Ok(value.height),
                hit: Ok(value.hit),
                move_: Ok(value.move_),
                prepare: Ok(value.prepare),
                repeat: Ok(value.repeat),
                repeat0: Ok(value.repeat0),
                say: Ok(value.say),
                skill1: Ok(value.skill1),
                skill2: Ok(value.skill2),
                skill3: Ok(value.skill3),
                skill4: Ok(value.skill4),
                skill5: Ok(value.skill5),
                skill6: Ok(value.skill6),
                stand: Ok(value.stand),
                subsummon: Ok(value.subsummon),
                summoned: Ok(value.summoned),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValueSummonAttack1 {
        info: Result<Option<super::SkillSkillValueSummonAttack1Info>, String>,
    }
    impl Default for SkillSkillValueSummonAttack1 {
        fn default() -> Self {
            Self {
                info: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValueSummonAttack1 {
        pub fn info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillSkillValueSummonAttack1Info>>,
            T::Error: std::fmt::Display,
        {
            self.info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValueSummonAttack1> for super::SkillSkillValueSummonAttack1 {
        type Error = String;
        fn try_from(value: SkillSkillValueSummonAttack1) -> Result<Self, String> {
            Ok(Self { info: value.info? })
        }
    }
    impl From<super::SkillSkillValueSummonAttack1> for SkillSkillValueSummonAttack1 {
        fn from(value: super::SkillSkillValueSummonAttack1) -> Self {
            Self {
                info: Ok(value.info),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValueSummonAttack1Info {
        attack_after: Result<Option<i64>, String>,
        attack_count: Result<Option<super::StrOrNum>, String>,
        ball: Result<serde_json::Map<String, serde_json::Value>, String>,
        bullet_speed: Result<Option<i64>, String>,
        effect: Result<serde_json::Map<String, serde_json::Value>, String>,
        effect_after: Result<Option<i64>, String>,
        hit: Result<serde_json::Map<String, serde_json::Value>, String>,
        mob_count: Result<Option<i64>, String>,
        priority: Result<Option<super::StrOrNum>, String>,
        range: Result<Option<super::SummonRange>, String>,
        type_: Result<Option<i64>, String>,
    }
    impl Default for SkillSkillValueSummonAttack1Info {
        fn default() -> Self {
            Self {
                attack_after: Ok(Default::default()),
                attack_count: Ok(Default::default()),
                ball: Ok(Default::default()),
                bullet_speed: Ok(Default::default()),
                effect: Ok(Default::default()),
                effect_after: Ok(Default::default()),
                hit: Ok(Default::default()),
                mob_count: Ok(Default::default()),
                priority: Ok(Default::default()),
                range: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValueSummonAttack1Info {
        pub fn attack_after<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.attack_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack_after: {}", e));
            self
        }
        pub fn attack_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.attack_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack_count: {}", e));
            self
        }
        pub fn ball<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.ball = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ball: {}", e));
            self
        }
        pub fn bullet_speed<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.bullet_speed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bullet_speed: {}", e));
            self
        }
        pub fn effect<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.effect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect: {}", e));
            self
        }
        pub fn effect_after<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.effect_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effect_after: {}", e));
            self
        }
        pub fn hit<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.hit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hit: {}", e));
            self
        }
        pub fn mob_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_count: {}", e));
            self
        }
        pub fn priority<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StrOrNum>>,
            T::Error: std::fmt::Display,
        {
            self.priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for priority: {}", e));
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SummonRange>>,
            T::Error: std::fmt::Display,
        {
            self.range = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValueSummonAttack1Info>
        for super::SkillSkillValueSummonAttack1Info
    {
        type Error = String;
        fn try_from(value: SkillSkillValueSummonAttack1Info) -> Result<Self, String> {
            Ok(Self {
                attack_after: value.attack_after?,
                attack_count: value.attack_count?,
                ball: value.ball?,
                bullet_speed: value.bullet_speed?,
                effect: value.effect?,
                effect_after: value.effect_after?,
                hit: value.hit?,
                mob_count: value.mob_count?,
                priority: value.priority?,
                range: value.range?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::SkillSkillValueSummonAttack1Info> for SkillSkillValueSummonAttack1Info {
        fn from(value: super::SkillSkillValueSummonAttack1Info) -> Self {
            Self {
                attack_after: Ok(value.attack_after),
                attack_count: Ok(value.attack_count),
                ball: Ok(value.ball),
                bullet_speed: Ok(value.bullet_speed),
                effect: Ok(value.effect),
                effect_after: Ok(value.effect_after),
                hit: Ok(value.hit),
                mob_count: Ok(value.mob_count),
                priority: Ok(value.priority),
                range: Ok(value.range),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValueSummonDie {
        info: Result<Option<super::SkillSkillValueSummonDieInfo>, String>,
    }
    impl Default for SkillSkillValueSummonDie {
        fn default() -> Self {
            Self {
                info: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValueSummonDie {
        pub fn info<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SkillSkillValueSummonDieInfo>>,
            T::Error: std::fmt::Display,
        {
            self.info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValueSummonDie> for super::SkillSkillValueSummonDie {
        type Error = String;
        fn try_from(value: SkillSkillValueSummonDie) -> Result<Self, String> {
            Ok(Self { info: value.info? })
        }
    }
    impl From<super::SkillSkillValueSummonDie> for SkillSkillValueSummonDie {
        fn from(value: super::SkillSkillValueSummonDie) -> Self {
            Self {
                info: Ok(value.info),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SkillSkillValueSummonDieInfo {
        attack_after: Result<Option<i64>, String>,
        mob_count: Result<Option<i64>, String>,
        range: Result<Option<super::SummonRange>, String>,
    }
    impl Default for SkillSkillValueSummonDieInfo {
        fn default() -> Self {
            Self {
                attack_after: Ok(Default::default()),
                mob_count: Ok(Default::default()),
                range: Ok(Default::default()),
            }
        }
    }
    impl SkillSkillValueSummonDieInfo {
        pub fn attack_after<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.attack_after = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attack_after: {}", e));
            self
        }
        pub fn mob_count<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.mob_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mob_count: {}", e));
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SummonRange>>,
            T::Error: std::fmt::Display,
        {
            self.range = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for range: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SkillSkillValueSummonDieInfo> for super::SkillSkillValueSummonDieInfo {
        type Error = String;
        fn try_from(value: SkillSkillValueSummonDieInfo) -> Result<Self, String> {
            Ok(Self {
                attack_after: value.attack_after?,
                mob_count: value.mob_count?,
                range: value.range?,
            })
        }
    }
    impl From<super::SkillSkillValueSummonDieInfo> for SkillSkillValueSummonDieInfo {
        fn from(value: super::SkillSkillValueSummonDieInfo) -> Self {
            Self {
                attack_after: Ok(value.attack_after),
                mob_count: Ok(value.mob_count),
                range: Ok(value.range),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SummonRange {
        lt: Result<Option<super::Vec2>, String>,
        r: Result<Option<i64>, String>,
        rb: Result<Option<super::Vec2>, String>,
        sp: Result<Option<super::Vec2>, String>,
    }
    impl Default for SummonRange {
        fn default() -> Self {
            Self {
                lt: Ok(Default::default()),
                r: Ok(Default::default()),
                rb: Ok(Default::default()),
                sp: Ok(Default::default()),
            }
        }
    }
    impl SummonRange {
        pub fn lt<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Vec2>>,
            T::Error: std::fmt::Display,
        {
            self.lt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lt: {}", e));
            self
        }
        pub fn r<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.r = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for r: {}", e));
            self
        }
        pub fn rb<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Vec2>>,
            T::Error: std::fmt::Display,
        {
            self.rb = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rb: {}", e));
            self
        }
        pub fn sp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Vec2>>,
            T::Error: std::fmt::Display,
        {
            self.sp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sp: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SummonRange> for super::SummonRange {
        type Error = String;
        fn try_from(value: SummonRange) -> Result<Self, String> {
            Ok(Self {
                lt: value.lt?,
                r: value.r?,
                rb: value.rb?,
                sp: value.sp?,
            })
        }
    }
    impl From<super::SummonRange> for SummonRange {
        fn from(value: super::SummonRange) -> Self {
            Self {
                lt: Ok(value.lt),
                r: Ok(value.r),
                rb: Ok(value.rb),
                sp: Ok(value.sp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Vec2 {
        x: Result<i64, String>,
        y: Result<i64, String>,
    }
    impl Default for Vec2 {
        fn default() -> Self {
            Self {
                x: Err("no value supplied for x".to_string()),
                y: Err("no value supplied for y".to_string()),
            }
        }
    }
    impl Vec2 {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Vec2> for super::Vec2 {
        type Error = String;
        fn try_from(value: Vec2) -> Result<Self, String> {
            Ok(Self {
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::Vec2> for Vec2 {
        fn from(value: super::Vec2) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
}
