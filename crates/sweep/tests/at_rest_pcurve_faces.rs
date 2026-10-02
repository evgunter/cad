//! **Tier 3's pcurve pass reads a face whole** (C4): a face is excused
//! only when every refusal its edges meet is one whose rows are not
//! owed (an uncovered class), whatever order the walk meets them in; and
//! a stored row stated over more of its carrier than the edge spans is
//! refused against the face's own window on a half-minted face.
//!
//! The excuse rows run on a quarter revolve of `dome_profile`, whose
//! sphere wall is minted, with struts added to that wall:
//!
//! - a GENERAL circle on the sphere (neither polar nor meridian), which
//!   the closed-form lane refuses as uncovered — rows not owed;
//! - a small circle in the sphere's tangent plane at its vertex, which
//!   is OFF the sphere (`CarrierOffChart`, a defect).
//!
//! Adopted from PCERT reviewer R2's probes on PR 3759.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, PcurveCache, PcurveCertifyError};
use geom_core::{Band, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{dome_profile, revolved_about_y};
use topo::pcurves::validate_pcurves;
use topo::{Body, FaceKey, HalfEdgeKey, MevSite, PcurveMintError};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn dome_quarter() -> Body<f64> {
    revolved_about_y(
        dome_profile(1.0),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    )
}

/// The minted sphere wall, its centre and radius, and its outer cycle.
fn sphere_wall(body: &Body<f64>) -> (FaceKey, Point3<f64>, f64, Vec<HalfEdgeKey>) {
    body.faces()
        .find_map(|(fk, f)| {
            let Surface::Sphere { center, radius, .. } = *body.get_surface(f.surface).unwrap()
            else {
                return None;
            };
            let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary
            else {
                return None;
            };
            assert!(
                body.pcurve(first).is_some(),
                "the dome's sphere wall is minted"
            );
            Some((fk, center, radius, body.loop_cycle(first).unwrap()))
        })
        .expect("the dome has a sphere wall")
}

fn start_point(body: &Body<f64>, he: HalfEdgeKey) -> Point3<f64> {
    let v = body.get_half_edge(he).unwrap().start;
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v * (1.0 / v.dot(v).sqrt())
}

/// A circle on the sphere `(c, r)` through `p`, in a plane tilted off
/// both polar (⊥ Y) and meridian (∋ Y).
fn general_circle_through(c: Point3<f64>, r: f64, p: Point3<f64>) -> Curve3<f64> {
    let n = unit(Vec3::new(0.7, 1.0, 0.4));
    let d = (p - c).dot(n);
    let center = c + n * d;
    Curve3::Circle {
        center,
        axis: n,
        radius: (r * r - d * d).sqrt(),
        u_ref: unit(p - center),
    }
}

/// A small circle in the sphere's tangent plane at `p`: it meets the
/// sphere only at `p`.
fn tangent_circle_at(c: Point3<f64>, p: Point3<f64>) -> Curve3<f64> {
    let normal = unit(p - c);
    let helper = if normal.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_z()
    };
    let t = unit(helper - normal * helper.dot(normal));
    let radius = 0.1;
    let center = p + t * radius;
    Curve3::Circle {
        center,
        axis: normal,
        radius,
        u_ref: unit(p - center),
    }
}

fn strut(body: &mut Body<f64>, he: HalfEdgeKey, carrier: Curve3<f64>, span: f64) {
    let end = carrier.eval(span);
    let spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, span).unwrap();
    body.mev(MevSite::Fan { he1: he, he2: he }, end, spec, tol())
        .unwrap();
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
    let mut body = dome_quarter();
    let (wall, c, r, cycle) = sphere_wall(&body);
    if let Some(i) = g {
        let p = start_point(&body, cycle[i]);
        strut(&mut body, cycle[i], general_circle_through(c, r, p), 0.2);
    }
    if let Some(i) = o {
        let p = start_point(&body, cycle[i]);
        strut(&mut body, cycle[i], tangent_circle_at(c, p), 0.5);
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

/// **A stale wide row on a half-minted face escapes the face's
/// window.** On a minted cylinder wall, one row is re-certified over
/// 0.4 rad more of its carrier than the edge spans (a row from before a
/// split, say), at either end, and a different half-edge's row is
/// detached: at every choice of the gap and the end, tier 3 refuses the
/// wide row `TrimEscape` against the window the face's derivation hulls
/// out to. Kills the mutant that hulls a half-minted face's window from
/// its own stored rows.
#[test]
fn a_stale_wide_row_on_a_half_minted_face_escapes_the_faces_window() {
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
                if j == i {
                    continue;
                }
                let mut body = base.clone();
                let wide = cache.pcurve().clone();
                let window = wide.chart_box(lo, hi);
                let row = PcurveCache::certify(wide, lo, hi, &carrier, &surface, window, band())
                    .expect("the carrier's own image certifies over a longer span");
                body.attach_pcurve(h1, row);
                body.detach_pcurve(h2);
                let f = validate_pcurves(&body, band());
                assert!(
                    f.iter().any(|e| matches!(
                        e,
                        PcurveMintError::Certify {
                            half_edge,
                            error: PcurveCertifyError::TrimEscape { .. },
                        } if *half_edge == h1
                    )),
                    "row {i} wide over [{lo}, {hi}], gap {j}: {f:?}"
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
