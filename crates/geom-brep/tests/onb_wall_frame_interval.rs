//! **The stored wall frame at `Interval`, on M10-5's 12-gon prism** —
//! the acceptance row for the orthonormal basis's world-axis choice.
//!
//! `Vec3::orthonormal_basis` crosses the normal with `e_z` when
//! `n.z² ≤ n.x² + n.y²` and with `e_y` otherwise, and normalizes. A vertical
//! wall (`n.z = 0`, the whole equator) is therefore as far from the
//! comparison's seam as a direction can be: the choice DECIDES over any
//! enclosure a wall's normal comes in, no sign is transferred anywhere,
//! and the stored `u_ref` is the exact in-plane horizontal
//! `(−n.y, n.x, 0)`.
//!
//! This replays the dumbbell prism at `Interval` over the ε-scaled
//! parameter box, mints each wall's plane through `newell_plane`, and
//! asserts, per wall:
//!
//! * every `u_ref` component is as tight as the wall's own normal —
//!   including the two walls whose `n.x` and `n.z` come back as
//!   `±4.4e-16` and `±2.2e-16` rather than exact zeros, which an order
//!   over the three components could not have decided between;
//! * the enclosure contains the `f64` frame (the containment the
//!   `Interval` lane exists to provide);
//! * the cell `Surface::eval` encloses over the window has the window's
//!   own `z`-extent — no hulled frame doubling it;
//! * the chart refines: halving the window moves a bound on both axes.
//!
//! The `refines` predicate is `editor_core::clearance`'s, copied rather
//! than called (it is private there): both axes must move a bound.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::newell_plane;
use geom_core::{Band, Bounds, Interval, Point3, Real, Tol};

/// THE CORPUS, written down: M10-5's dumbbell — a 12-gon whose prism
/// walls run every wall orientation the rule can meet (`n.x = 0`,
/// `n.y = 0`, and eight axis-aligned faces between them).
const DUMBBELL: [(f64, f64); 12] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 0.8),
    (3.0, 0.8),
    (3.0, 0.0),
    (5.0, 0.0),
    (5.0, 2.0),
    (3.0, 2.0),
    (3.0, 1.2),
    (2.0, 1.2),
    (2.0, 2.0),
    (0.0, 2.0),
];

/// The extrusion height, and the two parameter windows the refinement
/// test halves — a metre-scale cell and the ε-scaled one M10-5's
/// measurement was taken over.
const HEIGHT: f64 = 2.0;
const WINDOW: (f64, f64) = (0.0, 1.0);
const EPS_WINDOW: (f64, f64) = (0.0, 1e-9);

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn width(e: Interval) -> f64 {
    e.hi() - e.lo()
}

/// Wall `i`'s ring: the prism's side face, wound outward.
fn wall<T: Real>(i: usize) -> [Point3<T>; 4] {
    let (a, b) = (DUMBBELL[i], DUMBBELL[(i + 1) % DUMBBELL.len()]);
    let p = |x: f64, y: f64, z: f64| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    [
        p(a.0, a.1, 0.0),
        p(b.0, b.1, 0.0),
        p(b.0, b.1, HEIGHT),
        p(a.0, a.1, HEIGHT),
    ]
}

/// `editor_core::clearance::refines`, copied: BOTH axes must move a
/// bound of the cell's enclosure when the window is halved.
fn refines(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> bool {
    let cell = |u: (f64, f64), v: (f64, f64)| {
        surface.eval(
            Interval::from_bounds(u.0, u.1),
            Interval::from_bounds(v.0, v.1),
        )
    };
    let whole = cell(u, v);
    let mid = |(lo, hi): (f64, f64)| 0.5 * (lo + hi);
    let narrower = |b: &Point3<Interval>| {
        [(b.x, whole.x), (b.y, whole.y), (b.z, whole.z)]
            .iter()
            .any(|(h, w)| h.hi() < w.hi() || h.lo() > w.lo())
    };
    narrower(&cell((u.0, mid(u)), v)) && narrower(&cell(u, (v.0, mid(v))))
}

/// The `z`-extent of the cell `Surface::eval` encloses over a window.
fn cell_z_width(surface: &Surface<Interval>, u: (f64, f64), v: (f64, f64)) -> f64 {
    let p = surface.eval(
        Interval::from_bounds(u.0, u.1),
        Interval::from_bounds(v.0, v.1),
    );
    width(p.z)
}

/// The prism's walls, at both windows.
///
/// **Why the `z`-extent is the window's own span**: the chart is
/// `origin + u·u_ref + v·(n × u_ref)`; a wall takes the `e_z` arm and
/// therefore `u_ref = (−n.y, n.x, 0)` — horizontal in the plane — so
/// `n × u_ref` is `e_z` and the cell's `z` extent is exactly the `v`
/// span. The old sign-transferred frame carried
/// `u_ref.z ∈ [−|n.x|, |n.x|]` on every wall of the equator and added
/// `|u|·|n.x|` to it.
///
/// **All twelve walls decide**, including the two whose `n.x` and `n.z`
/// are noise around zero: the comparison is `n.z² ≤ n.x² + n.y²`, and a
/// wall's `n.y²` or `n.x²` is 1 while both sides of the noise are
/// `1e-32`. An order over the three components would have been
/// undecided on exactly those two, because which of two enclosures that
/// both contain zero is the smaller has no answer.
#[test]
fn every_wall_of_the_twelve_gon_prism_stores_a_frame_as_exact_as_its_normal() {
    for window in [WINDOW, EPS_WINDOW] {
        let span = window.1 - window.0;
        for i in 0..DUMBBELL.len() {
            let Ok(Surface::Plane {
                origin,
                normal,
                u_ref,
            }) = newell_plane::<Interval>(&wall::<Interval>(i), band())
            else {
                panic!("wall {i}: newell_plane refused at Interval");
            };
            let Ok(Surface::Plane { u_ref: uf, .. }) = newell_plane::<f64>(&wall::<f64>(i), band())
            else {
                panic!("wall {i}: newell_plane refused at f64");
            };
            // The frame can be no tighter than the normal it is built
            // from; the factor is headroom over the measured widths,
            // not a tolerance anything is tuned to.
            let n_width = width(normal.x).max(width(normal.y)).max(width(normal.z));
            let bound = 4.0 * n_width + 4.0 * f64::EPSILON;
            for (e, f, which) in [
                (u_ref.x, uf.x, "u_ref.x"),
                (u_ref.y, uf.y, "u_ref.y"),
                (u_ref.z, uf.z, "u_ref.z"),
            ] {
                assert!(
                    e.lo() <= f && f <= e.hi(),
                    "wall {i} {which}: the f64 frame's {f} is outside [{}, {}]",
                    e.lo(),
                    e.hi()
                );
                assert!(
                    e.is_certified(),
                    "wall {i} {which} cannot decide: [{}, {}]",
                    e.lo(),
                    e.hi()
                );
                assert!(
                    width(e) <= bound,
                    "wall {i} {which} is wider than its own normal: [{}, {}] \
                     (width {:e} against {bound:e}), n.z = [{}, {}]",
                    e.lo(),
                    e.hi(),
                    width(e),
                    normal.z.lo(),
                    normal.z.hi()
                );
            }
            // The comparison decides on every wall, and the census
            // records that two of them have normals an order over the
            // three components could not have decided between.
            let straddles = |e: Interval| e.lo() <= 0.0 && 0.0 <= e.hi();
            let noisy = straddles(normal.x) && straddles(normal.z) && width(normal.x) > 0.0;
            let d = normal.z.powi(2) - (normal.x.powi(2) + normal.y.powi(2));
            let decided = d.hi() <= 0.0 || d.lo() > 0.0;
            assert!(
                decided,
                "wall {i}: the axis comparison did not decide — n.z = [{}, {}]",
                normal.z.lo(),
                normal.z.hi()
            );
            assert_eq!(
                noisy,
                i == 2 || i == 8,
                "wall {i}: the noisy-normal census is stale — n.x = [{}, {}], n.z = [{}, {}]",
                normal.x.lo(),
                normal.x.hi(),
                normal.z.lo(),
                normal.z.hi()
            );
            let surface = Surface::Plane {
                origin,
                normal,
                u_ref,
            };
            let z = cell_z_width(&surface, window, window);
            assert!(
                (z - span).abs() <= span * 1e-13 + 16.0 * f64::EPSILON,
                "wall {i}: the cell's z-enclosure is {z:e} over a window of span \
                 {span:e} — a frame with a z-component widened it"
            );
            assert!(
                refines(&surface, window, window),
                "wall {i}: the stored chart does not refine over [{:e}, {:e}]",
                window.0,
                window.1
            );
        }
    }
}
