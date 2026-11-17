# Design fork `embed-records` — the problem, as handed to every designer

Read `docs/prompts/designer.md` in full before you start; it is binding.

**The row:** `work/embed/the-at-rest-gate-certifies-embedding.md` (P0),
its `## Spec`, lane C, item 3.

**What needs deciding.** Where, and of what, a finished body's
embedding promise is certified, given that the body's contact records
are not in hand at the place its verdict is derived.

**Why it is a question.**
- `docs/DESIGN.md` `[topology.tiers]` says a finished body
  (`AtRestBody`) "passes tier 3′ against its own contact records, none
  for a body that touches nothing, and its boundary is **embedded** off
  those records", the embedding "certified by the clearance engine's
  body-level injectivity question", "asked inside the gate once tier 3
  has passed", and that the body "passes both once, at the door that
  built it". The places table says a body's at-rest validity has one
  gate, the scalar's `finish`, "the one place a body's verdict is
  derived" (Ev, PR 6326). The embedding sentences are Ev's ruling on
  PR 6301.
- In the code, `AtRestPolicy::finish` reaches
  `AtRestBody::validate(body, tol)` (`crates/topo/src/validate.rs`),
  which runs tier 3 and keeps its verdict. It is handed no
  `ContactRecords`. The records live beside the finished body: the
  Boolean door finishes its result and returns the records next to it
  in `BooleanBody` without running the tier-3′ census
  (`boolean/ops.rs`, `gate` and the `BooleanBody` doc); the n-ary door
  derives records from the already finished aggregate with the
  recording census (`boolean/nary.rs`, `census::record_at_rest`); and
  tier 3′ is `AtRestBody::validate_pseudomanifold(&contacts, tol)`,
  asked of a body that is already finished.
- The gate unit's spec has the gate exclude recorded pairs through an
  exclusion "built from the body's `ContactRecords`", and pins that
  the tour's three bench assemblies finish. That assumes the records
  are there when the body finishes. The lane building the gate found
  that they are not.
- A tier-3′ body is not injective at its records: two faces meet at a
  recorded contact and share no edge. Asked without the records, the
  certificate reads `Violated` at distance 0 there (measured on
  `bench`, `bench60` and `bench90`).

**Why it matters now.** The gate lane is building lane C on
`embed/gate`, and its census (every suite with the gate asking) is
being taken. Until this is decided the lane asks at `finish` with no
exclusion and again at `validate_pseudomanifold` with the records, so
the census counts the bodies this question touches.

**Where to look.**
- `crates/topo/src/validate.rs`: `AtRestBody`, `validate`,
  `validate_pseudomanifold`, `census_verdict`.
- `crates/topo/src/props.rs`: `AtRestPolicy` (`finish`,
  `census_at_rest`, the leaf switch); `crates/topo/src/finish.rs`
  (`take`, the finding families).
- `crates/topo/src/boolean/ops.rs` (`gate`, `BooleanBody`),
  `crates/topo/src/boolean/nary.rs`, `crates/topo/src/census.rs`
  (`record_at_rest`), `crates/topo/src/shell.rs` (its finish).
- `crates/topo/src/clearance.rs`: the verdict door `embedded` and
  `Side`.
- `docs/DESIGN.md`: `[topology.tiers]`, the at-rest row of the places
  table, D10 and whatever governs contact records (who states one, and
  "never scanning to bless").
- The rulings: PR 6301 (embedding is part of what a finished body
  promises) and PR 6326 (`finish` is the one constructor of a finished
  body).
- `work/embed/a-recorded-contact-refuses-the-injectivity-question-until-contact-design-hands-it-off.md`.

A cold build of `topo` here takes 16 to 20 minutes and the box holds
one at a time, so do not build: read, and name any measurement you
need in `## For the orchestrator`. The gate lane's census will say how
many suite bodies carry records at their finish.
