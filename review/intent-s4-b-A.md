# Review A — PR #4354 "INTENT stage 4 PR B: coincidences are recorded at one door"

Frozen head `48fa9860e4`, merge-base with `origin/main` `f3755cd40f`. Reviewer A (correctness + style lane).

## Verdict: APPROVE-WITH-FIXES

The unit does what #4322 ruled: rows are keyed in the deciding op's inputs and named by read plus `StableName`; the door walks the document and reads no stamp; decision (`Coincidence`) and contact (`ContactRecords`) are two types; the lint reports exactly what rung 1 leaves. Nothing builds differently (every slow set green, no golden moved). The fixes are small: two deferrals owe a schedule, one wording overclaims, three docs rotted, and three carrier paths plus the lint's quiet half have no row that can go red.

## Runs (private `CARGO_TARGET_DIR`, foreground, ε default)

- `cargo nextest run -p editor-core --profile default --no-fail-fast` in two partitions: 1462 + 1460 = 2922 passed (the timing row `a_large_table_of_names_alike…` passed here; it cancelled a first fail-fast run only).
- `-p topo --profile default`: 1324 + 1323 = 2647 passed. `-p sweep --profile default`: 2482 passed. `-p pncad-py -p verbs -p pncad`: 251 passed.
- `cargo fmt --check` clean; `work.py lint` ok; all 20 `scripts/gates/*.sh` pass. CI on the head: test, lint, python suite, viewer, corrupt-input all success.
- Python not run locally (maturin wheel); CI's python job on `48fa9860e4` ran `test_checks.py::TestTheUnprovenCoincidenceResident` green.

## Findings

### MAJOR

None.

### MINOR

- **M1. Declared `Tangent`/`Seam` pairs are value-decided glue with no row and no schedule.** `crates/topo/src/boolean/mod.rs` `verify_declared_contacts` records only the `Rest`/`Continuation` arm; the `Tangent` arm (`verify_tangency_declaration`, the witness lane) and `Seam` arm glue on a value decision and emit nothing. The PR body defers them to E, but `work/intent/booleans-glue-on-zero.md` names no declared-tangent or seam recording (grep `tangen|seam|declar`: nothing). A disclosed deviation without a schedule (lane §Q6). Fix: one sentence in E's row or a filed row. Confidence: sure (by reading).
- **M2. The finding's sentence overclaims.** `crates/editor-core/src/checks.rs` story arm: "… holds only at the current values". The door has one rung; a row it leaves `Unproven` may well hold structurally (every box mitre, every shared-variable flush plate until C/D). D10's "holds only at the current values" is about rows that do not hold structurally, not rows not yet proven. The `.pyi` `CheckId`/`CheckEvidence` docstrings and `Coincidence.rung` doc repeat it. `CheckKind::Certified`'s own doc says "the finding is a theorem"; the implementer's "certified in the direction it stays silent" has the `Separation` precedent, but the sentence should say what the door knows ("not proven structural by …"). Confidence: sure.
- **M3. Three carrier paths and the lint's quiet half have no row that can go red.** Mutation M3 (empty `coincidences` at `ops.rs` `fallback`, `finish_fallback` and `rest.rs` `try_rest_union`): 239 tests in `coincidence|declared` filters over editor-core, topo, sweep all stay green. Only the seamed recut path is pinned. The REST-zip path is exactly the one a declared curved `Rest` takes, so the rows most likely to be unproven in practice ride untested code. Likewise no test runs `run_checks` over a document holding a proven row, so "quiet where the door proves" (C3) is pinned only through direct `prove` calls on hand-built rows. Confidence: sure (by execution).
- **M4. Doc rot the diff caused.** `checks.rs:1–7` module doc still says "its three residents" and lists three; `checks.rs` `CheckFinding` struct doc and `pncad.pyi` `class CheckFinding` still say "one subject — a body-denoting root output, attributed as `(root, output_ix)`", false for the new check (root is any node, `output_ix` is a constant 0). `subject_body(ev, finding.root, 0)` on such a finding answers a split node's above half. Confidence: sure.
- **M5. `a_transversal_split_records_nothing` cannot fail on the filter it names.** `crates/editor-core/tests/coincidence_door.rs`: a 2×2 square cut at `y = 1` has no operand vertex ON the plane, so the `runs.len() >= 2` filter is never exercised there. Mutation M1 (`>= 2` → `>= 1`) left it green; only topo's `m3_pr3_split::a_split_records_its_pinch_and_nothing_where_it_only_cuts` went red (six rows for two). Lane Q3 shape 1: the premise excludes the failing mode. Confidence: sure (by execution).

### NOTE

- **N1. PR body calls FORK-S4-4 open; it is ruled** (#4323, fork log row 94): the pass stays, rows in member space, the list is the author's fold order, the fold reads one verdict per carrier pair through lineage (F/E's build). The code is consistent with the ruling's B-side. Ev asked for tests or debug asserts on order independence: `a_unions_rows_do_not_depend_on_its_member_order` covers two members; a three-member union where a third member covers a pair is not pinned. Confidence: sure on the ruling, likely on the gap.
- **N2. Rows' cell order is node-id order, not the author's list order.** `judge_pairwise_contact` sorts members by id and makes the lower id operand A, so a row reads `(lower id, higher id)` whatever the list says; that is why `[a,b]` and `[b,a]` give byte-equal rows. Pre-existing enumeration; fine under "same in every order", but "in the order the decision read them" is the judgement's order, not the document's. Confidence: sure; whether it matters, unsure.
- **N3. The same-member declared pair records no row** (`a-unions-same-member-declared-pair-records-no-row`, filed P3). A glue decided from values and recorded nowhere the lint reads is a D10 hole however rare; P3 reads low for that. Scheduled, so not a MINOR. Confidence: likely.
- **N4. An instantiated part's rows do not reach the instantiating document's lint.** `wire.rs` `wire_instantiate_part` sets `coincidences: Arc::new([])` while `contacts` are carried through; `run_checks` on an assembly reports none of its parts' unproven rows. Recorded in the part's own document, so D10's "recorded" holds; the product's checks pane is silent on them. Confidence: likely.
- **N5. N6 as rewritten describes C's and E's state.** `crates/editor-core/src/names/README.md` N6: "The door reads its carrier at that name from the symbolic evaluation" (C) and "The kernel carries no recipe provenance of a description" (false until E: `GeomSource` is minted, composed through placements and read by `plane_source_rung`, `merge_faces`). The text is #4322's verbatim ("when this unit lands"), so it is Ev's; and the companion row for N6 now points only at `coincide.rs`, leaving the live `GeomSource` mechanism with no design-page home until E. Confidence: sure it is false today; unsure what to do given the ratification and the no-scaffolding rule.
- **N6. No `work/intent/log.md` entry for the unit.** Precedent on main is that unit PRs add none (entries are the orchestrator's), so recording, not asking. Confidence: likely.

## The implementer's rulings

1. **Merged/covered pairs record no own row — right.** `covered` and `rest_contacts` are filtered to `verified.one_carrier` (`mod.rs:4615`, `:4629`), and `declared_surface_pairs` (`ops.rs:3557`) keeps only `is_one_carrier` declared pairs, so each is the declaration door's decision re-read; the plane ladder's rung 2 (`plane_eq.rs:264` → `declared_reading`) runs only for pairs `declared_one_carrier` names (`vtxfac.rs:736`, `recl.rs:139`), the reduction passes `declared: false`. No glue goes unrecorded on that path. Sure.
2. **Tangent/Seam declared pairs left to E — under-specified** (M1): defensible in scope, unscheduled.
3. **§11 row 4 pinned on a hand-built row — right as far as it goes.** No production row proves in B (the kernel's rung 1 settles same-source before any margin), so the pin is over real cells with a synthetic margin; see M3 for what that leaves unpinned at the lint. Sure.
4. **`prove(doc, row)` — right.** Rung 1 is a recipe walk; C will add the evaluation. Sure.
5. **A union's rows are the pairwise judgement's — right** under #4323 (N1, N2, N3).
6. **Rung 1 walks placements as `Transform` nodes and `(pattern, i)` — right, not positional.** `i` is the document's own `PartSelect::Instance` read, and `Placed::Transform(node)` is the read's placement node, not its value: two Transforms of equal steps are two placements (unproven), one Transform read twice is one (proven). Verified by execution: the pattern row and M2. Sure.
7. **Discharge `Numeric` only, ContactRecords citing deferred to B2, vertex fusions parked on E** — as the orchestrator ruled; each has its row. Sure.

## Ev's 2026-10-03 transcript — reintroductions

None found. Equal values are never read as intent (the declared-rest row: coplanar caps of two extrudes stay `Unproven` with two constructions). Provenance is read from the document, not stamped (`grep GeomSource|AxisSource|ParamSource crates/editor-core/src/coincide.rs`: none). One record type per role. Nothing places or declares by position (N2 is an enumeration order, not a placement). No constraint falls back to an assertion.

## Claims

- **C1 One door, no row lost** — exercised. Every `BooleanBody`, `SplitResult`, `Blended`, `VerbOut`/`SplitOut` constructor carries rows (single `BooleanReduction` constructor at `mod.rs:4621`; `whole_body_side` empties legitimately). M4 (drop the union's rows) went red at `coincidence_door.rs:254`. Gaps: M1 (declared Tangent/Seam), N3, N4; M3 for the untested carriers. Likely sound; sure on the paths read.
- **C2 Rung 1 reads the document** — exercised. No stamp read in `coincide.rs`/`checks.rs`; `carrier_eq` keeps its same-source rung as the kernel's own (E's). One construction read twice proves (split halves through one Transform; one pattern instance through two Parts); equal-valued independent constructions do not (declared rest). M2 (drop the Transform from the walk) went red at `coincidence_door.rs:230`. Sure.
- **C3 The lint** — partly exercised. Reports exactly the `Unproven` rows (declared rest: one finding; filleted box: eight; off: skipped visibly). Quiet-on-proven only via `prove` (M3). Python agrees by CI's run of `test_checks.py`; the viewer's `coincidence_cells` has no test. Likely.
- **C4 Pins** — exercised by execution: `dsc_checks::the_registry_order_is_every_check` (4 residents), `band_planar_mitre::an_isosceles_turn_is_recorded…` (reads `topo::Coincidence`, pins the `Blended` carry), `carrier_eq::…reads_at_the_pair_door…` (renamed door), `TAG_INVENTORY` and `every_check_evidence_arm_projects…` (14 fields) all green; every digest/content-key/id golden green across the four slow sets. Sure.
- **C5 Rulings** — above.
- **C6 Rows bite** — M1, M2, M4 bite as stated; M3 does not (finding).

## Style

- `crates/topo/src/boolean/carrier_eq.rs:338,362,772`, `rest.rs:835,855`, `splitting/classify.rs:245`, `sweep/src/blend/mod.rs:493`: five "projection twin" pairs, each old two-tuple door kept as a `.map(|(a, b, _)| (a, b))` over its new margined triple. Each is the lane's almost-parallel shape with the dependency pointing the right way; still ten spellings of five doors, and the next margin-bearing caller will add a sixth. Q1. Likely.
- `crates/topo/src/boolean/mod.rs:5060` `declared_site`: re-reads `face_carrier` for both faces to label a row `PlaneLadder` or `CarrierLadder`, after `carrier_pair_reading` already read both carriers. Both labels name one deciding site (`declared_reading_margined`); the enum encodes carrier kind as if it were the site. Q1/Q7. Likely.
- `mod.rs:4974` `verify_one_carrier_declaration -> Result<Option<Option<Coincidence>>, _>`: outer `Option` is "verified", inner is "a margin decided it". Two bits in nested `Option`s. Q7. Sure.
- `crates/editor-core/src/checks.rs` `CheckFinding::root` doubles as "the deciding node" and `output_ix` is a constant for this check; the subject sentence branches on `check` to say "node" vs "root". An invariant held by convention where a second shape would do. Q7. Sure.
- `crates/topo/src/coincidence.rs:90` `Discharge` has one arm; a knob never varied until C (ruled, recorded only). Q7. Sure.
- `crates/editor-core/src/coincide.rs:244` `construction`: `(_, node) => body_input(node)?` is the catch-all for every carried segment the match does not name; `body_input` answers `Fillet`/`Chamfer`/`Shell`/`Split`/`Pattern`/`PlacedUnion`/`Transform`/`Part`. A `FromA` segment on a non-`Boolean` cannot occur, but the catch-all would silently route it to a one-body input rather than refuse. Sound today (unproven on `None`), fragile. Q7. Likely.
- `coincide.rs:30` `NamedCoincidence::margin` "For reporting only" while `topo::Coincidence::margin` is the deciding margin: one field, two doc sentences; C will read it. Q2. Unsure.
- `coincide.rs` `Residual` `Display` says only "two constructions"/"one"/"neither"; the constructions it carries are never said. Spec row 3 wanted "two sources". Q5. Likely.
- `crates/editor-core/src/eval/wire.rs:3030` `own(m)` re-reads `value_of(results, m)?.name_table` for naming while `operands[p].1` holds the member view of the same table; two spellings of "the member's table" in one closure, and the comment is what reconciles them. Q2. Likely.
- Class note: the `.pyi`/Rust docs that assume a finding is `(root, output_ix)` (M4) plausibly recur in `viewer` (`CheckRow::root` "which the button selects") and `pncad-py` `CheckFinding.subject_body`; sweep those with the fix.
- Whole-file read: `coincide.rs` (328 lines) end to end, `checks.rs` (1777) end to end; `wire.rs` (5785) only the touched regions (`OpOut`, `wire_blend`, `wire_split`, `wire_boolean`, `wire_union`, `judge_pairwise_contact`).

## Questions exercised

Q1 (grep + callers), Q2, Q3 (M1, M5), Q4 (N5, M4), Q5 (M4), Q6 (M1, N3), Q7, Q8 (checks.rs, coincide.rs). Not exercised: Python locally; the viewer pane; a three-member union with a covering third member; the REST-zip carrier on a real declared curved scene.
