//! Curved-face tessellation: UV-rectangle domain from the boundary
//! walk, interior grid sampling, CDT in parameter space, pole-fan
//! collapse, per-triangle certificates.
//!
//! This lane's domain contract is the swept UV rectangle: the boundary
//! polygon from [`crate::walk`] has bitwise-straight sides AND is its
//! own UV bounding box, so the domain is convex and no inside/outside
//! classification is needed — every CDT triangle is kept except
//! pole-degenerate ones, and the interior grid is strictly inside
//! every boundary constraint. Every sweep-authored face satisfies it;
//! it is not a property of iso-bounded input in general (a keyway is
//! iso-bounded and is a U), so it is CHECKED here rather than assumed
//! — [`TessellateError::UnsupportedCurvedDomain`], S28.
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
use crate::walk::{Chart, ChartKind, UvPoint, loop_polygon};

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
    // The swept-UV-rectangle contract, CHECKED rather than assumed.
    // Everything below — and the interior grid in particular — is a
    // function of this box, not of the polygon; see
    // `require_swept_rectangle`.
    require_swept_rectangle(fk, &polygon, (u0, u1, v0, v1))?;

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
    // GRID AFTER CONSTRAINTS, and it costs this lane nothing: the open
    // ranges below put every grid point strictly inside every
    // constraint BECAUSE `require_swept_rectangle` above has already
    // established that the boundary polygon IS this box. The premise,
    // not the ordering, is what carries the argument — which is why it
    // is now a refusal rather than a comment. (The hazard
    // `planar::triangulate_chart`'s header warns about is a
    // precondition of PLANAR's crossing bookkeeping, which this lane
    // does not build; S28 in `docs/SMELL-SCAN-2026-08.md` is the one
    // home for that history and for what spade does on a split.)
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

/// The indices of walk entries that do NOT lie on the boundary of the
/// UV bounding rectangle `[u0, u1] × [v0, v1]` — empty ⟺ the domain IS
/// that rectangle.
///
/// Sufficient, and it is the shape that matters: a rectilinear simple
/// polygon has a re-entrant corner exactly when some vertex sits
/// strictly inside its bounding box, and a re-entrant corner is what
/// lets an interior grid point land on — or across — a boundary
/// constraint.
///
/// The comparison is EXACT, never banded: [`crate::walk`] assigns each
/// side's constant coordinate once per EDGE (never per point), so a
/// rectangle side is bitwise straight and a near-miss here would itself
/// be the finding. It takes the caller's own bounding box rather than
/// recomputing one, so the rectangle it checks is exactly the box the
/// interior grid spans.
fn entries_off_bbox(poly: &[UvPoint], (u0, u1, v0, v1): (f64, f64, f64, f64)) -> Vec<usize> {
    poly.iter()
        .enumerate()
        .filter(|(_, e)| !(e.u == u0 || e.u == u1 || e.v == v0 || e.v == v1))
        .map(|(i, _)| i)
        .collect()
}

/// Refuses a curved face whose boundary walk is not its own UV bounding
/// rectangle — the swept-UV-rectangle contract this lane's interior
/// grid rests on, made a typed refusal.
///
/// **Why a refusal and not a comment.** The grid runs the OPEN ranges
/// `1..nu` × `1..nv` over the walk's own bounding box, which is
/// strictly interior iff the polygon IS that box. When it is not, the
/// grid splits boundary constraints and `inner_faces()` — which this
/// lane keeps wholesale, having no inside/outside classification —
/// emits triangles outside the face: a silently wrong mesh, and
/// [`fn@crate::tessellate`] does not run `check_mesh`, so it would
/// reach the caller unannounced. D2's addendum row 2 puts a
/// reachable-by-input, valid-but-unbuilt state behind a typed
/// `Unsupported*` error; D9's *"silent discard is never an answer"*
/// says the same. `trimmed` already refuses the same hazard typed
/// ([`TessellateError::SelfTouchingTrimLoop`]), and the structural
/// twin of this arm is [`TessellateError::RingOnCurvedFace`] sixty
/// lines up, whose stated reason is this very contract.
///
/// **Nothing in tree trips it** (the tests below sweep every chart this
/// build authors plus a boolean-cut face). What the check buys is that
/// the premise is enforced where it is USED: today a notched iso
/// domain is kept out of this lane only by other modules' limits — the
/// boolean refuses `CurvedPierceUnsupported`, and `import_step`'s
/// tier-3 at-rest gate refuses `PropsError::NotIsoRectangle` because
/// props' volume closed form happens to need the same rectangle. Both
/// can move without a line changing in `mesh`; this cannot.
fn require_swept_rectangle(
    fk: FaceKey,
    poly: &[UvPoint],
    bbox: (f64, f64, f64, f64),
) -> Result<(), TessellateError> {
    let off = entries_off_bbox(poly, bbox);
    if off.is_empty() {
        Ok(())
    } else {
        Err(TessellateError::UnsupportedCurvedDomain {
            face: fk,
            off_bbox: off.len(),
        })
    }
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
    //! **S28 — the swept-UV-rectangle premise this lane's grid rests
    //! on, pinned by the refusal that now enforces it.**
    //!
    //! The history (why `planar`'s ordering warning was never a claim
    //! about this lane, and what spade actually does on a split) has
    //! one home: S28 in `docs/SMELL-SCAN-2026-08.md`. These rows pin
    //! facts, not prose — the two spade behaviours the inertness
    //! argument rests on, the refusal itself, and the sweep showing
    //! nothing this build authors trips it.

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

    /// Every face this lane would take, walked to its UV polygon — the
    /// `tessellate` prologue (mesh ids, then the chord pass) run for
    /// the walk alone. The two `continue`s below are exactly the
    /// router's own screens (`tessellate.rs`), so a face that survives
    /// them is a face `tessellate_curved` receives.
    fn curved_walks(body: &Body<f64>) -> Vec<(FaceKey, Vec<UvPoint>)> {
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
            out.push((fk, poly));
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

    /// A partial-revolve sphere band — the pole-to-pole wedge whose
    /// walk is hardest (`revolves.rs`'s `survives_sphere_wedges_...`
    /// shape), absent from the sweep until the review asked for it.
    fn sphere_band(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -1.0), 1.0),
            ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta))
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

    /// Axis-touching partial wedge (`tests/common::axis_wedge`): the
    /// axis edge is an ordinary boundary edge shared by the two caps,
    /// which is one of the two shapes an earlier review lane built
    /// *because the walk is hardest there*.
    fn axis_wedge(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
        revolve(&validated(vec![lp]), axis_y(), Revolution::Partial(theta))
            .unwrap()
            .body
    }

    /// The mirror-nappe diamond (`review_m2_pr6_walk_shapes.rs`'s
    /// `diamond_profile`): a downward-opening cone under a partial
    /// revolve, so the nappe walls carry junction u/v assignments —
    /// the other hardest-walk shape.
    fn mirror_nappe(theta: f64) -> Body<f64> {
        let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 1.0), p2(1.0, 2.0)]);
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

    /// **The premise, swept.** Every curved face this build can put in
    /// front of [`tessellate_curved`] walks to a UV polygon that IS its
    /// own bounding rectangle, so [`require_swept_rectangle`] admits it
    /// — i.e. the guard this lane now carries refuses nothing in tree.
    ///
    /// Participation is asserted **per fixture**, not as a global
    /// count: `curved_walks` reaches its body through two silent
    /// `continue`s (`has_trim_carrier`, `Chart::of → None`), so a
    /// global floor lets a fixture that stopped contributing hide
    /// behind its siblings' faces. The die pip is the one this matters
    /// most for (§C10, the sweep this finding asks for): the boolean's
    /// chart re-cut is what keeps a CUT sphere face iso-rectangular,
    /// and if a future re-cut gave the cavity rim an ellipse carrier it
    /// would drop out of the sweep silently.
    #[test]
    fn every_curved_walk_is_its_own_bounding_rectangle() {
        for (name, body) in fixtures() {
            let walks = curved_walks(&body);
            assert!(
                !walks.is_empty(),
                "{name} contributes no curved walk — it is no longer sweeping this lane"
            );
            for (fk, poly) in walks {
                // Asserted through the production refusal, with the
                // predicate consulted only for the diagnostic.
                assert_eq!(
                    require_swept_rectangle(fk, &poly, bbox(&poly)),
                    Ok(()),
                    "{name} face {fk:?}: of {} walk entries, these lie strictly inside \
                     the UV bounding box: {:?}",
                    poly.len(),
                    entries_off_bbox(&poly, bbox(&poly))
                        .iter()
                        .map(|&i| (poly[i].u, poly[i].v))
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
                let mesh = crate::tessellate(&body, delta)
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

    /// **The refusal.** A notched iso domain is refused TYPED, naming
    /// both re-entrant corners; the swept rectangle passes. Asserting
    /// the production refusal rather than a predicate's return value is
    /// what makes this a guarantee about the lane instead of about a
    /// helper.
    ///
    /// The fixture is a synthetic polygon because no public
    /// construction mints such a body: the boolean refuses
    /// `CurvedPierceUnsupported`, and `import_step`'s tier-3 gate
    /// refuses `PropsError::NotIsoRectangle` before adoption (S28).
    /// That is precisely why the guard is here — the mesher is
    /// otherwise protected only by other modules' limits.
    #[test]
    fn a_notched_domain_is_refused_typed() {
        let fk = fixtures()
            .into_iter()
            .next()
            .and_then(|(_, b)| b.faces().next().map(|(fk, _)| fk))
            .unwrap();
        let rect = uv(&rectangle_domain());
        assert_eq!(require_swept_rectangle(fk, &rect, bbox(&rect)), Ok(()));
        let notch = uv(&notched_domain());
        assert_eq!(
            require_swept_rectangle(fk, &notch, bbox(&notch)),
            Err(TessellateError::UnsupportedCurvedDomain {
                face: fk,
                off_bbox: 2,
            }),
            "the two re-entrant corners are the entries strictly inside the box"
        );
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
}
