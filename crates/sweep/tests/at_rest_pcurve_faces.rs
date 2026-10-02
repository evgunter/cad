//! **Tier 3's pcurve pass reads a face whole** (C4): a face is excused
//! only when every refusal its edges meet is one whose rows are not
//! owed (an uncovered class), whatever order the walk meets them in; and
//! a stored row stated over more of its carrier than the edge spans is
//! refused, on a complete face and a half-minted one alike.
//!
//! The excuse rows run on a quarter revolve of a profile whose one arc
//! is centred off the axis, so its wall is a minted torus, with struts
//! added to that wall:
//!
//! - an OBLIQUE circle (neither a parallel nor a meridian), which the
//!   closed-form lane refuses as uncovered (`TorusGeneralCircle`) —
//!   rows not owed;
//! - a straight line, which no torus holds (`CarrierOffChart`, a
//!   defect).
//!
//! (R2's probes used a sphere's general circle as the uncovered class;
//! PR 3733 gave that class its route, so the rows moved to the torus.)
//!
//! Adopted from PCERT reviewer R2's probes on PR 3759.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, PcurveCache, PcurveCertifyError};
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

/// Tier 3's findings and the wall's mint, with the general strut at
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

/// **The controls: each strut alone.** The uncovered strut's wall is
/// excused, tier 3 clean and the mint storing nothing on it; the
/// off-chart strut's wall is loud in both, at whichever corner it
/// stands.
#[test]
fn each_strut_alone_is_excused_or_refused_by_its_own_class() {
    let (f, m, _) = verdicts(Some(0), None);
    assert!(f.is_empty(), "the uncovered strut is excused: {f:?}");
    assert_eq!(m, Ok(0), "the mint excuses the wall, storing nothing");
    for i in 0..2 {
        let (f, m, _) = verdicts(None, Some(i));
        assert!(matches!(f.as_slice(), [e] if off_chart(e)), "{f:?}");
        assert!(matches!(&m, Err(e) if off_chart(e)), "{m:?}");
    }
}

/// **An uncovered strut masks nothing.** The same wall carries the
/// off-chart strut beside the uncovered one, in four cycle orders: the
/// face is read whole before it is excused, so tier 3 names the
/// off-chart strut, and the mint refuses with it, in every order — and
/// in every order the reversed body (`Body::revert`, which carries the
/// rows and walks each loop the other way) reads the same.
#[test]
fn an_uncovered_strut_masks_no_off_chart_strut_in_any_cycle_order() {
    for (g, o) in [(0, 1), (1, 0), (0, 2), (2, 0)] {
        let (f, m, body) = verdicts(Some(g), Some(o));
        assert!(
            matches!(f.as_slice(), [e] if off_chart(e)),
            "general at {g}, off-chart at {o}: tier 3 names the off-chart strut: {f:?}"
        );
        assert!(
            matches!(&m, Err(e) if off_chart(e)),
            "general at {g}, off-chart at {o}: the mint refuses with it: {m:?}"
        );
        let reverted = body.revert().unwrap();
        let fr = validate_pcurves(&reverted, band());
        assert!(
            matches!(fr.as_slice(), [e] if off_chart(e)),
            "general at {g}, off-chart at {o}: reversed, the same verdict: {fr:?}"
        );
    }
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
                let window = wide.chart_box(lo, hi);
                let row = PcurveCache::certify(wide, lo, hi, &carrier, &surface, window, band())
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

/// **An uncovered strut masks no certificate either.** A covered strut
/// whose refusal the band cannot decide — a parallel tilted off the
/// torus's axis — stands on the same wall as the uncovered oblique
/// strut. Tilted 2e-9, it refuses in its derivation (`ChartWinding`);
/// tilted 1e-9 at the second corner, its image derives and only its
/// certificate refuses (`Envelope`), which an excused face once left
/// unread. The face is excused only when every refusal is not owed,
/// and the covered strut's is one of its refusals, so tier 3 names it,
/// the mint refuses with it, and the reversed body reads the same, in
/// every order.
#[test]
fn an_uncovered_strut_masks_no_refused_certificate() {
    let (f, _, _) = verdicts(None, None);
    assert!(f.is_empty(), "the bare wall is clean: {f:?}");
    for (g, c, tilt) in [(1, 0, 2e-9), (0, 1, 2e-9), (0, 1, 1e-9), (2, 1, 1e-9)] {
        let mut body = torus_quarter();
        let (wall, cycle) = torus_wall(&body);
        let p = start_point(&body, cycle[c]);
        arc_strut(&mut body, cycle[c], tilted_parallel_through(p, tilt), 0.2);
        let alone = validate_pcurves(&body, band());
        assert!(
            matches!(alone.as_slice(), [e] if certificate_refused(e)),
            "the control: the tilted parallel alone refuses its certificate: {alone:?}"
        );
        let p = start_point(&body, cycle[g]);
        arc_strut(&mut body, cycle[g], oblique_circle_through(p), 0.2);
        let f = validate_pcurves(&body, band());
        assert!(
            matches!(f.as_slice(), [e] if certificate_refused(e)),
            "general at {g}, parallel tilted {tilt} at {c}: tier 3 names the certificate: {f:?}"
        );
        let mut minted = body.clone();
        let m = topo::mint_pcurves_of(&mut minted, &[wall], tol());
        assert!(
            matches!(&m, Err(e) if certificate_refused(e)),
            "general at {g}, parallel tilted {tilt} at {c}: the mint refuses with it: {m:?}"
        );
        let fr = validate_pcurves(&body.revert().unwrap(), band());
        assert!(
            matches!(fr.as_slice(), [e] if certificate_refused(e)),
            "general at {g}, parallel tilted {tilt} at {c}: reversed, the same verdict: {fr:?}"
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
                let window = image.chart_box(lo, hi);
                let row = PcurveCache::certify(image, lo, hi, &carrier, &surface, window, band())
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
                let fr = validate_pcurves(&body.revert().unwrap(), band());
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
