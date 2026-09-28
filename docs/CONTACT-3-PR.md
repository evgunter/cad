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
  pieces' planes. What makes that exact is parity invariance. When both
  sides agree at `p`, the count does not depend on which of the two
  pieces covers `p`'s ruling:
  - where the loop passes through the junction, exactly one covers, and
    it counts the same either way;
  - where the loop turns there, both cover or neither does, adding 0 or
    2, which is even.

  So any consistent reading of the band gives the true parity. The arm
  reads it as just past the junction: a piece counts iff its `lo` end is
  active.
- **Anything else escalates:** incident pieces that disagree, a piece
  with both ends active, or a piece window in band with neither end
  active (only the cosine construction's narrow-window collapse produces
  that).

## Where it refuses, and how that is confined

`Unsupported` has exactly these paths, and the variant's doc names them:

- a ring;
- an edge that is not a meridian, rim or planar section of the wall
  (including a failed seat check, and a plane parallel to the axis);
- a loop the chart walk declines (not a cycle, or a walk error other
  than an escalation);
- images that do not chain half-edge to half-edge (null scaffolding);
- a meridians-only loop;
- a piece whose azimuth extent decides Zero.

A hit definitely outside the face's azimuth window is a miss. So is a
hit definitely outside a ball holding the outer loop, but only for a
face with no ring (F6a). A seamless band's outer loop is one rim and does
not hold the face, so a ringed face trusts only its window.

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

**The V-cut through two splits.** An earlier version of this record said
the V could not be built through any door. That was wrong: two `split`s
build it. The VALLEY keeps what is above both planes and the RIDGE what
is below both, with the prism's seams turned to π/2 + 0.05, near where
the crease meets the wall. Both are rows at tilts 0.4 and 1.1; their
counts are under "Floors and caps".

The notch-tool routes still fail:

- `subtract` of a notch tool is refused (`CurvedSectorSideUnsupported`)
  with the seams at 0/π, and fails with an internal `Euler(StaleKey)`
  with the seams under the ridge (filed,
  `work/issues/subtract-v-notch-from-seam-aligned-cylinder-escalates-stale-halfedge`);
- the union of two differently cut halves is refused
  (`CurvedPierceUnsupported`).

## Floors and caps

`every_tilted_cut_wall_reads_its_truth` fails a shape if any of these
hold:

- any answer is wrong;
- any refusal is outside two kinds: `VolumeUncertified`, or an
  escalation whose predicate is `bool_wall_trim` or
  `point_in_arc_loop_conic_window` (a planar ellipse face's arc window)
  with its margin strictly inside `(ε, K·ε)`;
- the admitted escalations exceed its cap;
- its answers fall below its floor.

The floor is the fewest answers over the ε rows unset, 1e-6 and 1e-12,
less 2. A change that turned chart-wall hits into refusals falls
through it.

| shape | answered (unset / 1e-6 / 1e-12) | floor | escalation cap |
|---|---|---|---|
| cut 0.3, below | 334 / 334 / 334 | 332 | 0 |
| cut 0.3, above | 400 / 400 / 400 | 398 | 0 |
| corner clip | 680 / 679 / 680 | 677 | 1 |
| corner clip, the chip | 24 / 23 / 24 | 21 | 1 |
| tilt 0.9, below | 305 / 305 / 305 | 303 | 0 |
| tilt 0.9, above | 456 / 456 / 456 | 454 | 0 |
| slab at tilt 1.0 | 386 / 386 / 386 | 384 | 0 |
| wedge | 522 / 522 / 522 | 520 | 0 |
| lens | 61 / 61 / 61 | 59 | 0 |
| valley at tilt 0.4 | 315 / 315 / 315 | 313 | 0 |
| ridge at tilt 0.4 | 302 / 302 / 302 | 300 | 0 |
| valley at tilt 1.1 | 233 / 233 / 233 | 231 | 1 |
| ridge at tilt 1.1 | 214 / 214 / 214 | 212 | 0 |
| cut 0.3 minus a box (subtract) | 198 / 198 / 198 | 196 | 0 |

The three admitted escalations are all at ε = 1e-6:

- the clip and the chip each have one: probe (0.8, 0.4, 2.25), pose "0.7
  about z", `bool_wall_trim`, margin 3.83e-6;
- the valley at tilt 1.1 has one: probe (0.4, −0.4, 0.25), pose "1.1
  about (1,2,3)", `point_in_arc_loop_conic_window` on the cut face's
  ellipse arc, margin 6.4e-6.

All valley and ridge rows answer with 0 wrong.

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
- The textual call-site count row is gone. The cosine-window inventory
  at `point_on_wall_in_face` names every site. Each is marked in its
  text: a call to `chart_azimuth_margin`, `narrower_than_period` or
  `chart_dir`, or the period guard's `T::tau() −`.
  `the_window_construction_sites_are_the_ones_listed` collects the
  enclosing functions of those marks in `solid_contain.rs` and
  `contain.rs`, and compares them with the list. It cannot see a site
  that restates the algebra without any of the four marks, and the
  inventory says so.
- The completed list now includes `wall_outline`'s own period guard
  (kept, because the parity argument's premise is checked where the
  class is resolved), `sphere_chart_trim`'s meridian span, and the
  class-question guards in `cone_chart_trim` and `torus_face_windows`.
  The three remaining written-out chart directions (`point_on_arc`,
  `point_on_sphere_in_face`, `sphere_chart_trim`) go through
  `chart_dir`.

## Mutation table

Each mutant was a source patch run against both suites (topo's
`wall_section_rows`, and sweep's `pis_arc_capped_poses`, whose rows
build real bodies through the split and subtract doors), then reverted.

| mutant | rows that go red |
|---|---|
| M1: sides swapped (count covering pieces ABOVE the hit) | none. **Equivalent**: at a generic ruling the covering count is even, so above-count and below-count have the same parity. Parity carries no side datum, so there is nothing to swap |
| M1b: junction read as just BEFORE instead of just after | none. **Equivalent**: when both incident pieces are on one side of `p`, their parity contribution is the same whichever covers `p`'s ruling, so either reading is exact |
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

No `ci-local.sh` and no doc-gate; `CARGO_INCREMENTAL=0`, own target. The
branch has origin/main merged in, including #3304's typed `SCHEDULE`,
and resolves `contain.rs` onto main's `CurvedPlacement` door.

- `cargo test -p topo --no-fail-fast`: 1540 passed, 0 failed, at
  `CAD_TOLERANCE_EPS` unset, 1e-6 and 1e-12.
- sweep, as `cargo nextest run -p sweep --partition count:k/3`, k = 1..3
  (the whole suite does not fit the build slot's express budget on a
  shared box): 1698 passed, 0 failed, at each of the three rows.
- `cargo clippy -p topo -p sweep --all-targets -- -D warnings`: clean.
- rustdoc on topo with the doc gate's flags (`-D warnings -A
  rustdoc::private_intra_doc_links`, `--document-private-items
  --all-features`): clean. A bare `-D warnings` run is red on origin/main
  too, over links `work/ciw/rustdoc-d-warnings-breakages-outside-the-doc-gate`
  catalogues.
- Every `scripts/gates/*.sh`, `--selftest` and real pass, plus
  `probe-suite-census.sh --citations`: all 0.
- `work.py lint`: ok. `fmt-all.sh --check`: ok.
- The editor-core concision suites were not built locally.
