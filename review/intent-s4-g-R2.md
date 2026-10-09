# Review R2: INTENT stage 4 PR G (#4405), "tangent joints are derived"

Frozen head `74821aee98`, base `db51132ff7`. Correctness lane plus the style lane.

## Verdict: APPROVE-WITH-FIXES

The mechanism is right and its rows bite. No tangency is stored, and
every Zero junction no constructor made is recorded at B's door.
Constructor joints are verified and not recorded, and the bits are
unmoved. Two things need fixing before merge, both on the record side.
The PR body's ruling 3 misdescribes the record the code emits for a
same-carrier junction, and no row pins it. The PR's own vocabulary
sweep is not clean by its own pattern. Neither breaks a claim's
substance, so there is no MAJOR.

## Findings

**MINOR-1. A same-carrier junction is recorded as `SameOriented`, not
`Tangent { aligned: true }`. The PR body says the latter, spec §8 says
"each recorded as `Coincidence { relation: Tangent … }`", and no row
pins either.** `editor-core/src/coincide.rs`
`name_junctions` maps `JointCarriers::Same => topo::Relation::SameOriented`.
- *Demonstrated by execution.* I ran a probe appended to
  `coincidence_door.rs` (not pushed). The document profile is
  `At(0,0) → LineTo(10,0) → LineTo(10.001,1e-10) → LineTo(10.001,5) → Start`.
  The lattice reads the junction as a corner, and validation reads it
  as one carrier continuing.
  - The node's rows are `[(SameOriented, ProfileJunction)]`.
  - The lint says "loop 0 step 1 continues on one carrier with loop 0
    step 2, decided from values (a profile junction no constructor
    made, margin 1e-10)".
- On the merits the code's choice is the better one. topo documents
  `SameOriented` as "one carrier, … a continuation", which this is,
  and `Tangent` would be the wrong name for carrier identity. So the
  fix is to the description and the pin, not the code:
  - say it in the PR body and in spec §8, or as a disclosed deviation;
  - add a document row for the same-carrier record, because
    `coincidence_door.rs`'s only new row is line/arc;
  - make `.pyi` `Coincidence` and `CheckEvidence.relation` say that a
    `profile_junction` row can be `same_oriented`.
- Confidence: sure.

**MINOR-2. The vocabulary sweep is not clean by its own pass-2 pattern,
and several of the survivors are now false.** The PR's pattern
`(un)?declared?[ _-]+(tangen|joint)|tangen\w*[ _-]+declar` still
matches about 30 lines in `crates/profile/src`. The false ones:
- `path.rs:3031`: "validation asks it of every declared joint too".
  Validation now skips same-carrier joints (ruling 6).
- `path.rs:4266–4269`, `cusp_kernel`: "The declaration it emits … the
  profile data gate judges declared joints … needs no second flag".
- `test_support.rs:9`: "with no declared-tangent joints". The field is
  gone.
- `path.rs:2356`: `Core`'s doc says "the declared joints".
- `path.rs:2664`/`:2741`: `declare_last`, "(the raw
  `declare_tangent`)", and `declare_seam`'s "EVERY declared arrival
  lands here".
- `path.rs:6321/6328`: the assertion text says "declared tangency".
- `program.rs:1092`/`:1354`, `family.rs:1123`, `sugar.rs:311`.

The PR body names five pass-2 hits, all fixed, plus verb-describing
prose it kept. These others were neither fixed nor dispositioned.
Demonstrated by inspection (grep). Confidence: sure. This is a class,
so the fix should re-run the pattern rather than patch this list.

**NOTE-1. Ruling 6's skip is load-bearing for nine rows, but it is right
on its merits.**
- *Mutant M5:* drop `carriers == JointCarriers::Tangent &&` in
  `validate.rs` `judge_joints`, so the heading question is asked of
  same-carrier joints too.
  - The profile suite stays green: 568/568.
  - editor-core goes red on nine rows: `m10_bulge_interval` ×4,
    `m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`,
    `sym11_exact_channel_rows`, `sym_9_retry_interval` ×2 and
    `m10_9_pins_interval`. The ninth red, `name_words_rows`, is the
    known timing row.
  - So the skip moves only K-stream and lane pins, not behaviour.
- *Merits, by execution:* a probe (`r2_probes.rs`, not pushed) shows
  that a same-carrier retrace, line or arc, is refused
  `NonSimple { kind: Overlap }` before 3b runs. The skipped question
  could only ever answer "does not reverse", so skipping it is honest,
  and the comment at the skip is true.
- One consequence the PR does not state: main asked this question of
  every *declared* continuation joint, `.line()` continuations
  included. Head asks none, so K-streams for continuation-built loops
  are shorter than on main. No pin covers that; I did not measure it.
- Confidence: sure for the mutant and the probe; likely for the
  consequence.

**NOTE-2. Ruling 7's kernel-defect ending is honest, as far as I can
show.**
- The constructed indices come only from `Core::declare_last` (the
  chain's own last vertex), `declare_seam` and the seam-fillet push
  (both 0), and `ConstructedLoop::carrier` (`0..n`).
- A probe shows the lattice refuses a one-arc chain closing on
  `Start.arrives_tangent()`: "a closed carrier is a circle primitive,
  not a chain leg". So `TangentJointOnFullTurn` is fixture-only.
- I did not prove that no chain closing drops a vertex after
  `declare_last`. Confidence: likely.

**NOTE-3. The `work/band/declared-joint-kind-zero-margin-reads-smooth`
release note is slightly wrong.** It says `junction_reverses` maps
`Zero` "now for every tangent joint, constructed or decided". It is
asked only for distinct-carrier joints. The bullets still stand.
Confidence: sure.

**NOTE-4. `validate::table_tangent_joints` validates with
`loop_index` 0 hard-coded.** A lift's `Unclassified(ProfileError)` text
therefore names "loop 0" whichever loop was lifted. The lift API is
per loop, so this may be acceptable, but the sentence it renders is
false for any later loop. Confidence: likely.

**NOTE-5. DS-Q6 "settled by D10" is binding text in a design doc.** It
follows from D1/D10 as ratified, and the PR is marked "not to be merged
by the lane", so this is only a reminder that it rides Ev's sign-off.
Confidence: sure.

## Claims

- **C1 (nothing stored): holds.** Inspection:
  - `ProfileLoop` is `{vertices, segments}`, and nothing serializes
    joints.
  - `LoopCanonical.tangent_joints` is compared against the re-derived
    set (`validate.rs:2683`), never adopted.
  - `ConstructedLoop.joints` is the lowering's output, sealed to
    `path`, with the `fixture` door gated on `test`/`test-support`.
  - The lift derives the set through `table_tangent_joints`.
- **C2 (recording): holds, apart from MINOR-1's relation name.**
  - `Node::Profile` is the only place a `ValuePayload::Profile` is
    minted (`wire.rs:1634`), and it routes through `name_junctions`
    into `with_coincidences`.
  - Guided and lane validation run the same `judge_joints`.
  - Mutants M1 and M3 (below) show that recording a constructor joint
    and dropping a decided one both go red.
- **C3 (bits): holds.** Three checks:
  1. The `fillet_stored_tangency` dump was diffed base against head at
     ε 1e-9, 1e-6 and 1e-12 by execution: 48/56/40 base loops against
     50/58/42 head. In every row:
     - no vertex or arc bits moved;
     - no verdict moved;
     - no joint set moved as a set;
     - five lines differ only in joint order;
     - the two new loops are `shared coverage corpus 14/15`, the
       circle forms.
  2. `sweep/tests/bitdump.rs`: its goldens are unchanged in the diff
     (the stripped fixture joints only), and it passes at head.
  3. `lib_g16_corpus_name_digests` and the m10/sym K pins are
     unchanged in the diff and pass at head.
  The `lift_census` class moves (`unequal_split` Refused→Value,
  `collinear_run` Wall→Bits) are behaviour changes by design, filed as
  P3 `lift-same-carrier-repair-is-reached-by-no-row`. They are not bit
  pins.
- **C4 (retired and kept refusals): holds.**
  - `UndeclaredTangency` is absent from code and tags.
  - The restated rows assert a recorded joint where it matters
    (`lib_u3_sections`, `a_swept_cusp…`, `test_paths.py` asserts no
    coincidences for the cusp verb).
  - Mutants of the kept refusals:
    - M2 (no `TangencyContradicted`) turns red:
      `a_constructed_joint_the_carriers_cross_is_contradicted`,
      `abandoning_a_fillet_exit_leg…` and
      `interval_lane::tangent_joints_are_derived_alike_at_interval`.
    - M4 (no full-turn check) turns red:
      `one_segment_loop::a_constructed_joint_on_a_full_turn_is_refused`.
  - The out-of-range check and `JunctionTangent`/`SeamTangent`:
    inspection only.
- **C5 (rulings):** see below.
- **C6 (surfaces): holds, apart from MINOR-1.**
  - The Python tags `tangent`, `cusp` and `profile_junction` agree
    with `coincide.rs`.
  - V5 and V6 read as spec §8 quotes them.
- **C7 (rows bite):** four mutants of my own, all killed except M5,
  which the profile crate cannot see and editor-core pins can:
  - M1 (record constructed joints): 18 rows red.
  - M2: 4 red.
  - M3 (record nothing): 9 red.
  - M4: 1 red.
  - M5: 0 red in profile, 9 red in editor-core.

## Rulings

1. **Right.** `ConstructedLoop` is the lattice's only output, so it is
   the honest carrier.
2. **Right.** The structure record is compared, not trusted.
3. **Right that a same-carrier joint is in the set.** Wrong as
   described: the code records `SameOriented`, which is the better
   name. See MINOR-1.
4. **Right.** The circle has no rows (pinned).
5. **Right.** Pinned by `an_arc_extension_leaves_no_joint_to_decide`.
6. **Right on its merits.** The retrace is refused upstream
   (demonstrated), although pins also depend on it (NOTE-1).
7. **Honest, as far as I can show** (NOTE-2).
8. **Acceptable.** `Origin::Piece` is reached only for piece cells.
9. **Right.** It retires the P3-filed repair path.
10. **Right in substance;** it needs Ev (NOTE-5).

## Style

- **S1. Three parallel per-joint lists and a fourth enum.**
  `validate.rs`: `tangent_joints`, `cusp_joints` and `decided_joints`
  (`DecidedJoint` has no `aligned`). `name_junctions` re-derives a
  decided joint's alignment by `binary_search` in `cusp_joints`.
  `JointCarriers` is `seg::JointClass` less `Transversal`. One
  per-joint record would hold all of it. Confidence: likely.
- **S2. Two traits answer "validate by provenance".** `LoopInput`
  (`validate.rs`) and `SectionLoop` (`sweep/src/skin.rs`) each
  dispatch `ProfileLoop` versus `ConstructedLoop`. Confidence: likely.
- **S3. `table_tangent_joints` is a second, partial validation entry**
  (segments plus the joint pass only). Its doc concedes that a loop
  passing it may still refuse. The lift now has its own copy of
  "derive the set". Confidence: likely.
- **S4. `ConstructedLoop::fixture` is `with_tangent_joints` under a
  new name:** a hand-authoring door for joints, test-gated as before.
  It is justified, but the old door survived as a renamed one.
  Confidence: unsure.
- **S5. `TangentJointOutOfRange`/`OnFullTurn` stay public, Python-tagged
  `ProfileError` variants whose only text is the kernel-defect
  ending.** A user-facing enum carries two fixture-only arms. I would
  have asserted. Confidence: unsure.
- **S6. `the_lattice_constructs_every_tangent_joint_it_builds` skips
  every corpus row that refuses,** with a floor of 10. A verb whose row
  started refusing would drop out silently (Q3). Confidence: likely.
- **S7. The sweep report claimed a clean code pass that its pattern
  contradicts** (MINOR-2). A clean sweep is evidence about the
  sweeper. Confidence: sure.
- **S8. Leftover `declare_*` names** in `Core` (`declare_last`,
  `declare_seam`) beside a field now documented as constructed joints:
  two spellings for one act. Confidence: sure.

Style questions exercised: Q1 (S1–S3), Q2 (MINOR-2, S8), Q3 (S6), Q4
(NOTE-3, `path.rs:3031`), Q5 (lib.rs crate doc against the code:
agrees), Q6 (the lift deviation is filed P3, done; MINOR-1's deviation
is undisclosed), Q7 (S5). Q8 was not exercised: I read `validate.rs`'s
header, joint pass, canonicalization and guided path, not the whole
file.

## Runs and what I could not exercise

- Private `CARGO_TARGET_DIR` per worktree: head `/home/user/tgt-r2`,
  base `/home/user/tgt-base`.
- **ε default:** `nextest --profile default -p profile -p editor-core
  -p sweep -p mesh -p stl -p pncad` passed 6470/6471. The one failure
  is `name_words_rows::a_large_table…`, the timing row. It is also red
  run alone at head (13.2 s), and red the same way at base
  `db51132ff7` (12.5 s), so it is this machine's speed, not this PR.
- **ε 1e-6, profile and editor-core:** passed 3494/3495, with the same timing row as the only failure.
- **Not exercised:** pncad-py and the Python suite, the viewer, the
  tour, clippy and the doc gate. I also did not mutate `JunctionTangent`
  or `SeamTangent`.
