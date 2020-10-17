import { Orchestrator, Config } from "@holochain/tryorama";

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

const orchestrator = new Orchestrator();

export const simpleConfig = {
  alice: Config.dna("../cloned_dna_tracker.dna.gz", null),
  bob: Config.dna("../cloned_dna_tracker.dna.gz", null),
};

const _template_dna_hash = "ewdf"
const _properties = {key1:"keyvalue1",key2:"keyvalue2"}
const _cloned_dna_hash = "dde"

orchestrator.registerScenario(
  "create and a cloned dna",
  async (s, t) => {
    const { conductor } = await s.players({
      conductor: Config.gen(simpleConfig),
    });
    await conductor.spawn();

    let clonedDNATrackerHash = await conductor.call(
      "alice",
      "cloned_dna_tracker",
      "register_cloned_dna",
      {
        template_dna_hash: _template_dna_hash,
        properties: _properties,
        cloned_dna_hash: _cloned_dna_hash
      }
    );
    t.ok(clonedDNATrackerHash);

    await sleep(10);

    let cloned_dnas = await conductor.call(
      "bob",
      "cloned_dna_tracker",
      "get_cloned_dnas_for_template",
      null
    );
    t.equal(cloned_dnas.length, 1);
  }
);

orchestrator.run();
