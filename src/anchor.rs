use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_anchor() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "@".to_string(),
        call_name: "anchor".to_string(),
        argument_types: vec![
            ("href".to_string(), Type::TInline),
            ("text".to_string(), Type::TOption(Box::new(Type::TInline))),
        ],
        return_type: Type::TInline,
    }))
}

// [@ a, b] => <a href="a">b</a>
#[plugin_fn]
pub fn anchor(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 2 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!(
                "Usage:
    1. [@ href]
    2. [@ href, text]"
            ),
            1,
        ));
    }
    let href = match &args[0] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("href must be Value::Text"),
                1,
            ))
        }
    };
    let text = match &args[1] {
        Value::TextOption(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("text must be Value::TextOption"),
                1,
            ))
        }
    };
    match text {
        Some(text) => Ok(format!("<a href=\"{}\">{}</a>", href, text)),
        None => Ok(format!("<a href=\"{}\">{}</a>", href, href)),
    }
}
