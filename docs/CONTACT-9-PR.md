# CONTACT-9: a boolean side code's On is a distance

This file is the PR body for the landing. The orchestrator folds it into
that PR and deletes it at merge.

Carries `work/contact/boolean-side-codes-lever-a-chord-direction-where-a-zero-is-read-as-on`.
Spec: `docs/CONTACT-9-SPEC.md`.

The unit went through these stages:
- It traced every Zero of the boolean's and the splitting lane's side
  codes (step 1).
- It tried to make each one fail (step 2).
- The orchestrator sent it on to step 3: the Zeros gave no wrong
  answer, but they refused ordinary geometry with a message that blamed
  the kernel.
- A full review of `9f414db16` asked for changes (no wrong answers).
  The fix pass is folded in below.

## What was wrong

`boolean::sectors::side_code` read a sector bound's side of a face
plane as `d̂·n̂ × arm`. `arm` was the SHORTER chord:
- of the sector, at `vtxfac` and `flank_key`;
- of the sector pair, at `pair_search` and edge-edge membership.

A levered reading has the right sign whenever it is definite. Its Zero
moves with the lever, and here the Zero was a verdict: `SideCode::On`,
"this bound lies in the plane". So a 10 m edge beside a 1 mm one read
On while its far end stood 5000 bands off.

Step 1 found each such On backstopped, never answered wrongly:
- **A single On bound.** On-edge resolution (`vtxfac`) or the on-edge
  engine (`recl_edges`) drops or moves a section germ. The join then
  refuses `UnpairedLooseEnds`, "(kernel bug)".
- **All four bounds On.** The carrier ladder, levered at the same arm,
  refuses undeclared, and the door contradicts a declared pair.

Step 2's poses all refused through the public ops:
- a needle's tip on a slab's face;
- a needle's tip on a block's corner;
- a tilted wedge.

Each answered with 1 m edges.

## What changed

### A line bound is read at its far vertex, in metres

`BoolSector` carries a `Reach` for each bound, which says what stands
behind it and so how it is read (`side_code`):
- **`Reach::Chord`** (a line edge): its far vertex's signed distance
  from the plane through the base vertex, `n̂·(q − p)`, through
  `sector_shape::plane_offset`.
  - It is ONE reading: the value decided is the value the curvature
    charge uses.
  - The splitting lane's vertex classes go through the same reader.
  - A Zero is a real one: the edge lies within the band of the plane.
- **`Reach::Extent`**: a conic's departure tangent, or a fitted edge's
  end-to-end chord, levered at the edge's own extent. A definite sign is
  exact; a Zero is first order (filed, below).
- **`Reach::Bisector`**: a subdivision direction, levered at its own
  sector's arm.

`start_edge()` and `end_edge()` are now derived from the reach, so a
test helper cannot build a real edge with a bisector's reach.

The curvature charge still passes a definite first-order verdict at the
arm. It now also passes at the bound's own reach, so a plane (lever
`f64::MAX`) always passes a definite metric reading.

### A parallel pair at a vertex pair is a coincidence, or refuses

In `pair_search`, a pair whose normals read parallel at the shorter arm
(`bool_faces_parallel` Zero) is a near-coincidence at the vertex.

**Why it cannot be anything else.** The bound that sets the arm reads at
most `arm·|n_a × n_b|` off the other plane, so it is On, or in band,
which escalates.

**What happens to it.** The pair goes to the carrier ladder as a
coincidence, every code On, as on main. Undeclared, it refuses
`UndeclaredCoincidence`; declared, the door has verified it.

**The review's MAJOR-1: the first fix read such a pair's bounds instead.**
- On the corner witness below, that read `b` On and `a` Out: half a
  crossing.
- `within` then admitted the tool's top against the block's `y = 0`
  face as a 1e-10 graze at the arm.
- The vertex's germs came out odd, a `ClassificationInvariant` where
  main refused typed.
- At the 1 mm scale the pose IS a coincidence: the tool's top and the
  block's `x` edge agree to the band, and part only beyond it.
- An answer needs the pair lane to read its germ tests and edge-sector
  keys at the long reaches too. That is filed with the witness, below.

**Consequence for the germ gate.** No pair reaching `insert::germ_dir`
was read parallel at the arm, so its gate, `|n_a × n_b|·arm`, is the
margin `pair_search` already read as definite. It stays as it was.

### A pierce's coplanar lump needs On bounds

`vtxfac`'s Delta 2 lumped a sector whose normal read parallel to the
pierced plane at the arm, overwriting both bounds' readings. It now
lumps only when both bounds also read On.
- A face that agrees at 1 mm but whose 10 m edge dips 500 bands reads
  that edge In, and the sector is not lumped.
- In band, the bounds are read first (the review's MINOR-3): a bound
  read definitely off decides, and only an all-On sector escalates.

The pierce's germ-line gate (`pierce_germ_dir`, `bool_germ_line`) is
levered at the sector's farther reach, `BoolSector::span`.
- A transition sector has a bound read at least `K·zero` off at its
  reach `L`.
- That reading is at most `L·|n_s × n_p|` plus the two vertices'
  residuals, up to `2·zero`.
- So the gate agrees with the classification whenever K > 3.

The shorter arm called such a sector coplanar and refused as an
invariant.

The pair lane's in-band parallelism cannot be settled the same way: the
arm-setting bound always reads On or in band there, so it still
escalates.

### Bisector Zeros refuse where they could decide

A bisector's code only relays its physical sector's side between two
twins. With mixed neighbours, its Zero changes no topology.

With both neighbours definitely on one side, it cannot read Zero when
K > 2:
- the shorter bound's own reading gives `s·a ≥ K·zero`;
- a reflex bisector then reads at least `K·zero/2`;
- a straight-band one reads at least `a·cos δ`.

The argument is written at `vtxfac`'s `resolve_on_entries` and at
`recl::resolve_bisector_graze`. Both refuse through one constructor,
`sectors::bisector_zero_refusal`, which is `Escalated` with predicate
`bool_sector_bisector_side`. Its payload is the enclosure `[−zero, zero]`
the reading is known to lie in, not a claim of a poisoned margin.

**The parity argument's precondition.** The metric reading removes the
dependence for every line bound. For bisectors it is enforced at these
two sites, which refuse loudly where K ≤ 2 could make them wrong. No
global K check was added.

### The join's refusal no longer claims a kernel bug

The text of `SplitJoinError::UnpairedLooseEnds` is shared by both
lanes. It now names what can still cause the refusal: an edge leaving a
vertex tangent to the surface it is read against, whose side is read to
finite order (first order in the boolean, second in the split).
Anything else it calls a kernel defect. The review's MINOR-6 found two
missing `\` continuations, which printed 18-space runs; both are fixed.
The variant is unchanged.

## Rows

All pass at ε = default, 1e-6, 3e-10, 1e-10 and 1e-12. The integration
rows are in `crates/topo/tests/contact9_side_codes.rs`.

`answers` checks ground truth:
- `∩` has the analytic sliver's corners, within ε;
- `∩` has the analytic volume, to 1%, and tier 3, where the volume door
  resolves the sliver;
- a point inside both operands is in `∩` and `∪` and out of `−`;
- `−` and `∪` pass tier 3.

The rows:
- `a_pierce_reads_a_dipping_edge_at_its_far_vertex` (`vtxfac`) and
  `a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`
  (`pair_search`). The step-2 needle poses answer, each beside a 1 m
  control.
- `a_sector_parallel_at_a_short_arm_is_coplanar_only_if_its_bounds_read_on`.
  A wedge on the block's top, with a 1 mm edge on the face and a 10 m
  edge dipping 500 bands:
  - it answers; it used to refuse as a `ClassificationInvariant`;
  - declared `Rest`, it is contradicted.
- `a_pierce_germ_line_is_read_at_the_sectors_reach`: the review's P1,
  with the long edge rising. `tool − block` is the sliver above the top
  (corners, `det·dip/(2|c_z|)`, membership).
- `a_near_coincident_pair_at_a_corner_refuses_typed`: the MAJOR-1
  witness.
  - `∩`, `−` and `∪` refuse `UndeclaredCoincidence`, or `Escalated` at
    ε = 1e-6, where an in-band edge contact comes first.
  - Its ×1000 control answers:
    - `∩` and `−` partition the tool, and `∪` is block + tool − `∩`;
    - membership holds on both sides;
    - tier 3 holds.
  - This corrects the earlier record's "no end-to-end fixture could pin
    this".
- `a_dip_inside_the_band_still_reads_on`. A dip of `ε/2` is a real On:
  the pose is a touch.
- `the_splitting_twin_reads_the_dipping_edge_at_its_far_vertex`: the
  sliver's corners, and its volume where resolved.
- Unit rows:
  - `sectors::tests::a_pair_parallel_at_a_short_arm_goes_to_the_ladder_as_a_coincidence`:
    the pair search records a coincidence with every code On.
  - `vtxfac::tests::a_bisector_on_between_one_sided_bounds_refuses` and
    `recl::tests::a_grazing_bisector_between_one_sided_keys_refuses`:
    both bisector refusals, direct. They are unreachable at K = 10 end
    to end, and no CI job runs another K. Each row carries a control
    that resolves.

Two rows skip tier 3 on the results named, and say why at the site:
- the wedge row, on all three results;
- the pierce germ-line row, on `−` and `∩`.

Their seam is a 1 mm edge between faces `dip/10` radians apart, which
the seam description reads at the seam's own length and leaves a
scaffold (filed).

**The volume floor (the review's MINOR-5).** `RESOLVED_VOLUME = 1e-12`
m³. The volume door's absolute error on these ~10 m-span results was
measured at up to ~1e-15 m³, over ε from 1e-6 to 1e-12:
- it returns half of a 4e-19 m³ tetrahedron with exact corners;
- it returns −2.96e-16 for a 5.8e-19 one, which tier 3 reports as
  `NegativeVolume`;
- it is off by 1–7% at 3e-10 and 1e-10.

A thousand times that error keeps the 1% check honest. Below the floor,
the corner set is the oracle and tier 3 is skipped on `∩`. The door's
error is filed.

**Rows that moved.** None outside this unit. `topo` and `sweep` at three
eps, `editor-core`, `test-utils` and the Python suite keep their
answers. `sectors.rs`'s existing unit rows now build real edges as
`Reach::Extent(1.0)`, the same levered reading at the same arm they
used before, and every value is unchanged.

## Mutants (the review's MINOR-2)

Each change was reverted alone, and the rows were run (`topo`: every
`contact9` row and the `sectors`, `recl` and `vtxfac` unit rows):

| Mutant | Red rows |
|---|---|
| M1: a line bound levered at the arm | the two needle rows, the wedge row, the pierce germ-line row |
| M2: a parallel pair keeps its read codes | the corner row, the pair-search unit row |
| M3: `germ_dir` levered at the reach | none; the whole `topo` suite runs green. No pose reaches it: the gate is the margin `pair_search` read as definite (above) |
| M4: `pierce_germ_dir` at `s.arm` | the pierce germ-line row |
| M5: lump without the On check | the wedge row, the pierce germ-line row |
| M6: `vtxfac` bisector refusal removed | its unit row |
| M7: graze refusal removed | its unit row |
| M8: in-band lump escalates before reading | none end to end: the witness (P1 at 5·10⁴·ε) now refuses one step later, at the pierce germ direction's `within` (filed), so both sides refuse |

## Sweep for the class (discipline §5)

Pattern: `Margin::levered(` in the sector code (`boolean/sectors.rs`,
`vtxfac.rs`, `recl.rs`, `insert.rs`, `plane_eq.rs`, `carrier_eq.rs`,
`splitting/neighborhood.rs`, `splitting/rules.rs`, `sector_shape.rs`).
Hits and disposition:

| Site | Reading | Disposition |
|---|---|---|
| `sectors.rs` `side_code` | a bound's side | fixed: metric for line bounds |
| `sectors.rs` `bool_faces_parallel` | near-coincidence | kept: its Zero routes to the ladder, since the arm-setting bound is On (above) |
| `vtxfac.rs` `bool_sector_coplanar` | coplanar lump | fixed: On bounds required, in band too |
| `vtxfac.rs` `bool_germ_line` | germ line exists | fixed: levered at the reach |
| `insert.rs` `bool_germ_line` | germ line exists | kept: the margin `pair_search` read as definite |
| `sectors.rs` `within` (pair search, germ directions) | direction in sector | filed twice: the corner witness's graze and the pierce germ direction |
| `sectors.rs` `parallel_same`, `recl.rs` `parallel_same_dir` | "same ray" | filed |
| `recl.rs` `bool_dir_same` | the sign of a cosine near ±1 | not this class: only the sign is read, and a Zero refuses |
| `insert.rs` `bool_strut_order` | an order | not a Zero-as-On |
| `plane_eq.rs` `bool_plane_parallel`, `bool_plane_orient` | the carrier ladder's plane rungs | the lump's arm, and the door's 1 m: filed (below) and on the `tang` row |
| `carrier_eq.rs` axis parallelism (cylinder, torus) | the ladder's curved rungs | the door's 1 m lever: the `tang` row |
| `splitting/neighborhood.rs` `split_bisector_side` | bisector side | K-free: rule (b)'s AOA→BELOW and BOB→ABOVE are right for a reflex bisector, and the straight band cannot read Zero |
| `splitting/rules.rs` `split_sector_coplanar` | coplanar | levered at the face extent, which bounds every face point's offset from above; sound |
| `sector_shape.rs` wideness rungs | subdivide or not | Zero and in-band both subdivide, which is sound at any angle |

What the pattern cannot match: a levered reading spelled without the
door. A second pass grepped `Margin::of(` in the same files for a product
with an arm, and found two:
- the curvature charge in `side_code`, a first-order-versus-sagitta
  comparison;
- the splitting lane's conic departure (`t̂·extent·n̂`), levered at its
  own extent, which descends to second order at its Zero.

The seam description in `boolean/ops.rs`, found by the wedge row, is
outside the sector code and is filed.

## Filed

All in `work/contact/`:
- `boolean-conic-side-code-zero-is-first-order`;
- `seam-description-reads-a-dihedral-at-the-seams-own-length`;
- `a-wedge-edge-on-a-block-top-from-its-corner-refuses-at-the-join`, a
  pre-existing `UnpairedLooseEnds`, at base and with 1 m edges;
- `boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm`;
- `a-vertex-pair-near-coincidence-refuses-where-its-long-edges-decide`
  (MAJOR-1's witness);
- `volume-door-reads-a-tiny-valid-boolean-result-wrong` (MINOR-5);
- `coplanar-lump-carrier-verdict-is-levered-at-the-sector-arm`
  (MINOR-4, the review's P4 witness; pre-existing);
- `pierce-germ-direction-within-is-levered-at-the-sector-arm` (MINOR-3's
  remainder).

The declaration door's 1 m lever is the `tang` row's
(`work/tang/torus-carrier-axis-margin-is-levered-by-one-not-the-ring`).
Evidence for its plane arm was added there.

`docs/predicate-dimension-audit.md` rows for the changed predicates are
updated.

## Local results

Filled at hand-back.
