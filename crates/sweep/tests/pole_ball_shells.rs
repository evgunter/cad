//! **A pole-touching ball shells through `topo::shell`.** The ball is the
//! half-disc lamina revolved a full turn (`test_support::ball_poled_y`):
//! one semicircular arc whose ends sit on the revolve axis, closed by the
//! axis chord — the ball a user authors when they revolve a half disc.
//! The thin solid is the difference of two concentric balls,
//! `4/3·π(r³ − (r − t)³)`, tier-3 valid, and tessellates watertight — at
//! a spread of radii, thicknesses and centre heights.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Tol, Vec3};
use sweep::test_support::ball_poled_y;
use sweep::test_support::finished;

#[test]
fn a_pole_touching_ball_shells_to_the_difference_of_two_balls() {
    let tol = Tol::witness();
    for (r, t, c) in [
        (1.0, 0.1, 0.0),
        (1.0, 0.05, 0.0),
        (1.0, 0.5, 0.0),
        (1.0, 0.9, 0.0),
        (3.0 / 64.0, 1.0 / 128.0, 3.0 / 64.0),
        (10.0, 1.0, -4.0),
        (2.5, 0.01, 7.0),
    ] {
        let label = format!("r {r} t {t} centre {c}");
        let ball = ball_poled_y(r, Vec3::new(0.0, c, 0.0), tol);
        assert_eq!(ball.faces().count(), 2, "{label}: one wall in two π-bands");
        let out = topo::shell(&finished("the operand", ball.clone(), tol), t, tol)
            .unwrap_or_else(|e| panic!("{label}: the ball shells, got {e:?}"))
            .body;
        assert_eq!(
            topo::validate_geometric(&out, tol),
            Ok(()),
            "{label}: the thin solid is tier-3 valid"
        );
        let props = topo::mass_properties(&out, tol).expect("props");
        let want = 4.0 / 3.0 * PI * (r.powi(3) - (r - t).powi(3));
        // The volume is a sum of r³-sized terms, so its rounding scales
        // with r³; the pad is the quadrature's own bound.
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
