use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_unordered_list() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "list".to_string(),
        call_name: "unordered_list".to_string(),
        argument_types: vec![("elems".to_string(), Type::TArray(Box::new(Type::TInline)))],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn unordered_list(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(anyhow::anyhow!("Usage: [std.list elem1, elem2, ..., elemN]"), 1));
    }
    let elems = match &args[0] {
        Value::TextArray(t) => t,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("elems must be Value::TextArray"),
                1,
            ))
        }
    };
    let mut result = String::new();
    for elem in elems {
        if elem.starts_with("<ul>") && elem.ends_with("</ul>") {
            result += elem;
            continue;
        }
        result += &format!("<li>{}</li>", elem);
    }
    Ok(format!("<ul>{}</ul>", result))
}

