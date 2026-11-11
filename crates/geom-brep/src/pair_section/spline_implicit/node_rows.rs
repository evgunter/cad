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

/// Binomial coefficient, `0` where `k > n`.
fn choose(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
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
    cross_charted(false)
}

/// **The cross saddle over a chart run either way**: with `flipped`, `x`
/// falls as `u` rises, so the wall's normal `S_u × S_v` is `−z` and the
/// chart's Hessian reads the same sector sign against the opposite
/// normal.
fn cross_charted(flipped: bool) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let a = [-0.5, 0.0, 0.5];
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            let x = if flipped {
                1.0 - 0.5 * i as f64
            } else {
                0.5 * i as f64
            };
            pt(x, 0.5 * j as f64, 4.0 * (x - 0.5) * a[j])
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
/// wall is the first surface and the side is read so; over a chart run
/// the other way the normal is `−z` and the side is read along it. Red against the
/// lines read in the wrong turn, the side bit read without `∇f·n`, a
/// ray matched as the runner-up, and the march run through the node's
/// box.
#[test]
fn a_node_lists_its_lines_side_and_four_arcs() {
    for (flipped, plane_first, wall) in [
        (false, true, PairSide::Second),
        (false, false, PairSide::First),
        (true, true, PairSide::Second),
        (true, false, PairSide::First),
    ] {
        let (p, w) = (level(0.0), cross_charted(flipped));
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
        let up = if flipped { -1.0 } else { 1.0 };
        assert!(
            node.normal.dot(Vec3::unit_z()) * up > 1.0 - 1e-12,
            "flipped {flipped}: the wall's own normal: {:?}",
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
        let wall_above = on_cross(mid) * node.normal.z > 0.0;
        assert_eq!(
            node.above,
            if wall_above { wall } else { wall.other() },
            "flipped {flipped}, plane first {plane_first}: the side in sector 0, along the \
             node's normal"
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
    // Pinned red at 1e-12: the chart accounting runs out of cells, which
    // a reading with its node handed in refuses as the node's tangency
    // (`work/sectarms/the-lemniscate-node-row-exceeds-the-accounting-budget-at-1e-12.md`).
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &[1e-12],
        "the lemniscate's node",
        &["Pinched { predicate: \"spline_critical_unread\" }"],
        a_loop_through_a_node_names_both_its_ends_row,
    );
}

/// The row [`a_loop_through_a_node_names_both_its_ends`] runs, under its
/// pin.
fn a_loop_through_a_node_names_both_its_ends_row() {
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
    witnessed_rays(&s, node);
}

/// Each arc end at `node` names the ray toward its own arc: the arc's
/// witness lies on that ray's side of the node. Red against an end's
/// sense read the other way (a start read as an end), where a loop's two
/// ends trade rays.
fn witnessed_rays(s: &PairSection<f64>, node: &SectionNode<f64>) {
    let Components::Counted(c) = s.components() else {
        panic!("a counted section: {:?}", s.components());
    };
    for (k, l, sense) in ends_at(s, 0) {
        let toward = c[k].witness.unwrap() - node.point;
        assert!(
            toward.dot(node.ray(l, sense)) > 0.0,
            "arc {k}'s end names the ray ({l:?}, {sense:?}) away from its witness {toward:?}"
        );
    }
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
/// decided Zero, no pinch. The sweep finds the touch's cells enclosed,
/// so the pair is read through `Sweep::Enclosing`. Red against an
/// enclosing section read without its points, the touch's box not read
/// as a region by the accounting, and the box not grown.
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
    let pivot = pose.pivot();
    let form = crate::ssi::residual::ImplicitForm::of(&pivot.plane)
        .unwrap()
        .lifted(pivot.lift);
    let none = vec![None; decided.len()];
    let listing = Listing {
        decided,
        witnesses: &none,
    };
    let partner = Bracketed {
        surface: &pivot.plane,
        form: &form,
    };
    super::listed(pose, partner, out, Some(&listing), band())
}

/// **A node not placed within the band refuses by name.** The cross
/// saddle's node, its point enclosure read wider than the band's zero, or
/// not certified (its width NaN): the listing refuses `spline_node_box`
/// rather than list a node it has not placed. Red against the width left
/// unread, and against a NaN width read as within the zero.
#[test]
fn a_node_not_placed_within_the_band_refuses() {
    for width in [2.0 * band().zero(), f64::NAN] {
        let (pose, mut decided, out) = crossing();
        if let Some(frame) = decided[0].node.as_mut() {
            frame.width = width;
        }
        let got = listing(pose, &decided, out);
        assert!(
            matches!(
                got,
                Err(SectionError::Pinched {
                    predicate: "spline_node_box"
                })
            ),
            "{width}: {got:?}"
        );
    }
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
    let extent = geom_core::test_support::fixture_lever(geom_core::Interval::from_certified(1.0));
    let between = Vec3::new(1.0, 1.0, 0.0) * core::f64::consts::FRAC_1_SQRT_2;
    assert!(super::ray_of(between, rays, extent, band()).is_err());
    assert_eq!(super::ray_of(Vec3::unit_y(), rays, extent, band()), Ok(1));
    let near = Vec3::new(0.9, 0.1, 0.0);
    let near = near.normalize();
    assert_eq!(super::ray_of(near, rays, extent, band()), Ok(0));
    // 50° off the x axis, out of the plane of the rays: past the floor.
    let (c, s) = (50f64.to_radians().cos(), 50f64.to_radians().sin());
    assert!(super::ray_of(Vec3::new(c, 0.0, s), rays, extent, band()).is_err());
    // Lines 30° apart: an end at 15°, between the two rays, is within 45°
    // of both, so the floor passes it and the lead alone refuses it; one
    // at 5° leads at 0, one at 25° at 1.
    let at = |deg: f64| Vec3::new(deg.to_radians().cos(), deg.to_radians().sin(), 0.0);
    let narrow = [at(0.0), at(30.0), -at(0.0), -at(30.0)];
    assert!(super::ray_of(at(15.0), narrow, extent, band()).is_err());
    assert_eq!(super::ray_of(at(5.0), narrow, extent, band()), Ok(0));
    assert_eq!(super::ray_of(at(25.0), narrow, extent, band()), Ok(1));
}

/// **A touch beside components that reach a side crosses them.** The
/// bowl `z = −2(x − ½)² − 2(y − ½)²` against `z = 0` is swept loop-free
/// with its top a touch decided Zero. Alone it is listed beside nothing
/// (the classification's loop-free listing); with any side reached it
/// is read whole with its box handed in, its arcs listed with it. The
/// touch-and-arc wall reaches the listing through `Sweep::Enclosing`,
/// so this is the row that reads `crosses`'s touch arm on a loop-free
/// sweep. Red against a touch beside ends left to the loop-free
/// listing.
#[test]
fn a_touch_beside_reaching_components_crosses_them() {
    use crate::pair_section::ChartReach;
    use crate::ssi::{ChartAxis, ChartEnd, ChartSide};
    let bowl = separable(
        &[-0.5, 2.0, -2.0],
        (0.0, 1.0),
        &[-0.5, 2.0, -2.0],
        (0.0, 1.0),
    );
    let p = level(0.0);
    let pose = SplineImplicitPose::of(&p, &bowl).unwrap();
    let reading = crate::ImplicitSection::of(&p, pose.net(), pose.domain(), band()).unwrap();
    let Sweep::LoopFree(decided) = sweep::sweep(
        reading.boxes(),
        reading.form(),
        reading.account_floor(),
        band(),
        pose.ball().certified_diameter(),
    ) else {
        panic!("the bowl is swept loop-free");
    };
    assert!(
        matches!(decided.as_slice(), [c] if c.kind == sweep::Kind::Touch),
        "{decided:?}"
    );
    assert!(!super::crosses(&decided, ChartReach::default()));
    for fixed in [ChartAxis::U, ChartAxis::V] {
        for end in [ChartEnd::Low, ChartEnd::High] {
            assert!(
                super::crosses(&decided, ChartReach::side(ChartSide { fixed, end })),
                "a side reached: {fixed:?} {end:?}"
            );
        }
    }
}

/// **A biquadratic wall over a linear chart**: `z = Σ m[k][l] u^k v^l`
/// over `[0, 1]²`, at `(x, y) = chart(u, v)`, which is affine, so the
/// control points' `(x, y)` are the chart at the Greville abscissae.
fn biquad(m: [[f64; 3]; 3], chart: impl Fn(f64, f64) -> (f64, f64)) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    // The Bernstein coefficient (i, j) of u^k v^l: (C(i,k)/C(2,k))·(C(j,l)/C(2,l)).
    let w = |i: usize, k: usize| choose(i, k) / choose(2, k);
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            let z = (0..3)
                .flat_map(|k| (0..3).map(move |l| (k, l)))
                .map(|(k, l)| w(i, k) * w(j, l) * m[k][l])
                .sum();
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = chart(0.5 * i as f64, 0.5 * j as f64);
            pt(x, y, z)
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap(),
    ))
}

/// The monomial coefficients in `(u, v)` of `k·(α₁X + β₁Y)(α₂X + β₂Y)`,
/// `X = u − ½ + s(v − ½)`, `Y = v − ½`: a node at the centre on the lines
/// `αX + βY = 0`, over a chart sheared by `s`.
fn two_lines(k: f64, [(a1, b1), (a2, b2)]: [(f64, f64); 2], s: f64) -> [[f64; 3]; 3] {
    // A linear form αX + βY in (u, v): constant, u and v coefficients.
    let line = |a: f64, b: f64| {
        [
            [a * (-0.5 - 0.5 * s) - 0.5 * b, a * s + b, 0.0],
            [a, 0.0, 0.0],
            [0.0; 3],
        ]
    };
    let (p, q) = (line(a1, b1), line(a2, b2));
    let mut out = [[0.0; 3]; 3];
    for i in 0..2 {
        for j in 0..2 {
            for k2 in 0..2 {
                for l in 0..2 {
                    out[i + k2][j + l] += k * p[i][j] * q[k2][l];
                }
            }
        }
    }
    out
}

/// **A generic node is listed.** Nodes whose lines are not the chart's
/// axes nor symmetric about them, at the run's ε: `2(Y − 1.5X)(Y + 1.5X)`
/// and `2(Y − 1.5X)(Y + 0.4X)`, the cross saddle `4XY` over a chart
/// sheared by `0.2` (`x = u + 0.2(v − ½)`), and the first under a sphere
/// of radius 100 resting on the node. Each lists one node and its four
/// arcs, every ray claimed once, each end naming the ray toward its own
/// arc. Red against the node ends' spans left ungraded (`Ends::graded`):
/// one span from the side to the node's box straddles in limb 3's tube.
#[test]
fn a_generic_node_is_listed() {
    let plain = |u: f64, v: f64| (u, v);
    let crossing = biquad(two_lines(2.0, [(-1.5, 1.0), (1.5, 1.0)], 0.0), plain);
    let leaning = biquad(two_lines(2.0, [(-1.5, 1.0), (0.4, 1.0)], 0.0), plain);
    let sheared = biquad(two_lines(4.0, [(1.0, 0.0), (0.0, 1.0)], 0.2), |u, v| {
        (u + 0.2 * (v - 0.5), v)
    });
    let ball = Surface::Sphere {
        center: pt(0.5, 0.5, 100.0),
        radius: 100.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for (what, partner, wall) in [
        ("lines at ±1.5", level(0.0), &crossing),
        ("lines at 1.5 and −0.4", level(0.0), &leaning),
        ("a sheared chart", level(0.0), &sheared),
        ("under a sphere", ball, &crossing),
    ] {
        let s = built(&partner, wall).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let [node] = s.nodes() else {
            panic!("{what}: one node: {:?}", s.nodes());
        };
        let mut rays: Vec<usize> = ends_at(&s, 0)
            .iter()
            .map(|&(_, l, sense)| SectionNode::<f64>::ray_index(l, sense))
            .collect();
        rays.sort_unstable();
        assert_eq!(rays, [0, 1, 2, 3], "{what}: each ray claimed once");
        witnessed_rays(&s, node);
    }
}

/// **A node the reading cannot account for reads as a tangency.** Lines
/// 45° apart in the chart (`4Y(Y − X)`): the sweep decides the node, but
/// the accounting about it runs out of cells, so the section refuses as
/// the pinch topo reads as a tangency (`spline_critical_unread`), never
/// as an unaccounted pair
/// (`work/sectarms/a-spline-node-the-reading-cannot-account-for-reads-as-a-tangency.md`).
/// Red against an unaccounted reading at a decided node left
/// unaccounted.
#[test]
fn a_node_the_reading_cannot_account_for_reads_as_a_tangency() {
    let wall = biquad(two_lines(4.0, [(0.0, 1.0), (-1.0, 1.0)], 0.0), |u, v| {
        (u, v)
    });
    assert!(
        matches!(
            built(&level(0.0), &wall),
            Err(SectionError::Pinched {
                predicate: super::CRITICAL_UNREAD
            })
        ),
        "{:?}",
        built(&level(0.0), &wall).map(|s| s.nodes().len())
    );
}

/// **Two decided points whose boxes meet decline the sweep**
/// ([`sweep::overlapping`]): the bowl's touch beside a copy of it moved
/// half its box's width, and a copy moved clear of it. Red against the
/// overlap left undeclined.
#[test]
fn two_decided_boxes_that_meet_decline() {
    let bowl = separable(
        &[-0.5, 2.0, -2.0],
        (0.0, 1.0),
        &[-0.5, 2.0, -2.0],
        (0.0, 1.0),
    );
    let p = level(0.0);
    let pose = SplineImplicitPose::of(&p, &bowl).unwrap();
    let reading = crate::ImplicitSection::of(&p, pose.net(), pose.domain(), band()).unwrap();
    let Sweep::LoopFree(decided) = sweep::sweep(
        reading.boxes(),
        reading.form(),
        reading.account_floor(),
        band(),
        pose.ball().certified_diameter(),
    ) else {
        panic!("the bowl is swept loop-free");
    };
    let [top] = decided.as_slice() else {
        panic!("{decided:?}");
    };
    let moved = |by: f64| sweep::Critical {
        at: (top.at.0 + by, top.at.1),
        region: crate::ssi::exhaust::UvRect {
            u: (top.region.u.0 + by, top.region.u.1 + by),
            v: top.region.v,
        },
        ..*top
    };
    let width = top.region.u.1 - top.region.u.0;
    let near = moved(0.5 * width);
    assert_eq!(sweep::overlapping(&[*top, near]), Some(near.at));
    assert_eq!(sweep::overlapping(&[*top, moved(2.0 * width)]), None);
}
