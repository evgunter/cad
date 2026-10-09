# Review R1: INTENT stage 4 PR G, "tangent joints are derived" (PR #4405)

Frozen head `74821aee98`, base `db51132ff7` (merge-base with main). Reviewer
R1 of a holdout pair (byte 180). Correctness lane plus the style lane
(`docs/prompts/reviewer-style-lane.md`). Every finding carries
sure / likely / unsure.

## Verdict: APPROVE-WITH-FIXES

The mechanism is right and its rows bite. No tangency is stored, and the
derived set is computed in one place (`validate.rs::judge_joints`). Every
kept refusal has a row that goes red without it: 8 mutants, 7 killed. The
profile digests are bit-equal, dumped and diffed against a base build at
two ε. The fixes cover one undisclosed deviation and the rows that should
pin it:

- A same-carrier joint that validation decides is recorded as
  `SameOriented`, where the PR body (ruling 3) and the spec (§8) say
  `Tangent`.
- No document-level row pins that arm, and a lattice-built profile
  reaches it.

None of the fixes moves geometry.

## Findings

**MINOR-1. A same-carrier decided joint is recorded as `SameOriented`,
not `Tangent{aligned:true}`.**
- Where: `editor-core/src/coincide.rs::name_junctions` maps
  `JointCarriers::Same => topo::Relation::SameOriented`.
- What the text says instead:
  - ruling (3) in the PR body: "relation `Tangent { aligned: true }`";
  - spec §8: "recorded as a `Coincidence { relation: Tangent, site:
    ProfileJunction }`";
  - README V6: "same-carrier continuation is a tangent joint".
- Either choice is defensible: `SameOriented` is §1's continuation
  relation and reads "continues on one carrier with". But the deviation is
  undisclosed, the PR body says the opposite of what the code does, and
  Python readers get an undocumented `same_oriented` at a
  `profile_junction`.
- Shown by inspection, and by mutant M7, which survives. Sure.

**MINOR-2. No row pins that relation at the document, and the lattice
reaches the arm.**
- Mutant M7 (`Same => Tangent{aligned:true}`) survives all 143
  editor-core tests matching `coincid|profile|tangen|junction|check`.
- Python covers only the `tangent` arm.
- The arm is live. Probe `r1_lattice_same_carrier_decided`:
  - a lattice-built loop,
    `Open.at(0,0).line_to(1000,0).line_to(1001,y).line_to(1001,5).line_to(Start)`;
  - `ConstructedProfile::validate` at y = 0.5ε and at y = ε returns
    `decided_joints = [(1, Same)]`;
  - the cause: the lattice levers the turn by the long arriving arm, so it
    sees a corner, while validation reads the short leg's far end, so it
    sees one carrier.
- One document row for this shape, asserting its relation, closes MINOR-1
  and MINOR-2 together. Shown by execution. Sure.

**NOTE-1. Spec row 21's lattice premise overclaims.**
`tangent_joints.rs::the_lattice_constructs_every_tangent_joint_it_builds`
is red "if a lattice site mints a zero-turn joint without constructing
it". It checks only the coverage corpus, which has no ε-scale junction.
The lattice does build decided joints:
- the PR's own `coincidence_door` row (φ = √ε);
- the probe above.
Read as "no verb row's own construction leaks", it holds. Its name and doc
say more. Execution. Sure.

**NOTE-2. The refusal count moved 34 → 32, not 33 → 32.**
- Parsed `FILED_NO_RECOURSE` at both commits: the shape-guard block holds
  34 entries at the base and 32 at the head.
- Two entries left: `Profile/UndeclaredTangency` and
  `Profile/TangentJointOutOfRange`.
- The base item's "33" was already stale. The head's "32" is correct, but
  the PR body's arithmetic is not. Execution. Sure.

**NOTE-3. The fillet dump's verdict column now measures a different
gate.**
- `fillet_stored_tangency.rs::validates` moved from `Profile::validate`
  (`Consistency::Decide`) to `ConstructedProfile::validate`
  (`ByConstruction`, which skips the arc-consistency checks).
- No verdict moved (C3), so the re-baseline is honest. But the golden no
  longer exercises the table gate.
- Its doc comment also lost the history of three earlier re-baselines.
- Inspection plus the dump diff. Sure.

**NOTE-4. `DISCIPLINES-DESIGN.md` calls profile tangency's position
"auto-record (D10)" (`:167`, `:229`, `:577`).**
- DS7 defines `auto-record` as a persisted record that later evaluations
  diff ("changes complain"). G's record is per-evaluation and diffs
  nothing.
- These reclassification edits go beyond the spec's "DISCIPLINES DS-Q6".
  They need Ev's eye whatever the label (the PR is already not
  lane-merged).
- Inspection. Likely.

## The claims

- **C1. No tangency is stored: holds.**
  - `ProfileLoop` is `{vertices, segments}`.
  - The only joint list is `ConstructedLoop.joints`. The lowering writes
    it from the verbs (`Core::finish`, `ConstructedLoop::carrier`); the
    only other writer is the `fixture` door, which is cfg
    `test`/`test-support`, and no shipped crate enables that.
  - `LoopCanonical.tangent_joints` is a derived per-evaluation record.
  - Nothing serializes either. `"tangent_joints"` survives only in the
    pre-program `bool13_goldens`.
  - Grep over `crates/*/src`. Sure.
- **C2. Holds for the profile node's f64 lane.**
  - M3 (drop the record) reds 9 rows.
  - M8 (drop the `wire.rs` emission) reds the `coincidence_door` row.
  - M2 (record constructed joints too) reds 18.
  - `TangencyContradicted` still verifies (M1).
  - Library callers that validate a raw `ProfileLoop` (`sweep::skin`)
    hold `decided_joints` but reach no door.
  - The interval and guided lanes were not exercised. Likely.
- **C3. Bit-equal: holds.** Three pins checked against a base build:
  1. **`fillet_stored_tangency` dump** (`CAD_DUMP_FILLETS=1`), diffed line
     by line at ε default (48 → 50 lines) and 1e-12 (40 → 42):
     - 0 verdict moves and 0 vertex or arc-bit moves at both ε;
     - 5 joint lists only re-ordered (each sorted-equal);
     - 2 rows added, `shared coverage corpus 14/15` (the circle forms).
  2. **`lift_census` printout, base vs head:**
     - every common row has identical class, steps, seam and worst-ulp;
     - `unequal_split` (REFUSED → ValueEqual) and `collinear_run`
       (WALL → BitIdentical) moved as stated;
     - `half_disc_undeclared` folded into `half_disc` with the same
       figures;
     - the tallies match: Bits 5 → 6, Value 7 → 7.
  3. **The refusal count:** 34 → 32 (NOTE-2).
  Sure.
- **C4. Holds.**
  - The retired variant, its tag and its concision entry are gone.
  - `TangencyContradicted`: M1 reds 4 rows.
  - `TangentJointOutOfRange`: M4 reds 2.
  - `TangentJointOnFullTurn`: M5 reds `one_segment_loop`.
  - `JunctionTangent`/`SeamTangent`: not mutated.
  - Sure.
- **C6. Holds except:**
  - MINOR-1;
  - the `.pyi` and `checks.rs` relation docstrings, which do not say a
    profile junction can be `same_oriented`.

  Otherwise:
  - the tags (`tangent`, `cusp`, `profile_junction`) match `topo`;
  - README V5/V6 and DS-Q6 match the code;
  - the filed P3 `lift-same-carrier-repair-is-reached-by-no-row` is
    accurate, since the census diff shows both former reachers now lift.

  Sure.
- **C7. The rows bite.**

| Mutant | Change | Result |
|---|---|---|
| M1 | no `TangencyContradicted` | 4 red |
| M2 | record constructed joints | 18 red |
| M3 | no record | 9 red |
| M4 | no range check | 2 red |
| M5 | no full-turn check | 1 red |
| M6 | ask the heading question on same-carrier joints | 7 red (m10/sym K-stream) |
| M7 | `Same` → `Tangent` | **survives** |
| M8 | no emission | 1 red |

## The rulings

1. **Right.** `ConstructedLoop` carrying the constructed joints is the
   spec's "derived at lowering from the constructors". Sure.
2. **Right.** The `LoopCanonical` record is a per-evaluation decision
   record that guided lanes compare (`TangentJoints`/`CuspJoints` flips).
   Likely.
3. **Same-carrier joints in the set: right** (D1). **Recorded under a
   name other than the one the PR claims:** `SameOriented`, not
   `Tangent{aligned:true}` (MINOR-1). On the merits a continuation is not
   a tangency between two carriers, so `SameOriented` is arguably better.
   Either way, pick one, state it in the spec or README, and pin it
   (MINOR-2). Sure.
4. **Right.** A circle's subdivision vertices are its construction. The
   verification still runs, and without this every circle would report a
   finding. Sure.
5. **Right.** Pinned by `an_arc_extension_leaves_no_joint_to_decide`.
   Likely.
6. **Right on its merits, not only for the pins.**
   - A same-carrier reversal never reaches the joint pass. Probes
     `r1_same_carrier_reversal` and `r1_cocircular_reversal` are refused
     by the simplicity pass (`NonSimple … Overlap`, "endpoint contact").
   - M6 shows that asking only adds K-stream decisions (+2
     `path_junction_side` per circle), with no verdict or refusal change.
   - Caveat: main asked this question for lattice continuations, and those
     decisions are dropped too. No pin saw them.
   - Sure.
7. **Honest.**
   - Lattice indices are in range by construction: `Core.tangent` holds
     vertex indices, and `carrier` uses `0..n`, empty when n < 2.
   - The only other source is `fixture`, which no shipped build carries.
   - Sure.
8. **Right.** Unsure about one edge: `prove` treats two equal `Piece`
   cells as `SameConstruction`, so a decided junction inside one piece ref
   would be silently proven. I found no verb that does this outside its
   own constructed joints. Not exercised.
9. **Right.** Evidence under C3. The now-unreached repair is filed (P3).
   Sure.
10. **Right in substance.** The "auto-record" label is wrong (NOTE-4).
    Likely.

## Style

- **Two parallel enums.** `validate.rs` `JointCarriers {Tangent, Same}`
  sits beside `seg.rs` `JointClass {Tangent, Transversal, SameCarrier}`,
  mapped arm for arm in `judge_joints`: a second spelling of one
  classification. Q1. Likely.
- **A record that is not self-contained.** A `DecidedJoint` does not carry
  its own heading. `coincide.rs::name_junctions` rebuilds `aligned` by a
  `binary_search` over the separate `cusp_joints` list. Q7. Likely.
- **The escalation text still offers a declaration.** A profile-junction
  escalation still renders `COINCIDENCE_RECOURSE`'s "declare the
  coincidence" (seen in the `chord_side` escalation of my probe), and
  nothing is left to declare there. It is noted only in
  `paths-refusals-short-of-the-shape-guard.md`'s new section, with no
  schedule beyond that open item. Q4/Q6. Likely.
- **Two bands over one junction.**
  - The lattice refuses an exact zero-turn junction (`JunctionTangent`),
    yet a φ = √ε one builds and is recorded `Tangent`. The lattice's
    levered margin and validation's clearance margin disagree there, and
    the new rows are built in that gap.
  - The spec sanctions it (§8). It still echoes the transcript's "why do
    I have to declare what I already said with numbers?" at the authoring
    door.
  - NOTE-1's overclaiming test is the same class.
  - Q3/Q7. Unsure whether it is in scope.
- **A golden's provenance was deleted.** `GOLDEN_DEFAULT`'s history now
  lives only in PR bodies (NOTE-3). Q2. Likely.
- **One-implementor trait.** `validate.rs`'s private `LoopInput` has two
  impls, one returning `&[]`, kept only to thread `constructed_joints`
  through a generic `L`. A `(&ProfileLoop, &[usize])` pair would do. Q7
  taste. Unsure.
- **Q8.** Read `judge_joints` through `canonicalize_loop` and all of
  `coincide.rs`'s new code end to end; I did not read the whole 2.7k-line
  `validate.rs`. Nothing further.

## Exercised, and not

**Exercised by execution** (private target dirs `tgt-r1` for the head,
`tgt-mut` for mutants, `tgt-base` for the base):

- **The slow set** (`--profile default`) over `profile` and `editor-core`:
  - ε default: 3494/3495;
  - `CAD_TOLERANCE_EPS=1e-12`: 3494/3495.
  - The one red in both is
    `name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time`
    (12–14 s). It fails identically at the **base** on this machine
    (14.0 s), so it is not this PR's.
- **C3:** the dump and census diffs.
- **C7:** M1–M8.
- **Probes:** two same-carrier reversals, and a lattice-built same-carrier
  decided joint.

**Not exercised:**

- sweep, mesh, stl, pncad, viewer, the Python suite and the tour;
- the interval and guided lanes for a decided joint;
- the `JunctionTangent`/`SeamTangent` mutants;
- ruling 8's equal-piece edge.

The probes stay in a scratch worktree. Nothing was pushed to the PR
branch.
