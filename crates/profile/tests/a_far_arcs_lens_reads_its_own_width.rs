//! **A lens of a far-centred arc is a loop by its own width.** Four
//! two-vertex profiles, each an arc of radius 1e6 and its chord, 10.3 to
//! 11.9 pm wide: `loop_orientation`'s margin `2A/P` is `r·θ²/12` to a
//! part in `θ²` (`θ − sin θ = θ³/6·(1 − θ²/20 + …)`, the chord
//! `r·θ·(1 − θ²/24 + …)`), past `10ε` at ε = 1e-12, so each is a loop
//! and validates. The segment's `θ − sin θ` taken as a difference keeps
//! `θ`'s rounding, a third of the term at these sweeps, and reads each
//! of the four a sliver.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};

/// `(start, end, bulge)` of each lens's arc; its chord closes it.
const LENSES: [((f64, f64), (f64, f64), f64); 4] = [
    (
        (8.191611212304761e-1, 4.7278271436708463e-1),
        (8.113812364013288e-1, 4.6451302760181334e-1),
        2.838511132073699e-9,
    ),
    (
        (-1.1397726560416199e-1, 1.68582610158772e0),
        (-1.2536398480149535e-1, 1.6832149900894735e0),
        2.920565841313923e-9,
    ),
    (
        (-1.9493880793654998e0, -1.0115182783283845e0),
        (-1.9401034714610665e0, -1.0176147058121157e0),
        2.7768054037357733e-9,
    ),
    (
        (1.470202016609492e-1, 1.360385627891691e0),
        (1.5594410993226657e-1, 1.352427237063906e0),
        -2.9892737768003196e-9,
    ),
];

#[test]
fn a_far_arcs_lens_past_ten_eps_wide_validates() {
    let tol = Tol::witness();
    if (tol.eps() - 1.0e-12).abs() > 1.0e-15 {
        test_utils::vacuity::stood_down(
            "ε other than 1e-12",
            "the lenses are sized to 1e-12's band: under any wider one they are slivers",
        );
        return;
    }
    for (i, &((x0, y0), (x1, y1), bulge)) in LENSES.iter().enumerate() {
        let (p0, p1) = (Point2::new(x0, y0), Point2::new(x1, y1));
        let chord = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
        let theta = 4.0 * bulge.abs().atan();
        let width = chord / (2.0 * (theta / 2.0).sin()) * theta * theta / 12.0;
        assert!(
            width > 10.2 * tol.eps() && width < 12.0 * tol.eps(),
            "lens {i} is {width:e} wide, not what this row is sized to"
        );
        let got = Profile::new(
            SketchPlane::xy(),
            vec![bulge_loop(vec![(p0, bulge), (p1, 0.0)])],
        )
        .validate(tol);
        assert!(
            got.is_ok(),
            "lens {i}, {width:e} wide: {:?}",
            got.map(|_| "valid")
        );
    }
}
