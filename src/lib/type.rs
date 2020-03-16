use crate::lang::{command::ExecutionContext, table::ColumnType, argument::Argument};
use crate::lang::errors::{CrushResult, argument_error};
use crate::lang::{value::Value, command::SimpleCommand, value::ValueType};
use crate::lang::scope::Scope;
use crate::lib::parse_util::{single_argument_type, single_argument_list, two_arguments};

fn to(mut context: ExecutionContext) -> CrushResult<()> {
    context.output.send(context.input.recv()?.cast(single_argument_type(context.arguments)?)?)
}

fn of(mut context: ExecutionContext) -> CrushResult<()> {
    context.output.send(Value::Type(context.input.recv()?.value_type()))
}

fn list(mut context: ExecutionContext) -> CrushResult<()> {
    let l = single_argument_type(context.arguments)?;
    context.output.send(Value::Type(ValueType::List(Box::new(l))))
}

fn dict(mut context: ExecutionContext) -> CrushResult<()> {
    two_arguments(&context.arguments)?;
    match (context.arguments.remove(0).value, context.arguments.remove(0).value) {
        (Value::Type(key_type), Value::Type(value_type)) => {
            context.output.send(Value::Type(ValueType::Dict(Box::new(key_type), Box::new(value_type))))
        }
        _ => return argument_error("Expected two types as input")
    }
}

fn parse_column_types(mut arguments: Vec<Argument>) -> CrushResult<Vec<ColumnType>> {
    let mut types = Vec::new();

    for arg in arguments.drain(..) {
        if let Value::Type(t) = arg.value {
            types.push(ColumnType::new(arg.name, t));
        } else {
            return argument_error(format!("Expected all parameters to be types, found {}", arg.value.value_type().to_string()).as_str())
        }
    }
    Ok(types)
}

fn r#struct(mut context: ExecutionContext) -> CrushResult<()> {
    context.output.send(Value::Type(ValueType::Struct(parse_column_types(context.arguments)?)))
}

fn r#table(mut context: ExecutionContext) -> CrushResult<()> {
    context.output.send(Value::Type(ValueType::Table(parse_column_types(context.arguments)?)))
}

fn r#table_stream(mut context: ExecutionContext) -> CrushResult<()> {
    context.output.send(Value::Type(ValueType::TableStream(parse_column_types(context.arguments)?)))
}

pub fn declare(root: &Scope) -> CrushResult<()> {
    let env = root.create_namespace("type")?;

    env.declare("to", Value::Command(SimpleCommand::new(to, true)))?;
    env.declare("of", Value::Command(SimpleCommand::new(of, false)))?;

    env.declare("list", Value::Command(SimpleCommand::new(list, false)))?;
    env.declare("dict", Value::Command(SimpleCommand::new(dict, false)))?;
    env.declare("struct", Value::Command(SimpleCommand::new(r#struct, false)))?;
    env.declare("table", Value::Command(SimpleCommand::new(table, false)))?;
    env.declare("table_stream", Value::Command(SimpleCommand::new(table_stream, false)))?;

    env.declare("integer", Value::Type(ValueType::Integer))?;
    env.declare("type", Value::Type(ValueType::Type))?;
    env.declare("text", Value::Type(ValueType::Text))?;
    env.declare("bool", Value::Type(ValueType::Bool))?;
    env.declare("closure", Value::Type(ValueType::Closure))?;
    env.declare("empty", Value::Type(ValueType::Empty))?;
    env.declare("field", Value::Type(ValueType::Field))?;
    env.declare("float", Value::Type(ValueType::Float))?;
    env.declare("duration", Value::Type(ValueType::Duration))?;
    env.declare("time", Value::Type(ValueType::Time))?;
    env.declare("command", Value::Type(ValueType::Command))?;
    env.declare("file", Value::Type(ValueType::File))?;
    env.declare("glob", Value::Type(ValueType::Glob))?;
    env.declare("regex", Value::Type(ValueType::Regex))?;
    env.declare("env", Value::Type(ValueType::Scope))?;
    env.declare("any", Value::Type(ValueType::Any))?;
    env.declare("binary", Value::Type(ValueType::Binary))?;
    env.declare("binary_stream", Value::Type(ValueType::BinaryStream))?;

    env.readonly();
    Ok(())
}
