use extism_pdk::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum PluginArgument {
    Arg1(String),
    Arg2(String, String),
    Arg3(String, String, String),
    Arg4(String, String, String, String),
    Arg5(String, String, String, String, String),
    Arg6(String, String, String, String, String, String),
    Arg7(String, String, String, String, String, String, String),
    Arg8(String, String, String, String, String, String, String, String),
}

#[plugin_fn]
pub fn bold(args: Json<PluginArgument>) -> FnResult<String> {
    let text = match args.0 {
        PluginArgument::Arg1(text) => text,
        _ => return Err(WithReturnCode::new(anyhow::anyhow!("a"), 1)),
    };
    Ok(format!("<b style=\"font-weight:bold;\">{}</b>", text))
}

#[plugin_fn]
pub fn anchor(args: Json<PluginArgument>) -> FnResult<String> {
    match args.0 {
        PluginArgument::Arg1(href) => {
            Ok(format!("<a href=\"{}\">{}</a>", href, href))
        },
        PluginArgument::Arg2(text, href) => {
            Ok(format!("<a href=\"{}\">{}</a>", href, text))
        },
        _ => {
            Err(WithReturnCode::new(anyhow::anyhow!("a"), 1))
        },
    }
}
