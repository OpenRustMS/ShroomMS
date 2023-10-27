use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::skill::eval::{EvalContext, self};


#[derive(Debug, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    pub lt: Vec2,
    pub rb: Vec2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Circ {
    pub sp: Vec2,
    pub radius: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementAttribute {
    Fire,
    Ice,
    Poison,
    Holy,
    Light,
    Physical,
    Dark,
}

impl TryFrom<&str> for ElementAttribute {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "f" => Self::Fire,
            "i" => Self::Ice,
            "s" => Self::Poison,
            "h" => Self::Holy,
            "l" => Self::Light,
            "p" => Self::Physical,
            "d" => Self::Dark,
            _ => anyhow::bail!("Invalid elem attribute: {}", s),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EvalExpr {
    Num(i32),
    Expr(eval::Expr),
}

impl EvalExpr {
    pub fn eval(&self, x: i32) -> i32 {
        match self {
            Self::Num(n) => *n,
            Self::Expr(expr) => EvalContext::new(x).eval(expr).ceil() as i32,
        }
    }
}

pub fn opt_map2<'a, D, T, U>(opt: &'a Option<D>) -> Result<Option<U>, U::Error>
where
    D: Deref<Target = T>,
    T: 'a + ?Sized,
    U: TryFrom<&'a T>,
{
    opt.as_deref().map(U::try_from).transpose()
}

pub fn opt_map1<'a, T, U>(opt: &'a Option<T>) -> Result<Option<U>, U::Error>
where
    T: 'a,
    U: TryFrom<&'a T>,
{
    opt.as_ref().map(U::try_from).transpose()
}