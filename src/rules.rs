use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_rules() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "---".to_string(),
        call_name: "rules".to_string(),
        argument_types: vec![],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn rules(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 0 {
        return Err(WithReturnCode::new(anyhow::anyhow!("Usage: {{std.---}}"), 1));
    }
    Ok(String::from("<hr />"))
}
