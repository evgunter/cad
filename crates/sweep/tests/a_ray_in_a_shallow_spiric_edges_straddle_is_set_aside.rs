//! **A ray in the straddle of a shallow spiric edge is set aside: a
//! read lost at `f64`, with the reader's side and point containment as
//! they were** (`topo`'s curved edge reading,
//! `solid_contain::curved_edges`).
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
//! body a door builds.

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

/// **Rays a few bands from a shallow spiric edge read no side there,
/// and the reader and point containment answer as before.** At each
/// sampled point of a spiric edge where the faces' dihedral sine is
/// under a fifth, rays are aimed 12 and 15 bands either way from the
/// edge along each face, at the aimed incidence about the face's normal
/// and about it tilted toward and away from the edge. Each reads `Out`
/// or is set aside, and those set aside on the carrier's straddle are counted: the straddle
/// brackets the ray's distance from the edge across the band's edge, so
/// it says neither that the ray meets the edge nor that it passes it.
/// The side reader's own aims stand clear of every edge and still read
/// `Out`; and points about those edge points, from a tenth of a
/// millimetre to a tenth away and clear of both carriers, are each
/// contained or not as the torus's and the plane's own equations say.
///
/// **Red under a carrier's escalation read as the edge met**
/// (`curved_edges::passes` handing the door a forged
/// `Ok(MarginDiag::value(0.0))` for a carrier that refused, or
/// `edge_gap::Reach::escalated` sorting every escalation within the
/// band): no ray is set aside on a straddle, each reading `Out` through
/// a rim no reading placed. At `f64` that reading and the faces' own
/// crossings agree on every such ray; what the straddle lacks is a
/// reading, not a right answer.
///
/// **This pins a loss, and is the next reader's to move.** The count of
/// rays set aside on a straddle is asserted above zero, so the row goes
/// red the day those reads come back: by a spiric lower bound that is
/// the distance from the curve, or by a straddle left to each face's
/// own trim
/// (`work/flushtail/a-uniform-ray-about-a-curved-edge-is-still-set-aside-on-a-faces-own-boundary.md`).
/// Whoever lands either turns that assertion round.
#[test]
fn a_ray_in_a_shallow_spiric_edges_straddle_is_set_aside() {
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
    let (mut straddled, mut read) = (0, 0);
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
                    got == "Ok(Side(Out))" || got.starts_with("Ok(SetAside("),
                    "at ε {eps:e}, {} {bands} bands from the edge: {got}",
                    f.label
                );
                if got.contains("curve_carrier_straddle") {
                    straddled += 1;
                }
                if got == "Ok(Side(Out))" {
                    read += 1;
                }
            }
        }
    }
    assert!(
        straddled > 0,
        "at ε {eps:e}: no ray was set aside on a straddle ({read} read `Out`)"
    );
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
    println!("ε {eps:e}: {straddled} rays set aside on a straddle, {read} read, {asked} points");
}
