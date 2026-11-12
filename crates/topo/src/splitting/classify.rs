//! Reduction step (ch. 14 `splitgenerate`, re-derived): the operand
//! gate (F5 → the C5 table, M5 PR 5), the cached vertex-vs-plane
//! trilean sweep (F6), and crossing insertion through the certified
//! `split_edge` lane — with the conic crossing-root lane for
//! circle/ellipse carriers.

use geom_brep::curve_face::plane_crossing::{
    ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane,
};
use geom_core::{Band, Decide, Margin, Point3, Sign, Tol};
use slotmap::SecondaryMap;

use super::{PlaneSide, SplitPlane, SplitReduceError};
use crate::body::Body;
use crate::entity::{EdgeKey, VertexKey};
use crate::null::CurveGeom;
use crate::validate::decide;
use crate::vertex_section::VertexRefusal;
use geom::cover::BoxAxes;

/// **The carrier gate, scoped to what the plane can reach** (C12.1:
/// the gate retires per arm, never wholesale).
///
/// A face passes if the split pipeline executes its `(kind × plane)`
/// arm — `Plane`, `Cylinder` and `Cone`. A face of any other kind
/// (`Sphere`, `Torus`, `Nurbs`, `Approx`) refuses typed only when the
/// plane MAY meet it: when its padded reach along the plane's normal
/// (`census::face_reach_in`, the face's cover at this body's scalar)
/// is not definitely on one side of the plane
/// ([`reach_clears`]). Behind a reach that clears, the face has no
/// vertex on or across the plane, no edge crossing it and no section
/// through it, so every later stage carries it through whole. The
/// reach is the face's STORED locus, so an `Approx` face clears by its
/// fit, which is the geometry there is to cut; one the plane may meet
/// refuses by kind, since an arm executed against the fit would cut
/// the approximation, not the surface the modeller described.
///
/// The reach is read in the plane's own frame
/// ([`geom::cover::BoxAxes::aimed`]), never in the world's
/// axes, so whether a cut clears a face is a fact about the two of
/// them and not about how the body is turned.
///
/// Edge carriers: `Line`, `Circle` and `Ellipse` pass. A `Spiric` or
/// `Nurbs` carrier refuses typed only when the plane may meet it
/// ([`edge_clears`]). A face whose surface key, or an edge whose curve
/// key, does not resolve is a torn body and panics (the operand is a
/// public body, at rest), and so does a null edge: every door that
/// reaches here holds its body tier-2 clean first (a finished operand,
/// or a test-support door's own read), and tier 2 refuses one.
pub(super) fn carrier_gate<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<(), SplitReduceError> {
    let frame = BoxAxes::aimed(plane.normal);
    for face in body.face_views() {
        let face_key = face.key();
        let surface = face.surface();
        let kind = surface.kind();
        match kind {
            geom::SurfaceKind::Plane | geom::SurfaceKind::Cylinder | geom::SurfaceKind::Cone => {}
            geom::SurfaceKind::Sphere
            | geom::SurfaceKind::Torus
            | geom::SurfaceKind::Spindle
            | geom::SurfaceKind::Nurbs
            | geom::SurfaceKind::Approx => {
                if !reach_clears(
                    crate::census::face_reach_in(body, face_key, band, &frame),
                    plane,
                    band,
                ) {
                    return Err(SplitReduceError::CurvedBooleanUnsupported {
                        face: face_key,
                        kind,
                    });
                }
            }
        }
    }
    for (edge_key, edge) in body.edges() {
        match body.edge_curve_linked(edge_key, edge) {
            CurveGeom::Certified(curve) => match curve.carrier() {
                geom::Curve3::Line(_) | geom::Curve3::Circle(..) | geom::Curve3::Ellipse(..) => {}
                // No crossing-root arm reads a spiric or a spline.
                geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
                    if !edge_clears(body, edge_key, plane, band, &frame) {
                        return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
                    }
                }
            },
            CurveGeom::NullScaffold(_) => unreachable!(
                "{edge_key:?} is a null edge, and every door reaching here holds its body tier-2 \
                 clean, which refuses a null edge at rest"
            ),
        }
    }
    Ok(())
}

/// Whether the plane certainly misses an edge: behind its own reach
/// (its cover's, `boolean::crate::cover::edge_cover`) or behind the reach of
/// either face it
/// bounds, since an edge lies on both its faces. A spline edge's own
/// reach is its control net's hull (its cover), which its faces'
/// reaches may beat. `frame` is the plane's
/// ([`BoxAxes::aimed`] at its normal).
///
/// `edge` is one this call's caller read out of the body: the gate's
/// arena walk over the at-rest operand, or `insert_crossings`' snapshot
/// of it, where only `split_edge`s of other snapshot edges stand
/// between, and they remove no edge and rewrite no other edge's halves.
/// So `edge` is proven, and its halves and their faces are links, which
/// panic on a miss ([`crate::live::OPERATORS_KEEP_LINKS`]).
fn edge_clears<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    plane: &SplitPlane<T>,
    band: Band,
    frame: &BoxAxes<T>,
) -> bool {
    let e = crate::live::proven(&body.edges, edge, crate::entity::EntityId::Edge);
    let reach = crate::cover::edge_cover(body, edge)
        .ok()
        .map(|cover| cover.box_in(frame));
    reach_clears(reach, plane, band)
        || [e.he_plus, e.he_minus].into_iter().any(|he| {
            let face = body.face_of_linked(he);
            reach_clears(
                crate::census::face_reach_in(body, face, band, frame),
                plane,
                band,
            )
        })
}

/// K name: an unarmed entity's padded reach, its distance from the
/// split plane along the plane's normal (metres).
const SPLIT_GATE_BOX_SIDE: &str = "split_gate_box_side";

/// Whether the plane certainly misses a reach read in the plane's
/// frame ([`BoxAxes::aimed`]), whose first coordinate is the
/// component along the plane's normal: the reach's gap from the
/// plane's own offset along that normal, less the boolean sweep's pad,
/// decided definitely positive. No reach (an entity the reach lane
/// cannot bound) answers `false`.
fn reach_clears<T: Decide>(
    reach: Option<(Point3<T>, Point3<T>)>,
    plane: &SplitPlane<T>,
    band: Band,
) -> bool {
    let Some((lo, hi)) = reach else {
        return false;
    };
    let pad = T::from_f64(crate::boxes::sweep_pad(band));
    // The frame turns about the world origin, so a reach's first
    // coordinate is `n·p`, and the world origin's own offset from the
    // plane turns it into `n·(p − q)`.
    let origin = geom::plane_offset(
        plane.origin,
        plane.normal.get(),
        Point3::new(T::zero(), T::zero(), T::zero()),
    );
    let gap = (lo.x + origin).max(T::zero() - (hi.x + origin)) - pad;
    matches!(
        decide(SPLIT_GATE_BOX_SIDE, Margin::of(gap), band),
        Ok(Sign::Positive)
    )
}

/// F6: classify every vertex against the plane through the
/// `split_vertex_side` trilean (margin = signed distance in meters,
/// linear band), then each vertex that reads ON against the sections
/// the plane ends at it ([`crate::vertex_section`]), its walk reading
/// the far vertices' offset verdicts. One offset evaluation per vertex
/// (the book recomputes per incident edge). In-band ⇒ typed
/// [`SplitReduceError::SliverVertex`], never a snap.
pub(super) fn classify_vertices<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    band: Band,
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    let tips = crate::sector_shape::SlitTips::of(body);
    classify_vertices_margined(body, plane, &tips, band).map(|(sides, on, _)| (sides, on))
}

/// Each vertex's side, the ON vertices in arena order, and the margin
/// each ON vertex was decided Zero on ([`classify_vertices_margined`]).
pub(super) type MarginedSides = (
    SecondaryMap<VertexKey, PlaneSide>,
    Vec<VertexKey>,
    SecondaryMap<VertexKey, geom_core::MarginDiag>,
);

/// [`classify_vertices`], with the margin each ON vertex was decided
/// Zero on: the coincidence a pinch records ([`crate::coincidence`]) —
/// its section reading where one was read, else its offset. `tips` are
/// `body`'s slit tips.
pub(super) fn classify_vertices_margined<T: Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    tips: &crate::sector_shape::SlitTips,
    band: Band,
) -> Result<MarginedSides, SplitReduceError> {
    let side_of = |sign| match sign {
        Sign::Negative => PlaneSide::Below,
        Sign::Positive => PlaneSide::Above,
        Sign::Zero => PlaneSide::On,
    };
    let mut offsets = SecondaryMap::new();
    let mut zero = Vec::new();
    for (vertex_key, p) in body.vertex_points() {
        let offset = geom::plane_offset_rounded(plane.origin, plane.normal.get(), p);
        let decided = crate::validate::decide_reported(
            "split_vertex_side",
            geom_core::Margin::of(offset.value),
            band,
        )
        .map_err(|diag| SplitReduceError::SliverVertex {
            vertex: vertex_key,
            diag,
        })?;
        if decided.sign == Sign::Zero {
            zero.push((vertex_key, p, offset, decided));
        }
        offsets.insert(vertex_key, side_of(decided.sign));
    }
    let mut sides = offsets.clone();
    let mut on_vertices = Vec::new();
    let mut on_margins = SecondaryMap::new();
    let section_plane = || {
        Ok(crate::vertex_section::SectionPlane {
            origin: plane.origin,
            normal: plane.normal,
            reference: geom_brep::ReferenceNormal::of_split_plane(plane.normal.get()),
        })
    };
    for (vertex_key, p, offset, on_plane) in zero {
        let far =
            |far| {
                offsets.get(far).copied().ok_or(VertexRefusal::Walk(
                    SplitReduceError::UnrecordedSide { vertex: far },
                ))
            };
        let decided = crate::vertex_section::read_sections(
            (body, tips),
            (vertex_key, p),
            section_plane,
            (
                crate::vertex_section::Offset {
                    value: offset,
                    normal: plane.normal.get(),
                },
                on_plane,
                far,
            ),
            band,
        )
        .map_err(|refusal| match refusal {
            VertexRefusal::Offset(diag) | VertexRefusal::Section(diag) => {
                SplitReduceError::SliverVertex {
                    vertex: vertex_key,
                    diag,
                }
            }
            VertexRefusal::Walk(e) => e,
            VertexRefusal::Star(fault) => fault.split_refusal(vertex_key, band),
            VertexRefusal::Side { face, diag } => SplitReduceError::SliverSector {
                vertex: vertex_key,
                face,
                reading: crate::splitting::SectorReading::Escalated(diag),
            },
            VertexRefusal::Plane(what) => unreachable!(
                "{vertex_key:?}: the split plane reads into the table whole, yet {what}"
            ),
        })?;
        if decided.sign == Sign::Zero {
            on_vertices.push(vertex_key);
            on_margins.insert(vertex_key, decided.margin);
        }
        sides.insert(vertex_key, side_of(decided.sign));
    }
    Ok((sides, on_vertices, on_margins))
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
///   ([`plane_crossing_lane`]), INDEPENDENT of endpoint verdicts:
///   same-side endpoints with a belly crossing the plane twice get
///   BOTH crossing vertices; an ON endpoint with one interior
///   crossing gets it; grazes land as single ON contacts for the
///   sector classification. Two roots split the parent then its
///   trailing child (ascending — the second root lives on the child's
///   span).
/// - **Spiric and spline carriers** have no crossing lane. One passes
///   uncut only where [`edge_clears`] certifies its whole locus on one
///   side of the plane, the carrier gate's own test; any other refuses
///   [`SplitReduceError::CurvedEdgeUnsupported`]. Same-side endpoints
///   are not enough: a spline's belly can cross between them.
///
/// Both crossing lanes are certified by `split_edge` itself (the
/// span's place of the parameter (`curve_pair::Span::place_param_read`) + full child re-certification —
/// the honest lane the raw book formula lacks). New vertices are ON by
/// the decision that placed them: a crossing root lies on the plane, and
/// a graze root within ε of it by `plane_crossing_belly_graze`'s Zero.
/// Their verdicts are cached without re-measuring. Returns each piece
/// a split minted with the edge it was split from, `(new edge,
/// divided-from edge)` in mint order, each checked as it was minted
/// ([`SplitReduceError::LineageRow`] where an earlier row names it).
pub(super) fn insert_crossings<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    plane: &SplitPlane<T>,
    sides: &mut SecondaryMap<VertexKey, PlaneSide>,
    on_vertices: &mut Vec<VertexKey>,
    tol: Tol,
) -> Result<crate::lineage::SplitEdgeDivisions, SplitReduceError> {
    let mut pieces =
        crate::lineage::SplitEdgeDivisions::new(crate::lineage::LineageRows::EdgeDivisions(None));
    let band = geom_core::Band::linear(tol)?;
    // Snapshot: splitting adds edges; only operand edges can cross (a
    // split child's remaining crossing is handled through the parent's
    // precomputed root list below).
    let snapshot: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in snapshot {
        let start = |body: &Body<T>, he, slot| {
            crate::live::linked(
                &body.half_edges,
                he,
                crate::entity::EntityId::HalfEdge,
                crate::entity::EntityId::Edge(edge_key),
                slot,
            )
            .start
        };
        let u = start(body, edge.he_plus, "he_plus");
        let v = start(body, edge.he_minus, "he_minus");
        // The snapshot's record is the live one: no `split_edge` before
        // this one touched this edge. So the curve is a link, and its
        // miss panics (`live::OPERATORS_KEEP_LINKS`).
        let curve = match body.edge_curve_linked(edge_key, &edge) {
            CurveGeom::Certified(c) => c.clone(),
            CurveGeom::NullScaffold(_) => unreachable!(
                "{edge_key:?} is a null edge, and every door reaching here holds its body tier-2 \
                 clean, which refuses a null edge at rest"
            ),
        };
        let (t0, t1) = curve.params();
        let roots: Vec<T> = match plane_crossing_lane(
            curve.carrier(),
            t0,
            t1,
            plane.origin,
            plane.normal.get(),
            band,
        ) {
            // Parallel either way: no root to insert, and a conic in
            // the plane has its endpoints ON through the vertex sides.
            PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { .. } | ConicPlaneMeet::Miss) => {
                continue;
            }
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(roots))) => roots,
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(fault))) => {
                return Err(SplitReduceError::CrossingEscalated {
                    edge: edge_key,
                    fault,
                });
            }
            // It passes only behind a box the plane cannot meet, read
            // here as at the gate, so its whole locus is on one side.
            PlaneCrossingLane::Unlaned => {
                if edge_clears(body, edge_key, plane, band, &BoxAxes::aimed(plane.normal)) {
                    continue;
                }
                return Err(SplitReduceError::CurvedEdgeUnsupported { edge: edge_key });
            }
            PlaneCrossingLane::Line => {
                // Endpoint verdicts strictly opposite ⇒ one
                // interpolated root.
                let crossing = matches!(
                    (sides[u], sides[v]),
                    (PlaneSide::Above, PlaneSide::Below) | (PlaneSide::Below, PlaneSide::Above)
                );
                if !crossing {
                    continue;
                }
                let dist = |vk: VertexKey| {
                    let p = body.resolve_vertex_point(vk, crate::live::Proven);
                    geom::plane_offset(plane.origin, plane.normal.get(), p)
                };
                let (d1, d2) = (dist(u), dist(v));
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
                    source: Box::new(source.from_driver()),
                }
            })?;
            sides.insert(created.vertex, PlaneSide::On);
            on_vertices.push(created.vertex);
            pieces
                .push((created.new_edge, target))
                .map_err(SplitReduceError::LineageRow)?;
            target = created.new_edge;
        }
    }
    Ok(pieces)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_brep::curve_face::plane_crossing::{
        ConicPlaneMeet, ConicRootFault, PlaneCrossingLane, plane_crossing_lane,
    };
    use geom_core::{Band, Point3, Vec3};

    /// **A graze's side decided zero keeps its margin, and each door that
    /// reads the fault ends on it.** A circle of radius 2ε in a plane
    /// square to the query plane, its centre ε off it, at K = 2: the
    /// parallel gate passes `R = 2ε`, the belly decides the graze
    /// (`R − |D| = ε`), and the graze's side `D = ±ε` decides zero, which
    /// only K ≤ 2 reaches. The nonzero gate rejects it carrying `D`, at
    /// every ε a run at K = 2 admits, either side, either normal. Read as a margin, it ends as
    /// one: the split's lever; the Boolean's edge-on-plane gap with
    /// its tighter tolerance; the census's coincidence menu, not the
    /// defect ending a poisoned margin takes.
    #[test]
    fn a_graze_side_decided_zero_keeps_its_margin() {
        use crate::boolean::refusal_routes::{BooleanDecision, DeclarationRead};
        for e in [1e-3, 1e-6, 1e-9, 1e-11] {
            let band = Band::new(e, 2.0 * e).unwrap();
            for (off, nz) in [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)] {
                let label = format!("ε = {e:e}, centre {off}·ε, normal {nz}·z");
                let conic = geom::Ellipse::new(
                    geom::Frame::new(
                        Point3::new(0.0, 0.0, off * e),
                        Vec3::unit_x(),
                        Vec3::unit_z(),
                    ),
                    2.0 * e,
                    2.0 * e,
                );
                let normal = Vec3::new(0.0, 0.0, nz);
                let got = plane_crossing_lane(
                    &geom::Curve3::Ellipse(conic),
                    0.0,
                    core::f64::consts::TAU,
                    Point3::origin(),
                    normal,
                    band,
                );
                let PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(
                    fault @ ConicRootFault::BellyGraze(diag),
                ))) = got
                else {
                    panic!("{label}: {got:?}");
                };
                assert_eq!(diag.predicate, Some("plane_crossing_graze_side"), "{label}");
                assert_eq!(
                    diag.margin.rejected_sign(),
                    Some(geom_core::Sign::Zero),
                    "{label}"
                );
                assert_eq!(
                    diag.margin.diagnostic_f64_for_error_text(),
                    geom_core::ErrorTextReading::Value(off * nz * e),
                    "{label}: the centre's offset"
                );
                let split = crate::SplitReduceError::CrossingEscalated {
                    edge: crate::EdgeKey::default(),
                    fault,
                }
                .render(geom_brep::recourse::Reading::Build);
                assert!(
                    split.contains("within the zero band")
                        && split.ends_with(&format!(
                            ". Recourse: {}",
                            super::super::SPLIT_COINCIDENCE_RECOURSE
                        )),
                    "{label}: the split's lever: {split}"
                );
                let boolean = crate::BooleanError::Escalated {
                    decision: BooleanDecision::of_conic_root(fault, DeclarationRead::Moot),
                    diag,
                }
                .render(geom_brep::recourse::Reading::Build);
                assert!(
                    boolean.contains("if this gap is intended, tighten the tolerance below")
                        && !boolean.contains("unreadable"),
                    "{label}: the Boolean's sized ending: {boolean}"
                );
                let census = crate::ValidationError::CensusEscalated { cause: diag }
                    .render(geom_brep::recourse::Reading::AtRest);
                assert!(
                    census.ends_with(&format!("Recourse: {}", geom_core::COINCIDENCE_RECOURSE))
                        && !census.contains("defect"),
                    "{label}: the census's coincidence menu: {census}"
                );
            }
        }
    }
}

/// **A crossing past the band of a thin ellipse's fast end splits**
/// (`work/onecopy/a-plane-crossing-past-the-band-of-a-thin-ellipses-fast-end-read-at-the-end.md`,
/// `work/onecopy/the-euler-splits-interiority-gate-meters-an-end-gap-at-the-speed-floor.md`).
/// A planar cap on the half of a 1000 × 1 ellipse (the unit cylinder cut
/// by a plane `cos φ = 10⁻³` off its axis) from one minor vertex to the
/// other, through its major end: its two ends are its fast ends, where it
/// runs 1000 m/rad. A plane square to the major axis, `s` along it from
/// the ends (50, 500 and 5000 zeros), crosses the arc `s` (by chord) from
/// each end. Three readings of the speed floor (1 m/rad), which reads
/// `s/1000` at the end or in band, stood between the crossing and its
/// split, one after the other:
/// - main's plane door placed the root at the end and dropped it;
/// - PR 6687's door placed it inside, and `split_edge`'s interiority gate
///   refused;
/// - with that gate on the span's reading, certification's check 2
///   refused the child from the end to the root (`IntervalNotForward`).
///
/// Each now reads the chord. A cut 10⁵ zeros in splits too, as a control.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod thin_ellipse_rows {
    use geom::{Curve3, Surface};
    use geom_brep::{CurveSelector, EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::{Band, Point3, Tol, Vec3};

    use crate::body::Body;
    use crate::construction::FaceSurface;
    use crate::euler::{MefSite, MevSite};

    /// The cap, and the ellipse's major direction.
    fn thin_cap(tol: Tol) -> (Body<f64>, Vec3<f64>) {
        let c = 1e-3_f64;
        let s = (1.0 - c * c).sqrt();
        let (axis, major) = (Vec3::new(-s, 0.0, c), Vec3::new(c, 0.0, s));
        let carrier = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(Point3::origin(), axis, major),
            1.0 / c,
            1.0,
        ));
        let half = core::f64::consts::FRAC_PI_2;
        let span = (half, 3.0 * half);
        let (p0, p1) = (carrier.eval(span.0), carrier.eval(span.1));
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p0, true).unwrap();
        let wall = body.add_surface(Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        });
        let plane = body.add_surface(Surface::Plane {
            origin: Point3::origin(),
            normal: axis,
            u_ref: major,
        });
        let arc = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p1,
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        pair: SurfacePair::new(wall, plane),
                        selector: CurveSelector::Witness(carrier.eval(2.0 * half)),
                    },
                    carrier,
                    param_start: span.0,
                    param_end: span.1,
                },
                tol,
            )
            .unwrap();
        body.mef(
            MefSite::Chords {
                he1: arc.he_minus,
                he2: arc.he_plus,
            },
            EdgeCurveSpec::line_between(p1, p0),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap();
        (body, major)
    }

    #[test]
    fn a_crossing_at_a_thin_ellipses_fast_end_splits() {
        splits_at_this_runs_band();
    }

    /// The row at ε 1e-12 and 1e-6 whatever ε this run reads: each in a
    /// child process of this test binary at that ε (the child row
    /// [`a_crossing_at_a_thin_ellipses_fast_end_splits_child`]), so a run
    /// at the default ε holds the band's two ends too.
    #[test]
    fn a_crossing_at_a_thin_ellipses_fast_end_splits_at_the_bands_ends() {
        let row = "splitting::classify::thin_ellipse_rows::a_crossing_at_a_thin_ellipses_fast_end_splits_child";
        for eps in [1e-12, 1e-6] {
            let exe = std::env::current_exe().expect("a running test binary has a path");
            let out = std::process::Command::new(&exe)
                .args([row, "--exact", "--ignored", "--test-threads=1"])
                .env(geom_core::tolerance::ENV_EPS, format!("{eps:e}"))
                .env_remove("CAD_AMBIGUITY_K")
                .output()
                .expect("the child row spawns");
            let stdout = String::from_utf8_lossy(&out.stdout);
            assert!(
                out.status.success() && stdout.contains("1 passed"),
                "at ε = {eps:e} the child row failed or did not run:\n{stdout}\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    /// The child the row above runs at each ε: the row itself.
    #[test]
    #[ignore = "child row: run at a stated ε by a_crossing_at_a_thin_ellipses_fast_end_splits_at_the_bands_ends"]
    fn a_crossing_at_a_thin_ellipses_fast_end_splits_child() {
        splits_at_this_runs_band();
    }

    /// The cuts and the control at the run's own band.
    fn splits_at_this_runs_band() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let cut = |s: f64| {
            let (mut body, major) = thin_cap(tol);
            let plane = crate::test_support_fixtures::split_plane(
                Point3::origin() + major * (-s),
                major,
                tol,
            );
            let (mut sides, mut on) = super::classify_vertices(&body, &plane, band).unwrap();
            let edges = body.edges().count();
            super::insert_crossings(&mut body, &plane, &mut sides, &mut on, tol)
                .map(|_| body.edges().count() - edges)
        };
        // The chord past the band; its gap at the floor, `s/1000`, at a
        // twentieth of the zero, half of it, and in the band's gap.
        for s in [50.0, 500.0, 5000.0].map(|k| k * band.zero()) {
            match cut(s) {
                Ok(pieces) => assert_eq!(pieces, 2, "s = {s:e}: both crossings split"),
                Err(e) => panic!("s = {s:e}: both crossings split, not {e:?}"),
            }
        }
        assert_eq!(
            cut(1e5 * band.zero()).unwrap(),
            2,
            "a hundred thousand zeros in, both crossings split"
        );
    }
}

/// **A torn curve panics at the split gate and at the crossing
/// insertion**: the miss is a kernel defect, never an operand's
/// scaffolding.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_rows {
    use geom_core::{Band, Point3, Tol, Vec3};

    use crate::entity::EntityId;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    #[test]
    fn a_torn_curve_panics_at_the_gate_and_the_crossing_insertion() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(tol).body;
        let plane = crate::test_support_fixtures::split_plane(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::unit_z(),
            tol,
        );
        let (edge, curve) = body.edges().next().map(|(k, e)| (k, e.curve)).unwrap();
        let (mut sides, mut on) = super::classify_vertices(&body, &plane, band).unwrap();
        body.curves.remove(curve);
        let named = format!("{}'s curve names", EntityId::Edge(edge));
        assert_torn_op_panics("carrier_gate", &mut body, &[&named, ROW_FOUR], |b| {
            super::carrier_gate(b, &plane, band)
        });
        assert_torn_op_panics("insert_crossings", &mut body, &[&named, ROW_FOUR], |b| {
            super::insert_crossings(b, &plane, &mut sides, &mut on, tol)
        });
    }

    /// **The gate's reads past a face it walked panic on a torn link.**
    /// `edge_clears` reads both faces of an edge the plane crosses, and a
    /// half whose loop was dropped panics naming it, where a read of the
    /// miss as no face would clear the edge on its other face alone.
    /// The gate's face reach (`census::face_reach_in`) reads the face's surface, and a dropped one
    /// panics naming it, where a read of the miss as no claim to make
    /// would answer the census's box.
    #[test]
    fn the_gate_panics_on_a_torn_face_of_an_edge_and_a_torn_surface() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use geom::cover::BoxAxes;
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let fresh = || crate::test_support_fixtures::geometric_cube::<f64>(tol).body;
        let plane = crate::test_support_fixtures::split_plane(
            Point3::new(0.0, 0.0, 0.5),
            Vec3::unit_z(),
            tol,
        );
        let frame = BoxAxes::aimed(plane.normal);

        let mut body = fresh();
        let crosses = |b: &crate::body::Body<f64>, e| {
            let (u, v) = b.edge_vertices(e).unwrap();
            let z = |w| b.get_point(b.get_vertex(w).unwrap().point).unwrap().z;
            (z(u) - 0.5) * (z(v) - 0.5) < 0.0
        };
        let (edge, data) = body
            .edges()
            .find(|&(k, _)| crosses(&body, k))
            .map(|(k, e)| (k, e.clone()))
            .unwrap();
        assert!(
            !super::edge_clears(&body, edge, &plane, band, &frame),
            "the plane meets an edge it crosses"
        );
        let lost = body.get_half_edge(data.he_plus).unwrap().parent_loop;
        body.loops.remove(lost);
        let named = format!(
            "{}'s parent_loop names {}",
            EntityId::HalfEdge(data.he_plus),
            EntityId::Loop(lost)
        );
        assert_torn_op_panics(
            "edge_clears",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| super::edge_clears(b, edge, &plane, band, &frame),
        );

        let mut body = fresh();
        let (face, surface) = body.faces().next().map(|(k, f)| (k, f.surface)).unwrap();
        assert!(
            crate::census::face_reach_in(&body, face, band, &frame).is_some(),
            "a sound face has a reach"
        );
        body.surfaces.remove(surface);
        let named = format!("{}'s surface names", EntityId::Face(face));
        assert_torn_op_panics(
            "face_reach_in",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| crate::census::face_reach_in(b, face, band, &frame),
        );
    }
}
