# Design fork `embed-refusal` — the problem, as handed to every designer

Read `docs/prompts/designer.md` in full before you start; it is binding.

**The row:** `work/embed/a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing.md`
(P1). Its `## What is to decide` ends with three "candidates the
measurement suggests, none weighed": they are the filer's notes, not the
options. The options are yours.

**What needs deciding.** What the embedding certificate owes once it has
met something it cannot certify. An ask (`topo::clearance::embedded`, the
question the at-rest gate puts to each solid of a finished body) ends in
one of three verdicts: `Embedded`, `Crosses` with a verified witness, or
`Refused` with the engine's refusal. Today an ask that has met a refusal
does not stop: the engine keeps the FIRST refusal it met
(`lower_refusal.get_or_insert`, about a dozen sites in
`crates/topo/src/clearance.rs`) and goes on walking the frontier of cell
pairs until it is empty or the budget is spent. The verdict is then
`Violated` if a verified crossing turned up anywhere, else that first
refusal (`Sweep::run`'s `match (violation, lower_refusal, …)`). So a
refusing ask reports one refusal, the first in frontier order, having
looked at everything it could afford to.

**What was measured** (the gate built on a branch and every suite run
with it asking each solid; `git show
origin/embed/gate:work/embed/the-at-rest-gate-certifies-embedding.md`,
"Census, second take", main `c2a0c41f40`, a 4-core box, CI's build):
- 341,880 asks; 29,187 s of gate; 17,754 s on asks that hold and
  **11,434 s on asks that refuse**;
- an ask that ended in a verified crossing after it had already kept a
  refusal: **38 asks, 5 s in all**;
- a `Budget(Pairs)` refusal runs to the whole budget by its nature (the
  first take: mean 65,493 splits of 65,536), and the dearest asks in
  the suites are departure refusals on torus bodies of 4 to 7 faces,
  126 to 498 s each, 27,000 to 65,500 splits (the row);
- 1,076 tests refuse on bodies that are valid and that the engine cannot
  yet read (engine units are in flight on them); 37 tests are named for
  a refusal and cross; 179 cross at a contact inside one solid.

**Why it is a question.**
- `docs/DESIGN.md` `[topology.tiers]` says of the at-rest gate: "Its
  refusal is the list of what it found, every finding kept, none folded
  into another: each finding belongs to a family (the decision or check
  that made it), and a family's findings share one text and one
  recourse". The certificate's verdict today is one refusal, or one
  crossing, per ask.
- The same paragraph, as Ev ruled it on PR 7101: the gate's fixed
  constant "counts only what that finish asks: it is a backstop against
  an ask that would not end, not a bound on a body". Ev's words on that
  PR: the budget is a fixed constant on one ask, "not stored", there so
  the program does not hang forever.
- A `Crosses` and a `Refused` are different findings to whoever reads
  them: one says the body is not embedded and where, the other says the
  engine could not tell. The at-rest gate refuses the body on either.
- A person finishing a body the engine cannot read waits for the whole
  budget to be told so.
- `work/onecopy/kept-refusal-loops-the-ladder-row-never-listed.md`
  dispositioned this engine's kept refusals in a sweep of that pattern:
  "Not a ladder. It is a branch-and-bound frontier whose cells subdivide.
  Each bound keeps its own first refusal in frontier order, and the
  receipt counts every refused cell. Nothing argues the first-in-frontier
  order across `EngineRefusal`'s kinds; no row, by ruling." The kernel
  has a home for searches that keep a reading and go on
  (`geom_brep::ladder`, `Answering`, a refusal's rank on its type).

**Where to look.**
- `crates/topo/src/clearance.rs`: `Bound`, `Sweep::run`, `walk`,
  `BoundVerdict`, `EngineRefusal`, `CellReceipt`, `embedded`,
  `Embedding`; `crates/topo/src/injective.rs` for what a cell pair's
  reading can return.
- `crates/topo/src/validate.rs` (`AtRestBody::validate`, how findings
  are listed and what a finding is), `crates/topo/src/props.rs` and
  `crates/topo/src/finish.rs` (what `finish` does with a verdict), and
  on `origin/embed/gate` the gate as built (`git diff
  origin/main...origin/embed/gate -- crates/topo/src`).
- `docs/DESIGN.md`: `[topology.tiers]`, `[tolerance.measure]`,
  `[tolerance.recourse]`, `[loud-failure]`, `[one-way.readings]`,
  `[simple.checks]`; `crates/topo/README.md` on the tiers;
  `crates/geom-brep/README.md` on the ladder.
- The rulings on this track: PR 6301 (embedding is part of what a
  finished body promises), PR 7068 (asked per solid; an in-solid record
  refuses typed), PR 7101 (a finish carries the facts of unchanged face
  pairs; the constant is a backstop). PR 7095 is open (the sliver) and
  is not this question.
- `work/embed/a-contradicted-extent-refuses-with-no-typed-cause.md`
  and the row's refs, for what a refusal's payload carries today.

**What is not known.** How the 11,434 s divide between asks whose first
refusal comes early in the walk and late; how many refusing asks would
report MORE than one distinct refusal (by face pair, by kind) if all were
kept; what a person or an agent reading a refusal does with it. If your
answer turns on a number that does not exist, say which.

A cold build of `topo` here takes 16 to 20 minutes and the box holds one
at a time, so do not build: read, and name any measurement you need in
`## For the orchestrator`.
