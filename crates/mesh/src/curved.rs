//! Curved-face tessellation: UV-rectangle domain from the boundary
//! walk, interior grid sampling, CDT in the lattice metric, pole-fan
//! collapse, per-triangle certificates.
//!
//! This lane's domain contract is the swept UV rectangle: the boundary
//! polygon from [`crate::walk`] IS its own UV bounding box, so the
//! domain is convex and no inside/outside classification is needed —
//! every CDT triangle is kept except pole-degenerate ones, and the
//! interior grid is strictly inside every boundary constraint. Every
//! sweep-authored face satisfies it; it is not a property of
//! iso-bounded input in general (a keyway is iso-bounded and is a U),
//! so it is CHECKED rather than assumed, as FIVE questions with five
//! homes — the first two asked by the route before this lane runs, the
//! last three by this lane:
//!
//! 1. **SHAPE** — *is the face's domain an iso-parameter rectangle?*
//!    Asked BEFORE the walk, by the router, on rim structure, through
//!    the predicate's own door `geom_brep::props::require_iso_rectangle`
//!    (the S58 single home of `props_rim_level`, which also requires
//!    every rim to encode the SAME material side, `props_rim_side`).
//! 2. **BRANCH** — *does each boundary edge's traversed ARC stay on
//!    one branch of the chart?* Also asked BEFORE the walk by the
//!    router, on each edge's stored span, through `geom_brep::props::
//!    require_one_chart_branch` (issue 1571). A certified iso CARRIER
//!    is not an arc on one branch: a sphere meridian's great circle
//!    contains both poles and a cone generator's line passes through
//!    the apex, and everything below reads ONE chart coordinate per
//!    edge. This door is `mesh`'s and the flux lane must not cite it —
//!    the argument is at the predicate.
//!
//!    The two doors' answer routes a ringless face whose stored images
//!    are iso lines (`tessellate::route_of`): a face both pass comes
//!    here, carrying their answer as a [`Rectangle`] this lane takes; a
//!    face either decides is no such rectangle goes to the chart lane
//!    (`crate::trimmed`) where that lane reads its domain, and refuses
//!    with the doors' verdict where it does not; and a face neither can
//!    decide, or whose stored data contradicts itself, refuses
//!    [`TessellateError::UnsupportedCurvedShape`] with the doors' cause.
//! 3. **A MERIDIAN** — *does the loop carry a meridian traversal at
//!    all?* Asked BY the walk, on its traversal kinds
//!    (`walk::require_a_meridian`), refusing
//!    [`TessellateError::MeridianFreeCurvedFace`], whose doc carries
//!    the argument. It is an existence question and not a span one:
//!    two rims at two levels refuse as one rim does. What it stands in
//!    front of is a rectangle of zero height, which question 5 admits
//!    (every entry is on its box —
//!    `tests::a_zero_height_box_passes_the_walk_consistency_check`) and
//!    the CDT triangulates to nothing. Props admits the sphere member,
//!    so neither door above refuses it.
//! 4. **TWO COLUMNS** — *can a loop with no rim open a u-extent at
//!    all?* Asked BY the walk, on the loop's incidence
//!    (`walk::require_two_columns`), refusing
//!    [`TessellateError::SingleColumnCurvedFace`], whose doc carries the
//!    argument. The mirror of question 3 on the other axis, and the
//!    mirror only in what it stands in front of: a rectangle of zero
//!    WIDTH, which question 5 admits for the same reason. It is not a
//!    span question either — the columns are never compared. A rim-free
//!    loop's extent comes from the distinct EDGES that open its iso
//!    sides, so the question is whether two of those exist. Both doors
//!    above admit the members; the branch door (question 2) takes the
//!    one member whose meridian carries a pole mid-edge. The one-face
//!    ball's loop, one wrap edge from pole to pole and back, is not
//!    asked: the walk places its two sides a period apart
//!    (`walk::wrap_strip`).
//! 5. **WALK CONSISTENCY** — *did the walk trace that rectangle?*
//!    Asked after, on the polygon, BANDED in metres
//!    ([`require_swept_rectangle`], refusing
//!    [`TessellateError::UnsupportedCurvedDomain`]).
//!    `walk::iso_side_starts` makes an iso side carried by several
//!    edges exactly straight at the source, so the band separates
//!    nothing in tree and is kept as a backstop with a synthetic
//!    witness (#653; the argument is at `entries_off_bbox`).
//!
//! Boundary polyline segments are inserted as CDT **constraints**, so
//! the triangulation conforms to the shared chord segments in both
//! adjacent faces (the watertightness guarantee).
//!
//! Pole fans: pole corners enter the CDT at their (distinct) UV
//! locations but map to the single pole mesh vertex; any triangle with
//! two corners of the same mesh vertex is degenerate in 3-D and is
//! dropped, which collapses the strip along the pole side into a fan
//! around the pole (one dropped triangle per collapsed side; its two
//! non-collapsed edges become the identified fan edges).
//!
//! **That count — ONE dropped triangle per collapsed side — is a
//! claim about the grid, not a theorem** (issue #678). It holds only
//! while the interior grid SEPARATES the two pole entries;
//! [`pole_columns`] is what makes that true and carries the argument,
//! including the one column count that falsifies it.
//!
//! **The two things that hold it up run in different builds, and which
//! builds is a manifest setting.** The floor is three lines and runs
//! everywhere. The `debug_assert` in [`tessellate_curved`]'s emit pass
//! that re-derives the conclusion over the patch (D2 addendum row 5)
//! is `#[cfg(debug_assertions)]`, which cargo's release DEFAULT would
//! drop — but the root `Cargo.toml`'s `[profile.release]` sets
//! `debug-assertions = true`, so **every profile this workspace builds
//! today runs the re-derivation**. "Compiled out of every shipping
//! build" is therefore the wrong tense for this tree: the guard is
//! cfg-conditional, and today the condition holds. That stanza is a pre-publish
//! posture and is on `DESIGN.md`'s *Before publishing* list to come
//! back out; with it gone, the floor is the entire guard in release,
//! for a class whose failure is a *silently* non-watertight mesh
//! returned as `Ok`. `tessellate` does not run
//! [`crate::validate::check_mesh`] in any build.
//!
//! **The MECHANISM is settled and the flag is not what settled it.**
//! The state this re-derives is D2 addendum **row 5** — the crate
//! computes `nu`/`nv` itself from `(surface, delta)`, so a firing means
//! the kernel's own sizing corrupted a mesh from a body
//! `topo::validate` accepts — and D9's converse half makes a panic the
//! obligation for such a state, not a tolerance. Whether release should
//! instead REFUSE typed was asked and ruled out for a row-5 state, in
//! **#884**, because downgrading a bug to a typed error launders it
//! into a supported outcome. The flag decides only the REACH.
//!
//! **What the ruling depends on, stated because it does depend on it:**
//! the competing reading is row 2 (*valid but unbuilt*, hence a typed
//! refusal), and it turns on whether [`pole_columns`] closes the
//! `nu == 2` class. If that floor is ever falsified, the state moves and
//! so does the mechanism.
//!
//! **The two cases that sat outside this re-derivation are now inside
//! one or the other of two, decided by measurement (issue 897).** The
//! full-2π seam — held off by [`pole_columns`]' arithmetic rather than
//! by any check — is covered by widening the emit pass's census from
//! pole-incident edges to IDENTIFIED ones ([`identified_ids`]).
//! Cross-face identification is outside a per-patch census by
//! construction, whatever its footprint, so it is re-derived once per
//! mesh instead, at the end of [`fn@crate::tessellate`], as a use count
//! over the chord segments the adjacent faces are supposed to share.
//!
//! **Both are `cfg`-CONDITIONAL, which today means both RUN in a
//! release build, and that is the ruled state rather than a gap in
//! it.** S65's ruling is that no guard for this class ships
//! UNCONDITIONALLY; `#[cfg(debug_assertions)]` is what that means in
//! code, and the manifest is what decides the reach. With the root
//! `Cargo.toml`'s pre-publish `[profile.release] debug-assertions =
//! true` in place — as it is now — every profile this workspace builds
//! runs both censuses, at the measured cost (donut, the corpus's
//! largest mesh: +13% to +15% of `tessellate` for the pair). When that
//! stanza comes back out at publish, both go quiet in release together
//! and the floor is again the whole of the shipped guard. Neither
//! census reads a tolerance in any build.
//!
//! Grid sizing: each kind's step is the coarsest its worst-triangle
//! theorem grants at δ (`crates/mesh/README.md` `[mesh.sizing]`; each
//! theorem is stated at its step function):
//! cylinder — hu = [`crate::sizing::sagitta_step`]`(δ, r)`, no interior
//! rows (ruled in v; its triangles are its rims' chords);
//! cone — hu = [`crate::sizing::cone_grid_step`] at the face's largest
//! rim, no interior rows (ruled in v; with both rims chorded on hu its
//! triangles stay within one column, so rows would move no bound);
//! sphere — `(hu, hv)` = [`crate::sizing::sphere_grid_steps`] at the
//! face's widest latitude; torus —
//! `(hu, hv)` = [`crate::sizing::torus_grid_steps`], one step per chart
//! direction from the doubly-curved chord bound that function derives.
//! The chord pass's one rule ([`crate::chords`]) chords every rim and
//! meridian at most those steps apart.
//!
//! # The lattice metric
//!
//! The CDT runs where the lattice is near square: each point enters at
//! [`crate::cdt::lattice_point`] under [`cdt_metric`], the face's own
//! lattice's metric ([`crate::chart_table::AnalyticRow::cell_metric`]:
//! its realized cell aspect to the nearest power of two), on every
//! chart. It is never the trimmed lane's sphere choice, the chart's own
//! metric at mid latitude: that is no lattice's spacing, and near a pole
//! it is 8 to 32 times one.
//!
//! Delaunay is not stretch-invariant. A chord point the grid did not
//! place (a meridian's, between two rows; a rim's, between two columns)
//! has, in raw UV, an empty circle `√(t(1 − t))·a` cells wide across the
//! lattice's long axis, `t` the point's fraction of its cell and `a` the
//! cell's aspect. A thin torus band's lattice is `a ≈ √(R/r)` times
//! taller than wide, and a meridian of it whose chord count is not its
//! row count (a whole turn's one slit, or a partial turn's two meridian
//! edges) put points between the rows that joined rim points four
//! columns away: 7.7 δ on PROFILE's filleted plate (R 0.305, r 0.005,
//! δ 1e-3). In the metric `a` is within `√2` of one and the circle under
//! a cell wide, inside the premise of `sizing::sphere_grid_steps`'
//! containment lemma (a cell aspect under `2√2` on either axis) on every
//! chart. A sphere's counts alone do not keep that premise: over this
//! crate's suite a curved sphere face with rows reaches raw aspects
//! from 0.032 (a one-column wedge) to 2.64, 29 of 3,045 such faces
//! outside it. In the metric every one enters the CDT between 0.71 and
//! 1.41.
//!
//! The lane's other arguments about its triangulation are kept by the
//! metric, which scales each axis by a positive constant and so keeps
//! the order of points along each row and column and every midpoint:
//! [`pole_columns`]' separation of the two pole entries (order and
//! equidistance along `u`, and re-derived over every patch by the emit
//! pass's census), and the ruled charts' strip argument
//! (`sizing::sagitta_step`, stated for any axis scaling).
//!
//! **What is measured and not proved.** The theorem above bounds a
//! lattice half cell; a triangle beside an off-lattice chord point has
//! no containment bound, and the certificate is what holds it. Measured
//! (`tests/the_one_slit_band.rs`'s bands; the review's 720), such a
//! triangle spans at most one column and at most `max(1, nv/m)` row
//! gaps, `nv` the face's rows and `m` its meridian's chords: a meridian
//! chord itself spans `nv/m`
//! (`work/tesslane/the-curved-lanes-triangles-beside-an-off-lattice-chord-point-have-no-containment-bound.md`).

use std::collections::HashMap;

use geom::Surface;
use geom_brep::PolygonOrientation;
use geom_core::{Band, Point3};
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::props::LoopEdgesError;
use topo::{Body, EdgeKey, FaceKey, LoopKey};

use crate::cert;
use crate::chart_table::{Analytic, AnalyticRow};
use crate::sizing::{Eps, SizingTols, ceil_count};
use crate::tessellate::{Patch, PatchVertex};
use crate::types::TessellateError;
use crate::walk::{Chart, UvPoint, gap_is_noise, loop_polygon};

/// What [`tessellate_curved`] refuses a ringed face with: the router's
/// defect, since `tessellate::route_of` sends a ringed face to the trimmed
/// lane.
pub(crate) const RINGED_FACE_ROUTED_HERE: &str =
    "a ringed face reached the curved lane, which reads the outer loop only (router defect)";

/// What [`tessellate_curved`] refuses a face with whose outer loop passes
/// one point record at two vertices ([`crate::walk::pinches`]): the walk
/// places junctions by position, and cannot put two copies of one point
/// on two sides of a slit. Such a face is the trimmed lane's
/// (`tessellate::route_of`'s pinch fact), and one here is the router's
/// defect.
pub(crate) const PINCHED_FACE_ROUTED_HERE: &str = "a face whose outer loop passes one point at \
     two vertices reached the curved lane, whose walk places junctions by position (router \
     defect)";

/// What [`tessellate_curved`] refuses a ruled face with when a chord
/// point lies inside one of its straight sides, as
/// [`TessellateError::UnsupportedCurve`] on that side's edge. A vertex
/// there sends the face to the trimmed lane
/// (`walk::splits_a_straight_side`), so what is left is a chord point the
/// chord pass put on the line: a spline neighbour's lattice raises a line
/// edge's chord count (`chords::tighten`), and no body this build makes
/// puts one beside a ruled face
/// (`work/tesslane/a-spline-neighbour-can-put-a-chord-point-on-a-ruled-faces-straight-side.md`).
pub(crate) const SIDE_CHORD_POINT: &str = "a chord point inside a straight side of a ruled face, \
     whose rectangle-lane arm takes bare sides only (a spline neighbour's lattice raised the \
     line's chord count)";

/// Tessellates one curved face into a [`Patch`]: the interior grid
/// points the face mints for itself, and its outward-wound triangles.
///
/// `shared` is the mesh arena as it stood before ANY face ran — the
/// topology vertices and the chord points, which is everything a
/// curved face reads from outside itself.
pub(crate) fn tessellate_curved(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    rectangle: &Rectangle,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    shared: &[Point3<f64>],
    tol: &SizingTols,
) -> Result<Patch, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    // This lane reads the outer loop alone, so a ring reaching it would be
    // meshed as face. A ringed face is the trimmed lane's
    // (`tessellate::route_of`), and one here is the router's defect.
    if !face.rings.is_empty() {
        return Err(TessellateError::MissingEntity {
            what: RINGED_FACE_ROUTED_HERE,
        });
    }
    if crate::walk::pinches(body, face.outer)? {
        return Err(TessellateError::MissingEntity {
            what: PINCHED_FACE_ROUTED_HERE,
        });
    }
    // Questions 1 and 2 (module docs) are the route's: a face reaches
    // this lane only where both doors passed (`tessellate::route_of`, by
    // [`iso_rectangle_shape`]), so everything the walk does below — the
    // collapse of same-kind runs in `iso_side_starts` above all — reads a
    // loop of rims and meridians each on one chart branch.
    let chart = Chart::of(surface).ok_or(TessellateError::MissingEntity {
        what: "curved chart",
    })?;
    let polygon = loop_polygon(
        body, &chart, rectangle, chords, shared, fk, face.outer, tol.eps, tol.band,
    )?;
    // A COUNT, and only a count. Every loop measured into this line came
    // from a state questions 3 and 4 now refuse on their own terms first
    // — a one-generator cylinder face counted two entries — so this is
    // BELIEVED UNREACHABLE BY INPUT, and left as a typed refusal rather
    // than promoted to `unreachable!`, which would want a proof this does
    // not have. What would show the belief wrong: a loop whose every
    // traversal is a continuation of one iso side and whose edges carry
    // fewer than three chord ids between them, with two distinct opening
    // edges so question 4 admits it. A construction that reaches this
    // line is a row for it, not a reason to delete it.
    if polygon.len() < 3 {
        return Err(TessellateError::MissingEntity {
            what: "degenerate curved boundary",
        });
    }

    // Domain bbox and orientation.
    let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    let mut area2 = 0.0;
    for (i, e) in polygon.iter().enumerate() {
        u0 = u0.min(e.u);
        u1 = u1.max(e.u);
        v0 = v0.min(e.v);
        v1 = v1.max(e.v);
        let n = &polygon[(i + 1) % polygon.len()];
        area2 += e.u * n.v - n.u * e.v;
    }
    // The swept-UV-rectangle contract, CHECKED rather than assumed.
    // Everything below — and the interior grid in particular — is a
    // function of this box, not of the polygon; see
    // `require_swept_rectangle`.
    // The chart's lever arms per entry, so the check below can measure
    // in metres (`entries_off_bbox`); u's varies over the loop on a
    // cone and a sphere, which is why it reads the entry's own point.
    let levers: Vec<(f64, f64)> = polygon
        .iter()
        .map(|e| (chart.radial(shared[e.id as usize]), chart.v_lever()))
        .collect();
    require_swept_rectangle(fk, &polygon, &levers, (u0, u1, v0, v1), tol.eps)?;

    // The polygon's orientation is READ, never taken from the face's
    // `sense`: the CDT's triangles are counter-clockwise in the chart
    // and the patch's are wound as the polygon is. The walk puts the
    // sense in the polygon (`PolygonOrientation::of_face` is the one it
    // then has): from the loop's STORED TRAVERSAL order, which
    // interior-left ties to the outward normal, and on the wrap strip,
    // whose loop walks one edge both ways and so has no order to read,
    // from the loop's stored lift (`walk::wrap_strip`). `revert` reverses the
    // loops AND flips `sense` together, so folding the bit in here as
    // well would double-count the reversal and emit inward-wound
    // triangles.
    let flip = PolygonOrientation::of_signed_area(area2) == Some(PolygonOrientation::Clockwise);

    // The walk's chart singularity, as `grid_counts` reads it: whether
    // a pole/apex entry is present, and how many INTERIOR chord points
    // the meridian sides carry between the pole row and the opposite
    // row. Zero on a cone (its meridian is a straight ruling); always
    // several on a sphere (an arc) — which is the occlusion #678's
    // sphere-arm unreachability argument rests on, so it is measured
    // and asserted rather than assumed. `u0`/`u1` are the polygon's
    // own extremes and an iso side is bitwise straight since #653, so
    // the side test is exact.
    let has_pole = polygon.iter().any(|e| e.pole);
    // An entry inside a meridian side, strictly between its rows.
    let inside_a_side = |e: &&UvPoint| e.v > v0 && e.v < v1 && (e.u <= u0 || e.u >= u1);
    let meridian_interior = polygon
        .iter()
        .filter(|e| !e.pole && inside_a_side(e))
        .count();
    // A ruled face's sides are bare: the strip argument at `grid_counts`'
    // ruled arms holds every triangle between two neighbouring rulings.
    if crate::chart_table::ruled(surface)
        && let Some(e) = polygon.iter().find(inside_a_side)
    {
        let edge = crate::walk::loop_edges(body, face.outer, fk)?
            .into_iter()
            .map(|(ek, _)| ek)
            .find(|ek| chords.get(ek).is_some_and(|ids| ids.contains(&e.id)))
            .ok_or(TessellateError::MissingEntity {
                what: "edge chords",
            })?;
        return Err(TessellateError::UnsupportedCurve {
            edge,
            note: SIDE_CHORD_POINT,
        });
    }

    // Grid counts per chart (module docs), from the face's chart row,
    // which the chord pass reads too, so a face's rims and its grid come
    // from one value.
    let row = AnalyticRow::of(body, fk, surface)?;
    let (nu, nv) = grid_counts(
        &row,
        tol.delta,
        (u1 - u0, v1 - v0),
        has_pole,
        meridian_interior,
    )?;

    // `meta`, the grid and every certificate read UV.
    let metric = cdt_metric(&row, (u0, u1), (v0, v1), (nu, nv));
    // CDT: boundary entries (fixed walk order) + constraints + grid.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    // Per-CDT-vertex metadata, indexed by handle index (insertion order).
    let mut meta: Vec<(f64, f64, PatchVertex, bool)> = Vec::new();
    let insert = |cdt: &mut ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
                  meta: &mut Vec<(f64, f64, PatchVertex, bool)>,
                  u: f64,
                  v: f64,
                  id: PatchVertex,
                  pole: bool|
     -> Result<spade::handles::FixedVertexHandle, TessellateError> {
        // `meta` keeps the chart's own `(u, v)`; only the CDT sees the
        // floored point.
        let h = cdt
            .insert(crate::cdt::lattice_point(u, v, metric))
            .map_err(|_| TessellateError::Triangulation { face: fk })?;
        if h.index() == meta.len() {
            meta.push((u, v, id, pole));
        }
        // Two mesh ids at one point (two vertices, or two coincident
        // edges' chord points) would share this handle's one id. A walk
        // that is its own UV box (`require_swept_rectangle`) passes no
        // point twice but at a pole, where one id repeats; the pinch read
        // is `planar::Pinches`, and this lane refuses rather than mesh
        // the second id's triangles under the first.
        match (meta[h.index()].2, id) {
            (PatchVertex::Shared(was), PatchVertex::Shared(now)) if was != now => {
                Err(TessellateError::PinchWedge { face: fk })
            }
            _ => Ok(h),
        }
    };
    let mut handles = Vec::with_capacity(polygon.len());
    for e in &polygon {
        handles.push(insert(
            &mut cdt,
            &mut meta,
            e.u,
            e.v,
            PatchVertex::Shared(e.id),
            e.pole,
        )?);
    }
    for i in 0..handles.len() {
        let (a, b) = (handles[i], handles[(i + 1) % handles.len()]);
        if a != b && !cdt.exists_constraint(a, b) {
            // `add_constraint` panics on crossing constraints (corrupt
            // input geometry) — pre-check to keep failure typed.
            if !cdt.can_add_constraint(a, b) {
                return Err(TessellateError::Triangulation { face: fk });
            }
            cdt.add_constraint(a, b);
        }
    }
    // GRID AFTER CONSTRAINTS, and it costs this lane nothing: the open
    // ranges below put every grid point strictly inside every
    // constraint BECAUSE `require_swept_rectangle` above has already
    // established that the boundary polygon IS this box — to within
    // the ε band it measures against, which is the honest statement of
    // what the GUARD proves. Since #653 the walk proves more: every
    // iso side gets one coordinate however many edges carry it, so the
    // polygon is its own box bitwise and the residual ulps that used to
    // make the CDT emit slivers here are gone at the source. The
    // premise, not the ordering, is what carries the argument — which
    // is why it is a refusal rather than a comment. (The hazard
    // `planar::triangulate_chart`'s header warns about is a
    // precondition of PLANAR's crossing bookkeeping, which this lane
    // does not build; the tests below pin what spade actually does on
    // a split, which is what that warning turns on.)
    let (uspan, vspan) = (u1 - u0, v1 - v0);
    // The face's OWN points, numbered from zero: a grid point takes the
    // next local index, and takes it only if the CDT did not dedupe it
    // onto a vertex already inserted. One index per point KEPT, so the
    // indices are exactly `0..interior.len()`.
    let mut interior: Vec<Point3<f64>> = Vec::new();
    for j in 1..nv {
        #[allow(clippy::cast_precision_loss)]
        let v = v0 + vspan * (j as f64 / nv as f64);
        for i in 1..nu {
            #[allow(clippy::cast_precision_loss)]
            let u = u0 + uspan * (i as f64 / nu as f64);
            let p = surface.eval(u, v);
            #[allow(clippy::cast_possible_truncation)]
            let id = PatchVertex::Local(interior.len() as u32);
            let h = insert(&mut cdt, &mut meta, u, v, id, false)?;
            if h.index() == meta.len() - 1 && meta[meta.len() - 1].2 == id {
                interior.push(p);
            }
        }
    }

    // Emit: drop pole-degenerate triangles, certify the rest.
    let mut triangles = Vec::new();
    // The face's verdict over its triangles (`cert::FaceVerdict`).
    let mut verdict = cert::FaceVerdict::new();
    let mut book = crate::join::Book::default();
    for f in cdt.inner_faces() {
        let vs = f.vertices();
        let m: Vec<(f64, f64, PatchVertex, bool)> =
            vs.iter().map(|v| meta[v.fix().index()]).collect();
        let ids = [m[0].2, m[1].2, m[2].2];
        if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
            continue; // pole-collapsed sliver
        }
        let tri = [
            ids[0].position(shared, &interior),
            ids[1].position(shared, &interior),
            ids[2].position(shared, &interior),
        ];
        let uv = [[m[0].0, m[0].1], [m[1].0, m[1].1], [m[2].0, m[2].1]];
        let pole = [m[0].3, m[1].3, m[2].3];
        // A pole or apex entry has no azimuth (`crate::chart_table`).
        let (bound, scale) = row.cert(tri, uv, pole);
        verdict.read(bound, tol.delta, scale, tol.eps);
        let wound = crate::tessellate::Wound(flip);
        book.push(&mut triangles, wound.of(ids), wound.of(uv), wound.of(pole));
    }

    // D2 addendum row 5 (`DESIGN.md`: kernel bug detectable only by
    // re-derivation -> `debug_assert`). The pole-fan argument in the
    // module header is structural, and the one thing that ever
    // falsified it — #678's two entries fanning over one shared
    // column — surfaced by accident, in a review of an unrelated PR.
    // Nothing else re-derives manifoldness in any build: `tessellate`
    // does not run `check_mesh`, so with `pole_columns` in place a
    // three-line arithmetic coincidence is otherwise the only thing
    // between a pole face and a silently non-manifold mesh. Re-derive
    // the conclusion here, over THIS patch's IDENTIFIED edges only
    // (the class's whole footprint, and O(triangles)): a fan edge is
    // interior to the patch and used twice, or on its boundary and
    // used once with the neighbouring face supplying the other use.
    // Four uses is the #678 signature.
    //
    // IDENTIFIED, not pole-incident (issue 897). [`identified_ids`]
    // carries why a seam double-traversal and a pole corner are one
    // set; [`pole_columns`]' arithmetic holds the full-2π seam off in
    // the lane that actually has seams, and this census is the check
    // that arithmetic stands in for.
    //
    // THE PRICE, and where it is paid. A face whose walk identifies
    // nothing pays one hash-set build over its boundary polygon and
    // then one branch: the set is empty, so no patch is materialised
    // in mesh ids and no edge is counted. Debug assertions are ON in
    // this crate's release profile (workspace `Cargo.toml`), so that
    // is the shipped path and not a test-only one — which is why the
    // empty case must not allocate.
    //
    // A face that identifies something pays a copy of its own patch
    // and then a scan of the copy, O(triangles) in both. The SCAN is
    // what is measured: on the donut — two torus patches, each
    // carrying a seam of `nv + 1` identified ids, 648 to 16 080
    // triangles over δ = 0.1 to 0.004 — it is 2.2 % to 2.7 % of
    // `tessellate` (dev profile with this crate at opt-level 2; median
    // of four warm rounds, whose spread is under 2 %). The copy sits
    // on top of that and is not separately measured. Under the price
    // already paid for the pole half, and it buys the case a
    // mechanical check.
    // This lane reads no fold.
    let splits = book.splits(&triangles);
    let patch = Patch {
        interior,
        triangles,
        splits,
    };
    #[cfg(debug_assertions)]
    {
        let over = overused_identified_edge(&polygon, || patch.census_ids(shared));
        debug_assert!(
            over.is_none(),
            "face {fk:?}: identified-vertex fan edge {:?} used {} times in one \
             patch (nu = {nu}, nv = {nv}); the collapse-and-drop argument is \
             falsified — see curved::pole_columns, issue #678",
            over.map(|(e, _)| e),
            over.map_or(0, |(_, n)| n)
        );
    }

    verdict.refusal(fk, tol.delta)?;
    Ok(patch)
}

/// **The two doors' "rectangle" answer, as a value**: minted by
/// [`iso_rectangle_shape`] alone, where both doors pass, and taken by
/// [`tessellate_curved`], so a face reaches this lane only on that
/// answer and no route can hand it one without asking.
///
/// It carries what the shape door decided on the way: each outer-loop
/// half-edge's iso family, a meridian (`U`) or a rim (`V`)
/// (`geom_brep::props::require_iso_rectangle`). The walk and the chord
/// rule read that one decision ([`Rectangle::side`]) rather than
/// classifying the carriers a second time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Rectangle {
    sides: Vec<(topo::HalfEdgeKey, geom_brep::IsoFamily)>,
}

impl Rectangle {
    /// The iso family the shape door certified for the outer-loop
    /// half-edge `he`; `None` for a half-edge the door was not asked
    /// about, which on this lane is a routing defect.
    pub(crate) fn side(&self, he: topo::HalfEdgeKey) -> Option<geom_brep::IsoFamily> {
        self.sides
            .iter()
            .find_map(|&(h, family)| (h == he).then_some(family))
    }
}

/// A curved face's shape at the two doors ([`iso_rectangle_shape`]).
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Shape {
    /// An iso-parameter rectangle whose every boundary arc stays on one
    /// branch of the chart: this lane's domain.
    Rectangle(Rectangle),
    /// Decided to be no such rectangle, with the doors' verdict.
    NoRectangle(geom_brep::props::PropsError),
}

/// **The SHAPE door and the BRANCH door, read as the route**: this
/// face's outer loop, flattened by [`topo::props::loop_edges`] (the cycle
/// the walk reads), handed to `geom_brep::props::require_iso_rectangle`
/// and then to `require_one_chart_branch`, at the run's props band.
/// `tessellate::route_of` asks it of a ringless face whose stored images
/// are iso lines.
///
/// - Both pass: [`Shape::Rectangle`], this lane's, with the
///   [`Rectangle`] it takes.
/// - A door DECIDES the face is no such rectangle — `NotIsoRectangle`
///   (a notch, rims that encode different sides, a carrier of no iso
///   kind), `Unimplemented` (a spline or spiric carrier),
///   `NotOneChartBranch` (an arc through the chart singularity, which
///   this walk would read a half-turn away): [`Shape::NoRectangle`] with
///   the verdict, valid input this lane is not built for.
/// - Any other refusal is no verdict on the shape — a decision in band,
///   stored data that contradicts its surface or its certification —
///   and is [`TessellateError::UnsupportedCurvedShape`] with that cause.
///   The match names every variant, so one props adds is classified
///   here before it compiles.
///
/// Asking the shape door first means a doubly-defective face reports its
/// shape; the branch door decides arc membership on carriers the parse
/// has certified. The branch door is `mesh`'s and not the flux lane's,
/// which folds the singularity in and measures such a face exactly (the
/// argument is at the predicate). An empty loop is this crate's
/// [`TessellateError::EmptyLoop`], reported before the flatten.
pub(crate) fn iso_rectangle_shape(
    body: &Body<f64>,
    fk: FaceKey,
    lk: LoopKey,
    surface: &Surface<f64>,
    band: Band,
) -> Result<Shape, TessellateError> {
    use geom_brep::props::PropsError as P;
    let lp = body.cycle_loop(lk).map_err(|miss| match miss {
        topo::NoCycle::Stale => TessellateError::MissingEntity { what: "loop" },
        topo::NoCycle::Torn => TessellateError::MissingEntity {
            what: "a loop that does not walk",
        },
        topo::NoCycle::Lone(_) => TessellateError::EmptyLoop { face: fk },
    })?;
    // `loop_edges` reports exactly the two states a flatten can reach,
    // in its own type, so this match is exhaustive over them and a
    // third arm minted in `topo` is a compile error here — D2 row 0
    // across the crate boundary, in place of an `unreachable!` over
    // arms the flatten never produced.
    let (outer, half_edges) = topo::props::loop_edges(lp).map_err(|e| match e {
        LoopEdgesError::NullScaffoldEdge { edge } => TessellateError::NullScaffoldEdge { edge },
        LoopEdgesError::Corrupt { what } => TessellateError::MissingEntity { what },
    })?;
    let doors = geom_brep::props::iso_rectangle_sides(surface, &outer, band).and_then(|sides| {
        geom_brep::props::require_one_chart_branch(surface, &outer, band).map(|()| sides)
    });
    match doors {
        // The door answers one side per outer-loop edge, in loop order;
        // a count that differs is a router defect, refused rather than
        // truncated by the zip.
        Ok(sides) if sides.len() != half_edges.len() => Err(TessellateError::MissingEntity {
            what: "the shape door answered a side count other than the loop's half-edge count \
                   (router defect)",
        }),
        Ok(sides) => Ok(Shape::Rectangle(Rectangle {
            sides: half_edges.into_iter().zip(sides).collect(),
        })),
        Err(
            verdict @ (P::NotIsoRectangle { .. } | P::Unimplemented | P::NotOneChartBranch { .. }),
        ) => Ok(Shape::NoRectangle(verdict)),
        Err(
            source @ (P::Escalated { .. }
            | P::OffSurface { .. }
            | P::SpanUncertified { .. }
            | P::Defect { .. }
            | P::NappeSpanning
            | P::SphereLoop { .. }
            | P::ConeLoop { .. }
            | P::HolesExceedFace
            | P::SenseContradicted
            | P::SenseInverted
            | P::SeamRegionOvercovered { .. }
            | P::SeamRegionCoverUnresolved
            | P::SeamRegionSensesDisagree
            | P::RingWoundWithFace
            | P::ChartLoopOpen
            | P::LoopsWindChart
            | P::DegenerateFace
            | P::QuadratureUnsupported { .. }),
        ) => Err(TessellateError::UnsupportedCurvedShape { face: fk, source }),
    }
}

/// The walk entries that do NOT lie on the boundary of the UV bounding
/// rectangle `[u0, u1] × [v0, v1]`, each with **how far off it is in
/// metres** — empty ⟺ the domain IS that rectangle.
///
/// Sufficient, and it is the shape that matters: a rectilinear simple
/// polygon has a re-entrant corner exactly when some vertex sits
/// strictly inside its bounding box, and a re-entrant corner is what
/// lets an interior grid point land on — or across — a boundary
/// constraint.
///
/// **THE COMPARISON IS BANDED, IN METRES, AND IS A BACKSTOP.** Each
/// axis' gap is converted to metres by its own lever arm
/// ([`Chart::radial`] for u — the point's distance from the axis — and
/// [`Chart::v_lever`] for v) and banded by [`crate::walk::gap_is_noise`],
/// the crate's ONE ε band and its only caller.
/// The band admits ulp wobble on a straight side and nothing else: a
/// genuine re-entrant corner (a keyway, a milled flat) is a FEATURE
/// width off the box, six or more orders of magnitude outside ε.
///
/// Every walk this build produces sits on its box BITWISE, so no
/// in-tree fixture needs the band and its red-when-reverted row is
/// synthetic and says so:
/// `the_band_admits_a_sub_eps_entry_that_the_exact_form_refuses`.
/// Kept anyway, because the direction of a wrong answer is asymmetric
/// here: this guard REFUSES, so an over-tight comparison rejects a
/// valid part, while an over-loose one lets through a wobble six
/// orders of magnitude smaller than the feature it would have to be
/// confused with. What makes the entries bitwise straight, and what
/// would stop doing so, is `walk::iso_side_starts`.
///
/// **A lever of ZERO is not a zero distance, it is NO metric — and an
/// axis without one admits nothing.** [`Chart::radial`] vanishes
/// exactly on the chart axis (a sphere pole, a cone apex), where every
/// u names the same point, so `gap_is_noise(du, 0, eps)` is `0 < eps`
/// and would admit that entry at any u at all — the fail-open
/// `walk`'s own `unreachable!` refuses to write, whose retired default
/// was this same zero lever. A pole entry is admitted on its other
/// axis instead, and is entitled to be: the walk gives it the CHART's
/// pole v ([`Chart::poles`]) and the box is the entries' own min/max,
/// so its v gap is zero bitwise — the fact the old exemption rested
/// on silently, asserted now by `a_pole_entry_sits_on_its_own_box_in_v`.
///
/// It takes the caller's own bounding box rather than recomputing one,
/// so the rectangle it checks is exactly the box the interior grid
/// spans. `levers` is parallel to `poly`.
///
/// # Closed at bitwise zero, NARROWED rather than eliminated
///
/// `lever > 0.0` refuses the exact-`0.0` lever a chart singularity
/// produces, and that is the whole class an adversarial sweep of this
/// build found: 834 zero-lever entries across five crates, every one a
/// pole with `lu == 0` and its `v` gap bitwise zero. It does **not**
/// close the band on a lever that is merely TINY. At `lu = 1e-20`,
/// `gap * lu < eps` is true for every gap a UV coordinate can hold, so
/// a sub-ε lever admits exactly the way a zero one did.
/// `walk::iso_side_starts` bars the same `Chart::radial` with
/// `radial(junction) > eps` — **one predicate, two spellings, and this
/// is the looser one.** Nothing in tree produces a non-zero sub-ε
/// lever (`radial` is a distance from an axis; a point is on it or a
/// modelling distance away from it), so widening this to `> eps` is a
/// behaviour change on a class nobody has shown reachable. Stated
/// here rather than taken.
///
/// **`lv` vanishing is UNEXERCISED, not handled.** Every zero lever in
/// the tree is a `lu` at a pole, so the payload's `(true, false)` and
/// `(false, false)` arms below are argued and not measured. The route
/// that would exercise them — a torus whose `R < r` gives a horn or
/// spindle axis point — is refused by `sweep::revolve` at construction
/// and, on the import door which reads both radii verbatim, by
/// `topo::validate`'s tier-3 `DegenerateTorus`; so no body reaching a
/// mesh at rest carries one, and this guard REFUSES such a face rather
/// than meshing it in any case.
fn entries_off_bbox(
    poly: &[UvPoint],
    levers: &[(f64, f64)],
    (u0, u1, v0, v1): (f64, f64, f64, f64),
    eps: Eps,
) -> Vec<(usize, f64)> {
    // An axis puts the entry ON the box when its own gap is noise in
    // metres, and an axis with no lever arm has no metric and puts it
    // nowhere (doc above). NaN-safe by construction: `gap_is_noise` is
    // a `<`, so a poisoned coordinate is false on both axes and the
    // entry stays a refusal rather than being admitted.
    let on_box = |gap: f64, lever: f64| lever > 0.0 && gap_is_noise(gap, lever, eps);
    // A NaN lever would read as "no metric" on both tests above and
    // produce a refusal reporting 0 m — silent where the crate is
    // supposed to be loud. `Chart::radial` is a norm and cannot
    // produce one from finite input, so this is a kernel-defect
    // detector, not an input check.
    debug_assert!(
        levers.iter().all(|&(lu, lv)| !lu.is_nan() && !lv.is_nan()),
        "a lever arm is NaN: the payload below cannot distinguish it from a \
         degenerate chart axis, and both report 0 m"
    );
    poly.iter()
        .zip(levers)
        .enumerate()
        .filter_map(|(i, (e, &(lu, lv)))| {
            // Every entry is inside the box by construction (the box is
            // the entries' own min/max), so the spatial distance to the
            // box BOUNDARY is the smallest of the four side gaps, each
            // converted to metres by its axis' lever arm. Taken per
            // axis (nearest of the two sides, then the band) so the
            // lever arm goes to the shared predicate rather than being
            // applied here: `lu`, `lv >= 0`, so per-axis and
            // all-four-at-once select the same gap either way.
            let du = (e.u - u0).abs().min((e.u - u1).abs());
            let dv = (e.v - v0).abs().min((e.v - v1).abs());
            if on_box(du, lu) || on_box(dv, lv) {
                return None;
            }
            // The payload is a DISTANCE, so it reports the axes that
            // have one: at a pole `du * lu` is 0 m for every u, and a
            // refusal reading 0 m tells its reader nothing — the
            // number is what separates "re-author your part" from
            // "kernel bug" (`require_swept_rectangle`).
            let d = match (lu > 0.0, lv > 0.0) {
                (true, true) => (du * lu).min(dv * lv),
                (true, false) => du * lu,
                (false, true) => dv * lv,
                // No metric on EITHER axis: the chart is degenerate in
                // both directions here, so the entry and the box share
                // one point in space and 0 m is the true distance, not
                // a blind default. It reads the same as the payload
                // this match exists to remove — and is not it, because
                // an entry ON the box never reaches here (it returned
                // above), so a reported 0 means exactly "off the box in
                // UV, nowhere in space", which routes to "kernel bug"
                // correctly. UNREACHABLE today: it needs `lv == 0`,
                // which no fixture produces (see the doc).
                (false, false) => 0.0,
            };
            Some((i, d))
        })
        .collect()
}

/// Refuses a curved face whose boundary walk is not its own UV bounding
/// rectangle — the swept-UV-rectangle contract this lane's interior
/// grid rests on, made a typed refusal.
///
/// **This is the WALK-CONSISTENCY question, and only that** (issue
/// 726). The SHAPE question — *is the domain an iso-rectangle at all* —
/// is asked before the walk by the router ([`iso_rectangle_shape`]), on
/// rim structure, through `geom_brep::props`' own door; what reaches here
/// is a face whose domain props has certified rectangular, and the
/// question left is whether the walk traced it: whether every iso side
/// came out straight (#653's ulp wobble, the reason the bar is SPATIAL
/// and banded in metres rather than structural) and whether every
/// entry landed where the rim structure says it should. Two
/// derivations of the shape used to live here and in props; one does
/// now, and this check measures the walk against it.
///
/// **Why a refusal and not a comment.** The grid runs the OPEN ranges
/// `1..nu` × `1..nv` over the walk's own bounding box, which is
/// strictly interior iff the polygon IS that box. When it is not, the
/// grid splits boundary constraints and `inner_faces()` — which this
/// lane keeps wholesale, having no inside/outside classification —
/// emits triangles outside the face: a silently wrong mesh, and
/// [`fn@crate::tessellate`] does not run `check_mesh`, so it would
/// reach the caller unannounced. D9's *"silent discard is never an
/// answer"* makes it a refusal; `trimmed` refuses the same hazard
/// typed ([`TessellateError::SelfTouchingTrimLoop`]), and the
/// structural twin of this arm is the dispatch's: a face holding a ring
/// is no rectangle, so `tessellate::route_of` sends it to the trimmed
/// lane and it never reaches this one.
///
/// **What can still trip it, with the shape door in front.** A walk
/// that failed to straighten a side — sub-ε is absorbed by the band,
/// and above it is a kernel-bug report — and an iso-bounded loop the
/// rim predicate cannot see: a zero-width slit (two meridians up and
/// down one column to an interior level) has every rim at an extreme
/// and a walk entry a feature width inside its box. The payload's
/// distance is what separates the two, which is why it is a distance
/// and not a count.
///
/// **Nothing in tree trips it** (the tests below sweep every chart this
/// build authors plus a boolean-cut face, as authored AND under a
/// general rigid placement with a multiply-carried iso side). The
/// notched domains that used to be kept out of this lane only by other
/// modules' limits — the boolean's `CurvedPierceUnsupported`, tier 3's
/// `VolumeUncomputable { NotIsoRectangle }`, which tier 3 no longer
/// raises on a cylinder, cone or sphere, whose closed forms read any
/// boundary of their kind — are routed to the chart lane by the shape
/// door in this crate ([`iso_rectangle_shape`]).
///
/// **The `walk::iso_side_starts` collapse on two non-iso CARRIERS
/// cannot defeat it any more; the collapse on a non-iso ARC still
/// can.** That collapse merges consecutive same-kind traversals onto
/// one coordinate on the premise that every boundary edge is an iso
/// curve of the chart. Two tilted plane sections of a SPHERE both
/// classify `Rim`, merge onto one `v`, and the polygon IS its own
/// bounding rectangle, which this check cannot tell from a real one.
/// (With no other side the loop has no meridian, and the walk refuses
/// it before a polygon exists — `tests::the_lens_walk_refuses_for_
/// want_of_a_meridian_behind_the_shape_door`; beside a meridian side
/// the merged pair would reach this check and pass it.) The shape door
/// decides that face is no rectangle on
/// `props_rim_axis_parallel` before the walk runs, on every kind
/// (structural: per-edge CARRIER certification — a torus Villarceau
/// lens refuses `props_rim_fit`). The SHAPE door does not certify that
/// a traversed arc stays on one chart meridian — a great-circle arc
/// crossing a pole passes it, and if it reached this walk
/// `mid_azimuth` would read the pole's `u` and the closing column
/// would disagree by π — so a second door certifies it:
/// `props::require_one_chart_branch`, cited beside the first in
/// [`iso_rectangle_shape`], which routes that face away from this walk
/// (issue 1571; the rows are
/// `mesh/tests/mesh11_arc_branch.rs`, and the seam between the two
/// doors on one face is `mesh7r1_probes::a_pole_crossing_arc_is_
/// refused_by_the_branch_door_not_the_shape_door`). The qualification
/// at `walk::iso_side_starts` is closed, and the premise it instanced
/// is now established rather than inherited — `walk`'s header says
/// which door establishes which half.
///
/// **A measured constant in this crate is re-derivable from the tree,
/// or it says it is not.** The band's own doc ([`entries_off_bbox`])
/// is where that bites hardest, and `walk`'s module header states the
/// straight-iso-side premise more fully than any other site.
fn require_swept_rectangle(
    fk: FaceKey,
    poly: &[UvPoint],
    levers: &[(f64, f64)],
    bbox: (f64, f64, f64, f64),
    eps: Eps,
) -> Result<(), TessellateError> {
    let off = entries_off_bbox(poly, levers, bbox, eps);
    let Some(&(first, _)) = off.first() else {
        return Ok(());
    };
    // The payload has to separate "re-author your part" from "kernel
    // bug, file it", and a COUNT cannot: `off_bbox: 1` is what a
    // one-corner keyway reports and what a 6e-17 m wobble reported.
    // The distance is the number that tells them apart (S19's
    // postmortem lesson — the day a refusal fires is the day someone
    // needs its payload), and the (u, v) says where to look. Had the
    // sweep that "proved" the exact form unreachable printed margins
    // instead of pass/fail, the exactness claim would have been
    // visibly fragile before it shipped.
    Err(TessellateError::UnsupportedCurvedDomain {
        face: fk,
        off_bbox: off.len(),
        first_uv: (poly[first].u, poly[first].v),
        max_distance: off.iter().fold(0.0_f64, |m, &(_, d)| {
            if d.is_nan() || m.is_nan() || d > m {
                d
            } else {
                m
            }
        }),
    })
}

/// The pole-fan separation rule on the u column count (issue #678).
///
/// The module header's fan argument — one dropped triangle per
/// collapsed side — needs the interior grid to SEPARATE the walk's two
/// pole entries, and `nu == 2` is the one column count that does not:
/// the single interior column at `u = (u0 + u1)/2` is equidistant from
/// both, the CDT fans both over its upper half, and the identified
/// mesh edge `(pole, column)` is used FOUR times. `check_mesh` reports
/// `NonManifoldEdge`; `tessellate`, which does not run it, returns
/// that mesh as `Ok`.
///
/// `nu == 2` is singular, which is why this is an equality and not
/// `max(3)`. At `nu == 1` the inner grid range `1..nu` is empty, so
/// the entries fan over the BOUNDARY walk, which is ordered along the
/// rim and splits between them at one shared vertex. At `nu >= 3` each
/// entry's nearest column occludes the other's.
///
/// **Moot on the ruled charts, which take no rows.** The cylinder and
/// cone arms of [`grid_counts`] emit no interior points, so there is no
/// column to fan over, and the lane takes a ruled face only with bare
/// sides.
///
/// **Prophylactic on the sphere, and not clean by luck.** A sphere's
/// meridian is an ARC and always carries interior chord points, which
/// occlude the cross-fan — `sphere_wedge(pi/4)` at delta = 0.05 is
/// **clean at `nu = 2, nv = 8`**, seven interior column vertices and
/// an overlap that could have held six edges, which is the observation
/// the paragraph rests on and the one that makes this arm different
/// from the cone's rather than a restatement of it. Dirty needs
/// `nv >= 3`, and the meridian is chorded at the grid's own step
/// (`chords`' one rule), so `nv >= 3` FORCES at least
/// two of those points. The arm is kept (Ev, 2026-08-19) so the rule reads as one
/// sentence rather than one chart but not the other, and the mechanism
/// it rests on — the occluding points EXIST — is asserted in
/// [`grid_counts`]'s sphere arm rather than written down here.
///
/// Both readings come from an A/B over 281 public-API configurations:
/// 7 rows dirty -> clean, every one of them cone; 57 clean rows
/// re-size, 49 of them sphere; no sphere row ever measured dirty, out
/// of ~200; zero drift in the three render corpora. Those last two are
/// #678's own open questions — *does the sphere lane reach the defect
/// in practice*, *is any corpus body in the changed class* — answered,
/// and this is their only home. **Historical** — that sweep is not in
/// the tree and nothing here re-derives it.
///
/// **The option not taken** is `nu == 2 && nv >= 3`, available right
/// here since both counts are in the same arm: it would preserve all
/// eight cone re-sizings, but it makes `nu` a function of `nv` — the
/// schedule's two axes stop being independent — and it leaves the
/// `ceil` knife edge live at `nv <= 3` instead of removing the class.
/// The third option, per-face manifoldness re-derivation over the
/// emitted patch, is the D2-addendum row-5 mechanism and ships BESIDE
/// this floor in [`tessellate_curved`]'s emit pass rather than instead
/// of it.
///
/// Only pole faces with `nu == 2` re-size, and a full revolve is never
/// one: [`crate::sizing::sagitta_step`] hard-caps at
/// [`crate::sizing::MAX_ANGULAR_STEP`] on both branches and
/// [`crate::sizing::torus_grid_steps`]' azimuth step is capped against the same value
/// here, so a `2*pi` span gives `nu >= 8`.
///
/// **That arithmetic is VERIFIED and it is NOT the seam case's whole
/// argument** (issue 897, and the distinction is the finding). Verified:
/// `the_full_2pi_seam_never_reaches_the_two_column_shape` runs the
/// capped step over its whole range and gets `nu >= 8` every time, with
/// the row that goes red if the cap is lowered — at `pi`, four times
/// today's cap, the same `2*pi` span sizes to exactly the two columns
/// the fan needs — so the claim depends on the cap and says which way.
/// **Not the whole argument, and the reason is PER ARM.** A full-`2*pi`
/// seam does not live on one arm; the corpus puts one on three, and the
/// bound means different things on each. The interior grid is
/// `for j in 1..nv { for i in 1..nu }`, so it is EMPTY unless BOTH
/// ranges are — `nv == 1` empties it however many columns `nu`
/// schedules, which is the distinction the bound alone does not make:
///
/// * **Torus** — the seam arm where the bound is fully protective. The
///   donut's two patches carry a seam on both meridians and size
///   `nu x nv` = 27x6 up to 134x30 over the corpus's deltas, i.e. 130
///   up to **3 857** interior grid vertices per patch. Eight columns
///   is a floor on a set with hundreds to thousands of members, and
///   the two seam entries are separated by every one of them.
/// * **Sphere, seam-carrying and pole-free** — protective exactly when
///   `nv >= 2`.
/// * **Cylinder and cone** — the arms where the bound is VACUOUS,
///   structurally and at every delta: [`grid_counts`]' ruled arms return
///   `(nu, 1)` by construction, so the ROW range `1..nv` is empty and
///   no interior point is ever emitted, whatever `nu` says. The
///   washer's walls are `nu` = 10..71 with `nv = 1` throughout. What
///   separates the seam's two entries there is the boundary chord
///   rows, which this paragraph never mentioned.
///
/// So the floor is a real separation on most seam faces and no
/// separation at all on the cylinder ones, and reading it as covering
/// "the seam case" flattens that. The emission is therefore re-derived
/// rather than argued — the emit pass's census runs on every patch the
/// walk identifies a vertex on, on every arm, seam or pole.
fn pole_columns(nu: usize, has_pole: bool) -> usize {
    if has_pole && nu == 2 { 3 } else { nu }
}

/// The mesh ids this patch's boundary walk IDENTIFIES: an id the walk
/// placed at two or more DISTINCT UV locations, together with the
/// pole/apex entries. Empty ⟺ no boundary vertex of this patch is
/// reachable at two places in parameter space, and then the emit
/// pass's re-derivation has nothing to re-derive.
///
/// **The two sources are one set, and that is the point.** A pole
/// corner and a full-2π seam double-traversal are the same situation
/// for the collapse-and-drop argument — one mesh vertex entering the
/// CDT at two parameter locations, with a triangle spanning both
/// dropped as degenerate — and the argument that the drop leaves a
/// fan rather than a hole is the same argument in both. Splitting
/// them into two cases is what let the seam half go unchecked while
/// the pole half was re-derived every build ([`pole_columns`] holds
/// the seam off by arithmetic, and that arithmetic is protective on
/// some arms and vacuous on others — the per-arm reading is there).
///
/// A pole entry that appears ONCE is still in the set: it cannot be
/// collapsed, but including it keeps the census's footprint a
/// superset of the pole-incident one it replaces, so no coverage
/// moves with this definition.
///
/// The repeat half is [`crate::cdt::ids_at_two_uvs`], where the
/// spade-equality rule lives for every lane; this function is that set
/// unioned with the pole flags.
#[cfg(debug_assertions)]
fn identified_ids(polygon: &[UvPoint]) -> std::collections::HashSet<u32> {
    let mut identified = crate::cdt::ids_at_two_uvs(polygon.iter().map(|e| (e.u, e.v, e.id)));
    identified.extend(polygon.iter().filter(|e| e.pole).map(|e| e.id));
    identified
}

/// The identified-vertex edge this patch uses more than twice, if any
/// — the emitted form of #678's class, re-derived over the emission
/// rather than argued from the grid.
///
/// A fan edge around an identified vertex is interior to the patch and
/// used twice, or on the patch boundary and used once with the
/// neighbouring face supplying the other use. Three or more uses in
/// ONE patch means the collapse left something other than a fan, which
/// is the non-manifold state, and four is #678's own signature.
///
/// Returns the edge and its use count so the caller can name both.
///
/// `triangles` is a THUNK, and that is the whole reason this wrapper
/// exists: empty [`identified_ids`] means there is nothing to
/// re-derive, and on that branch the patch is never materialised in
/// mesh ids at all. A wedge wall or an untrimmed patch pays one hash
/// set over its boundary and nothing else.
#[cfg(debug_assertions)]
fn overused_identified_edge(
    polygon: &[UvPoint],
    triangles: impl FnOnce() -> Vec<[u32; 3]>,
) -> Option<((u32, u32), usize)> {
    let identified = identified_ids(polygon);
    if identified.is_empty() {
        return None;
    }
    crate::walk::overused_identified_edge_in(&identified, &triangles())
}

/// **The metric a curved face's CDT runs in** (module docs, "The lattice
/// metric"): its own lattice's, over the box `(u0, u1) × (v0, v1)` of
/// `nu × nv` cells ([`AnalyticRow::cell_metric`]).
pub(crate) fn cdt_metric(
    row: &AnalyticRow<'_>,
    u: (f64, f64),
    v: (f64, f64),
    counts: (usize, usize),
) -> f64 {
    row.cell_metric(u, v, counts)
}

/// The interior grid counts `(nu, nv)` for the face's UV spans, at the
/// face's chart row's steps ([`AnalyticRow::steps`]) and the lattice it
/// lays from them ([`AnalyticRow::lattice_steps`]).
///
/// `has_pole` and `meridian_interior` describe the boundary walk's
/// chart singularity: whether it carries a pole/apex entry at all, and
/// how many interior chord points its meridian sides carry between the
/// pole row and the opposite row ([`pole_columns`]). The sphere and
/// spindle arms read them; the ruled arms take no rows whatever the walk
/// carries, and a cylinder's and a torus's rows list no singular point
/// ([`AnalyticRow::singular_points`]), so `has_pole` is false on those
/// two arms for every input there is, and the `debug_assert`s below say
/// so in a form that fails if it ever stops being true.
///
/// KNOWN SIBLING CLASS, recorded and scheduled rather than decided
/// here: a count of 1 on either axis empties the interior grid's
/// `1..nu × 1..nv` ranges, so the OTHER axis' schedule is computed
/// and dropped — the sphere and torus arms at `nu == 1`, and
/// `trimmed::lattice_candidates`, are the members. The cylinder and
/// cone arms take no rows by their theorems, so their `nu` is the
/// face's column count and nothing more; the rest stay
/// certificate-backstopped (measured watertight and in-tolerance —
/// PR 1507's class sweep) and are the follow-up issue's to decide.
fn grid_counts(
    row: &AnalyticRow<'_>,
    delta: f64,
    (uspan, vspan): (f64, f64),
    has_pole: bool,
    meridian_interior: usize,
) -> Result<(usize, usize), TessellateError> {
    let steps = row.steps(delta);
    let (hu, hv) = steps;
    // The lattice the row lays: the periodic steps capped on a torus and
    // a spindle sheet, which this lane reads alone.
    let (lu, lv) = row.lattice_steps(steps);
    match row.kind() {
        Analytic::Cylinder { .. } => {
            debug_assert!(!has_pole, "a cylinder's row lists no singular point");
            // `(nu, 1)`: ruled in v at constant radius, so no rows in
            // any column count; the cone arm below has the same shape.
            Ok((ceil_count(uspan, hu)?, 1))
        }
        // `(nu, 1)`, the cylinder's shape and for its reason: no rows.
        // `cert::cert_cone` charges a chord across `Δu` no more than its
        // larger radius's sagitta, and with both rims chorded on this
        // step (`chords`' one rule) and nothing on the sides between
        // them (the lane admits no other ruled face), the strip argument
        // at `sizing::sagitta_step` holds every triangle to one column,
        // so the worst one is a rim-chord triangle on the largest rim at
        // δ (`sizing::cone_grid_step`'s theorem). A row would add
        // triangles and move no bound.
        Analytic::Cone { .. } => Ok((ceil_count(uspan, hu)?, 1)),
        Analytic::Sphere { .. } => {
            let (nu, nv) = (ceil_count(uspan, hu)?, ceil_count(vspan, hv)?);
            // D2 addendum row 5 — and the whole warrant for calling
            // this arm PROPHYLACTIC rather than corrective
            // (`pole_columns`). Assert the MECHANISM, not the
            // conclusion: a sphere pole face that reaches `nu == 2`
            // with enough rows to be dirty (`nv >= 3`) is claimed to
            // be saved by the interior chord points its ARC meridians
            // always carry. If one ever arrives without them, the
            // unreachability argument is false — and we want that
            // from this line rather than from another accident, since
            // the class was found by accident once already.
            debug_assert!(
                !(has_pole && nu == 2) || nv < 3 || meridian_interior >= 2,
                "sphere pole face at nu == 2, nv = {nv} carries \
                 {meridian_interior} interior meridian chord points; \
                 #678's unreachability argument needs >= 2 (one per \
                 meridian side)"
            );
            Ok((pole_columns(nu, has_pole), nv))
        }
        Analytic::Torus { .. } => {
            debug_assert!(!has_pole, "a torus's row lists no singular point");
            Ok((ceil_count(uspan, lu)?, ceil_count(vspan, lv)?))
        }
        // The torus's steps; an apex is a pole entry, so a face at one
        // takes the pole column rule as the sphere's does.
        Analytic::Spindle { .. } => Ok((
            pole_columns(ceil_count(uspan, lu)?, has_pole),
            ceil_count(vspan, lv)?,
        )),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! **The swept-UV-rectangle premise this lane's grid rests on,
    //! pinned by the refusal that now enforces it.**
    //!
    //! These rows pin facts, not prose — the two spade behaviours the
    //! inertness argument rests on (which are why `planar`'s ordering
    //! warning was never a claim about this lane), the refusal itself,
    //! and the sweep showing nothing this build authors trips it.

    use super::*;
    use crate::sizing::MAX_ANGULAR_STEP;
    use crate::sizing::cap_angular;
    use geom_core::Tol;
    use geom_core::{Affine3, Point2, Vec2, Vec3};
    use profile::{
        Profile, ProfileLoop, RawLoop, SketchPlane, ValidatedProfile, test_support::bulge_loop,
    };
    use sweep::ExtrudeSide;
    use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
    use topo::construction::Construct;

    /// **The CDT's cells keep the containment lemma's premise on every
    /// chart** (module docs, "The lattice metric"): a curved face's cell,
    /// in the plane [`cdt_metric`] puts it in, has aspect under `2√2` on
    /// either axis, the premise of `sizing::sphere_grid_steps`' lemma.
    /// Over sphere zones and caps off the equator, a whole sphere and a
    /// thin torus band, at δ from 1e-1 to 1e-4, through the lane's own
    /// `grid_counts`. The thin band's raw aspect is far outside it.
    ///
    /// It pins [`cdt_metric`], not its call site. That `tessellate_curved`
    /// hands its CDT this metric is witnessed by the bytes: with the call
    /// site reading the trimmed lane's sphere metric instead,
    /// `d9_mesh_goldens`' `sphere_wedge` golden reds and nothing read off
    /// the mesh does, since no sphere face built here carries a boundary
    /// point off its lattice, where the plane would show.
    #[test]
    fn the_cdt_cell_keeps_the_containment_lemmas_aspect_on_every_chart() {
        use core::f64::consts::{FRAC_PI_2, TAU};
        let frame = geom::Frame::new(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let sphere = Surface::Sphere { frame, radius: 1.0 };
        let band = Surface::Torus {
            frame,
            major_radius: 0.3,
            minor_radius: 1e-3,
        };
        let bound = 2.0 * core::f64::consts::SQRT_2;
        let mut raw_outside = 0usize;
        let mut faces = 0usize;
        let windows: Vec<(&Surface<f64>, f64, (f64, f64))> = [
            (0.0, 30.0),
            (20.0, 70.0),
            (50.0, 80.0),
            (60.0, 88.0),
            (70.0, 89.0),
            (-80.0, -50.0),
            (30.0, 90.0),
            (60.0, 90.0),
            (80.0, 90.0),
            (-90.0, 90.0),
        ]
        .into_iter()
        .map(|(lo, hi): (f64, f64)| (&sphere, TAU, (lo.to_radians(), hi.to_radians())))
        .chain([(&band, TAU, (FRAC_PI_2, core::f64::consts::PI))])
        .collect();
        for (surface, uspan, (v0, v1)) in windows {
            let sphere_face = matches!(surface, Surface::Sphere { .. });
            let widest = if v0 <= 0.0 && v1 >= 0.0 {
                1.0
            } else {
                v0.abs().min(v1.abs()).cos()
            };
            let row =
                AnalyticRow::new(surface, if sphere_face { widest } else { f64::NAN }).unwrap();
            let has_pole = sphere_face && (v1 >= FRAC_PI_2 || v0 <= -FRAC_PI_2);
            for delta in [1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 1e-4] {
                let (nu, nv) =
                    grid_counts(&row, delta, (uspan, v1 - v0), has_pole, 2 * 64).unwrap();
                if nv < 2 {
                    continue;
                }
                faces += 1;
                #[allow(clippy::cast_precision_loss)]
                let aspect = (uspan / nu as f64) / ((v1 - v0) / nv as f64);
                let k = cdt_metric(&row, (0.0, uspan), (v0, v1), (nu, nv));
                let cdt = aspect / k;
                raw_outside += usize::from(!(1.0 / bound < aspect && aspect < bound));
                assert!(
                    1.0 / bound < cdt && cdt < bound,
                    "{surface:?} over v [{v0}, {v1}] at δ = {delta}: {nu} × {nv} cells of raw \
                     aspect {aspect} enter the CDT at aspect {cdt} (metric {k}), outside the \
                     lemma's 2√2"
                );
            }
        }
        assert!(faces > 40, "the windows lay rows: {faces}");
        assert!(
            raw_outside > 0,
            "the thin band's raw aspect is past the lemma's bound"
        );
    }

    fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
        Profile::new(SketchPlane::xy(), loops)
            .validate(Tol::witness())
            .unwrap()
    }

    fn axis_y() -> RevolveAxis<f64> {
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        }
    }

    /// **A ruled face with a chord point inside a straight side refuses
    /// here** ([`SIDE_CHORD_POINT`]), on that side's edge. No body puts a
    /// chord point inside a ruling (a spline neighbour's lattice would), so
    /// the call is made by hand, on a cylinder wall whose seam is split:
    /// the route sends that face to the trimmed lane
    /// (`tessellate::route_of`'s `walk::splits_a_straight_side` fact), dropping the doors'
    /// [`Rectangle`] unspent, and the row spends it here, its split vertex
    /// standing in for the chord point. Without the refusal the lane
    /// meshes the face on its ruled arm's `(nu, 1)` grid.
    #[test]
    fn a_ruled_face_with_a_point_inside_a_side_refuses_here() {
        let lp = ProfileLoop::polygon([
            Point2::new(1.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0),
            Point2::new(1.0, 1.0),
        ]);
        let wall = revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body();
        let (placed, _) = split_each_edge_then_place(&wall, &[0.5]);
        let tol = crate::sizing::tests::witness_tols(0.01);
        let mut refused = 0usize;
        for (_, body) in &placed {
            let mut positions = Vec::new();
            let mut vids = HashMap::new();
            for (vk, p) in body.vertex_points() {
                #[allow(clippy::cast_possible_truncation)]
                vids.insert(vk, positions.len() as u32);
                positions.push(p);
            }
            let routes = crate::tessellate::Routes::witness(body);
            let chords = crate::chords::compute_chords(
                body,
                tol.delta,
                &routes,
                &vids,
                &mut positions,
                &mut crate::nurbs_cert::FaceBounds::new(),
            )
            .unwrap();
            for (fk, _) in body.faces() {
                let surface = body.face_surface(fk).unwrap();
                let outer = body.get_face(fk).unwrap().outer;
                let Ok(Shape::Rectangle(rectangle)) =
                    iso_rectangle_shape(body, fk, outer, surface, tol.band)
                else {
                    continue;
                };
                if !crate::walk::splits_a_straight_side(body, fk, surface).unwrap() {
                    continue;
                }
                let answer =
                    tessellate_curved(body, fk, surface, &rectangle, &chords.ids, &positions, &tol);
                assert!(
                    matches!(
                        answer,
                        Err(TessellateError::UnsupportedCurve { edge, note })
                            if note == SIDE_CHORD_POINT
                                && matches!(
                                    body.get_curve_geom(body.get_edge(edge).unwrap().curve)
                                        .and_then(|g| g.certified())
                                        .map(|c| c.carrier()),
                                    Some(geom::Curve3::Line(..))
                                )
                    ),
                    "face {fk:?}: {:?}",
                    answer.map(|p| p.triangles.len())
                );
                refused += 1;
            }
        }
        assert_eq!(refused, 2, "each wall face's split seam");
    }

    /// **A ringed face handed to this lane refuses as the router's
    /// defect.** The pipe of radius 1 united with a bar through one side
    /// of its wall keeps the bar's section as a ring of the wall, which
    /// `tessellate::route_of` sends to the trimmed lane; called here
    /// anyway, the lane refuses rather than mesh the ring's hole as face.
    #[test]
    fn a_ringed_face_here_refuses_as_a_routing_defect() {
        let disc = validated(vec![bulge_loop(vec![
            (Point2::new(-1.0, 0.0), 1.0),
            (Point2::new(1.0, 0.0), 1.0),
        ])]);
        let pipe = extrude(
            &disc,
            Extrusion::Distance {
                depth: 4.0,
                side: ExtrudeSide::Along,
            },
            Tol::witness(),
        )
        .unwrap()
        .body;
        let bar = topo::test_support::finished(
            "the bar",
            sweep::test_support::brick::<f64>((-1.1, 1.1), (0.15, 0.7), (1.6, 2.1), Tol::witness()),
            Tol::witness(),
        );
        let body = topo::union(&pipe, &bar, Tol::witness())
            .unwrap()
            .body()
            .expect("a body")
            .body
            .clone();
        let (fk, surface) = body
            .faces()
            .find(|(_, f)| !f.rings.is_empty())
            .and_then(|(fk, _)| Some((fk, body.face_surface(fk)?)))
            .expect("the wall holds a ring");
        assert!(
            matches!(surface, Surface::Cylinder { .. }),
            "the ringed face is the wall"
        );
        let tol = SizingTols {
            delta: 1e-2,
            eps: crate::sizing::Eps::at(Tol::witness()),
            band: geom_core::Band::linear(Tol::witness()).unwrap(),
        };
        assert!(
            matches!(
                tessellate_curved(
                    &body,
                    fk,
                    surface,
                    // No door ran on a ringed face, so it has no sides.
                    &Rectangle { sides: Vec::new() },
                    &HashMap::new(),
                    &[],
                    &tol
                ),
                Err(TessellateError::MissingEntity { what }) if what == RINGED_FACE_ROUTED_HERE
            ),
            "a ringed face refuses at the curved lane's door"
        );
    }

    /// **A pinched face handed to this lane refuses as the router's
    /// defect.** The block `[−2, 4] × [−2, 2] × [0, 4]` with a full bore of
    /// radius 1 about z, built as the union of its two sides of a slit
    /// along the bore's ruling through `(1, 0)`: the bore's face is one
    /// full-turn wall whose seam is the slit's two edges, its outer loop
    /// passing each end of the slit at two vertices on one point record.
    /// The doors certify it a rectangle, and `tessellate::route_of` sends
    /// it to the trimmed lane on its pinch; called here anyway, the lane
    /// refuses before its walk, which cannot place the two copies of one
    /// point on the slit's two sides.
    #[test]
    fn a_pinched_face_here_refuses_as_a_routing_defect() {
        let tol = Tol::witness();
        let prism = |pts: &[(f64, f64)]| {
            sweep::test_support::finished(
                "a prism",
                sweep::test_support::extruded(
                    SketchPlane::xy_at(Point3::new(0.0, 0.0, -1.0)),
                    vec![ProfileLoop::polygon(
                        pts.iter()
                            .map(|&(x, y)| Point2::new(x, y))
                            .collect::<Vec<_>>(),
                    )],
                    6.0,
                    tol,
                ),
                tol,
            )
        };
        let block = sweep::test_support::finished(
            "the bored block",
            sweep::test_support::extruded(
                SketchPlane::xy(),
                vec![
                    ProfileLoop::polygon([
                        Point2::new(-2.0, -2.0),
                        Point2::new(4.0, -2.0),
                        Point2::new(4.0, 2.0),
                        Point2::new(-2.0, 2.0),
                    ]),
                    bulge_loop(vec![
                        (Point2::new(-1.0, 0.0), 1.0),
                        (Point2::new(1.0, 0.0), 1.0),
                    ]),
                ],
                4.0,
                tol,
            ),
            tol,
        );
        let side = |tool: &[(f64, f64)]| {
            topo::intersect(&block, &prism(tool), tol)
                .unwrap()
                .body()
                .expect("a side")
                .body
                .clone()
        };
        let body = topo::union(
            &side(&[
                (1.0, 0.0),
                (3.0, 1.0),
                (3.0, -5.0),
                (5.0, -5.0),
                (5.0, 5.0),
                (-5.0, 5.0),
                (-5.0, 0.0),
            ]),
            &side(&[
                (1.0, 0.0),
                (-5.0, 0.0),
                (-5.0, -5.0),
                (3.0, -5.0),
                (3.0, -1.0),
            ]),
            tol,
        )
        .unwrap()
        .body()
        .expect("the two sides")
        .body
        .clone()
        .into_body();
        let sizing = crate::sizing::tests::witness_tols(0.05);
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, p) in body.vertex_points() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(p);
        }
        let routes = crate::tessellate::Routes::witness(&body);
        let chords = crate::chords::compute_chords(
            &body,
            sizing.delta,
            &routes,
            &vids,
            &mut positions,
            &mut crate::nurbs_cert::FaceBounds::new(),
        )
        .unwrap();
        let (fk, surface) = body
            .faces()
            .find(|&(fk, _)| matches!(body.face_surface(fk), Some(Surface::Cylinder { .. })))
            .and_then(|(fk, _)| Some((fk, body.face_surface(fk)?)))
            .expect("the bore");
        let outer = body.get_face(fk).unwrap().outer;
        assert!(
            crate::walk::pinches(&body, outer).unwrap(),
            "the bore's loop passes the slit's ends twice"
        );
        let Ok(Shape::Rectangle(rectangle)) =
            iso_rectangle_shape(&body, fk, outer, surface, sizing.band)
        else {
            panic!("the doors certify the full-turn wall a rectangle");
        };
        let answer = tessellate_curved(
            &body,
            fk,
            surface,
            &rectangle,
            &chords.ids,
            &positions,
            &sizing,
        );
        assert!(
            matches!(
                answer,
                Err(TessellateError::MissingEntity { what }) if what == PINCHED_FACE_ROUTED_HERE
            ),
            "a pinched face refuses at the curved lane's door: {:?}",
            answer.map(|p| p.triangles.len())
        );
    }

    /// The UV bounding box of a walk polygon — the same `(u0, u1, v0,
    /// v1)` [`tessellate_curved`] derives, so a row that feeds a
    /// synthetic polygon to [`require_swept_rectangle`] checks it
    /// against the box the real grid would span.
    fn bbox(poly: &[UvPoint]) -> (f64, f64, f64, f64) {
        let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for e in poly {
            u0 = u0.min(e.u);
            u1 = u1.max(e.u);
            v0 = v0.min(e.v);
            v1 = v1.max(e.v);
        }
        (u0, u1, v0, v1)
    }

    /// The run's kernel ε — the length this lane's band measures
    /// against, read from the same place `tessellate` reads it.
    fn eps() -> Eps {
        Eps::at(Tol::witness())
    }

    /// Unit lever arms for a synthetic polygon, so its UV units ARE
    /// metres and the band's margin can be read straight off the
    /// fixture's coordinates.
    fn unit_levers(n: usize) -> Vec<(f64, f64)> {
        vec![(1.0, 1.0); n]
    }

    fn uv(poly: &[(f64, f64)]) -> Vec<UvPoint> {
        poly.iter()
            .enumerate()
            .map(|(i, &(u, v))| UvPoint {
                u,
                v,
                #[allow(clippy::cast_possible_truncation)]
                id: i as u32,
                pole: false,
            })
            .collect()
    }

    /// One face's walk as the rows below consume it: the face, its UV
    /// polygon, and the per-entry lever arms `tessellate_curved` builds.
    type Walked = (FaceKey, Vec<UvPoint>, Vec<(f64, f64)>);

    /// A walk's polygon and its lever arms, without the face.
    type WalkOf = (Vec<UvPoint>, Vec<(f64, f64)>);

    /// Every face this lane would take, walked to its UV polygon — the
    /// `tessellate` prologue (mesh ids, the route, then the chord pass)
    /// run for the walk alone. A face the route hands this lane is a face
    /// `tessellate_curved` receives.
    ///
    /// `name` is the fixture's, carried into the failure message: a
    /// face that does not walk says which body it belongs to.
    fn curved_walks(name: &str, body: &Body<f64>) -> Vec<Walked> {
        curved_walk_results(body)
            .into_iter()
            .map(|(fk, walk)| {
                let (poly, levers) =
                    walk.unwrap_or_else(|e| panic!("{name}: face {fk:?} does not walk: {e:?}"));
                (fk, poly, levers)
            })
            .collect()
    }

    /// [`curved_walks`] with each face's walk kept as the walk's own
    /// answer, for a row whose subject is a walk that refuses.
    fn curved_walk_results(body: &Body<f64>) -> Vec<(FaceKey, Result<WalkOf, TessellateError>)> {
        walk_results(body, crate::tessellate::Routes::witness(body))
    }

    /// The walk of every face `routes` hands this lane, on the chord pass
    /// a run on those routes would make.
    fn walk_results(
        body: &Body<f64>,
        routes: crate::tessellate::Routes,
    ) -> Vec<(FaceKey, Result<WalkOf, TessellateError>)> {
        let eps = Eps::at(Tol::witness());
        let band = Band::linear(Tol::witness()).unwrap();
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, p) in body.vertex_points() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(p);
        }
        let chords = crate::chords::compute_chords(
            body,
            0.025,
            &routes,
            &vids,
            &mut positions,
            &mut crate::nurbs_cert::FaceBounds::new(),
        )
        .unwrap()
        .ids;
        let mut out = Vec::new();
        for (fk, face) in body.faces() {
            if routes.lane(fk).unwrap()
                != crate::tessellate::Routed::To(crate::tessellate::Lane::Curved)
            {
                continue;
            }
            let Some(chart) = Chart::of(body.face_surface(fk).unwrap()) else {
                panic!("face {fk:?} has no iso chart to walk")
            };
            let rectangle = routes
                .rectangle(fk)
                .expect("a curved-lane face has its doors' answer");
            let walk = loop_polygon(
                body, &chart, rectangle, &chords, &positions, fk, face.outer, eps, band,
            )
            .map(|poly| {
                // The same lever arms `tessellate_curved` builds.
                let levers = poly
                    .iter()
                    .map(|e| (chart.radial(positions[e.id as usize]), chart.v_lever()))
                    .collect();
                (poly, levers)
            });
            out.push((fk, walk));
        }
        out
    }

    fn ball() -> Body<f64> {
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -1.0), 1.0),
            (Point2::new(0.0, 1.0), 0.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    /// A partial-revolve sphere band — the pole-to-pole wedge whose
    /// walk is hardest (`revolves.rs`'s `survives_sphere_wedges_...`
    /// shape), absent from the sweep until the review asked for it.
    fn sphere_band(theta: f64) -> Body<f64> {
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -1.0), 1.0),
            (Point2::new(0.0, 1.0), 0.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Partial(theta),
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    fn cone_body() -> Body<f64> {
        let lp = ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, 1.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    fn washer() -> Body<f64> {
        let lp = ProfileLoop::polygon([
            Point2::new(1.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0),
            Point2::new(1.0, 1.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    fn donut() -> Body<f64> {
        let lp = bulge_loop(vec![
            (Point2::new(2.0, -0.5), 1.0),
            (Point2::new(2.0, 0.5), 1.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    fn wedge(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([
            Point2::new(1.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0),
            Point2::new(1.0, 1.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Partial(theta),
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    /// Axis-touching partial wedge (`tests/common::axis_wedge`): the
    /// axis edge is an ordinary boundary edge shared by the two caps,
    /// which is one of the two shapes an earlier review lane built
    /// *because the walk is hardest there*.
    fn axis_wedge(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(0.0, 1.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Partial(theta),
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    /// The mirror-nappe diamond (`review_m2_pr6_walk_shapes.rs`'s
    /// `diamond_profile`): a downward-opening cone under a partial
    /// revolve, so the nappe walls carry junction u/v assignments —
    /// the other hardest-walk shape.
    fn mirror_nappe(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([
            Point2::new(1.0, 0.0),
            Point2::new(2.0, 1.0),
            Point2::new(1.0, 2.0),
        ]);
        revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Partial(theta),
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    fn rounded_prism() -> Body<f64> {
        let b = core::f64::consts::FRAC_PI_8.tan();
        let r = 0.5;
        let v = |pos: Point2<f64>, bulge: f64| (pos, bulge);
        let lp = bulge_loop(vec![
            v(Point2::new(r, 0.0), 0.0),
            v(Point2::new(2.0 - r, 0.0), b),
            v(Point2::new(2.0, r), 0.0),
            v(Point2::new(2.0, 2.0 - r), b),
            v(Point2::new(2.0 - r, 2.0), 0.0),
            v(Point2::new(r, 2.0), b),
            v(Point2::new(0.0, 2.0 - r), 0.0),
            v(Point2::new(0.0, r), b),
        ]);
        extrude(
            &validated(vec![lp]),
            Extrusion::Distance {
                depth: 1.0,
                side: ExtrudeSide::Along,
            },
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body()
    }

    /// The die pip (`sweep`'s `m5_s13_pips` shape): a 4 × 4 × 1 slab
    /// with a radius-0.5 ball subtracted 0.2 above its top face. The
    /// cavity's two sphere faces are the only curved faces in this
    /// crate's reach that a BOOLEAN produced rather than a sweep.
    fn die_pip() -> Body<f64> {
        let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(4.0, 4.0),
            Point2::new(0.0, 4.0),
        ]);
        let slab = extrude(
            &validated(vec![lp]),
            Extrusion::Distance {
                depth: 1.0,
                side: ExtrudeSide::Along,
            },
            Tol::witness(),
        )
        .unwrap()
        .body;
        let half = bulge_loop(vec![
            (Point2::new(0.0, -0.5), 1.0),
            (Point2::new(0.0, 0.5), 0.0),
        ]);
        let ball = revolve(
            &validated(vec![half]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body;
        let ball = topo::transform_rigid(
            &ball,
            &geom_core::RigidMotion::translation(Vec3::new(2.0, 2.0, 1.2)).unwrap(),
        )
        .unwrap();
        let slab = topo::test_support::finished("the die slab", slab, Tol::witness());
        let ball = topo::test_support::finished("the die ball", ball, Tol::witness());
        let cut = topo::boolean::subtract(&slab, &ball, Tol::witness()).expect("the die pip cuts");
        (*cut.body().expect("a pip is a dent, not a void").body).clone()
    }

    fn fixtures() -> Vec<(&'static str, Body<f64>)> {
        let pi = core::f64::consts::PI;
        vec![
            ("ball", ball()),
            ("sphere band (pi/2)", sphere_band(pi / 2.0)),
            (
                "sphere band (2pi - 0.08)",
                sphere_band(core::f64::consts::TAU - 0.08),
            ),
            ("cone", cone_body()),
            ("mirror nappe (pi + 0.05)", mirror_nappe(pi + 0.05)),
            ("washer", washer()),
            ("donut", donut()),
            ("wedge(pi/2)", wedge(pi / 2.0)),
            ("wedge(pi)", wedge(pi)),
            ("wedge(2pi - 0.05)", wedge(core::f64::consts::TAU - 0.05)),
            ("axis wedge(pi/2)", axis_wedge(pi / 2.0)),
            ("axis wedge(pi + 0.5)", axis_wedge(pi + 0.5)),
            ("rounded prism", rounded_prism()),
            ("die pip (boolean cut)", die_pip()),
        ]
    }

    /// **The ball face beside a slit meshes as the whole sphere.** The ball
    /// with one meridian cut at two points, a second arc stated on that
    /// meridian between them (the slit face), and the other meridian killed:
    /// the ball face's loop runs pole to pole and back with the slit's two
    /// arcs in its chart image, so it is no wrap strip and its columns are
    /// its lift's. The lift turns the face a whole turn round each pole, so
    /// the slit's two arcs stand a turn apart and the patch encloses the unit
    /// ball, the volume only the whole-turn region holds. The body's other
    /// half, the slit face, refuses (`the_one_face_ball.rs`, the slit row).
    #[test]
    fn the_ball_face_beside_a_slit_meshes_as_the_whole_sphere() {
        use core::f64::consts::PI;
        use topo::construction::MefSite;
        let tol = Tol::witness();
        let mut body = ball();
        let north = Point3::new(0.0, 1.0, 0.0);
        let at = |body: &Body<f64>, v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let pole = body
            .vertices()
            .find(|&(v, _)| (at(&body, v) - north).norm() < 1e-12)
            .map(|(v, _)| v)
            .unwrap();
        let out: Vec<_> = body
            .half_edges()
            .filter(|(_, h)| h.start == pole)
            .map(|(he, _)| he)
            .collect();
        let cut = body.get_half_edge(out[0]).unwrap().edge;
        let p = body.split_edge(cut, 1.0, tol).unwrap();
        let q = body.split_edge(p.new_edge, 2.0, tol).unwrap();
        let (pv, qv) = (
            at(&body, p.vertex) - Point3::origin(),
            at(&body, q.vertex) - Point3::origin(),
        );
        let normal = pv.cross(qv) * (1.0 / pv.cross(qv).norm());
        let meridian = geom::Curve3::circle(Point3::origin(), normal, 1.0, pv);
        let pq = body.get_edge(p.new_edge).unwrap();
        let from_p = [pq.he_plus, pq.he_minus]
            .into_iter()
            .find(|&he| body.get_half_edge(he).unwrap().start == p.vertex)
            .unwrap();
        let to_q = body.get_half_edge(from_p).unwrap().next;
        body.mef(
            MefSite::Chords {
                he1: from_p,
                he2: to_q,
            },
            geom_brep::EdgeCurveSpec::arc_of_circle(meridian, 0.0, pv.dot(qv).acos()).unwrap(),
            topo::construction::FaceSurface::Inherit,
            tol,
        )
        .unwrap();
        body.kef_describing(out[1], &[], tol).unwrap();
        let fk = body.face_of_half_edge(out[0]).unwrap();

        let sizing = crate::sizing::tests::witness_tols(0.05);
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, p) in body.vertex_points() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(p);
        }
        let routes = crate::tessellate::Routes::witness(&body);
        let chords = crate::chords::compute_chords(
            &body,
            sizing.delta,
            &routes,
            &vids,
            &mut positions,
            &mut crate::nurbs_cert::FaceBounds::new(),
        )
        .unwrap();
        let surface = body.face_surface(fk).unwrap();
        let outer = body.get_face(fk).unwrap().outer;
        let Ok(Shape::Rectangle(rectangle)) =
            iso_rectangle_shape(&body, fk, outer, surface, sizing.band)
        else {
            panic!("the ball face is routed to the curved lane");
        };
        let patch = tessellate_curved(
            &body,
            fk,
            surface,
            &rectangle,
            &chords.ids,
            &positions,
            &sizing,
        )
        .expect("the ball face meshes");
        let triangles = patch.place(&mut positions);
        let six_v: f64 = triangles
            .iter()
            .map(|t| {
                let [a, b, c] = t.map(|i| positions[i as usize] - Point3::origin());
                a.dot(b.cross(c))
            })
            .sum();
        let (volume, area) = (4.0 * PI / 3.0, 4.0 * PI);
        assert!(
            (six_v / 6.0 - volume).abs() <= sizing.delta * area,
            "the ball face encloses the unit ball: {} against {volume}",
            six_v / 6.0
        );
    }

    /// A fixture's volume and area, measured by props on the body as built
    /// (a revolve, which tier 3 is not asked to pass here).
    fn enclosed(body: &Body<f64>) -> (f64, f64) {
        // Raw door: a revolve as built here, with no tier-3 pass asked of it;
        // the rows read its volume and area, not its validity.
        let m = topo::test_support::measure_unvalidated(
            body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("the fixture measures")
        .enclosure;
        (m.volume, m.surface_area)
    }

    /// **A split band wider than a half turn meshes its own volume.** A
    /// rimless pole-to-pole band revolved most of the way round, one meridian
    /// split near one pole and the other near the other, unplaced: its columns are the lift's, the
    /// face's own branches. Read off the loop's area direction, the split
    /// band took its complement sliver at 2π − 1e-3, which meshes watertight
    /// and encloses a negative volume. Sphere bands, a lemon's and an
    /// apple's spindle sheet, and each band reverted to its void, at two δ:
    /// at the coarse one `δ·area` is loose, at the fine one it is not.
    #[test]
    fn a_split_band_wider_than_a_half_turn_meshes_its_own_volume() {
        use core::f64::consts::TAU;
        let revolved = |bulge: f64, theta: f64| {
            revolve(
                &validated(vec![bulge_loop(vec![
                    (Point2::new(0.0, -1.0), bulge),
                    (Point2::new(0.0, 1.0), 0.0),
                ])]),
                axis_y(),
                Revolution::Partial(theta),
                Tol::witness(),
            )
            .unwrap()
            .body
            .into_body()
        };
        let theta = TAU - 1e-3;
        for (name, body) in [
            ("sphere band", sphere_band(theta)),
            ("lemon band", revolved(0.5, theta)),
            ("apple band", revolved(1.6, theta)),
        ] {
            let (volume, area) = enclosed(&body);
            // The band's two meridians; the third edge is the axis, the caps'.
            let meridians: Vec<EdgeKey> = body
                .edges()
                .map(|(ek, _)| ek)
                .filter(|&ek| {
                    let carrier = body
                        .get_edge(ek)
                        .and_then(|e| body.get_curve_geom(e.curve))
                        .and_then(|g| g.certified())
                        .map(|c| c.carrier());
                    !matches!(carrier, Some(geom::Curve3::Line(..)))
                })
                .collect();
            assert_eq!(
                meridians.len(),
                2,
                "{name}: a rimless band is two meridians"
            );
            // One meridian split near each end of the band.
            for (&ek, frac) in meridians.iter().zip([0.1, 0.9]) {
                let mut split = body.clone();
                let (t0, t1) = split
                    .get_edge(ek)
                    .and_then(|e| split.get_curve_geom(e.curve))
                    .and_then(|g| g.certified())
                    .unwrap()
                    .params();
                split
                    .split_edge(ek, t0 + (t1 - t0) * frac, Tol::witness())
                    .unwrap_or_else(|e| panic!("{name} {ek:?}@{frac}: split: {e:?}"));
                let void = split.revert().unwrap();
                for (what, solid, sign) in [("solid", &split, 1.0), ("void", &void, -1.0)] {
                    for delta in [0.1, 0.02] {
                        let mesh =
                            crate::tessellate(solid, delta, Tol::witness()).unwrap_or_else(|e| {
                                panic!("{name} {what} {ek:?}@{frac} δ {delta}: {e:?}")
                            });
                        assert_eq!(
                            crate::validate::check_volume(&mesh, sign * volume, area, delta),
                            Ok(()),
                            "{name} {what} {ek:?}@{frac} δ {delta}: its own volume"
                        );
                    }
                }
            }
        }
    }

    /// **A band wider than a half turn meshes its own volume wherever it is
    /// placed** (the split row above, unplaced, carries the class): the same
    /// bands at θ = 2π − 0.08, 2π − 0.02 and 3π/2, every edge split and the
    /// body rigidly placed, checked against props.
    #[test]
    fn a_placed_band_wider_than_a_half_turn_meshes_its_own_volume() {
        use core::f64::consts::TAU;
        for theta in [TAU - 0.08, TAU - 0.02, 1.5 * core::f64::consts::PI] {
            let body = sphere_band(theta);
            let (volume, area) = enclosed(&body);
            for fracs in [&[0.5][..], &[0.312_9, 0.156_45][..]] {
                let (placed, skipped) = split_each_edge_then_place(&body, fracs);
                assert!(skipped.is_empty(), "θ {theta} @{fracs:?}: {skipped:?}");
                for (i, placed) in placed {
                    let mesh =
                        crate::tessellate(&placed, 0.1, Tol::witness()).unwrap_or_else(|e| {
                            panic!("θ {theta} edge {i} @{fracs:?}: the band meshes: {e:?}")
                        });
                    assert_eq!(
                        crate::validate::check_volume(&mesh, volume, area, 0.1),
                        Ok(()),
                        "θ {theta} edge {i} @{fracs:?}: the placed band encloses its own volume"
                    );
                }
            }
        }
    }

    /// Every edge of `body` split at `fracs` of its own parameter
    /// range, then the whole body placed by an oblique rigid map —
    /// #653's two ingredients, applied to a fixture wholesale.
    ///
    /// One body per (edge, fracs) pair, because splitting every edge of
    /// one body at once would also change which faces are adjacent to
    /// which and stop being a controlled comparison.
    ///
    /// Returns `(placed bodies, skipped edges)`. **The skips are
    /// returned rather than `continue`d silently**: an edge that drops
    /// out here is an edge the row above never covers, and a helper
    /// that swallows them lets a fixture stop contributing without any
    /// row going red. The caller asserts the skip list is empty.
    fn split_each_edge_then_place(
        body: &Body<f64>,
        fracs: &[f64],
    ) -> (Vec<(usize, Body<f64>)>, Vec<String>) {
        // Deliberately irrational-ish (see `split_and_placed_frustum_wedge`):
        // an axis-aligned or dyadic placement keeps the sub-edge
        // azimuths bitwise equal and the rows below go green for the
        // wrong reason.
        let irr = Vec3::new(0.317_8_f64, 0.941_2, -0.110_9);
        let irr = irr * (1.0 / irr.norm());
        let placement = Affine3::from_parts(
            Affine3::rotation_about_axis(Point3::origin(), irr, 1.0 / 3.0).linear,
            Vec3::new(0.117, -0.339_1, 5.001_7),
        );
        let (mut out, mut skipped) = (Vec::new(), Vec::new());
        for (i, ek) in body.edges().map(|(k, _)| k).enumerate() {
            let Some(curve) = body
                .get_edge(ek)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
            else {
                skipped.push(format!("edge {i}: no certified curve geometry"));
                continue;
            };
            let (t0, t1) = curve.params();
            let mut b = body.clone();
            let mut failed = None;
            for &f in fracs {
                // Not `ok &= …`: the second split of a failed first is
                // meaningless, and the first failure is the reportable
                // one.
                if let Err(e) = b.split_edge(ek, t0 + (t1 - t0) * f, Tol::witness()) {
                    failed = Some(format!("edge {i} @{f}: split_edge {e:?}"));
                    break;
                }
            }
            if let Some(why) = failed {
                skipped.push(why);
                continue;
            }
            match topo::transform_rigid(
                &b,
                &geom_core::test_support::rigid(&placement, Tol::witness()),
            ) {
                Ok(placed) => out.push((i, placed)),
                Err(e) => skipped.push(format!("edge {i}: transform_rigid {e:?}")),
            }
        }
        (out, skipped)
    }

    /// The largest distance, in metres, from any walk entry to its own
    /// face's UV bounding box, over every curved face of `body`.
    ///
    /// **It calls [`entries_off_bbox`] with `eps = 0.0` rather than
    /// re-deriving the distance.** `gap_is_noise` is a strict `<`, so a
    /// zero band admits nothing and every entry comes back carrying the
    /// distance production measures, in production's own per-axis
    /// spelling. There is one spelling of this metric in the crate and
    /// it is the one that ships; a hand transcription in a test goes
    /// stale the moment the kernel's changes.
    ///
    /// **So this is the METRIC, not the guard**, and the difference is
    /// the band: at `eps = 0.0` the admit test is uniformly false, so
    /// no entry here takes the branch a real tessellation takes for
    /// all of them. The banded verdict on the same bodies is the
    /// caller's `UnsupportedCurvedDomain` arm.
    ///
    /// The degenerate-axis rule does show through: a pole entry
    /// reports its v gap here rather than the 0 m its vanished u lever
    /// would have made of any u — which is what lets this metric see a
    /// pole entry off its box at all.
    fn worst_entry_off_box(name: &str, body: &Body<f64>) -> f64 {
        let mut worst: f64 = 0.0;
        for (_, poly, levers) in curved_walks(name, body) {
            for (_, d) in entries_off_bbox(&poly, &levers, bbox(&poly), Eps::exactly(0.0)) {
                worst = worst.max(d);
            }
        }
        worst
    }

    /// **THE #653 ROW.** An iso side carried by several edges is
    /// bitwise straight, and the mesh over it is watertight — for
    /// every chart class, under an oblique placement, with every edge
    /// of every fixture split in turn.
    ///
    /// **The assertion is `== 0.0`, and that is the point.** Before
    /// this fix each sub-edge derived its own constant coordinate from
    /// its own `mid_azimuth` point, so the side's entries landed
    /// ULPS apart and the polygon was not its own bounding rectangle
    /// to the last bit. Under an oblique placement that produced two
    /// consequences: the exact form of [`entries_off_bbox`] refused
    /// valid parts (the row above), and — the live defect — the CDT
    /// saw a sliver-generating wobble and emitted a **silently
    /// non-watertight** mesh, `NonManifoldEdge` under
    /// [`crate::validate::check_mesh`] while `tessellate` returned
    /// `Ok`. A banded margin cannot separate those from the correct
    /// case (measured: the two populations interleave at ~1e-16 m), so
    /// the columns are fixed at the source instead and the residue is
    /// **identically zero**, asserted here rather than inferred.
    ///
    /// Reverting `walk::iso_side_starts` to `vec![true; m]` turns both
    /// halves red.
    ///
    /// # The floor is PER FIXTURE, and derived, not pinned
    ///
    /// `every_curved_walk_is_its_own_bounding_rectangle` two hundred
    /// lines down argues the case: a global floor lets a fixture
    /// that stopped contributing hide behind its siblings. So this row
    /// asserts, per fixture and per split pattern, that the number of
    /// placed bodies EQUALS the fixture's edge count — every edge
    /// participates — and that `split_each_edge_then_place` skipped
    /// nothing. That floor derives itself from the fixture list, so
    /// adding a fixture cannot silently shrink the sweep, which a
    /// transcribed total cannot promise.
    ///
    /// The totals are still REPORTED (in the failure message and by
    /// the two constants below) because a reader wants the scale; they
    /// are not what the row rests on.
    ///
    /// # What this row cannot see — stated, because an unstated blind
    /// spot reads as a verified negative
    ///
    /// - **Trim-carrier faces.** `curved_walks` skips them exactly as
    ///   the router does, and `crate::trimmed` runs its own walk. An
    ///   iso side carried by several edges on a TRIMMED face is not
    ///   covered here or anywhere.
    /// - **Bodies `revolve` refuses at construction** — horn and
    ///   spindle tori, and every profile that fails its checks. The
    ///   chart classes below are the constructible ones, which is not
    ///   the same as all of them.
    /// - **Rim runs from DISTINCT carrier circles.** This is the real
    ///   gap. `split_edge` keeps ONE carrier, so the two sub-edges of a
    ///   split rim read the same centre and radius and agree bitwise
    ///   even WITHOUT the fix: the Rim arm of `iso_side_starts` is
    ///   executed on every rim split here and can never differ. So the
    ///   swept sibling ships with no red-when-reverted evidence — only
    ///   the meridian half has any. What would mint it is two
    ///   independently stated co-`v` `CIRCLE`s, which is a STEP
    ///   authoring job (the fixture generator in
    ///   `crates/step-import/tests/fixtures/split-iso/` is the obvious
    ///   place). Owed work, not covered here.
    #[test]
    fn a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight() {
        // Single and double splits: two sub-edges and three, the second
        // being what `split_and_placed_frustum_wedge` mints.
        const PATTERNS: [&[f64]; 2] = [&[0.5], &[0.312_9, 0.156_45]];
        let (mut checked, mut refused) = (0usize, 0usize);
        let (mut crooked, mut dirty): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
        for (name, body) in fixtures() {
            let edges = body.edges().count();
            let mut here = 0usize;
            // A split and a rigid placement move neither the volume nor the
            // area, so the fixture's own measure is every placed body's.
            let props = enclosed(&body);
            for fracs in PATTERNS {
                let (placed, skipped) = split_each_edge_then_place(&body, fracs);
                assert!(
                    skipped.is_empty(),
                    "{name} @{fracs:?}: the sweep dropped edges silently — {skipped:?}"
                );
                assert_eq!(
                    placed.len(),
                    edges,
                    "{name} @{fracs:?}: every one of the fixture's {edges} edges must \
                     produce a split, placed body"
                );
                for (i, placed) in placed {
                    let worst = worst_entry_off_box(name, &placed);
                    if worst != 0.0 {
                        crooked.push(format!("{name} edge {i} @{fracs:?}: {worst} m"));
                    }
                    // A typed refusal is not a counterexample: a few of
                    // these exceed the chord certificate at this δ and
                    // refuse identically before and after. What must
                    // never happen is `Ok` plus a dirty mesh — the #653
                    // defect's signature.
                    match crate::tessellate(&placed, 0.1, Tol::witness()) {
                        Ok(mesh) => {
                            checked += 1;
                            here += 1;
                            if let Err(e) = crate::validate::check_mesh(&mesh) {
                                dirty.push(format!("{name} edge {i} @{fracs:?}: {e:?}"));
                            }
                            // Watertight is not enough: a face meshed as its
                            // complement closes too.
                            if let Err(e) =
                                crate::validate::check_volume(&mesh, props.0, props.1, 0.1)
                            {
                                dirty.push(format!("{name} edge {i} @{fracs:?}: volume {e:?}"));
                            }
                        }
                        // The one refusal that IS a counterexample:
                        // the domain guard running at the run's own ε.
                        // `worst_entry_off_box` measured this same
                        // population at `eps = 0.0`, where the admit
                        // test is uniformly false and no entry takes
                        // the branch a real tessellation takes for all
                        // of them; this arm is where the banded form
                        // gets its verdict on the same bodies, for
                        // free.
                        Err(TessellateError::UnsupportedCurvedDomain {
                            off_bbox,
                            first_uv,
                            max_distance,
                            ..
                        }) => crooked.push(format!(
                            "{name} edge {i} @{fracs:?}: the banded guard refused it — \
                             {off_bbox} entries off the box, first at {first_uv:?}, \
                             worst {max_distance} m"
                        )),
                        Err(_) => refused += 1,
                    }
                }
            }
            assert!(
                here > 0,
                "{name} contributed no meshed configuration at all — it is in the \
                 fixture list but exercises nothing"
            );
        }
        // Both halves reported together: reverting the fix makes the
        // first list long and the second non-empty, and a reader of the
        // failure should see the whole population, not its first member.
        assert!(
            crooked.is_empty() && dirty.is_empty(),
            "of {checked} meshed configurations ({refused} refused typed): \
             {} have a walk entry off their own UV box {crooked:?}; \
             {} meshed non-watertight or off their volume {dirty:?}",
            crooked.len(),
            dirty.len()
        );
        // Reported, not rested on: the per-fixture floors above are
        // what fail when the sweep shrinks. This line exists so the
        // scale is visible without running the row.
        assert_eq!(
            (checked, refused),
            (TOTAL_MESHED, TOTAL_REFUSED),
            "the sweep's totals moved; if a fixture was added or removed this is the \
             one line to update, and the per-fixture floors above are the guarantee"
        );
    }

    /// The #653 row's totals, measured. They are asserted so that a
    /// change in the fixture list is VISIBLE rather than silent — the
    /// row's actual guarantee is its per-fixture floor, not these.
    const TOTAL_MESHED: usize = 250;
    /// Typed refusals in the same sweep: none. A split ruling sends its
    /// face to the trimmed lane (`walk::splits_a_straight_side`), which
    /// meshes the mirror nappe's split walls. The donut contributes none
    /// either — a split
    /// SEAM MERIDIAN is folded back into the meridian it carries by
    /// props' torus parse (identity through the split lineage, never
    /// through the stored circle's values), so the shape door answers
    /// for the pieces what it answers for the edge;
    /// `tests/iso_rectangle_door.rs` pins that body to every consumer,
    /// and a split RIM was always fine. Reach of a split meridian in
    /// production: `split_edge` on a torus seam directly, or a blend
    /// whose surgery splits one (`sweep::blend::surgery` splits seam
    /// and meridian edges; whether one lands on a torus is
    /// unmeasured).
    const TOTAL_REFUSED: usize = 0;

    /// **The premise, swept.** Every curved face this build can put in
    /// front of [`tessellate_curved`] walks to a UV polygon that IS its
    /// own bounding rectangle, so [`require_swept_rectangle`] admits it
    /// — i.e. the guard this lane now carries refuses nothing in tree.
    ///
    /// Participation is asserted **per fixture**, not as a global
    /// count: `curved_walks` reaches its body through a silent
    /// `continue` (the route), so a
    /// global floor lets a fixture that stopped contributing hide
    /// behind its siblings' faces. The die pip is the one this matters
    /// most for: the boolean's
    /// chart re-cut is what keeps a CUT sphere face iso-rectangular,
    /// and if a future re-cut gave the cavity rim an ellipse carrier it
    /// would drop out of the sweep silently.
    #[test]
    fn every_curved_walk_is_its_own_bounding_rectangle() {
        for (name, body) in fixtures() {
            let walks = curved_walks(name, &body);
            assert!(
                !walks.is_empty(),
                "{name} contributes no curved walk — it is no longer sweeping this lane"
            );
            for (fk, poly, levers) in walks {
                // Asserted through the production refusal, with the
                // predicate consulted only for the diagnostic.
                assert_eq!(
                    require_swept_rectangle(fk, &poly, &levers, bbox(&poly), eps()),
                    Ok(()),
                    "{name} face {fk:?}: of {} walk entries, these lie strictly inside \
                     the UV bounding box (uv, metres off): {:?}",
                    poly.len(),
                    entries_off_bbox(&poly, &levers, bbox(&poly), eps())
                        .iter()
                        .map(|&(i, d)| ((poly[i].u, poly[i].v), d))
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    /// The whole pipeline agrees: no fixture refuses. A face-level
    /// walk check could pass while `tessellate` failed for an unrelated
    /// reason, and the guard is new code on the hot path — this row is
    /// the in-tree regression evidence itself.
    #[test]
    fn no_fixture_refuses_through_the_public_entry_point() {
        for (name, body) in fixtures() {
            for delta in [0.25, 0.04] {
                let mesh = crate::tessellate(&body, delta, Tol::witness())
                    .unwrap_or_else(|e| panic!("{name} at delta={delta} refused: {e:?}"));
                crate::validate::check_mesh(&mesh)
                    .unwrap_or_else(|e| panic!("{name} at delta={delta} meshed dirty: {e:?}"));
            }
        }
    }

    /// The U-shaped domain the red rows use: `[0, 4] × [0, 4]` with
    /// `[1, 3] × [2, 4]` bitten out of the top — every side iso, so
    /// nothing upstream of this lane would reroute it.
    fn notched_domain() -> Vec<(f64, f64)> {
        vec![
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 4.0),
            (3.0, 4.0),
            (3.0, 2.0),
            (1.0, 2.0),
            (1.0, 4.0),
            (0.0, 4.0),
        ]
    }

    fn rectangle_domain() -> Vec<(f64, f64)> {
        vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
    }

    /// **The spatial check's own refusal.** A notched polygon is refused
    /// TYPED, naming both re-entrant corners, where they are, and how
    /// far off the box they sit; the swept rectangle passes. Asserting
    /// the production refusal rather than a predicate's return value is
    /// what makes this a guarantee about the check instead of about a
    /// helper.
    ///
    /// The fixture is a synthetic polygon on purpose: a real notched
    /// body never reaches this check any more — the shape door decides it
    /// is no rectangle on rim structure first, and the route sends it
    /// elsewhere (`tests/iso_rectangle_door.rs`, the keyway) — so this row is the walk-consistency question asked in
    /// isolation, which is the only way to ask it.
    #[test]
    fn a_notched_domain_is_refused_typed() {
        let fk = fixtures()
            .into_iter()
            .next()
            .and_then(|(_, b)| b.faces().next().map(|(fk, _)| fk))
            .unwrap();
        let rect = uv(&rectangle_domain());
        assert_eq!(
            require_swept_rectangle(fk, &rect, &unit_levers(rect.len()), bbox(&rect), eps()),
            Ok(())
        );
        let notch = uv(&notched_domain());
        assert_eq!(
            require_swept_rectangle(fk, &notch, &unit_levers(notch.len()), bbox(&notch), eps()),
            Err(TessellateError::UnsupportedCurvedDomain {
                face: fk,
                off_bbox: 2,
                first_uv: (3.0, 2.0),
                max_distance: 1.0,
            }),
            "the two re-entrant corners are the entries strictly inside the box"
        );
    }

    /// **A zero-height box passes this check**, and a zero-width one
    /// does too: every entry of a polygon on one `v` (or one `u`) lies
    /// on its own degenerate bounding box at distance zero, which the
    /// run's band admits, so [`entries_off_bbox`] finds nothing. That is
    /// why walk consistency cannot stand in for the questions asked
    /// before it: the polygon a loop of rims only would walk to is this
    /// one, and what refuses that loop is `walk::require_a_meridian`,
    /// not this check. Synthetic, like its neighbours: no walk this
    /// build produces has zero height.
    #[test]
    fn a_zero_height_box_passes_the_walk_consistency_check() {
        let fk = fixtures()
            .into_iter()
            .next()
            .and_then(|(_, b)| b.faces().next().map(|(fk, _)| fk))
            .unwrap();
        for (name, flat) in [
            (
                "zero height",
                uv(&[(0.0, 0.5), (1.0, 0.5), (2.0, 0.5), (3.0, 0.5)]),
            ),
            (
                "zero width",
                uv(&[(0.5, 0.0), (0.5, 1.0), (0.5, 2.0), (0.5, 3.0)]),
            ),
        ] {
            let b = bbox(&flat);
            assert!(
                b.0.to_bits() == b.1.to_bits() || b.2.to_bits() == b.3.to_bits(),
                "{name}: fixture precondition"
            );
            assert_eq!(
                require_swept_rectangle(fk, &flat, &unit_levers(flat.len()), b, eps()),
                Ok(()),
                "{name}"
            );
        }
    }

    /// **THE BAND'S WITNESS.** A sub-ε off-box entry must be ADMITTED,
    /// and the exact comparison this replaced must refuse the same
    /// entry — passed through [`entries_off_bbox`] itself, not
    /// compared as two literals.
    ///
    /// This row exists because #653 removed the band's live evidence.
    /// Every walk this build produces now sits on its box bitwise, so
    /// the banded and exact forms agree on every real fixture and
    /// `a_split_then_placed_swept_face_is_not_refused` — #648's own
    /// regression row — no longer discriminates between them. A guard
    /// with no red-when-reverted row is a guard nobody is testing, so
    /// the witness is synthetic and says so: the fixture is one
    /// rectangle vertex nudged strictly inside the box by a few ulps of
    /// its own coordinate, which is the exact shape a multiply-carried
    /// iso side produced before #653.
    ///
    /// The nudge is derived from the tree (`next_up` on the fixture's
    /// own coordinate), not transcribed from a sweep — see the doc on
    /// `require_swept_rectangle` for why measured constants are not
    /// allowed to be the argument here any more.
    #[test]
    fn the_band_admits_a_sub_eps_entry_that_the_exact_form_refuses() {
        let fk = fixtures()
            .into_iter()
            .next()
            .and_then(|(_, b)| b.faces().next().map(|(fk, _)| fk))
            .unwrap();
        // The rectangle with a MID-SIDE vertex pulled a few ulps inside
        // the box — the shape a multiply-carried iso side produced
        // before #653, where the second sub-edge's column missed the
        // first's by the last bits of an `atan2`. A nudged CORNER would
        // not do: it still sits on the box's other axis and
        // `entries_off_bbox` takes the nearer of the two.
        // `unit_levers` makes UV units metres, so the wobble reads
        // straight off the coordinates.
        let mut inside = 4.0_f64;
        for _ in 0..4 {
            inside = f64::from_bits(inside.to_bits() - 1);
        }
        let poly = uv(&[
            (0.0, 0.0),
            (4.0, 0.0),
            (inside, 2.0),
            (4.0, 4.0),
            (0.0, 4.0),
        ]);
        let levers = unit_levers(poly.len());
        let b = bbox(&poly);
        let wobble = 4.0 - inside;
        assert!(
            wobble > 0.0 && eps().dominates(wobble),
            "fixture: {wobble} m must be a sub-eps nudge"
        );

        // The EXACT form. `gap_is_noise` is a strict `<`, so `eps = 0`
        // bands nothing at all and every entry comes back — the ones
        // ON the box at distance 0.0. Dropping those reproduces the
        // pre-band comparison exactly, and it sees the nudged entry:
        // one off-box vertex, at index 2, `wobble` metres out.
        let exact: Vec<_> = entries_off_bbox(&poly, &levers, b, Eps::exactly(0.0))
            .into_iter()
            .filter(|&(_, d)| d > 0.0)
            .collect();
        assert_eq!(
            exact,
            vec![(2, wobble)],
            "the exact form refuses this entry"
        );

        // The band admits it, and so does the production refusal.
        assert!(
            entries_off_bbox(&poly, &levers, b, eps()).is_empty(),
            "a {wobble} m wobble is float noise, not a re-entrant corner"
        );
        assert_eq!(
            require_swept_rectangle(fk, &poly, &levers, b, eps()),
            Ok(()),
            "the banded guard must not refuse a rectangle that is straight to ulps"
        );
    }

    /// **THE ZERO-LEVER WITNESS.** An axis whose lever arm is 0 carries
    /// no metric, so it decides nothing — the entry has to be on the
    /// box in the other one.
    ///
    /// `Chart::radial` is 0 exactly at a chart singularity, and
    /// `gap * 0 < eps` is true for every gap there is: banded against a
    /// vanished lever, the u axis admits a pole entry at ANY u. The two
    /// halves are asserted together because only the pair is the rule —
    /// the first alone would be satisfied by refusing every pole entry
    /// there is, which would false-refuse every sphere and cone this
    /// lane meshes.
    #[test]
    fn a_zero_lever_axis_admits_nothing() {
        // Strictly inside the box on BOTH axes — a re-entrant corner,
        // 2 m off in v at unit lever.
        let poly = uv(&[(0.0, 0.0), (4.0, 0.0), (2.0, 2.0), (4.0, 4.0), (0.0, 4.0)]);
        let mut levers = unit_levers(poly.len());
        levers[2] = (0.0, 1.0);
        assert_eq!(
            entries_off_bbox(&poly, &levers, bbox(&poly), eps()),
            vec![(2, 2.0)],
            "an entry off the box on both axes is off the box, and the distance \
             reported is the one axis that has a metric"
        );

        // The shape a pole entry actually has: interior in u, ON the
        // box in v. Admitted, and it must be — this is every sphere
        // pole and cone apex in the suite.
        let poly = uv(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 4.0), (0.0, 4.0)]);
        let mut levers = unit_levers(poly.len());
        levers[3] = (0.0, 1.0);
        assert!(
            entries_off_bbox(&poly, &levers, bbox(&poly), eps()).is_empty(),
            "a pole entry is on its box in v, and that is what admits it"
        );
    }

    /// **THE EXEMPTION, ASSERTED.** Every pole entry the walk emits
    /// carries a zero u lever and sits on its own bounding box in v.
    ///
    /// [`entries_off_bbox`] admits a pole entry on its v gap alone
    /// (the u axis has no metric there), so this is the fact that
    /// makes a sphere pole and a cone apex meshable rather than
    /// refused. It held silently before it was written down: the walk
    /// gives a pole entry the CHART's pole v ([`Chart::poles`]), the
    /// box is the entries' own min/max, and no chart puts a v beyond
    /// its own pole. If a face ever arrives whose pole v is interior —
    /// a cone spanning both nappes would be one — this row goes red
    /// and the guard REFUSES that face rather than meshing it, which
    /// is the outcome the payload argument in
    /// [`require_swept_rectangle`] wants.
    ///
    /// **The non-vacuity floor is global, not per fixture**, and that
    /// is a real difference from the rows above: most
    /// fixtures here have no pole at all, so "this fixture stopped
    /// contributing" is not distinguishable from "this fixture never
    /// had one". What IS per fixture is the other direction — a pole
    /// entry can only come from a chart that has poles — so a walk
    /// that started inventing them is caught where it happens.
    #[test]
    fn a_pole_entry_sits_on_its_own_box_in_v() {
        let mut seen = 0usize;
        let mut per_fixture: Vec<(&str, usize)> = Vec::new();
        for (name, body) in fixtures() {
            let mut here = 0usize;
            for (fk, poly, levers) in curved_walks(name, &body) {
                let (_, _, v0, v1) = bbox(&poly);
                let charted = body
                    .face_surface(fk)
                    .and_then(Chart::of)
                    .is_some_and(|c| !c.poles().is_empty());
                for (e, &(lu, _)) in poly.iter().zip(&levers).filter(|(e, _)| e.pole) {
                    here += 1;
                    assert!(
                        charted,
                        "{name} face {fk:?}: a pole entry at ({}, {}) on a chart whose \
                         `poles()` is empty",
                        e.u, e.v
                    );
                    assert_eq!(
                        lu, 0.0,
                        "{name} face {fk:?}: pole entry at ({}, {}) carries a u lever of \
                         {lu}, not 0 — `entries_off_bbox`'s degenerate-axis rule is \
                         about a different set of entries than the walk's pole flag",
                        e.u, e.v
                    );
                    assert!(
                        e.v == v0 || e.v == v1,
                        "{name} face {fk:?}: pole entry at ({}, {}) is interior in v \
                         (box v {v0}..{v1}), so nothing admits it and the face is \
                         refused — see `entries_off_bbox`",
                        e.u,
                        e.v
                    );
                }
            }
            seen += here;
            per_fixture.push((name, here));
        }
        assert!(
            seen > 0,
            "no fixture walked a pole entry, so this row asserts nothing: {per_fixture:?}"
        );
    }

    /// **THE MARGIN — the whole argument for banding in one row.**
    ///
    /// Between what the band must admit and what it must refuse there
    /// are orders of magnitude, so no calibration question arises:
    ///
    /// - a genuine re-entrant corner sits a FEATURE width off the box.
    ///   The notch fixture's corners are 1 m off; the shallowest
    ///   keyway anyone machines is millimetres. Refused.
    /// - the wobble of a multiply-carried iso side was the last bits of
    ///   an `atan2` — an ULP of a UV coordinate, which is what the
    ///   fixture below computes from the tree rather than quoting.
    ///   Admitted.
    ///
    /// ε = 1e-9 m sits between them. This row pins that the band is
    /// placed on the ε scale and not on either population's scale, so a
    /// future ε change is visible here rather than silently
    /// re-classifying parts.
    #[test]
    fn the_band_separates_a_feature_from_an_ulp_by_orders_of_magnitude() {
        let e = eps();
        let notch = uv(&notched_domain());
        let corner = entries_off_bbox(&notch, &unit_levers(notch.len()), bbox(&notch), e);
        assert_eq!(corner.len(), 2);
        let feature = corner.iter().map(|&(_, d)| d).fold(0.0_f64, f64::max);
        // The admit side, derived here rather than quoted: one ulp of a
        // UV coordinate at the notch fixture's own scale. Unit levers,
        // so this is metres.
        let ulp = f64::from_bits(4.0_f64.to_bits() + 1) - 4.0;
        assert!(
            e.dominates(ulp) && e.separates(feature),
            "the band must sit strictly between an ulp of a UV coordinate ({ulp} m) \
             and a feature-sized notch ({feature} m); eps = {e}"
        );
        assert!(
            feature / ulp > 1e14,
            "the two populations must be separated by orders of magnitude, not tuning"
        );
    }

    /// The counterexample body: a frustum wedge (`revolve` of a
    /// trapezoid through π/2) whose first boundary edge is split TWICE
    /// with the public [`topo::Body::split_edge`], then placed by a
    /// rigid rotation+translation with no special structure.
    ///
    /// Both halves matter. The split makes one iso side of the
    /// cylindrical/conical face carry three edges, so its column comes
    /// from three different `mid_azimuth` points of the same carrier;
    /// the oblique placement is what stops those three `atan2`s from
    /// landing on the same bits. Either alone meshes clean under an
    /// EXACT comparison — it is the pair that produced the false
    /// refusal (#653).
    ///
    /// Nothing here is a kernel back door: `split_edge` is public API,
    /// and the same topology is what every STEP exporter emits when a
    /// vertex lands on an otherwise-straight face boundary.
    fn split_and_placed_frustum_wedge() -> Body<f64> {
        let lp = ProfileLoop::polygon([
            Point2::new(0.5, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(1.0, 2.0),
            Point2::new(0.5, 2.0),
        ]);
        let mut body = revolve(
            &validated(vec![lp]),
            axis_y(),
            Revolution::Partial(core::f64::consts::FRAC_PI_2),
            Tol::witness(),
        )
        .unwrap()
        .body
        .into_body();
        let ek = body.edges().map(|(k, _)| k).next().unwrap();
        let (t0, t1) = body
            .get_curve_geom(body.get_edge(ek).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        body.split_edge(ek, t0 + (t1 - t0) * 0.312_9, Tol::witness())
            .unwrap();
        body.split_edge(ek, t0 + (t1 - t0) * 0.156_45, Tol::witness())
            .unwrap();
        // Deliberately irrational-ish: an axis-aligned or dyadic
        // placement keeps the sub-edge azimuths bitwise equal and the
        // row goes green for the wrong reason.
        let irr = Vec3::new(0.317_8_f64, 0.941_2, -0.110_9);
        let irr = irr * (1.0 / irr.norm());
        let placement = Affine3::from_parts(
            Affine3::rotation_about_axis(Point3::origin(), irr, 1.0 / 3.0).linear,
            Vec3::new(0.117, -0.339_1, 5.001_7),
        );
        topo::transform_rigid(
            &body,
            &geom_core::test_support::rigid(&placement, Tol::witness()),
        )
        .unwrap()
    }

    /// **THE REGRESSION ROW (issue #653).** This body meshed
    /// watertight before the guard existed and must keep doing so.
    ///
    /// Under an EXACT comparison it refused
    /// `UnsupportedCurvedDomain { off_bbox: 1 }` on a face whose bbox
    /// is `u = [-7.633e-16, 1.5708]` with the offending entry at
    /// `u = -6.384e-16` — 1.249e-16 rad off, which on that entry's
    /// 0.5 m lever arm is 6.245e-17 m, seven orders of magnitude
    /// inside ε. The domain IS the swept rectangle; the exact test was
    /// measuring float noise.
    ///
    /// **This row no longer goes red if the band is removed** — it did
    /// when it was written, and #653's walk fix is what changed that:
    /// the three sub-edges now share one column bitwise, so `worst`
    /// below is exactly `0.0` and an exact comparison would admit this
    /// body too. Kept as a triangle-count pin and as the
    /// oblique-placement fixture; the band's own case is argued in
    /// [`require_swept_rectangle`].
    #[test]
    fn a_split_then_placed_swept_face_is_not_refused() {
        let body = split_and_placed_frustum_wedge();
        for (fk, poly, levers) in curved_walks("split and placed frustum wedge", &body) {
            assert_eq!(
                require_swept_rectangle(fk, &poly, &levers, bbox(&poly), eps()),
                Ok(()),
                "face {fk:?} of the split+placed wedge is the swept rectangle"
            );
        }
        // The residue, measured through the production path rather than
        // asserted from the issue.
        let worst = worst_entry_off_box("split and placed frustum wedge", &body);
        assert_eq!(
            worst, 0.0,
            "since #653 the three sub-edges share one column bitwise, so this \
             body's walk IS its bounding rectangle exactly — {worst} m off the \
             box means the iso-side runs stopped working"
        );

        let mesh = crate::tessellate(&body, 0.1, Tol::witness()).expect("must not refuse");
        crate::validate::check_mesh(&mesh).expect("must mesh watertight");
        // The split ruling's face is the trimmed lane's
        // (`walk::splits_a_straight_side`), which walks its two rows
        // (`tessellate::Lane::Walked`) and lays no point inside it: 28,
        // where the chart lane's lattice and CDT made 38.
        let n: usize = mesh.patches.iter().map(|p| p.triangles.len()).sum();
        assert_eq!(n, 28, "the split+placed wedge's triangle count at δ = 0.1");
    }

    /// **What the refusal replaces**, replayed through this lane's own
    /// insertion order. On the swept rectangle the grid touches the
    /// boundary not at all; on the notched domain it splits boundary
    /// constraints and lands on a boundary VERTEX. Both are properties
    /// of the GRID (they are counted against zero on the rectangle,
    /// where the same `inner_faces()` runs), which is why the
    /// ghost-triangle count that used to sit here is gone: with the
    /// grid loop emptied (`nu = nv = 1`) a notched domain still emits
    /// 2 of 8 triangles in the notch, because `inner_faces()` fills the
    /// convex hull — so that assertion could not tell the grid's doing
    /// from this lane's keep-every-inner-face policy. The ghost
    /// geometry is real and is why the refusal exists; it is just not
    /// evidence about the ordering.
    #[test]
    fn the_grid_reaches_the_boundary_only_when_the_premise_fails() {
        assert_eq!(
            replay(&rectangle_domain()),
            (0, 0),
            "on the swept rectangle the grid must not touch the boundary at all"
        );
        let (splits, hits) = replay(&notched_domain());
        assert!(
            splits > 0,
            "a notched domain must split boundary constraints"
        );
        assert!(
            hits > 0,
            "a notched domain must put a grid point on a boundary vertex"
        );
    }

    /// **The two spade facts the inertness argument rests on**, pinned
    /// rather than asserted in prose. `spade` is a CARET requirement,
    /// so a 2.x bump could move either with nothing else in this crate
    /// going red:
    ///
    /// 1. inserting a point that lands exactly ON a constraint splits
    ///    it and re-flags **both** halves as constraints — which is why
    ///    a split corrupts nothing the CDT is later asked about here;
    /// 2. the new vertex takes the **next** handle index — which is
    ///    what keeps `meta`, indexed by handle index, aligned.
    ///
    /// Note the scope: (1) is about what this lane READS. It is not a
    /// claim that a split is harmless at the crate's altitude — a split
    /// boundary constraint breaks the watertightness contract in
    /// `lib.rs` (both adjacent faces conforming to the same segments),
    /// which is the 3-D T-junction `trimmed` refuses as
    /// `SelfTouchingTrimLoop`. That is the reason the domain check is a
    /// refusal and not a comment.
    #[test]
    fn spade_splits_a_constraint_into_two_constraints_and_appends_the_vertex() {
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        let hs: Vec<_> = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
            .iter()
            .map(|&(u, v)| cdt.insert(SpadePoint::new(u, v)).unwrap())
            .collect();
        for i in 0..hs.len() {
            cdt.add_constraint(hs[i], hs[(i + 1) % hs.len()]);
        }
        let (a, b) = (hs[0], hs[1]);
        assert!(cdt.exists_constraint(a, b));
        let n = cdt.num_vertices();
        let m = cdt.insert(SpadePoint::new(2.0, 0.0)).unwrap();

        assert_eq!(m.index(), n, "the split vertex must take the NEXT index");
        assert!(
            !cdt.exists_constraint(a, b),
            "the whole segment must no longer be a constraint"
        );
        assert!(
            cdt.exists_constraint(a, m) && cdt.exists_constraint(m, b),
            "BOTH halves must be re-flagged as constraints"
        );
    }

    /// **A cone patch takes no rows, at any column count** (the cone
    /// arm's theorem, `sizing::cone_grid_step`): the π/6 wedge (ρ_max = 1,
    /// α = π/4, vspan = √2) at δ = 0.1 (one column), 0.0125 (two) and
    /// 0.001 (five), with and without an apex entry, sizes to `(nu, 1)`.
    /// Rows reintroduced at any column count red here, and `nu` is the
    /// column count the rims are chorded at.
    #[test]
    fn a_cone_patch_takes_no_rows_at_any_column_count() {
        // The π/6 wedge fixture's own chart: half-angle π/4, ρ_max = 1.
        let surface = Surface::Cone {
            frame: geom::Frame::new(
                Point3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            half_angle: core::f64::consts::FRAC_PI_4,
        };
        let row = crate::chart_table::AnalyticRow::new(&surface, 1.0).unwrap();
        let (uspan, vspan) = (core::f64::consts::FRAC_PI_6, 2.0_f64.sqrt());
        for (delta, nu) in [(0.1, 1), (0.0125, 2), (0.001, 5)] {
            for has_pole in [true, false] {
                assert_eq!(
                    grid_counts(&row, delta, (uspan, vspan), has_pole, 0).unwrap(),
                    (nu, 1),
                    "δ {delta}, has_pole {has_pole}: {nu} columns and no rows"
                );
            }
        }
    }

    /// Replays this lane's order — boundary points, constraints, then a
    /// 4 × 4 interior grid over the polygon's own bounding box — and
    /// reports (constraint splits, grid points that landed on an
    /// existing vertex).
    fn replay(poly: &[(f64, f64)]) -> (usize, usize) {
        let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
            ConstrainedDelaunayTriangulation::new();
        let hs: Vec<_> = poly
            .iter()
            .map(|&(u, v)| cdt.insert(SpadePoint::new(u, v)).unwrap())
            .collect();
        for i in 0..hs.len() {
            let (a, b) = (hs[i], hs[(i + 1) % hs.len()]);
            if a != b && !cdt.exists_constraint(a, b) {
                assert!(
                    cdt.can_add_constraint(a, b),
                    "the fixture must not self-cross"
                );
                cdt.add_constraint(a, b);
            }
        }
        let (u0, u1, v0, v1) = bbox(&uv(poly));
        let before = cdt.num_constraints();
        let (mut hits, nu, nv) = (0usize, 4usize, 4usize);
        for j in 1..nv {
            #[allow(clippy::cast_precision_loss)]
            let v = v0 + (v1 - v0) * (j as f64 / nv as f64);
            for i in 1..nu {
                #[allow(clippy::cast_precision_loss)]
                let u = u0 + (u1 - u0) * (i as f64 / nu as f64);
                let n = cdt.num_vertices();
                if cdt.insert(SpadePoint::new(u, v)).unwrap().index() != n {
                    hits += 1;
                }
            }
        }
        (cdt.num_constraints() - before, hits)
    }

    /// A boundary entry at a chosen UV with a chosen mesh id.
    ///
    /// Gated with the rows that use it: they call
    /// `#[cfg(debug_assertions)]` items, so with debug-assertions off
    /// the rows are gone and this would be dead code.
    #[cfg(debug_assertions)]
    fn entry(u: f64, v: f64, id: u32, pole: bool) -> UvPoint {
        UvPoint { u, v, id, pole }
    }

    #[cfg(debug_assertions)]
    #[test]
    fn identified_ids_takes_the_seam_the_pole_filter_misses() {
        // RED FIRST for the widening (issue 897). A full-2π seam
        // double-traversal: one mesh id, two ends of the u range, no
        // pole flag anywhere. The set this census runs on must contain
        // it; the filter it replaced — `e.pole` — is empty here, which
        // is precisely how the seam case went unchecked.
        let seam = vec![
            entry(0.0, 0.0, 7, false),
            entry(0.0, 1.0, 8, false),
            entry(core::f64::consts::TAU, 1.0, 8, false),
            entry(core::f64::consts::TAU, 0.0, 7, false),
        ];
        assert_eq!(seam.iter().filter(|e| e.pole).count(), 0);
        let mut got: Vec<u32> = identified_ids(&seam).into_iter().collect();
        got.sort_unstable();
        assert_eq!(got, vec![7, 8]);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn identified_ids_keeps_a_pole_corner_and_drops_an_ordinary_walk() {
        // The set stays a SUPERSET of the pole-incident one it
        // replaced: a pole entry is in it even when it appears once
        // and so cannot be collapsed.
        let one_pole = vec![
            entry(0.0, 0.0, 1, true),
            entry(1.0, 0.0, 2, false),
            entry(1.0, 1.0, 3, false),
        ];
        assert_eq!(
            identified_ids(&one_pole).into_iter().collect::<Vec<_>>(),
            vec![1]
        );
        // And a walk that identifies nothing costs the census nothing.
        // The thunk PANICS if it is called: the empty case must not
        // materialise the patch in mesh ids, and this is the row that
        // fails if that regresses (debug assertions are on in the
        // release profile, so the allocation would ship).
        let plain = vec![
            entry(0.0, 0.0, 1, false),
            entry(1.0, 0.0, 2, false),
            entry(1.0, 1.0, 3, false),
        ];
        assert!(identified_ids(&plain).is_empty());
        assert_eq!(
            overused_identified_edge(&plain, || {
                panic!("the patch must not be materialised when nothing is identified")
            }),
            None
        );
    }

    #[cfg(debug_assertions)]
    #[test]
    fn a_repeat_at_the_same_uv_is_one_spade_vertex_and_not_identified() {
        // "Distinct" is spade's `==`, not a bit compare: an entry the
        // CDT merges cannot be fanned apart, so it is not in the set.
        let merged = vec![
            entry(0.0, -0.0, 4, false),
            entry(1.0, 0.0, 5, false),
            entry(-0.0, 0.0, 4, false),
        ];
        assert!(identified_ids(&merged).is_empty());
    }

    #[cfg(debug_assertions)]
    #[test]
    fn a_seam_vertex_fanned_over_one_column_is_caught() {
        // RED FIRST for the guard itself, in #678's own shape but with
        // a SEAM rather than a pole supplying the identification: the
        // two entries of id 8 end up sharing a single interior column
        // (id 9), so the edge (8, 9) is used four times in one patch.
        let seam = vec![
            entry(0.0, 0.0, 7, false),
            entry(0.0, 1.0, 8, false),
            entry(core::f64::consts::TAU, 1.0, 8, false),
            entry(core::f64::consts::TAU, 0.0, 7, false),
        ];
        let fanned = [[8, 9, 7], [9, 8, 10], [8, 9, 11], [9, 8, 12]];
        assert_eq!(
            overused_identified_edge(&seam, || fanned.to_vec()),
            Some(((8, 9), 4))
        );
        // The fan the argument PREDICTS — every identified edge at two
        // uses — is quiet.
        let clean = [[7, 8, 9], [8, 7, 10], [9, 8, 10], [7, 9, 10]];
        assert_eq!(overused_identified_edge(&seam, || clean.to_vec()), None);
    }

    /// The census threshold is at THREE uses, not four.
    ///
    /// Three is already the non-manifold state — an undirected edge
    /// interior to a patch has two uses and a boundary one has one, so
    /// a third use is the defect however many more follow. #678's own
    /// witness happens to show four, and a threshold written from that
    /// witness (`n > 3`) is a mutant this census would otherwise
    /// survive: every other row here fans an edge four times. This row
    /// fans one exactly three times.
    #[cfg(debug_assertions)]
    #[test]
    fn three_uses_of_an_identified_edge_is_already_the_defect() {
        let seam = vec![
            entry(0.0, 0.0, 7, false),
            entry(0.0, 1.0, 8, false),
            entry(core::f64::consts::TAU, 1.0, 8, false),
            entry(core::f64::consts::TAU, 0.0, 7, false),
        ];
        let thrice = [[8, 9, 7], [9, 8, 10], [8, 9, 11]];
        assert_eq!(
            overused_identified_edge(&seam, || thrice.to_vec()),
            Some(((8, 9), 3)),
            "a third use is the defect; a threshold of four would pass this patch"
        );
    }

    #[test]
    fn the_full_2pi_seam_never_reaches_the_two_column_shape() {
        // `pole_columns`' argument for the seam case, VERIFIED as the
        // arithmetic it claims to be: the fan needs a single interior
        // column between the two identified entries (`nu == 2`), and a
        // 2π span cannot size to one while the angular cap holds.
        let tau = core::f64::consts::TAU;
        // THE MECHANISM, NAMED AS THE ONE THAT FIRES. An earlier
        // spelling of this row opened with
        // `const { assert!(MAX_ANGULAR_STEP <= FRAC_PI_4) }`, which
        // reds at COMPILE time if the cap is raised — so the runtime
        // asserts the row advertises would never have run, and the row
        // named a mechanism that could not be the one to fire. The cap
        // is pinned here instead, through the same door the sizing
        // arms use, so a raised cap reds THIS assert, at run time, in
        // the same battery as the rest.
        assert_eq!(
            ceil_count(tau, cap_angular(f64::INFINITY)).unwrap(),
            8,
            "the angular cap decides the seam bound: at pi/4 a 2*pi span takes eight columns"
        );
        for step in [
            f64::MIN_POSITIVE,
            1e-12,
            1e-6,
            MAX_ANGULAR_STEP,
            1.0,
            1e6,
            f64::INFINITY,
            f64::NAN,
        ] {
            // A refusal is the far side of the same claim — the count
            // the step asks for is past `ceil_count`'s 2^24 cap, which
            // is not two columns either.
            assert!(
                !matches!(ceil_count(tau, cap_angular(step)), Ok(n) if n < 8),
                "a 2π span at capped step {step} sized below eight columns"
            );
        }
        // The row that goes RED if the floor is lowered: at a cap of π
        // — four times today's — the same span sizes to exactly the
        // two-column shape, and the arithmetic stops holding the case
        // off. The cap is what this depends on, and this says so.
        assert_eq!(ceil_count(tau, core::f64::consts::PI).unwrap(), 2);
    }

    /// **The oblique lens has no walk.** It is a sphere face bounded by
    /// two tilted plane sections meeting off the axis. Handed to the walk
    /// with both arcs stated as rims (what a classifier by axis would
    /// read them as, `|n · axis| > 0.5`), the loop has no meridian and
    /// `walk::loop_polygon` refuses it on that structure alone. A walk
    /// of it would merge both arcs onto ONE `v` — a polygon that is its
    /// own zero-height bounding box, which the spatial check
    /// ([`require_swept_rectangle`]) cannot tell from a rectangle and
    /// the CDT triangulates to nothing — and that polygon is what the
    /// refusal stands in front of. The shape door decides the same face
    /// is no rectangle earlier still, on its rims' CARRIERS, so the route
    /// never hands it to this walk; the door does not establish
    /// the walk's arc premise (issue 1571), and this row does not claim
    /// it does.
    #[test]
    fn the_lens_walk_refuses_for_want_of_a_meridian_behind_the_shape_door() {
        let (body, face) = crate::witness_bodies::oblique_lens();
        // The route sends both lens faces to the trimmed lane (below);
        // handed to this lane all the same, with both oblique arcs stated
        // as rims (what a classifier by axis would read them as), the walk
        // refuses the lens on its own terms: no side is a meridian.
        let routes = body
            .faces()
            .fold(crate::tessellate::Routes::witness(&body), |r, (fk, f)| {
                let (_, half_edges) =
                    topo::props::loop_edges(body.cycle_loop(f.outer).expect("a cycle loop"))
                        .unwrap();
                let sides = half_edges
                    .into_iter()
                    .map(|he| (he, geom_brep::IsoFamily::V))
                    .collect();
                r.with_curved(fk, Rectangle { sides })
            });
        let walked = walk_results(&body, routes);
        let (_, lens) = walked
            .iter()
            .find(|(fk, _)| *fk == face)
            .expect("the lens face reaches the walk");
        assert_eq!(
            lens.as_ref().map(|_| ()),
            Err(&TessellateError::MeridianFreeCurvedFace {
                face,
                surface: geom::SurfaceKind::Sphere,
            }),
            "two arcs stated as rims and nothing else"
        );
        let surface = body.face_surface(face).unwrap();
        let outer = body.get_face(face).unwrap().outer;
        let band = Band::linear(Tol::witness()).unwrap();
        assert_eq!(
            geom_brep::props::require_iso_rectangle(
                surface,
                &topo::props::loop_edges(body.cycle_loop(outer).expect("a cycle loop"))
                    .unwrap()
                    .0,
                band
            ),
            Err(geom_brep::props::PropsError::NotIsoRectangle {
                what: "props_rim_axis_parallel",
            }),
            "the shape door decides the lens is no rectangle on rim structure"
        );
        assert_eq!(
            iso_rectangle_shape(&body, face, outer, surface, band),
            Ok(Shape::NoRectangle(
                geom_brep::props::PropsError::NotIsoRectangle {
                    what: "props_rim_axis_parallel",
                }
            )),
            "so the router sends it away from the walk"
        );
    }
}
