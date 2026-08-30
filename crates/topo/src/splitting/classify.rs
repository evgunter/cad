//! Reduction step (ch. 14 `splitgenerate`, re-derived): the operand
//! gate (F5 → the C5 table, M5 PR 5), the cached vertex-vs-plane
//! trilean sweep (F6), and crossing insertion through the certified
//! `split_edge` lane — with the conic crossing-root lane for
//! circle/ellipse carriers.

use geom_core::{Band, Decide, Margin, Sign, Tol};
use slotmap::SecondaryMap;

use super::{PlaneSide, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::VertexKey;
use crate::null::CurveGeom;
use crate::validate::decide;

/// The operand gate — the M3 planar gate refactored onto THE C5
/// dispatch table (M5 PR 5, C12.1): a face passes iff the split
/// pipeline executes its `(kind × plane)` arm — `Plane` (the M2/M3
/// seam, bit-identical) and `Cylinder` (the rung-2 conic lane landed
/// here). Every other kind refuses typed, **citing its rung routing**
/// (`CurvedBooleanUnsupported` retires per arm, never wholesale).
/// Edge carriers: `Line`/`Circle`/`Ellipse` pass (the crossing and
/// split lanes handle all three); `Nurbs` refuses typed (a rung-3
/// carrier in the input operand — the general rung is implemented, this
/// gate has not retired). Pre-existing null scaffolding refuses as
/// ever.
pub(super) fn gate_operand<T: Decide>(body: &Body<T>) -> Result<(), SplitReduceError> {
    for (face_key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            return Err(SplitReduceError::CurvedBooleanUnsupported {
                face: face_key,
                kind: geom_brep::SurfaceKind::Nurbs,
            });
        };
        let kind = geom_brep::SurfaceKind::of(surface);
        match kind {
            geom_brep::SurfaceKind::Plane | geom_brep::SurfaceKind::Cylinder => {}
            // `Approx` refuses HERE, by kind, rather than passing as
            // the spline its fit is: a split arm executed against the
            // fit would cut the approximation, not the surface the
            // modeller described.
            geom_brep::SurfaceKind::Cone
            | geom_brep::SurfaceKind::Sphere
            | geom_brep::SurfaceKind::Torus
            | geom_brep::SurfaceKind::Nurbs
            | geom_brep::SurfaceKind::Approx => {
                return Err(SplitReduceError::CurvedBooleanUnsupported {
                    face: face_key,
                    kind,
                });
            }
        }
    }
    for (edge_key, edge) in body.edges() {
        match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(curve)) => match curve.carrier() {
                geom::Curve3::Line { .. }
                | geom::Curve3::Circle { .. }
                | geom::Curve3::Ellipse { .. } => {}
                geom::Curve3::Nurbs(_) => {
                    return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
                }
            },
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        }
    }
    Ok(())
}

/// F6: classify every vertex against the plane through the
/// `split_vertex_side` trilean (margin = signed distance in meters,
/// linear band), caching the verdict per vertex — one predicate site,
/// one evaluation per vertex (the book recomputes per incident edge).
/// In-band ⇒ typed [`SplitReduceError::SliverVertex`], never a snap.
pub(super) fn classify_vertices<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    let mut sides = SecondaryMap::new();
    let mut on_vertices = Vec::new();
    for (vertex_key, vertex) in body.vertices() {
        let p = *body
            .get_point(vertex.point)
            .ok_or(SplitReduceError::CorruptOperand { vertex: vertex_key })?;
        let margin = Margin::of((p - plane.origin).dot(plane.normal));
        let side = match decide("split_vertex_side", margin, band) {
            Ok(Sign::Negative) => PlaneSide::Below,
            Ok(Sign::Positive) => PlaneSide::Above,
            Ok(Sign::Zero) => {
                on_vertices.push(vertex_key);
                PlaneSide::On
            }
            Err(diag) => {
                return Err(SplitReduceError::SliverVertex {
                    vertex: vertex_key,
                    diag,
                });
            }
        };
        sides.insert(vertex_key, side);
    }
    Ok((sides, on_vertices))
}

/// The plane-crossing roots of a **conic** carrier over its span
/// (M1 fix, M5 PR 5 review): the signed distance along the carrier is
/// the sinusoid `d(θ) = D + R·cos(θ − φ)` with `D = (center − q)·n̂`,
/// `R·cos φ = s_u·(û·n̂)`, `R·sin φ = s_v·(v̂·n̂)` (`s_u/s_v` the
/// semi-axes — `r/r` for a circle). Unlike a line, a conic edge can
/// cross the plane an EVEN number of times between same-side
/// endpoints (the belly case) or once beyond an ON endpoint — so
/// crossing detection is **root-based, endpoint-verdict-free**:
///
/// 1. `split_conic_belly_graze` — margin `R − |D|` (meters): Negative
///    ⇒ the carrier never meets the split plane — no crossing;
///    Positive ⇒ two distinct roots `φ ± acos(−D/R)`; Zero ⇒ the
///    plane grazes the carrier's extremum — ONE (double) root, whose
///    insertion (if in-span) leaves a same-side ON contact for the
///    sector adjudication (rule (b)'s AOA/BOB — the established graze
///    net); in-band ⇒ typed escalation (F6).
/// 2. Each root, translated into `[t₀, t₀ + τ)`, is classified
///    against the span by `split_conic_crossing_root` — the two
///    margins `(t − t₀)·meter` and `(t₁ − t)·meter` (meters at the
///    conservative minor-semi-axis meter): both Positive ⇒ a genuine
///    interior crossing (returned for insertion); any Zero ⇒ the root
///    sits at an existing endpoint vertex (the ON-endpoint belly case
///    — the vertex sweep already recorded it, nothing to insert); any
///    Negative ⇒ outside the span; in-band ⇒ typed escalation (the
///    crossing grazes an edge end — an ill-conditioned operand/plane
///    pair).
/// 3. Two interior roots are ordered ascending through
///    `split_conic_root_order` (never a raw comparison); ε-close
///    roots collapse to one insertion (Zero) or escalate (in-band).
///
/// Downstream, `split_edge`'s own interiority trilean and child
/// certification re-verify every insertion — this lane proposes,
/// never silently commits.
fn conic_crossing_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<Option<Result<Vec<T>, geom_core::Indeterminate>>, ()> {
    conic_plane_crossing_roots(carrier, t0, t1, plane.origin, plane.normal, band)
}

/// The plane-form core of [`conic_crossing_roots`], shared with the
/// boolean reduction sweep (M5 PR 9 — the same C12.1 machinery, the
/// same named trileans, against ANY plane rather than the split
/// lane's one). Semantics and return shape documented above.
pub(crate) fn conic_plane_crossing_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    plane_origin: geom_core::Point3<T>,
    plane_normal: geom_core::Vec3<T>,
    band: Band,
) -> Result<Option<Result<Vec<T>, geom_core::Indeterminate>>, ()> {
    let (center, axis, u_ref, s_u, s_v) = match *carrier {
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, u_ref, radius, radius),
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, u_ref, major, minor),
        geom::Curve3::Line { .. } | geom::Curve3::Nurbs(_) => return Err(()),
    };
    let v_ref = axis.cross(u_ref);
    let d0 = (center - plane_origin).dot(plane_normal);
    let a = u_ref.dot(plane_normal) * s_u;
    let b = v_ref.dot(plane_normal) * s_v;
    // powi, NEVER a*a: both amplitudes straddle zero on near-parallel
    // frames (a rim circle against a perpendicular side plane), and a
    // plain interval product's spurious negative low end poisons the
    // sqrt — the M2 interval-square bug class, found live here when
    // the boolean's IDEALIZED sweep (M5 PR 9) first drove this lane
    // over distant conic×plane pairs under the Interval scalar (the
    // realized lane's boxes never examine them, so only the brute
    // path escalated: a strategy divergence, the exact thing the
    // differential suite exists to catch).
    let r = (a.powi(2) + b.powi(2)).sqrt();
    // 0. The PARALLEL-frame gate (M5 S13 fix pass): a conic whose
    // plane is parallel to the query plane (both amplitudes zero)
    // either never meets it or lies wholly IN it, and both take
    // ENDPOINT treatment only — the M3 coplanar rule (interior events
    // surface via neighbor faces). Routed structurally here; the
    // coplanar sub-case previously fell through to the graze arm with
    // a 0/0 phase and escalated on an Invalid margin — loud, but
    // shapeless. The in-band twin escalates (F6).
    match decide("split_conic_plane_parallel", Margin::of(r), band) {
        Ok(Sign::Zero) => return Ok(None),
        Ok(Sign::Positive | Sign::Negative) => {}
        Err(diag) => return Ok(Some(Err(diag))),
    }
    // 1. Does the sinusoid reach zero at all — and how many roots?
    let both_roots = match decide("split_conic_belly_graze", Margin::of(r - d0.abs()), band) {
        Ok(Sign::Negative) => return Ok(None),
        Ok(Sign::Positive) => true,
        // Graze: the double root, processed once (processing both
        // would split twice at coincident parameters and escalate on
        // the second interiority check — same refusal, worse site).
        Ok(Sign::Zero) => false,
        Err(diag) => return Ok(Some(Err(diag))),
    };
    // The sinusoid's phase, branch-stabilized (M5 S13): `atan2`'s cut
    // sits on the negative-`a` axis, and an interval `b` that touches
    // zero there (a revolve-built frame's honest trig slop) explodes
    // the enclosure to a full period even though the ROOT SET —
    // everything downstream consumes phi mod τ — is unchanged. On the
    // definitely-negative-`a` frame the same phase is computed as
    // `atan2(−b, −a) + π` (identical roots mod τ, cut now on the
    // benign axis). The frame trilean is a computation choice between
    // two mathematically identical formulas, so its degenerate and
    // in-band arms keep the direct formula (a deterministic tie-break,
    // D9 — near a ≈ 0 the cut is far from both).
    let phi = match decide("split_conic_phase_frame", Margin::of(a), band) {
        Ok(Sign::Negative) => (T::zero() - b).atan2(T::zero() - a) + T::pi(),
        Ok(Sign::Positive | Sign::Zero) | Err(_) => b.atan2(a),
    };
    // Clamped acos (rounding can push the ratio a hair outside ±1 at
    // the graze boundary; min/max are Real lattice ops).
    let arg = ((T::zero() - d0) / r)
        .min(T::one())
        .max(T::zero() - T::one());
    let delta = arg.acos();
    let tau = T::tau();
    // The conservative meter (radians → meters): the minor semi-axis.
    let meter = s_v;
    let mut roots: Vec<T> = Vec::with_capacity(2);
    let candidates: [Option<T>; 2] = if both_roots {
        [Some(phi + delta), Some(phi - delta)]
    } else {
        [Some(phi + delta), None]
    };
    let half = T::from_f64(0.5);
    let mid = (t0 + t1) * half;
    // Per-candidate interiority under TWO reduction anchors (M5 S13).
    // The verdict is about `c mod τ` against `[t₀, t₁]`, but any single
    // periodic-reduction anchor has one degenerate point where the
    // Interval floor straddles an integer and the enclosure explodes to
    // a full period. The two anchors are the two windows: the midpoint
    // one is `reduce_periodic_centred` about `mid`, the fallback is
    // `reduce_periodic` about `t₀`, and their jumps are half a period
    // apart BY CONSTRUCTION, which is what makes the retry a different
    // representation rather than a second try at the same one.
    // Anchored at the span MIDPOINT the bad point is the
    // midpoint's antipode (a definitely-EXTERIOR root — the re-run of a
    // split fragment meets the full circle's opposite intersection
    // exactly there), anchored at t₀ it is t₀ itself (a
    // definitely-ON-endpoint root — the fragment's own start vertex,
    // already swept). The two bad points coincide only on a full-period
    // span (a closed edge's lone vertex on the plane), so an
    // indeterminate verdict under one anchor retries the other — the
    // SAME margin in a non-degenerate representation, never a second
    // tolerance — and only a double failure escalates (F6).
    let verdict_at = |t: T| -> Result<(bool, T), geom_core::Indeterminate> {
        for margin in [
            Margin::metered(t - t0, meter),
            Margin::metered(t1 - t, meter),
        ] {
            match decide("split_conic_crossing_root", margin, band) {
                Ok(Sign::Positive) => {}
                // At an endpoint vertex (already swept) or outside
                // the span: nothing to insert for this root.
                Ok(Sign::Zero | Sign::Negative) => return Ok((false, t)),
                Err(diag) => return Err(diag),
            }
        }
        Ok((true, t))
    };
    for c in candidates.into_iter().flatten() {
        let centred = mid + (c - mid).reduce_periodic_centred(tau);
        let (interior, t) = match verdict_at(centred) {
            Ok(v) => v,
            Err(first) => {
                let anchored = t0 + (c - t0).reduce_periodic(tau);
                match verdict_at(anchored) {
                    Ok(v) => v,
                    Err(_) => return Ok(Some(Err(first))),
                }
            }
        };
        if interior {
            roots.push(t);
        }
    }
    // Ascending insertion order, decided through the trilean door
    // (never a raw scalar comparison).
    if roots.len() == 2 {
        match decide(
            "split_conic_root_order",
            Margin::metered(roots[1] - roots[0], meter),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Negative) => roots.swap(0, 1),
            // Coincident-but-both-interior roots: a graze the graze
            // margin called Positive — insert once.
            Ok(Sign::Zero) => {
                roots.truncate(1);
            }
            Err(diag) => return Ok(Some(Err(diag))),
        }
    }
    Ok(Some(Ok(roots)))
}

/// Crossing insertion (M1 fix — even-crossing completeness):
///
/// - **Line carriers** keep the M3 rule BIT-IDENTICALLY: a crossing
///   iff the cached endpoint verdicts are strictly opposite (complete
///   for lines — the affine distance has one root, and an ON endpoint
///   IS that root), split at the exact interpolation
///   `t = t₀ + (t₁ − t₀)·d₁/(d₁ − d₂)` (comparison-free; safe: the
///   strict-opposite-signs decision bounds the denominator away from
///   zero).
/// - **Conic carriers** use the root-based lane
///   ([`conic_crossing_roots`]), INDEPENDENT of endpoint verdicts:
///   same-side endpoints with a belly crossing the plane twice get
///   BOTH crossing vertices; an ON endpoint with one interior
///   crossing gets it; grazes land as single ON contacts for the
///   rule (b) adjudication. Two roots split the parent then its
///   trailing child (ascending — the second root lives on the child's
///   span).
///
/// Both lanes are certified by `split_edge` itself (the
/// `split_edge_param_interior` trilean + full child re-certification —
/// the honest lane the raw book formula lacks). New vertices are ON
/// **by construction** (declared coincidence): their verdicts are
/// cached without re-measuring.
pub(super) fn insert_crossings<T: Decide>(
    body: &mut Body<T>,
    plane: &SplitPlane<T>,
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    on_vertices: &mut Vec<VertexKey>,
    tol: Tol,
) -> Result<(), SplitReduceError> {
    let band = geom_core::Band::linear(tol)?;
    // Snapshot: splitting adds edges; only operand edges can cross (a
    // split child's remaining crossing is handled through the parent's
    // precomputed root list below).
    let snapshot: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in snapshot {
        let start = |body: &Body<T>, he| {
            body.get_half_edge(he)
                .map(|h: &crate::entity::HalfEdge| h.start)
        };
        let u = start(body, edge.he_plus).ok_or(SplitReduceError::Euler(
            crate::euler::EulerOpError::StaleKey {
                key: crate::entity::EntityId::Edge(edge_key),
            },
        ))?;
        let v = start(body, edge.he_minus).ok_or(SplitReduceError::Euler(
            crate::euler::EulerOpError::StaleKey {
                key: crate::entity::EntityId::Edge(edge_key),
            },
        ))?;
        let curve = match body.get_curve_geom(edge.curve) {
            Some(CurveGeom::Certified(c)) => c.clone(),
            _ => return Err(SplitReduceError::ScaffoldingOperand { edge: edge_key }),
        };
        let (t0, t1) = curve.params();
        let roots: Vec<T> = match conic_crossing_roots(curve.carrier(), t0, t1, plane, band) {
            Ok(None) => continue,
            Ok(Some(Ok(roots))) => roots,
            Ok(Some(Err(diag))) => {
                return Err(SplitReduceError::CrossingEscalated {
                    edge: edge_key,
                    diag,
                });
            }
            Err(()) => {
                // Line lane (the M3 path, bit-identical): endpoint
                // verdicts strictly opposite ⇒ one interpolated root.
                let crossing = matches!(
                    (sides[u], sides[v]),
                    (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above)
                );
                if !crossing {
                    continue;
                }
                let dist = |body: &Body<T>, vk: VertexKey| -> Option<T> {
                    let p = *body.get_point(body.get_vertex(vk)?.point)?;
                    Some((p - plane.origin).dot(plane.normal))
                };
                let (Some(d1), Some(d2)) = (dist(body, u), dist(body, v)) else {
                    return Err(SplitReduceError::CorruptOperand { vertex: u });
                };
                vec![t0 + (t1 - t0) * (d1 / (d1 - d2))]
            }
        };
        // Insert ascending: the first split leaves the parent with
        // [t₀, tₐ] and mints the trailing child [tₐ, t₁]; the second
        // root (if any) lives on that CHILD.
        let mut target = edge_key;
        for t in roots {
            // Any refusal here (in practice the certification lane's
            // strict-row ResidualExceeded) gets the crossing site
            // attached; the typed Euler error stays nested whole.
            let created = body.split_edge(target, t, tol).map_err(|source| {
                SplitReduceError::CrossingInsertion {
                    edge: target,
                    endpoints: (u, v),
                    source,
                }
            })?;
            sides.insert(created.vertex, PlaneSide::On);
            on_vertices.push(created.vertex);
            target = created.new_edge;
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! DIRECT trilean rows for the conic crossing lane (M5 PR 5 fix
    //! pass, review M3): `split_conic_belly_graze` and
    //! `split_conic_crossing_root` each get their definite /
    //! exactly-degenerate / in-band arms against a pure band (the
    //! geom-core test discipline: never `Band::linear` in a lib test).

    use geom::Curve3;
    use geom_core::{Band, Point3, Vec3};

    use super::conic_crossing_roots;
    use crate::splitting::SplitPlane;

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// Unit circle in the xy-plane about the origin (meter = 1, so
    /// parameter margins ARE meters).
    fn circle() -> Curve3<f64> {
        Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
    }

    fn plane_y(c: f64) -> SplitPlane<f64> {
        SplitPlane {
            origin: Point3::new(0.0, c, 0.0),
            normal: Vec3::unit_y(),
        }
    }

    /// `split_conic_belly_graze`, all three arms: definitely-secant
    /// (two roots), definitely-missing (no roots), exactly-tangent
    /// (one graze root), and in-band (typed escalation).
    #[test]
    fn belly_graze_trio() {
        let c = circle();
        // Secant (margin R − |D| = 1, definite): the two roots of
        // sin θ = 0.5 land in the span, ascending.
        let roots = conic_crossing_roots(&c, 0.1, 6.0, &plane_y(0.5), band())
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(roots.len(), 2);
        assert!((roots[0] - core::f64::consts::FRAC_PI_6).abs() < 1e-12);
        assert!((roots[1] - (core::f64::consts::PI - core::f64::consts::FRAC_PI_6)).abs() < 1e-12);
        // Missing (margin −1, definite): no crossing at all.
        assert!(
            conic_crossing_roots(&c, 0.1, 6.0, &plane_y(2.0), band())
                .unwrap()
                .is_none()
        );
        // Exactly tangent (margin 0): ONE graze root at π/2.
        let roots = conic_crossing_roots(&c, 0.1, 6.0, &plane_y(1.0), band())
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(roots.len(), 1);
        assert!((roots[0] - core::f64::consts::FRAC_PI_2).abs() < 1e-4);
        // In-band (margin −3ε): typed escalation, named.
        let diag = conic_crossing_roots(&c, 0.1, 6.0, &plane_y(1.0 + 3e-9), band())
            .unwrap()
            .unwrap()
            .unwrap_err();
        assert_eq!(diag.predicate, Some("split_conic_belly_graze"));
    }

    /// `split_conic_crossing_root`, all three arms: definitely
    /// interior (returned), exactly at an endpoint (skipped — the
    /// vertex sweep owns it), and in-band of an endpoint (typed
    /// escalation).
    #[test]
    fn crossing_root_trio() {
        let c = circle();
        // Roots of sin θ = 0 are θ ∈ {0, π}: with span [0, 2] the θ = 0
        // root sits EXACTLY at the endpoint (skipped, Zero arm) and the
        // θ = π root is definitely interior (returned).
        let roots = conic_crossing_roots(&c, 0.0, 2.0 + 2.0, &plane_y(0.0), band())
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(roots.len(), 1);
        assert!((roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // Both roots definitely interior: span (−1, 4).
        let roots = conic_crossing_roots(&c, -1.0, 4.0, &plane_y(0.0), band())
            .unwrap()
            .unwrap()
            .unwrap();
        assert_eq!(roots.len(), 2);
        assert!(roots[0].abs() < 1e-12 || (roots[0] - core::f64::consts::PI).abs() < 1e-12);
        // In-band: a root 5e-9 (meters, meter = r = 1) inside the
        // span end — typed escalation, named.
        let diag = conic_crossing_roots(&c, -5e-9, 2.0, &plane_y(0.0), band())
            .unwrap()
            .unwrap()
            .unwrap_err();
        assert_eq!(diag.predicate, Some("split_conic_crossing_root"));
    }

    /// Line carriers refuse the conic lane (the `Err(())` sentinel the
    /// caller maps to the bit-identical M3 interpolation path).
    #[test]
    fn line_carriers_take_the_m3_lane() {
        let line = Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::unit_x(),
        };
        assert!(conic_crossing_roots(&line, 0.0, 1.0, &plane_y(0.5), band()).is_err());
    }

    /// **The midpoint anchor's straddle row, at `Interval`** (issue
    /// 1191). Nothing in the shipped suites drives either anchor into a
    /// straddle, so this row does it on the site's own numbers.
    ///
    /// A candidate root sitting ON the span's start is the ordinary
    /// case — a re-run split fragment meets the full circle at its own
    /// start vertex — and at `Interval` its enclosure straddles `t₀`.
    /// The MIDPOINT anchor's jump is half a period away from there, so
    /// the recentred parameter comes back at the width of its input and
    /// the interiority verdict is reached; the `t₀` anchor's jump is
    /// exactly on it, which is why it is the FALLBACK and not the first
    /// try. The row measures both reductions on the same box so the
    /// design's premise — two jumps, half a period apart — is pinned
    /// rather than described.
    ///
    /// Consults no tolerance: the two widths are widths, and the band
    /// is only what the site's own margins need in order to run.
    #[cfg(feature = "interval")]
    #[test]
    fn a_root_on_the_span_start_reduces_at_input_width_under_the_midpoint_anchor() {
        use geom_core::{Bounds, Interval, Real};

        let ex = Interval::from_f64;
        let c = Curve3::Circle {
            center: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            axis: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            radius: ex(1.0),
            u_ref: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
        };
        let plane = SplitPlane {
            origin: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            normal: Vec3::new(ex(0.0), ex(1.0), ex(0.0)),
        };
        // The span is the upper semicircle; the plane's two crossings
        // are its own endpoints.
        let (t0, t1) = (ex(0.0), ex(core::f64::consts::PI));
        let roots = conic_crossing_roots(&c, t0, t1, &plane, band())
            .expect("the lane runs")
            .expect("the plane cuts the circle")
            .expect("the endpoint roots classify — no anchor straddles them");
        assert!(
            roots.is_empty(),
            "both roots are the span's own endpoints and belong to the vertex sweep"
        );

        // The premise, measured: a hairline box about `t₀` is a
        // hairline under the midpoint anchor and a whole period under
        // the `t₀` anchor.
        let tau = Interval::tau();
        let mid = (t0 + t1) * ex(0.5);
        let cand = Interval::from_bounds(-1e-15, 1e-15);
        let centred = mid + (cand - mid).reduce_periodic_centred(tau);
        let anchored = t0 + (cand - t0).reduce_periodic(tau);
        assert!(
            centred.hi() - centred.lo() <= 1e-9,
            "the midpoint anchor widened: [{}, {}]",
            centred.lo(),
            centred.hi()
        );
        assert!(
            anchored.hi() - anchored.lo() >= core::f64::consts::TAU,
            "the t0 anchor is supposed to be the wide one here; it gave [{}, {}]",
            anchored.lo(),
            anchored.hi()
        );
    }
}
