//! **The stored wall frame at `Interval`, on M10-5's 12-gon prism** —
//! the acceptance row for the axis-order orthonormal basis.
//!
//! `Vec3::orthonormal_basis` crosses the normal with the world axis of
//! its smallest-magnitude component and normalizes, so a vertical wall
//! (`n.z = 0`, the whole equator) is the case the order DECIDES rather
//! than the case it is degenerate on: `|n.z|` is the strict minimum, no
//! sign is transferred anywhere, and the stored `u_ref` comes back as a
//! point enclosure of the exact in-plane horizontal `(−n.y, n.x, 0)`.
//!
//! This replays the dumbbell prism at `Interval` over the ε-scaled
//! parameter box, mints each wall's plane through `newell_plane`, and
//! asserts, per wall:
//!
//! * every `u_ref` component is a point enclosure to within 1e-15 —
//!   including the two walls whose `n.z` encloses `[−2.2e-16, 2.2e-16]`
//!   rather than the exact zero, which decide here because
//!   `|n.z| ≤ 2.2e-16` is far below the other two magnitudes;
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
use geom_core::{Band, Bounds, Interval, Point3, Real, Tol, Vec3};

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

/// The frame is a point enclosure to within this — the walls are
/// axis-aligned or exactly diagonal, so the true widths are 0 and this
/// is headroom, not a tolerance anything is tuned to.
const EXACT: f64 = 1e-15;

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

/// The walls whose axis choice is UNDECIDED over the enclosure, and
/// why: `newell_plane`'s cross-sum cannot cancel exactly on the two
/// rings whose `y` is `0.8` and `1.2`, so their normals come back with
/// `n.x ∈ ±4.4e-16` and `n.z ∈ ±2.2e-16` rather than exact zeros. Which
/// of those two is the smaller magnitude is then a real question with
/// no answer over the box — both enclosures contain zero — so the door
/// hulls the `z`-axis and `x`-axis candidates, which differ by a
/// quarter turn about the normal. The hull is the honest answer and it
/// contains the `f64` frame; it is not exact, and pretending otherwise
/// is what this list refuses to do.
const UNDECIDED: [usize; 2] = [2, 8];

/// The prism's walls, at both windows.
///
/// **Why the `z`-extent is the window's own span** on a wall whose
/// frame is decided: the chart is `origin + u·u_ref + v·(n × u_ref)`;
/// the axis order gives a vertical wall `k = z` and therefore
/// `u_ref = (−n.y, n.x, 0)` — horizontal in the plane — so `n × u_ref`
/// is `e_z` and the cell's `z` extent is exactly the `v` span. The old
/// sign-transferred frame carried `u_ref.z ∈ [−|n.x|, |n.x|]` on every
/// wall of the equator and added `|u|·|n.x|` to it.
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
            let decided = !UNDECIDED.contains(&i);
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
                if decided {
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
            }
            // The undecided pair is undecided for the stated reason and
            // for no other: both candidate axes' magnitudes enclose
            // zero. A wall that stopped meeting that description while
            // staying on the list — or one that started meeting it
            // while off it — is a census that has moved.
            let straddles = |e: Interval| e.lo() <= 0.0 && 0.0 <= e.hi();
            assert_eq!(
                !decided,
                straddles(normal.x) && straddles(normal.z) && width(normal.x) > 0.0,
                "wall {i}: the undecided census is stale — n.x = [{}, {}], n.z = [{}, {}]",
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
            if decided {
                assert!(
                    (z - span).abs() <= span * 1e-13 + 16.0 * f64::EPSILON,
                    "wall {i}: the cell's z-enclosure is {z:e} over a window of span \
                     {span:e} — a frame with a z-component widened it"
                );
            }
            assert!(
                refines(&surface, window, window),
                "wall {i}: the stored chart does not refine over [{:e}, {:e}]",
                window.0,
                window.1
            );
        }
    }
}
