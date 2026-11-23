//! **A refused member's window encloses where it can meet the span**:
//! a counterexample search over line spans against one drawn edge
//! standing within a few escalations of them (module row below).

#![allow(clippy::unwrap_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/curve_face.rs",
    "crates/geom-brep/src/curve_pair/span.rs",
    "crates/geom/src/cover.rs",
    "crates/geom/src/cover/",
];

use core::f64::consts::{FRAC_PI_2, PI};

use geom::Curve3;
use geom_core::{Band, Point3, Vec3};

use crate::curve_pair::Span;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn line(origin: [f64; 3], dir: [f64; 3]) -> Curve3<f64> {
    let [x, y, z] = dir;
    Curve3::line(
        Point3::new(origin[0], origin[1], origin[2]),
        Vec3::new(x, y, z).normalize(),
    )
}

/// **A refused member's near points lie in a window**: a line span
/// against one drawn edge (a short segment or an arc of radius 1e-2 to
/// 1e2) standing within a few escalations of it, wherever the located
/// pass leaves the edge undecided and reads nothing else. Every sampled
/// point of the edge within the band's zero of the line lies, by its
/// foot, inside an undecided window. This checks that the window
/// encloses the member's cover along the line: red where the cover's
/// range or its mapping to the line's parameter is wrong. It does not
/// go red if the window's widening by the escalation either way is
/// dropped, since the points it samples are within the zero band, which
/// the cover's own range already holds; nor does it sample a meeting
/// between its 4001 stations.
#[test]
fn a_refused_members_near_points_lie_in_a_window() {
    let mut rng = test_utils::fuzz::start("refused-member-window");
    let esc = band().escalate();
    for case in 0..test_utils::fuzz::scaled(3000) {
        let ang = rng.unit() * PI;
        let (c, sn) = (ang.cos(), ang.sin());
        let o = [rng.range(-2.0, 2.0), rng.range(-2.0, 2.0), 0.0];
        let carrier = line(o, [c, sn, 0.0]);
        let span = Span::new(&carrier, (-3.0, 3.0));
        let at =
            |t: f64, off: f64| Point3::new(o[0] + t * c - off * sn, o[1] + t * sn + off * c, 0.0);
        let off = rng.range(-3.0, 3.0) * esc;
        let (cv, sp) = if case % 2 == 0 {
            let a = at(rng.range(-2.0, 0.0), off);
            let b = at(rng.range(0.0, 2.0), off + (rng.unit() - 0.5) * 4.0 * esc);
            (Curve3::line(a, b - a), (0.0, 1.0))
        } else {
            let r = 10f64.powf(rng.range(-2.0, 2.0));
            let top = ang + FRAC_PI_2;
            let w = rng.range(0.01, 1.51);
            let reach = w * rng.unit();
            (
                Curve3::circle(
                    at(rng.range(-1.0, 1.0), -r - off),
                    Vec3::unit_z(),
                    r,
                    Vec3::unit_x(),
                ),
                (top - w, top + reach),
            )
        };
        let located = super::located(&span, &[(cv.eval(sp.0), Span::new(&cv, sp))], &[], band());
        if located.undecided.is_empty()
            || !located.reading.meetings.is_empty()
            || !located.reading.shared.is_empty()
        {
            continue;
        }
        for i in 0..=4000 {
            let d = cv.eval(sp.0 + (sp.1 - sp.0) * f64::from(i) / 4000.0)
                - Point3::new(o[0], o[1], 0.0);
            let (t, dist) = (d.x * c + d.y * sn, (d.y * c - d.x * sn).abs());
            if dist <= ZERO && t.abs() <= 3.0 {
                assert!(
                    located
                        .undecided
                        .iter()
                        .any(|u| u.window.0 - 1e-12 <= t && t <= u.window.1 + 1e-12),
                    "case {case}: t {t} — {}",
                    test_utils::fuzz::replay()
                );
            }
        }
    }
}
