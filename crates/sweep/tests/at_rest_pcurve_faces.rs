//! **Tier 3's pcurve pass on a torus wall** (C4): a strut no torus
//! holds is a defect tier 3 and the mint name; a stored row stated over
//! more of its carrier than the edge spans is refused, on a complete
//! face and a half-minted one alike; and a face bounded by a Villarceau
//! arc mints.
//!
//! The strut rows run on a quarter revolve of a profile whose one arc
//! is centred off the axis, so its wall is a minted torus, with struts
//! added to that wall:
//!
//! - an OBLIQUE circle (neither a parallel, a meridian nor a Villarceau
//!   circle), which the torus's incidence test reads off the torus
//!   (`CarrierOffChart`);
//! - a circle ⊥ the axis through the tube's crest, centred off the
//!   axis, which the incidence test reads on the torus while it is no
//!   circle of it (`CarrierGrazesChart`);
//! - a straight line, which no torus holds (`CarrierOffChart`).
//!
//! Adopted from PCERT reviewer R2's probes on PR 3759.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, Grazer, Pcurve, PcurveCache, PcurveCertifyError};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::pcurves::validate_pcurves;
use topo::{Body, FaceKey, HalfEdgeKey, MevSite, PcurveMintError};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A quarter revolve about Y of the region under a quarter arc of
/// radius 0.5 centred at (1, 0): its curved wall is a torus.
fn torus_quarter() -> Body<f64> {
    let bulge = (core::f64::consts::FRAC_PI_8).tan();
    revolved_about_y(
        vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.5, 0.0), bulge),
            (Point2::new(1.0, 0.5), 0.0),
        ],
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    )
}

/// The minted torus wall and its outer cycle.
fn torus_wall(body: &Body<f64>) -> (FaceKey, Vec<HalfEdgeKey>) {
    body.faces()
        .find_map(|(fk, f)| {
            let Surface::Torus { .. } = *body.get_surface(f.surface).unwrap() else {
                return None;
            };
            let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            assert!(body.pcurve(first).is_some(), "the torus wall is minted");
            Some((fk, body.loop_cycle(first).unwrap()))
        })
        .expect("the revolve has a torus wall")
}

fn start_point(body: &Body<f64>, he: HalfEdgeKey) -> Point3<f64> {
    let v = body.get_half_edge(he).unwrap().start;
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v * (1.0 / v.dot(v).sqrt())
}

/// A circle through `p` in a plane tilted off both the torus's
/// parallels (⊥ Y) and its meridians (∋ Y).
fn oblique_circle_through(p: Point3<f64>) -> Curve3<f64> {
    let axis = unit(Vec3::new(0.7, 1.0, 0.4));
    let helper = Vec3::unit_x();
    let u = unit(helper - axis * helper.dot(axis));
    let radius = 0.3;
    Curve3::Circle {
        center: p - u * radius,
        axis,
        radius,
        u_ref: u,
    }
}

/// A circle ⊥ the torus's axis through `p` on its crest, centred
/// `2·K·ε` off the axis: past the centring band, while it leaves the
/// crest by only the square of that.
fn crest_circle_through(p: Point3<f64>) -> Curve3<f64> {
    let delta = 2.0 * tol().k() * tol().eps();
    let center = Point3::new(0.0, p.y, delta);
    let radius = p.distance(center);
    Curve3::Circle {
        center,
        axis: Vec3::unit_y(),
        radius,
        u_ref: (p - center) * (1.0 / radius),
    }
}

fn strut(body: &mut Body<f64>, he: HalfEdgeKey, end: Point3<f64>, spec: EdgeCurveSpec<f64>) {
    body.mev(MevSite::Fan { he1: he, he2: he }, end, spec, tol())
        .unwrap();
}

fn arc_strut(body: &mut Body<f64>, he: HalfEdgeKey, carrier: Curve3<f64>, span: f64) {
    let end = carrier.eval(span);
    let spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, span).unwrap();
    strut(body, he, end, spec);
}

/// A straight strut from `he`'s start, 0.2 up the axis.
fn line_strut(body: &mut Body<f64>, he: HalfEdgeKey) {
    let p = start_point(body, he);
    let end = p + Vec3::new(0.05, 0.2, 0.05);
    strut(body, he, end, EdgeCurveSpec::line_between(p, end));
}

fn off_chart(e: &PcurveMintError) -> bool {
    matches!(
        e,
        PcurveMintError::Certify {
            error: PcurveCertifyError::CarrierOffChart { .. },
            ..
        }
    )
}

/// Tier 3's findings and the wall's mint, with the oblique strut at
/// cycle position `g` and the off-chart strut at `o` (`None` leaves
/// that strut out), and the body.
fn verdicts(
    g: Option<usize>,
    o: Option<usize>,
) -> (
    Vec<PcurveMintError>,
    Result<usize, PcurveMintError>,
    Body<f64>,
) {
    let mut body = torus_quarter();
    let (wall, cycle) = torus_wall(&body);
    if let Some(i) = g {
        let p = start_point(&body, cycle[i]);
        arc_strut(&mut body, cycle[i], oblique_circle_through(p), 0.2);
    }
    if let Some(i) = o {
        line_strut(&mut body, cycle[i]);
    }
    let findings = validate_pcurves(&body, band());
    let mut minted = body.clone();
    let mint = topo::mint_pcurves_of(&mut minted, &[wall], tol());
    (findings, mint, body)
}

fn grazes(e: &PcurveMintError) -> bool {
    matches!(
        e,
        PcurveMintError::Certify {
            error: PcurveCertifyError::CarrierGrazesChart {
                grazer: Grazer::TorusCircle,
                ..
            },
            ..
        }
    )
}

/// **Each strut alone is refused by its own class**, at whichever
/// corner it stands: the oblique circle and the line are off the torus,
/// and the crest circle grazes it. Tier 3 names the strut and the mint
/// refuses with it; none is excused.
#[test]
fn each_strut_alone_is_refused_by_its_own_class() {
    for i in 0..2 {
        let (f, m, _) = verdicts(Some(i), None);
        assert!(
            matches!(f.as_slice(), [e] if off_chart(e)),
            "oblique at {i}: {f:?}"
        );
        assert!(
            matches!(&m, Err(e) if off_chart(e)),
            "oblique at {i}: {m:?}"
        );
        let (f, m, _) = verdicts(None, Some(i));
        assert!(
            matches!(f.as_slice(), [e] if off_chart(e)),
            "line at {i}: {f:?}"
        );
        assert!(matches!(&m, Err(e) if off_chart(e)), "line at {i}: {m:?}");
    }
    let mut body = torus_quarter();
    let (wall, cycle) = torus_wall(&body);
    let crest = cycle
        .iter()
        .copied()
        .find(|&he| (start_point(&body, he).y - 0.5).abs() < 1e-12)
        .expect("the wall has a corner on the tube's crest");
    let p = start_point(&body, crest);
    arc_strut(&mut body, crest, crest_circle_through(p), 0.2);
    let f = validate_pcurves(&body, band());
    assert!(
        matches!(f.as_slice(), [e] if grazes(e)),
        "crest circle: {f:?}"
    );
    let m = topo::mint_pcurves_of(&mut body, &[wall], tol());
    assert!(matches!(&m, Err(e) if grazes(e)), "crest circle: {m:?}");
}

/// **A stale wide row is refused, complete or half-minted.** On a
/// minted cylinder wall, one row is re-certified over 0.4 rad more of
/// its carrier than the edge spans (a row from before a split, say), at
/// either end, with the face left complete or a different half-edge's
/// row detached: at every choice of the end and the gap, tier 3 refuses
/// the wide row `RowInterval` — its interval is not its edge's.
#[test]
fn a_stale_wide_row_is_refused_complete_or_half_minted() {
    let base =
        crate::common::operands::n_arc_boss::<f64>(geom_core::Point2::new(0.0, 0.0), 3, 0.0, 1.0);
    let (wall, cycle) = base
        .faces()
        .find_map(|(fk, f)| {
            if !matches!(
                base.get_surface(f.surface).unwrap(),
                Surface::Cylinder { .. }
            ) {
                return None;
            }
            let topo::LoopBoundary::Cycle { first } = base.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            Some((fk, base.loop_cycle(first).unwrap()))
        })
        .expect("the boss has a cylinder wall");
    let surface = base
        .get_surface(base.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    let mut cases = 0;
    for (i, &h1) in cycle.iter().enumerate() {
        let edge = base.get_edge(base.get_half_edge(h1).unwrap().edge).unwrap();
        let carrier = base
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap()
            .carrier()
            .clone();
        if !matches!(carrier, Curve3::Circle { .. }) {
            continue;
        }
        let cache = base.pcurve(h1).unwrap().clone();
        let (t0, t1) = cache.params();
        for (lo, hi) in [(t0, t1 + 0.4), (t0 - 0.4, t1)] {
            for (j, &h2) in cycle.iter().enumerate() {
                let mut body = base.clone();
                let wide = cache.pcurve().clone();
                let row = PcurveCache::certify(wide, lo, hi, &carrier, &surface, band())
                    .expect("the carrier's own image certifies over a longer span");
                body.attach_pcurve(h1, row);
                if j != i {
                    body.detach_pcurve(h2);
                }
                let f = validate_pcurves(&body, band());
                assert!(
                    f.contains(&PcurveMintError::RowInterval { half_edge: h1 }),
                    "row {i} wide over [{lo}, {hi}], gap {j} (none where {j} = {i}): {f:?}"
                );
                cases += 1;
            }
        }
    }
    assert!(
        cases >= 8,
        "the boss wall has arc rows to widen: {cases} cases"
    );
}

/// A parallel of the torus through `p`, its axis tilted `tilt` off the
/// torus's own: a covered class whose image derives, but whose
/// certificate does not decide at a tilt inside the band's escalation
/// zone.
fn tilted_parallel_through(p: Point3<f64>, tilt: f64) -> Curve3<f64> {
    let r = (p.x * p.x + p.z * p.z).sqrt();
    Curve3::Circle {
        center: Point3::new(0.0, p.y, 0.0),
        axis: unit(Vec3::new(tilt, 1.0, 0.0)),
        radius: r,
        u_ref: Vec3::new(p.x / r, 0.0, p.z / r),
    }
}

fn certificate_refused(e: &PcurveMintError) -> bool {
    matches!(
        e,
        PcurveMintError::Certify {
            error: PcurveCertifyError::Escalated { .. },
            ..
        }
    )
}

/// **A covered strut whose refusal the band cannot decide is named** —
/// a parallel tilted off the torus's axis. Tilted 2ε, it refuses in its
/// derivation (`ChartWinding`); tilted ε, its image derives and only
/// its certificate refuses (`Envelope`). Tier 3 names it, the mint
/// refuses with it, and the reversed body reads the same, at either
/// corner.
#[test]
fn a_tilted_parallel_strut_refuses_its_certificate() {
    let (f, _, _) = verdicts(None, None);
    assert!(f.is_empty(), "the bare wall is clean: {f:?}");
    let eps = tol().eps();
    for (c, k) in [(0, 2.0), (1, 2.0), (1, 1.0)] {
        let tilt = k * eps;
        let mut body = torus_quarter();
        let (wall, cycle) = torus_wall(&body);
        let p = start_point(&body, cycle[c]);
        arc_strut(&mut body, cycle[c], tilted_parallel_through(p, tilt), 0.2);
        let f = validate_pcurves(&body, band());
        assert!(
            matches!(f.as_slice(), [e] if certificate_refused(e)),
            "parallel tilted {tilt} at {c}: tier 3 names the certificate: {f:?}"
        );
        let mut minted = body.clone();
        let m = topo::mint_pcurves_of(&mut minted, &[wall], tol());
        assert!(
            matches!(&m, Err(e) if certificate_refused(e)),
            "parallel tilted {tilt} at {c}: the mint refuses with it: {m:?}"
        );
        let fr = validate_pcurves(&body.revert(), band());
        assert!(
            matches!(fr.as_slice(), [e] if certificate_refused(e)),
            "parallel tilted {tilt} at {c}: reversed, the same verdict: {fr:?}"
        );
    }
}

fn discontinuous(e: &PcurveMintError) -> bool {
    matches!(
        e,
        PcurveMintError::LoopDiscontinuity { .. } | PcurveMintError::LoopNotClosed { .. }
    )
}

/// **A single row restated a whole period over is refused at every
/// cycle position, complete or half-minted, and reversed.** On the
/// minted cylinder wall of a three-arc boss, one arc row is restated one
/// period over, its interval and image shifted together, so its ends
/// still evaluate to its edge's vertices (`RowInterval` reads it equal)
/// and it certifies. Only the loop's one-branch continuity can see it.
/// At every row, either shift, and every choice of one other detached
/// row (or none), tier 3 names a discontinuity: the joint into the
/// cycle's first half-edge — the wrap — is read as the closure whenever
/// the chain reaches it, and a gap is carried by the image the mint
/// would derive there. The reversed body (`Body::revert`) reads the
/// same.
///
/// Adopted from the PCERT delta reviewer's probe on PR 3759, where the
/// row at cycle position 0 with its successor detached read only the
/// gap.
#[test]
fn a_row_shifted_a_whole_period_is_refused_at_every_position() {
    let base =
        crate::common::operands::n_arc_boss::<f64>(geom_core::Point2::new(0.0, 0.0), 3, 0.0, 1.0);
    let (wall, cycle) = base
        .faces()
        .find_map(|(fk, f)| {
            if !matches!(
                base.get_surface(f.surface).unwrap(),
                Surface::Cylinder { .. }
            ) {
                return None;
            }
            let topo::LoopBoundary::Cycle { first } = base.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            Some((fk, base.loop_cycle(first).unwrap()))
        })
        .expect("the boss has a cylinder wall");
    let surface = base
        .get_surface(base.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    let tau = core::f64::consts::TAU;
    let mut cases = 0;
    for (i, &h1) in cycle.iter().enumerate() {
        let edge = base.get_edge(base.get_half_edge(h1).unwrap().edge).unwrap();
        let carrier = base
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap()
            .carrier()
            .clone();
        if !matches!(carrier, Curve3::Circle { .. }) {
            continue;
        }
        let cache = base.pcurve(h1).unwrap().clone();
        let (t0, t1) = cache.params();
        for shift in [tau, -tau] {
            let (lo, hi) = (t0 + shift, t1 + shift);
            for (j, &h2) in cycle.iter().enumerate() {
                let mut body = base.clone();
                let image = cache.pcurve().clone();
                let row = PcurveCache::certify(image, lo, hi, &carrier, &surface, band())
                    .expect("the row certifies one period over");
                body.attach_pcurve(h1, row);
                if j != i {
                    body.detach_pcurve(h2);
                }
                let f = validate_pcurves(&body, band());
                assert!(
                    f.iter().any(discontinuous),
                    "row {i} shifted {shift}, gap {j} (none where {j} = {i}): {f:?}"
                );
                let fr = validate_pcurves(&body.revert(), band());
                assert!(
                    fr.iter().any(discontinuous),
                    "row {i} shifted {shift}, gap {j}, reversed: {fr:?}"
                );
                cases += 1;
            }
        }
    }
    assert!(
        cases >= 16,
        "the boss wall has arc rows to shift: {cases} cases"
    );
}

/// **A valid loop moved a whole period over reads clean at every gap.**
/// Every row of the boss wall's loop is restated one period over,
/// consistently — the re-statement `Body::revert` makes. Complete, it
/// reads clean; with any one half-edge detached, tier 3 reports that gap
/// and nothing else, and the reversed body reads the same. The wrap the
/// walk allows sits at the cycle's first half-edge, so where that
/// half-edge is the gap the wrap passes to the next stored row rather
/// than to the gap's image on its principal branch.
///
/// Adopted from the PCERT delta reviewer's round-2 probe on PR 3759,
/// where the gap at cycle position 0 read a spurious `LoopDiscontinuity`.
#[test]
fn a_loop_moved_a_period_over_reads_only_its_gap_at_every_position() {
    let base =
        crate::common::operands::n_arc_boss::<f64>(geom_core::Point2::new(0.0, 0.0), 3, 0.0, 1.0);
    let (wall, cycle) = base
        .faces()
        .find_map(|(fk, f)| {
            if !matches!(
                base.get_surface(f.surface).unwrap(),
                Surface::Cylinder { .. }
            ) {
                return None;
            }
            let topo::LoopBoundary::Cycle { first } = base.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            Some((fk, base.loop_cycle(first).unwrap()))
        })
        .expect("the boss has a cylinder wall");
    let surface = base
        .get_surface(base.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    let tau = core::f64::consts::TAU;
    for shift in [tau, -tau] {
        let mut moved = base.clone();
        for &he in &cycle {
            let cache = base.pcurve(he).unwrap();
            let image = cache
                .pcurve()
                .map_affine(|p| Point2::new(p.x + shift, p.y), |v| v);
            let edge = base.get_edge(base.get_half_edge(he).unwrap().edge).unwrap();
            let carrier = base
                .get_curve_geom(edge.curve)
                .unwrap()
                .certified()
                .unwrap()
                .carrier()
                .clone();
            let (t0, t1) = cache.params();
            let row = PcurveCache::certify(image, t0, t1, &carrier, &surface, band())
                .expect("the row certifies a period over");
            moved.attach_pcurve(he, row);
        }
        let complete = validate_pcurves(&moved, band());
        assert!(complete.is_empty(), "moved {shift}, complete: {complete:?}");
        for (j, &gap) in cycle.iter().enumerate() {
            let mut body = moved.clone();
            body.detach_pcurve(gap);
            let only_gap = [PcurveMintError::MissingCache { half_edge: gap }];
            let f = validate_pcurves(&body, band());
            assert_eq!(f, only_gap, "moved {shift}, gap {j}");
            let fr = validate_pcurves(&body.revert(), band());
            assert!(
                fr.len() == 1 && matches!(fr[0], PcurveMintError::MissingCache { .. }),
                "moved {shift}, gap {j}, reversed: {fr:?}"
            );
        }
    }
}

/// The vertex of `face`'s outer cycle at `p`.
fn half_edge_at(body: &Body<f64>, face: FaceKey, p: Point3<f64>) -> HalfEdgeKey {
    let topo::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("the face's outer loop is a cycle")
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .find(|&he| start_point(body, he).distance(p) < 1e-9)
        .unwrap_or_else(|| panic!("no vertex of the face at {p:?}"))
}

/// **A torus face bounded by a Villarceau arc mints, on either
/// family.** A half revolve (azimuth `0 → π`) of the circle of radius
/// `r` about `(R, 0)` has two torus faces, the upper and lower halves,
/// each with corners on the outer equator at azimuths `0` and `π` and
/// on the inner equator at the same two. The Villarceau circle centred
/// `r` along `+x` meets the equator at `(R + r, 0, 0)`, the outer corner
/// at `0`, and half a turn later at `(−(R − r), 0, 0)`, the inner corner
/// at `π`; between them it crosses the upper half on one family and
/// the lower on the other. `mef` joins the two corners with that arc,
/// described in the torus's chart (`EdgeDescription::Chart`), and mints
/// the split face's rows: each half of the arc is a `FocalSection` row,
/// each of the two faces' loops closes, and tier 3 reads the body
/// clean.
#[test]
fn a_torus_face_bounded_by_a_villarceau_arc_mints_on_either_family() {
    let (big, r) = (2.0_f64, 0.5_f64);
    for family in [1.0_f64, -1.0] {
        let mut body = revolved_about_y(
            vec![
                (Point2::new(big + r, 0.0), 1.0),
                (Point2::new(big - r, 0.0), 1.0),
            ],
            Revolution::Partial(core::f64::consts::PI),
            tol(),
        );
        // Which way the revolve sweeps: the side its outer equator
        // crosses at azimuth π/2.
        let sweep = body
            .edges()
            .find_map(|(_, e)| {
                let m = body
                    .get_curve_geom(e.curve)
                    .unwrap()
                    .certified()
                    .unwrap()
                    .mid_point();
                (m.y.abs() < 1e-9 && (m.z.abs() - (big + r)).abs() < 1e-9).then_some(m.z.signum())
            })
            .expect("the outer equator crosses azimuth π/2");
        let (outer, inner) = (
            Point3::new(big + r, 0.0, 0.0),
            Point3::new(-(big - r), 0.0, 0.0),
        );
        // The upper (lower) half: the torus face one of whose edges
        // passes over (under) the equator.
        let face = body
            .faces()
            .find_map(|(fk, f)| {
                let Surface::Torus { .. } = *body.get_surface(f.surface).unwrap() else {
                    return None;
                };
                let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary
                else {
                    return None;
                };
                body.loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .any(|he| {
                        let e = body.get_half_edge(he).unwrap().edge;
                        let curve = body.get_edge(e).unwrap().curve;
                        let m = body
                            .get_curve_geom(curve)
                            .unwrap()
                            .certified()
                            .unwrap()
                            .mid_point();
                        m.y * family > 0.5 * r
                    })
                    .then_some(fk)
            })
            .expect("the half revolve has both torus halves");
        let chart = body.get_face(face).unwrap().surface;
        let tilt = (r / big).asin();
        let side = Vec3::new(0.0, family * tilt.sin(), sweep * tilt.cos());
        let villarceau = Curve3::Circle {
            center: Point3::new(r, 0.0, 0.0),
            axis: Vec3::unit_x().cross(side),
            radius: big,
            u_ref: Vec3::unit_x(),
        };
        assert!(villarceau.eval(0.0).distance(outer) < 1e-12);
        assert!(villarceau.eval(core::f64::consts::PI).distance(inner) < 1e-12);
        let spec = EdgeCurveSpec::arc_of_circle(villarceau, 0.0, core::f64::consts::PI)
            .unwrap()
            .at_rest_in_chart(chart, false);
        let he1 = half_edge_at(&body, face, outer);
        let he2 = half_edge_at(&body, face, inner);
        let made = body
            .mef(
                topo::MefSite::Chords { he1, he2 },
                spec,
                topo::FaceSurface::Inherit,
                tol(),
            )
            .unwrap_or_else(|e| panic!("family {family}: the Villarceau split refuses: {e}"));
        for he in [made.he_plus, made.he_minus] {
            let row = body
                .pcurve(he)
                .unwrap_or_else(|| panic!("family {family}: {he:?} has no row"));
            assert!(
                matches!(row.pcurve(), Pcurve::FocalSection { .. }),
                "family {family}: the arc's row is its focal section: {:?}",
                row.pcurve()
            );
        }
        let findings = validate_pcurves(&body, band());
        assert!(
            findings.is_empty(),
            "family {family}: tier 3 reads the split clean: {findings:?}"
        );
    }
}
