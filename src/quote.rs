use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_inline_quote() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: ">".to_string(),
        call_name: "inline_quote".to_string(),
        argument_types: vec![
            ("text".to_string(), Type::TInline),
            ("cite".to_string(), Type::TOption(Box::new(Type::TInline))),
        ],
        return_type: Type::TInline,
    }))
}

#[plugin_fn]
pub fn metadata_block_quote() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: ">".to_string(),
        call_name: "block_quote".to_string(),
        argument_types: vec![
            ("text".to_string(), Type::TInline),
            ("cite".to_string(), Type::TOption(Box::new(Type::TInline))),
        ],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn inline_quote(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 2 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!(
                "Usage:
    1. [std.> text]
    2. [std.> text, cite]"
            ),
            1,
        ));
    }
    let text = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::Text"),
                1,
            ))
        }
    };
    let cite = match &args[1] {
        Value::TextOption(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::TextOption"),
                1,
            ))
        }
    };
    match cite {
        Some(cite) => Ok(format!("<q cite=\"{}\">{}</q>", cite, text)),
        None => Ok(format!("<q>{}</q>", text)),
    }
}

#[plugin_fn]
pub fn block_quote(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 2 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!(
                "Usage:
    1. {{std.> text}}
    2. {{std.> text, cite}}"
            ),
            1,
        ));
    };
    let text = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::Text"),
                1,
            ))
        }
    };
    let cite = match &args[1] {
        Value::TextOption(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::TextOption"),
                1,
            ))
        }
    };
    match cite {
        Some(cite) => Ok(format!(
            "<blockquote cite=\"{}\">{}</blockquote>",
            cite, text
        )),
        None => Ok(format!("<blockquote>{}</blockquote>", text)),
    }
}
