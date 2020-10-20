use hdk3::prelude::*;
use std::string::String;
mod cloned_dna;
mod utils;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

entry_defs![
    Path::entry_def(),
    cloned_dna::ClonedDNA::entry_def()
];

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct TemplateHashInput {
    pub hashstring: String
}

/** Clone template API **/

#[hdk_extern]
pub fn register_cloned_dna(
    cloned_dna_data: cloned_dna::ClonedDNA,
) -> ExternResult<EntryHash> {
    cloned_dna::register_clone(cloned_dna_data)
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct ClonedDnaList(Vec<(EntryHash, cloned_dna::ClonedDNA)>);
#[hdk_extern]
pub fn get_cloned_dnas_for_template(template_hash: TemplateHashInput) -> ExternResult<ClonedDnaList> {
    let cloned_dnas = cloned_dna::get_clones_by_template(template_hash)?;

    Ok(ClonedDnaList(cloned_dnas))
}
