/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor,
} = require("@holochain/tryorama");

process.on("unhandledRejection", (error) => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/example-dna.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly

    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
  ),
});

const dna = Config.dna(dnaPath, "scaffold-test");
const conductorConfig = Config.gen(
  { clonedDnaTracker: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000",
    },
  }
);

orchestrator.registerScenario(
  "register cloned dna and get it",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: conductorConfig, bob: conductorConfig },
      true
    );

    const templateDna = "QmfUjRUyc8H2MkyhNDfmaEAPuKRSrRZWqcTUkyJ6YNawCs";
    const clonedDnaHash = "Qm23j09yc8HaskyhdffmaEAPuKRSrRZWqcTUkyJ6Yhgwcv";

    const clonedDna = {
      template_dna_hash: templateDna,
      properties: {
        sample_property1: "this one",
        sample_property2: "this two",
      },
      cloned_dna_hash: clonedDnaHash,
    };

    // Make a call to a Zome function
    // indicating the function, and passing it an input
    const addr = await alice.call(
      "clonedDnaTracker",
      "cloned-dnas-tracker",
      "register_cloned_dna",
      { cloned_dna: clonedDna }
    );
    t.ok(addr.Ok);

    // Wait for all network activity to settle
    await s.consistency();

    const result = await bob.call(
      "clonedDnaTracker",
      "cloned-dnas-tracker",
      "get_cloned_dnas_for_template",
      {
        template_dna: templateDna,
      }
    );

    // check for equality of the actual and expected results
    t.deepEqual(result, {
      Ok: [clonedDna],
    });
  }
);

orchestrator.run();
