//! `split_edge` — the named lmev edge-split idiom (M3 PR 1): split an
//! edge at a definitely-interior carrier parameter, producing two edges
//! sharing a new vertex, each carrying a **certified sub-interval**
//! description of the parent's geometry.
//!
//! Ch. 14's reduction step inserts every proper plane crossing with the
//! book's `lmev(e->he1, e->he2->nxt, p)` — an idiom whose argument pair
//! isolates exactly one half-edge and whose correctness the notes flag
//! as a transcription hazard. This module makes it a first-class
//! operator instead: the surgery is re-derived under our interior-left
//! convention (below), the split point arrives as a **carrier
//! parameter** (the reduction computes it as `t = t₁·d₁/(d₁ − d₂)`-style
//! interpolation), and the geometry is handled honestly — the parent's
//! carrier is unchanged, the interval splits at `t`, each child's
//! description is the parent's restricted to its sub-interval
//! ([`geom_brep::EdgeCurve::split_specs`]), and **both children pass
//! the full certification gate before any mutation** (D4 ¶2; the
//! restriction arithmetic is verified, never trusted). The parent's
//! description-adjacency obligations transfer unchanged: children keep
//! the parent's description kind and surface keys, and the adjacent
//! faces are untouched by the surgery.
//!
//! Serves ch. 14 `splitgenerate` edge splitting and ch. 15's edge×face
//! reduction sweep (M3 PRs 2 and 4).

use geom_brep::CertifyError;
use geom_core::{Band, Decide, Margin, Sign, Tol};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, GeomRef, HalfEdgeKey, VertexKey};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::EulerOpError;
use crate::geometry::{CurveKey, PointKey};
use crate::provenance::Provenance;

/// Every key minted (and the one possibly killed) by one
/// [`Body::split_edge`] call.
///
/// Direction conventions: the parent edge survives as the **first
/// child** (`start(he_plus)` → the new vertex, carrier interval
/// `[t₀, t]` — its `he_plus`/`he_minus` keys are unchanged); the new
/// edge is the **second child** (new vertex → the parent's old end,
/// interval `[t, t₁]`), its `he_plus` starting at the new vertex — so
/// both children run **forward on the parent's carrier** and the
/// he_plus forward contract holds with no curve reversal anywhere.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitEdgeCreated {
    /// The new interior vertex (`emanating` = `he_plus`, the second
    /// child's plus half).
    pub vertex: VertexKey,
    /// The new point: the carrier evaluated at the split parameter.
    pub point: PointKey,
    /// The new (second-child) edge, new vertex → old end.
    pub new_edge: EdgeKey,
    /// The second child's plus half: new vertex → old end, spliced
    /// after the parent's plus half in its loop.
    pub he_plus: HalfEdgeKey,
    /// The second child's minus half: old end → new vertex, spliced
    /// before the parent's minus half in its loop.
    pub he_minus: HalfEdgeKey,
    /// The parent's replacement curve (certified `[t₀, t]` child).
    pub first_curve: CurveKey,
    /// The new edge's curve (certified `[t, t₁]` child).
    pub second_curve: CurveKey,
    /// The parent's original curve (dead key) iff killing it orphaned
    /// it (it always does in operator-built bodies; the scan keeps the
    /// op sound standalone).
    pub killed_curve: Option<CurveKey>,
}

impl<T: Decide> Body<T> {
    /// Splits `edge` at carrier parameter `t`, minting a new vertex at
    /// `carrier(t)` and a new edge so that the parent covers `[t₀, t]`
    /// and the new edge `[t, t₁]` of the unchanged carrier (type docs
    /// for the direction conventions; module docs for the ch. 14/15
    /// consumers).
    ///
    /// Euler vector: `(v +1, e +1, f 0, h 0, r 0, s 0)` — `mev`'s
    /// vector (an edge split IS a mev, applied mid-edge).
    ///
    /// **Interiority is trilean** (Q1): the two sub-spans `t − t₀` and
    /// `t₁ − t` are metered into meters exactly like the certification
    /// span gate (`·radius` for circle carriers) and each must classify
    /// **definitely positive** through the
    /// `split_edge_param_interior` predicate — at/outside an endpoint
    /// refuses ([`EulerOpError::SplitParamNotInterior`]), in-band
    /// escalates ([`EulerOpError::SplitParamEscalated`]; a poisoned
    /// margin, e.g. an unmeterable carrier, escalates the same way).
    ///
    /// **Surgery** (derived from the interior-left rule, replacing the
    /// book's `lmev(e->he1, e->he2->nxt, p)`): with parent halves
    /// `hp = he_plus` (u → v) and `hm = he_minus` (v → u), the new
    /// plus half is spliced immediately **after `hp`** (in `hp`'s
    /// loop) and the new minus half immediately **before `hm`** (in
    /// `hm`'s loop); `hm`'s start is rebound to the new vertex `w`.
    /// `hp` then derives its end as `w` — the parent becomes the
    /// first child with no further pointer changes. Degenerate sites
    /// fall out of the same two splices: a strut
    /// (`next(hp) == hm`) becomes the two-edge strut
    /// `… hp → n⁺ → n⁻ → hm …`, and self-loop parents (u = v,
    /// including one-half-edge circular loops) split into a two-edge
    /// digon — the second splice reads `prev(hm)` **after** the first
    /// splice precisely so these coincidence cases land correctly.
    ///
    /// **Emanating rules** (deterministic): `w.emanating = n⁺` (the
    /// second child's plus half — the analogue of `mev`'s new-vertex
    /// rule); `v.emanating` is rebound to `n⁻` iff it was `hm` (which
    /// no longer starts at v); all other anchors are untouched.
    ///
    /// **Minting order** (D9, exact): point, first-child curve,
    /// second-child curve, vertex, edge, `he_plus`, `he_minus`. **Kill
    /// order**: the parent's curve iff orphaned, after the parent edge
    /// is repointed to the first-child curve.
    ///
    /// # Precondition check order
    ///
    /// `edge` resolves ([`EulerOpError::StaleKey`]); its halves and
    /// the two splice neighbours `next(he_plus)` / `prev(he_minus)`
    /// resolve (`StaleKey`); its curve entry resolves
    /// ([`EulerOpError::StaleGeometry`]) and is certified
    /// ([`EulerOpError::NullScaffoldCurve`] — null scaffolding has
    /// nothing to split); both endpoint vertices and their points
    /// resolve (`StaleKey`/`StaleGeometry`); the interiority trilean
    /// (above); both child specs certify
    /// ([`EulerOpError::Certification`] — endpoints
    /// `start(hp) → carrier(t)` and `carrier(t) → end(hp)`, he_plus
    /// forward order on each child).
    ///
    /// # Tier-3 caveat (review F2)
    ///
    /// Splitting a circle rim of an iso-rectangle patch (e.g. a
    /// plane×cylinder intersection rim on an extruded disc) leaves the
    /// curved wall's loop no longer an iso-rectangle: mass properties
    /// / tier 3 then refuse **typed** (`NotIsoRectangle`) on a body
    /// that was tier-3 before the split. Honest and loud, but the
    /// split knocks the body out of the props inventory until the
    /// patch machinery generalizes (pinned in
    /// `review_m3_pr1_sweep.rs::split_circle_carrier_intersection_edge`).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn split_edge(
        &mut self,
        edge: EdgeKey,
        t: T,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let edge_data = self.get_edge(edge).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let (hp, hm) = (edge_data.he_plus, edge_data.he_minus);
        let (hp, hp_data) = self.resolve_half_edge_live(hp)?;
        let (hm, hm_data) = self.resolve_half_edge_live(hm)?;
        // The two splices write through `next(hp)` and `prev(hm)` as
        // well as the parent's own halves, whose proofs came out of the
        // resolves above; prove these two now so the mutation below
        // cannot fail midway (atomicity). `prev(hm)` changes
        // under splice 1 — see the splice for the case that moves it,
        // and for why the new value is proven too.
        let hp_next = self.require_live(hp_data.next)?;
        let hm_prev = self.require_live(hm_data.prev)?;
        let entry = self
            .get_curve_geom(edge_data.curve)
            .ok_or(EulerOpError::StaleGeometry {
                key: GeomRef::Curve(edge_data.curve),
            })?;
        let curve = entry
            .certified()
            .cloned()
            .ok_or(EulerOpError::NullScaffoldCurve {
                curve: edge_data.curve,
            })?;
        // Interiority (trilean, Q1): both sub-spans definitely
        // positive, metered in meters like the certification span gate.
        let (t0, t1) = curve.params();
        let scale = match *curve.carrier() {
            geom::Curve3::Line { .. } => T::one(),
            geom::Curve3::Circle { radius, .. } => radius,
            // The conic lane (M5 PR 5, C12.3): metered at the MINOR
            // semi-axis — the conservative meter (|dP/dθ| ≥ minor), so
            // a sub-span this gate accepts as definitely interior is
            // truly clear of the endpoints in meters.
            geom::Curve3::Ellipse { minor, .. } => minor,
            // The general rung (M5 PR 7, C12.3): a fitted SSI carrier
            // is metered at the CERTIFIED LOWER BOUND on ‖C′(t)‖ —
            // the same conservative posture as the conic lane's minor
            // semi-axis, derived from the derivative control net's
            // convex-hull property for an integral net, and (since M7)
            // from the quotient-rule assembly over the HOMOGENEOUS net
            // for a rational one — see `NurbsCurve3::speed_lower_bound`
            // for both derivations. A carrier whose speed genuinely
            // collapses yields a non-positive or poison meter, and the
            // interiority trilean below then escalates honestly instead
            // of accepting a split that is not clear of the endpoints
            // in meters.
            geom::Curve3::Nurbs(ref n) => n.speed_lower_bound(),
        };
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;
        for margin in [
            Margin::metered(t - t0, scale),
            Margin::metered(t1 - t, scale),
        ] {
            match geom_core::k_stats::decide("split_edge_param_interior", margin, band) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Zero | Sign::Negative) => {
                    return Err(EulerOpError::SplitParamNotInterior { edge });
                }
                Err(diag) => {
                    return Err(EulerOpError::SplitParamEscalated { edge, diag });
                }
            }
        }
        let (u, v) = (hp_data.start, hm_data.start);
        let p_u = self.resolve_vertex_point(u)?;
        let p_v = self.resolve_vertex_point(v)?;
        let p_new = curve.carrier().eval(t);
        // ---- Geometry gate (still no mutation): both children must
        // certify against their own endpoints.
        let (spec1, spec2) = curve.split_specs(t);
        let cert1 = self.certify_edge_spec(spec1, p_u, p_new, tol)?;
        let cert2 = self.certify_edge_spec(spec2, p_new, p_v, tol)?;

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented above): point, curve1, curve2,
        // vertex, edge, he_plus, he_minus.
        let provenance = Provenance::SplitEdge { edge };
        let point = self.add_point(p_new);
        let first_curve = self.add_curve(cert1);
        let second_curve = self.add_curve(cert2);
        let w = self.add_vertex(
            crate::entity::Vertex {
                point,
                emanating: None, // patched below
            },
            provenance.clone(),
        );
        let new_edge = self.mint_edge(second_curve, &provenance);
        let (n_plus, n_minus) = self.mint_halves(
            new_edge,
            // Second child's plus half: w → v, in hp's loop.
            (w, hp_data.parent_loop),
            // Second child's minus half: v → w, in hm's loop.
            (v, hm_data.parent_loop),
            &provenance,
        );
        // Splice 1: hp → n⁺ → old next(hp), in hp's loop.
        self.link_half_edges(hp, n_plus);
        self.link_half_edges(n_plus, hp_next);
        // Splice 2: current prev(hm) → n⁻ → hm, in hm's loop. Splice 1
        // moved that prev in the strut case (next(hp) == hm ⇒ prev(hm)
        // is now n⁺), and the two cases are exhaustive: splice 1 writes
        // exactly two `prev` fields, `n_plus`'s and `hp_next`'s, and
        // `hm != n_plus` because `n_plus` was minted in this call while
        // `hm` came out of the arena — so the only one that can be
        // `hm`'s is `hp_next`'s. Deriving the value rather than
        // re-reading it keeps both branches proven: the mint above, and
        // the plan phase.
        let hm_prev = if hm == hp_next { n_plus } else { hm_prev };
        self.link_half_edges(hm_prev, n_minus);
        self.link_half_edges(n_minus, hm);
        // The splice is done; past it the new halves are ordinary keys.
        let (n_plus, n_minus) = (n_plus.key(), n_minus.key());
        // The parent's minus half now starts at w (the parent derives
        // its new end w through n⁺/n⁻'s starts).
        let Some(he) = self.get_half_edge_mut(hm.key()) else {
            unreachable!(
                "split_edge: `hm` resolved in the plan phase and this op kills no half-edge"
            )
        };
        he.start = w;
        // Parent edge → first-child curve; old curve killed iff
        // orphaned.
        let Some(e) = self.get_edge_mut(edge) else {
            unreachable!("split_edge: `edge` resolved in the plan phase and this op kills no edge")
        };
        e.curve = first_curve;
        let killed_curve = self
            .remove_curve_if_orphaned(edge_data.curve)
            .then_some(edge_data.curve);
        // Emanating rules (documented above). `v` is read and written
        // through ONE lookup: the condition is a field of the borrow
        // that performs the write, so no path reaches the write with
        // `v` unproven.
        let Some(vertex) = self.get_vertex_mut(w) else {
            unreachable!("split_edge: `w` is minted by this function, above")
        };
        vertex.emanating = Some(n_plus);
        let Some(vertex) = self.get_vertex_mut(v) else {
            unreachable!(
                "split_edge: `v` proven live by the plan phase (resolve_vertex_point) and \
                 this op kills no vertex"
            )
        };
        if vertex.emanating == Some(hm.key()) {
            vertex.emanating = Some(n_minus);
        }

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                half_edges: 2,
                edges: 1,
                vertices: 1,
                ..ArenaDelta::ZERO
            },
            "split_edge",
        );
        Ok(SplitEdgeCreated {
            vertex: w,
            point,
            new_edge,
            he_plus: n_plus,
            he_minus: n_minus,
            first_curve,
            second_curve,
            killed_curve,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
    use geom_core::Tol;

    use geom::Curve3;
    use geom_brep::{EdgeCurveSpec, EdgeDescription, MappedCurve, SketchSegment};
    use geom_core::{Affine3, Point2, Point3, Vec3};

    use super::*;
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::{deep_snapshot, ops_cube};
    use crate::validate::{validate, validate_closed};

    /// Splitting a cube edge (line carrier) at mid-parameter: intervals
    /// split, both children certify, endpoints/adjacency line up, and
    /// the cube stays a tier-2 closed solid.
    #[test]
    fn split_line_edge_mid() {
        let cube = ops_cube(Tol::witness());
        let mut body = cube.body;
        let edge = cube.mevs[0].edge; // A → B, chord length 1, params [0,1]
        let created = body.split_edge(edge, 0.5, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(validate_closed(&body), Ok(()));
        // The new vertex sits at the chord midpoint.
        let p = body.get_point(created.point).unwrap();
        assert!((p.x - 0.5).abs() < 1e-12 && p.y.abs() < 1e-12 && p.z.abs() < 1e-12);
        // Parent covers [0, 0.5]; the new edge [0.5, 1].
        let c1 = body
            .get_curve_geom(created.first_curve)
            .unwrap()
            .certified()
            .unwrap();
        assert_eq!(c1.params(), (0.0, 0.5));
        let c2 = body
            .get_curve_geom(created.second_curve)
            .unwrap()
            .certified()
            .unwrap();
        assert_eq!(c2.params(), (0.5, 1.0));
        // Parent's old curve was orphaned and killed.
        assert!(created.killed_curve.is_some());
        // Derived incidence: parent runs A → w, new edge w → B.
        let parent = body.get_edge(edge).unwrap();
        assert_eq!(body.half_edge_end(parent.he_plus), Some(created.vertex));
        assert_eq!(
            body.get_half_edge(created.he_plus).unwrap().start,
            created.vertex
        );
        assert_eq!(
            body.half_edge_end(created.he_plus),
            Some(cube.mevs[0].vertex)
        );
        // Provenance: typed SplitEdge birth records.
        assert_eq!(
            body.provenance(crate::EntityId::Edge(created.new_edge)),
            Some(&Provenance::SplitEdge { edge })
        );
    }

    /// A quarter-circle arc edge (circle carrier, placed-segment arc
    /// description): split at the 45° parameter; both children certify
    /// (the bulge restriction is exercised) and the parent carrier is
    /// shared unchanged.
    #[test]
    fn split_arc_edge() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(1.0, 0.0, 0.0)).unwrap();
        let spec = EdgeCurveSpec {
            description: EdgeDescription::MappedCurve(MappedCurve::PlacedSegment {
                segment: SketchSegment::Arc {
                    a: Point2::new(1.0, 0.0),
                    b: Point2::new(0.0, 1.0),
                    bulge: (PI / 8.0).tan(),
                },
                place: Affine3::identity(),
            }),
            carrier: Curve3::Circle {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: FRAC_PI_2,
        };
        let arc = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(0.0, 1.0, 0.0),
                spec,
                Tol::witness(),
            )
            .unwrap();
        let created = body
            .split_edge(arc.edge, FRAC_PI_4, Tol::witness())
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        let p = body.get_point(created.point).unwrap();
        let r = FRAC_PI_4.cos();
        assert!((p.x - r).abs() < 1e-12 && (p.y - r).abs() < 1e-12);
        let c2 = body
            .get_curve_geom(created.second_curve)
            .unwrap()
            .certified()
            .unwrap();
        assert_eq!(c2.params(), (FRAC_PI_4, FRAC_PI_2));
        // The children share the parent's carrier bitwise.
        assert!(matches!(
            c2.carrier(),
            Curve3::Circle { radius, .. } if *radius == 1.0
        ));
        // The restricted description is still a placed arc.
        assert!(matches!(
            c2.description(),
            EdgeDescription::MappedCurve(MappedCurve::PlacedSegment {
                segment: SketchSegment::Arc { .. },
                ..
            })
        ));
    }

    /// A full-period self-loop (scaffolding circle, revolved-point
    /// description): splitting at π yields a two-edge digon loop —
    /// the coincidence-heavy splice cases land correctly.
    #[test]
    fn split_self_loop_edge() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        // One-edge circular face at the far vertex (self-loop edge with
        // the canonical scaffolding circle).
        let circ = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        let created = body.split_edge(circ.edge, PI, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        // The split vertex sits diametrically across the unit
        // scaffolding circle (center p + x̂, u_ref = −x̂).
        let p = body.get_point(created.point).unwrap();
        assert!((p.x - 3.0).abs() < 1e-12 && p.y.abs() < 1e-12);
        // The circular face's loop is now the two-edge digon.
        let f = body.get_face(circ.face).unwrap();
        let crate::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
            panic!("circular face lost its cycle");
        };
        assert_eq!(body.loop_cycle(first).unwrap().len(), 2);
    }

    /// The strut case: next(he_plus) == he_minus. Splitting yields the
    /// two-edge strut chain … hp → n⁺ → n⁻ → hm … (the second splice
    /// reads prev(hm) after the first).
    #[test]
    fn split_strut_edge() {
        let cube = ops_cube(Tol::witness());
        let mut body = cube.body;
        let anchor = cube.mevs[0].he_plus;
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                Point3::new(2.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        let created = body.split_edge(strut.edge, 0.5, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        let hp = body.get_edge(strut.edge).unwrap().he_plus;
        let chain1 = body.get_half_edge(hp).unwrap().next;
        assert_eq!(chain1, created.he_plus);
        let chain2 = body.get_half_edge(chain1).unwrap().next;
        assert_eq!(chain2, created.he_minus);
        let chain3 = body.get_half_edge(chain2).unwrap().next;
        assert_eq!(chain3, body.get_edge(strut.edge).unwrap().he_minus);
    }

    /// Interiority refusals: at an endpoint (Zero), outside (Negative),
    /// and in-band (Escalated) — each typed, each leaving the body
    /// deeply untouched. The in-band probe is computed from the run's
    /// Band so the test holds at every ε row.
    #[test]
    fn split_param_refusals_are_typed_and_atomic() {
        let cube = ops_cube(Tol::witness());
        let mut body = cube.body;
        let edge = cube.mevs[0].edge;
        let band = Band::linear(Tol::witness()).unwrap();
        let before = deep_snapshot(&body);
        for (t, expect_escalated) in [
            (0.0, false),
            (1.0, false),
            (1.5, false),
            (-0.25, false),
            ((band.zero() + band.escalate()) * 0.5, true),
        ] {
            let err = body.split_edge(edge, t, Tol::witness()).unwrap_err();
            match (expect_escalated, &err) {
                (false, EulerOpError::SplitParamNotInterior { edge: e }) => {
                    assert_eq!(*e, edge);
                }
                (true, EulerOpError::SplitParamEscalated { edge: e, .. }) => {
                    assert_eq!(*e, edge);
                }
                other => panic!("unexpected: {other:?}"),
            }
            assert_eq!(deep_snapshot(&body), before, "body changed on Err");
        }
    }

    /// Splitting a null-scaffold edge is refused by type.
    #[test]
    fn split_null_edge_is_refused() {
        let cube = ops_cube(Tol::witness());
        let mut body = cube.body;
        let he = body
            .get_vertex(cube.seed.vertex)
            .unwrap()
            .emanating
            .unwrap();
        let null = body
            .mev_null(
                MevSite::Fan { he1: he, he2: he },
                crate::null::NewVertexSide::Above,
            )
            .unwrap();
        let err = body.split_edge(null.edge, 0.5, Tol::witness()).unwrap_err();
        assert_eq!(err, EulerOpError::NullScaffoldCurve { curve: null.curve });
    }

    /// Determinism (D9): identical histories (including the split) are
    /// byte-identical.
    #[test]
    fn split_replay_is_byte_identical() {
        let build = || {
            let cube = ops_cube(Tol::witness());
            let mut body = cube.body;
            body.split_edge(cube.mevs[0].edge, 0.25, Tol::witness())
                .unwrap();
            body.split_edge(cube.mevs[3].edge, 0.75, Tol::witness())
                .unwrap();
            body
        };
        assert_eq!(deep_snapshot(&build()), deep_snapshot(&build()));
    }
}
