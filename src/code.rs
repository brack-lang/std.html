use std::{fs::File, io::Read};

use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_inline_code() -> MetaData {
    MetaData {
        command_name: "$".to_string(),
        call_name: "inline_code".to_string(),
        argument_types: vec![
            ("src".to_string(), Type::TInline),
            (
                "language".to_string(),
                Type::TOption(Box::new(Type::TInline)),
            ),
        ],
        return_type: Type::TInline,
    }
}

pub(crate) fn metadata_block_code() -> MetaData {
    MetaData {
        command_name: "$".to_string(),
        call_name: "block_quote".to_string(),
        argument_types: vec![
            ("src".to_string(), Type::TInline),
            (
                "language".to_string(),
                Type::TOption(Box::new(Type::TInline)),
            ),
        ],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn inline_code(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 2 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!(
                "Usage:
    1. [std.$ src]
    2. [std.$ src, language]"
            ),
            1,
        ));
    }
    let src = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("src must be Value::Text"),
                1,
            ))
        }
    };
    let language = match &args[1] {
        Value::TextOption(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("language must be Value::TextOption"),
                1,
            ))
        }
    };
    let file = File::open(src);
    if file.is_err() {
        return Err(WithReturnCode::new(anyhow::anyhow!("file not found"), 1));
    }
    let mut file = file?;
    let mut code = String::new();
    if let Err(_) = file.read_to_string(&mut code) {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("something went wrong reading the file"),
            1,
        ));
    }
    match language {
        Some(language) => Ok(format!(
            "<code class=\"language-{}\">{}</code>",
            language, code
        )),
        None => Ok(format!("<code>{}</code>", code)),
    }
}

#[plugin_fn]
pub fn block_code(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 2 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!(
                "Usage:
    1. {{std.$ src}}
    2. {{std.$ src, language}}"
            ),
            1,
        ));
    };
    let src = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("src must be Value::Text"),
                1,
            ))
        }
    };
    let language = match &args[1] {
        Value::TextOption(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("language must be Value::TextOption"),
                1,
            ))
        }
    };
    let file = File::open(src);
    if file.is_err() {
        return Err(WithReturnCode::new(anyhow::anyhow!("file not found"), 1));
    }
    let mut file = file?;
    let mut code = String::new();
    if let Err(_) = file.read_to_string(&mut code) {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("something went wrong reading the file"),
            1,
        ));
    }
    match language {
        Some(language) => Ok(format!(
            "<pre><code class=\"language-{}\">{}</code></pre>",
            language, code
        )),
        None => Ok(format!("<pre><code>{}</code></pre>", code)),
    }
}
