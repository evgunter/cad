# Review B — PR #4354, INTENT stage 4 PR B (frozen head 48fa9860e4)

Base `f3755cd40f` (merge-base with origin/main). Private target dir, foreground runs.

## Verdict: APPROVE-WITH-FIXES

The record, the carry, the door and the lint are built as #4322 rules: the door
walks reads and names, never `GeomSource`; equal-valued constructions stay
unproven (shown by execution); the rows bite under mutation; no digest, content
key or id moved. Nothing below is a soundness hole in the dangerous direction
(the door proving something false). The fixes are: two carrier paths and one
site with no test that can go red, an N6 page that describes E/C as present
fact, a false finding on the spec's own row-4 scene, and the lint's silence over
instantiated parts.

## Runs

- `cargo nextest run -p editor-core --profile default`: **2921/2922**; the red is
  `name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time`
  (timing row, red on main too, as the PR says).
- `cargo nextest run -p topo --profile default`: **2648/2648** (my probe included).
- C4 by name: `docm4_evaluation_identity`, `eval9_nominal_in_the_key`,
  `edit_placement_corpus_bits`, `wire_rv_bytes` → 16/16 green. No golden file is
  in the diff (`git diff --stat | grep -i golden` is empty).

## Findings

**MINOR-1 — Rows dropped on three of the four boolean result sites would go
unnoticed.** `topo/src/boolean/ops.rs` `fallback` and `finish_fallback`
(`coincidences: red.coincidences.clone()`) and `boolean/rest.rs` `try_rest_union`
(`coincidences: red.coincidences`). C1 says no row is lost on any carrier path.
I set all three to `Vec::new()` and the whole `coincidence_door` suite stayed
green (9/9). No test reads a boolean's rows except through the recut path, and
nothing uses `DecisionSite::CarrierLadder` (curved pairs). The code is right
today, but nothing would catch it going wrong. Confidence: sure (mutation run).

**MINOR-2 — The spec's own §11 row-4 scene yields a false finding.** I split a box
at z=0.5, then joined the halves with a `Boolean::Union` that declares the
section caps `REST` and the −x wall pieces a `Continuation`. Result: the wall
pieces get no row (the kernel's same-source rung settles them, as ruling 2 says).
The two section caps get one `SameOpposite`/`PlaneLadder` row, and the door leaves
it **Unproven**: `SectionFace{Above,0}` and `SectionFace{Below,0}` are two minted
roles. Both caps are the one tool plane read twice, so by D10 the row is the same
construction and structural. Ruling 2 says no production row reaches the door
with one construction read twice. That holds only because `Construction` is a
minted *role*, which is finer than a construction (the split's side qualifier is
not a different carrier). This errs toward noise, not toward false proof. But it
is a finding every split-and-reunion will carry until C, and it contradicts row 4
as the spec wrote it. Confidence: sure (executed, probe below); whether C absorbs
it: likely.

**MINOR-3 — N6 states E's and C's world in present tense.**
`crates/editor-core/src/names/README.md` N6 now says "The door reads its carrier at
that name from the symbolic evaluation" and "The kernel carries no recipe
provenance of a description". At this head, `coincide::prove` reads no carrier
and runs no symbolic evaluation (it compares walked names), and
`topo/src/source.rs` `GeomSource` still decides the kernel's same-source rung
(`plane_eq.rs`, `carrier_eq.rs` `declared_verdict`). The table row also dropped
`source.rs`/`plane_eq.rs`, which still implement it. The text is #4322's ratified
end state (commit 67f811615), so the wording is not in question. Landing it
before E and C makes a present-tense design page false; README pages are present
tense only (CLAUDE.md). Q4 case: doc ahead of code. Confidence: sure.

**MINOR-4 — Checking a product document is silent about its parts' unproven
rows.** `eval/wire.rs` `wire_instantiate_part` sets `coincidences: Arc::new([])`
("The part's own rows are its document's, read there"), while `contacts` are
carried through. `CheckKind::Certified` promises that silence means proven. On a
product, silence about a part's declared glue means "not looked at". B2
(`contact-records-cite-their-decision`) schedules the instantiate seam's
*citation*, but no unit names the lint reading through instances. Confidence:
likely (read, not executed).

**MINOR-5 — Glues still unrecorded after B, and how each is scheduled.**
(a) A same-member declared pair in an n-ary union is verified at a fold step whose
rows `wire_union` discards. It is filed as `a-unions-same-member-declared-pair-records-no-row`,
P3 `open`, with no unit. The filing also says the fix "depends on FORK-S4-4's
answer", but #4323 answered it (the pass stays as the union's coincidence door).
(b) Declared `Tangent`/`Seam` pairs: `verified.tangent.insert` is a value
decision with no row. Spec §6 has E record witness-lane glue, but E's unit file
names only the undeclared raise sites. (c) Vertex fusions: filed, parked on E.
Spec §0's "after B, every coincidence the kernel decides from values today
(declared glue, …) is recorded" is now false on (a) and (b). Confidence: sure for
(a) by reading `judge_pairwise_contact`'s `if i == j { continue; }`; likely for (b).

**MINOR-6 — Spec §3 now contradicts itself.** `docs/INTENT-STAGE4-SPEC.md` §3
keeps "**The merge.** … one row per absorbed face" directly above the new "**Not**
the merge's or the covered pairs' own rows". Only one of the two bullets was
re-worded. Confidence: sure.

**NOTE-1 — The editor-core transversal row cannot go red on the break it names.**
`coincidence_door::a_transversal_split_records_nothing` splits a block whose
vertices are all off-plane. Under the mutation `runs.len() >= 1` ("every ON vertex
is recorded", §11 row 6's break) it stayed green; only topo's
`a_split_records_its_pinch_and_nothing_where_it_only_cuts` went red (6 rows ≠ 2).
Confidence: sure.

**NOTE-2 — Pinches on the below side are recorded.** I checked by execution:
NOTCHED split by −y, and MIRRORED split by ±y, each record 2 rows (the mirrored
lane carries them). Confidence: sure.

**NOTE-3 — Placement chains are compared, not composed.** `Placed::Transform(node)`
lists are compared syntactically. #4322 says "composing placements", so two
transforms that compose to one map, or instance 0's identity against the bare
input, read as two constructions. This is sound (it only under-proves); H and C
own the rest. Confidence: sure.

**NOTE-4 — The PR body is stale on FORK-S4-4.** It calls the fork open; #4323
ruled on it. Separately, the union's rows come out in `by_id` (mint) order, and
the new test pins equality across member orders. Round 3 of #4323 withdrew the
mint sort as the *method* of order independence ("hides the arbitrariness"). The
rows are order-free by definition, which is right, but their order is the sort.
Confidence: likely.

## C-claims

- **C1**: emission exists at the declared rung, the split pinch and the battery
  turn, verified by reading and by mutation. Real cells, relation, site and the
  deciding verdict's margin are present. The losses are MINOR-1 (untested paths),
  MINOR-4 and MINOR-5. Partly holds.
- **C2**: holds. `coincide.rs` reads no `GeomSource`/`AxisSource`/`ParamSource`.
  Executed: two placements of one prototype (dx=0.5), declared continuations →
  4 rows, all `Unproven`, with `placed: [Transform(m1)]` vs `[Transform(m2)]`.
  Equal values are not read as intent. Same construction proves only on
  hand-built rows (theirs, re-run green). See MINOR-2 for the converse failure.
  (dx=0 refused `UndeclaredCoincidence` because the fixture's flush pairs don't
  cover a fully coincident pair. That is pre-existing and not B's.)
- **C3**: the lint reports exactly the `Unproven` rows (`checks.rs`
  `unproven_coincidence`) and stays quiet on proven ones. The mutation that
  dropped boolean and union recording turned `a_declared_rest…` and
  `a_unions_rows…` red. Python agrees word for word (`test_checks.py`, read, not
  run). The viewer's `coincidence_cells` has no test (`node_labels.rs` covers
  only `check_rows`' old fields).
- **C4**: holds (above). The re-baselined pins each moved for the reason the body
  gives; I read all of them in the diff.
- **C6**: re-ran three mutations. (i) door ignores placements (`a.minted == b.minted`)
  → 3 red (`a_row_over_one_placed…`, `a_patterns_instances…`, my equal-values
  probe). (ii) record every ON vertex → topo pinch row red. (iii) drop
  `wire_boolean`/`wire_union` rows → 2 red. (iv) drop fallback/REST carries →
  0 red (MINOR-1).

## Implementer rulings

1. *Merged/covered pairs record no own row*: **right** for the merge.
   `ops.rs` `declared_surface_pairs` feeds the merge only declared one-carrier
   pairs, which the declaration door records once. But glues do go unrecorded
   elsewhere (MINOR-5 a/b).
2. *Tangent/Seam declared pairs left to E*: **under-specified**. Spec §6 covers
   it, E's unit file does not, and §0's interim claim is now false.
3. *§11 row 4 on hand-built rows*: **acceptable but its premise is wrong**.
   The spec's own scene does reach the door, with a row that ought to prove and
   does not (MINOR-2).
4. *`prove(doc, row)`*: **right** for rung 1. C will need the evaluation back,
   which the spec's signature has.
5. *Union rows from the pairwise judgement*: **right** per #4323 (the pass is the
   union's coincidence door, rows in member space). NOTE-4 on order.
6. *Placements as Transform nodes and (pattern, i)*: **right, not positional**.
   `i` is the pattern's own recipe index (D9: "recipe-level provenance carries
   pattern indices"), and `(node, i)` equality is "the same entry read twice".
   It is conservative, not composed (NOTE-3).

## Style

- `coincide.rs` `construction` is a second carry-through walk beside
  `names/attribute.rs` `attribute`. They share `origin`, but the loop is written
  twice (one walks names, one walks names plus reads). That is Q1, two walks of
  one partition. Confidence: likely.
- `coincide.rs` `body_input` is a hand-written list of single-operand nodes next
  to `Node::inputs()` (`node.rs:3388`). A new placing or carrying node falls to
  `None` silently, which is sound but invisible. Confidence: likely.
- There are four margined twins: `classify_vertices`/`_margined`,
  `declared_reading`/`_margined`, `carrier_pair_verdict`/`_reading`, and blend
  `classify`/`classify_reported`. Each old door survives as a projection with
  1–2 callers, so a second version never replaced the first. Confidence: likely.
- The battery row's cells are the two requested edges. The third edge, where the
  mitre lands and which `Relation::EqualAngles`' doc names ("with a third"), is
  not in the row, so neither the viewer nor Python can select it. The spec drew
  this case as `(VertexKey, VertexKey) at a turn`. Confidence: likely.
- `CheckFinding::root` now means "the root" for three checks and "the deciding
  node" for one, with `output_ix: 0` as filler. That is one field with two
  meanings by variant, and the viewer's hover still says "select the root this
  finding is about" (`viewer/src/app.rs`). Confidence: likely.
- `band_planar_mitre.rs` reads the margin's value through
  `diagnostic_f64_for_error_text()`, a reporting door used as a data read in a
  test. Confidence: unsure.
- `Residual`'s `Display` answers one of three fixed sentences and never says
  *what* differs (role, placement, or both), although it holds both
  constructions. Confidence: likely.
- The unit row's title still promises "each ContactRecords row citing its
  decision", which moved to B2 (Q5). Confidence: sure.
- `NamedCell::Tool` always walks to `None`, so a split-pinch row can never be
  proven at this door, by construction rather than by any rung's verdict.
  Intended for B, but the type does not say so. Confidence: unsure.

## Exercised / not

Exercised: C1 (read plus mutation), C2 (executed both directions), C3 (Rust,
mutation; Python by reading), C4 (three suites by name plus the full runs), C5,
C6 (four mutations). Q1, Q2, Q3, Q4, Q5, Q6 and Q7 were asked. Q8: read
`coincide.rs` and `topo/src/coincidence.rs` whole; not `wire.rs` (5k+ lines).
Not run: the Python suite, the viewer, ε-row profiles, sweep/verbs/pncad-py
suites. Probes (not pushed to the PR): an editor-core equal-values test and a
split-reunion test, and a topo below-side pinch test.
