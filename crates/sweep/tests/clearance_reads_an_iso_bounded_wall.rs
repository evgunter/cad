//! **The clearance engine reads an iso-bounded NURBS face.** A loft or
//! sweep wall is bounded by its chart's own isos, so it is the whole
//! patch and its window is the knot domain (`topo::clearance`'s
//! `iso_bounded_domain`). These rows run the engine on such walls, each
//! against a separation the fixture fixes:
//!
//! - two facing loft walls `GAP` apart, at `Interval`, hold below the
//!   gap and are violated above it, with a witness on the two walls,
//!   and their minimum separation brackets `GAP`;
//! - a swept wall, at `f64`, against a slab under its rising flank,
//!   where the root cells overlap and only subdivision answers;
//! - two pillows' curved rational walls, at `f64`, violated 1 % above
//!   their separation, which only a sound enclosure of every cell
//!   reaches: an enclosure that missed part of its cell would discharge
//!   the nearest pair and read `Holds`;
//! - a rational (arc-railed) loft's every face has a window, at `f64`;
//! - the enclosure of a wall cell narrows as the cell halves.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_4;

use geom::NurbsCurve3;
use geom::Surface;
use geom_core::interval::Interval;
use geom_core::linalg::frame::path_start_frame;
use geom_core::{Band, Bounds, CertifiedBounds, Decide, Point2, Point3, Real, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::{Lofted, loft_body, sweep_body};
use topo::clearance::{
    BoundVerdict, Budgets, Candidate, ClearanceBound, DEFAULT_MAX_CELL_DEPTH, Side, Sweep, Which,
    min_separation,
};
use topo::{AtRestPolicy, Body, FaceKey};

type I = Interval;

/// The scalars a row builds and asks at.
trait Lane: Decide + CertifiedBounds + AtRestPolicy {}
impl<T: Decide + CertifiedBounds + AtRestPolicy> Lane for T {}

/// The gap every row fixes, metres.
const GAP: f64 = 0.1;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn at<T: Real>(x: f64) -> T {
    T::from_f64(x)
}

fn mid<T: CertifiedBounds>(x: T) -> f64 {
    let x = I::from_certified(x);
    0.5 * (x.lo() + x.hi())
}

fn width(x: I) -> f64 {
    x.hi() - x.lo()
}

/// The closed quad `[x0, x1] × [y0, y1]` in its sketch plane.
fn rect<T: Real>(x0: f64, x1: f64, y0: f64, y1: f64) -> Vec<ProfileLoop<T>> {
    let corner = |x: f64, y: f64| (Point2::new(at(x), at(y)), T::zero());
    vec![bulge_loop(vec![
        corner(x0, y0),
        corner(x1, y0),
        corner(x1, y1),
        corner(x0, y1),
    ])]
}

/// **The loft column about `x = cx`**: three sections at `z = 0, 1, 2`
/// at v-degree 2, `x ∈ [cx − 1, cx + 1]` in each and the middle one
/// wider in `y`. Its `x = cx ± 1` walls are curved patches lying in
/// those planes, so two columns `2 + GAP` apart face across `GAP`.
fn column(cx: f64) -> Lofted<I> {
    let places = [0.0, 1.0, 2.0].map(|z| SketchPlane::xy_at(Point3::new(at(cx), at(0.0), at(z))));
    let sections = [
        rect(-1.0, 1.0, -1.0, 1.0),
        rect(-1.0, 1.0, -1.5, 1.5),
        rect(-1.0, 1.0, -1.0, 1.0),
    ];
    loft_body::<I>(&sections, &places, 2, Tol::witness()).expect("the column lofts at Interval")
}

/// A face's chart point at the middle of its knot domain, as `f64`.
fn centre<T: Lane>(body: &Body<T>, face: FaceKey) -> Point3<f64> {
    let surface = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap();
    let Surface::Nurbs(net) = surface else {
        panic!("a wall is a NURBS face, got {surface:?}");
    };
    let halfway = |(lo, hi): (T, T)| (lo + hi) * at(0.5);
    let p = surface.eval(
        halfway(net.knot_values_u().domain()),
        halfway(net.knot_values_v().domain()),
    );
    Point3::new(mid(p.x), mid(p.y), mid(p.z))
}

/// The wall of `lofted` whose centre maximizes `score`.
fn wall_by<T: Lane>(lofted: &Lofted<T>, score: impl Fn(Point3<f64>) -> f64) -> FaceKey {
    lofted.side_faces[0]
        .iter()
        .flatten()
        .copied()
        .max_by(|&a, &b| score(centre(&lofted.body, a)).total_cmp(&score(centre(&lofted.body, b))))
        .expect("a loft has walls")
}

/// **The witness check**: a pair of the two cells' on-face stations
/// whose interval distance is wholly under `c`, evaluated afresh on the
/// stored surfaces. `Ok` carries the distance's upper end.
fn verify<'a, T: Lane>(
    (a, b): (&'a Body<T>, &'a Body<T>),
    c: f64,
) -> impl FnMut(Candidate<'_, '_, T>) -> Result<f64, String> + 'a {
    move |candidate| {
        let (fa, fb) = candidate.faces();
        let surface = |body: &Body<T>, f: FaceKey| {
            body.get_surface(body.get_face(f).unwrap().surface)
                .unwrap()
                .clone()
        };
        let (sa, sb) = (surface(a, fa), surface(b, fb));
        let (la, lb) = candidate.on_face_stations();
        let mut best = f64::INFINITY;
        for &(au, av) in &la {
            let pa = sa.eval(at(au), at(av)).map(I::from_certified);
            for &(bu, bv) in &lb {
                let pb = sb.eval(at(bu), at(bv)).map(I::from_certified);
                let d = (pa - pb).norm();
                best = best.min(d.hi());
            }
        }
        if best < c {
            Ok(best)
        } else {
            Err(format!("no station pair is wholly under {c}: best {best}"))
        }
    }
}

fn sweep(c: f64) -> Sweep {
    Sweep {
        bound: ClearanceBound::at_least(c),
        band: band(),
        budgets: Budgets {
            max_cell_depth: DEFAULT_MAX_CELL_DEPTH,
            max_cell_pairs: 4096,
        },
    }
}

/// Runs the bound question at `c` between one face of each body.
fn bound<T: Lane>(
    (a, fa): (&Body<T>, FaceKey),
    (b, fb): (&Body<T>, FaceKey),
    c: f64,
) -> BoundVerdict<f64> {
    let side = |body, face| Side {
        body,
        faces: vec![face],
    };
    let (sa, sb) = (side(a, fa), side(b, fb));
    let wa = sa
        .windows(Which::First)
        .expect("an iso-bounded wall has a window");
    let wb = sb
        .windows(Which::Second)
        .expect("an iso-bounded wall has a window");
    let report = sweep(c).run(&wa, &wb, false, verify((a, b), c));
    println!(
        "[nurbswin] c = {c}: {:?}, {:?}",
        report.verdict, report.receipt
    );
    assert!(report.receipt.holds(), "the receipt adds up at c = {c}");
    report.verdict
}

/// The two columns and their facing walls: `(left, its x = 1 wall)`,
/// `(right, its x = 1 + GAP wall)`.
fn facing() -> ((Lofted<I>, FaceKey), (Lofted<I>, FaceKey)) {
    let left = column(0.0);
    let right = column(2.0 + GAP);
    let lw = wall_by(&left, |p| p.x);
    let rw = wall_by(&right, |p| -p.x);
    assert!(
        (centre(&left.body, lw).x - 1.0).abs() < 1e-12
            && (centre(&right.body, rw).x - (1.0 + GAP)).abs() < 1e-12,
        "the facing walls lie in their planes"
    );
    ((left, lw), (right, rw))
}

/// **Two facing loft walls hold above their gap and are violated below
/// it**, the violation's witness a pair of wall points no nearer than
/// the gap.
#[test]
fn facing_loft_walls_hold_above_the_gap_and_violate_below_it() {
    let ((left, lw), (right, rw)) = facing();
    let (a, b) = ((&left.body, lw), (&right.body, rw));
    assert_eq!(
        bound(a, b, 0.5 * GAP),
        BoundVerdict::Holds,
        "half the gap holds"
    );
    match bound(a, b, 1.25 * GAP) {
        BoundVerdict::Violated(d) => assert!(
            (GAP - 1e-12..1.25 * GAP).contains(&d),
            "the witness is a pair of wall points, no nearer than the gap: {d}"
        ),
        other => panic!("1.25 × the gap is violated, got {other:?}"),
    }
}

/// **The minimum separation of the two walls brackets the gap.**
#[test]
fn the_minimum_separation_of_two_loft_walls_brackets_the_gap() {
    let ((left, lw), (right, rw)) = facing();
    let side = |body, face| Side {
        body,
        faces: vec![face],
    };
    let bracket = min_separation(
        &side(&left.body, lw),
        &side(&right.body, rw),
        false,
        band(),
        Budgets {
            max_cell_pairs: 16,
            ..Budgets::MIN_SEPARATION
        },
    )
    .expect("two iso-bounded walls have a bracket");
    println!("[nurbswin] {}", bracket.render());
    // The gap as built: the right column's centre `2 + GAP` rounds
    // once, and both subtractions are exact.
    let built = (2.0 + GAP - 1.0) - 1.0;
    let hi = bracket.hi().expect("a wall pair witnesses an upper end");
    assert!(
        bracket.lo() <= built && built <= hi,
        "[{}, {hi}] brackets {built}",
        bracket.lo(),
    );
    assert!(
        hi - bracket.lo() < GAP,
        "the bracket is tighter than the gap it brackets: [{}, {hi}]",
        bracket.lo(),
    );
}

/// Half-width of the swept square, metres.
const H: f64 = 0.25;
/// Radius of the bowl path, metres.
const BOWL_R: f64 = 3.0;

/// **The bowl**: a square of half-width `H` swept along the arc of
/// radius `BOWL_R` about `(0, 0, BOWL_R)` in the `xz` plane, from `−45°`
/// to `+45°` through its lowest point, the origin. The path is the
/// cubic through 17 points of that arc, symmetric about `x = 0`, and
/// the middle of the 9 stations sits at its lowest point. The wall
/// facing away from the arc's centre is curved along the sweep, and
/// its lowest point, at that station, is `(0, 0, −H)` with normal `−z`.
fn bowl() -> (Lofted<f64>, FaceKey) {
    let pts: Vec<Point3<f64>> = (0..=16)
        .map(|k| {
            let phi = FRAC_PI_4 * (f64::from(k) / 8.0 - 1.0);
            Point3::new(BOWL_R * phi.sin(), 0.0, BOWL_R * (1.0 - phi.cos()))
        })
        .collect();
    let path = NurbsCurve3::<f64>::interpolate(&pts, 3).expect("the bowl path interpolates");
    let (lo, _) = path.domain();

    let start = path_start_frame(path.eval(lo), path.deriv(lo), Tol::witness())
        .expect("the path's start tangent fixes a frame");
    let swept = sweep_body::<f64>(
        &rect(-H, H, -H, H),
        SketchPlane::from_frame(start),
        &path,
        9,
        2,
        Tol::witness(),
    )
    .expect("the bowl sweeps");
    let centre_of_arc = Point3::new(0.0, 0.0, BOWL_R);
    let outer = wall_by(&swept, |p| (p - centre_of_arc).norm());
    (swept, outer)
}

/// **A sweep wall against a plane.** The plane is the top cap of a slab
/// under the bowl's rising flank, `x ∈ [1, 1.5]`, `GAP` below the wall
/// where it is lowest over the slab, at `x = 1`. The wall's whole box
/// reaches below that cap, so no answer comes from the root cells.
///
/// The bowl's outer wall is the circle of radius `R + H` about the
/// arc's centre, swept along `y`, so the separation is the distance from
/// the cap's nearest edge, `(1, z_cap)`, to that centre, less `R + H`;
/// the interpolated wall stands off the circle by far less than the
/// margins below.
#[test]
fn a_sweep_wall_against_a_plane_holds_above_the_gap_and_violates_below_it() {
    let (swept, outer) = bowl();
    let (x0, x1) = (1.0, 1.25);
    let wall_at = |x: f64| BOWL_R - (BOWL_R + H).mul_add(BOWL_R + H, -x * x).sqrt();
    let top = wall_at(x0) - GAP;
    let separation = x0.hypot(BOWL_R - top) - (BOWL_R + H);
    let level = |z: f64| SketchPlane::xy_at(Point3::new(0.0, 0.0, z));
    let slab = loft_body::<f64>(
        &[rect(x0, x1, -0.5, 0.5), rect(x0, x1, -0.5, 0.5)],
        &[level(top - 0.5), level(top)],
        1,
        Tol::witness(),
    )
    .expect("the slab lofts");
    let a = (&swept.body, outer);
    let b = (&slab.body, slab.top);
    let (holds, violated) = (0.5 * separation, 5.0 * separation);
    assert_eq!(
        bound(a, b, holds),
        BoundVerdict::Holds,
        "half the separation {separation} holds"
    );
    match bound(a, b, violated) {
        BoundVerdict::Violated(d) => assert!(
            (0.9 * separation..violated).contains(&d),
            "the witness is a wall point over the cap: {d} against {separation}"
        ),
        other => panic!("{violated} is violated, got {other:?}"),
    }
}

/// **A rational loft's walls have windows too**, at `f64`: the arc
/// prism's arc wall carries `IsoArc` rims, and every face of the prism
/// — the planar caps beside the walls — is admitted.
#[test]
fn every_face_of_an_arc_railed_loft_has_a_window() {
    let prism = crate::common::arc_prism();
    let rational = prism.faces().any(|(_, f)| {
        matches!(
            prism.get_surface(f.surface),
            Some(Surface::Nurbs(net)) if net.weights().iter().any(|&w| w != 1.0)
        )
    });
    assert!(rational, "the arc's wall is rational");
    Side {
        body: &prism,
        faces: prism.faces().map(|(k, _)| k).collect(),
    }
    .windows(Which::First)
    .expect("every face of the arc prism has a window");
}

/// **A wall cell's enclosure narrows as it halves**: the enclosure the
/// engine's cells read (`Surface::rect_enclosure`), over the bowl's
/// outer wall, at cells of side `2^-k` of its domain about the domain's
/// centre. The halving the window door's `refines` asks for is the
/// first step of this table.
#[test]
fn a_wall_cell_enclosure_narrows_as_it_halves() {
    let (swept, outer) = bowl();
    let surface = swept
        .body
        .get_surface(swept.body.get_face(outer).unwrap().surface)
        .unwrap()
        .map_scalar(I::from_certified);
    let Surface::Nurbs(net) = &surface else {
        panic!("the wall is a NURBS face");
    };
    let ((u0, u1), (v0, v1)) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let (u0, u1, v0, v1) = (mid(u0), mid(u1), mid(v0), mid(v1));
    let (um, vm) = (0.5 * (u0 + u1), 0.5 * (v0 + v1));
    let cell = |k: i32| {
        let (hu, hv) = (
            0.5 * (u1 - u0) * 2f64.powi(-k),
            0.5 * (v1 - v0) * 2f64.powi(-k),
        );
        ((um - hu, um + hu), (vm - hv, vm + hv))
    };
    let diameter = |p: Point3<I>| Vec3::new(width(p.x), width(p.y), width(p.z)).norm();
    let start = std::time::Instant::now();
    let widths: Vec<f64> = (0..=10)
        .map(|k| {
            let (u, v) = cell(k);
            diameter(surface.rect_enclosure(u, v))
        })
        .collect();
    let per_cell = start.elapsed() / 11;
    println!("[nurbswin] rect_enclosure diameters by halving: {widths:?}, {per_cell:?} per cell");
    assert!(widths[0].is_finite(), "the whole wall encloses finitely");
    for (k, pair) in widths.windows(2).enumerate() {
        assert!(
            pair[1] < pair[0],
            "halving cell {k} narrows its enclosure: {} -> {}",
            pair[0],
            pair[1]
        );
    }
    assert!(
        widths[10] < widths[0] / 256.0,
        "ten halvings narrow the enclosure at least 256-fold: {} -> {}",
        widths[0],
        widths[10]
    );
}

/// **The pillow**: a quad of half-width `a`, its top edge `0.05`
/// longer at each end, whose two slanted `x` sides are arcs bulging
/// outward, lofted at v-degree 2 through sections at `z = 0, 0.1, 1`
/// of half-widths `0.15, 0.2, 0.12`, centred on `x = cx`. Its `x`
/// walls are rational patches curved in both directions, mirror images
/// of each other through `x = cx`. The slant puts each wall's furthest
/// point off its arc's middle, and the uneven sections put it off the
/// middle section, so it is at no station of the wall's root cell.
fn pillow(cx: f64) -> Lofted<f64> {
    let section = |a: f64| {
        let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
        vec![bulge_loop(vec![
            v(-a, -a, 0.0),
            v(a, -a, -0.3),
            v(a + 0.05, a, 0.0),
            v(-a - 0.05, a, -0.3),
        ])]
    };
    let places = [0.0, 0.1, 1.0].map(|z| SketchPlane::xy_at(Point3::new(cx, 0.0, z)));
    loft_body::<f64>(
        &[section(0.15), section(0.2), section(0.12)],
        &places,
        2,
        Tol::witness(),
    )
    .expect("the pillow lofts")
}

/// **A curved pair violated just above its separation.** Two pillows
/// face across their bulging walls. The walls are mirror images through
/// the plane between them, so their separation is `d − 2·m`, `m` the
/// furthest the left wall reaches past its centre, and a dense sample
/// of `m` bounds that separation from above. At 5 % above the sampled
/// separation the pair is violated, which the walk can only certify
/// once its cells close in on the one nearest point pair: an enclosure
/// that missed part of its cell would discharge that pair on a `lo`
/// above the bound, and the row would read `Holds`. The minimum
/// separation's `lo` must not rise above the sampled separation either.
#[test]
fn curved_walls_are_violated_just_above_their_separation() {
    let d = 0.7;
    let (left, right) = (pillow(0.0), pillow(d));
    let lw = wall_by(&left, |p| p.x);
    let rw = wall_by(&right, |p| -p.x);
    let surface = left
        .body
        .get_surface(left.body.get_face(lw).unwrap().surface)
        .unwrap();
    let Surface::Nurbs(net) = surface else {
        panic!("the bulging wall is a NURBS face");
    };
    let ((u0, u1), (v0, v1)) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let n = 400;
    let reach = (0..=n)
        .flat_map(|i| (0..=n).map(move |j| (i, j)))
        .map(|(i, j)| {
            let s = |k: usize, lo: f64, hi: f64| lo + (hi - lo) * (k as f64) / (n as f64);
            surface.eval(s(i, u0, u1), s(j, v0, v1)).x
        })
        .fold(f64::NEG_INFINITY, f64::max);
    let sampled = d - 2.0 * reach;
    assert!(sampled > 0.05, "the pillows stand apart: {sampled}");
    let c = 1.01 * sampled;
    match bound((&left.body, lw), (&right.body, rw), c) {
        BoundVerdict::Violated(w) => assert!(w < c, "the witness: {w} against {c}"),
        other => panic!("{c} is violated (separation at most {sampled}), got {other:?}"),
    }
    let side = |body, face| Side {
        body,
        faces: vec![face],
    };
    let bracket = min_separation(
        &side(&left.body, lw),
        &side(&right.body, rw),
        false,
        band(),
        Budgets::MIN_SEPARATION,
    )
    .expect("two iso-bounded walls have a bracket");
    println!(
        "[nurbswin] pillows: sampled {sampled}; {}",
        bracket.render()
    );
    assert!(
        bracket.lo() <= sampled,
        "lo {} bounds the separation, which is at most {sampled}, from below",
        bracket.lo()
    );
}
