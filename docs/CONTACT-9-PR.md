# CONTACT-9: a boolean side code's On is a distance

This file is the PR body for the landing. The orchestrator folds it into
that PR and deletes it at merge.

Carries `work/contact/boolean-side-codes-lever-a-chord-direction-where-a-zero-is-read-as-on`.
Spec: `docs/CONTACT-9-SPEC.md`. The unit traced every Zero of the
boolean's and the splitting lane's side codes (step 1), tried to make
each fail (step 2), and was sent on to step 3 by the orchestrator: the
Zeros gave no wrong answer, but they refused ordinary geometry with a
message that blamed the kernel.

## What was wrong

`boolean::sectors::side_code` read a sector bound's side of a face
plane as `d̂·n̂ × arm`, with `arm` the SHORTER chord of the sector
(`vtxfac`, `flank_key`) or of the sector pair (`pair_search`,
edge-edge membership). A levered reading has the right sign whenever it
is definite. Its Zero moves with the lever, and here the Zero was a
verdict: `SideCode::On`, "this bound lies in the plane". A 10 m edge
beside a 1 mm one read On while its far end stood 5000 bands off.

The trace (step 1) found each such On backstopped, never answered
wrongly:
- **A single On bound.** On-edge resolution (`vtxfac`) or the on-edge
  event engine (`recl_edges`) drops or moves a section germ at the
  vertex. The two faces flanking the bound each keep a section end with
  no partner, and the join refuses `UnpairedLooseEnds`, "(kernel bug)".
- **All four bounds On.** The pair goes to the carrier ladder, levered
  at the same arm: undeclared, it refuses as a coincidence ("the
  geometry coincides"); declared, the door contradicts it.

Step 2 built the poses through the public ops: a needle's tip on a
slab's face, on a block's corner, and a tilted wedge. Every one refused.
The same pose with 1 m edges answered correctly.

## What changed

### A line bound is read at its far vertex, in metres

`BoolSector` carries a `Reach` per bound, which says what stands behind
it and so how it is read (`side_code`):
- `Reach::Chord` (a line edge): its far vertex's signed distance from
  the plane through the base vertex, `n̂·(q − p)`, through
  `sector_shape::point_side`. The splitting lane's vertex classes now
  go through the same reader (`splitting/classify.rs`), so the two
  lanes read a real vertex's side in one place. A Zero is a real one:
  the edge lies within the band of the plane, so every resolution built
  on it is ε-true of it.
- `Reach::Extent` (a conic or fitted edge): its departure direction
  levered at the edge's OWN extent. A definite sign is exact. Its Zero
  is a first-order tangency; the site says what that does not certify,
  and the residue is filed (below).
- `Reach::Bisector`: a subdivision direction, levered at its own
  sector's arm (it was the pair's shorter arm in `pair_search`).

The curvature charge is unchanged for a definite first-order verdict;
it now also passes when the displacement at the bound's own reach
exceeds the sagitta there, so a plane (lever `f64::MAX`) always passes a
definite metric reading.

### Coplanar needs On bounds

Two verdicts called a sector coplanar from its normal alone, at the
shorter arm, and then overwrote or bypassed the bounds' readings:
- `pair_search`'s `bool_faces_parallel` sent the pair to the overlap
  test and, all On, to the carrier ladder;
- `vtxfac`'s `bool_sector_coplanar` lumped both bounds.

Parallelism now only proposes coplanar: the bounds must also read On at
their reach. A face whose normal agrees at 1 mm but whose 10 m edge
dips 500 bands reads that edge In and takes the crossing path.

Two germ-line gates levered the same parallelism at the shorter arm and
refused "coplanar" as a kernel invariant: `insert::germ_dir` and
`vtxfac::pierce_germ_dir`. They are now levered at the sectors' farther
reach (`BoolSector::span`). A pair or sector with a bound read
definitely off the other plane has that reading at most `|n_a × n_b|`
times its reach, so the gates agree with the classification by
construction.

### Bisector Zeros refuse where they could decide

A bisector's code only relays its physical sector's side between two
twins, so with mixed neighbours its Zero changes no topology. With both
neighbours definitely on one side, it cannot read Zero when K > 2:
- the shorter bound's own reading gives `s·a ≥ K·zero`;
- a reflex bisector then reads at least `K·zero/2`;
- a straight-band one reads at least `a·cos δ`.

The argument is written at `vtxfac`'s on-edge resolution and at
`recl::resolve_bisector_graze`. Both now refuse, typed
(`Escalated`, predicate `bool_sector_bisector_side`), where a bisector
reads On between two readings on one side, rather than resolve it: that
is the K ≤ 2 case, where a reflex sector would be read as not crossing.

**The parity argument's precondition (the orchestrator's item 3).** The
metric reading removes the dependence for every line bound: an On is
ε-true, so no wrong germ reaches the join from one. For bisectors the
dependence remains, and it is enforced at the two sites above, not at
the join: the configuration refuses loudly where K ≤ 2 could make it
wrong. No global K check was added.

### The join's refusal no longer claims a kernel bug

`SplitJoinError::UnpairedLooseEnds` said "(kernel bug)". The levered
line-bound path is gone and the bisector path refuses before the join,
but a curved edge's first-order tangency (`Reach::Extent`) is still a
levered On and has not been excluded. The message now names that cause
and calls the rest a kernel defect. The variant and its fields are
unchanged.

## Rows

All new, in `crates/topo/tests/contact9_side_codes.rs` unless noted.
Each passes at ε = default, 1e-6 and 1e-12.
- `a_pierce_reads_a_dipping_edge_at_its_far_vertex` (`vtxfac`) and
  `a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`
  (`pair_search`). The needle poses from step 2, which refused at the
  join, now answer. Each is checked against ground truth, with a
  control of 1 m edges beside it:
  - `∩` has the analytic sliver's corners (within ε);
  - `∩` has the analytic volume and passes tier 3, where the volume
    door resolves the sliver;
  - a point inside both operands is in `∩` and `∪` and out of `−`;
  - `−` and `∪` pass tier 3.
- `a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`
  (`vtxfac`'s coplanar lump). A wedge on the block's top, with a 1 mm
  edge on that face and a 10 m edge dipping 500 bands:
  - it answers with the corners, volume and membership;
  - it used to refuse "geometrically coplanar sector with
    definitely-distinct plane" as a kernel invariant;
  - declared `Rest`, it is contradicted at the door.

  Tier 3 is not run on it: its seam is filed (below).
- `a_dip_inside_the_band_still_reads_on`. A dip of `ε/2` is a real On:
  the pose is a touch, `∪` is the sum and `−` leaves the slab.
- `the_splitting_twin_reads_the_dipping_edge_at_its_far_vertex`. The
  split of the needle is the analytic sliver.
- `boolean::sectors::tests::a_pair_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`
  (unit). The pair search itself, on two synthetic sectors: the record
  reads `(On, In)`, not all-On. It is the `pair_search` twin of the
  wedge row. The v-v wedge pose fails at base with 1 m edges too
  (filed, below), so no end-to-end fixture could pin this.

The volume floor, `RESOLVED_VOLUME = 1e-15` m³: at ε = 1e-12 the sliver
is ~4e-19 m³, and the volume door returns half of it for a tetrahedron
whose vertices are exact; tier 3's signed-volume check reads the same
integral negative. Below the floor the corner set, which fixes the
sliver and so its volume, is the oracle. This limit is the same with 1 m
edges, where every reading is definite.

**Rows that moved.** None outside this file. `topo` and `sweep` at three
eps, all of `editor-core` and `test-utils`, and the Python suite keep
their answers. Inside the unit: `sectors.rs`'s existing unit rows now
pass `Reach::Bisector(arm)` at the arm they used before. That is the
same levered reading, and every value is unchanged.

## Sweep for the class (discipline §5)

Pattern: `Margin::levered(` in the sector code (`boolean/sectors.rs`,
`vtxfac.rs`, `recl.rs`, `insert.rs`, `splitting/neighborhood.rs`,
`splitting/rules.rs`, `sector_shape.rs`). Hits and disposition:

| Site | Reading | Disposition |
|---|---|---|
| `sectors.rs` `side_code` | a bound's side | fixed: metric for line bounds |
| `sectors.rs` `bool_faces_parallel` | coplanar | fixed: proposes only; On bounds required |
| `vtxfac.rs` `bool_sector_coplanar` | coplanar lump | fixed: the same |
| `insert.rs` / `vtxfac.rs` `bool_germ_line` | germ line exists | fixed: levered at the reach |
| `sectors.rs` `within` | direction in sector | not this class: a Zero widens the candidate set, and the codes decide |
| `sectors.rs` `parallel_same`, `recl.rs` `parallel_same_dir` | "same ray" | filed (below): a verdict, levered at the shorter arm and at 1 m |
| `recl.rs` `bool_dir_same` | the sign of a cosine near ±1 | not this class: only the sign is read, and a Zero refuses |
| `insert.rs` `bool_strut_order` | an order | not a Zero-as-On |
| `splitting/neighborhood.rs` `split_bisector_side` | bisector side | K-free: rule (b)'s AOA→BELOW / BOB→ABOVE is correct for a reflex bisector, and the straight band cannot read Zero |
| `splitting/rules.rs` `split_sector_coplanar` | coplanar | levered at the face extent, which bounds every face point's offset from above; sound |
| `sector_shape.rs` wideness rungs | subdivide or not | Zero and in-band both subdivide, which is sound at any angle |

What the pattern cannot match: a levered reading spelled without the
door (`x * arm` fed to `Margin::of`). The second pass grepped
`Margin::of(` in the same files for a product with an arm. It found two:
- the curvature charge in `side_code`, a first-order-versus-sagitta
  comparison and not a Zero verdict;
- the splitting lane's conic departure (`t̂·extent·n̂`), which is
  levered at its own extent and descends to second order at its Zero. The seam description in
`boolean/ops.rs`, found by the wedge row, is outside the sector code and
is filed.

## Filed

All in `work/contact/`:
- `boolean-conic-side-code-zero-is-first-order`: a curved edge's Zero
  is a first-order tangency. The splitting lane descends to second
  order there; the boolean does not.
- `seam-description-reads-a-dihedral-at-the-seams-own-length`: a 1 mm
  seam between faces 5e-8 rad apart is left a scaffold at rest. It is
  the wedge row's tier-3 finding.
- `a-wedge-edge-on-a-block-top-from-its-corner-refuses-at-the-join`: a
  pre-existing `UnpairedLooseEnds`, at base and with 1 m edges.
- `boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm`.

The declaration door's 1 m lever is the `tang` row's
(`work/tang/torus-carrier-axis-margin-is-levered-by-one-not-the-ring`).
Evidence for its plane arm was added there, and this unit does not widen
to it. `docs/predicate-dimension-audit.md` rows for the changed
predicates are updated.

## Local results

Head `3d1966b66` (code; this file is committed after it):
- `topo` + `sweep` at ε = default, 1e-6 and 1e-12: all pass;
- `editor-core` (all): 2272 passed;
- `test-utils`: 79 passed;
- the Python suite: 857 tests, OK. It ran through
  `crates/pncad-py/run-python-tests.sh`, which stages the cdylib; it did
  not build a maturin wheel (this box has no pip or venv);
- clippy (`--workspace --all-targets --all-features`, `demos/tour`,
  `demos/wild`, `pncad-py --features python`), `cargo fmt --all
  --check`, `scripts/gates/*.sh` and `work.py lint`: clean.
