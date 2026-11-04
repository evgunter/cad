//! **The node and touch rows** (C5, NURBS × implicit stage 3b): a node
//! decided on the classification's sweep is listed with its point, the
//! wall's normal, its two lines and its sector side, read off the
//! Hessian that decided it; its arcs are read whole, each ending on
//! entering the node's box and naming the ray it leaves along; a touch
//! beside arcs is listed with them. Each row names the mutant that turns
//! it red, and runs at the run's ε (the suite runs 1e-9, 1e-6 and 1e-12).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::sweep::{self, Sweep};
use super::{Bracketed, Listing, SplineImplicitPose};
use crate::intersect::SectionError;
use crate::pair_section::{
    ComponentShape, Components, NodeLine, PairSection, PairSide, RaySense, SectionNode,
    SectionRead, SsiLane, pair_section,
};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's tolerance forms a band")
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// The level plane `z = h`.
fn level(h: f64) -> Surface<f64> {
    Surface::Plane {
        origin: pt(0.5, 0.5, h),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// Binomial coefficient.
fn choose(n: usize, k: usize) -> f64 {
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

/// The Bernstein coefficients, of degree `c.len() − 1`, of the
/// polynomial `Σ c_k t^k` over `[0, 1]`.
fn bernstein(c: &[f64]) -> Vec<f64> {
    let n = c.len() - 1;
    (0..=n)
        .map(|i| (0..=i).map(|k| choose(i, k) / choose(n, k) * c[k]).sum())
        .collect()
}

/// The monomial coefficients of `A(s·t + o)`, `A` given by its own.
fn compose(a: &[f64], s: f64, o: f64) -> Vec<f64> {
    let mut out = vec![0.0; a.len()];
    for (k, &ak) in a.iter().enumerate() {
        // (s t + o)^k
        for (j, slot) in out.iter_mut().enumerate().take(k + 1) {
            *slot += ak * choose(k, j) * s.powi(j as i32) * o.powi((k - j) as i32);
        }
    }
    out
}

/// **A separable graph wall** `z = A(x(u)) + B(y(v))` over `[0, 1]²`,
/// `x = x0 + sx·u`, `y = y0 + sy·v`, `A` and `B` given by their monomial
/// coefficients in `x` and `y`: of degree `A`'s in `u`, `B`'s in `v`.
fn separable(a: &[f64], (x0, sx): (f64, f64), b: &[f64], (y0, sy): (f64, f64)) -> Surface<f64> {
    let (pu, pv) = (a.len() - 1, b.len() - 1);
    let au = bernstein(&compose(a, sx, x0));
    let bv = bernstein(&compose(b, sy, y0));
    let knots = |p: usize| {
        let mut k = vec![0.0; p + 1];
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    };
    let control = (0..=pu)
        .flat_map(|i| {
            let (au, bv) = (&au, &bv);
            (0..=pv).map(move |j| {
                pt(
                    x0 + sx * i as f64 / pu as f64,
                    y0 + sy * j as f64 / pv as f64,
                    au[i] + bv[j],
                )
            })
        })
        .collect();
    let n = (pu + 1) * (pv + 1);
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(knots(pu), knots(pv), control, vec![1.0; n]).unwrap(),
    ))
}

/// **The cross saddle** `z = 4(x − ½)(y − ½)` over `[0, 1]²`: its zero
/// lines `x = ½` and `y = ½` cross at the centre and reach the four sides
/// at their middles.
fn cross() -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let a = [-0.5, 0.0, 0.5];
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            pt(0.5 * i as f64, 0.5 * j as f64, 4.0 * a[i] * a[j])
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap(),
    ))
}

/// **The oblique saddle** `z = 2(x − ½)² − 4(y − ½)²`, its zero lines
/// `y − ½ = ±(x − ½)/√2` reaching the sides `x = 0` and `x = 1` inside
/// them.
fn oblique() -> Surface<f64> {
    separable(
        &[0.5, -2.0, 2.0],
        (0.0, 1.0),
        &[-1.0, 4.0, -4.0],
        (0.0, 1.0),
    )
}

/// The section `pair_section` builds of `a` and `b` through the march.
fn built(a: &Surface<f64>, b: &Surface<f64>) -> Result<PairSection<f64>, SectionError> {
    match pair_section(a, b, None, Some(SsiLane::marched()), band())? {
        SectionRead::Built(s) => Ok(s),
        SectionRead::Elsewhere(_) => panic!("a spline × implicit pair is built here"),
    }
}

/// The arcs of a counted section with an end at node `n`, each with the
/// ray it leaves along there, in its index order.
fn ends_at(s: &PairSection<f64>, n: usize) -> Vec<(usize, NodeLine, RaySense)> {
    let Components::Counted(c) = s.components() else {
        panic!("a counted section: {:?}", s.components());
    };
    let mut out = Vec::new();
    for (k, comp) in c.iter().enumerate() {
        if let ComponentShape::Open { ends } = comp.shape {
            for e in ends.nodes.both().into_iter().flatten() {
                if e.node == n {
                    out.push((k, e.line, e.sense));
                }
            }
        }
    }
    out
}

/// The wall's height over the chart's `(x, y)` along the zero set, read
/// at the point `p` of the plane `z = 0`: a separable graph's `z` there.
fn on_cross(p: Point3<f64>) -> f64 {
    4.0 * (p.x - 0.5) * (p.y - 0.5)
}

/// **A node lists its lines, its side and its four arcs.** The cross
/// saddle against `z = 0`: the centre is a node decided Zero. The value
/// lists it at `(½, ½, 0)` with the wall's normal `+z`, its lines the
/// chart's axes, the second turned from the first counterclockwise about
/// the normal, and the wall above the plane in sector 0 (where
/// `(x − ½)(y − ½) > 0`). Its four arcs each run from a side's middle to
/// the node and name the ray from the node toward that side, the four
/// rays each claimed once, one node-joined set of the four, a tangency
/// decided Zero, every carrier unminted. With the plane asked second the
/// wall is the first surface and the side is read so. Red against the
/// lines read in the wrong turn, the side bit read without `∇f·n`, a
/// ray matched as the runner-up, and the march run through the node's
/// box.
#[test]
fn a_node_lists_its_lines_side_and_four_arcs() {
    for (plane_first, wall) in [(true, PairSide::Second), (false, PairSide::First)] {
        let (p, w) = (level(0.0), cross());
        let s = if plane_first {
            built(&p, &w)
        } else {
            built(&w, &p)
        }
        .unwrap_or_else(|e| panic!("plane first {plane_first}: {e:?}"));
        let [node] = s.nodes() else {
            panic!("one node: {:?}", s.nodes());
        };
        assert!(
            node.point.distance(pt(0.5, 0.5, 0.0)) <= 1e-9,
            "{:?}",
            node.point
        );
        assert!(
            node.normal.dot(Vec3::unit_z()) > 1.0 - 1e-12,
            "{:?}",
            node.normal
        );
        for l in node.lines {
            assert!(
                l.z.abs() <= 1e-12 && (l.x.abs() > 1.0 - 1e-12 || l.y.abs() > 1.0 - 1e-12),
                "a line along a chart axis: {l:?}"
            );
        }
        assert!(
            node.lines[0].cross(node.lines[1]).dot(node.normal) > 0.0,
            "the second line counterclockwise from the first: {:?}",
            node.lines
        );
        // Sector 0, between the two positive rays: the wall's height there.
        let mid = pt(0.5, 0.5, 0.0) + (node.lines[0] + node.lines[1]) * 0.1;
        let wall_above = on_cross(mid) > 0.0;
        assert_eq!(
            node.above,
            if wall_above { wall } else { wall.other() },
            "plane first {plane_first}: the side in sector 0"
        );
        let ends = ends_at(&s, 0);
        assert_eq!(ends.len(), 4, "four arc ends at the node: {ends:?}");
        let mut rays: Vec<usize> = ends
            .iter()
            .map(|&(_, l, sense)| SectionNode::<f64>::ray_index(l, sense))
            .collect();
        rays.sort_unstable();
        assert_eq!(rays, [0, 1, 2, 3], "each ray claimed once");
        let Components::Counted(c) = s.components() else {
            unreachable!()
        };
        for &(k, l, sense) in &ends {
            // The arc's witness lies on the side of the node its ray
            // points to.
            let toward = c[k].witness.unwrap() - node.point;
            assert!(
                toward.dot(node.ray(l, sense)) > 0.0,
                "arc {k} names the ray toward its side: {toward:?}"
            );
            assert!(matches!(
                c[k].carrier,
                crate::pair_section::ClassCarrier::Unminted(_)
            ));
        }
        assert_eq!(s.node_sets().len(), 1, "{:?}", s.node_sets());
        assert_eq!(s.node_sets()[0].arcs, [0, 1, 2, 3]);
        assert!(
            s.tangency()
                .is_some_and(|z| z.decision.name() == "spline_critical_value"),
            "{:?}",
            s.tangency()
        );
    }
}

/// **An oblique node's lines are its null lines.** The oblique saddle
/// against `z = 0`: the node's lines are the zero lines
/// `y − ½ = ±(x − ½)/√2`, not the chart's axes, so each line's point a
/// tenth from the node stands on the wall's zero set, and each arc's
/// witness on the ray it names. Red against a null direction read off
/// the other root.
#[test]
fn an_oblique_nodes_lines_are_its_null_lines() {
    let s = built(&level(0.0), &oblique()).unwrap();
    let [node] = s.nodes() else {
        panic!("one node: {:?}", s.nodes());
    };
    for l in node.lines {
        let q = node.point + l * 0.1;
        let z = 2.0 * (q.x - 0.5).powi(2) - 4.0 * (q.y - 0.5).powi(2);
        assert!(
            z.abs() <= 1e-9,
            "a line off the zero set: {l:?} reads {z:e}"
        );
    }
    assert_eq!(ends_at(&s, 0).len(), 4);
}

/// **A loop through a node names both its ends.** The lemniscate of
/// Gerono `y² = x² − x⁴`, the zero set of `z = x⁴ − x² + y²` over
/// `x ∈ [−1.2, 1.2]`, `y ∈ [−1, 1]`: two loops leave the node at the
/// origin and return to it, no side reached. Each loop is an arc with both
/// ends at the node, the two ends naming two different rays, the four rays
/// claimed once between them, one node-joined set. Red against the node
/// box left out of the march (each loop closes on its seed through the
/// node and certifies nowhere) and against a between-node arc never traced
/// from its seeds.
#[test]
fn a_loop_through_a_node_names_both_its_ends() {
    let wall = separable(
        &[0.0, 0.0, -1.0, 0.0, 1.0],
        (-1.2, 2.4),
        &[0.0, 0.0, 1.0],
        (-1.0, 2.0),
    );
    let s = built(&level(0.0), &wall).unwrap();
    let [node] = s.nodes() else {
        panic!("one node: {:?}", s.nodes());
    };
    assert!(
        node.point.distance(pt(0.0, 0.0, 0.0)) <= 1e-9,
        "{:?}",
        node.point
    );
    let ends = ends_at(&s, 0);
    assert_eq!(ends.len(), 4, "{ends:?}");
    let mut arcs: Vec<usize> = ends.iter().map(|e| e.0).collect();
    arcs.dedup();
    assert_eq!(
        arcs.len(),
        2,
        "two loops, each with both ends at the node: {ends:?}"
    );
    let mut rays: Vec<usize> = ends
        .iter()
        .map(|&(_, l, sense)| SectionNode::<f64>::ray_index(l, sense))
        .collect();
    rays.sort_unstable();
    assert_eq!(rays, [0, 1, 2, 3]);
    assert_eq!(s.node_sets().len(), 1);
}

/// **The touch-and-arc wall**: `z = k(A(x) + B(y))` over `[0, 1]²`,
/// `k = 2`, `B(y) = −(y − ½)²`, and `A` the quadratic spline with a knot at
/// `x = ½`: `−(x − 0.3)²` before it and `4x² − 4.4x + 1.16` after, which
/// meet with one slope there. Its top at `(0.3, ½)` touches `z = 0`, and
/// past `x ≈ 0.66` it rises through `z = 0` in an arc from the side
/// `v = 0` to `v = 1`.
fn touch_and_arc() -> Surface<f64> {
    let k = 2.0;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    // A's spline coefficients: its blossoms at (0,0), (0,½), (½,1), (1,1);
    // the Greville abscissae 0, ¼, ¾, 1.
    let (a, xs) = ([-0.09, 0.06, -0.14, 0.76], [0.0, 0.25, 0.75, 1.0]);
    let b = [-0.25, 0.25, -0.25];
    let control = (0..12)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            pt(xs[i], 0.5 * j as f64, k * (a[i] + b[j]))
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, vec![1.0; 12]).unwrap(),
    ))
}

/// **A touch beside an arc is listed with it.** The touch-and-arc wall
/// against `z = 0`: its top touches the plane, its Hessian definite, and
/// an arc crosses the wall from the side `v = 0` to `v = 1`. The section
/// is the arc and the touch, a point witnessed at the top, a tangency
/// decided Zero, no pinch. Red against the touch beside ends left to
/// pinch, and against the touch's box not claimed from seeding and
/// accounting.
#[test]
fn a_touch_beside_an_arc_is_listed_with_it() {
    let s = built(&level(0.0), &touch_and_arc()).unwrap_or_else(|e| panic!("{e:?}"));
    let Components::Counted(c) = s.components() else {
        panic!("{:?}", s.components());
    };
    let points: Vec<_> = c
        .iter()
        .filter(|c| c.shape == ComponentShape::Point)
        .collect();
    assert!(
        matches!(points.as_slice(), [p] if p.witness.is_some_and(|w| w.distance(pt(0.3, 0.5, 0.0)) <= 1e-6)),
        "one touch at the top: {c:?}"
    );
    assert_eq!(
        c.iter()
            .filter(|c| matches!(c.shape, ComponentShape::Open { .. }))
            .count(),
        1,
        "the arc beside it: {c:?}"
    );
    assert!(s.nodes().is_empty());
    assert!(
        s.tangency()
            .is_some_and(|z| z.decision.name() == "spline_critical_value"),
        "{:?}",
        s.tangency()
    );
}

/// The cross saddle's reading, its decided points and its outcome, read
/// as the classification reads them.
fn crossing() -> (
    SplineImplicitPose<f64>,
    Vec<sweep::Critical>,
    crate::ssi::SsiOutcome,
) {
    let (p, w) = (level(0.0), cross());
    let pose = SplineImplicitPose::of(&p, &w).unwrap();
    let reading = crate::ImplicitSection::of(&p, pose.net(), pose.domain(), band()).unwrap();
    let Sweep::LoopFree(decided) = sweep::sweep(
        reading.boxes(),
        reading.form(),
        reading.account_floor(),
        band(),
        pose.ball().certified_diameter(),
    ) else {
        panic!("the cross saddle is swept loop-free");
    };
    let out = super::read_crossing(reading, &decided).unwrap();
    (pose, decided, out)
}

/// The listing of `out` with `decided` handed in.
fn listing(
    pose: SplineImplicitPose<f64>,
    decided: &[sweep::Critical],
    out: crate::ssi::SsiOutcome,
) -> Result<PairSection<f64>, SectionError> {
    let heights = pose.pivot().heights;
    let none = vec![None; decided.len()];
    let listing = Listing {
        decided,
        witnesses: &none,
    };
    super::listed(pose, Bracketed::Plane(heights), out, &[listing], band())
}

/// **A node not placed within the band refuses by name.** The cross
/// saddle's node, its point enclosure read wider than the band's zero:
/// the listing refuses `spline_node_box` rather than list a node it has
/// not placed. Red against the width left unread.
#[test]
fn a_node_not_placed_within_the_band_refuses() {
    let (pose, mut decided, out) = crossing();
    if let Some(frame) = decided[0].node.as_mut() {
        frame.width = 2.0 * band().zero();
    }
    let got = listing(pose, &decided, out);
    assert!(
        matches!(
            got,
            Err(SectionError::Pinched {
                predicate: "spline_node_box"
            })
        ),
        "{got:?}"
    );
}

/// **A node whose rays are not each claimed once refuses by name.** The
/// cross saddle's reading with one of its arcs listed twice: two ends
/// claim one ray, and the listing refuses `spline_node_ray`; with one arc
/// dropped, a ray is unclaimed, and it refuses the same. Red against a
/// doubled claim kept, and against the four-claim check skipped.
#[test]
fn a_node_whose_rays_are_not_each_claimed_once_refuses() {
    for edit in ["doubled", "dropped"] {
        let (pose, decided, mut out) = crossing();
        match edit {
            "doubled" => {
                let again = out.branches[0].clone();
                out.branches.push(again);
            }
            _ => {
                out.branches.pop();
            }
        }
        let got = listing(pose, &decided, out);
        assert!(
            matches!(
                got,
                Err(SectionError::Pinched {
                    predicate: "spline_node_ray"
                })
            ),
            "{edit}: {got:?}"
        );
    }
}

/// **An end between two rays names neither.** An end leaving at 45° off
/// both of two neighbouring rays leads neither by a decided gap and is
/// refused or escalated, never named; one along a ray names it; one 50°
/// off every ray is past the floor and refused. Red against the runner-up
/// read as the ray, and against the 45° floor skipped.
#[test]
fn an_end_between_two_rays_names_neither() {
    let rays = [
        Vec3::unit_x(),
        Vec3::unit_y(),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    ];
    let extent = geom_core::Lever::of_consumed_extent(geom_core::Interval::from_certified(1.0));
    let between = Vec3::new(1.0, 1.0, 0.0) * core::f64::consts::FRAC_1_SQRT_2;
    assert!(super::ray_of(between, rays, extent, band()).is_err());
    assert_eq!(super::ray_of(Vec3::unit_y(), rays, extent, band()), Ok(1));
    let near = Vec3::new(0.9, 0.1, 0.0);
    let near = near * (1.0 / near.norm());
    assert_eq!(super::ray_of(near, rays, extent, band()), Ok(0));
    // 50° off the x axis, out of the plane of the rays: past the floor.
    let (c, s) = (50f64.to_radians().cos(), 50f64.to_radians().sin());
    assert!(super::ray_of(Vec3::new(c, 0.0, s), rays, extent, band()).is_err());
}
