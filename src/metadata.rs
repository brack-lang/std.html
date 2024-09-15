use brack_sdk_rs::MetaData;
use extism_pdk::{FnResult, Json, plugin_fn};

use crate::{
    anchor::metadata_anchor,
    bold::metadata_bold,
    code::{metadata_block_code, metadata_inline_code},
    footnote::metadata_footnote,
    headings::metadata_headings,
    image::metadata_image,
    italic::metadata_italic,
    list::{metadata_ordered_list, metadata_unordered_list},
    quote::{metadata_block_quote, metadata_inline_quote},
    rules::metadata_rules,
    strike::metadata_strike, table::metadata_table,
};

#[plugin_fn]
pub fn get_metadata() -> FnResult<Json<Vec<MetaData>>> {
    let mut metadata = Vec::new();
    metadata.push(metadata_anchor());
    metadata.push(metadata_bold());
    metadata.push(metadata_inline_code());
    metadata.push(metadata_block_code());
    metadata.push(metadata_footnote());
    metadata.append(&mut metadata_headings());
    metadata.push(metadata_image());
    metadata.push(metadata_italic());
    metadata.push(metadata_unordered_list());
    metadata.push(metadata_ordered_list());
    metadata.push(metadata_inline_quote());
    metadata.push(metadata_block_quote());
    metadata.push(metadata_rules());
    metadata.push(metadata_strike());
    metadata.push(metadata_table());
    Ok(Json(metadata))
}
