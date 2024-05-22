use brack_sdk_rs::{ast::AST, MetaData, Type};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_footnote() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "^".to_string(),
        call_name: "footnote".to_string(),
        argument_types: vec![], // TODO: macro does not have the custom type,
        return_type: Type::TAST,
    }))
}

#[plugin_fn]
pub fn footnote(Json((overwhole_ast, id)): Json<(AST, String)>) -> FnResult<Json<AST>> {
    let AST::Document(mut node) = overwhole_ast.clone() else {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("footnote must be called in Document"),
            1,
        ));
    };
    node.id = format!("footnote-{}", id);
    Ok(Json(AST::Document(node)))
}
