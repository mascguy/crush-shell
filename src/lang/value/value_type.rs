use crate::lang::errors::{error, mandate, CrushResult, to_crush_error};
use crate::lang::{value::Value, table::ColumnType};
use crate::util::glob::Glob;
use regex::Regex;
use std::error::Error;
use crate::lang::parser::parse_name;
use crate::lang::command::CrushCommand;
use std::collections::HashMap;
use crate::lib::types;
use lazy_static::lazy_static;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ValueType {
    String,
    Integer,
    Time,
    Duration,
    Field,
    Glob,
    Regex,
    Command,
    File,
    TableStream(Vec<ColumnType>),
    Table(Vec<ColumnType>),
    Struct(Vec<ColumnType>),
    List(Box<ValueType>),
    Dict(Box<ValueType>, Box<ValueType>),
    Scope,
    Bool,
    Float,
    Empty,
    Any,
    BinaryStream,
    Binary,
    Type,
}

lazy_static! {
    pub static ref EMPTY_METHODS: HashMap<Box<str>, Box<dyn CrushCommand + Sync + Send>> = {
        HashMap::new()
    };
}

impl ValueType {
    pub fn fields(&self) -> &HashMap<Box<str>, Box<dyn CrushCommand + Sync + Send>> {
        match self {
            ValueType::List(_) =>
                &types::list::METHODS,
            ValueType::Dict(_, _) =>
                &types::dict::METHODS,
            ValueType::String =>
                &types::string::METHODS,
            ValueType::File =>
                &types::file::METHODS,
            ValueType::Regex =>
                &types::re::METHODS,
            ValueType::Glob =>
                &types::glob::METHODS,
            ValueType::Integer =>
                &types::integer::METHODS,
            ValueType::Float =>
                &types::float::METHODS,
            ValueType::Duration =>
                &types::duration::METHODS,
            ValueType::Time =>
                &types::time::METHODS,
            ValueType::Table(_) =>
                &types::table::METHODS,
            ValueType::TableStream(_) =>
                &types::table_stream::METHODS,
            _ => &EMPTY_METHODS,
        }
    }


    pub fn is(&self, value: &Value) -> bool {
        (*self == ValueType::Any) || (*self == value.value_type())
    }

    pub fn materialize(&self) -> ValueType {
        match self {
            ValueType::String |
            ValueType::Integer |
            ValueType::Time |
            ValueType::Duration |
            ValueType::Field |
            ValueType::Glob |
            ValueType::Regex |
            ValueType::Command |
            ValueType::File |
            ValueType::Scope |
            ValueType::Float |
            ValueType::Empty |
            ValueType::Any |
            ValueType::Binary |
            ValueType::Type |
            ValueType::Bool => self.clone(),
            ValueType::BinaryStream => ValueType::Binary,
            ValueType::TableStream(o) => ValueType::Table(ColumnType::materialize(o)),
            ValueType::Table(r) => ValueType::Table(ColumnType::materialize(r)),
            ValueType::Struct(r) => ValueType::Struct(ColumnType::materialize(r)),
            ValueType::List(l) => ValueType::List(Box::from(l.materialize())),
            ValueType::Dict(k, v) => ValueType::Dict(Box::from(k.materialize()), Box::from(v.materialize())),
        }
    }

    pub fn is_hashable(&self) -> bool {
        match self {
            ValueType::Scope |
            ValueType::List(_) |
            ValueType::Dict(_, _) |
            ValueType::Command |
            ValueType::BinaryStream |
            ValueType::TableStream(_) |
            ValueType::Struct(_) |
            ValueType::Table(_) => false,
            _ => true,
        }
    }

    pub fn is_comparable(&self) -> bool {
        self.is_hashable()
    }

    pub fn parse(&self, s: &str) -> CrushResult<Value> {
        match self {
            ValueType::String => Ok(Value::String(Box::from(s))),
            ValueType::Integer => match s.parse::<i128>() {
                Ok(n) => Ok(Value::Integer(n)),
                Err(e) => error(e.description()),
            }
            ValueType::Field => Ok(Value::Field(mandate(parse_name(s), "Invalid field name")?)),
            ValueType::Glob => Ok(Value::Glob(Glob::new(s))),
            ValueType::Regex => Ok(Value::Regex(Box::from(s), to_crush_error(Regex::new(s))?)),
            ValueType::File => Ok(Value::String(Box::from(s))),
            ValueType::Float => Ok(Value::Float(to_crush_error(s.parse::<f64>())?)),
            ValueType::Bool => Ok(Value::Bool(to_crush_error(s.parse::<bool>())?)),
            _ => error("Failed to parse cell"),
        }
    }
}

impl ToString for ValueType {
    fn to_string(&self) -> String {
        match self {
            ValueType::String => "string".to_string(),
            ValueType::Integer => "integer".to_string(),
            ValueType::Time => "time".to_string(),
            ValueType::Duration => "duration".to_string(),
            ValueType::Field => "field".to_string(),
            ValueType::Glob => "glob".to_string(),
            ValueType::Regex => "regex".to_string(),
            ValueType::Command => "command".to_string(),
            ValueType::File => "file".to_string(),
            ValueType::TableStream(o) => format!("table_stream {}", o.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ")),
            ValueType::Table(r) => format!("table {}", r.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ")),
            ValueType::Struct(r) => format!("struct {}", r.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ")),
            ValueType::List(l) => format!("list {}", l.to_string()),
            ValueType::Dict(k, v) => format!("dict {} {}", k.to_string(), v.to_string()),
            ValueType::Scope => "scope".to_string(),
            ValueType::Bool => "bool".to_string(),
            ValueType::Float => "float".to_string(),
            ValueType::Empty => "empty".to_string(),
            ValueType::Any => "any".to_string(),
            ValueType::BinaryStream => "binary_stream".to_string(),
            ValueType::Binary => "binary".to_string(),
            ValueType::Type => "type".to_string(),
        }
    }
}
