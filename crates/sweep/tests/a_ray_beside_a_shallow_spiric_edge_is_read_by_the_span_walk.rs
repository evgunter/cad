//! **A ray beside a shallow spiric edge is read by the walk over the
//! edge's span: no ray is set aside on the carrier's straddle, with the
//! reader's side and point containment as they were** (`topo`'s curved
//! edge reading, `boolean::solid_contain::curved_edges`, through
//! `geom_brep::curve_pair::Span::read`).
//!
//! A torus about `y` (major 2, minor 0.5) cut by the plane
//! `x = 1.5 − 10⁻³`, a millimetre inside its inner equator, through the
//! boolean door. The cut's edges are spiric ovals, and near each oval's
//! tip the plane meets the torus at a dihedral whose sine is under a
//! fifth. A spiric's lower bound on a point's distance from it is
//! the greater of the point's distances from the plane and from the
//! torus (`geom::Curve3::clearance`), so a ray crossing either face a
//! few bands from the edge there stands within the zero band of both
//! carriers and past the band from the curve's foot: the carrier's two
//! bounds straddle the band (`curve_carrier_straddle`), at `f64`, on a
//! body a door builds. The span reads such a point against the span
//! itself, and the piece walk over the span clears it (`Span::clear`):
//! off the span, certified, so the ray reads `Out`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{brick, revolved_about_y};
use topo::Body;

/// The torus with its inner flat.
fn flat_torus(tol: Tol) -> Body<f64> {
    let tube = vec![(Point2::new(2.5, 0.0), 1.0), (Point2::new(1.5, 0.0), 1.0)];
    let torus = revolved_about_y(tube, Revolution::Full, tol);
    let cutter = brick::<f64>((-3.0, 1.5 - 1e-3), (-3.0, 3.0), (-3.0, 3.0), tol);
    let rest = |x: &Body<f64>| topo::finish(x.clone(), tol).expect("an operand finishes");
    topo::intersect(&rest(&torus), &rest(&cutter), tol)
        .expect("the cut builds")
        .body()
        .expect("the cut leaves a body")
        .body
        .clone()
        .into_body()
}

/// **Rays a few bands from a shallow spiric edge are read, none set
/// aside on a crossing, and the reader and point containment answer as
/// before.** At each sampled point of a spiric edge where the faces'
/// dihedral sine is under a fifth, rays are aimed 12 and 15 bands either
/// way from the edge along each face, at the aimed incidence about the
/// face's normal and about it tilted toward and away from the edge. Each
/// reads `Out` or is set aside as a graze (`SetAside(Graze)`: in band of
/// an end of the edge, or undecided against one, which refuses the ray
/// rather than answering it). None is set aside on a crossing: where
/// the carrier's bounds straddle the band, the walk over the edge's span
/// certifies the ray's point clear of the span. The side reader's own
/// aims stand clear of every edge and still read `Out`; and points about
/// those edge points, from a tenth of a millimetre to a tenth away and
/// clear of both carriers, are each contained or not as the torus's and
/// the plane's own equations say.
///
/// **Red where the span's clearance is not read** (`Span::read` with its
/// clearance step off): four rays are set aside on the carrier's
/// straddle (`SetAside(Crossing(Escalated(..)))`, `curve_carrier_straddle`)
/// at ε 1e-9. Before the span read its own clearance, this row pinned
/// that loss, by a count of rays set aside on a straddle above zero.
#[test]
fn a_ray_beside_a_shallow_spiric_edge_is_read_by_the_span_walk() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let eps = tol.eps();
    let body = flat_torus(tol);
    let frames = topo::test_support::edge_frames(
        &body,
        &[0.002, 0.03, 0.1, 0.3, 0.5, 0.7, 0.9, 0.97, 0.998],
        band,
    );
    let spiric: Vec<_> = frames
        .iter()
        .filter(|f| f.label.contains("Spiric"))
        .collect();
    let sine = |f: &topo::test_support::EdgeFrame| {
        spiric
            .iter()
            .filter(|g| g.face != f.face && (g.point - f.point).norm() < 1e-9)
            .map(|g| g.normal.cross(f.normal).norm())
            .fold(f64::INFINITY, f64::min)
    };
    let shallow: Vec<_> = spiric.iter().filter(|f| sine(f) < 0.2).collect();
    assert!(
        !shallow.is_empty(),
        "at ε {eps:e}: no sampled point of a spiric edge is shallow"
    );
    let mut read = 0;
    for f in &shallow {
        for bands in [-15.0, -12.0, 12.0, 15.0] {
            let aim = f.point + f.into * (bands * eps);
            for normal in [
                f.normal,
                (f.normal + f.into * 0.7).normalize(),
                (f.normal - f.into * 0.7).normalize(),
            ] {
                let got = topo::test_support::ray_at(&body, f.face, aim, normal, band);
                assert!(
                    got == "Ok(Side(Out))" || got == "Ok(SetAside(Graze))",
                    "at ε {eps:e}, {} {bands} bands from the edge: {got}",
                    f.label
                );
                if got == "Ok(Side(Out))" {
                    read += 1;
                }
            }
        }
    }
    assert!(read > 0, "at ε {eps:e}: no ray read `Out`");
    assert_eq!(
        topo::test_support::aimed_rays(&body, band),
        ["Ok(Side(Out))"],
        "the reader's own aims"
    );
    // The torus about `y` and the plane: a point's signed clearance of
    // each carrier, negative on the body's side.
    let clearances = |p: geom_core::Point3<f64>| {
        let rho = (p.x * p.x + p.z * p.z).sqrt();
        (
            ((rho - 2.0).powi(2) + p.y * p.y).sqrt() - 0.5,
            p.x - (1.5 - 1e-3),
        )
    };
    let mut asked = 0;
    for (k, f) in shallow.iter().enumerate() {
        for (j, reach) in [1e-4, 1e-2, 0.1].into_iter().enumerate() {
            let turn = 0.7 + 1.3 * (k + 4 * j) as f64;
            let q = f.point + (f.normal * turn.cos() + f.into * turn.sin()) * reach;
            let (torus, plane) = clearances(q);
            // Within a thousand bands of a carrier the query may refuse
            // in band, which is its answer there.
            if torus.abs().min(plane.abs()) < 1e3 * eps {
                continue;
            }
            asked += 1;
            let want = if torus < 0.0 && plane < 0.0 {
                "Ok(In)"
            } else {
                "Ok(Out)"
            };
            assert_eq!(
                format!("{:?}", topo::point_in_solid(&body, q, band, tol)),
                want,
                "at ε {eps:e}, {reach:e} from {}",
                f.label
            );
        }
    }
    assert!(
        asked > 0,
        "at ε {eps:e}: no point stands clear of both carriers"
    );
    println!("ε {eps:e}: {read} rays read, {asked} points");
}
