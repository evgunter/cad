# CONTACT-3: the cylinder wall arm reads a wall's outline by parity along the ruling, not by its vertex rectangle

Carries `work/contact/cylinder-wall-trim-overcovers-a-tilted-section`.

## What changes

`point_in_solid`'s ray lane decided whether a ray hit a cylinder wall face
by reading `cylinder_chart_trim`'s azimuth × height rectangle, taken from
the face's boundary VERTICES. A wall bounded by a tilted planar section is
not that rectangle: it reaches past the section where it stands high and
falls short where it dips. The certified walk therefore answered `In`
for points across a cut.

The ray lane now resolves a wall's outline (`wall_outline`,
`crates/topo/src/boolean/solid_contain.rs`) on the first hit that lands in
the face's azimuth window, and reads the hit against it
(`point_on_wall_in_face`). There are three classes:

- **`Rectangle { h }`**: rims and meridians, the rims on exactly two
  levels. This is the rectangle, read exactly as before: the same
  margins and the same predicate names.
- **`Chart { pieces, junctions }`**: no ring, and every outer-loop edge
  is a meridian, a rim, or an on-axis planar section. The window must be
  narrower than a period. The hit is read by parity along its ruling.
- **`Unsupported { reach }`**: everything else. It is a confined, typed
  refusal, `PointInSolidError::WallOutlineUnsupported`.

## The class predicate, and where it lives

`wall_outline` is the one predicate. The face-level door
(`contain::curved_face_containment`) asks it and serves `Rectangle` only,
as before. `iso_bounded_wall`, which was private to `contain.rs`, is
deleted.

The exact class has no plane count and no monotone-chain premise.
A cut running out through a cap, a steep cut through both caps, a slab,
a convex roof, a lens, and a stepped iso outline are all in it. `Chart`
is a strictly larger class than the first pass's two-plane `Sections`.
On the three-or-more-plane walls that the first pass refused wholesale,
it answers exactly.

## Why parity along the ruling is exact

The full argument is at `wall_outline`.

1. Every rim and section plane meets each ruling exactly once, because
   none is parallel to the axis. In the chart `(θ, h)`, each piece is
   therefore a graph over its own azimuth sub-window, and a meridian is
   a vertical segment.
2. The window is narrower than a period (decided), so the outer loop is
   a simple closed curve in a plane strip. The face is the bounded
   region it encloses.
3. At an azimuth that is no junction's, the upward ray from the hit
   meets no meridian. It meets a piece iff the piece covers `θ` and lies
   above the hit, and each such crossing is transversal. So membership
   is the parity of the covering pieces the hit lies below.

The piece sub-windows come from the same branch-pinned chart walk as the
face window (`chord_join::face_azimuth_images`, factored out of
`run_azimuth_window`, which now folds over it). So a piece's window and
the face's are on one branch.

**The side is the physical perpendicular distance to the piece's plane,
in metres**: `WallPlane::above`, with the normal oriented to `n̂·â > 0`.
It is not the height along the ruling, which a steep section inflates by
`1/|n̂·â|` (F4).

**At a junction, the hit is decided there, never by picking a piece.**
A junction is where consecutive pieces meet, at a vertex or through a
meridian run. A hit whose ruling is within the band of a junction's is
handled as follows:

- **Strictly inside the meridian run's height span:** it is on the
  meridian, so it grazes.
- **Otherwise:** it must be definitely on one side of both incident
  pieces' planes. A definite margin exceeds the band, and crossing the
  band moves the point less than that, so the region is constant across
  it. The hit is therefore read as just past the junction: a piece counts
  iff its `lo` end is active.
- **Anything else escalates:** incident pieces that disagree, a piece
  with both ends active, or a piece window in band with neither end
  active (only the cosine construction's narrow-window collapse produces
  that).

## Where it refuses, and how that is confined

`Unsupported` is now only a ring, or an edge that is none of the three
kinds: a non-planar curve, an off-axis or mis-seated ellipse, or null
scaffolding. A hit definitely outside the face's azimuth window is a
miss. So is a hit definitely outside a ball holding the outer loop, but
only for a face with no ring (F6a). A seamless band's outer loop is one
rim and does not hold the face, so a ringed face trusts only its window.

Class resolution is lazy (F5). `face_geo` no longer resolves the class;
`wall_hit` does, after the hit's window test. A wall no ray reaches is
never classified, and an in-band class margin on it cannot escalate a
query it plays no part in.

## Shapes, head against base

Measured on the unit cylinder of height 2.5, at six rigid poses, on a 5³
probe grid clear of every boundary (`every_tilted_cut_wall_reads_its_truth`).
Each cell is answered / wrong. The only refusal is `VolumeUncertified`,
the at-infinity side that the props lane owns.

| shape | base (c903cdcfe) | head |
|---|---|---|
| cut 0.3, below | 374 / 40 | 334 / 0 |
| cut 0.3, above | 412 / 12 | 400 / 0 |
| corner clip (out through the cap) | 680 / 0 | 680 / 0 |
| corner clip, the chip | 71 / 46 | 24 / 0 |
| tilt 0.9 through the centre, below | 527 / 212 | 305 / 0 |
| tilt 0.9, above | 538 / 76 | 456 / 0 |
| slab at tilt 1.0 | 594 / 230 | 386 / 0 |
| wedge (convex roof) | 541 / 18 | 522 / 0 |
| lens (two sections crossing on the seams) | 50 / 2 | 61 / 0 |
| cut 0.3 minus a box (subtract door) | 238 / 40 | 198 / 0 |

`WallOutlineUnsupported` refusals on every shape: 0.

Every probe that head leaves unanswered is a `VolumeUncertified`: its
rays now honestly cross nothing where base counted a false crossing.
The iso walls are bit-identical to base. A digest of every answer over
2 × 4374 probes (plain cylinder, quarter sector) matched at the first
pass, and the rectangle arm is unchanged since.

**The V-cut cannot be built through any door on this tree:**

- `subtract` of a notch tool is refused (`CurvedSectorSideUnsupported`)
  with the seams at 0/π.
- With the seams under the ridge it fails with an internal
  `Euler(StaleKey)`, filed as `work/issues/subtract-v-notch-from-seam-aligned-cylinder-escalates-stale-halfedge`.
- The union of two differently cut halves is refused
  (`CurvedPierceUnsupported`).

Its outline, two section pieces meeting at a concave vertex, is the
wedge's with the other concavity, and parity has no concavity case.

## Cone and sphere (S3)

Unchanged from the first pass. A tilted section of either is unbuildable
through split, the boolean and STEP import. The sphere refuses such a
face whole; the cone carries the same latent defect, filed. The sphere's
stepped-outline sibling (`latitude_extremes` folds any number of rim
levels) is filed too, citing both reviewers.

## Rows

- `crates/sweep/tests/pis_arc_capped_poses.rs`:
  - `the_cut_cylinder_reads_its_truth`: now also asserts that the only
    refusal is `VolumeUncertified`, and that answers outnumber it.
  - `every_tilted_cut_wall_reads_its_truth`: new. The shapes above,
    through the split and subtract doors, going through `wall_outline`
    from real bodies.
  - `a_wall_point_across_the_section_is_not_on_the_upper_half` and
    `iso_bounded_walls_answer_through_their_rectangle`, as before.
- `crates/topo/src/boolean/wall_section_rows.rs`:
  - per-side rows (between, past the rim, the over-cover, the
    under-cover, outside the window, on the section, and in band on
    three sides);
  - `half_a_threshold_off_a_steep_section_never_answers` (F4,
    `n̂·â = 0.05`, 0.5ε off);
  - the stepped outline and its junction rows (on the step, above both
    floors, above the roof, at the junction azimuth and in band of it on
    both sides);
  - the two `Unsupported` confinement rows (ball and ringed);
  - `a_wall_no_ray_reaches_never_escalates_the_query` (F5). A real
    `cyl_wall_sheet` resolves to `Rectangle`; with its radius put in
    band it escalates when asked, and a query that never reaches it
    still answers.
- The textual call-site count row is deleted (F6i). The cosine-window
  inventory at `point_on_wall_in_face` is a named list, and the period
  guard now has one home (`narrower_than_period`), so the fourth
  restated site is gone.

## Mutation table

Each mutant was a source patch run against both suites (topo's
`wall_section_rows`, and sweep's `pis_arc_capped_poses`, whose rows
build real bodies through the split and subtract doors), then reverted.

| mutant | rows that go red |
|---|---|
| M1: sides swapped (count covering pieces ABOVE the hit) | none. **Equivalent**: at a generic ruling the covering count is even, so above-count and below-count have the same parity. Parity carries no side datum, so there is nothing to swap |
| M1b: junction read as just BEFORE instead of just after | none. **Equivalent**: the argument shows the region is constant across the junction's band on both sides, so either reading is exact |
| M3: every in-class wall read as `Rectangle` | `every_tilted_cut_wall_reads_its_truth`, `the_cut_cylinder_reads_its_truth`, `a_wall_point_across_the_section_is_not_on_the_upper_half` |
| M4: the `Unsupported` confinement answers every hit a miss | `an_unreadable_outline_refuses_only_within_its_reach`, `a_ringed_outline_trusts_only_its_window` (unit rows only: no door on this tree mints an out-of-class wall) |
| M6: only the first two covering pieces counted | `every_tilted_cut_wall_reads_its_truth`, `a_stepped_outline_reads_its_notch`, `a_hit_on_a_junction_azimuth_is_decided_at_the_junction` |
| M7: a junction whose pieces disagree picks a side | `a_hit_between_a_junctions_pieces_escalates`. The branch is unreachable from a resolved outline (near a vertex without a meridian both planes pass within the band of the hit; a meridian run's span catches the rest), so it is a guard, and this row holds it |
| M9: F4 reverted (side as height along the ruling) | `half_a_threshold_off_a_steep_section_never_answers`, `every_tilted_cut_wall_reads_its_truth` |
| M10: F5 reverted (class resolved in `face_geo`) | `a_wall_no_ray_reaches_never_escalates_the_query` |

## Sweep: every place a trim rectangle answers membership

The first pass's hit list stands:

- the cylinder pre-pass and `cast_ray` arms: fixed;
- the face door: on the shared predicate;
- the cone: filed;
- the sphere: class-checked, stepped sibling now filed;
- the torus: class-checked;
- `join.rs`'s arc-side window: not membership;
- shell's vertex footprint: filed.

The pieces' azimuth sub-windows are new readers of the chart walk. Their
windows are the walk's own exact entry and exit, not the hull's padded
range.

## Territory seam

- `crates/topo/src/chord_join.rs` (chord): `face_azimuth_images` is the
  run walk's per-edge output. `run_azimuth_window` and
  `face_azimuth_window` fold over it and are behaviour-preserving.
- `crates/topo/src/splitting/containment.rs` (reach): `loop_reach`,
  unchanged since the first pass.
- `crates/sweep/tests/pis_arc_capped_poses.rs` (tcost/tint): the spec's
  pinned file.
- `scripts/gates/loop-boundary-discards.sh`: the register re-keyed.
  `iso_bounded_wall`'s entry is gone with its site, and
  `face_azimuth_window`'s entry moves to `face_azimuth_images` with the
  let-else it names. `wall_outline` has no `LoopBoundary` discard.
- `docs/predicate-dimension-audit.md`: seven rows for the new predicate
  names.
- The editor-core concision lists, as before.

## Filed

- `work/contact/cone-chart-trim-reads-a-tilted-section-as-its-vertex-window`
- `work/contact/sphere-chart-trim-folds-any-number-of-rim-levels`
- `work/shell/shell-clearance-footprint-reads-vertices-not-arcs`
- `work/issues/subtract-v-notch-from-seam-aligned-cylinder-escalates-stale-halfedge`

## Local verification

No `ci-local.sh` and no doc-gate; `CARGO_INCREMENTAL=0`, own target.

- `cargo test -p topo --no-fail-fast`: 1512 passed, 0 failed, at
  `CAD_TOLERANCE_EPS` unset, 1e-6 and 1e-12.
- `cargo test -p sweep --no-fail-fast`: 1673 passed, 0 failed, 7
  ignored, at unset and 1e-12. At 1e-6 the battery found one failure
  in `every_tilted_cut_wall_reads_its_truth`: one probe of the corner
  clip and its chip escalated (`bool_wall_trim`, margin 3.8e-6 inside
  the 1e-6 run's band). That is a ray landing within the band of a
  piece's edge, which is a typed refusal and not an answer. The row
  now admits `Escalated` beside `VolumeUncertified`, and still forbids
  `WallOutlineUnsupported`, every other refusal and every wrong answer.
  The pinned suite was re-run green at all three rows after that.
- `cargo clippy -p topo -p sweep --all-targets -- -D warnings`: clean.
- rustdoc on topo:
  - With the doc gate's own flags (`-D warnings -A
    rustdoc::private_intra_doc_links`, `--document-private-items
    --all-features`): clean.
  - Bare `RUSTDOCFLAGS='-D warnings' cargo doc -p topo --no-deps`: red,
    and equally red on origin/main. It reports pre-existing private and
    unresolved links (`attach.rs`, `body.rs`, `boolean/mod.rs`'s
    `SweepStrategy::Idealized`, …), which
    `work/ciw/rustdoc-d-warnings-breakages-outside-the-doc-gate` already
    catalogues.
- Every `scripts/gates/*.sh`, `--selftest` and real pass, plus
  `probe-suite-census.sh --citations`: all 0.
- `work.py lint`: ok. `fmt-all.sh --check`: ok.
- The editor-core concision suites were not built locally (disk).
