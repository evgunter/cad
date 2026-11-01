# SPEC: INTENT stage 5 — assertions and the at-rest lints

Scope: DESIGN `[assertions]` and `[assertions.quiet]`, as `work/intent/plan.md` stage 5 states it:

- `Assert` with `=` (PR A, built, #4508);
- interference as its own finding (PR B, built, #4560);
- the at-rest census as a check, and the quieting rule's contact half (PR C, `work/atrest/the-at-rest-census-is-a-check.md`).

**The ratified text C builds.** DESIGN governs where anything below disagrees with it, and this spec elaborates it without changing it.

- `[assertions]`, *The at-rest census*: the census is each `⊔`'s door, it examines the meetings between that operator's operands pairwise, and nothing it finds refuses. An arrival is one operand at one value of its indices. A contact is an `unproven-coincidence` finding unless it is structural. An overlap is an interference finding, and a pair the census has no lane for is a finding saying it could not look. Inside one arrival the census decides and reads nothing. A finding belongs to the `⊔` whose census decided it, is reported once there, and a `⊔` the product does not read is checked all the same.
- `[assertions.quiet]`: a holding assertion reading a `Gap` directly quiets a contact when the `Gap` is taken between the finding's two cells as its record carries them, at whatever dimension they are, wherever they lie; it quiets an interference by the face-pair reading B built. An assertion quiets only findings between bodies of its own space.
- `[coincidence.door]`: the `unproven-coincidence` lint checks every coincidence inferred from values, a contact the at-rest census finds included. `[coincidence.structural]`: a mate is the first rung's statement, and a flag mate establishes the flush pair its walls make.
- `[topology.records]`: between two operands of a `⊔` at rest, the census records each touch as a finding and leaves a contact record citing it.
- `[operations.operators]` and `[coincidence.booleans]`: `⊔` is the fourth operator, each operator decides the meetings between its operands, and `A ⊔ A` is an interference finding.
- `[placement.placement]` and `[placement.world]`: a copy carries its source's contact records and census decisions, which stay the source's; an instance's are carried and reported once, as its document's.
- State of the build: A5's at-rest gate (`assemble` and its refusal) retires, its census becoming a check, with `Separation`.
- CHECKS DS6 (`crates/editor-core/CHECKS.md`): an assertion is a declarable exception, a quiet finding is listed with what quiets it, and a finding outside what the document can yet say is loud, never refuses and is not quietable. DS3: a check offers off/warn/error, or off/warn where it ships no way to quiet a finding.

**Rulings this spec builds on.**

- **Fork-log row 91** (FORK-S5C, approved on PR 4320). Nothing at rest refuses, and a caller wanting a gate runs `enforce_checks`. At-rest contacts are the `unproven-coincidence` lint's. `AtRest` owns interference and could-not-look, and defaults to Warn. Assertions are the declarable exception. `Separation`, `assemble` and A5's refusal retire.
- **Fork-log row 98** (FORK-S5Q, approved on PR 4319). Only a `Gap` read directly quiets, and only with values of the finding's sign. A contact is quiet at exactly its two cells, and an unbounded overlap is loud.
- **Ev on PR 4644** (2026-10-11): an unproven row is ONE finding. So `AtRest` reports no contact (`stage5-contact-finding-and-the-unproven-row-report-one-contact-twice`).
- **PR 4666** (Ev, 2026-10-17 to 19): the cell-reading quieting wording, and the census between arrivals. The confirmation inside an arrival is transitional, scheduled to retire (`the-in-arrival-census-check-retires`).
- **PR 5494**: each operator decides its operands' meetings, `⊔` is an operator, the census is its door, and an arrival is one operand.
- **PR 5555**: a mate is the first rung, an area contact is one face pair, and a flush pair is a finding unless a flag mate or the replay establishes it.

**Baseline.** C is written against D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`, PR 5636, branch `intent/dprime3-assemble-product-slot` at `199c17c1`) merged over main at `c19debfa`, which carries stage 4 I (PR 5637: the mate mint is gone). The two had not been merged together when this was written. So each site below is named where it stands on D′3's branch. The exceptions are on main only, from 5637: `mated_pairs`, `CarriedFindings`, and the mint-free `assembly.rs`. C's lane re-greps on the merged main at dispatch and states the drift.

The state C starts from (D′3 over main):

- **The census at each `⊔`.** `product::product_recorded` gathers the body the product slot reads. Where that is a `⊔`, each operand is one arrival, grafted as the `⊔` grafted it (`product.rs`, `gather`). The gather runs `topo::record_at_rest` with one group per arrival (`checks/at_rest.rs`, `copy_groups`) and keeps the census's verdicts on the product (`Product::census`) and its rows (`Product::coincidences`, each an `AtRestRow`). Every `⊔` the slot does not read is gathered the same way, beside it (`product::assemble_gathers`, `Product::assembles`, `AssembleGather`). `wire_assemble` (`eval/wire.rs`) grafts and decides nothing (D′3's deviation 1). If D′3's pending MAJOR 1 moves the census into the `⊔`'s wire, the resident reads each `Assemble` node's value instead of the gathers, and nothing else in §4 changes.
- **The gate** (`assembly.rs`). `assemble` and `assemble_gathered` call `verdict` on the product and `gate_assembles` on each other `⊔`. `verdict` refuses `AssemblyError::CarriedAtRest` on a part's carried refusal (`ahead_of_at_rest`). It then reads the census (on D′3's branch, less what a part already answered, `checks::at_rest::less_carried`, which neither D′3 nor C kept) and partitions it (`checks::at_rest::partition`): interference findings go to `Assembly::interference`, and everything else refuses `AssemblyError::AtRest`. An unread `⊔` whose gather refuses refuses `AssemblyError::Assemble`. A part's verdict crosses the instance seam as `seam_verdict` reads it: `CarriedInterference` and `CarriedAtRestFinding`, each with a `Route`, reported once per part row (`once_per_part_row`).
- **The lint** (`checks.rs`, `door_rows`, `DoorJob`). `CheckId::UnprovenCoincidence` already reads the at-rest rows of the product and of every unread `⊔`. Its knob is `Advisory`, because it reads no assertion.
- **`Separation`** (`checks.rs`, `separation`, `mated_pairs`). It reports two solids from different subjects that `topo::SolidSeparation`'s boxes cannot prove apart. It is suppressed where a record cites a row the door's first rung proves, and its knob is `Advisory`.
- **Callers of the gate**: the viewer's at-rest badge (`viewer/src/session.rs`: `assemble_gathered` in the landing, `badge`, `AtRestBadge`); Python (`pncad.assemble`, `py/assembly.rs` `assemble`; `product_memo.rs`); the tour (`demos/tour/src/assembly.rs`, two `assemble` calls); and the gallery's separation count (`demos/tour/src/gallery.rs`, `advisory`). D′3 also adds `Doc.assemble` and `Node.assemble` to Python, which author the `⊔` operator and are not the gate.

## 0. Ordering

| PR | Unit | State |
|---|---|---|
| A | `an-assertion-relates-by-equality` | built (#4508): `AssertionRelation { AtLeast, AtMost, Equal }` |
| B | `interference-at-rest-is-a-finding` | built (#4560): `InterferenceFinding` per overlap, quieted by `quiet_verdicts` |
| C | `the-at-rest-census-is-a-check` | this spec, §4 |

**C dispatches after D′3 merges, and waits on nothing else.**

- **Why after D′3.** D′3 moves the census to each `⊔` and rewrites the `assembly.rs`, `product.rs` and `checks/at_rest.rs` paths C dissolves. Built first, C would rewrite a path D′3 is moving.
- **What C does not wait on.** Three INTENT units, all parked behind the carry and E's core: `a-placement-carries-its-sources-at-rest-result`, `each-space-runs-its-census-between-arrivals` and `the-in-arrival-census-check-retires`. Each changes what the census hands the resident (§10), not where the resident reads it or what it reports. C lands first and reads the census as D′3 leaves it.
- **The intermediate state is whole.** After C there is no at-rest gate. Every census outcome at every `⊔` is a finding:
  - a contact is the lint's row;
  - an overlap is an `AtRest` interference finding;
  - anything else is an `AtRest` could-not-look finding.

  Each is quiet or loud, and only `enforce_checks` refuses. The transitional in-arrival check is a could-not-look source until its own unit retires it.

**Rejected:**

- **C before D′3** would rewrite `gate_spaces` and `space_gathers`, which D′3 has already replaced by `gate_assembles` and `assemble_gathers`.
- **C split into "resident" and "contact half"** would leave the lint's at-rest rows unquietable in a state where `AtRest` refuses nothing and the lint is the only voice for a contact. That state is whole, but nothing in it is smaller to review. The contact half is one matching rule in a resident that already reads the same rows.

## 1. Final shapes (after C)

**Assertions** are as A built them (`measure.rs`, `node.rs`): `Node::Assertion { value, relation, bound }` with `AssertionRelation { AtLeast, AtMost, Equal }`, decided through `ASSERT_BOUND`.

**Findings at rest** (`checks.rs`, `checks/at_rest.rs`):

- **An arrival** is one operand of a `⊔` at one value of its indices, as `[assertions]` defines it. The census names it by the read the operand entered the `⊔` through: the operand's `VarId`, at the member's index values for a family. `CopyRef` and `FaceSite` keep their shape, read as an arrival and a face as the arrival's own name table spells it (`StableName`), as D′3's `checks/at_rest.rs` already un-keys a `⊔`'s `From { read, of }`.
- **A contact is the lint's, and only the lint's.** A touch the census records between two arrivals is a `CensusAtRest` row (`AtRestRow`) and a record citing it. `CheckId::UnprovenCoincidence` reports it unless the door proves it, once, under the `⊔` that decided it. `AtRest` has no contact finding. This holds for a flush pair as well.
- **`CheckId::AtRest` reports two kinds of finding**, each under the `⊔` that decided it (`FindingSubject` names that `⊔`'s output):
  - **interference**: `InterferenceFinding` as B built it, between two arrivals, quiet or loud;
  - **could-not-look**: a verdict between or inside arrivals that the census could not turn into a decision, loud, unquietable, and never refusing (DS6). Its evidence names why (§4).
- **Carried findings** are a part's own. Interference and could-not-look findings that crossed an instance seam are reported once per part row, with their `Route`, as the part's, judged by the part's assertions. An assertion of the host never quiets one: the finding belongs to the part's `⊔`, and the host's `Gap` reads bodies of the host's space (`[assertions.quiet]`).
- **Quiet or loud.** `CheckFinding` gains `quiet: Option<RecipeNodeId>`, the assertion that quiets it. A quiet finding is listed, naming its assertion, so "checked and intended" stays distinct from "not checked". `enforce_checks` never refuses it.
- **The quieting rule** has one home, `checks/at_rest.rs`, and two halves. Both read an assertion's verdict in the evaluation the census ran in, never its measure's value, and both admit only an assertion whose `value` reads one `Gap` measure's output directly.
  - **Interference** (B's `quiet_verdicts`, unchanged): an opposed pair of faces of the two arrivals that bound the overlap, every bounding face between the asserted carriers, and the bound decided negative.
  - **Contact** (C): `= 0`, the `Gap`'s two selections are the row's two cells as the row names them, and the assertion is in the `⊔`'s space. The lint calls it for each at-rest row the door does not prove.
- **The resident.** `CheckId::AtRest`: `kind() == Certified`, `reads_subject() == true`, knob `Severity` (Off/Warn/Error), default Warn. `Error` is legal because an assertion quiets every interference finding the kernel can bound (DS3, DS6). At `Error`, `enforce_checks` refuses a loud interference finding and nothing else the resident reports.

**What retires.** These go: the gate (`assemble`, `assemble_gathered`, `verdict`, `gate_assembles`, `ahead_of_at_rest`); `Assembly`; `AssemblyError` with every arm (§4 says where each goes); `CheckId::Separation` with `CheckEvidence::NotSeparated` and `SeparationUnavailable`; `ChecksConfig::separation`; `mated_pairs`; and the viewer's `AtRestBadge::{Certified, Refused}`. The census, the records, the rows and the carried-row types stay. They are the resident's input.

**Who reads a finding.** The checks report, and through it the viewer, Python and the tour. No door, edit, evaluation or export reads one.

## 2. PR A — built

`an-assertion-relates-by-equality`, #4508. `=` is decided pointwise through the one `ASSERT_BOUND` site. Over a window superset it needs both ends admitted. In the analysis lanes it is held to the Zero band. C reads its verdicts and changes nothing of it.

## 3. PR B — built

`interference-at-rest-is-a-finding`, #4560. `checks::at_rest::partition` takes the census's interference verdicts between two copies out of the refusal. These are `InstanceInterference`, `EdgeFacePierce`, and a `SameSide` crossing (`topo`'s typed side field). Each such pair is intersected (`LocalizeLane`), and each connected solid of the intersection is one `InterferenceFinding`, named by the faces of both copies that bound it. An intersection that refuses leaves `Overlap::Unlocalized`, which is loud and unquietable. `quiet_verdicts` is the interference half of the quieting rule. C keeps all of it and moves only who reports it: the registry instead of `Assembly`.

## 4. PR C — `the-at-rest-census-is-a-check` (cost H; ~60 files, 2–3k lines)

### What the resident reads at each `⊔`

For the product (the `⊔`, or other body, the slot reads) and for each `AssembleGather` the product carries, the resident takes the gather's census verdicts as the gate reads them today:

1. `Product::census` as the gather kept it (a part's pair-less escalation re-found inside its copy is reported, loud, until it names its pair: `work/contacthold/an-escalated-census-finding-names-no-pair.md`);
2. `partition` into interference findings and the rest;
3. the product's carried findings (`carried_interference`, `carried_at_rest`), each once per part row.

The census is not re-run. The resident runs no predicate of its own and calls the same functions the gate called, in the same order, so the census's output and the interference findings are bit-equal to the gate's. A `⊔` whose gather refuses is a could-not-look finding against that `⊔`, not an error of the report. The registry's own `ChecksError::Product` stays for the product slot's gather alone, as every subject-reading resident raises it.

### What it reports

- **Interference.** Each `InterferenceFinding`, under its `⊔`, quieted by `quiet_verdicts`.
- **Could-not-look**, one finding per verdict, with typed evidence `AtRestEvidence` (in `checks/at_rest.rs`, carried on `CheckEvidence::AtRest`):
  - `NoLane(ValidationError)`: a pair the census had no lane for or left undecided (`CensusUnsupported`, `CensusLaneUnsupported`, `CensusUndecidable`, `CensusEscalated`), or a touch between two arrivals it could not clear, which `record_at_rest` keeps as a finding, not a row;
  - `Malformed(ValidationError)`: the product's or the `⊔`'s body failing tiers 1–3;
  - `InsideArrival(ValidationError)`: a touch or overlap inside one arrival that no carried record backs and no carried finding matches (`UnrecordedContact`, `InstanceInterference` within one group), or a stale record (`StaleContactDeclaration`). This is the transitional in-arrival check's output (§10), the arrival's door's defect;
  - `NotGathered(ProductError)`: a `⊔` the product does not read whose gather refused (today's `AssemblyError::Assemble`);
  - `NotRun`: a scalar whose at-rest policy runs no census (`topo::AtRestOutcome::NotRunAtThisScalar`). That is one finding, not a clean report;
  - `Carried(CarriedAtRestFinding)`: a part's could-not-look finding, crossed with its route (today's `AssemblyError::CarriedAtRest`).

  None of these refuses, at any severity. They are reported loud and counted apart from interference, because what the resident could not judge is not something the document said, and no assertion answers it (DS6).
- **Carried interference.** Each `CarriedInterference` under the instance it came through, quiet or loud as the part judged it.
- **No contact.** `AtRest` reports no contact. The rows go to the lint, unchanged.

### The contact half of the quieting rule

It lives in `checks/at_rest.rs` beside `quiet_verdicts`, and the `unproven-coincidence` lint calls it. For each at-rest row of a `⊔` (an `AtRestRow`, or the product's or a gather's `coincidences`) that the door's findings leave unproven, the row's finding is quiet when some assertion meets all of the following:

1. its `value` reads, directly, the output of one `Measure` whose primitive is `Gap { outer, inner }`;
2. its two selections, in either order, are the row's two cells as the row names them: each selection's body is the read the cell's arrival entered the `⊔` through (or a pick of that piece), and its name is the cell's name there;
3. its relation is `=` and its bound is decided zero;
4. its verdict in the evaluation the census ran in is `Holds`.

The space condition follows from the selections: a `Gap` over the `⊔`'s operands reads bodies of the `⊔`'s space. The rule matches by cells, not by bodies, so a second contact between the same two arrivals stays loud. It reads verdicts only, so a failed or poisoned assertion quiets nothing, gates nothing, and the finding stays loud (`a-failed-requirement-refuses-the-whole-product`'s acceptance is kept). The rule is written over a row's two cells, whatever the row's kind and the cells' dimension (§11):

- **Rows.** C applies it to the at-rest rows of each `⊔`. An operation's unproven row (a boolean's glue, a split's ON verdict, a blend's) is quieted by the same rule once its cells can be selected (`an-assertion-quiets-an-operations-unproven-row`).
- **Dimensions.** C quiets every row whose two cells `Gap`'s door admits: a face, an edge or a vertex (`gap-reads-vertex-and-edge-cells`). A row whose cell `Gap` refuses (a curved edge or a cell beside a curved face, `gap-reads-no-curved-cell-below-a-face`) stays loud, since no `Gap` over it holds.
- **A flush pair is never quieted by an assertion.** It is no contact, and only a flag mate or the replay establishes it (`[assertions]`).
- **The lint's knob stays `Advisory`.**

### Severity and the refusing path

- `ChecksConfig::at_rest: Severity`, default `Warn`.
- `enforce_checks` refuses a finding only where an assertion could have quieted it. It skips every finding whose `quiet` is `Some`, and every finding no assertion the quieting rule accepts could quiet (`CheckEvidence::never_refuses`): every `AtRest` could-not-look finding, an unlocalized overlap, and a bounded one the rule finds no quieting `Gap` for (`InterferenceFinding::quietable`), whatever the severity. So the only `AtRest` findings that refuse are loud quietable interference findings at `Error`.
- A quiet finding stays stale-safe for free: an assertion whose finding goes away still checks its own measure, and a `Gap = 0` whose cells separate is `Violated`.

### Order

`AtRest` takes `Separation`'s place in `CheckId::ALL`, after `UnprovenCoincidence`, among the subject-reading residents. Its findings follow the gathers' order: the product first, then `Product::assembles` in document order. Within one `⊔`, they follow the census's order of first mention, with carried findings after the `⊔`'s own, in gather order. `checks.rs`' module doc ("a body check's findings follow the placements' document order") is restated to that.

### What retires, and where each `AssemblyError` arm goes

| Today | After C |
|---|---|
| `AssemblyError::Product` | the gather's own refusal, raised by `product_recorded` to whoever gathers; in the registry, `ChecksError::Product` |
| `AssemblyError::Assemble` | `AtRestEvidence::NotGathered` |
| `AssemblyError::CarriedAtRest` | `AtRestEvidence::Carried`, one finding per carried row |
| `AssemblyError::AtRest` | `AtRestEvidence::{NoLane, Malformed, InsideArrival}` |
| `Assembly::interference`, `::carried_interference` | `AtRest` findings |
| `Assembly::{body, names, contacts}` | the product's (`Product`), unchanged |

- `assemble`, `assemble_gathered`, `verdict`, `gate_assembles`, `ahead_of_at_rest`, `Reported`, `Assembly`, `AssemblyError` and `AtRestFinding`'s refusal role are deleted. `at_rest_stage` becomes the resident's reading of one gather.
- `Route`, `CarriedInterference`, `CarriedAtRestFinding`, `CarriedFindings`, `Pinned`, `once_per_part_row`, `seam_verdict` and `AtRestRow` are the resident's and the lint's, and move to `checks/at_rest.rs`. Whether `assembly.rs` survives as a file is the lane's call.
- `seam_verdict` keeps its one departure, "the stage's overlaps cross beside its refusal": a part's could-not-look findings and interference both cross, and neither refuses the host.
- `CheckId::Separation`, `separation`, `mated_pairs`, `CheckEvidence::{NotSeparated, SeparationUnavailable}` and `ChecksConfig::separation` are deleted. `topo::SolidSeparation` stays in the kernel, and the census's own BVH pre-filter (`Candidates`) is the pair filter. The resident adds no box test of its own.

### Surfaces

- **pncad** re-exports `AtRestEvidence` and stops exporting `assemble`, `Assembly` and `AssemblyError` (`crates/pncad/src/document.rs`).
- **Viewer.** The landing runs the registry over the product it already gathered (`run_checks_on`), in place of `assemble_gathered` (`session.rs`). `AtRestBadge` becomes the resident's counts: loud interference, quiet interference, and could-not-look. The `badge` rows and `frame.rs`' badge site are restated. The badge keeps the product body, which no longer comes out of an `Assembly`.
- **Python.**
  - `pncad.assemble(doc, evaluation)`, `Assembly` and `AssemblyError` go. `InterferenceFinding` and `CarriedInterference` stay, read off the report's `AtRest` findings instead of off `Assembly`. `product_memo.rs` reads the product and the report.
  - `CheckId.AtRest` replaces `CheckId.Separation`, and `AtRestEvidence` is added. `run_checks` reports the findings, and each `CheckFinding` gains `quiet`.
  - `Doc.assemble` and `Node.assemble` (the operator) are untouched.
  - The `.pyi`, `tags.rs`, `check_payload.rs` and the binding census follow.
- **Tour.** `assembly.rs`' two `assemble` calls read the product and the report. The gallery's `advisory` counts `AtRest` findings, loud and quiet, instead of `Separation`.

### Docs

These change no ratified decision; each describes code this PR changes.

- `crates/editor-core/ASSEMBLY.md`. A5's opening and *Interference.* paragraphs become the row's text, restated per `⊔`. **A5 — The at-rest check:** "At each `⊔`, the census examines every pair of arrivals the census's pre-filter cannot prove apart, and decides each one apart, in contact, overlapping or undecided. A contact is recorded at the coincidence door; unless it is structural, the `unproven-coincidence` lint reports it, quiet under a holding `Gap = 0` between its two cells. An overlap is an `AtRest` interference finding, localised to the faces bounding it, or loud and unquietable when the intersection refuses. An undecided pair is an `AtRest` could-not-look finding, which refuses at no severity (DS6). Each finding is quiet or loud. Nothing refuses: a caller that wants a gate runs `enforce_checks` at `Error`." The where-in-code row, *A part's own census findings* (its gate wording), and the file's "the at-rest gate in the code still refuses" fence follow.
- `crates/editor-core/CHECKS.md`. The residents list loses `Separation` and gains `AtRest`. DS6's "Second resident SHIPPED: the product-separation check" paragraph and the waiver paragraph's `Separation` example are replaced by the at-rest resident, as row 91 approved. The resident's knob is `Severity` because assertions are its declarable exception.
- `crates/topo/README.md`. C6's "the editor's gate reads that verdict" and *At the assembly layer* read "the at-rest check".
- `docs/DESIGN.md`, State of the build: "A5's at-rest gate … with CHECKS DS6's product-separation resident" moves from what retires to what is built.
- `docs/guide/assembly.md`: the gate sections are rewritten.

### Sites

`checks.rs` (`CheckId`, `ALL`, `ChecksConfig`, `CheckFinding`, `CheckEvidence`, `run_checks_on`, `enforce_checks`, `door_rows`, `separation`, `mated_pairs`), `checks/at_rest.rs`, `assembly.rs`, `product.rs` (`Product`'s doc links to `assemble_gathered`), `lib.rs`; `pncad/src/document.rs`; `viewer/src/session.rs`, `frame.rs`, `app.rs`; `pncad-py` (`py/assembly.rs`, `py/checks.rs`, `product_memo.rs`, `check_payload.rs`, `tags.rs`, `errors.rs`, `pncad.pyi`, tests); `demos/tour/src/assembly.rs`, `gallery.rs`; the editor-core tests asserting `AssemblyError`, which the lane counts at dispatch.

## 5. What moves (C)

| | C |
|---|---|
| Census output, records, rows, interference findings | unmoved (the same functions, in the same order) |
| At-rest outcome | the gate is gone; every outcome is a report |
| Checks report | `AtRest` added, `Separation` gone; `CheckFinding::quiet`; at-rest rows the contact half quiets go quiet |
| `enforce_checks` | skips quiet findings and every finding no assertion could quiet (`never_refuses`) |
| Python `.pyi` and census | `assemble`/`Assembly`/`AssemblyError` gone; `CheckId.AtRest`, `AtRestEvidence` |
| Ids, wire, content keys, geometry | unmoved |

**No f64 geometry moves.** Every body digest, measured value, verdict bit and tour frame is bit-equal.

## 6. Test plan (each row names the runtime value that breaks it)

Rows 1–4 are A's and 5–10 are B's, built and kept as their PRs pinned them (`intent_s5_b_interference.rs` cites 5–10): `=` at f64, nothing else moves, `=` over a box, the window superset; interference reports, quiet by a one-sided bound, a second overlap stays loud, a pierce is interference, unlocalized, a failed requirement gates nothing. C's rows follow, in `crates/editor-core/tests/intent_s5_c_at_rest_check.rs`.

11. **A contact is one finding.** Two bodies resting face to face as operands of the product's `⊔`, placed by values, with no mate: one `unproven-coincidence` finding and no `AtRest` finding. *Breaks if* `AtRest` reports the contact (two findings for one row).
12. **Contact quiet under `= 0`.**
    - With `Gap(a.foot, b.top) = 0` over the row's two cells, the lint's finding is listed with `quiet == Some(assertion)`.
    - With `≥ 0` it is loud, and so with `≤ 0`.
    - With a `Gap` over another face pair of the same two bodies it is loud.
    - With a `Gap = 0` that is `Violated` (the bodies moved apart), there is no row and the assertion reports its own failure.
    - The same pair placed by a mate has no finding.
    - A flush pair of two coplanar faces of the two bodies, with `Gap = 0` over its two cells, stays loud.
    - A vertex-on-face row (a prism's apex vertex resting on a block's top) is quiet under a holding `Gap = 0` over its vertex and its face, and loud under `Gap ≥ 0`.
    - An edge-on-face row (the prism's apex edge lying on the top) is quiet under a holding `Gap = 0` over its edge and its face, and loud under `Gap ≥ 0`.
    - *Breaks if* the rule matches by body pair (the other-face row quiets), `≥ 0` quiets (the straddle reading), a mated pair is reported, the flush pair quiets, or the rule reads face cells only (a vertex or an edge row stays loud under `= 0`).
13. **A could-not-look finding never refuses.**
    - A `CensusUndecidable` pair between two arrivals is one could-not-look finding, reported at Warn and Error, and `enforce_checks` refuses it at neither.
    - A loud interference finding refuses at Error only.
    - A quiet one refuses at none.
    - *Breaks if* a could-not-look finding refuses at any severity, or a quiet finding counts.
14. **`Separation` has no successor of its own.** The heat sink reports one `AtRest` interference finding per fin and no `NotSeparated`. Two arrivals whose boxes overlap but which the census clears have no finding. *Breaks if* a box test is kept as a verdict (the cleared pair is reported).
15. **Per `⊔`.**
    - A `⊔` the product slot does not read, whose operands overlap, reports its interference under that `⊔`.
    - Two bodies of different spaces overlapping in coordinates have no finding.
    - A `⊔` whose gather refuses is one `NotGathered` finding, and the rest of the report stands.
    - *Breaks if* the resident reads only the slot's `⊔`, compares bodies across spaces, or a gather refusal fails the report.
16. **`A ⊔ A` refuses nothing.** D′3's `one_body_assembled_twice_is_an_interference_finding` reads today's `AssemblyError::AtRest`. After C, the same document gathers, and `AtRest` reports it loud. `[coincidence.booleans]` makes it an interference finding, so the lane records which verdicts the census gives (§7). *Breaks if* any severity but `Error` refuses it, or it reports a contact.
17. **Carried, once.** A part whose own `⊔` holds an interference that the part's assertion quiets, instanced twice into the host: one carried finding, quiet, by the part's assertion. A host `Gap` over picks of the instance's pieces quiets nothing. *Breaks if* the finding is reported twice, or the host's assertion quiets the part's finding.
18. **Not run at this scalar.** At `Dual64` the resident reports one `NotRun` finding. *Breaks if* the report is clean (DS6: a check that could not look says so).
19. **Python.** `pncad.assemble` is gone, and `run_checks(doc, ev).findings` holds the `AtRest` interference finding that row 5's document reported on `Assembly.interference`, with the same faces. `Doc.assemble` still authors a `⊔`. *Breaks if* the finding's sites differ (the move re-derived them).

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, `dsc_checks::the_registry_order_is_every_check`, `f6_variants!`.

## 7. Risks

- **The in-arrival check is noisy before the carry.** Until `a-placement-carries-its-sources-at-rest-result` lands, a `⊔` in another space that is placed into the world reaches the product as one arrival, and its own interference is re-found inside that arrival. Today that refuses `AtRest`. After C it is one `InsideArrival` finding per copy, beside the source `⊔`'s own finding. It is loud and refuses nothing, which is the safe direction, and the carry removes it (§10). C must not suppress it: the match is the carry's.
- **`Gap`'s carrier scope.** C5's `Gap` has same-kind arms only, so an interference or a contact on a carrier pair `Gap` has no arm for cannot be quieted. That is a frontier, not a defect: the finding stays loud.
- **A cross-space `Gap`.** "An assertion quiets only findings between bodies of its own space" rests on `Measure`'s door refusing a `Gap` whose two selections lie in different spaces (`each-space-runs-its-census-between-arrivals` asks this be checked; `measure.rs`, D′1's `space_faults`). C's contact half matches cells by the `⊔`'s reads, so it never matches such an assertion either way.
- **`A ⊔ A` may not read as interference.** D′3 pins that the census over two arrivals of one body finds something (`AssemblyError::AtRest`), not that `partition` takes it as an interference verdict. If the census reports those verdicts as coincident faces or undecided pairs, C maps them to could-not-look findings, which are loud and never refuse. That is short of `[coincidence.booleans]`'s interference finding. The lane measures which it is, and if it is not interference, it files the census's reading as its own row rather than special-casing a body read twice in the resident.
- **MAJOR 1 of D′3.** If D′3 lands with the census moved into `wire_assemble`, the resident reads each `Assemble` node's value. If it lands without, the resident reads the gathers. The re-grep at dispatch says which.

## 8. What C rewrites (ratified text quoted)

DESIGN governs where a companion clause disagrees. Each rewrite below describes code C changes and changes no decision, so it lands with C (CLAUDE.md, "Git workflow").

- **ASSEMBLY A5**, opening: "**A5 — The at-rest gate.** `assembly::assemble` gathers the product …" and "`AssemblyError::AtRest` is a verdict against the document". Rewritten as §4's A5 text.
- **ASSEMBLY A5 *Interference.***: "reported on `Assembly::interference` and never refused" becomes "an `AtRest` finding".
- **ASSEMBLY**, the top-of-file fence: "The at-rest gate in the code still refuses. Those clauses (A3 and A5) …". A5 leaves it.
- **CHECKS DS6**: "**Second resident SHIPPED (2026-08-29): the product-separation check.** … Its severity knob is `Advisory` (Off/Warn) …". The resident retires (row 91), and the paragraph is replaced by the at-rest resident's.
- **topo C6**: "Between two copies at rest the editor's gate reads that verdict". It becomes "the at-rest check".
- **`checks.rs` module doc**: "a body check's findings follow the placements' document order". It changes with the code.

## 9. The in-scope rows

- **`a-failed-requirement-refuses-the-whole-product`** (closed). C must not regress it: the quieting rule reads verdicts, and a poisoned assertion quiets nothing and gates nothing (rows 10 and 12).
- **`stage5-contact-finding-and-the-unproven-row-report-one-contact-twice`** (closed): `AtRest` has no contact finding (§1).
- **`an-assertion-quiets-an-operations-unproven-row`** and **`gap-reads-vertex-and-edge-cells`** carry the contact half past C (§11, Q1 and Q2).
- **`separation-quiets-only-first-rung-rows`** (rides with C): `Separation` and its suppression retire, so it closes with C.

## 10. Boundaries

- **D′3** (`assemble-is-an-operator-and-the-product-is-a-slot`) owns `Node::Assemble`, the product slot, `assemble_gathers`, the census at each `⊔` with one group per arrival, picks, and the `AssemblyError::Assemble` arm C retires. C reads what it leaves.
- **The carry** (`a-placement-carries-its-sources-at-rest-result`, INTENT, parked on D′3 and E's core) makes every placement carry its source's records and census findings, generalising `PartValue`'s carry. C's readers of carried findings (`CarriedInterference`, `CarriedAtRestFinding`, `once_per_part_row`) are what it generalises. C keeps them one type, so the carry adds sources and no reader. Until it lands, a placed non-instance `⊔`'s findings are re-found inside the copy (§7).
- **The census per `⊔`** (`each-space-runs-its-census-between-arrivals`, INTENT, parked on the carry) builds the rest of `[assertions]`:
  - the confirm-and-match inside an arrival;
  - findings named by the reads they entered the `⊔` through;
  - reporting once at the deciding `⊔`.

  C's `InsideArrival` arm is that check's output, whatever shape the unit gives it. C's `FaceSite` is read off the reads D′3 already keys. If that unit re-keys a site, C's matching follows the key and not the reverse.
- **The in-arrival check retires** (`the-in-arrival-census-check-retires`, INTENT, parked). When the census reads nothing inside an arrival, `AtRestEvidence::InsideArrival` has no source. That unit deletes the arm with its check.
- **Stage 4** is built (H #5300, I #5637). Declared contacts, the mate mint and `ContactClass` are gone, and the lint reads the at-rest rows. C adds only the contact half's matching to the lint, and the lint's door is unchanged.
- **`d10-one-way-to-say-intent-is-unbuilt`** waits on C. After C, State of the build lists A5's gate and `Separation` as built.

## 11. Decided questions

The first cut's FORKs (S5-1 to S5-5) are settled by fork-log rows 91 and 98 and by PR 4666's cell reading. The four questions below decide how far C's contact half reaches. D10 settles the final state of each, so what remains is sequencing, which is the ATREST orchestrator's (2026-10-26).

**Q1 — Which unproven rows the contact half quiets.** The final state is every unproven row. `[intent.checked]` says of every unproven coincidence that "an assertion at its site quiets it", and that includes a boolean's glue, a split's ON verdict and a blend's rows. C builds the at-rest rows, with the rule written over a row's two cells so that it applies unchanged to an operation's row. The operation rows' cells are named in the deciding node's input tables, and selecting them is `an-assertion-quiets-an-operations-unproven-row`.

**Q2 — Cells below a face.** The final state is every dimension: `[assertions.quiet]` says "at whatever dimension they are". `Gap`'s door admits a face, an edge or a vertex (`MeasurePrimitive::admits`, `measure.rs`; `gap-reads-vertex-and-edge-cells`). So C quiets every row whose two cells a holding `Gap` reads, at any dimension, with the rule written cell-generic. A row over cells `Gap` refuses stays loud (`gap-reads-no-curved-cell-below-a-face`).

**Q3 — A flush pair.** A `Gap` assertion never quiets one. `[assertions]` says a flush pair is "no contact, but a coincidence … a finding unless established, by a flag mate or by the replay", and `[assertions.quiet]` quiets "a contact". Ev, on PR 5555: "the proper way to specify this would be a mate that additionally indicates that it is flush".

**Q4 — The lint's knob.** C leaves `ChecksConfig::unproven_coincidence` as `Advisory`. It is re-read against CHECKS DS3 ("off / warn / error, or off / warn where it ships no way to quiet a finding") once both rows above land. A flush pair is never quietable by an assertion, so a lint-wide `Error` needs a rule this spec does not write.
