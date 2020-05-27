# Holochain Cloned-Dnas-Tracker

This is a very small holochain zome to keep track of all cloned dnas for a given template that exist in your happ environment.

This is the usual dev workflow:

1. Put this zome in a "lobby dna", which everyone in your happ has installed. 
2. When alice clones a template DNA and creates a new DNA, they call `register_cloned_dna` in this zome to announce it to people that don't have it installed.
3. Bob is exploring the app, and retrieves a list of existing DNAs he hasn't installed by calling `get_cloned_dnas_for_template`.
4. If they want to join the DNA created by Alice, they can do so by cloning the template DNA with the same properties that Alice used to clone the template.

## Usage

Clone this repository as a git submodule inside your zomes folder, and compile your application as usual.