//! Curved-face tessellation: UV-rectangle domain from the boundary
//! walk, interior grid sampling, CDT in parameter space, pole-fan
//! collapse, per-triangle certificates.
//!
//! Every curved M2 face is a swept UV rectangle (crate docs); the
//! boundary polygon from [`crate::walk`] has bitwise-straight sides,
//! so the domain is convex and no inside/outside classification is
//! needed — every CDT triangle is kept except pole-degenerate ones.
//! Boundary polyline segments are inserted as CDT **constraints**, so
//! the triangulation conforms to the shared chord segments in both
//! adjacent faces (the watertightness guarantee).
//!
//! Pole fans: pole corners enter the CDT at their (distinct) UV
//! locations but map to the single pole mesh vertex; any triangle with
//! two corners of the same mesh vertex is degenerate in 3-D and is
//! dropped, which collapses the strip along the pole side into a fan
//! around the pole (one dropped triangle per collapsed side; its two
//! non-collapsed edges become the identified fan edges — manifoldness
//! is re-checked by the mesh validator).
//!
//! Grid sizing (heuristic; the certificates are the guarantee), from
//! δ_s = δ/2 and φ = [`crate::chords::sagitta_angle`]:
//! cylinder — hu = φ(δ_s, r), no interior rows (ruled in v);
//! cone — hu = φ(δ_s, ρ_max), rows every ρ_max·hu slant meters (ruled
//! in v, but rows keep triangles azimuth-local so the radius-scaled
//! certificate stays tight); sphere — hu = hv = φ(δ_s, r); torus —
//! hu = hv = √(δ_s/(3(R+2r))) (matching the boundary chord
//! tightening in [`crate::chords`]).

use std::collections::HashMap;

use geom_core::Point3;
use geom_surfaces::Surface;
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::{Body, EdgeKey, FaceKey};

use crate::cert;
use crate::chords::{ceil_count, sagitta_angle, torus_grid_step};
use crate::types::TessellateError;
use crate::walk::{Chart, ChartKind, loop_polygon};

/// The call's tolerance bundle: δ (the promise), δ_s = δ/2 (sizing),
/// and the run's kernel ε (pole identification only — never sizing).
pub(crate) struct Tol {
    /// The chordal tolerance δ.
    pub delta: f64,
    /// The sizing target δ_s = δ/2.
    pub delta_s: f64,
    /// The kernel ε (pole/apex vertex identification).
    pub eps: f64,
}

/// Tessellates one curved face into outward-wound triangles,
/// appending interior grid points to `positions`.
pub(crate) fn tessellate_curved(
    body: &Body<f64>,
    fk: FaceKey,
    surface: &Surface<f64>,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    positions: &mut Vec<Point3<f64>>,
    tol: &Tol,
) -> Result<Vec<[u32; 3]>, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    if !face.rings.is_empty() {
        return Err(TessellateError::RingOnCurvedFace { face: fk });
    }
    let chart = Chart::of(surface).ok_or(TessellateError::MissingEntity {
        what: "curved chart",
    })?;
    let polygon = loop_polygon(body, &chart, chords, positions, fk, face.outer, tol.eps)?;
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
    // S10 CATEGORY B — do NOT multiply by the face's `sense_sign`.
    // `area2` is the UV shoelace of the boundary walk, so its sign is
    // derived entirely from the loop's STORED TRAVERSAL order, which
    // interior-left already ties to the face's outward normal: the
    // polygon runs CCW in the chart's UV plane iff the outward normal
    // agrees with the chart normal, i.e. iff `sense`. A reversed face
    // therefore lands here with a negative `area2` and flips, which is
    // exactly right. `revert` reverses the loops AND flips `sense`
    // together, so multiplying would double-count the reversal and
    // emit inward-wound triangles. (The one place the sense IS read on
    // this path is the pole-to-pole band's azimuth in `walk` — that
    // reads a DIRECTION in the chart frame, not a winding, and the two
    // reads do not overlap.)
    let flip = area2 < 0.0;

    // Grid steps per kind (module docs).
    let (nu, nv) = grid_steps(
        &chart,
        tol.delta_s,
        u1 - u0,
        v1 - v0,
        v0.abs().max(v1.abs()),
    )?;

    // CDT: boundary entries (fixed walk order) + constraints + grid.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    // Per-CDT-vertex metadata, indexed by handle index (insertion order).
    let mut meta: Vec<(f64, f64, u32, bool)> = Vec::new();
    let insert = |cdt: &mut ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
                  meta: &mut Vec<(f64, f64, u32, bool)>,
                  u: f64,
                  v: f64,
                  id: u32,
                  pole: bool|
     -> Result<spade::handles::FixedVertexHandle, TessellateError> {
        let h = cdt
            .insert(SpadePoint::new(u, v))
            .map_err(|_| TessellateError::Triangulation { face: fk })?;
        if h.index() == meta.len() {
            meta.push((u, v, id, pole));
        }
        Ok(h)
    };
    let mut handles = Vec::with_capacity(polygon.len());
    for e in &polygon {
        handles.push(insert(&mut cdt, &mut meta, e.u, e.v, e.id, e.pole)?);
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
    // GRID AFTER CONSTRAINTS — and NOT the hazard
    // `planar::triangulate_chart`'s header warns about (S28). That
    // warning is a precondition of PLANAR's crossing bookkeeping: a
    // vertex landing on a constraint splits it, and the two halves
    // would be counted where the whole segment was. This lane keeps no
    // such bookkeeping — `inner_faces()` emits every triangle — and
    // spade re-flags BOTH halves of a split edge as constraints, so a
    // split would corrupt nothing read here.
    //
    // What actually keeps the grid off the boundary is the swept-UV-
    // rectangle contract (module docs): the walk's polygon IS its own
    // bounding box, so `i` and `j` running the OPEN ranges below put
    // every grid point strictly inside every constraint. That premise —
    // not the ordering — is the thing to re-check if this lane is ever
    // handed a face whose iso boundary is not a rectangle; the tests at
    // the foot of this file pin it, including through a boolean cut,
    // and show what a notched domain does instead.
    let (uspan, vspan) = (u1 - u0, v1 - v0);
    for j in 1..nv {
        #[allow(clippy::cast_precision_loss)]
        let v = v0 + vspan * (j as f64 / nv as f64);
        for i in 1..nu {
            #[allow(clippy::cast_precision_loss)]
            let u = u0 + uspan * (i as f64 / nu as f64);
            let p = surface.eval(u, v);
            #[allow(clippy::cast_possible_truncation)]
            let id = positions.len() as u32;
            let h = insert(&mut cdt, &mut meta, u, v, id, false)?;
            if h.index() == meta.len() - 1 && meta[meta.len() - 1].2 == id {
                positions.push(p);
            }
        }
    }

    // Emit: drop pole-degenerate triangles, certify the rest.
    let mut triangles = Vec::new();
    let mut worst: f64 = 0.0;
    for f in cdt.inner_faces() {
        let vs = f.vertices();
        let m: Vec<(f64, f64, u32, bool)> = vs.iter().map(|v| meta[v.fix().index()]).collect();
        let ids = [m[0].2, m[1].2, m[2].2];
        if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
            continue; // pole-collapsed sliver
        }
        let tri = [
            positions[ids[0] as usize],
            positions[ids[1] as usize],
            positions[ids[2] as usize],
        ];
        let uv = [[m[0].0, m[0].1], [m[1].0, m[1].1], [m[2].0, m[2].1]];
        let pole = [m[0].3, m[1].3, m[2].3];
        let bound = match chart.kind {
            ChartKind::Cylinder { r } => cert::cert_cylinder(chart.anchor, chart.axis, r, tri),
            ChartKind::Sphere { r } => cert::cert_sphere(chart.anchor, r, tri),
            ChartKind::Cone { half_angle } => cert::cert_cone(half_angle, uv, pole),
            ChartKind::Torus { major, minor } => cert::cert_torus(major, minor, uv),
        };
        // Sticky-NaN accumulation: `f64::max` would silently drop a
        // poisoned bound.
        if bound.is_nan() || worst.is_nan() || bound > worst {
            worst = bound;
        }
        triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
    }
    if worst.is_nan() || worst > tol.delta {
        return Err(TessellateError::CertificateExceeded {
            face: fk,
            bound: worst,
            requested: tol.delta,
        });
    }
    Ok(triangles)
}

/// Interior grid step counts (nu, nv) for the face's UV spans.
fn grid_steps(
    chart: &Chart,
    delta_s: f64,
    uspan: f64,
    vspan: f64,
    v_absmax: f64,
) -> Result<(usize, usize), TessellateError> {
    let cap = core::f64::consts::FRAC_PI_4;
    match chart.kind {
        ChartKind::Cylinder { r } => {
            let hu = sagitta_angle(delta_s, r);
            Ok((ceil_count(uspan, hu)?, 1))
        }
        ChartKind::Cone { half_angle } => {
            let rho_max = v_absmax * half_angle.sin();
            let hu = sagitta_angle(delta_s, rho_max);
            let hv = rho_max * hu;
            Ok((ceil_count(uspan, hu)?, ceil_count(vspan, hv)?))
        }
        ChartKind::Sphere { r } => {
            // Deliberate 1.25 sizing margin: near the equator a
            // full-step grid triangle's true deviation approaches
            // 2·δ_s = δ from below, so sizing at exactly δ_s would
            // lean on ceil_count's step-shrink (span/⌈span/h⌉ < h) as
            // the only slack. Targeting δ_s/1.25 buys real headroom
            // cheaply (≈12% more steps per axis) and keeps future
            // sizing tweaks from silently landing on the certificate
            // boundary; the certificate remains the backstop.
            let h = sagitta_angle(delta_s / 1.25, r);
            Ok((ceil_count(uspan, h)?, ceil_count(vspan, h)?))
        }
        ChartKind::Torus { major, minor } => {
            let h = torus_grid_step(delta_s, major, minor).min(cap);
            Ok((ceil_count(uspan, h)?, ceil_count(vspan, h)?))
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! **S28 — why this lane's grid-after-constraints order is inert,
    //! pinned so the next reader need not re-derive it.**
    //!
    //! `planar::triangulate_chart`'s header warns that a vertex landing
    //! on a constraint splits it. That is a precondition of PLANAR's
    //! crossing bookkeeping (PR #116's even-odd flood fill), written
    //! five days after this file, and it was never a claim about this
    //! lane: nothing here counts constraint traversals, and spade
    //! re-flags both halves of a split edge.
    //!
    //! The premise that DOES carry this lane is the swept-UV-rectangle
    //! contract: the boundary walk's polygon is its own bounding box,
    //! so an interior grid over that box (`1..nu` × `1..nv`) can never
    //! reach a constraint. The rows below pin the premise over every
    //! chart this build authors — including a face produced by a
    //! BOOLEAN cut, whose chart re-cut is what keeps it iso-rectangular
    //! — and show, on a notched domain, exactly what the premise buys.

    use super::*;
    use geom_core::{Affine3, Point2, Tolerance, Vec2, Vec3};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane, ValidatedProfile};
    use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};

    fn p2(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }

    fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
        Profile::new(SketchPlane::xy(), loops)
            .validate(Tolerance::get())
            .unwrap()
    }

    fn axis_y() -> RevolveAxis<f64> {
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        }
    }

    /// The indices of polygon entries that do NOT lie on the polygon's
    /// own bounding rectangle — empty ⟺ the UV domain IS that
    /// rectangle.
    ///
    /// Sufficient, and it is the shape that matters: a rectilinear
    /// simple polygon has a re-entrant corner exactly when some vertex
    /// sits strictly inside its bounding box, and a re-entrant corner
    /// is what lets an interior grid point land on — or across — a
    /// boundary constraint.
    ///
    /// The comparison is EXACT, never banded: the walk assigns each
    /// side's constant coordinate once per edge, so a rectangle side is
    /// bitwise straight (`walk` module docs) and a near-miss here would
    /// itself be the finding.
    fn entries_off_bbox(poly: &[(f64, f64)]) -> Vec<usize> {
        let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for &(u, v) in poly {
            u0 = u0.min(u);
            u1 = u1.max(u);
            v0 = v0.min(v);
            v1 = v1.max(v);
        }
        poly.iter()
            .enumerate()
            .filter(|&(_, &(u, v))| !(u == u0 || u == u1 || v == v0 || v == v1))
            .map(|(i, _)| i)
            .collect()
    }

    /// Every face this lane would take, walked to its UV polygon — the
    /// `tessellate` prologue (mesh ids, then the chord pass) run for
    /// the walk alone.
    fn curved_walks(body: &Body<f64>) -> Vec<(FaceKey, Vec<(f64, f64)>)> {
        let eps = Tolerance::get().eps;
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, v) in body.vertices() {
            #[allow(clippy::cast_possible_truncation)]
            vids.insert(vk, positions.len() as u32);
            positions.push(*body.get_point(v.point).unwrap());
        }
        let (chords, _) =
            crate::chords::compute_chords(body, 0.025, &vids, &mut positions).unwrap();
        let mut out = Vec::new();
        for (fk, face) in body.faces() {
            if crate::trimmed::has_trim_carrier(body, fk).unwrap() {
                continue;
            }
            let Some(chart) = Chart::of(body.get_surface(face.surface).unwrap()) else {
                continue;
            };
            let poly =
                loop_polygon(body, &chart, &chords, &positions, fk, face.outer, eps).unwrap();
            out.push((fk, poly.iter().map(|e| (e.u, e.v)).collect()));
        }
        out
    }

    fn ball() -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    fn cone_body() -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    fn washer() -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    fn donut() -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(p2(2.0, -0.5), 1.0),
            ProfileVertex::new(p2(2.0, 0.5), 1.0),
        ]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
            .unwrap()
            .body
    }

    fn wedge(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta))
            .unwrap()
            .body
    }

    fn rounded_prism() -> Body<f64> {
        let b = core::f64::consts::FRAC_PI_8.tan();
        let r = 0.5;
        let v = |pos: Point2<f64>, bulge: f64| ProfileVertex::new(pos, bulge);
        let mut lp = ProfileLoop::new(vec![
            v(p2(r, 0.0), 0.0),
            v(p2(2.0 - r, 0.0), b),
            v(p2(2.0, r), 0.0),
            v(p2(2.0, 2.0 - r), b),
            v(p2(2.0 - r, 2.0), 0.0),
            v(p2(r, 2.0), b),
            v(p2(0.0, 2.0 - r), 0.0),
            v(p2(0.0, r), b),
        ]);
        let n = lp.vertices().len();
        lp = lp.with_tangent_joints((0..n).collect());
        extrude(&validated(vec![lp]), Extrusion::Distance(1.0))
            .unwrap()
            .body
    }

    /// The die pip (`sweep`'s `m5_s13_pips` shape): a 4 × 4 × 1 slab
    /// with a radius-0.5 ball subtracted 0.2 above its top face. The
    /// cavity's two sphere faces are the only curved faces in this
    /// crate's reach that a BOOLEAN produced rather than a sweep.
    fn die_pip() -> Body<f64> {
        let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
            p2(0.0, 0.0),
            p2(4.0, 0.0),
            p2(4.0, 4.0),
            p2(0.0, 4.0),
        ]);
        let slab = extrude(&validated(vec![lp]), Extrusion::Distance(1.0))
            .unwrap()
            .body;
        let half = ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -0.5), 1.0),
            ProfileVertex::new(p2(0.0, 0.5), 0.0),
        ]);
        let ball = revolve(&validated(vec![half]), axis_y(), Revolution::Full)
            .unwrap()
            .body;
        let ball =
            topo::transform_rigid(&ball, &Affine3::translation(Vec3::new(2.0, 2.0, 1.2))).unwrap();
        topo::boolean::subtract(&slab, &ball)
            .expect("the die pip cuts")
            .body()
            .expect("a pip is a dent, not a void")
            .body
            .clone()
    }

    /// **The premise.** Every curved face this build can put in front of
    /// [`tessellate_curved`] walks to a UV polygon that IS its own
    /// bounding rectangle — which is what puts the interior grid
    /// strictly inside every boundary constraint, and is therefore the
    /// reason the grid-after-constraints order costs nothing here.
    ///
    /// The die pip is in the list on purpose (§C10, the sweep this
    /// finding asks for): the boolean's chart re-cut is what keeps a
    /// CUT sphere face iso-rectangular, and if that ever stops holding,
    /// this row is where it surfaces — in the lane that would silently
    /// mesh the bounding box.
    #[test]
    fn every_curved_walk_is_its_own_bounding_rectangle() {
        let bodies: Vec<(&str, Body<f64>)> = vec![
            ("ball", ball()),
            ("cone", cone_body()),
            ("washer", washer()),
            ("donut", donut()),
            ("wedge(pi/2)", wedge(core::f64::consts::FRAC_PI_2)),
            ("wedge(pi)", wedge(core::f64::consts::PI)),
            ("wedge(2pi - 0.05)", wedge(core::f64::consts::TAU - 0.05)),
            ("rounded prism", rounded_prism()),
            ("die pip (boolean cut)", die_pip()),
        ];
        let mut walked = 0;
        for (name, body) in bodies {
            for (fk, poly) in curved_walks(&body) {
                walked += 1;
                let off = entries_off_bbox(&poly);
                assert!(
                    off.is_empty(),
                    "{name} face {fk:?}: {} of {} walk entries lie strictly inside the UV \
                     bounding box, so the domain is not that box — the interior grid can \
                     reach a boundary constraint and `inner_faces()` emits triangles \
                     outside the face. Offenders: {:?}",
                    off.len(),
                    poly.len(),
                    off.iter().map(|&i| poly[i]).collect::<Vec<_>>()
                );
            }
        }
        assert!(walked >= 14, "only {walked} curved faces walked");
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

    /// The bite `notched_domain` takes out of its bounding box — the
    /// region that is inside the box and OUTSIDE the face.
    const NOTCH: [f64; 4] = [1.0, 3.0, 2.0, 4.0];

    /// **RED half, part 1** (`REVIEW-STYLE-BRIEF` Q3): the predicate the
    /// row above trusts detects the notch — it is not a tautology that
    /// passes on everything.
    #[test]
    fn the_premise_predicate_detects_a_notched_domain() {
        assert!(entries_off_bbox(&rectangle_domain()).is_empty());
        assert_eq!(
            entries_off_bbox(&notched_domain()),
            vec![4, 5],
            "the two re-entrant corners are the entries strictly inside the box"
        );
    }

    /// **RED half, part 2.** What the premise actually buys, replayed
    /// through this lane's own insertion order. On the rectangle the
    /// grid touches nothing. On the notched domain it splits boundary
    /// constraints, lands on a boundary VERTEX, and leaves triangles
    /// inside the notch — i.e. outside the face — which is the silently
    /// wrong mesh, not a refusal and not a panic.
    #[test]
    fn the_grid_reaches_the_boundary_exactly_when_the_premise_fails() {
        let (splits, hits, outside) = replay(&rectangle_domain(), NOTCH);
        assert_eq!(
            (splits, hits),
            (0, 0),
            "on the swept rectangle the grid must not touch the boundary at all"
        );
        assert!(
            outside > 0,
            "sanity: the probe box IS inside the rectangle, so it must find triangles \
             there — otherwise the `outside` counter below proves nothing"
        );
        let (splits, hits, outside) = replay(&notched_domain(), NOTCH);
        assert!(
            splits > 0,
            "a notched domain must split boundary constraints"
        );
        assert!(
            hits > 0,
            "a notched domain must put a grid point on a boundary vertex"
        );
        assert!(
            outside > 0,
            "and `inner_faces()` must then emit triangles outside the face"
        );
    }

    /// Replays this lane's order — boundary points, constraints, then a
    /// 4 × 4 interior grid over the polygon's own bounding box — and
    /// reports (constraint splits, grid points that landed on an
    /// existing vertex, emitted triangles whose centroid falls in
    /// `probe`).
    fn replay(poly: &[(f64, f64)], probe: [f64; 4]) -> (usize, usize, usize) {
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
        let (mut u0, mut u1, mut v0, mut v1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for &(u, v) in poly {
            u0 = u0.min(u);
            u1 = u1.max(u);
            v0 = v0.min(v);
            v1 = v1.max(v);
        }
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
        let mut outside = 0;
        for f in cdt.inner_faces() {
            let vs = f.vertices();
            let (cu, cv) = (
                vs.iter().map(|p| p.position().x).sum::<f64>() / 3.0,
                vs.iter().map(|p| p.position().y).sum::<f64>() / 3.0,
            );
            if cu > probe[0] && cu < probe[1] && cv > probe[2] && cv < probe[3] {
                outside += 1;
            }
        }
        (cdt.num_constraints() - before, hits, outside)
    }
}
