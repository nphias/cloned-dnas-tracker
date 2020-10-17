use crate::utils;
use hdk3::prelude::*;
use std::collections::HashMap;

#[hdk_entry(id = "cloned_dna", visibility = "public")]
#[derive(Clone)]
pub struct ClonedDNA {
    template_dna_hash: EntryHash,
    properties: HashMap<String, String>,
    cloned_dna_hash: EntryHash,
}

pub fn register_clone(
    cloned_dna: ClonedDNA,
) -> ExternResult<EntryHash> {
   // let agent_info = agent_info!()?;

    create_entry!(cloned_dna.clone())?;
    let cloned_dna_hash = hash_entry!(cloned_dna)?;
    let path = Path::from("cloned_dnas");
    path.ensure()?;
    create_link!(path.hash()?, cloned_dna_hash.clone())?;
    Ok(cloned_dna_hash)
}

pub fn get_clones_by_template(template_hash: EntryHash) -> ExternResult<Vec<(EntryHash, ClonedDNA)>> {
    //let path = Path::from("cloned_dnas");
    let links = get_links!(template_hash)?;//, utils::link_tag("resource->booking_request")?)?;
    //let links = get_links!(path.hash()?)?;

    links
        .into_inner()
        .iter()
        .map(|link| utils::try_get_and_convert::<ClonedDNA>(link.target.clone()))
        .collect()
}
