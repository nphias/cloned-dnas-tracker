#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

mod cloned_dna;

use cloned_dna::{get_cloned_dnas_for_template as get_clone, register_cloned_dna as register_clone, ClonedDNA};

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn cloned_dna_entry_def() -> ValidatingEntryType {
        cloned_dna::definition()
    }

    #[entry_def]
    fn anchor_entry_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn register_cloned_dna(cloned_dna: ClonedDNA) -> ZomeApiResult<Address> {
        register_clone(cloned_dna)
    }

    #[zome_fn("hc_public")]
    fn get_cloned_dnas_for_template(template_dna: Address) -> ZomeApiResult<Vec<ClonedDNA>> {
        get_clone(template_dna)
    }
}
