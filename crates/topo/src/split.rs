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
//! restriction arithmetic is verified, never trusted). A tangent
//! child stores what the must-carry rule over the child demands, which
//! over a short child can be an image in an adjacent chart rather than
//! the parent's tangency; every other child keeps the parent's
//! description kind and surface keys, and the adjacent faces are
//! untouched by the surgery.
//!
//! Serves ch. 14 `splitgenerate` edge splitting and ch. 15's edge×face
//! reduction sweep (M3 PRs 2 and 4).

use geom_brep::recourse::{Reading, Refused, RefusedArm, RefusedSide, SizedDecision, SizedPass};
use geom_brep::{CertifyError, EdgeCurveSpec};
use geom_core::{Band, Decide, InfSpeed, Margin, Point3, Tol};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, GeomRef, HalfEdgeKey, VertexKey};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::EulerOpError;
use crate::geometry::{CurveKey, PointKey};
use crate::live::{Arg, dangling_link, link, lookup};
use crate::provenance::Provenance;

/// Where a crossing lands along its edge, in words: the subject every
/// decision on it states, here and wherever an edge is split at a
/// crossing.
pub(crate) const CROSSING_INTERIOR: &str = "whether a crossing lands strictly inside its edge";

/// The lever every door that splits an edge at a crossing has (the
/// split, the blend, the Boolean): the geometry.
pub(crate) const CROSSING_LEVER: &str =
    "move the geometry so the crossing lands clearly away from the edge's ends";

/// The lever a split of a smooth edge (a tangency or a pair) has where the must-carry rule
/// over a piece is in band: the piece's length decides its sagitta.
pub(crate) const SMOOTH_PIECE_LEVER: &str =
    "move the geometry so the crossing cuts a longer or a shorter piece off the smooth edge";

/// What a crossing's interiority margin measures.
pub(crate) const CROSSING_SIZE: &str = "distance from the edge's end";

/// [`Body::split_edge`](crate::construction::Construct::split_edge)'s interiority decision: it passes only on a
/// crossing definitely inside its edge.
pub(crate) const SPLIT_PARAM_INTERIOR: SizedDecision = SizedDecision {
    lever: CROSSING_LEVER,
    size: CROSSING_SIZE,
    passes: SizedPass::Positive,
    side: RefusedSide::Lever,
    no_size: None,
};

/// The one ending of [`SPLIT_PARAM_INTERIOR`]'s refused `arm`, at the
/// operation that asked. Both of `split_edge`'s interiority refusals end
/// here, and neither offers a declaration: no door that splits an edge
/// takes one naming where on the edge a crossing lands.
pub(crate) fn split_param_ending(arm: RefusedArm<'_>) -> String {
    SPLIT_PARAM_INTERIOR.recourse(arm, Reading::Build)
}

/// Every key minted (and the one possibly killed) by one
/// [`Body::split_edge`](crate::construction::Construct::split_edge) call.
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
    /// The door [`Construct::split_edge`](crate::construction::Construct::split_edge) forwards to;
    /// documented there.
    pub(crate) fn split_edge(
        &mut self,
        edge: EdgeKey,
        t: T,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError>
    where
        T: crate::props::AtRestPolicy,
    {
        self.split_edge_minting(edge, t, None, tol)
    }

    /// [`Self::split_edge`](crate::construction::Construct::split_edge), with the new vertex holding `at`'s own
    /// bits rather than `carrier(t)`'s: the split lands on a point the
    /// caller already holds (another vertex's), and the two must read
    /// one point. Both children certify against `at`, so a point off
    /// the carrier past the band refuses as any endpoint would.
    pub(crate) fn split_edge_onto(
        &mut self,
        edge: EdgeKey,
        t: T,
        at: Point3<T>,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError>
    where
        T: crate::props::AtRestPolicy,
    {
        self.split_edge_minting(edge, t, Some(at), tol)
    }

    /// What a child of a split edge describes. The restriction
    /// ([`geom_brep::EdgeCurve::split_specs`]) verbatim, except that a
    /// tangent child stores what the must-carry rule over the CHILD
    /// demands ([`geom_brep::must_carry_over_edge`]): the rule reads
    /// the child's own stations, levered at its faces' extent
    /// ([`fn@crate::wedge_reach`]) read from the child's middle, so a child
    /// can read under-determined, or in band, where its parent read
    /// jet-determinate.
    ///
    /// Under-determined stores the conventional description where the
    /// child rests ([`crate::rest_locus`]). A pair child, and a child
    /// image that is not a wrap, is asked again too
    /// ([`geom_brep::reask_pair`], [`geom_brep::reask_chart`]): its
    /// stations are not its parent's, and the verdict reads them.
    ///
    /// The pair's keys are links of the parent's certified description,
    /// so a key that does not resolve is a torn body, not a refusal.
    fn tangent_piece(
        &self,
        edge: EdgeKey,
        spec: EdgeCurveSpec<T>,
        band: Band,
    ) -> Result<EdgeCurveSpec<T>, EulerOpError> {
        let surface = |k| {
            self.get_surface(k).unwrap_or_else(|| {
                dangling_link(
                    EntityId::Edge(edge),
                    "curve's description",
                    GeomRef::Surface(k),
                )
            })
        };
        let refused = |refusal| match refusal {
            geom_brep::PairRefusal::InBand(geom_brep::MustCarryEscalation::FirstOrder(
                escalation,
            )) => EulerOpError::SplitSmoothEscalated {
                edge,
                diag: escalation.diag(),
            },
            geom_brep::PairRefusal::InBand(geom_brep::MustCarryEscalation::SecondOrder(diag)) => {
                EulerOpError::SplitSmoothBendEscalated { edge, diag }
            }
            geom_brep::PairRefusal::Mixed(stations) => EulerOpError::SplitSmoothRefuted {
                edge,
                mixed: Some(stations),
            },
        };
        let (pair, witness) = match spec.description {
            geom_brep::EdgeDescriptionSpec::TangentIntersection { pair, witness } => {
                (pair, witness)
            }
            geom_brep::EdgeDescriptionSpec::OnPair { pair, declared } => {
                let description = self
                    .reask_on(
                        pair,
                        crate::edge_faces(self, edge),
                        &spec.carrier,
                        (spec.param_start, spec.param_end),
                        declared,
                        band,
                    )
                    .map_err(refused)?;
                return Ok(EdgeCurveSpec {
                    description,
                    ..spec
                });
            }
            // A piece of an image is asked the same over its own span;
            // it keeps the image where the rule still reads the join
            // under-determined.
            geom_brep::EdgeDescriptionSpec::Chart { wrap: false, .. } => {
                return Ok(
                    match self
                        .reask_chart_on(
                            edge,
                            &spec.carrier,
                            (spec.param_start, spec.param_end),
                            band,
                        )
                        .map_err(refused)?
                    {
                        Some(description) => EdgeCurveSpec {
                            description,
                            ..spec
                        },
                        None => spec,
                    },
                );
            }
            geom_brep::EdgeDescriptionSpec::Intersection { .. }
            | geom_brep::EdgeDescriptionSpec::Chart { .. }
            | geom_brep::EdgeDescriptionSpec::Scaffold(_) => return Ok(spec),
        };
        let [ka, kb] = pair.keys();
        // The parent's two faces, each beside the key it lies on: the
        // region the rest is read over.
        let sides = crate::readback::edge_sides_of(
            self,
            edge,
            crate::live::proven(&self.edges, edge, EntityId::Edge),
        );
        let face_on = |k| {
            [sides.plus, sides.minus]
                .into_iter()
                .find(|side| side.surface == k)
                .map(|side| side.face)
        };
        let rest = crate::rest_locus(
            self,
            [(ka, face_on(ka)), (kb, face_on(kb))],
            &spec.carrier,
            (spec.param_start, spec.param_end),
            band,
        );
        let description = geom_brep::must_carry_description(
            geom_brep::SmoothJoin {
                surfaces: [(ka, surface(ka)), (kb, surface(kb))],
                carrier: &spec.carrier,
                interval: (spec.param_start, spec.param_end),
                reach: crate::wedge_reach(
                    self,
                    crate::edge_faces(self, edge),
                    &spec.carrier,
                    (spec.param_start, spec.param_end),
                ),
                witness,
            },
            band,
            |_, diag| EulerOpError::SplitSmoothEscalated { edge, diag },
            |_, diag| EulerOpError::SplitSmoothBendEscalated { edge, diag },
            |mixed| EulerOpError::SplitSmoothRefuted { edge, mixed },
        )?
        .spec(rest);
        Ok(EdgeCurveSpec {
            description,
            ..spec
        })
    }

    fn split_edge_minting(
        &mut self,
        edge: EdgeKey,
        t: T,
        at: Option<Point3<T>>,
        tol: Tol,
    ) -> Result<SplitEdgeCreated, EulerOpError>
    where
        T: crate::props::AtRestPolicy,
    {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let edge_data = lookup(&self.edges, edge, EntityId::Edge, Arg("edge"))?.clone();
        let (hp, hm) = (edge_data.he_plus, edge_data.he_minus);
        let (hp, hp_data) = self.resolve_half_edge_live(hp, link(EntityId::Edge(edge), "he_plus"));
        let (hm, hm_data) = self.resolve_half_edge_live(hm, link(EntityId::Edge(edge), "he_minus"));
        // The two splices write through `next(hp)` and `prev(hm)` as
        // well as the parent's own halves, whose proofs came out of the
        // resolves above; prove these two now so the mutation below
        // cannot fail midway (atomicity). `prev(hm)` changes
        // under splice 1 — see the splice for the case that moves it,
        // and for why the new value is proven too.
        let hp_next = self.require_live(hp_data.next, link(EntityId::HalfEdge(hp.key()), "next"));
        let hm_prev = self.require_live(hm_data.prev, link(EntityId::HalfEdge(hm.key()), "prev"));
        let curve = self.stored_curve(edge)?.clone();
        // Interiority (trilean, Q1): both sub-spans definitely
        // positive, metered in meters like the certification span gate.
        let (t0, t1) = curve.params();
        let scale = match *curve.carrier() {
            geom::Curve3::Line { .. } => InfSpeed::new(T::one()),
            geom::Curve3::Circle { radius, .. } => InfSpeed::new(radius),
            // The conic lane (M5 PR 5, C12.3): metered at the SMALLER
            // semi-axis magnitude — the conservative meter
            // (|dP/dθ| ≥ min(|a|, |b|)), so a sub-span this gate accepts
            // as definitely interior is truly clear of the endpoints in
            // meters. The stored semi-axes carry no order
            // (`geom_brep::Conic`); it is `minor` in the ordinary order.
            geom::Curve3::Ellipse { semi_u, semi_v, .. } => {
                InfSpeed::new(geom::Curve3::semi_axes_by_size(semi_u, semi_v).1)
            }
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
            // The spiric's speed floor is its minor radius (`|dP/dv|
            // ≥ r`, the variant docs), the same meter certification
            // spans it at.
            geom::Curve3::Spiric { minor_radius, .. } => InfSpeed::new(minor_radius),
        };
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;
        for margin in [
            Margin::metered(t - t0, scale),
            Margin::metered(t1 - t, scale),
        ] {
            match geom_core::k_stats::decide_reported("split_edge_param_interior", margin, band) {
                Ok(decided) => {
                    if let Some(verdict) = Refused::of(decided, band) {
                        return Err(EulerOpError::SplitParamNotInterior { edge, verdict });
                    }
                }
                Err(diag) => {
                    return Err(EulerOpError::SplitParamEscalated { edge, diag });
                }
            }
        }
        let (u, v) = (hp_data.start, hm_data.start);
        let p_u = self.linked_vertex_point(u, EntityId::HalfEdge(hp.key()), "start");
        let p_v = self.linked_vertex_point(v, EntityId::HalfEdge(hm.key()), "start");
        let p_new = at.unwrap_or_else(|| curve.carrier().eval(t));
        // ---- Geometry gate (still no mutation): both children must
        // certify against their own endpoints.
        let (spec1, spec2) = curve.split_specs(t);
        let spec1 = self.tangent_piece(edge, spec1, band)?;
        let spec2 = self.tangent_piece(edge, spec2, band)?;
        let faces = crate::edge_faces(self, edge);
        let origin =
            crate::policy_lane::Origin::Kernels(crate::policy_lane::KernelSite::SplitChild);
        let child = |spec, start, end| {
            self.certify_edge_spec(origin, Some(edge), spec, start, end, faces, tol)
        };
        let cert1 = child(spec1, p_u, p_new)?;
        let cert2 = child(spec2, p_new, p_v)?;
        // ---- Pcurve gate (still no mutation): each parent half-edge's
        // stored chart row, stated over the two children's sub-intervals
        // (placed again where it is closed-form or projected) and
        // certified, with the joints the children make. Read-only, so a
        // refusal leaves the body untouched like every gate above it.
        let [rows_plus, rows_minus] =
            crate::pcurves::split_cache(self, [hp.key(), hm.key()], t, band).map_err(
                |crate::pcurves::SplitRowError { half_edge, refusal }| match refusal {
                    crate::pcurves::SplitRefusal::Certify(error) => EulerOpError::PcurveSplit {
                        edge,
                        half_edge,
                        error,
                    },
                    crate::pcurves::SplitRefusal::Joint(diag) => {
                        EulerOpError::SplitJointUndecided {
                            edge,
                            half_edge,
                            diag,
                        }
                    }
                },
            )?;

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
        // The joints the splice makes, read before it. Each child pair
        // joins at the split point with the element the restriction
        // decided; the joint out of the child whose exit is the parent's
        // is the parent's old one, unless that child was placed again
        // over its own span, when the restriction decided it afresh. A
        // side the restriction carried nothing for takes no element.
        let carried =
            |rows: &Option<crate::pcurves::CarriedRows<T>>,
             placed: fn(&crate::pcurves::CarriedRows<T>) -> Option<crate::joint::JointElement>,
             old: HalfEdgeKey| {
                rows.as_ref().map_or((None, None), |rows| {
                    (Some(rows.joint), placed(rows).or(self.joint(old)))
                })
            };
        let (into_n_plus, into_hp_next) = carried(&rows_plus, |r| r.leaving, hp_next.key());
        let (into_hm, into_n_minus) = carried(&rows_minus, |r| r.entering, hm.key());
        // The joints into the fed child on the plus side and out of the
        // first child on the minus side keep their half-edges, so the
        // splice does not write them; a re-placed child's are written
        // after it.
        let entering_hp = rows_plus.as_ref().and_then(|r| r.entering);
        let leaving_hm = rows_minus.as_ref().and_then(|r| r.leaving);
        // Splice 1: hp → n⁺ → old next(hp), in hp's loop.
        self.link_half_edges(hp, n_plus, into_n_plus);
        self.link_half_edges(n_plus, hp_next, into_hp_next);
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
        self.link_half_edges(hm_prev, n_minus, into_n_minus);
        self.link_half_edges(n_minus, hm, into_hm);
        // The splice is done; past it the new halves are ordinary keys.
        let (n_plus, n_minus) = (n_plus.key(), n_minus.key());
        if let Some(e) = entering_hp {
            self.joints.insert(hp.key(), e);
        }
        if let Some(e) = leaving_hm {
            self.joints.insert(hm_data.next, e);
        }
        // The chart rows certified above: the parent halves keep the
        // first child's (they ARE the first child's halves), the new
        // halves take the second child's. Both children lie in their
        // parent's own loop, so the loop's one-branch unwrap is the
        // parent's and needs no re-pinning.
        if let Some(rows) = rows_plus {
            self.pcurves.insert(hp.key(), rows.parent_half);
            self.pcurves.insert(n_plus, rows.new_half);
        }
        if let Some(rows) = rows_minus {
            self.pcurves.insert(hm.key(), rows.parent_half);
            self.pcurves.insert(n_minus, rows.new_half);
        }
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
    use geom_brep::{
        EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, MappedCurve, MappedSource,
        SketchSegment,
    };
    use geom_core::{Affine3, Arc2, Point2, Point3, Vec3};

    use super::*;
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::deep_snapshot;
    use crate::test_support_fixtures::declined_cube;
    use crate::validate::{validate, validate_closed};

    /// Splitting a cube edge (line carrier) at mid-parameter: intervals
    /// split, both children certify, endpoints/adjacency line up, and
    /// the cube stays a tier-2 closed solid.
    #[test]
    fn split_line_edge_mid() {
        let cube = declined_cube::<f64>(Tol::witness());
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
    /// (the arc restriction is exercised) and the parent carrier is
    /// shared unchanged.
    #[test]
    fn split_arc_edge() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(1.0, 0.0, 0.0), true).unwrap();
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Scaffold(MappedCurve::whole(
                MappedSource::PlacedSegment {
                    segment: SketchSegment::Arc {
                        a: Point2::new(1.0, 0.0),
                        b: Point2::new(0.0, 1.0),
                        arc: Arc2 {
                            centre: Point2::new(0.0, 0.0),
                            radius: 1.0,
                            sweep: FRAC_PI_2,
                        },
                    },
                    place: Affine3::identity(),
                },
            )),
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
            EdgeDescription::Scaffold(MappedCurve {
                source: MappedSource::PlacedSegment {
                    segment: SketchSegment::Arc { .. },
                    ..
                },
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
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
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
        let cube = declined_cube::<f64>(Tol::witness());
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
        let cube = declined_cube::<f64>(Tol::witness());
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
                (false, EulerOpError::SplitParamNotInterior { edge: e, .. }) => {
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
        let cube = declined_cube::<f64>(Tol::witness());
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
            let cube = declined_cube::<f64>(Tol::witness());
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
