use std::collections::{HashMap, Vec};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FieldError;

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid field formulation! Must be: ...")
    }
}

pub struct Field {
    name: String,
    default: String,
    help: String,
    position: i8,
    options: Vec<String>,
    width: i8,
}

impl Field {
    pub fn new() -> Field {
        Field {
            name: String::new(),
            default: String::new(),
            help: String::new(),
            position: 0,
            options: Vec::new(),
            width: 10,
        }
    }
    pub fn build(
        name: String,
        default: String,
        help: String,
        position: i8,
        options: Vec<String>,
        width: i8,
    ) -> Result<Field, FieldError> {
        Ok(Field {
            name: name,
            default: default,
            help: help,
            position: position,
            options: options,
            width: width,
        })
    }
}

pub struct Card {
    values: HashMap<String, Field>,
}

pub struct CardErr;

impl Card {
    pub fn new(fields: Vec<Field>) -> Result<(), CardErr> {
        let mut c = Card {
            values: HashMap::new(),
        };

        for field in fields.iter() {
            c.values[field.name] = field
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
