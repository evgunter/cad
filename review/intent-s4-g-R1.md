# Review R1: INTENT stage 4 PR G, "tangent joints are derived" (PR #4405)

Frozen head `74821aee98`, base `db51132ff7` (merge-base with main). Reviewer
R1 of a holdout pair (byte 180). Correctness lane plus the style lane
(`docs/prompts/reviewer-style-lane.md`). Every finding carries
sure / likely / unsure.

## Verdict: APPROVE-WITH-FIXES

The mechanism is right and its rows bite. No tangency is stored. The
derived set is computed in one place (`validate.rs::judge_joints`). Every
kept refusal has a row that goes red without it (8 mutants, 7 killed). The
profile digests are bit-equal (dumped and diffed against a base build at
two ε). The fixes are about one undisclosed deviation and the rows that
should pin it:

- A same-carrier joint that validation decides is recorded as
  `SameOriented`. The PR body (ruling 3) and the spec (§8) say
  `Tangent`.
- No document-level row pins that arm, and a lattice-built profile
  reaches it.

None of the fixes moves geometry.

## Findings

**MINOR-1. A same-carrier decided joint is recorded as `SameOriented`,
not as `Tangent{aligned:true}` as the PR body and the spec say.**
- Where: `editor-core/src/coincide.rs::name_junctions` maps
  `JointCarriers::Same => topo::Relation::SameOriented`.
- What the text says instead:
  - Ruling (3) in the PR body: "recorded as `JointCarriers::Same`, with
    relation `Tangent { aligned: true }`".
  - Spec §8: "each recorded as a `Coincidence { relation: Tangent, site:
    ProfileJunction }`".
  - README V6: "same-carrier continuation is a tangent joint".
- So the code deviates from the spec's letter, and the PR body says the
  opposite of what the code does. Either choice is defensible:
  `SameOriented` is §1's continuation relation and reads "continues on
  one carrier with". But the deviation is undisclosed, and Python readers
  get `same_oriented` at a `profile_junction`, which nothing documents.
- Demonstrated by inspection, and by mutant M7 (below), which survives.
  Confidence: sure.

**MINOR-2. No row pins the relation of a same-carrier decided joint at
the document, and a lattice-built profile reaches that arm.**
- Mutant M7 (`Same => Tangent{aligned:true}`) survives all 143
  editor-core tests matching `coincid|profile|tangen|junction|check`.
- Python covers only the `tangent` arm (`test_checks.py`).
- The arm is live, not hypothetical. Probe `r1_lattice_same_carrier_decided`:
  - Built through the lattice:
    `Open.at(0,0).line_to(1000,0).line_to(1001,y).line_to(1001,5).line_to(Start)`.
  - `ConstructedProfile::validate` at y = 0.5ε and y = ε returns
    `decided_joints = [(1, Same)]`.
  - Why: the lattice levers the turn by the long arriving arm, so it sees
    a corner; validation reads the short leg's far end, so it sees one
    carrier.
- A document row for this shape, asserting its relation, closes MINOR-1
  and MINOR-2 together. Demonstrated by execution. Confidence: sure.

**NOTE-1. Spec row 21's lattice premise overclaims.**
`tangent_joints.rs::the_lattice_constructs_every_tangent_joint_it_builds`
is red "if a lattice site mints a zero-turn joint without constructing
it". It only checks the coverage corpus, which has no ε-scale junction.
The lattice does build loops whose joints validation decides:
- the PR's own `coincidence_door` row (a line and an arc at φ = √ε);
- my probe above.
Read as "no verb row's own construction leaks", it holds. Its name and
doc say more. Demonstrated by execution. Confidence: sure.

**NOTE-2. The refusal count moved 34 → 32, not 33 → 32.**
- The PR body says the shape-guard rows went "33 → 32 … with
  `UndeclaredTangency` retired".
- Measured by parsing `FILED_NO_RECOURSE` at both commits: the block
  under `// work/paths/paths-refusals-short-of-the-shape-guard.md` holds
  34 entries at the base and 32 at the head.
- Two entries left: `Profile/UndeclaredTangency` and
  `Profile/TangentJointOutOfRange`.
- The base item's "33" was already off by one. The head's "32" is
  correct, so the end state is right and the stated arithmetic is not.
Demonstrated by execution. Confidence: sure.

**NOTE-3. The fillet dump's verdict column now measures a different
gate.**
- `fillet_stored_tangency.rs::validates` moved from `Profile::validate`
  (`Consistency::Decide`) to `ConstructedProfile::validate`
  (`ByConstruction`, which skips the three arc-consistency checks).
- No verdict moved at ε default or at 1e-12 (C3 below), so the re-baseline
  is honest. Even so, the golden's verdict half no longer exercises the
  table gate it did.
- The golden's doc comment also lost its whole re-baseline history, three
  earlier moves explained in place.
Demonstrated by inspection plus the dump diff. Confidence: sure.

**NOTE-4. `DISCIPLINES-DESIGN.md` names profile tangency's position
"auto-record (D10)", but DS7 defines `auto-record` as a persisted,
diffed record.**
- DS7's definition: "subsequent evaluations diff the definite findings
  against the recorded set, and changes complain".
- G's record is per-evaluation and diffs nothing.
- The new wording appears at `:167`, in DS2's residents list (`:229`) and
  in the grade table (`:577`).
- These are edits to a design doc's classification, beyond the spec's
  "DISCIPLINES DS-Q6". They need Ev's eye whatever the label. The PR
  already says it is not lane-merged.
Demonstrated by inspection. Confidence: likely.

## The claims

- **C1. No tangency is stored: holds.**
  - `ProfileLoop` is `{vertices, segments}`.
  - The only joint list is `ConstructedLoop.joints`, which the lowering
    writes from the verbs (`Core::finish`, `ConstructedLoop::carrier`),
    plus the `fixture` door. That door is cfg `test`/`test-support`, and
    no shipped crate (pncad-py, viewer, pncad) enables `test-support`.
  - `LoopCanonical.tangent_joints` is a derived per-evaluation record.
  - Nothing serializes either. `"tangent_joints"` survives only in the
    pre-program `bool13_goldens`.
  - Inspection (grep over `crates/*/src`). Confidence: sure.
- **C2. Every Zero junction no constructor made is recorded, and no
  constructed one is: holds for the profile node's f64 lane.**
  - M3 (drop the record) reds 9 rows.
  - M8 (drop the emission in `wire.rs`) reds the `coincidence_door` row.
  - M2 (record constructed joints too) reds 18 rows.
  - `TangencyContradicted` still verifies (M1).
  - Library callers that validate a raw `ProfileLoop` (e.g.
    `sweep::skin`'s `SectionLoop`) get `decided_joints` on the value but
    reach no document door. That fits B's "every op that decides" only
    at the node level.
  - I did not exercise the interval or guided lanes' rows (below).
  - Confidence: likely.
- **C3. The digests are bit-equal: holds.** Three pins checked by
  execution against a base build:
  1. **`fillet_stored_tangency` dump**, `CAD_DUMP_FILLETS=1`, diffed
     line by line (ε default: 48 → 50 lines; 1e-12: 40 → 42):
     - 0 verdict moves and 0 vertex or arc-bit moves at both ε;
     - 5 joint lists only re-ordered (each sorted-equal);
     - 2 rows added, `shared coverage corpus 14/15` (the circle forms).
  2. **`lift_census::the_census` printout, base vs head:**
     - every row present at both commits has identical class, step count,
       seam and worst-ulp figures;
     - `unequal_split` (REFUSED → ValueEqual) and `collinear_run`
       (WALL → BitIdentical) moved for the stated reason;
     - `half_disc_undeclared` folded into `half_disc`, with the same
       figures;
     - the tallies match: Bits 5 → 6, Value 7 → 7.
  3. **The refusal count**: 34 → 32 (NOTE-2).
  Confidence: sure.
- **C4. `UndeclaredTangency` retires, and every kept refusal has a red
  row: holds.**
  - The variant, its tag and its concision-chain entry are gone.
  - `TangencyContradicted`: M1 reds 4 rows.
  - `TangentJointOutOfRange`: M4 reds 2 rows.
  - `TangentJointOnFullTurn`: M5 reds `one_segment_loop`.
  - `JunctionTangent`/`SeamTangent`: not mutated (the PR names
    `bool11_probes`).
  - Confidence: sure.
- **C5. The rulings: see the next section.**
- **C6. The surfaces agree, except:**
  - the relation for a same-carrier joint (MINOR-1);
  - the `.pyi` and `checks.rs` docstrings, which list `tangent`/`cusp`
    for profile junctions but not `same_oriented`.

  Otherwise:
  - tags `tangent`/`cusp`/`profile_junction` match `topo::Relation` and
    `DecisionSite`;
  - README V5/V6 and DS-Q6 match the code;
  - the filed P3 row `lift-same-carrier-repair-is-reached-by-no-row` is
    accurate (the census diff shows both former reachers now lift).

  Confidence: sure.
- **C7. The rows bite: yes.** 8 mutants, 7 killed:

| Mutant | Change | Result |
|---|---|---|
| M1 | no `TangencyContradicted` | 4 red |
| M2 | record constructed joints | 18 red |
| M3 | no record | 9 red |
| M4 | no range check | 2 red |
| M5 | no full-turn check | 1 red |
| M6 | ask the heading question on same-carrier joints | 7 red (m10/sym K-stream pins) |
| M7 | `Same` → `Tangent{aligned:true}` | **survives** |
| M8 | no emission at the profile node | 1 red |

## The rulings

1. **`ConstructedLoop` carries the constructed joints: right.** It is the
   spec's "derived at lowering from the constructors". The loop is a plain
   cache, and a raw table has no constructor. Sure.
2. **The `LoopCanonical` record is kept, now derived: right.** Guided lanes
   compare it (`canonicalize_loop`'s `TangentJoints` and `CuspJoints`
   flips). It is a per-evaluation decision record, not an authored datum.
   Likely.
3. **Same-carrier joints are in the set: right.** That is D1's "every
   zero-turn joint". **Recorded under the wrong name, relative to its own
   claim.** The code records `SameOriented`, not `Tangent{aligned:true}`
   (MINOR-1). On the merits a continuation is a different relation from a
   tangency between two carriers, so `SameOriented` is arguably the better
   name. Either way, choose one, state it in the spec or README, and pin it
   (MINOR-2). Sure.
4. **The circle forms construct their joints: right.** A circle's
   subdivision vertices are its construction, and without this every
   lattice circle would report a finding. The verification still runs.
   Sure.
5. **Arc extension constructs nothing: right.** It is pinned by
   `an_arc_extension_leaves_no_joint_to_decide`. Likely.
6. **The heading question is skipped for same-carrier joints: right on its
   merits, not only for the pins.**
   - A same-carrier joint cannot reverse in a loop that passed the
     simplicity pass. Probes `r1_same_carrier_reversal` and
     `r1_cocircular_reversal` both refuse earlier: `NonSimple … Overlap`
     and "endpoint contact".
   - M6 shows that asking only adds K-stream decisions (+2
     `path_junction_side` per circle). No verdict or refusal moves.
   - Caveat: main asked this question for lattice continuations, so this
     PR also drops those decisions. No pin saw them.
   - Sure.
7. **`TangentJointOutOfRange` and `TangentJointOnFullTurn` end in
   `KERNEL_DEFECT_ENDING`: honest.**
   - The lattice's indices are in range by construction: `Core.tangent`
     holds vertex indices, and `carrier` uses `0..n`, empty when n < 2.
   - The only other source is the `fixture` door, which no shipped build
     carries.
   - Sure.
8. **`NamedCell::Piece` with `DecisionSite::ProfileJunction`: right.**
   Unsure about one edge: `prove` treats two equal `Piece`s as
   `SameConstruction`. A decided junction between two segments that one
   step drew under one piece ref would be silently proven. I found no
   verb that does this outside its own constructed joints, and did not
   exercise it.
9. **The lift derives the set, and `compare` checks set equality: right.**
   The census evidence is under C3. The repair that is now unreached is
   filed (P3). Sure.
10. **DS-Q6 settled by D10: right in substance.** The "auto-record" label
    is wrong (NOTE-4). Likely.

## Style

- **Two parallel classification enums.** `validate.rs` `JointCarriers
  {Tangent, Same}` sits beside `seg.rs` `JointClass {Tangent, Transversal,
  SameCarrier}`, mapped arm for arm in `judge_joints`. It is a second
  spelling of one classification, kept for the public surface. Q1.
  Likely.
- **The relation is assembled from two lists.** A `DecidedJoint` does not
  carry its own heading. `coincide.rs::name_junctions` rebuilds `aligned`
  by `binary_search` over `cusp_joints`, a separate list. The type would
  have to hold it for the record to be self-contained. Q7. Likely.
- **The escalation text still offers a declaration.** A profile-junction
  escalation still renders `COINCIDENCE_RECOURSE`'s "declare the
  coincidence" (seen in my probe's `chord_side` escalation), and a profile
  junction now has nothing to declare. Filed as an observation in
  `paths-refusals-short-of-the-shape-guard.md`'s new section, with no
  schedule beyond that open item. Q4/Q6. Likely.
- **The lattice and validation disagree about the same junction.** The
  lattice refuses an exact zero-turn junction (`JunctionTangent`), but a
  junction at φ = √ε builds and is recorded as `Tangent`.
  - That is the lattice's levered margin against validation's
    carrier-clearance margin, two bands over one junction.
  - The PR's new rows are built in exactly that gap.
  - It is spec-sanctioned (§8 keeps the gate). Still, it is the
    transcript's "why do I have to declare what I already said with
    numbers?" at the authoring door: the user is refused the exact case
    and accepted the near one. Q7. Unsure whether it is in scope.
- **A golden lost its history.** `fillet_stored_tangency.rs`'s
  `GOLDEN_DEFAULT` doc dropped its whole re-baseline history. The PR body
  now holds what moved, which is the instrument's own instruction, but
  the in-code record of three earlier moves is gone. Q2. Likely.
- **One lattice-made zero-turn class reaches the record.** The record
  exists for D1's "junction no constructor made", and lattice point legs
  (`line_to`/`arc_to` with a sub-band turn) are a lattice-made zero-turn
  class that reaches it. The test claiming the lattice constructs every
  tangent joint is NOTE-1's overclaim. This is the same class as the
  previous bullet. Q3/Q5. Likely.
- **One-implementor trait.** `validate.rs`'s `LoopInput` is a private
  trait with two impls, one returning `&[]`, kept to thread
  `constructed_joints` through generic `L`. A `(&ProfileLoop, &[usize])`
  pair would do. Q7 taste. Unsure.
- **Q8: whole file.** Read `crates/profile/src/validate.rs`
  `judge_joints` → `canonicalize_loop` end to end (not the whole
  2.7k-line file), and all of `coincide.rs`'s new code. Nothing beyond
  the above.

## Exercised, and not

**Exercised by execution** (private target dirs: `tgt-r1` for the head,
`tgt-mut` for mutants, `tgt-base` for the base):

- **The slow set** (`--profile default`) over `profile` and `editor-core`
  at ε default: 3494/3495.
  - The one red is
    `name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time`
    (12.3 s).
  - It fails identically at the **base** on this machine (14.0 s), so it
    is a slow-machine timing row and not this PR's.
- **The same set at `CAD_TOLERANCE_EPS=1e-12`:** 3494/3495, the same
  timing row the only red.
- **C3:** the dump and census diffs.
- **C7:** M1–M8.
- **Probes:** two same-carrier reversals, and a lattice-built same-carrier
  decided joint.

**Not exercised:**

- sweep, mesh, stl, pncad, viewer, the Python suite and the tour (the PR
  reports them green; I did not re-run them);
- the interval and guided lanes' rows for a decided joint;
- the `JunctionTangent`/`SeamTangent` mutants;
- ruling 8's equal-piece edge.

My probes live only in a scratch worktree. Nothing was pushed to the PR
branch.
