use crate::{utils,TemplateHashInput};
use hdk3::prelude::*;
use std::collections::HashMap;
use std::string::String;

#[hdk_entry(id = "cloned_dna", visibility = "public")]
#[derive(Clone)]
pub struct ClonedDNA {
    template_dna_hash: String,
    properties: HashMap<String, String>,
    cloned_dna_hash: String,
}

//path by template_dna_hash
pub fn register_clone(
    cloned: ClonedDNA,
) -> ExternResult<EntryHash> {
   // let agent_info = agent_info!()?;

    create_entry!(cloned.clone())?;
    let cloned_hash = hash_entry!(cloned.clone())?;
    let path = Path::from(format!("cloned_templates.{:?}", cloned.template_dna_hash));
    path.ensure()?;
    create_link!(path.hash()?, cloned_hash.clone())?;
    Ok(cloned_hash)
}

pub fn get_clones_by_template(template_hash: TemplateHashInput) -> ExternResult<Vec<(EntryHash, ClonedDNA)>> {
    let path = Path::from(format!("cloned_templates.{:?}",template_hash.hashstring.clone()));
    let links = get_links!(path.hash()?)?;

    links
        .into_inner()
        .iter()
        .map(|link| utils::try_get_and_convert::<ClonedDNA>(link.target.clone()))
        .collect()
}
