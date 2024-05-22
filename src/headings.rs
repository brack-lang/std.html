use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_headings_level1() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "*".to_string(),
        call_name: "headings_level1".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn metadata_headings_level2() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "**".to_string(),
        call_name: "headings_level2".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn metadata_headings_level3() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "***".to_string(),
        call_name: "headings_level3".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn metadata_headings_level4() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "****".to_string(),
        call_name: "headings_level4".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn metadata_headings_level5() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "*****".to_string(),
        call_name: "headings_level5".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn metadata_headings_level6() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "******".to_string(),
        call_name: "headings_level6".to_string(),
        argument_types: vec![("text".to_string(), Type::TInline)],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn headings_level1(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.* text}}"),
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
    Ok(format!("<h1>{}</h1>", text))
}

#[plugin_fn]
pub fn headings_level2(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.** text}}"),
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
    Ok(format!("<h2>{}</h2>", text))
}

#[plugin_fn]
pub fn headings_level3(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.*** text}}"),
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
    Ok(format!("<h3>{}</h3>", text))
}

#[plugin_fn]
pub fn headings_level4(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.**** text}}"),
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
    Ok(format!("<h4>{}</h4>", text))
}

#[plugin_fn]
pub fn headings_level5(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.***** text}}"),
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
    Ok(format!("<h5>{}</h5>", text))
}

#[plugin_fn]
pub fn headings_level6(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 1 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.****** text}}"),
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
    Ok(format!("<h6>{}</h6>", text))
}
