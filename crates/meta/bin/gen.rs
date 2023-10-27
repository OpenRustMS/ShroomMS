use meta::schemas::field_mapper::Field;
use meta::{schemas, skill, FIELD_REGIONS};
use proto95::id::MapId;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn parse_dir_name(s: &str) -> Option<usize> {
    s.strip_suffix(".img").and_then(|s| s.parse().ok())
}

/*fn load<T: DeserializeOwned>(name: &str) -> anyhow::Result<T> {
    // We use bincode for now
    let file = std::fs::File::open(format!("{name}.bincode"))?;
    Ok(bincode::deserialize_from(file)?)
}*/

fn save<T: serde::Serialize>(name: &str, v: &T, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    // We use bincode for now
    let file = std::fs::File::create(out_dir.as_ref().join(format!("{name}.bincode")))?;
    bincode::serialize_into(file, v)?;
    Ok(())
}

fn load_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    let file = std::fs::File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}

fn write_json<T: serde::Serialize>(
    name: &str,
    v: &T,
    out_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let file = std::fs::File::create(out_dir.as_ref().join(format!("{name}.json")))?;
    serde_json::to_writer_pretty(file, v)?;
    Ok(())
}

fn parse_skill(p: impl AsRef<Path>) -> impl Iterator<Item = anyhow::Result<(u32, skill::Skill)>> {
    let skill_img: schemas::shroom_schemas::Skill = load_json(p).unwrap();

    skill_img.skill.into_iter().map(|(id, mut skill)| {
        let id = id.parse::<u32>()?;
        if skill.common.is_none() {
            let levels = skill.level.len();
            let last_level = &skill.level[&levels.to_string()];

            skill.common = Some(last_level.try_into()?);
        }

        let mut skill = skill::Skill::try_from(&skill)?;
        skill.id = id;
        Ok((id, skill))
    })
}

fn parse_field(p: impl AsRef<Path>, id: u32) -> anyhow::Result<(u32, Field)> {
    let img: schemas::shroom_schemas::Field = load_json(p).unwrap();
    let mut field = Field::try_from(&img)?;
    field.id = MapId(id);
    Ok((id, field))
}

fn gen_skills(skill_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let skills = std::fs::read_dir(skill_dir)?
        .filter_map(|dir| {
            if let Ok(dir) = dir {
                if let Some(_num) = parse_dir_name(&dir.file_name().to_string_lossy()) {
                    return Some(dir);
                }
            }
            None
        })
        .par_bridge()
        .flat_map(|dir| {
            let img_file = dir.path().join("img.json");
            parse_skill(img_file).par_bridge()
        })
        .collect::<anyhow::Result<BTreeMap<u32, skill::Skill>>>()?;

    write_json("skills", &skills, out_dir)?;
    Ok(())
}

fn gen_fields(dir: impl AsRef<Path>, region: u8, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let data = std::fs::read_dir(dir)?
        .filter_map(|dir| {
            if let Ok(dir) = dir {
                if let Some(num) = parse_dir_name(&dir.file_name().to_string_lossy()) {
                    return Some((num, dir));
                }
            }
            None
        })
        .par_bridge()
        .map(|(id, dir)| {
            let img_file = dir.path().join("img.json");
            parse_field(img_file, id as u32)
        })
        .collect::<anyhow::Result<BTreeMap<u32, Field>>>()?;

    save(
        &format!("fields{region}"),
        &data,
        out_dir.as_ref().join("fields"),
    )?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let p = PathBuf::from_str(
        "/home/jonas/Dokumente/projects/shroom_data/crates/util/shroom-wz-pack/out",
    )?;
    let out_dir =
        PathBuf::from_str("/home/jonas/Dokumente/projects/open-rust-ms/ShroomMS/game_data/rbin")?;

    for i in FIELD_REGIONS {
        gen_fields(p.join(format!("maps/Map/Map{}", i)), i, &out_dir)?;
    }
    gen_skills(p.join("skill"), &out_dir)?;
    Ok(())
}
