# SPEC: INTENT stage 5 — assertions and the at-rest lints

Scope: D10's **Assertions** paragraph (`docs/DESIGN.md` §D10), as `work/intent/plan.md` stage 5 states it:

- `Assert` with `=`;
- the quieting rule;
- the at-rest census as a check;
- interference as its own finding.

D10's paragraph, verbatim, since every unit below builds one of its sentences:

> **Assertions.** `Assert { measure, relation, bound }` (`≤`, `≥`, `=`, the bound a variable) checks and never places. At rest the census examines the copies of each space pairwise, and nothing it finds refuses. Contact between copies is an `unproven-coincidence` finding unless it is structural (a mate-placed face is); an overlap of their material is an interference finding; and a pair the census has no lane for is a finding too, saying it could not look, so no outcome of the census is silence. A contact or interference finding observes the sign of the gap (CONTACT-DESIGN C5) between two copies: a contact is `g = 0` at two cells, an interference `g < 0` over one connected overlap of their material. Holding assertions quiet a finding when they say the same: each reads a `Gap`'s output directly, over an opposed pair of faces of the two copies, and admits only values of the finding's sign (`= 0` for a contact; `≤ b` or `= b` with `b` negative for an interference). A contact is quiet when such an assertion's two faces are the two cells the census found coincident; an interference, when the assertion's two faces bound the overlap and every face bounding it lies between the carriers of an asserted pair. An assertion speaks for nothing else, so a new contact or overlap anywhere else is loud until something says otherwise, and an overlap the kernel cannot bound is loud and nothing quiets it. The bodies of a pattern's `Bodies` are examined like any others, and an assertion about each of them is written once, as a `map` over the `Bodies`: one assertion per member, each reading the one bound variable, as many as the pattern's `Count`. That quiets each member against another copy; an overlap between two members (neighbours in a ring) is quieted by a map over pairs of members, adjacent pairs or all pairs.

**Rulings this spec builds on.** Ev's rule that the redesign governs the text it replaced applies throughout.

- **FORK-5** (#4218): a `Measure` defines an *observed* variable, read only by an assertion. Stage 2's unit D builds it: one `Measure` is one primitive defining one observed scalar, and `Assertion { value: S, bound: S, dir }` reads a scalar variable (`docs/INTENT-STAGE2-SPEC.md` §5). This stage builds on that shape. "The same measure" (D10) is well defined only once a measure is one primitive, which is why every unit here waits on D.
- **FORK-2b** (#4220): the product is the world, every copy a `PlaceInWorld` defines (stage 2 C). "Between copies" means between those copies' outputs.
- **FORK-3** (#4222): a `Face`/`Edge` variable is a selection of a `Body` variable (stage 2 E). "The same site" is spelled as selections of the copies' `Body` outputs.
- **The isosceles-mitre answer** (Ev, 2026-10-06, `work/intent/isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box.md`): whether a value-decided coincidence is structural is stage 4's door. This stage reads the door's verdicts and never decides structure itself.

**Baseline.** The grep is at main `047d10d5d` (stage 1 finished, stage 2 A merged at #4295, stage 2 B in flight on `intent/s2-b-reads`). Every unit here lands after stage 2 D, and C after stages 3 and 4, so `file:line` citations are where each site is **today**. Each dispatching orchestrator re-greps at its unit's trigger and states the drift. Stage 2 C, D and E rewrite `product.rs`, `measure.rs`, `node.rs`'s `Measure`/`Assertion` arms and `eval/wire.rs`'s `wire_measure`, so line numbers there will move.

The state of the mechanisms at the baseline:

- **Assertions.** `Node::Assertion { measure: RecipeNodeId, bound: S, dir: AssertionDir }` (`node.rs:2796`). `AssertionDir` is `{AtLeast, AtMost}` (`measure.rs:799`). `decide_assertion` (`measure.rs:1084`) decides `measured − bound` (or its mirror) through `ASSERT_BOUND`. The verdict is `AssertionVerdict::{Holds, Violated, Unevaluated}` (`measure.rs:827`). Nothing reads a verdict except reports (MC `mc.rs:469`, stackup `stackup.rs:1047`, the box driver's `assertion_at`, `drive.rs:2195`).
- **The at-rest gate** (ASSEMBLY A5). `assembly::assemble` (`assembly.rs:1171`) gathers the product and calls `verdict` (`:1273`), which refuses on carried and own mint refusals, then runs `T::gate_at_rest_declared` (the tier-3′ census with the mates' declarations) and refuses `AssemblyError::AtRest` on any finding attributed against the document, or `Uncertified` when every finding is a declined declaration.
- **Interference.** The census's containment arm (`topo/src/census.rs:5634`) pushes `ValidationError::InstanceInterference { outer, inner, witness }` (`topo/src/validate.rs:1753`). A5 attributes it `Unattributed` (`assembly.rs:1791`), so it refuses `AtRest`. A transverse pierce between two solids is `UndeclaredContact { EdgeFacePierce }`, "categorically undeclarable at rest" (`census.rs:28`).
- **The check registry** (DISCIPLINES DS6, `checks.rs`). There are three residents (`CheckId`, `:60`; `ALL`, `:134`). `Separation` reports pairs of gathered solids from different roots that `topo::SolidSeparation`'s boxes cannot prove apart, and suppresses a pair whose contact the product declares (`separation`, `:1355`; `declared_pairs`, `:1424`). It is `Advisory`: it cannot refuse, because it ships no waiver vocabulary (DS6).
- **Callers of the gate.** The viewer's at-rest badge (`viewer/src/session.rs:1413`, `badge` at `:3404`, `AtRestBadge` at `:757`), Python (`pncad-py/src/py/assembly.rs:853`, `product_memo.rs:184`) and the tour (`demos/tour/src/assembly.rs:937`, `:1372`). Export does not call it.

## 0. Ordering: three PRs, each green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Representation step it completes | Goldens |
|---|---|---|---|---|
| A | `an-assertion-relates-by-equality` | `AssertionRelation { AtLeast, AtMost, Equal }` replaces `AssertionDir`; the `relation` field; the `=` arm of the decision in every lane | **`Assert { measure, relation, bound }` with `=`** | assertion-bearing files re-blessed (the wire field is renamed); verdict bits of every existing assertion unmoved |
| B | `interference-at-rest-is-a-finding` | the at-rest gate partitions the census's interference verdicts out of its refusal; an `InterferenceFinding` per overlap between two copies; the quieting rule's one home, with its interference half | **interference is a finding of its own** | documents that refused only on interference now gate `Ok` with findings; census goldens unmoved |
| C | `the-at-rest-census-is-a-check` | a registry resident `CheckId::AtRest` replaces A5's gate; stage 4's at-rest contact findings join it; the quieting rule gains its contact half; `Separation` retires into it (FORK-S5-5) | **the at-rest census is a check** | the viewer badge, the tour gallery's check rows and Python's `assemble` restated |

**Why this order:**

- **A and B do not wait on stages 3 and 4.** A extends the arm stage 2 D rewrites and touches nothing of spaces or coincidences. B partitions a verdict the census already decides (a vertex inside another copy's material, a transverse pierce) and needs only world copies (C), single-primitive measures (D) and selections (E) to state a site. They can be built in parallel, and both can be dispatched as soon as stage 2 E is merged.
- **A before C.** C's contact half is quieted by an assertion that the gap is zero, and `=` is A's.
- **B before C.** B is the smaller step that proves the quieting rule on the one finding that exists before stage 4. C then moves a working rule and a working finding into the registry, rather than building both there at once.
- **C waits on stages 3 and 4.** Its contact findings are stage 4's `unproven-coincidence` records at rest, and stage 4 retires the declared seats, `ContactClass` on mates and A5's hard error on an unattributed contact, which are the rest of today's gate. It checks per space, and spaces are stage 3's. Built before them, C would carry a declaration-attribution half that stage 4 deletes, and a space walk that stage 3 rewrites.

**Each intermediate state is a whole representation:**

- After A, an assertion states one of three relations, and nothing else changes.
- After B, the gate refuses everything it refused before except interference, which it reports, quiet or loud, on the `Assembly`.
- After C, there is no at-rest gate. The census is a check, and its findings are contact and interference, each quiet or loud.

**Rejected:**

- **A+B as one PR.** They share no site, and a reviewer would have to separate an enum change across 46 files from a census partition.
- **B inside C.** That puts the only finding testable before stage 4 behind stage 4, and lands a new finding, a new rule and a new resident in one diff.
- **C before stage 4, with the declared attribution kept.** That builds `Attribution`'s declined/refuted/unattributed split into a check resident one stage before stage 4 deletes it (D10's retire list: the declared-contact seats and "ASSEMBLY A5's hard error on an unattributed contact").

## 1. Final shapes (after C)

**Assertions** (`measure.rs`, `node.rs`):

- `AssertionRelation { AtLeast, AtMost, Equal }`, symbols `>=`, `<=`, `=`. `AssertionDir` is gone.
- `Node::Assertion { value: S, relation: AssertionRelation, bound: S }`. `value` reads an observed scalar (stage 2 D), and `bound` reads any scalar variable of the same dimension (D10: "the bound a variable").
- **The decision.** The comparand of `=` is `measured − bound`, through the one `ASSERT_BOUND` site.
  - Zero holds, and either definite sign is `Violated`.
  - The sliver band is `Unevaluated { Indeterminate }`, as for the other two relations.
  - Over a window-superset enclosure (`MinClearance` at `Interval`), `=` needs **both** ends admitted to hold, and either one to be violated. So `MinClearance = b` never holds over a one-sided window. It says so with `WindowSuperset`, never guessing.
- **In the analysis lanes, `=` is decided pointwise, as the other relations are.**
  - The box driver holds it on a leaf only when the margin's enclosure lies within the Zero band. A structural zero is proven there by E12's symbolic lane, and a value-decided equality refuses or bisects to budget.
  - Monte Carlo counts a draw as holding when the draw's margin decides Zero.
  - Stackup reports the margin's enclosure as it does today.
  - That is D10's "the analysis lanes … see such a coincidence as the point it is", applied to a written equality. No lane treats `=` specially.

**Findings at rest** (`checks.rs`, a new `checks/at_rest.rs`):

- A **copy** is a `Body` output of a world placement (stage 2 C; stage 3 respells the placement, not the output). A **face site** is `(copy: VarId, face: StableName)`, the face as the copy's own name table spells it.
- `AtRestFinding` is one of two:
  - **`Contact { a: FaceSite, b: FaceSite }`.** This is stage 4's value-decided coincidence of two faces of two copies at rest, which the door did not prove structural (a mate-placed face pair is structural, D10). Stage 4 records it and its lint judges it. This stage reports it as an at-rest finding and decides whether it is quiet.
  - **`Interference { a: VarId, b: VarId, overlap: OverlapSite }`.** Two copies whose materials overlap, decided by the census: a vertex strictly inside the other's material, a transverse pierce (`EdgeFacePierce`), or a same-side in-plane crossing. `OverlapSite` is per FORK-S5-2 (recommended: one finding per connected component of the copies' intersection, named by the face sites bounding it).
- **Quiet or loud.** Each finding carries `quiet: Option<RecipeNodeId>`, the assertion that quiets it. A quiet finding is still listed, naming its assertion, so "checked and intended" stays distinct from "not checked" (DS6). It takes no severity, and `enforce_checks` never refuses it.
- **The quieting rule**, in one home, `at_rest::quieted_by(finding, doc, evaluation) -> Option<RecipeNodeId>`. A finding is quiet exactly when some assertion meets all of the following:
  1. it is **on the same measure at the same site**: its `value` reads, directly, the output of one `Measure` whose primitive is the finding's measure, over selections of the finding's two copies naming the finding's site (FORK-S5-1);
  2. its verdict **in the same evaluation** is `Holds`. `Violated`, `Unevaluated` or a poisoned assertion quiets nothing;
  3. **its admitted set does not straddle zero** in the sense FORK-S5-3 settles. The recommendation: the admitted set `{x : x relation bound}` lies inside the finding's own stratum of C5's signed gap, which is `{0}` for contact (so only `= 0`) and `(−∞, 0)` for interference (so `≤ b` or `= b` with `b < 0`).

  It reads the assertion's verdict, never its measure's value. So a failed requirement quiets nothing and gates nothing (`a-failed-requirement-refuses-the-whole-product`'s acceptance is kept).
- **The resident.** `CheckId::AtRest` reads the subject product (`reads_subject() == true`). It runs the tier-3′ census per space (stage 3's spaces, copies compared only within one), collects stage 4's at-rest contact records and the interference verdicts, and quiets each finding.
  - **Where the census has no lane** (`CensusUnsupported`, `CensusUndecidable`, `CensusEscalated`, `CensusLaneUnsupported`) and where the body is malformed (tiers 1–3), the resident cannot look. It says so as a finding with its own evidence, per DS6's "a check that could not look says so as a FINDING".
  - Severity and the refusing path are FORK-S5-4's. The recommendation is `Severity` (Off/Warn/Error, default Warn), legal under DS6 because an assertion is this resident's waiver vocabulary. `enforce_checks` refuses a loud contact or interference finding at Error. A could-not-look finding never refuses, whatever the severity: it is reported loud and counted apart, because that the resident could not judge is not something the document said, and no assertion answers it (DS6).
- **What retires.**
  - `assembly::assemble` and `assemble_gathered`, with `AssemblyError::AtRest`. The other arms are stage 4's (`Mint`, `CarriedMintRefusal`, `Uncertified`) or stage 3's (`Space`).
  - `Assembly` as a type: its body, names and records are the product's.
  - `CheckId::Separation` and `CheckEvidence::NotSeparated` (FORK-S5-5).
  - The viewer's `AtRestBadge::{Certified, Refused}`, which becomes the resident's counts: loud, quiet and could-not-look.

**Who reads a finding.** The checks report, the viewer, Python and the tour. No door, edit, evaluation or export reads one. A finding is a report, as an assertion's verdict is (E10).

## 2. PR A — `an-assertion-relates-by-equality` (cost M; ~50 files, 600–900 lines, mostly a rename)

1. `measure.rs:799` `AssertionDir` becomes `AssertionRelation`, gaining `Equal` (`symbol() == "="`). The field `dir` becomes `relation` on `Node::Assertion` (`node.rs:2809`, as stage 2 D leaves it).
2. `decide_assertion` (`measure.rs:1084`) gains the `Equal` arm per §1.
   - The comparand is `measured − bound`. `Ok(Zero)` holds if `certified.admits` both ends. `Ok(Positive | Negative)` is violated if it admits the end that decided.
   - `ASSERT_BOUND_DECISION`'s lever, "move the bound", is unchanged.
   - `UnevaluatedReason::WindowSuperset` names which end refused.
3. **The content key** (`eval/mod.rs:5753`): `Equal => 3`. The `AtLeast`/`AtMost` bytes are unchanged, so no existing key moves.
4. **Persistence.** The `Node` serde form carries `relation` with the variant tag `Equal`. A file with `dir` refuses `Unreadable` with the regenerate recourse (VR9's door). The mint preimage is the stored node (`mint.rs` module doc), so every id minted after a document's first assertion insert moves, digests included. Re-bless with `M4_PR6_BLESS_GOLDEN=1` and list the moved files in the PR.
5. **Analysis lanes.** These need no new code, only rows. The box driver (`drive.rs:2195`), MC (`mc.rs:469`) and stackup (`stackup.rs:1047`) read the verdict, so `=` reaches them through `decide_assertion`. Their digests encode the verdict arm, not the relation.
6. **Surfaces.**
   - pncad re-exports `AssertionRelation`.
   - Python: `AssertionDir` becomes `AssertionRelation` with `Equal` (`py/measure.rs:290`), the `dir=` keyword becomes `relation=`, and `pncad.pyi:2212`, `tags.rs` and the census follow (19 Python test sites).
   - The viewer's assertion rows (`pane/features.rs:793–803`) render `=`.
   - The tour's three uses (`plate.rs`, `chain.rs`, `projectbox.rs`) are renamed.
7. **Docs.** ERROR-DESIGN E10's persisted line is rewritten to D10's shape (§8). E3's rewording is stage 2 D's (`error-design-e3-calls-a-measure-a-sink`). If D merges without it, A carries it.

**Sites.** `AssertionDir` appears at 148 sites in 46 files (editor-core src 10, editor-core tests 37 files, viewer 6, pncad-py 6, tour 2, pncad tests 1). The rename is a sed over `AssertionDir` and `dir:` in `Node::Assertion` literals, with compile-driven residue.

**Goldens.** The ids in every `.pncad`/`.cad` holding an assertion move: the corpus `measured_web`, `tolerance` and `mcplate` documents, `tests/golden/golden.cad` if it holds one (re-grep), and the tour's plate files. Every assertion verdict's bits, every MC and stackup digest and every geometry digest are unmoved, which is the PR's check (test 2).

## 3. PR B — `interference-at-rest-is-a-finding` (cost H; ~45 files, 1.5–2.5k lines)

**Kernel: unchanged.** `topo` keeps `ValidationError::InstanceInterference` and the pierce arms as the census's typed verdicts. Whether an overlap refuses is the caller's policy, and inside one node's body (a pattern's `Bodies`, a multi-solid boolean result) an overlap stays the defect it is (`work/restfront/tier-3-admits-two-solids-of-one-body-whose-material-overlaps.md`). The partition is the at-rest door's: D10 states interference as a finding **at rest, between copies**.

**The gate** (`assembly.rs`):

- `verdict` (`:1273`) partitions `T::gate_at_rest_declared`'s errors into **interference verdicts** and the rest. An interference verdict is any of the following between solids of two different copies:
  - `InstanceInterference`;
  - `UndeclaredContact` whose contact is `EdgeFacePierce`;
  - an `EdgeEdgeCross` the side test read `SameSide`, which today reaches the refusal only as rendered witness text (`topo/README.md`, "Three-valued side verdict"). B gives the census a typed field for it (one `topo` field, the README's "no bool may stand there" honoured).
- The rest keep today's attribution and refusal order (`Mint`, `CarriedMintRefusal`, `AtRest`, `Uncertified`) until stages 3, 4 and C.
- **The overlap site** (FORK-S5-2, recommended (b)). For each interfering pair of copies, the gate intersects the two copies' bodies (`topo` boolean intersection, the f64 lane at the gate's ε).
  - Each connected solid of the result is one `InterferenceFinding`, named by the face sites of both copies that bound it. The result's faces carry the operands' names, `FromA`/`FromB` (N-machinery).
  - An intersection that refuses yields one finding for the pair with `overlap: Unlocalized(refusal)`. That finding is loud, never refuses at any severity, and cannot be quieted (test 9; DS6's frontier). A refusal as a sliver means the overlap is about ε thick, which the representation cannot tell from contact; a press fit well above ε intersects.
- `Assembly` gains `interference: Vec<InterferenceFinding>` in census order. `assemble` returns `Ok` when interference findings are all there is.

**The quieting rule** (`checks/at_rest.rs`, new; §1): `quieted_by` with its interference half.

- An assertion on a `Measure` whose primitive is `Gap { outer, inner }` (C5's signed gap) quiets a finding when all of these hold:
  - its two selections are of the finding's two copies' `Body` outputs;
  - each names a face bounding the finding's overlap;
  - its verdict holds;
  - its admitted set is inside `(−∞, 0)` (FORK-S5-3).
- `MinClearance` and `Distance` are unsigned and quiet no interference (FORK-S5-1).
- The rule reads `Doc::reads` (stage 2 B) to go from an assertion to its measure, and the measure's slots to its selections (stage 2 E). It never evaluates anything.

**Surfaces.**

- `Assembly::interference` is re-exported.
- Python: `Assembly.interference` and `InterferenceFinding` (`a`, `b`, `faces`, `quiet_by`), with the census and the `.pyi` following.
- The viewer badge gains a third state, `Certified { minted, interference: (loud, quiet) }`. `badge` (`session.rs:3404`) keeps the body.

**Docs** (D10 governs; §8 quotes each):

- ASSEMBLY A5 *Interference.* is rewritten.
- topo C6's invariant sentence ("an undeclared interference is always a typed error") is rewritten at the at-rest door. The kernel sentence stays for one body.
- MATE-4B-CROSSING's "`EdgeFacePierce` stays categorical" is rewritten: at rest between copies a pierce is interference evidence.

**Sites.** `assembly.rs` (`verdict`, `attribute` `:1615`, the `Unattributed` list `:1764–1791`, `Assembly` `:458`), `topo/src/census.rs` (the `SameSide` field), a new `checks/at_rest.rs`, `viewer/src/session.rs`, `pncad-py` (`py/assembly.rs`, `tags.rs:3098`, census), the tour (`assembly.rs`).

**Tests that move.** The rows asserting `AssemblyError::AtRest` on an interference: `refusal_concision_at_rest.rs`, `p2_gauge_offsets_and_spaces.rs` and the gallery heat sink, whose fins sit 1/16 inside the base by its own design. Each now reads one interference finding per fin. `bool4*` and `h14` in `topo/tests` are unmoved, because the kernel is unchanged.

**Goldens.** The `perf12_census_*` files are unmoved (kernel). Tour frames are unmoved (geometry). Gallery rows whose gate refused on interference now gate `Ok`, and the PR lists them.

## 4. PR C — `the-at-rest-census-is-a-check` (cost H; ~60 files, 2–3k lines)

Lands after stages 3 and 4. What it consumes from them is stated in §10, and its sites are re-grepped at its trigger.

**Kernel, document layer.**

- `CheckId::AtRest` joins `ALL` (`checks.rs:134`). `reads_subject() == true`, and `kind() == Certified`: the census's decided verdicts are theorems, and its undecided ones are could-not-look findings.
- The resident body is today's `verdict`, minus everything stages 3 and 4 retired:
  - per space, it runs the census over the product's body with the records stage 4 leaves;
  - it maps stage 4's at-rest contact records to `Contact` findings and B's interference verdicts to `Interference` findings;
  - it applies `quieted_by`, now with the contact half: a holding `Gap = 0` or `Distance = 0` over the finding's two face sites (FORK-S5-1, FORK-S5-3);
  - it reports no-lane and malformed-body verdicts as could-not-look findings.
- **Severity** per FORK-S5-4. The recommendation is `Severity`, default Warn, with `Error` legal because assertions are the waiver vocabulary DS6 requires, and with its staleness direction for free: an assertion whose finding goes away still checks its own measure, and a `Gap = 0` whose copies separate is `Violated`.
- `assemble`, `assemble_gathered`, `Assembly` and `AssemblyError::AtRest` are deleted. Callers take `product::product_recorded` plus `run_checks_on`.
- `checks.rs`' module doc ("findings follow root-list order") is rewritten to "placement order".
- **`Separation` retires into the resident** (FORK-S5-5). Its box test, `topo::SolidSeparation`, becomes the resident's pair pre-filter: a pair the boxes prove apart has no finding, and every other pair goes to the census. The resident's findings are the precise ones.

**Surfaces.**

- **Viewer.** `AtRestBadge` becomes a counts badge (loud, quiet, could-not-look) over the resident's report. `session.rs:1413` runs the registry instead of `assemble_gathered`. The badge rows (`AtRestBadge`, 37 sites in 10 files) are restated.
- **Python.** `pncad.assemble` goes. `Doc.checks()` gains the resident, `CheckId.AtRest` with the finding classes is added, and `product_memo.rs:184` reads the report.
- **Tour.** The gallery's `Report::Separation(n)` rows (`gallery.rs:136–296`) become `Report::AtRest { loud, quiet }`. `assembly.rs:937`, `:1372` and `:1540` read the report. `:1540`'s Tangent-mate refusal is stage 4's to move, because `ContactClass` is gone by then.

**Docs.** ASSEMBLY A5 is rewritten as "the at-rest check". DISCIPLINES DS6's second-resident paragraph and its grade table follow FORK-S5-5, and the waiver paragraph follows FORK-S5-4. `docs/guide/assembly.md`'s gate sections are rewritten.

## 5. What moves

| | A | B | C |
|---|---|---|---|
| Node ids, pinned hex | after the first assertion insert (the wire field) | — | — |
| Wire | `relation`, `Equal` | — | — |
| Content keys | `Equal => 3` only | — | — |
| Assertion verdict bits, MC and stackup digests | unmoved | — | — |
| Kernel census output | — | one typed `SameSide` field | — |
| At-rest outcome | — | interference-only refusals become `Ok` with findings | the gate is gone, and every outcome is a report |
| Checks report | — | — | `AtRest` resident added, `Separation` gone |
| Python `.pyi` and census | `AssertionRelation` | `Assembly.interference` | `assemble` gone, `CheckId.AtRest` |

**f64 geometry does not move in any PR.** Every body digest, measured value, verdict bit, solved pose and tour frame is bit-equal. B computes an intersection to localize a finding, and that intersection is a report, built and dropped, never a body anything reads.

## 6. Test plan (each row names the runtime value that breaks it)

1. **(A) `=` decides at f64.**
   - A `Distance` measure of 10 mm under `= 10 mm` is `Holds`, and under `= 10.001 mm` is `Violated { measured: 0.010, bound: 0.010001 }`.
   - A margin inside the sliver band is `Unevaluated { Indeterminate }`.
   - *Breaks if* `Equal` reuses one of `AtLeast`'s ends (`10 = 9` holds) or the band arm is lost (a margin of ε/2 is called `Violated`).
2. **(A) Nothing else moves.** Every corpus assertion's `AssertionVerdict` bits, the `mcplate` MC holding counts, the `tolerance` stackup digest and every body digest are bit-equal pre/post A. *Breaks if* the content-key byte of `AtLeast`/`AtMost` changed (a memo miss is visible as a re-evaluation count), or the comparand of an existing relation flipped.
3. **(A) `=` over a box.**
   - On `ParamBoxVerdict`'s interval lane, `w − w = 0` (one variable read twice) holds with E12's symbolic dial on.
   - Two toleranced typed `5 mm` under `a − b = 0` do not hold: the leaf is `Unevaluated`, and the driver bisects to `Budget`.
   - *Breaks if* `=` is special-cased to hold on the nominal (the second leaf holds).
4. **(A) The window superset.** `MinClearance = 2 mm` at `Interval` is `Unevaluated { WindowSuperset }`, never `Holds`. *Breaks if* `Equal` admits from one end.
5. **(B) Interference reports.**
   - The gallery heat sink (fins 1/16 inside the base) gates `Ok` with one loud `InterferenceFinding` per fin.
   - Each finding's overlap is bounded by faces including the fin's bottom face and the base's top face.
   - Pre-B the census refused the same product `InstanceInterference` (the `perf12_census_*` goldens' heat-sink lines).
   - *Breaks if* the partition misses `InstanceInterference` (still a refusal), or a fin's finding names no face (localization dropped).
6. **(B) Quiet by a one-sided bound.**
   - A pin pressed into a bore (pin radius 5.01 mm, bore 5 mm), with `Gap(bore, pin) ≤ −0.005 mm` over selections of the two world copies: the finding is listed with `quiet == Some(assertion)`.
   - With `≤ 0` it stays loud (FORK-S5-3).
   - With the assertion over the **unplaced** bodies' faces it stays loud (FORK-S5-1: not the same site).
   - With `Gap ≤ −0.02 mm` (`Violated`) it stays loud.
   - *Breaks if* the rule reads the measure's value instead of the verdict (the violated row quiets), or matches by body instead of copy.
7. **(B) A second overlap of the same pair stays loud.** The same pin and plate, with the pin also cutting into a second feature of the plate. There are two findings, and the bore assertion quiets only the one its faces bound. *Breaks if* the site is the copy pair (FORK-S5-2 (a)): both go quiet.
8. **(B) A pierce is interference.** A tilted brick whose edge pierces a plate face between two copies, with no vertex inside (BOOL-4 R2's construction) is one interference finding, not `AtRest`. *Breaks if* `EdgeFacePierce` between copies still routes to `UndeclaredContact`'s refusal.
9. **(B) Unlocalized.** An interfering pair whose intersection refuses (a sliver at ε, or a curved pair the join has no arm for) gives one finding with `overlap: Unlocalized`, loud, which no assertion quiets and no severity refuses. *Breaks if* a refused intersection drops the finding (the overlap vanishes from the report, which C6's "never silently passed" forbids), or an assertion quiets it.
10. **(B) A failed requirement gates nothing.** The pin-and-bore document whose quieting assertion's measure names a face that vanished (a failed select): `product()` is `Ok`, the finding is loud, and the assertion reports its own failure. *Breaks if* a poisoned assertion poisons the gate.
11. **(C) Contact quiet under `= 0`.**
    - Two copies resting face to face, placed by values rather than a mate (stage 4 records the coincidence as unproven): one loud `Contact` finding.
    - With `Gap(a, b) = 0` over the two face sites it is quiet, and with `Gap ≥ 0` it is loud.
    - The same pair placed by a mate has no finding (structural).
    - *Breaks if* `≥ 0` quiets (the straddle reading), or the mate-placed pair is reported.
12. **(C) A could-not-look finding never refuses.**
    - A `CensusUndecidable` pair (curved × planar within reach) is a could-not-look finding, reported at Warn and Error, and `enforce_checks` refuses it at none.
    - A loud interference refuses only at Error.
    - A quiet one never refuses.
    - *Breaks if* a could-not-look finding refuses at any severity, or a quiet finding counts.
13. **(C) Separation's successor.**
    - The heat sink's five `Separation` findings become five `Interference` findings, and no `NotSeparated` remains.
    - A pair the boxes cannot separate but the census clears has no finding.
    - *Breaks if* the box test is kept as a verdict (the cleared pair is reported).
14. **(C) Per space.** Two unplaced groups in their own spaces overlapping in the world's coordinates have no finding between them (stage 3's spaces). *Breaks if* the resident runs one census over every copy.
15. **(A–C) Python.** `relation=AssertionRelation.Equal` round-trips, `Assembly.interference` (B) and then `doc.checks().at_rest` (C) read the same finding, and the census and `.pyi` agree.

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, `dsc_checks::the_registry_order_is_every_check`, `f6_variants!`.

## 7. Risks

- **B's intersection cost and soundness.** The intersection runs only on pairs the census already found interfering, and at the gate's ε. It can refuse where the census decided, for example at a sliver or on a curved arm the join lacks. Test 9 pins the refusal as a loud, unquietable finding, which is the safe direction. If it is measured too slow on the heat sink at 160 fins, FORK-S5-2's (c), localizing by the census's witnesses, is the fallback.
- **`=` over a box needs the symbolic lane.** Without E12's dial, a structural `=` refuses on every leaf (interval dependency), so a CI row gating `=` over a box must run the symbolic lane. Test 3 pins both.
- **The quieting rule reads assertions from a report path.** This is the first time anything at rest reads an assertion. The rule reads verdicts only, and an assertion it reads is never a reason to refuse, so `a-failed-requirement-refuses-the-whole-product`'s acceptance holds (test 10).
- **C's boundary with stage 4.** If stage 4 leaves contact findings on `Assembly`, or keeps `assemble` refusing, C's diff shifts accordingly. §10 states what C assumes, and the reconciliation happens when stage 4's spec lands.
- **Gap's carrier scope.** C5's `Gap` has arms for same-kind carriers only. Interference between, say, a box and a cylinder has no signed measure, so no assertion can quiet it. That is a frontier and not a defect (the finding stays loud), but it will show on real assemblies. Its successor is a signed primitive per carrier pair, not a looser rule.

## 8. What retires and what is rewritten (ratified text quoted)

D10 governs where a companion clause disagrees. Each rewrite below follows D10's Assertions paragraph and changes no D10 decision, so it lands with its unit. The exceptions are those a FORK marks.

- **ERROR-DESIGN E10** (A): "`Assertion { measure: NodeId, bound: Quantity, dir: AtLeast | AtMost }`" is rewritten as `Assertion { value: observed scalar variable, relation: ≤ | ≥ | =, bound: scalar variable }`. The "Open sub-question: should a failing Assertion gate `build()`? v1 says no" is answered by D10's "checks and never places" and the sentence closes.
- **ASSEMBLY A5 *Interference.*** (B): "An interference fit (one instance's material containing a vertex of another's) is decided by the census's material test and refused typed (`ValidationError::InstanceInterference`). The recorded gate-skips that `crates/topo/README.md`'s C6 declares are not implemented." It is rewritten to: an overlap between two copies is a finding of its own, quiet under a holding one-sided gap assertion at its site.
- **topo C6** (B): "Invariant: an undeclared interference is always a typed error; no blanket "disable interference checking" exists." It becomes: at rest between copies, an interference is always a finding, loud unless an assertion at its site quiets it, and there is still no blanket switch. Inside one body it stays a typed error. **C6's `Fit` class and its mass-property and export sentences are stage 4's** (D10's list: "C6's `Fit` as a class").
- **topo MATE-4B-CROSSING** (B): "**`EdgeFacePierce` stays categorical.** A transverse dive is interpenetration until a C6 vocabulary exists". At rest between copies it is interference evidence; the vocabulary is D10's finding.
- **ASSEMBLY A5** (C), its opening: "`assembly::assemble` gathers the product …, and runs the scalar's at-rest policy … `AssemblyError::AtRest` is a verdict against the document". It is rewritten as the at-rest check. The minting and attribution paragraphs are stage 4's.
- **DISCIPLINES DS6** (C, FORK-S5-4 and FORK-S5-5): "**Second resident SHIPPED (2026-08-29): the product-separation check.** … Its severity knob is `Advisory` (Off/Warn) …" and "Any check … may offer `error` **iff it ships a waiver vocabulary**: a per-finding, stable-name-keyed acknowledgment record". Both are design choices, so they wait for their forks.
- **`checks.rs` module doc** (C): "findings follow root-list order". This changes with the code that changed it.

## 9. The in-scope rows

- **`a-failed-requirement-refuses-the-whole-product`** closes in stage 2 C (its re-park note: the gather reads only placements). Stage 5 must not regress it. Its new reader is the quieting rule, which reads verdicts and never refuses (test 10). Its note gains a line saying so.
- **`a-measured-part-is-not-a-product-root`** closes in stage 2 C (test 7 there). Stage 5 depends on it: the measures that quiet a finding read the placed copies, which is that row's case. No work here.
- **`error-design-e3-calls-a-measure-a-sink`** lands with stage 2 D. If D merges without it, A carries it, because A rewrites the neighbouring E10 anyway.

## 10. Boundaries with stages 3 and 4

The stage-3 and stage-4 specs (`intent/stage3-spec`, `intent/stage4-spec`) were not pushed when this was written. These are the calls this spec makes; the PR body records their reconciliation.

- **Stage 4 owns**:
  - the coincidence door and the recording of every value-decided coincidence, at-rest contacts included;
  - the `unproven-coincidence` lint;
  - the retirement of the declared seats (`BooleanCoincidence`, a mate's `ContactClass`, `Fit`), the undeclared refusals, and **A5's hard error on an unattributed contact**;
  - with them, `AssemblyError::{Mint, CarriedMintRefusal, Uncertified}`, `MintRefusal`, `Attribution` and the topo README's "Attribution at the assembly layer".
- **Stage 5 owns** the at-rest door's reporting. Today's gate is stage 5's to dissolve, and the interference verdicts are B's. Stage 4 should leave `EdgeFacePierce` and the `SameSide` crossing between copies as B leaves them, as interference evidence and not `unproven-coincidence` findings: a pierce is not a coincidence.
- **The `Separation` check's suppression** reads `ContactRecords` (`checks.rs:1424`). If stage 4 deletes the mate-minted records before C, it re-keys the suppression to the structural contact stage 4 keeps, or leaves the resident unsuppressed (louder, which is the safe direction). C then retires the resident either way (FORK-S5-5).
- **Stage 3 owns** spaces, placement as the bundle of mates, `AssemblyError::Space` and A11 (4)'s declaring mates (`mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`). C's resident checks per space in whatever shape stage 3 leaves, and compares copies only within one.
- **The C row's `blocked_on`** names `value-decided-coincidences-have-no-recording-door` (stage 4's door) and `mate-offset-verified-…` (stage 3) as the stage-level triggers that exist today. When the stage-3 and stage-4 units are filed, it is re-pointed to their last units.

## 11. Open questions

### FORKs (each for a designer pair; the ones marked *ratified text* go to Ev through an `[ev]` PR)

**FORK-S5-1 — What "the same measure at the same site" is.** *Changes no ratified text: it gives D10's phrase its first definition.*

- **The problem.** D10 quiets a finding under "an assertion on the same measure at the same site", and nothing defines either half. A finding is about two copies in the world. A measure reads selections of `Body` variables, which may be the copies' outputs or the unplaced bodies they copy (construction never reads the world, so a measure of the unplaced bodies is in their construction frames and says nothing about the copies). A measure is one of four primitives, and only `Gap` is signed.
- **Options**, each a final state:
  - **(a) `Gap` only, over the copies.** The assertion's value reads one `Gap` measure directly, both selections are of the finding's copies' outputs, and its faces are the finding's site. One rule for both findings. Contacts on carrier pairs `Gap` has no arm for (a cylinder lying on a plane) cannot be quieted.
  - **(b) Per finding.** Contact under `Gap` or `Distance` (both zero exactly at a contact; `Distance` covers mixed carriers), and interference under `Gap` only (sign needed). Both over the copies' outputs.
  - **(c) The finding defines its measure.** A new primitive, `Coincidence(a, b)`, the margin the door decided, which an assertion reads. Exact, but it is a measure whose meaning is "whatever the census computed", a second description of a primitive the user can already write.
- **Recommendation: (b)**, likely. A measure of the unplaced bodies never matches, whichever option. A definition over a measure (`m + 0 mm`) does not match either, because "the same measure" is the measure's own output and a definition is another variable.

**FORK-S5-2 — The site of an interference finding.** *Changes no ratified text.*

- **The problem.** The census decides interference per pair of solids, with one witness vertex or pierce. An assertion is about two faces. Take a pin pressed into a bore: the witness is a bore-rim vertex inside the pin, and the natural assertion is `Gap(bore, pin) ≤ −δ`. The two copies may also overlap a second time somewhere the assertion says nothing about.
- **Options:**
  - **(a) The copy pair.** Any qualifying assertion over faces of the two copies quiets every interference between them. Cheap. One assertion can hide a second, unintended overlap (test 7).
  - **(b) Per overlap component.** The gate intersects the two copies, and each connected solid of the result is one finding, named by the face sites bounding it. An assertion quiets it when its two faces both bound that component. Precise. It costs an intersection per interfering pair, and a refused intersection leaves the finding loud and unquietable.
  - **(c) The census's witnesses.** The faces incident to each witness vertex or pierce event are the site. No intersection, but the witness is the first in arena order, so the site is partial and arbitrary: the pin's face is incident to no witness when the pin passes through.
- **Recommendation: (b)**, likely. A finding is a statement about an overlap, and the overlap's own boundary is the only site that matches what an author asserts.

**FORK-S5-3 — What "does not straddle zero" admits.** *Ratified text: it re-words D10's quieting sentence to state its principle.*

- **The problem.** D10: quiet when the bound "the observation meets and that does not straddle zero", with two examples (`= 0` for contact, "a bound on one side of zero" for interference). Read literally, a contact under `gap ≥ 0` is quiet: the admitted set `[0, ∞)` lies on one side of zero. But `≥ 0` says "no interference", not "contact intended". And `≤ 0` admits contact and interference both.
- **Options:**
  - **(a) The literal set reading.** Quiet when the assertion holds and its admitted set does not cross zero. `≥ 0` quiets a contact, and `≤ 0` quiets an interference.
  - **(b) The stratum reading.** C5's three strata of the signed gap are clearance (`g > 0`), contact (`g = 0`) and interference (`g < 0`). An assertion quiets a finding when it holds and its admitted set lies inside the finding's own stratum. Only `= 0` quiets a contact, and only `≤ b` or `= b` with `b < 0` quiets an interference. D10's examples become the whole rule.
  - **(c) As (b), but the interference stratum is closed (`≤ 0` quiets).**
- **Recommendation: (b)**, likely. An assertion quiets a finding by stating the very thing the finding says happened. The sentence would read: "a finding is quiet exactly when an assertion on the same measure at the same site holds and admits only the finding's own stratum of the gap: zero for a contact, negative for an interference."

**FORK-S5-4 — The shape of "the census as a check", and where it refuses.** *Ratified text: DISCIPLINES DS6's waiver paragraph, and how D10's "neither refuses where the census has a lane" binds.*

- **The problem.** A5 is a gate today: `assemble` returns `Err` on a finding. D10 makes contact and interference findings, which refuse nowhere the census has a lane, and leaves the no-lane case refusing. The registry's posture is that residents report and only `enforce_checks` refuses, and DS6 lets a resident offer `error` only with a waiver vocabulary.
- **Options:**
  - **(a) A5 stays a door.** `assemble` keeps its `Result`, refuses only no-lane and malformed bodies, and its `Ok` carries the findings, quiet or loud. The registry is unchanged. Two finding surfaces (the checks report and the assembly) stay.
  - **(b) A registry resident.** `CheckId::AtRest` with `Severity`. Assertions are its waiver vocabulary, and their staleness direction is free. No-lane is a could-not-look finding, reported loud and never refused (DS6: nothing answers it). `assemble` retires.
  - **(c) As (b), but `Advisory`.** It never refuses. No-lane findings are reported only.
- **Recommendation: (b)**, likely. It is the plan's wording, it leaves one finding surface, and an assertion is the per-finding, provenance-carrying, staleness-checked record DS6 asks a waiver to be. DS6's waiver paragraph is re-worded to name assertions as the at-rest resident's waiver.

**FORK-S5-5 — The `Separation` check's fate.** *Ratified text: DS6's second-resident paragraph (DISCIPLINES is "WIP, provisionally accepted").*

- **The problem.** `Separation` reports pairs of gathered solids its boxes cannot prove apart, suppressed by declarations. After C, the at-rest resident decides, for the same pairs, whether they are clear, in contact or interfering. Two residents would answer "do these copies meet" at two strengths, one of them a box test that cannot tell contact from overlap, and its suppression reads declarations stage 4 deletes.
- **Options:**
  - **(a) It retires into the at-rest resident**, its boxes becoming the resident's pre-filter.
  - **(b) It stays as a cheap report**, its suppression re-keyed to structural contact.
  - **(c) It stays unsuppressed.**
- **Recommendation: (a)**, likely. One question has one resident. The heat sink's five findings become five interference findings with sites.

### Questions with a recommendation (not forks)

1. **Is a quiet finding listed?** Yes, naming the assertion that quiets it, outside every count that refuses. "Checked and intended" differs from "not checked" (DS6's visible-skip rule, applied to findings).
2. **Which evaluation does the rule read?** The one the census ran in, at its scalar. An assertion `Unevaluated` at that scalar quiets nothing.
3. **Mass properties of a product with interference.** Today mass properties sum solids, so an overlap is counted twice. C6's opt-in overlap subtraction is the `Fit` era's and retires with it in stage 4. **Recommendation:** leave mass properties as they are, and let the interference finding be what says the sum double-counts. A subtraction is a later door over findings, not a stage-5 deliverable.
4. **Does `=` need a written tolerance?** No. `=` is decided at the document's ε like every margined verdict (D4). An engineering tolerance on a gap is two assertions (`≥ b₁`, `≤ b₂`), and those quiet nothing at a contact, by FORK-S5-3 (b).
5. **Interference inside one node's value** (a multi-solid boolean result). That is not "at rest between copies". It stays the kernel's typed error (`restfront/tier-3-admits-two-solids-of-one-body-whose-material-overlaps`). Stage 5 does not touch it. The members of a pattern's `Bodies` are not one value: an overlap between two of them, or between one and another copy, is a finding like any other, and an assertion over every member is written once as a `map` over the `Bodies` (D10; `work/intent/a-map-over-a-patterns-bodies-asserts-once-per-member.md`).
