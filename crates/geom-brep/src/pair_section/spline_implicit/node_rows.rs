//! **The node and touch rows** (C5, NURBS × implicit stage 3b): a node
//! decided on the classification's sweep is listed with its point, the
//! wall's normal, its two lines and its sector side, read off the
//! Hessian that decided it; its arcs are read whole, each carried on to
//! the node and naming the ray it leaves along (C2's node end); a touch
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

/// Whether component `k`'s start, rather than its end, is at a node.
fn ends_at_start(c: &[crate::pair_section::SectionComponent<f64>], k: usize) -> bool {
    match c[k].shape {
        ComponentShape::Open { ends } => ends.nodes.start.is_some(),
        _ => panic!("an open arc"),
    }
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
/// decided Zero, every arc carried to the node: its span ends at the
/// node's point leaving along its ray, its last box the node's. With the plane asked second the
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
            // Carried to the node: its span ends at the node's point,
            // leaving along the ray it names, its pad read box by box
            // with the node's own box last.
            let crate::pair_section::ClassCarrier::Fitted(arc) = &c[k].carrier else {
                panic!("arc {k} is carried: {:?}", c[k].carrier);
            };
            let arc = arc.as_ref().as_ref().expect("arc carried to its node");
            let starts = ends_at_start(c, k);
            let t = if starts { arc.span.0 } else { arc.span.1 };
            assert!(
                arc.carrier.eval(t).distance(node.point) <= 1e-9,
                "arc {k} ends at the node: {:?}",
                arc.carrier.eval(t)
            );
            let leaving = arc.carrier.ders1(t).1 * if starts { 1.0 } else { -1.0 };
            assert!(
                leaving.normalize().dot(node.ray(l, sense)) > 0.99,
                "arc {k} leaves along its ray: {leaving:?}"
            );
            let held = if starts {
                arc.boxes.first()
            } else {
                arc.boxes.last()
            };
            assert_eq!(
                held.and_then(|b| b.node),
                Some(0),
                "arc {k}'s last box is the node's: {:?}",
                arc.boxes
            );
            assert!(
                arc.boxes.iter().filter(|b| b.node.is_some()).count() == 1,
                "one node box"
            );
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

/// **A node the reading cannot certify about reads as a tangency.**
/// Lines 45° apart in the chart (`4Y(Y − X)`): the sweep decides the
/// node, but its arcs' tubes straddle near it at every grade, so the
/// section refuses as a pinch named by the tube's transversality, which
/// topo reads as a tangency, never as an unaccounted pair
/// (`work/sectarms/a-spline-node-the-reading-cannot-account-for-reads-as-a-tangency.md`).
/// Red against the straddle carried as an unaccounted reading.
#[test]
fn a_node_the_reading_cannot_account_for_reads_as_a_tangency() {
    let wall = biquad(two_lines(4.0, [(0.0, 1.0), (-1.0, 1.0)], 0.0), |u, v| {
        (u, v)
    });
    let got = built(&level(0.0), &wall);
    assert!(
        matches!(
            got,
            Err(SectionError::Pinched {
                predicate: super::CRITICAL_TUBE
            })
        ),
        "{:?}",
        got.map(|s| s.nodes().len())
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

/// **An arc carried to its node reads its sites box by box** (C2's node
/// end). The cross saddle against `z = 0` carries its four arcs to the
/// node, each one's pad read box by box. A site at the node reads the
/// node; a site on the `+x` arc three node boxes out reads that arc
/// alone, though the arc's widest pad reaches the `+y` arc beside it; a
/// site in the node's box and off the node reads on the arcs whose boxes
/// there hold it, which share the node, and escalates on its distance
/// from it.
///
/// Red against: the pad read whole (the site three boxes out reads on
/// two arcs and escalates); and a site on two arcs that share a node,
/// off it, read on the last of them (`two-node-reading-arms-no-row-reaches`'s
/// m9: the box's site reads on an arc).
#[test]
fn an_arc_carried_to_its_node_reads_its_sites_box_by_box() {
    use crate::pair_section::{SiteReading, SiteRefusal};
    let s = built(&level(0.0), &cross()).expect("the cross reads");
    let carried = s
        .carry(Some(SsiLane::marched()), band())
        .expect("its four arcs carry to the node");
    let node = pt(0.5, 0.5, 0.0);
    let toward_x = carried
        .components()
        .iter()
        .position(|c| c.witness.x > 0.6)
        .expect("an arc toward +x");
    let arc = &carried.components()[toward_x];
    let node_box = arc
        .boxes
        .iter()
        .find(|b| b.node.is_some())
        .expect("the arc's node box");
    let h = 0.5 * (node_box.hi[0] - node_box.lo[0]);
    let out = node + Vec3::unit_x() * (3.0 * h);
    assert!(
        arc.pad > 2.0 * 3.0 * h,
        "the widest pad {} reaches the +y arc from {out:?}",
        arc.pad
    );
    match carried.site(out, band()) {
        Ok(SiteReading::OnArc(foot)) => assert_eq!(foot.component, toward_x),
        other => panic!("{out:?} reads on the +x arc alone: {other:?}"),
    }
    let inside = node + Vec3::unit_x() * (0.5 * h);
    assert!(
        matches!(carried.site(inside, band()), Err(SiteRefusal::Escalated(_))),
        "{inside:?} in the node's box escalates: {:?}",
        carried.site(inside, band())
    );
    assert!(matches!(
        carried.site(node, band()),
        Ok(SiteReading::AtNode(0))
    ));
}

/// **An edge through or short of a node is no node's at rest** (C2's
/// node end, re-read about the edge's end vertex): straight edges along
/// the cross's line `y = ½` from its rim, at the edge certificate's
/// door, each ending a centimetre, a millimetre or a micron past the
/// node, refuse: none is read as ending at the node, whose box would
/// waive the transverse enclosure over the stretch where it straddles.
/// One ending a millimetre short of it certifies, an arc's end and no
/// node's.
///
/// Red against: the node's box read over the end span's reach, and the
/// end left unbound to the node.
#[test]
fn an_edge_through_or_short_of_a_node_is_no_nodes_at_rest() {
    use crate::certify::{EdgeCurve, EdgeCurveSpec, NurbsLane};
    use crate::description::{CurveSelector, EdgeDescriptionSpec, SurfacePair};
    use geom::{Curve3, NurbsCurve3};
    let (partner, wall) = (level(0.0), cross());
    let k1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    for (what, x_end) in [
        ("past by a centimetre", 0.51),
        ("past by a millimetre", 0.501),
        ("past by a micron", 0.500_001),
    ] {
        let carrier = Curve3::Nurbs(std::sync::Arc::new(
            NurbsCurve3::new(
                k1.clone(),
                vec![pt(0.0, 0.5, 0.0), pt(x_end, 0.5, 0.0)],
                vec![1.0; 2],
            )
            .unwrap(),
        ));
        let mut arena = slotmap::SlotMap::<crate::keys::SurfaceKey, Surface<f64>>::with_key();
        let (k_a, k_b) = (arena.insert(partner.clone()), arena.insert(wall.clone()));
        let edge = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(k_a, k_b),
                    selector: CurveSelector::Witness(carrier.mid_point(0.0, 1.0)),
                },
                carrier: carrier.clone(),
                param_start: 0.0,
                param_end: 1.0,
            },
            carrier.eval(0.0),
            carrier.eval(1.0),
            move |k| arena.get(k).cloned(),
            geom_core::test_support::fixture_lever(1.0),
            band(),
            NurbsLane::certified(),
        );
        assert!(edge.is_err(), "{what}: refuses at rest");
    }
    let short = Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(
            k1,
            vec![pt(0.0, 0.5, 0.0), pt(0.499, 0.5, 0.0)],
            vec![1.0; 2],
        )
        .unwrap(),
    ));
    let mut arena = slotmap::SlotMap::<crate::keys::SurfaceKey, Surface<f64>>::with_key();
    let (k_a, k_b) = (arena.insert(partner), arena.insert(wall));
    let edge = EdgeCurve::certify(
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(k_a, k_b),
                selector: CurveSelector::Witness(short.mid_point(0.0, 1.0)),
            },
            carrier: short.clone(),
            param_start: 0.0,
            param_end: 1.0,
        },
        short.eval(0.0),
        short.eval(1.0),
        move |k| arena.get(k).cloned(),
        geom_core::test_support::fixture_lever(1.0),
        band(),
        NurbsLane::certified(),
    );
    assert!(
        edge.is_ok(),
        "short by a millimetre: an arc's end certifies: {edge:?}"
    );
}

/// The cross saddle `k·XY` and the 78° node `k·(Y − 1.5X)(Y + 0.4X)`
/// over a chart scaled by `s`, and the lemniscate scaled by `s`, the
/// three node fixtures the scale rows read.
fn scaled(s: f64) -> [(&'static str, Surface<f64>); 2] {
    [
        (
            "the cross",
            biquad(two_lines(4.0 * s, [(1.0, 0.0), (0.0, 1.0)], 0.0), |u, v| {
                (s * u, s * v)
            }),
        ),
        (
            "the 78° node",
            biquad(
                two_lines(2.0 * s, [(-1.5, 1.0), (0.4, 1.0)], 0.0),
                |u, v| (s * u, s * v),
            ),
        ),
    ]
}

/// The lemniscate of [`a_loop_through_a_node_names_both_its_ends`],
/// scaled by `s`.
fn scaled_lemniscate(s: f64) -> Surface<f64> {
    separable(
        &[0.0, 0.0, -1.0 / s, 0.0, 1.0 / (s * s * s)],
        (-1.2 * s, 2.4 * s),
        &[0.0, 0.0, 1.0 / s],
        (-s, 2.0 * s),
    )
}

/// Every arc of the carried section `c` ends at `node` (within `tol`),
/// its node box its last box.
fn carried_to(
    what: &str,
    c: &crate::pair_section::CarriedSection<f64>,
    node: Point3<f64>,
    tol: f64,
) {
    for (k, arc) in c.components().iter().enumerate() {
        let [start, end] = arc.shape.nodes();
        let t = match (start, end) {
            (Some(_), _) => arc.span.0,
            (None, Some(_)) => arc.span.1,
            (None, None) => panic!("{what}: arc {k} ends at the node"),
        };
        assert!(
            arc.carrier.eval(t).distance(node) <= tol,
            "{what}: arc {k} is carried to the node: {:?}",
            arc.carrier.eval(t)
        );
        assert!(
            arc.boxes.iter().any(|b| b.node.is_some()),
            "{what}: arc {k}'s last box is the node's"
        );
    }
}

/// **A node is listed and carried to at every scale** (C2's node end,
/// C5): the cross and the 78° node over walls a hundredth, one and a
/// hundred units across each list one node and carry their four arcs to
/// it, at the run's ε (but the 78° node a hundred units across at
/// 1e-12, whose chain straddles before it reaches the node, as on
/// main). At a hundredth of a unit and ε 1e-6 the node's
/// settle reach is a quarter of the wall, and its box reaches past the
/// box the march entered, where one reading of the wall's jet over it
/// no longer contracts.
///
/// Red against: the node box read with one reading of the Hessian over
/// it (`node::NODE_BOX_SPLITS` 1): at a hundredth of a unit the node is
/// not read at the chain's end, whose tube straddles.
#[test]
fn a_node_is_listed_and_carried_to_at_every_scale() {
    for s in [0.01, 1.0, 100.0] {
        for (what, wall) in scaled(s) {
            let what = format!("{what} at {s}");
            // At 1e-12 a hundred units across, the 78° node's chain to
            // the box its march enters straddles, as it does on main.
            if what == "the 78° node at 100" && Tol::witness().eps() <= 1e-12 {
                let refused = built(&level(0.0), &wall);
                assert!(
                    matches!(refused, Err(SectionError::Unaccounted { .. })),
                    "{what}: refuses its chain at 1e-12: {refused:?}"
                );
                continue;
            }
            let built = built(&level(0.0), &wall).unwrap_or_else(|e| panic!("{what}: {e:?}"));
            let [node] = built.nodes() else {
                panic!("{what}: one node: {:?}", built.nodes());
            };
            let carried = built
                .carry(Some(SsiLane::marched()), band())
                .unwrap_or_else(|e| panic!("{what}: carries: {e:?}"));
            assert_eq!(carried.components().len(), 4, "{what}: four arcs");
            carried_to(&what, &carried, node.point, 1e-9 * s.max(1.0));
        }
    }
}

/// **A lemniscate a hundredth of a unit across is listed and carried**:
/// its node's two loops each carried to it at both ends. Red against the
/// node's fine pad reaching past `ssi::certify::NODE_PAD_REACH` of one
/// node-box half (sixteen): its tube narrows across most of the wall,
/// and the reading's accounting runs out of cells.
#[test]
fn a_lemniscate_a_hundredth_across_is_listed_and_carried() {
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &[1e-12],
        "the lemniscate's node",
        &["Pinched { predicate: \"spline_critical_unread\" }"],
        || {
            let s = 0.01;
            let built = built(&level(0.0), &scaled_lemniscate(s)).unwrap();
            let carried = built.carry(Some(SsiLane::marched()), band()).unwrap();
            assert_eq!(carried.components().len(), 2, "two loops");
            carried_to("the lemniscate", &carried, pt(0.0, 0.0, 0.0), 1e-11);
        },
    );
}

/// **A node's box is its settle reach** (C2's node end): on the cross
/// saddle `z = 4XY` (`φ`'s Hessian of norm 4 in metres per metre²) each
/// arc's node box reaches between 6 and 12 times `√ε` (the band's zero)
/// from the node, at every ε the suite runs: the march's settled
/// samples stop where `|∇φ|·d` falls to `ssi::ends::NODE_SETTLE_FLOOR`
/// tolerances, `d ≤ 4√ε`, and the box is `ssi::certify::NODE_BOX_REACH`
/// of the last one's reach.
///
/// Red against: the settle floor at 4 (the box at about 2√ε) or 4096
/// (the end span past the node's reach, no node read), the settles at 4
/// (the box at the march's entry, about 60√ε), and the box's reach
/// halved or doubled.
#[test]
fn a_node_box_is_its_settle_reach() {
    let s = built(&level(0.0), &cross()).unwrap();
    let carried = s.carry(Some(SsiLane::marched()), band()).unwrap();
    for (k, c) in carried.components().iter().enumerate() {
        let b = c
            .boxes
            .iter()
            .find(|b| b.node.is_some())
            .expect("a node box");
        let half = 0.5 * (b.hi[0] - b.lo[0]).max(b.hi[1] - b.lo[1]);
        let ratio = half / band().zero().sqrt();
        assert!(
            (6.0..=12.0).contains(&ratio),
            "arc {k}'s node box reaches {ratio}√ε"
        );
    }
}

/// **The node reading decides a node and nothing else** (C5, C2's node
/// end; [`crate::ssi::node::decide`]): over a box about the cross's
/// centre it reads a node; with the plane a micron off the node's level
/// the saddle point's value is past the band, and with it half the band
/// off, within it, the reading a node; over the bowl's top a touch; and
/// over a box a micron from the node holding no critical point, or one
/// whose edge holds the node, where the operator never proves the zero
/// alone, nothing.
///
/// Red against: the Hessian's determinant's sign dropped (the bowl reads
/// a node), the value's decision dropped (the plane a micron off reads a
/// node), and the Krawczyk proof dropped (the box with the node on its
/// edge reads a node).
#[test]
fn the_node_reading_decides_a_node_and_nothing_else() {
    use crate::ssi::exhaust::UvRect;
    use crate::ssi::node::{Jets, Kind, decide};
    use crate::ssi::residual::ImplicitForm;
    let about = |c: (f64, f64), h: f64| UvRect {
        u: (c.0 - h, c.0 + h),
        v: (c.1 - h, c.1 + h),
    };
    let bowl = separable(
        &[-0.5, 2.0, -2.0],
        (0.0, 1.0),
        &[-0.5, 2.0, -2.0],
        (0.0, 1.0),
    );
    let off = (0.500_02, 0.499_97);
    type Case = (&'static str, Surface<f64>, f64, UvRect, Option<Kind>);
    let cases: [Case; 6] = [
        ("the node", cross(), 0.0, about(off, 1e-4), Some(Kind::Node)),
        (
            "a micron off its level",
            cross(),
            1e-6,
            about(off, 1e-4),
            None,
        ),
        (
            "within the band of its level",
            cross(),
            0.5 * band().zero(),
            about(off, 1e-4),
            Some(Kind::Node),
        ),
        (
            "the bowl's top",
            bowl,
            0.0,
            about(off, 1e-4),
            Some(Kind::Touch),
        ),
        (
            "no critical point",
            cross(),
            0.0,
            about((0.5 + 1e-6, 0.5), 2e-7),
            None,
        ),
        (
            "the node on the box's edge, never proved alone",
            cross(),
            0.0,
            UvRect {
                u: (0.5, 0.5002),
                v: (0.4999, 0.5001),
            },
            None,
        ),
    ];
    for (what, wall, h, rect, want) in cases {
        let Surface::Nurbs(net) = &wall else {
            panic!("a spline wall")
        };
        let jets = Jets::of(net).expect("the wall lifts");
        let form = ImplicitForm::of(&level(h)).expect("a plane");
        let got = decide(&jets, &form, rect, band()).map(|d| d.kind);
        assert_eq!(got, want, "{what}");
    }
}

/// A hand-built branch over the flat wall `S(u, v) = (u, v, 0)`: its
/// chart chain along `v = ½` through `pts` (degree 1, on `knots`'s
/// interior), the carrier `carrier`, and the node box `node` at its end.
fn hand_built(
    pts: &[(f64, f64)],
    inner: &[f64],
    carrier: geom::NurbsCurve3<f64>,
    node: [Option<(f64, f64)>; 2],
) -> crate::ssi::SsiBranch {
    use crate::ssi::certify::SsiCertificate;
    use crate::ssi::{BranchEnd, SsiTube};
    use geom::{Curve3, NurbsCurve2};
    use geom_core::Point2;
    let mut k = vec![0.0, 0.0];
    k.extend_from_slice(inner);
    k.extend([1.0, 1.0]);
    let pcurve = NurbsCurve2::new(
        KnotVector::clamped(k, 1).unwrap(),
        pts.iter().map(|&(u, v)| Point2::new(u, v)).collect(),
        vec![1.0; pts.len()],
    )
    .unwrap();
    crate::ssi::SsiBranch {
        carrier: Curve3::Nurbs(std::sync::Arc::new(carrier)),
        params: (0.0, 1.0),
        end: BranchEnd::Closed,
        certificate: SsiCertificate {
            samples: 0,
            on_locus_max: 0.0,
            hull_sup: 0.0,
            tube: SsiTube::Chart {
                rung: 0.01,
                pad_u: 0.01,
                pad_v: 0.01,
                nodes: node,
            },
            tube_transversality: 1.0,
            tube_boxes: 3,
        },
        witness: pt(0.7, 0.5, 0.0),
        pcurve_a: None,
        pcurve_b: Some(pcurve),
        march_tol: 1e-9,
    }
}

/// The flat wall `S(u, v) = (u, v, 0)` and the node at its centre on
/// the chart's axes.
fn flat() -> (NurbsSurface<f64>, SectionNode<f64>) {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let net = NurbsSurface::new(
        k.clone(),
        k,
        vec![
            pt(0.0, 0.0, 0.0),
            pt(0.0, 1.0, 0.0),
            pt(1.0, 0.0, 0.0),
            pt(1.0, 1.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let node = SectionNode {
        point: pt(0.5, 0.5, 0.0),
        normal: Vec3::unit_z(),
        lines: [Vec3::unit_x(), Vec3::unit_y()],
        above: PairSide::First,
    };
    (net, node)
}

/// **An end's ray is its last box's sector, and its tangent must agree**
/// (C5; `ray_at`): a chain along `v = ½` from `u = 0.9` into the node's
/// box, its last box before it in the `+x` ray's sector. A carrier
/// coming in along `+x` names that ray; one whose end tangent leaves the
/// node along `+y` though its tube lies along `+x` refuses
/// `spline_node_ray`; and a chain of two windows, both node boxes, holds
/// no box of its own to read a sector from, and refuses as unread.
///
/// Red against: the end tangent's check dropped (the hooked carrier
/// names `+x`), and the window before the node's box read though it is
/// itself a node's (a two-box chain reads a sector off a node's box and
/// refuses as a ray, `spline_node_ray`).
#[test]
fn an_ends_ray_is_its_last_boxs_sector_and_its_tangent_agrees() {
    use geom::NurbsCurve3;
    let (net, node) = flat();
    let extent = geom_core::test_support::fixture_lever(geom_core::Interval::from_certified(1.0));
    let k = |p: usize| {
        let mut v = vec![0.0; p + 1];
        v.extend(vec![1.0; p + 1]);
        KnotVector::clamped(v, p).unwrap()
    };
    let chain = [(0.9, 0.5), (0.6, 0.5), (0.52, 0.5), (0.5, 0.5)];
    let inner = [1.0 / 3.0, 2.0 / 3.0];
    let straight = NurbsCurve3::new(
        k(1),
        vec![pt(0.9, 0.5, 0.0), pt(0.5, 0.5, 0.0)],
        vec![1.0; 2],
    )
    .unwrap();
    let hooked = NurbsCurve3::new(
        k(2),
        vec![pt(0.9, 0.5, 0.0), pt(0.5, 0.6, 0.0), pt(0.5, 0.5, 0.0)],
        vec![1.0; 3],
    )
    .unwrap();
    let box_at_end = [None, Some((0.04, 0.04))];
    let agree = super::ray_at(
        &hand_built(&chain, &inner, straight.clone(), box_at_end),
        &net,
        (1, &node),
        extent,
        band(),
    );
    assert_eq!(agree, Ok(0), "a carrier along +x names +x");
    let differ = super::ray_at(
        &hand_built(&chain, &inner, hooked, box_at_end),
        &net,
        (1, &node),
        extent,
        band(),
    );
    assert_eq!(
        differ,
        Err(SectionError::Pinched {
            predicate: super::NODE_RAY
        }),
        "a tangent leaving along +y with its tube along +x"
    );
    let both = super::ray_at(
        &hand_built(
            &[(0.5, 0.5), (0.52, 0.5), (0.5, 0.5)],
            &[0.5],
            straight,
            [Some((0.04, 0.04)), Some((0.04, 0.04))],
        ),
        &net,
        (1, &node),
        extent,
        band(),
    );
    assert!(
        matches!(both, Err(SectionError::Unaccounted { .. })),
        "two node boxes and no box between: {both:?}"
    );
}

/// **A witness is read in an arc's own window, never the node's box**
/// (`witness`): on the cross saddle, a chain along `v = ½` into the
/// node's box whose middle parameter falls in that box has no witness; the node's box holds the
/// node's four arcs, where a bracket could land on another.
///
/// Red against: the node's box read as the witness's window.
#[test]
fn a_witness_is_read_in_an_arcs_own_window() {
    use geom::NurbsCurve3;
    let wall = cross();
    let Surface::Nurbs(net) = &wall else {
        panic!("a spline wall")
    };
    let k1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let carrier =
        NurbsCurve3::new(k1, vec![pt(0.9, 0.5, 0.0), pt(0.5, 0.5, 0.0)], vec![1.0; 2]).unwrap();
    // The node's span holds the middle parameter.
    let b = hand_built(
        &[(0.9, 0.5), (0.6, 0.5), (0.5, 0.5)],
        &[0.25],
        carrier,
        [None, Some((0.2, 0.2))],
    );
    let plane = level(0.0);
    let form = crate::ssi::residual::ImplicitForm::of(&plane).unwrap();
    let got = super::witness(
        &b,
        net,
        super::Bracketed {
            surface: &plane,
            form: &form,
        },
    );
    assert!(got.is_none(), "no witness in the node's box: {got:?}");
}

/// **Two arcs of one node are held apart outside its box** (`apart`):
/// two arcs leaving a node at the origin along `+x` and nearly along it,
/// each read box by box. Where their boxes meet only inside the node's
/// box, both carry; where they meet again 0.9 from the node, both refuse
/// `PadsMeet`, as two arcs sharing no node do.
///
/// Red against: every pair of arcs sharing a node skipped whole (the
/// far meeting carried, where a site there reads on both and escalates
/// as if at the node).
#[test]
fn two_arcs_of_one_node_are_held_apart_outside_its_box() {
    use crate::pair_section::{
        CarriedComponent, CarryError, CarryRefusal, ClassCarrier, ComponentEnds, NodeEnd, NodeEnds,
        PadBox, SectionComponent,
    };
    let node_box = PadBox {
        lo: [-1e-3; 3],
        hi: [1e-3; 3],
        pad: 1e-3,
        node: Some(0),
    };
    let arc = |line, far: f64| {
        let ends = ComponentEnds {
            nodes: NodeEnds {
                start: Some(NodeEnd {
                    node: 0,
                    line,
                    sense: RaySense::Along,
                }),
                end: None,
            },
            ..ComponentEnds::default()
        };
        let shape = ComponentShape::Open { ends };
        let boxes = vec![
            node_box,
            PadBox {
                lo: [5e-4, -2e-4, -1e-4],
                hi: [0.5, far, 1e-4],
                pad: 1e-4,
                node: None,
            },
            PadBox {
                lo: [0.5, -1e-4, -1e-4],
                hi: [1.0, 1e-4, 1e-4],
                pad: 1e-4,
                node: None,
            },
        ];
        SectionComponent {
            shape,
            winds: [false, false],
            witness: None,
            pad: 0.0,
            carrier: ClassCarrier::Fitted(Box::new(Ok(CarriedComponent {
                carrier: geom::Curve3::line(pt(0.0, 0.0, 0.0), Vec3::unit_x()),
                span: (0.0, 1.0),
                shape,
                certificate: None,
                winds: [false, false],
                pad: 1e-4,
                reach: 0.0,
                witness: pt(0.5, 0.0, 0.0),
                boxes,
            }))),
            cut_ends: None,
        }
    };
    let carried =
        |c: &SectionComponent<f64>| matches!(&c.carrier, ClassCarrier::Fitted(f) if f.is_ok());
    // The second arc's boxes beside the first's: they meet at the node's
    // box, and again along the far box, 0.9 from the node.
    let mut meeting = vec![arc(NodeLine::First, 2e-4), arc(NodeLine::Second, 2e-4)];
    super::apart(&mut meeting, band());
    assert!(
        meeting.iter().all(|c| matches!(
            &c.carrier,
            ClassCarrier::Fitted(f) if matches!(f.as_ref(), Err(CarryError::Refused(CarryRefusal::PadsMeet { .. })))
        )),
        "arcs whose boxes meet far from their node refuse: {:?}",
        meeting.iter().map(carried).collect::<Vec<_>>()
    );
    // Moved off along `y` past the far box's reach, they meet only in
    // the node's box (each first box reaching into it).
    let mut apart_but_at_node = vec![arc(NodeLine::First, 2e-4), {
        let mut c = arc(NodeLine::Second, 2e-4);
        if let ClassCarrier::Fitted(f) = &mut c.carrier
            && let Ok(a) = f.as_mut()
        {
            for b in a.boxes.iter_mut().filter(|b| b.node.is_none()) {
                b.lo[1] += 0.01;
                b.hi[1] += 0.01;
            }
            a.boxes[1].lo = [5e-4, 5e-4, -1e-4];
        }
        c
    }];
    super::apart(&mut apart_but_at_node, band());
    assert!(
        apart_but_at_node.iter().all(carried),
        "arcs meeting only at their node's box carry"
    );
}
