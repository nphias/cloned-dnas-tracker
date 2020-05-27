use hdk::prelude::*;
use holochain_anchors;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ClonedDNA {
    template_dna_hash: Address,
    properties: HashMap<String, JsonString>,
    cloned_dna_hash: Address,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "cloned_dna",
        description: "Represents a cloned DNA from a certain template",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<ClonedDNA>| {
            Ok(())
        },
        links: [
            from!(
              holochain_anchors::ANCHOR_TYPE,
              link_type: "template->clone",
              validation_package: || {
                  hdk::ValidationPackageDefinition::Entry
              },
              validation: | _validation_data: hdk::LinkValidationData | {
                  Ok(())
              }
            ),
            from!(
              holochain_anchors::ANCHOR_TYPE,
              link_type: "dna->clone",
              validation_package: || {
                  hdk::ValidationPackageDefinition::Entry
              },
              validation: | _validation_data: hdk::LinkValidationData | {
                  Ok(())
              }
            )
        ]
    )
}

/**
 * Registers the new cloned DNA in the DHT, adding links from the template DNA and the final DNA
 */
pub fn register_cloned_dna(cloned_dna: ClonedDNA) -> ZomeApiResult<Address> {
    let clone_address = hdk::commit_entry(&Entry::App("cloned_dna".into(), cloned_dna.clone().into()))?;

    let template_anchor_address =
        holochain_anchors::anchor("dna".into(), cloned_dna.template_dna_hash.clone().into())?;
    hdk::link_entries(
        &template_anchor_address,
        &clone_address,
        "template->clone",
        cloned_dna.cloned_dna_hash.clone().to_string().as_str(),
    )?;

    let dna_anchor_address =
        holochain_anchors::anchor("dna".into(), cloned_dna.cloned_dna_hash.clone().into())?;
    hdk::link_entries(
        &dna_anchor_address,
        &clone_address,
        "dna->clone",
        cloned_dna.template_dna_hash.to_string().as_str(),
    )?;

    Ok(clone_address)
}

/**
 * Returns all tracked cloned DNAs for the given template
 */
pub fn get_cloned_dnas_for_template(template_dna: Address) -> ZomeApiResult<Vec<ClonedDNA>> {
    let template_anchor_address = holochain_anchors::anchor("dna".into(), template_dna.into())?;

    hdk::utils::get_links_and_load_type(
        &template_anchor_address,
        LinkMatch::Exactly("template->clone"),
        LinkMatch::Any,
    )
}
