use std::{collections::BTreeMap, ops::RangeInclusive};

use proto95::{
    game::{mob::MobId, npc::NpcId, reactor::ReactorId},
    id::MapId,
    shared::{FootholdId, Rect2D, Vec2},
};
use serde::{Deserialize, Serialize};

use super::shroom_schemas as sch;

fn map_bool(value: &Option<sch::Bool>) -> bool {
    value.as_ref().map(|v| v.into()).unwrap_or(false)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldNpc {
    pub id: NpcId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub hide: bool,
    pub range_x: RangeInclusive<i16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldMob {
    pub id: MobId,
    pub pos: Vec2,
    pub fh: FootholdId,
    pub hide: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldLife {
    Mob(FieldMob),
    Npc(FieldNpc),
}

impl TryFrom<&sch::FieldLifeValue> for FieldLife {
    type Error = anyhow::Error;

    fn try_from(value: &sch::FieldLifeValue) -> Result<Self, Self::Error> {
        let pos = Vec2::new(value.x.unwrap() as i16, value.y.unwrap() as i16);
        let hide = map_bool(&value.hide);
        let fh = *value.fh.as_ref().unwrap() as FootholdId;
        let id = value.id.as_ref().unwrap().into();
        Ok(match value.type_.as_deref().unwrap() {
            "m" => FieldLife::Mob(FieldMob { id, pos, fh, hide }),
            "n" => FieldLife::Npc(FieldNpc {
                id,
                pos,
                fh,
                hide,
                range_x: value.rx0.unwrap() as i16..=value.rx1.unwrap() as i16,
            }),
            _ => {
                anyhow::bail!("Unknown life type")
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldReactor {
    pub pos: Vec2,
    pub name: Option<String>,
    pub id: ReactorId,
    pub time: Option<u32>,
}

impl TryFrom<&sch::FieldReactorValue> for FieldReactor {
    type Error = anyhow::Error;

    fn try_from(value: &sch::FieldReactorValue) -> Result<Self, Self::Error> {
        let id = value.id.as_ref().unwrap().into();
        Ok(Self {
            pos: Vec2::new(value.x.unwrap() as i16, value.y.unwrap() as i16),
            name: value.name.clone(),
            id,
            time: value.reactor_time.map(|v| v as u32),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldPortal {
    pub pos: Vec2,
    pub only_once: bool,
    pub hide_tooltip: bool,
    pub has_delay: bool,
    pub teleport: bool,
    pub reactor_name: Option<String>,
    pub script: Option<String>,
    pub session_value: Option<String>,
    pub session_value_key: Option<String>,
    pub tm: Option<MapId>,
    pub tn: Option<String>,
    pub pn: Option<String>,
    pub pt: Option<MapId>,
}

impl TryFrom<&sch::FieldPortalValue> for FieldPortal {
    type Error = anyhow::Error;

    fn try_from(value: &sch::FieldPortalValue) -> Result<Self, Self::Error> {
        Ok(Self {
            pos: Vec2::new(value.x.unwrap() as i16, value.y.unwrap() as i16),
            only_once: map_bool(&value.only_once),
            hide_tooltip: map_bool(&value.hide_tooltip),
            has_delay: map_bool(&value.delay),
            teleport: map_bool(&value.teleport),
            reactor_name: value.reactor_name.clone(),
            script: value.script.clone(),
            session_value: value.session_value.clone(),
            session_value_key: value.session_value_key.clone(),
            pn: value.pn.clone(),
            tn: value.tn.clone(),
            tm: value.tm.map(|v| MapId(v as u32)),
            pt: value.pt.map(|v| MapId(v as u32)),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Foothold {
    pub pt1: Vec2,
    pub pt2: Vec2,
    pub next: FootholdId,
    pub prev: FootholdId,
    pub forbid_falldown: bool,
    pub cant_through: bool,
    pub force: Option<i32>,
    pub piece: Option<i32>,
}

impl From<&sch::Fh> for Foothold {
    fn from(value: &sch::Fh) -> Self {
        Self {
            pt1: Vec2::new(value.x1 as i16, value.y1 as i16),
            pt2: Vec2::new(value.x2 as i16, value.y2 as i16),
            next: value.next as FootholdId,
            prev: value.prev as FootholdId,
            forbid_falldown: map_bool(&value.forbid_fall_down),
            cant_through: map_bool(&value.cant_through),
            force: value.force.as_ref().map(|v| *v as i32),
            piece: value.piece.as_ref().map(|v| *v as i32),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: MapId,
    pub cloud: bool,
    pub scroll_disable: bool,
    pub no_regen: bool,
    pub fly: bool,
    pub zakum_hack: bool,
    pub rect: Rect2D,
    pub return_field: Option<MapId>,
    pub forced_return_field: Option<MapId>,
    pub portals: BTreeMap<u8, FieldPortal>,
    pub life: BTreeMap<u32, FieldLife>,
    pub reactors: BTreeMap<u32, FieldReactor>,
    pub footholds: BTreeMap<FootholdId, BTreeMap<FootholdId, BTreeMap<FootholdId, Foothold>>>,
}

impl Field {
    pub fn get_return_field(&self) -> MapId {
        self.return_field
            .or(self.forced_return_field)
            .unwrap_or(self.id)
    }

    pub fn get_first_portal_id(&self) -> Option<u8> {
        self.portals.keys().next().cloned()
    }

    pub fn get_portal_by_name(&self, tn: &str) -> Option<(u8, &FieldPortal)> {
        self.portals
            .iter()
            .find(|(_, p)| p.pn.as_deref() == Some(tn))
            .map(|(k, v)| (*k, v))
    }
}

impl TryFrom<&sch::Field> for Field {
    type Error = anyhow::Error;

    fn try_from(value: &sch::Field) -> Result<Self, Self::Error> {
        let info = value.info.as_ref().ok_or(anyhow::anyhow!("No info"))?;

        let fhs = &value.foothold;

        let footholds = fhs.iter().map(
            |(id, fh)| {
                let id = id.parse::<FootholdId>()?;
                let fhs = fh.iter().map(
                    |(id, fh)| {
                        let id = id.parse::<FootholdId>()?;
                        let fhs = fh.iter().map(
                            |(id, fh)| {
                                let id = id.parse::<FootholdId>()?;
                                let fh = Foothold::from(fh);
                                Ok((id, fh))
                            }
                        ).collect::<anyhow::Result<BTreeMap<FootholdId, Foothold>>>()?;
                        Ok((id, fhs))
                    }
                ).collect::<anyhow::Result<BTreeMap<FootholdId, BTreeMap<FootholdId, Foothold>>>>()?;
                Ok((id, fhs))
            }
        ).collect::<anyhow::Result<BTreeMap<FootholdId, BTreeMap<FootholdId, BTreeMap<FootholdId, Foothold>>>>>()?;

        Ok(Self {
            id: MapId::NONE,
            cloud: map_bool(&info.cloud),
            scroll_disable: map_bool(&info.scroll_disable),
            no_regen: map_bool(&info.no_regen_map),
            fly: map_bool(&info.fly),
            zakum_hack: map_bool(&info.zakum2_hack),
            return_field: info.return_map.map(|v| MapId(v as u32)),
            forced_return_field: info.forced_return.map(|v| MapId(v as u32)),
            rect: Rect2D::new(
                Vec2::new(
                    info.vr_left.unwrap_or(0) as i16,
                    info.vr_bottom.unwrap_or(0) as i16,
                )
                .to_point(),
                Vec2::new(
                    info.vr_right.unwrap_or(0) as i16,
                    info.vr_top.unwrap_or(0) as i16,
                )
                .to_point(),
            ),
            portals: value
                .portal
                .iter()
                .map(|(id, portal)| {
                    let id = id.parse::<u8>()?;
                    let portal = FieldPortal::try_from(portal)?;
                    Ok((id, portal))
                })
                .collect::<anyhow::Result<BTreeMap<u8, FieldPortal>>>()?,
            life: value
                .life
                .iter()
                .map(|(id, life)| {
                    let id = id.parse::<u32>()?;
                    let life = FieldLife::try_from(life)?;
                    Ok((id, life))
                })
                .collect::<anyhow::Result<BTreeMap<u32, FieldLife>>>()?,
            footholds,
            reactors: value
                .reactor
                .iter()
                .map(|(id, reactor)| {
                    let id = id.parse::<u32>()?;
                    let reactor = FieldReactor::try_from(reactor)?;
                    Ok((id, reactor))
                })
                .collect::<anyhow::Result<BTreeMap<u32, FieldReactor>>>()?,
        })
    }
}
