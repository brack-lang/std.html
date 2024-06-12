use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_unordered_list() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "-list".to_string(),
        call_name: "unordered_list".to_string(),
        argument_types: vec![("elems".to_string(), Type::TArray(Box::new(Type::TInline)))],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn metadata_ordered_list() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "#list".to_string(),
        call_name: "ordered_list".to_string(),
        argument_types: vec![("elems".to_string(), Type::TArray(Box::new(Type::TInline)))],
        return_type: Type::TBlock,
    }))
}

fn is_list(text: &str) -> bool {
    let is_unordered_list = text.starts_with("<ul>") && text.ends_with("</ul>");
    let is_ordered_list = text.starts_with("<ol>") && text.ends_with("</ol>");
    return is_unordered_list && is_ordered_list;
}

#[plugin_fn]
pub fn unordered_list(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.-list elem1, elem2, ..., elemN}}"),
            1,
        ));
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
        if is_list(elem) {
            result += elem;
            continue;
        }
        result += &format!("<li>{}</li>", elem);
    }
    Ok(format!("<ul>{}</ul>", result))
}

#[plugin_fn]
pub fn ordered_list(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.#list elem1, elem2, ..., elemN}}"),
            1,
        ));
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
        if is_list(elem) {
            result += elem;
            continue;
        }
        result += &format!("<li>{}</li>", elem);
    }
    Ok(format!("<ol>{}</ol>", result))
}
