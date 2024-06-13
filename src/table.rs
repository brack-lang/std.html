use brack_sdk_rs::{MetaData, Type, Value};
use extism_pdk::{plugin_fn, FnResult, Json, WithReturnCode};

#[plugin_fn]
pub fn metadata_table() -> FnResult<Json<MetaData>> {
    Ok(Json(MetaData {
        command_name: "table".to_string(),
        call_name: "table".to_string(),
        argument_types: vec![
            ("col".to_string(), Type::TInline),
            ("elems".to_string(), Type::TArray(Box::new(Type::TInline))),
        ],
        return_type: Type::TBlock,
    }))
}

#[plugin_fn]
pub fn table(Json(args): Json<Vec<Value>>) -> FnResult<String> {
    if args.len() != 2 {
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Usage: {{std.table col, elems}}"),
            1,
        ));
    }
    let col = match &args[0] {
        Value::Text(t) => t.parse::<i32>(),
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("col must be Value::Text"),
                1,
            ))
        }
    };
    let col = match col {
        Ok(c) => c,
        Err(err) => return Err(WithReturnCode::new(anyhow::anyhow!(err), 1)),
    };
    let elems = match &args[1] {
        Value::TextArray(ta) => ta,
        _ => {
            return Err(WithReturnCode::new(
                anyhow::anyhow!("elems must be Value::TextArray"),
                1,
            ))
        }
    };
    let mut result = String::from("");
    for i in 0..(elems.len() as i32 + col - 1) / col {
        let mut tr = String::from("");
        for j in 0..col {
            tr += &format!("<td>{}</td>", elems[(i * col + j) as usize]);
        }
        result += &format!("<tr>{}</tr>", tr);
    }
    Ok(format!("<table><tbody>{}</tbody></table>", result))
}
