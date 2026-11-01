//! **The join levers a vertex's reading at its faces' extent**
//! (`topo::boolean::edge_join`'s `joinable`): the pair's wedge at a
//! valence-2 vertex is read as every reader of an edge's wedge reads it
//! (`topo::wedge_reach`), so a short edge beside the vertex does not
//! shrink the lever.
//!
//! The D prism ∩ a brick from `x = c` (`c = √(1 − sin²θ)`) leaves a
//! vertical line edge where the plane `x = c` meets the cylinder at
//! `y = sin θ`, the pair's wedge `sin θ`. Cutting a station `s` from
//! that edge's end mints a piece `s` long, whose transversality margin,
//! levered at its own length, would be `sin θ · s`; levered at its
//! faces' extent it is definite, so the cut certifies on either side of
//! `sin θ · s = Kε`, and the join takes the station back and the result
//! passes tiers 3 and 3′.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_core::{Band, Point2, Point3, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::construction::Construct;
use topo::{AtRestBody, Body, BooleanResult, EdgeKey, intersect};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The D: the unit disc's half `x ≥ 0`, extruded `z ∈ (0, 1)`.
fn d_prism() -> AtRestBody<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -1.0), 1.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, 0.0));
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    let built = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap();
    finished("the D", built.body, tol())
}

/// The D ∩ the brick from `x = c`, `c = √(1 − sin²θ)`, and its vertical
/// line edge at `y = sin θ`, oriented so its parameter runs from `z = 0`.
fn cut_d(sin: f64) -> (Body<f64>, EdgeKey, f64) {
    let c = (1.0 - sin * sin).sqrt();
    let b = finished(
        "the brick",
        brick((c, 4.0), (-4.0, 4.0), (-3.0, 3.0), tol()),
        tol(),
    );
    let Ok(BooleanResult::Body(r)) = intersect(&d_prism(), &b, tol()) else {
        panic!("D ∩ brick at sin θ = {sin} builds")
    };
    let body = r.body.into_body();
    let (edge, u0) = body
        .edges()
        .find_map(|(e, d)| {
            let c_geom = body.get_curve_geom(d.curve)?.certified()?;
            let Curve3::Line { origin, dir } = *c_geom.carrier() else {
                return None;
            };
            let on = (origin.x - c).abs() < 1e-9 && (origin.y - sin).abs() < 1e-9;
            // The parameter at z = 0.
            (on && dir.z.abs() > 0.5).then(|| {
                let at0 = (Point3::new(c, sin, 0.0) - origin).dot(dir) / dir.dot(dir);
                (e, at0)
            })
        })
        .expect("the vertical edge where x = c meets the cylinder");
    (body, edge, u0)
}

/// Cuts a station `s` up the vertical edge from `z = 0`: the split's
/// refusal, or the body with the station.
fn station(sin: f64, s: f64) -> Result<Body<f64>, topo::EulerOpError> {
    let (mut body, edge, u0) = cut_d(sin);
    let curve = body.get_edge(edge).unwrap().curve;
    let c = body
        .get_curve_geom(curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone();
    let Curve3::Line { dir, .. } = *c.carrier() else {
        unreachable!()
    };
    let u = u0 + s * dir.z.signum() / dir.norm();
    body.split_edge(edge, u, tol()).map(|_| body)
}

/// The D ∩ the brick at `sin θ = 1` (the brick from `x = 0`), with a
/// station cut `s` along its cap arc at `z = 0` from the arc's end.
fn arc_station(s: f64) -> Result<Body<f64>, topo::EulerOpError> {
    let (mut body, _, _) = cut_d(1.0);
    let (arc, u) = body
        .edges()
        .find_map(|(e, d)| {
            let c = body.get_curve_geom(d.curve)?.certified()?;
            let Curve3::Circle { center, .. } = *c.carrier() else {
                return None;
            };
            (center.z.abs() < 1e-9).then(|| {
                let (t0, t1) = c.params();
                (e, if t1 > t0 { t0 + s } else { t0 - s })
            })
        })
        .expect("the cap arc at z = 0");
    body.split_edge(arc, u, tol()).map(|_| body)
}

/// `body` joins its one station back and passes tiers 3 and 3′.
fn joins_sound(what: &str, mut body: Body<f64>) {
    let joined = body
        .join_edges(band(), tol())
        .unwrap_or_else(|e| panic!("{what}: the join refused {e:?}"));
    assert_eq!(joined.len(), 1, "{what}: the join takes the station back");
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol()),
        Ok(()),
        "{what}: tier 3′"
    );
}

/// **A station cut either side of `sin θ · s = Kε` builds and joins
/// back sound**, at `sin θ` = 0.5, 0.1 and 0.02: the piece's wedge is
/// levered at its faces' extent, not at its own length, so neither cut
/// mints a piece the band holds. The cut keeps the D ∩ brick's volume,
/// the unit disc's segment beyond `x = c` (`θ − sin θ cos θ`) over the
/// unit height, and its material: a point inside the segment is in, one
/// on the D short of `x = c` out. The cut's `Transversality` refusal
/// this row pinned before has no row now
/// (`work/lever/the-arms-the-faces-lever-left-without-a-row.md`).
#[test]
fn a_station_on_the_line_edge_builds_either_side_of_k_eps_and_joins_back() {
    use topo::SolidContainment::{In, Out};
    let k = band().escalate();
    for sin in [0.5, 0.1, 0.02] {
        let theta = f64::asin(sin);
        let c = theta.cos();
        let exact = theta - sin * c;
        for frac in [0.95, 1.05] {
            let what = format!("sin θ = {sin}, sin θ · s = {frac} Kε");
            let cut = station(sin, frac * k / sin)
                .unwrap_or_else(|e| panic!("{what}: the cut builds, got {e:?}"));
            // Raw door: `cut` is construction state whose joinable vertices tier 3 refuses (JoinableVertexAtRest).
            let v = topo::test_support::measure_unvalidated(
                &cut,
                tol(),
                topo::test_support::pinned_resolution(tol()),
            )
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume;
            assert!(
                (v - exact).abs() <= 1e-9 * exact,
                "{what}: the volume is the segment's, {v:e} vs {exact:e}"
            );
            let held = |p: Point3<f64>| topo::point_in_solid(&cut, p, band(), tol()).unwrap();
            assert_eq!(
                [
                    held(Point3::new(0.5 * (c + 1.0), 0.0, 0.5)),
                    held(Point3::new(0.5 * c, 0.0, 0.5)),
                ],
                [In, Out],
                "{what}: the segment holds its material"
            );
            joins_sound(&what, topo::Body::clone(&cut));
        }
    }
}

/// **The cap arc at `sin θ = 1`**: a station under `10ε` along it
/// refuses at the split (its parameter is in the band of the arc's
/// end), and one at `10.5ε` joins back sound.
#[test]
fn a_station_on_the_cap_arc_refuses_under_ten_eps_and_joins_from_ten_and_a_half() {
    let eps = band().zero();
    let under = arc_station(9.5 * eps);
    assert!(
        matches!(under, Err(topo::EulerOpError::SplitParamEscalated { .. })),
        "s = 9.5ε: the split refuses, got {:?}",
        under.as_ref().map(|_| ())
    );
    let past =
        arc_station(10.5 * eps).unwrap_or_else(|e| panic!("s = 10.5ε: the cut builds, got {e:?}"));
    joins_sound("the cap arc, s = 10.5ε", past);
}
