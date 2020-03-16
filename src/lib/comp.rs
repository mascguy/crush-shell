use crate::lang::command::ExecutionContext;
use crate::lang::errors::{CrushResult, argument_error};
use crate::lang::{command::SimpleCommand, value::Value};
use crate::lang::scope::Scope;
use std::cmp::Ordering;

pub fn gt(mut context: ExecutionContext) -> CrushResult<()> {
    if context.arguments.len() != 2 {
        return argument_error("Expected exactly two arguments");
    }
    let l = context.arguments.remove(0).value;
    let r = context.arguments.remove(0).value;
    match l.partial_cmp(&r) {
        Some(ordering) => context.output.send(Value::Bool(ordering == Ordering::Greater)),
        None => return argument_error("Uncomparable values"),
    }
}

pub fn lt(mut context: ExecutionContext) -> CrushResult<()> {
    if context.arguments.len() != 2 {
        return argument_error("Expected exactly two arguments");
    }
    let l = context.arguments.remove(0).value;
    let r = context.arguments.remove(0).value;
    match l.partial_cmp(&r) {
        Some(ordering) => context.output.send(Value::Bool(ordering == Ordering::Less)),
        None => return argument_error("Uncomparable values"),
    }
}

pub fn lte(mut context: ExecutionContext) -> CrushResult<()> {
    if context.arguments.len() != 2 {
        return argument_error("Expected exactly two arguments");
    }
    let l = context.arguments.remove(0).value;
    let r = context.arguments.remove(0).value;
    match l.partial_cmp(&r) {
        Some(ordering) => context.output.send(Value::Bool(ordering != Ordering::Greater)),
        None => return argument_error("Uncomparable values"),
    }
}

pub fn gte(mut context: ExecutionContext) -> CrushResult<()> {
    if context.arguments.len() != 2 {
        return argument_error("Expected exactly two arguments");
    }
    let l = context.arguments.remove(0).value;
    let r = context.arguments.remove(0).value;
    match l.partial_cmp(&r) {
        Some(ordering) => context.output.send(Value::Bool(ordering != Ordering::Less)),
        None => return argument_error("Uncomparable values"),
    }
}

pub fn eq(mut context: ExecutionContext) -> CrushResult<()> {
    if context.arguments.len() != 2 {
        return argument_error("Expected exactly two arguments");
    }
    let l = context.arguments.remove(0).value;
    let r = context.arguments.remove(0).value;
    context.output.send(Value::Bool(l.eq(&r)))
}

pub fn neq(mut context: ExecutionContext) -> CrushResult<()> {
    if context.arguments.len() != 2 {
        return argument_error("Expected exactly two arguments");
    }
    let l = context.arguments.remove(0).value;
    let r = context.arguments.remove(0).value;
    context.output.send(Value::Bool(!l.eq(&r)))
}

pub fn not(mut context: ExecutionContext) -> CrushResult<()> {
    if context.arguments.len() != 1 {
        return argument_error("Expected exactly one argument");
    }
    match context.arguments.remove(0).value {
        Value::Bool(b) => context.output.send(Value::Bool(!b)),
        _ => argument_error("Expected a boolean argument")
    }
}

pub fn declare(root: &Scope) -> CrushResult<()> {
    let env = root.create_namespace("comp")?;
    root.r#use(&env);
    env.declare("gt", Value::Command(SimpleCommand::new(gt, false)))?;
    env.declare("gte", Value::Command(SimpleCommand::new(gte, false)))?;
    env.declare("lt", Value::Command(SimpleCommand::new(lt, false)))?;
    env.declare("lte", Value::Command(SimpleCommand::new(lte, false)))?;
    env.declare("eq", Value::Command(SimpleCommand::new(eq, false)))?;
    env.declare("neq", Value::Command(SimpleCommand::new(neq, false)))?;
    env.declare("not", Value::Command(SimpleCommand::new(not, false)))?;
    env.readonly();
    Ok(())
}
