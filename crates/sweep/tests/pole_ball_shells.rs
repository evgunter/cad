//! **A pole-touching ball shells through `topo::shell`.** The profile is
//! ONE semicircular arc whose two ends sit on the revolve axis, closed by
//! the axis chord: the ball a user authors when they revolve a half disc.
//! The thin solid is the difference of two concentric balls,
//! `4/3·π(r³ − (r − t)³)`, tier-3 valid, and tessellates watertight —
//! at a spread of radii, thicknesses and centre heights, and with the
//! meridian authored as a run of cocircular arcs (the revolve builds a
//! run as one wall, so the poles and the shelled body are the same).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

use super::common::bulge;

/// The full revolve of the half disc of radius `r` centred at height `c`
/// on the axis: the meridian `(0, c − r) → (0, c + r)` as cocircular
/// arcs meeting at the latitudes `seams` (ascending, in `(−π/2, π/2)`;
/// none is the one bulge-1 arc), closed by the axis chord.
fn pole_ball(r: f64, c: f64, seams: &[f64]) -> Body<f64> {
    let centre = Point2::new(0.0, c);
    let pts: Vec<Point2<f64>> = core::iter::once(-FRAC_PI_2)
        .chain(seams.iter().copied())
        .chain(core::iter::once(FRAC_PI_2))
        .map(|a| Point2::new(r * a.cos(), c + r * a.sin()))
        .collect();
    let mut verts: Vec<_> = pts
        .windows(2)
        .map(|w| (w[0], bulge(w[0], w[1], centre)))
        .collect();
    verts.push((*pts.last().expect("two poles"), 0.0));
    let lp = bulge_loop(verts);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the half disc validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the half disc revolves")
    .body
}

#[test]
fn a_pole_touching_ball_shells_to_the_difference_of_two_balls() {
    let tol = Tol::witness();
    let q = PI / 4.0;
    let near = FRAC_PI_2 - 0.05;
    for (r, t, c, seams) in [
        (1.0, 0.1, 0.0, &[][..]),
        (1.0, 0.05, 0.0, &[]),
        (1.0, 0.5, 0.0, &[]),
        (1.0, 0.9, 0.0, &[]),
        (3.0 / 64.0, 1.0 / 128.0, 3.0 / 64.0, &[]),
        (10.0, 1.0, -4.0, &[]),
        (2.5, 0.01, 7.0, &[]),
        (1.0, 0.05, 0.0, &[q]),
        (1.0, 0.05, 0.0, &[-q]),
        (1.0, 0.05, 0.0, &[0.0]),
        (1.0, 0.05, 0.0, &[-q, 0.0, q]),
        (1.0, 0.02, 0.0, &[near]),
        (1.0, 0.02, 0.0, &[-near, near]),
    ] {
        let label = format!("r {r} t {t} centre {c} seams {seams:?}");
        let ball = pole_ball(r, c, seams);
        assert_eq!(ball.faces().count(), 2, "{label}: one wall in two π-bands");
        assert_eq!(
            topo::validate_geometric(&ball, tol),
            Ok(()),
            "{label}: the operand is tier-3 valid"
        );
        let out = topo::shell(&ball, t, tol)
            .unwrap_or_else(|e| panic!("{label}: the ball shells, got {e:?}"))
            .body;
        assert_eq!(
            topo::validate_geometric(&out, tol),
            Ok(()),
            "{label}: the thin solid is tier-3 valid"
        );
        let props = topo::mass_properties(&out, tol).expect("props");
        let want = 4.0 / 3.0 * PI * (r.powi(3) - (r - t).powi(3));
        assert!(
            (props.volume - want).abs() <= 1e-12 * r.powi(3) + props.volume_pad,
            "{label}: volume {} vs the closed form {want} (pad {})",
            props.volume,
            props.volume_pad
        );
        let mesh = mesh::tessellate(&out, 1e-2 * r, tol)
            .unwrap_or_else(|e| panic!("{label}: tessellates, got {e:?}"));
        mesh::validate::check_mesh(&mesh)
            .unwrap_or_else(|e| panic!("{label}: watertight, got {e:?}"));
    }
}
