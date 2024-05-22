use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_italic() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "/".to_string(),
        call_name: "italic".to_string(),
        // TODO
        argument_types: vec![],
        return_type: Type::TInline,
    }))
}

#[plugin_fn]
pub fn italic(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(anyhow::anyhow!("Usage: [/ text]"), 1));
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
    Ok(format!("<em>{}</em>", text))
}
