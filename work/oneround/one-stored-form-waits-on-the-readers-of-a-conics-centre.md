---
id: one-stored-form-waits-on-the-readers-of-a-conics-centre
kind: unit
title: Every reader that rebuilds or restates a conic from its centre goes through the conic: the sites, and what each showed when the centre was a derived one
status: closed
opened: 2026-11-21
priority: P1
cost: H
refs: [the-section-table-admits-a-curve-on-its-resolution, a-conics-points-are-restated-in-centre-form-outside-its-evaluator, a-section-conic-is-anchored-at-a-general-point-of-its-arc]
branch: oneround/conic-readers
pr: 7510
closed: 2026-11-23
---


## What is owed

A conic is one of two placements behind the one `Conic` reading, held
about its centre or about a point of the arc it serves, and every
reading of where a conic is goes through the conic (PR 7258, approved).
There is no single stored form to wait for. What is owed is the last
clause: the readers below take a conic's centre as a datum and rebuild
or restate the conic from it themselves, so each of them reads an
arc-held conic through a centre that is derived, and loses or misreads
the placement.

This row is the spec of the readers unit: each family below, and each
door on the list after it, reads the conic (its point, its cover, its
image, its placement) and not `frame().origin()` and the sizes.

## What a derived centre costs these readers, measured

Measured once with EVERY conic's centre derived
(`oneround/anchored-conic-one-form` at `e04a87227`, a build in which a
door handed a centre summed the anchor `c + u·a` once and
`Conic::center()` / `Conic::frame()` derived the centre back on each
read). That build is not the design; it is the measurement of what
these readers do with a derived centre, which an arc-held conic gives
them today. Every reader that takes a conic's centre as a datum then
reads `(c + u·a) − u·a`:

- at `Interval`, the semi-axis's width and `a` times the direction's
  enter the centre twice over, and a point half a turn from the anchor
  carries them three times where the centre form carries them once;
- at `f64`, the derived centre is `fl(fl(c + u·a) − u·a)`, a rounding of
  the semi-axis off `c`, so anything stated from the centre is a
  different conic from the one the evaluator draws.

The rows it moved, with their numbers, are on
`analysis/oneround/conic-vertex-probes` under
`analysis/oneround/conic-vertex/step-b-one-form/`.

## The families of reader, and what each showed

The unit's earlier sweep counted about 90 production sites that read a
conic's centre as a datum. By family:

| family | where | what it showed with every centre derived |
|---|---|---|
| an arc's cover | `Curve3::cover_over`, `crates/geom/src/cover/` | at `a = 1e7` the cover's ball misses the evaluator's point by 4.2e-10 m (`arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size`). **The ball misses the arc's TRUE point by that much on `main` already**: there the evaluator makes the cover's own 4.2e-10 error (`1e7 − 1e7·cos t`) and the row passes; under one form the evaluator is right to 6e-17 and the cover is the one left wrong (the analysis branch's `two-facts.md`) |
| an edge's extent bound | `crates/geom-brep/src/extent.rs` | **short by an ulp**: a circle's sup 4 under the evaluated diameter 4.000000000000001 (`edge_extent_bounds`) |
| chart-frame readers of a circle leg | `crates/topo/src/ruled_region.rs`, `torus_region.rs` | the count of queries a frame answers moves (35 → 36, 33 → 51); not shown wrong |
| an edge's chart image | certification's check 4, `crates/geom-brep/src/certify.rs` (`pcurve_map_residual`) | a residual of an ulp of the radius where zero; a carried certificate no longer equals a fresh one |
| `Curve3::param_near` | `crates/step-import/src/build.rs` (`settled_foot`), and every foot read | `eval ∘ param_near` no longer idempotent; the STEP round trip's fixed point goes by an ulp |
| two statements of one circle compared by bit | `crates/mesh/src/curved.rs` (the pinch read) | another face refuses first |
| the certified lane's readers at `Interval` | blend's contact edge (`shared_row_touch`), rung 3's `ssi_hull_sup`, profile validation, the cone lane's envelope, the graze side | four builds become refusals, and the widths in the analysis table |
| the symbolic scalar | `sweep::swept`'s placed rim, limb 2's memo | a registration is lost: `c + u·r − u·r` is not recognised as `c` |

The fourth to sixth rows are
`a-conics-points-are-restated-in-centre-form-outside-its-evaluator.md`'s
three; the cover, the extent bound and the chart-frame readers are the
same class and are not on that row.

## Where it bites today

Only an ellipse can be held on its arc, and only `plane_cone_section`
mints one so. For it: the cover and the chart image are still stated
from the derived centre
(`the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding-and-the-depth-gap-decides-on-it`,
`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`).
The families above are where the same reading would bite every conic
that comes to be held on its arc
(`a-section-conic-is-anchored-at-a-general-point-of-its-arc`).

## The doors that rebuild a conic through a centre constructor

Each reads a conic's centre frame and builds a conic from it with
`Circle::new` / `Curve3::circle` (or an ellipse's), so its result is
held at a centre whatever its source was. Listed by both reviews of
PR 7203; none meets an arc-held conic today, because every one of the
same-locus sites takes a circle and a circle is held at its centre
only. They are the sites that lose the placement the day a circle can
be held on its arc.

- Same locus, the placement lost: `crates/topo/src/replace_face.rs`
  (the three doors the unit named, circle arms only),
  `crates/topo/src/offset_axial.rs` (two), `crates/topo/src/ring_path.rs`
  (`torus_circle_arc`), the `Rim` round trip
  (`crates/topo/src/rim_wedge.rs`, `crates/topo/src/boolean/mod.rs`, two
  sites), `MeetSegment::Arc` in `crates/topo/src/validate.rs`,
  `crates/geom-brep/src/curve_pair/on_surface.rs`, and `crates/sweep/src/swept.rs`
  (`circle_at`, to evaluate).
- Moved, or another curve: `crates/geom/src/pole_form.rs`
  (`half_turn_curve`, which also re-squares the frame),
  `crates/sweep/src/blend/junction.rs`,
  `crates/sweep/src/blend/open/end_face.rs`.
- From an ellipse's derived centre frame, a different conic:
  `crates/topo/src/ring_path.rs` (`ConicArc::distance_floor`),
  `crates/sweep/src/blend/surgery.rs` (`piece_along`).

- Brought by `main` since, both on the import's path only:
  `crates/step-import/src/build.rs` (`column_circle`, which rebuilds a
  circle through `Curve3::circle` with its axis reversed, re-squaring
  the frame), and `crates/step-import/src/on_built.rs` (`Conic::of`,
  which restates a circle and an ellipse as centre + `a·cos` + `b·sin`
  from `frame()`).

`crates/step-export/src/writer.rs` writes the derived centre, which is
Ev's decision on PR 4812 and not one of these.

## Not established

- The exact count per family. The table names where each was seen, not
  every site.
- Six moved rows were not checked against an oracle (the analysis
  README's "moved, not checked"). None is shown wrong on `main` today.
  The one suspicion they leave about `main` is already on the
  restatements row: a predicate that holds only while two statements of
  one circle agree to the bit (`mesh`'s pinch read, and perhaps
  `SelfTouchingTrimLoop`, which
  `a_slit_tip_on_a_curved_face_reads_its_full_turn::the_slit_block_meshes`
  now trips).

## The unit (2026-11-23): `oneround/conic-readers`

Letter H, tier dual (a concurrent pair). The unit changes what every
reader of a conic may ask of it, across five crates, and a reader
converted wrongly reads a conic an ulp of its semi-axis off with
nothing red at compile: an interface decision that is hard to change
later, and logic where believing a site takes more than reading it.

**In scope.** The sentence PR 7258 put in the README: "Every reading
of where a conic is goes through the conic."

1. What the `Conic` reading offers, so that no reader outside
   `crates/geom/src/curves/conic.rs` needs `frame().origin()` and the
   sizes to say where the conic is: its point, its cover over a span,
   its extent along a direction, its image in a chart or a plane, its
   foot, and a map that keeps its placement. The unit finds out which
   of these the families below need; it adds no reading nothing needs.
2. The eight families of the table above, each site: the arc's cover,
   the edge's extent bound, the chart-frame readers of a circle leg,
   the edge's chart image, `param_near`, the two statements of one
   circle compared by bit, the certified lane's readers at `Interval`,
   the symbolic scalar's registration.
3. The doors that rebuild a conic through a centre constructor (the
   lists above, the two from `main` included): each keeps the
   placement of what it was handed, or says at the site why its result
   is another conic.
4. A gate or a type that keeps it so: after the unit, a production
   read of a conic's centre outside its module is either gone or on a
   list with its reason, as the lever doors are.

**Where it must show something, not assert it.** For the cone's
section, the one conic held on its arc today, the cover and the chart
image are stated from a derived centre
(`the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding-and-the-depth-gap-decides-on-it`,
`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md`):
both close here, each judged against the curve the stored numbers
state (the exact-rational oracle of
`carrier_resolution::tests::a_far_centred_section_stands_at_the_scale_its_form_is_stored_at`).
A conic held at its centre keeps its bits unless a reader was wrong
about it, and every bit that moves is reported with its direction.

**Out of scope.** The anchor at a general point of the arc and the
readers that pair the parameter with `u_ref` / `v_ref`
(`a-section-conic-is-anchored-at-a-general-point-of-its-arc`); the
constructions that should hold their arc
(`a-construction-whose-centre-runs-off-still-stores-a-centre`, which
waits on this unit and on that row); what a stored frame's truth is
(`work/onehome/a-stored-frame-reads-as-two-curves-a-rounding-of-the-size-apart.md`);
`step-export`'s writer, which writes the derived centre by Ev's
decision on PR 4812.

**A design question, if one appears, is not the unit's to settle**:
what the reading offers is bounded by the README's sentence and by
what the families need. If two shapes of the interface are both
viable and the choice binds the general-anchor row, the lane stops and
asks.

## Corrections from the unit's first PR (7510, 2026-11-23)

Census at the merge base `2b9cd8b24`, taken with the compiler (the
centre doors marked `#[deprecated]`, uses collected from
`cargo check --workspace --lib --bins --all-features`), outside
`conic.rs`: **339 reads on 291 lines in 63 files**, not about 90.

- 137 of them read a direction only (`frame().axis()`, `.u_ref()`,
  `.v_ref()`) and went to `Conic::axis` / `u_ref` / `v_ref`. About
  half of those sit on a line that also reads the centre, to rebuild a
  placement; that line stays listed for its centre read.
- `crates/topo/src/ruled_region.rs`, `crates/mesh/src/curved.rs` and
  the production half of `crates/geom-brep/src/extent.rs` read no conic
  frame on main. The chart-frame family's one site is
  `torus_region.rs` `on_edge_plane`; the pinch read's family has
  `mesh/src/sizing.rs` `sphere_face_widest_cos`.
- `Curve3::param_near` and `step-import`'s `settled_foot` already go
  through `Conic::azimuth`, which reads an arc-held ellipse from its
  anchor (PR 7203): that family had nothing to convert.
- The `Rim` round trip in `crates/topo/src/boolean/mod.rs` (two
  `Curve3::circle(rim.center, …)` sites) reads a struct another
  function filled, so no centre door fires there and the gate does not
  see it; it stays on this row's list of doors by name.
- `Conic::unit_in_plane` reads the derived centre by default and has
  three production callers; `Curve3::circle_at` and `Cover::arc` take a
  conic restated on a caller's frame. All three are doors of the gate.

What the first PR did: the direction readings, the cover and the
extent, and the gate (`scripts/gates/conic-centre-reads.sh`, with
`clippy.toml`'s `disallowed-methods`). What it left is on four rows:
`the-cone-sections-chart-image-and-its-kin-are-stated-from-the-conics-centre`,
`a-conic-made-from-a-conic-is-rebuilt-from-its-centre-and-frame`,
`the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre`,
`a-circles-centre-is-read-as-a-datum-under-the-conic-gate`; and
`the-arc-held-cover-is-looser-than-the-centre-forms-on-a-long-window`.

## Closed (2026-11-23, PR 7510)

The first part landed as PR 7510 (`oneround/conic-readers`): a conic's direction, extent and cover are read through the conic (`axis`, `axis_unit`, the extent and cover doors), the centre-frame doors are gated, and the gate's census is clippy's own diagnostics (`--force-warn clippy::disallowed_methods`, read by `scripts/conic_centre_census.py --check`) pinned as reads per file in `scripts/gates/conic-centre-reads.counts` (203 reads in 58 files at the merge). Reviewed by a concurrent pair with a delta read each; its record is `docs/model-compare-log/rows/PR-7510.md`.

What the row asked for and this PR did not do is not left here. It is CONIC's slate (`work/conic/`, opened 2026-11-23 from Ev's answer on PR 7523): the certified lane's harmonics about the centre, a conic made from a conic, the cone section's chart image and its kin, a construction whose centre runs off, a circle's centre read as a datum under the gate, the three compare doors, and the rest of that plan's order. The second conic unit this track holds, `an-arc-held-ellipses-readers-decide-apart-from-its-derived-centre` (PR 7543), is stacked on this one. Filed by the unit on CIW: the census holds only the crates a scoped run selects, and the nightly runs none.
