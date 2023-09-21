use std::str::FromStr;

use serde::{Deserialize, Serialize, de::Error};

use evalexpr::{context_map, Node, EvalexprError, Value};


#[derive(Debug, Clone)]
pub struct StrTerm{ 
    term: String,
    node: Node
}

impl StrTerm {
    pub fn eval(&self, x: i64) -> anyhow::Result<i64> {
        Ok(self.node.eval_float_with_context(
            &context_map! {
                "x" => x as f64,
                "d" => Function::new(|argument| {
                    if let Ok(float) = argument.as_float() {
                        Ok(Value::Float(float.floor()))
                    } else {
                        Err(EvalexprError::expected_float(argument.clone()))
                    }
                }),
                "u" => Function::new(|argument| {
                    if let Ok(float) = argument.as_float() {
                        Ok(Value::Float(float.ceil()))
                    } else {
                        Err(EvalexprError::expected_float(argument.clone()))
                    }
                }),
            }.unwrap()
        )?.floor() as i64)
    }
}

impl Serialize for StrTerm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.term)
    }
}

impl<'de> Deserialize<'de> for StrTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let term = String::deserialize(deserializer)?;
        let node =  evalexpr::build_operator_tree(&term).map_err(|err| D::Error::custom(err))?;
        Ok(StrTerm { term, node })
    }
}

impl FromStr for StrTerm {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node = evalexpr::build_operator_tree(s)?;
        Ok(StrTerm { term: s.to_string(), node })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum EitherStringOrInt {
    String(StrTerm),
    Int(i64),
}

impl EitherStringOrInt {
    pub fn eval(&self, x: i64) -> anyhow::Result<i64> {
        Ok(match self {
            EitherStringOrInt::String(term) => term.eval(x)?,
            EitherStringOrInt::Int(i) => *i,
        })
    }
}

impl TryFrom<String> for EitherStringOrInt {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::String(s.parse()?))
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval() {
        let term = "11+2*d(x/5)".parse::<StrTerm>().unwrap();
        assert_eq!(term.eval(0).unwrap(), 11);
        assert_eq!(term.eval(4).unwrap(), 11);
        assert_eq!(term.eval(4).unwrap(), 11);
        assert_eq!(term.eval(5).unwrap(), 13);
        assert_eq!(term.eval(6).unwrap(), 13);

        
        let term = "11+2*u(x/5)".parse::<StrTerm>().unwrap();
        assert_eq!(term.eval(0).unwrap(), 11);
        assert_eq!(term.eval(1).unwrap(), 13);
        assert_eq!(term.eval(4).unwrap(), 13);
        assert_eq!(term.eval(4).unwrap(), 13);
        assert_eq!(term.eval(5).unwrap(), 13);
        assert_eq!(term.eval(6).unwrap(), 15);
    }

    #[test]
    fn serialize_eval() {
        let term = "11+2*u(x/5)".parse::<StrTerm>().unwrap();
        let ser = serde_json::to_string(&term).unwrap();
        let de_term: StrTerm = serde_json::from_str(&ser).unwrap();


        for i in 0..15 {
            assert_eq!(term.eval(i).unwrap(), de_term.eval(i).unwrap());    
        }
    }
}
