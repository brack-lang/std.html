use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

pub(crate) fn metadata_image() -> MetaData {
    MetaData {
        command_name: "img".to_string(),
        call_name: "image".to_string(),
        argument_types: vec![
            ("src".to_string(), Type::TInline),
            ("alt".to_string(), Type::TInline),
            (
                "caption".to_string(),
                Type::TOption(Box::new(Type::TInline)),
            ),
        ],
        return_type: Type::TBlock,
    }
}

#[plugin_fn]
pub fn image(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 3 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!(
                "Usage:
    1. {{std.img src, alt}}
    2. {{std.img src, alt, caption}}"
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
    let alt = match &args[1] {
        Value::Text(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("alt must be Value::Text"),
                1,
            ))
        }
    };
    let caption = match &args[2] {
        Value::TextOption(to) => to,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("caption must be Value::TextOption"),
                1,
            ))
        }
    };
    match caption {
        Some(cap) => Ok(format!(
            "<figure><img src=\"{}\" alt=\"{}\" /><figcaption>{}</figcaption></figire>",
            src, alt, cap
        )),
        None => Ok(format!(
            "<figure><img src=\"{}\" alt=\"{}\" /></figire>",
            src, alt
        )),
    }
}
