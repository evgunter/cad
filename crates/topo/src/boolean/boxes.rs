//! Certified-conservative boxes for candidate generation (M5 PR 8,
//! C10) and its siblings.
//!
//! **The contract, in one sentence: every box this module returns
//! contains the entity's whole locus, or is the poison box.** The
//! trees these feed only ever PRUNE, so a box that is not a superset
//! silently loses a pair the exact predicates would have accepted; a
//! poison box overlaps everything and therefore prunes nothing, which
//! is the honest answer when no cheap superset is known. No Q1
//! predicate runs on a box; classification is untouched (`reduce`
//! module docs). [`sweep_pad`] is sized so the padding can never lose
//! an accepted pair (its derivation below).
//!
//! [`FaceBoxRule`] is the ONE statement of which surface kinds have a
//! cheap sound box and by what construction; [`face_box`] is its
//! `f64`-bracket instantiation, and `census`'s `reach_box` is its
//! scalar-lane instantiation (that module's docs carry why there are
//! two arithmetics and only one rule).
//!
//! An allowlisted [`geom_core::Bounds`] seam (ratified 2026-07-29 —
//! see geom-core `real.rs`, Bounds scope rule; the C10 tree is the
//! subdivision driver): coordinates enter as `[lo(), hi()]` brackets,
//! poison flows to the poison box, which never prunes.

use bvh::Aabb;
use geom_core::{Band, Bounds, Decide, Point3, Real, Vec3};
use geom_surfaces::Surface;
use geom_surfaces::nurbs::NurbsSurface;

use super::BooleanError;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, LoopKey, VertexKey};

/// The sweep's box pad in meters — what candidate generation must add
/// so pruning can never lose an accepted pair. Derivation (each term
/// against the sweep's accept conditions in `reduce`):
///
/// - an event point classifies ON the face plane / boundary only
///   within `band.zero()` (`bool_vertex_face_side`, `contfp`'s
///   `bool_contact_*` sites) — one `zero` for the point-to-face gap;
/// - vertices sit on their carriers only up to attachment-time
///   certification (`Certificate::max_residual ≤ ε`, the same run
///   tolerance the linear band's `zero` is built from) — one more
///   `zero` per side for vertex-extent honesty;
/// - `band.escalate()` on top dominates every remaining f64 slop
///   (crossing-point interpolation and `eval` rounding are
///   session-box-scale ulps, orders below it) and keeps near-boundary
///   escalation zones inside candidate range.
///
/// The sum is deliberately generous — a bigger pad only admits more
/// candidates (conservative direction); it never changes any answer
/// (the differential suite pins that).
pub(crate) fn sweep_pad(band: Band) -> f64 {
    band.escalate() + 2.0 * band.zero()
}

fn corrupt(what: &'static str) -> BooleanError {
    BooleanError::ClassificationInvariant { what }
}

/// **The one soundness rule for a face's box**, stated per surface
/// kind: which cheap construction yields a genuine SUPERSET of the
/// face's locus. Every consumer that bounds a face reads its arm from
/// here, so no two of them can quietly disagree about which kinds are
/// boxable.
///
/// The variants carry the surface payload the construction needs, so
/// the kind is matched ONCE and each lane only performs its own
/// arithmetic. The soundness argument per arm:
///
/// - [`BoundaryHull`](Self::BoundaryHull) — **Plane.** A planar face
///   lies in the convex hull of its boundary, so the hull of the
///   boundary's own certified boxes contains it whatever the boundary
///   curves are. The hull of the boundary VERTICES alone does not: a
///   circular rim bulges past its endpoints, and this engine's
///   plane×cylinder lane mints exactly that face.
/// - [`CylinderSlab`](Self::CylinderSlab) — **Cylinder.** The wall's
///   belly bulges past its chords, so the box is the whole cylinder
///   slab over the face's axial range (the axial coordinate is linear
///   along the surface, so the face's axial extremes lie on its
///   boundary), widened by the full radius in every coordinate.
/// - [`WholeBall`](Self::WholeBall) — **Sphere.** A band's belly
///   bulges past its poles and seam arcs, so the box is the whole ball
///   `center ± r`; every surface point is within `r` of the center.
/// - [`ControlNet`](Self::ControlNet) — **NURBS.** The patch bulges
///   past the hull of its boundary exactly as the sphere does, but it
///   lies in the hull of its CONTROL NET (nonnegative basis, strictly
///   positive weights — `geom_surfaces::boxes::nurbs_surface_aabb`
///   carries the citation), over the whole domain and a fortiori over
///   any trim.
/// - [`NoSoundBox`](Self::NoSoundBox) — **Cone, Torus**, and a face
///   whose surface is missing. No cheap superset is known, so nothing
///   is claimed: the box is poison (never prunes) where a box is
///   built, and a refusal where a certificate is wanted.
///
/// Both loose arms are loose in the conservative direction on purpose:
/// a bigger box only admits candidates.
pub(crate) enum FaceBoxRule<'a, T: Real> {
    /// Hull the boundary's certified loci — see the type docs.
    BoundaryHull,
    /// The axial slab widened by the radius — see the type docs.
    CylinderSlab {
        /// The `v = 0` point on the axis.
        origin: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The cylinder's radius.
        radius: T,
    },
    /// The whole ball `center ± r` — see the type docs.
    WholeBall {
        /// The sphere's center.
        center: Point3<T>,
        /// The sphere's radius.
        radius: T,
    },
    /// The control net's hull — see the type docs.
    ControlNet(&'a NurbsSurface<T>),
    /// No cheap superset exists — see the type docs.
    NoSoundBox,
}

/// The [`FaceBoxRule`] for a face's surface — the single kind→rule
/// mapping. A kind added to [`Surface`] lands on
/// [`FaceBoxRule::NoSoundBox`] only by being written here, never by
/// falling through a wildcard in some consumer.
pub(crate) fn face_box_rule<T: Real>(surface: Option<&Surface<T>>) -> FaceBoxRule<'_, T> {
    match surface {
        Some(Surface::Plane { .. }) => FaceBoxRule::BoundaryHull,
        Some(Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        }) => FaceBoxRule::CylinderSlab {
            origin: *origin,
            axis: *axis,
            radius: *radius,
        },
        Some(Surface::Sphere { center, radius, .. }) => FaceBoxRule::WholeBall {
            center: *center,
            radius: *radius,
        },
        Some(Surface::Nurbs(patch)) => FaceBoxRule::ControlNet(patch),
        Some(Surface::Cone { .. } | Surface::Torus { .. }) | None => FaceBoxRule::NoSoundBox,
    }
}

/// The face's certified box, padded — [`FaceBoxRule`]'s
/// `f64`-bracket instantiation, and therefore a genuine superset of
/// the face's locus for every kind that has one, the poison box for
/// every kind that does not.
///
/// Poison here is not a refusal: it overlaps everything, so a face
/// whose kind has no cheap superset is simply never pruned and reaches
/// the exact predicates — which is where a `Cone`/`Torus` operand
/// meets its typed refusal anyway.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when the face's topology
/// is corrupt (a lost entity, an unwalkable loop).
pub(crate) fn face_box<T: Decide + Bounds>(
    body: &Body<T>,
    face: FaceKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let f = body.get_face(face).ok_or(corrupt("face box: face lost"))?;
    let boxed = match face_box_rule(body.get_surface(f.surface)) {
        FaceBoxRule::NoSoundBox => return Ok(Aabb::poison()),
        FaceBoxRule::ControlNet(patch) => geom_surfaces::boxes::nurbs_surface_aabb(patch),
        FaceBoxRule::WholeBall { center, radius } => {
            let r = radius.hi();
            Aabb {
                min_x: center.x.lo() - r,
                min_y: center.y.lo() - r,
                min_z: center.z.lo() - r,
                max_x: center.x.hi() + r,
                max_y: center.y.hi() + r,
                max_z: center.z.hi() + r,
            }
        }
        FaceBoxRule::CylinderSlab {
            origin,
            axis,
            radius,
        } => {
            let radius = radius.hi();
            let mut h_min = f64::INFINITY;
            let mut h_max = f64::NEG_INFINITY;
            for ek in boundary_edges(body, f)? {
                let eb = edge_box(body, ek, 0.0)?;
                for &(x, y, z) in &[
                    (eb.min_x, eb.min_y, eb.min_z),
                    (eb.max_x, eb.max_y, eb.max_z),
                    (eb.min_x, eb.min_y, eb.max_z),
                    (eb.min_x, eb.max_y, eb.min_z),
                    (eb.max_x, eb.min_y, eb.min_z),
                    (eb.min_x, eb.max_y, eb.max_z),
                    (eb.max_x, eb.min_y, eb.max_z),
                    (eb.max_x, eb.max_y, eb.min_z),
                ] {
                    let h = (x - origin.x.lo()) * axis.x.lo()
                        + (y - origin.y.lo()) * axis.y.lo()
                        + (z - origin.z.lo()) * axis.z.lo();
                    h_min = h_min.min(h);
                    h_max = h_max.max(h);
                }
            }
            if !(h_min.is_finite() && h_max.is_finite()) {
                return Ok(Aabb::poison());
            }
            // Pad the axial range too (interval-lane bracket slop is
            // dominated by the pad; conservative direction).
            h_min -= pad;
            h_max += pad;
            let along = |c: f64, a: f64| (c + a * h_min, c + a * h_max);
            let (x0, x1) = along(origin.x.lo(), axis.x.lo());
            let (y0, y1) = along(origin.y.lo(), axis.y.lo());
            let (z0, z1) = along(origin.z.lo(), axis.z.lo());
            Aabb {
                min_x: x0.min(x1) - radius,
                min_y: y0.min(y1) - radius,
                min_z: z0.min(z1) - radius,
                max_x: x0.max(x1) + radius,
                max_y: y0.max(y1) + radius,
                max_z: z0.max(z1) + radius,
            }
        }
        FaceBoxRule::BoundaryHull => {
            // Every boundary edge's own certified box, plus the
            // isolated-vertex loops (which have no edge to speak for
            // them).
            let mut b: Option<Aabb> = None;
            let mut grow = |x: Aabb| b = Some(b.map_or(x, |acc: Aabb| acc.hull(&x)));
            for lk in loops_of(f) {
                let l = body.get_loop(lk).ok_or(corrupt("face box: loop lost"))?;
                match l.boundary {
                    LoopBoundary::Empty { vertex } => {
                        let p = vertex_point(body, vertex)?;
                        grow(Aabb::from_points([p]).unwrap_or_else(Aabb::poison));
                    }
                    LoopBoundary::Cycle { first } => {
                        for he in body
                            .loop_cycle(first)
                            .ok_or(corrupt("face box: unwalkable loop"))?
                        {
                            let ek = body
                                .get_half_edge(he)
                                .ok_or(corrupt("face box: half-edge lost"))?
                                .edge;
                            grow(edge_box(body, ek, 0.0)?);
                        }
                    }
                }
            }
            b.unwrap_or_else(Aabb::poison)
        }
    };
    Ok(boxed.padded(pad))
}

/// A face's loop keys, outer first — the walk order every arm here
/// shares (D9: fixed, so two boxes of one face fold identically).
fn loops_of(f: &crate::entity::Face) -> impl Iterator<Item = LoopKey> + '_ {
    core::iter::once(f.outer).chain(f.rings.iter().copied())
}

/// Every edge on the face's boundary, in [`loops_of`] order.
fn boundary_edges<T: Decide + Bounds>(
    body: &Body<T>,
    f: &crate::entity::Face,
) -> Result<Vec<EdgeKey>, BooleanError> {
    let mut out = Vec::new();
    for lk in loops_of(f) {
        let l = body.get_loop(lk).ok_or(corrupt("face box: loop lost"))?;
        let LoopBoundary::Cycle { first } = l.boundary else {
            continue;
        };
        for he in body
            .loop_cycle(first)
            .ok_or(corrupt("face box: unwalkable loop"))?
        {
            out.push(
                body.get_half_edge(he)
                    .ok_or(corrupt("face box: half-edge lost"))?
                    .edge,
            );
        }
    }
    Ok(out)
}

/// The edge's certified box, padded — the module contract's
/// curve-side arm: a superset of the edge's locus, or poison.
///
/// - `Line`: the two endpoints (the locus is the chord up to
///   certification residual — inside the pad).
/// - `Circle`, `Ellipse`: the FULL conic's center-±-amplitude box
///   hulled with the endpoints — a superset of any arc of it (an
///   arc's belly bulges past its chord; the full-turn box is
///   deliberately loose, the conservative direction).
/// - `Nurbs`, and an edge whose carrier is null scaffolding: poison
///   (never prunes). Nothing is certified about the locus, so nothing
///   is claimed — the chord is NOT a bound for either.
///
/// The `Nurbs` arm is the ONE place a sound cheap box exists and is
/// deliberately not taken: `geom_curves::boxes::nurbs_curve_aabb`
/// would give the control-net hull, exactly as the face rule's
/// [`FaceBoxRule::ControlNet`] arm does. Taking it would TIGHTEN this
/// box — it would start pruning pairs that are examined today — and
/// tightening is a different obligation from soundness: a rung-3
/// operand gate has to admit the kind first. Poison is already the
/// conservative answer, so nothing is unsound while it waits.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when the edge's topology
/// is corrupt.
pub(crate) fn edge_box<T: Decide + Bounds>(
    body: &Body<T>,
    edge: EdgeKey,
    pad: f64,
) -> Result<Aabb, BooleanError> {
    let e = body.get_edge(edge).ok_or(corrupt("edge box: edge lost"))?;
    let start_of = |he| -> Result<Point3<T>, BooleanError> {
        let vk = body
            .get_half_edge(he)
            .ok_or(corrupt("edge box: half-edge lost"))?
            .start;
        vertex_point(body, vk)
    };
    let (a, b) = (start_of(e.he_plus)?, start_of(e.he_minus)?);
    let vertex_box = Aabb::from_points([a, b]).unwrap_or_else(Aabb::poison);
    let Some(carrier) = body
        .get_curve_geom(e.curve)
        .and_then(crate::null::CurveGeom::certified)
        .map(geom_brep::EdgeCurve::carrier)
    else {
        return Ok(Aabb::poison());
    };
    let conic = match carrier {
        geom_curves::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Some((*center, *axis, *radius, *radius, *u_ref)),
        geom_curves::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => Some((*center, *axis, *major, *minor, *u_ref)),
        geom_curves::Curve3::Nurbs(_) => return Ok(Aabb::poison()),
        geom_curves::Curve3::Line { .. } => None,
    };
    let boxed = match conic {
        None => vertex_box,
        Some((c, axis, sa, sb, u_ref)) => {
            // Per-coordinate amplitude |û_i|·a + |v̂_i|·b of the full
            // conic (v̂ = axis × û), hulled with the endpoint box.
            let v_ref = axis.cross(u_ref);
            let reach = |ui: f64, vi: f64| ui.abs() * sa.hi() + vi.abs() * sb.hi();
            let rx = reach(u_ref.x.hi(), v_ref.x.hi());
            let ry = reach(u_ref.y.hi(), v_ref.y.hi());
            let rz = reach(u_ref.z.hi(), v_ref.z.hi());
            let full = Aabb {
                min_x: c.x.lo() - rx,
                min_y: c.y.lo() - ry,
                min_z: c.z.lo() - rz,
                max_x: c.x.hi() + rx,
                max_y: c.y.hi() + ry,
                max_z: c.z.hi() + rz,
            };
            Aabb {
                min_x: full.min_x.min(vertex_box.min_x),
                min_y: full.min_y.min(vertex_box.min_y),
                min_z: full.min_z.min(vertex_box.min_z),
                max_x: full.max_x.max(vertex_box.max_x),
                max_y: full.max_y.max(vertex_box.max_y),
                max_z: full.max_z.max(vertex_box.max_z),
            }
        }
    };
    Ok(boxed.padded(pad))
}

fn vertex_point<T: Decide + Bounds>(
    body: &Body<T>,
    v: VertexKey,
) -> Result<Point3<T>, BooleanError> {
    body.get_vertex(v)
        .and_then(|vd| body.get_point(vd.point))
        .copied()
        .ok_or(corrupt("face/edge box: vertex point lost"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! **The superset contract, asserted against the locus itself.**
    //!
    //! Every row here samples the face's TRUE locus and asserts the box
    //! contains every sample. That is the contract (module docs), and
    //! it is the assertion that degrades correctly: a rule that drops
    //! any part of the bulge goes red, not only the one that drops all
    //! of it, and the spans are swept rather than chosen so no single
    //! fixture can be the reason it passes.

    use super::*;
    use crate::euler::{FaceSurface, MefSite, MevSite};
    use geom_brep::{EdgeCurveSpec, EdgeGeometry};
    use geom_core::{Point3, Vec3};
    use geom_curves::Curve3;
    use geom_surfaces::Surface;

    /// The pad every row boxes with — the sweep's own, so a row that
    /// only passes because of a generous pad would have to say so.
    fn pad() -> f64 {
        sweep_pad(Band::linear().unwrap())
    }

    /// `p` is inside `b` — the containment the contract promises.
    fn holds(b: &Aabb, p: Point3<f64>) -> bool {
        p.x >= b.min_x
            && p.x <= b.max_x
            && p.y >= b.min_y
            && p.y <= b.max_y
            && p.z >= b.min_z
            && p.z <= b.max_z
    }

    fn plane_z0() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
    }

    fn cyl_r(r: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }
    }

    /// A PLANAR face whose rim is a circular arc: the sector of radius
    /// `r` spanning `[0, span]` in azimuth, closed by two radii. This
    /// is the shape the plane×cylinder lane mints as a cylinder's cap,
    /// and the shape whose locus leaves its boundary-vertex hull.
    ///
    /// Returns the body and the sector face.
    fn arc_sector(r: f64, span: f64) -> (Body<f64>, FaceKey) {
        let on = |t: f64| Point3::new(r * t.cos(), r * t.sin(), 0.0);
        let (a, b, c) = (on(0.0), on(span), Point3::origin());
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(a).unwrap();
        let plane = body.add_surface(plane_z0());
        let cyl = body.add_surface(cyl_r(r));
        let arc = EdgeCurveSpec {
            description: EdgeGeometry::Intersection {
                s1: plane,
                s2: cyl,
                witness: on(span * 0.5),
            },
            carrier: Curve3::Circle {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: r,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: span,
        };
        let e_ab = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                b,
                arc,
            )
            .unwrap();
        let e_bc = body
            .mev_line(
                MevSite::Fan {
                    he1: e_ab.he_minus,
                    he2: e_ab.he_minus,
                },
                c,
            )
            .unwrap();
        let he = body
            .find_half_edge(seed.face, e_bc.vertex, e_ab.vertex)
            .unwrap();
        let face = body
            .mef(
                MefSite::Chords {
                    he1: he,
                    he2: e_ab.he_plus,
                },
                EdgeCurveSpec::line_between(c, a),
                FaceSurface::Shared(plane),
            )
            .unwrap()
            .face;
        (body, face)
    }

    /// **The reported defect.** A planar face's rim bulges past its
    /// boundary VERTICES, so a vertex-hull box is not a superset and
    /// `Bvh::overlapping` can prune a pair the exact predicates would
    /// have accepted.
    ///
    /// The span is swept from a shallow arc to a reflex one, and the
    /// radius with it: the miss grows with the sagitta, so a rule that
    /// covers only part of the bulge fails at the larger spans while
    /// passing the small ones. A single fixture cannot be the reason
    /// this row is green.
    #[test]
    fn a_planar_faces_circular_rim_is_inside_its_box() {
        for &r in &[0.001, 1.0, 250.0] {
            for span_deg in [10.0_f64, 90.0, 179.0, 181.0, 300.0, 359.0] {
                let span = span_deg.to_radians();
                let (body, face) = arc_sector(r, span);
                let b = face_box(&body, face, pad()).unwrap();
                // The locus is the convex hull of its boundary and the
                // box is convex, so sampling the boundary settles it.
                for i in 0..=512 {
                    let t = span * f64::from(i) / 512.0;
                    let p = Point3::new(r * t.cos(), r * t.sin(), 0.0);
                    assert!(
                        holds(&b, p),
                        "rim point at {t} rad left the box (r = {r}, span = {span_deg}°): {b:?}"
                    );
                }
            }
        }
    }

    /// The same claim stated as a margin: the box must reach the arc's
    /// extreme in the direction the vertex hull cannot see. A
    /// half-turn sector's rim tops out at `y = r` while both its
    /// vertices sit at `y ≤ 0`, so a vertex-hull box misses by `r` —
    /// this row measures that gap rather than trusting a sample to
    /// land on it.
    #[test]
    fn the_boxs_reach_beyond_the_vertex_hull_is_the_whole_bulge() {
        let r = 2.0;
        let (body, face) = arc_sector(r, core::f64::consts::PI);
        let b = face_box(&body, face, pad()).unwrap();
        // Both boundary vertices and the sector's centre are at y ≤ 0;
        // the rim reaches y = r.
        assert!(
            b.max_y >= r,
            "the box must reach the rim's extreme y = {r}, got {}",
            b.max_y
        );
    }

    /// **The NURBS half of the same defect.** A patch's interior
    /// bulges past the hull of its boundary — here a biquadratic whose
    /// boundary lies entirely in `z = 0` while its centre control
    /// point lifts the surface to `z = 1/4`. The control-net hull
    /// contains it; the boundary hull does not.
    #[test]
    fn a_nurbs_patchs_interior_bulge_is_inside_its_box() {
        use geom_core::spline::KnotVector;
        use geom_surfaces::nurbs::NurbsSurface;
        let kv = KnotVector::unit_segment(2);
        let p = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
        let control = vec![
            p(0.0, 0.0, 0.0),
            p(0.0, 0.5, 0.0),
            p(0.0, 1.0, 0.0),
            p(0.5, 0.0, 0.0),
            p(0.5, 0.5, 1.0),
            p(0.5, 1.0, 0.0),
            p(1.0, 0.0, 0.0),
            p(1.0, 0.5, 0.0),
            p(1.0, 1.0, 0.0),
        ];
        let patch = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap();
        let surface = Surface::Nurbs(std::sync::Arc::new(patch));
        // The lifted interior, on the surface itself — its boundary
        // curves all lie in `z = 0`, so no hull of the BOUNDARY can
        // contain this point.
        let mid = surface.eval(0.5, 0.5);
        assert!(mid.z > 0.2, "the fixture must actually bulge, got {mid:?}");
        assert!(
            surface.eval(0.0, 0.5).z.abs() < 1e-15,
            "the fixture's boundary must lie in z = 0"
        );

        let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
        body.set_face_surface(face, FaceSurface::New(surface))
            .unwrap();
        let b = face_box(&body, face, pad()).unwrap();
        assert!(
            holds(&b, mid),
            "the patch's own interior point left its box: {b:?}"
        );
    }

    /// A kind with no cheap sound box claims NOTHING: the poison box,
    /// which overlaps everything and therefore prunes nothing. The
    /// alternative — a hull of the boundary — would be a claim the
    /// kernel cannot make for a cone or a torus.
    #[test]
    fn kinds_without_a_sound_box_are_poison_and_never_prune() {
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let torus = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        for s in [cone, torus] {
            let (mut body, face) = arc_sector(1.0, core::f64::consts::PI);
            body.set_face_surface(face, FaceSurface::New(s)).unwrap();
            let b = face_box(&body, face, pad()).unwrap();
            assert!(b.min_x.is_nan(), "an unboxable kind must poison: {b:?}");
            assert!(
                b.overlaps(&Aabb {
                    min_x: 1e6,
                    min_y: 1e6,
                    min_z: 1e6,
                    max_x: 2e6,
                    max_y: 2e6,
                    max_z: 2e6,
                }),
                "poison must never prune"
            );
        }
    }
}
