//! JOIN-2 review lane r1's probes (PR 3880): tangent germs whose far
//! ends the reduction records alike or misleadingly, and two edges
//! between one vertex pair. Every pose is two prisms whose interiors
//! are disjoint, so the oracle is the operands' own volumes: A ∪ B =
//! vA + vB, A ∖ B = vA, B ∖ A = vB, A ∩ B empty. Each line is
//! `common::differential::outcome`; the row fails on any `BAD` or
//! wrong-empty line, never on a refusal.
//! `cargo test -p sweep --release --test all join2_r1 -- --ignored --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{Open, ProfileLoop, RawLoop, Start};
use sweep::test_support::{extruded, sketch_at};
use topo::{Body, BooleanOp};

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

/// tan(π/8): a quarter-circle's bulge.
const Q: f64 = 0.414_213_562_373_095_03;

fn prism(pts: &[(f64, f64, f64)], z0: f64, h: f64) -> Body<f64> {
    // Declare every joint where an arc meets a line tangentially (the
    // probes' arcs are all fillets or chords' bulges).
    let n = pts.len();
    let joints = (0..n)
        .filter(|&i| {
            let (prev, here) = (pts[(i + n - 1) % n].2, pts[i].2);
            (prev.abs() == Q) != (here.abs() == Q)
        })
        .collect();
    let chain = pts
        .iter()
        .map(|&(x, y, b)| (Point2::new(x, y), b))
        .collect();
    extruded(
        sketch_at(z0),
        vec![bulge_loop(chain).with_tangent_joints(joints)],
        h,
        tol(),
    )
}

fn rounded(w: f64, h: f64, r: f64) -> ProfileLoop<f64> {
    let t = tol();
    Open.at(Point2::new(w / 2.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w, h / 2.0), t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(w / 2.0, h), t)
        .unwrap()
        .toward(-1.0, 0.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .at(Point2::new(0.0, h / 2.0), t)
        .unwrap()
        .toward(0.0, -1.0, t)
        .unwrap()
        .fillet(r, t)
        .unwrap()
        .to(Start, t)
        .unwrap()
        .into()
}

fn ell_rounded(r: f64) -> ProfileLoop<f64> {
    let t = tol();
    let mut path = Open.at(Point2::new(3.0, 0.0)).toward(1.0, 0.0, t).unwrap();
    for (corner, (dx, dy)) in [
        (Point2::new(6.0, 1.5), (0.0, 1.0)),
        (Point2::new(4.5, 3.0), (-1.0, 0.0)),
        (Point2::new(3.0, 4.5), (0.0, 1.0)),
        (Point2::new(1.5, 6.0), (-1.0, 0.0)),
        (Point2::new(0.0, 3.0), (0.0, -1.0)),
    ] {
        path = path
            .fillet(r, t)
            .unwrap()
            .at(corner, t)
            .unwrap()
            .toward(dx, dy, t)
            .unwrap();
    }
    path.fillet(r, t).unwrap().to(Start, t).unwrap().into()
}

fn slab(outline: ProfileLoop<f64>, z0: f64) -> Body<f64> {
    extruded(sketch_at(z0), vec![outline], 1.0, tol())
}

fn vol(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, tol()).unwrap().volume
}

/// Every op in both orders; returns the lines.
fn run(label: &str, p: &Body<f64>, q: &Body<f64>) -> Vec<String> {
    let mut out = Vec::new();
    if std::env::var("R1_ONLY").is_ok_and(|o| o != label) {
        return out;
    }
    for (order, a, b) in [("PA", p, q), ("QA", q, p)] {
        let (va, vb) = (vol(a), vol(b));
        let dab = match topo::flush::find_flush_candidates(a, b, tol()) {
            Ok(f) => topo::flush::declare_all(&f),
            Err(e) => {
                out.push(format!("{label} {order} decls ERR {e:?}"));
                continue;
            }
        };
        let dba = match topo::flush::find_flush_candidates(b, a, tol()) {
            Ok(f) => topo::flush::declare_all(&f),
            Err(e) => {
                out.push(format!("{label} {order} decls ERR {e:?}"));
                continue;
            }
        };
        let join =
            match topo::test_support::boolean_join_refusal(BooleanOp::Union, a, b, &dab, tol()) {
                Ok(None) => "join=builds".to_string(),
                Ok(Some(e)) => format!(
                    "join=refuses:{}",
                    format!("{e:?}").chars().take(60).collect::<String>()
                ),
                Err(e) => format!(
                    "join=reduce-err:{}",
                    format!("{e:?}").chars().take(60).collect::<String>()
                ),
            };
        out.push(format!(
            "{label} {order} U {} [{join}]",
            outcome(topo::union_with(a, b, &dab, tol()), va + vb, tol())
        ));
        out.push(format!(
            "{label} {order} S_ab {}",
            outcome(topo::subtract_with(a, b, &dab, tol()), va, tol())
        ));
        out.push(format!(
            "{label} {order} S_ba {}",
            outcome(topo::subtract_with(b, a, &dba, tol()), vb, tol())
        ));
        out.push(format!(
            "{label} {order} I {}",
            outcome(topo::intersect_with(a, b, &dab, tol()), 0.0, tol())
        ));
    }
    out
}

fn check(lines: &[String]) {
    for l in lines {
        println!("{l}");
    }
    let bad: Vec<&String> = lines
        .iter()
        .filter(|l| l.contains(" BAD ") || l.contains("WRONG") || l.contains("OK-UNMEASURED"))
        .collect();
    assert!(bad.is_empty(), "wrong bodies: {bad:#?}");
}

/// The tangency row's poses at radii toward 0 and toward the plate
/// size, and mismatched fillets near equality.
#[test]
#[ignore = "review probe"]
fn join2_r1_tangent_radii() {
    let (w, h) = (6.0, 4.0);
    let sharp = || {
        ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(w, 0.0),
            Point2::new(w, h),
            Point2::new(0.0, h),
        ])
    };
    let ell = || {
        ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(6.0, 0.0),
            Point2::new(6.0, 3.0),
            Point2::new(3.0, 3.0),
            Point2::new(3.0, 6.0),
            Point2::new(0.0, 6.0),
        ])
    };
    let mut lines = Vec::new();
    for r in [0.01, 0.05, 1.5, 1.9, 1.99] {
        lines.extend(run(
            &format!("plate r={r}"),
            &slab(sharp(), 1.0),
            &slab(rounded(w, h, r), 0.0),
        ));
        lines.extend(run(
            &format!("plate-under r={r}"),
            &slab(sharp(), -1.0),
            &slab(rounded(w, h, r), 0.0),
        ));
    }
    for r in [0.01, 0.05, 1.2, 1.49] {
        lines.extend(run(
            &format!("L r={r}"),
            &slab(ell(), 1.0),
            &slab(ell_rounded(r), 0.0),
        ));
    }
    for (u, l) in [(0.5, 0.49), (0.49, 0.5), (0.5, 0.5), (1.9, 0.1)] {
        lines.extend(run(
            &format!("rr {u}/{l}"),
            &slab(rounded(w, h, u), 1.0),
            &slab(rounded(w, h, l), 0.0),
        ));
    }
    check(&lines);
}

/// **The filed residue**: two tangent edges parting with like far-end
/// touches. The lower plate's south side has a convex fillet at
/// (a, 0) into a notch whose east wall comes back to y = 0 at a + 3;
/// the upper plate's straight south edge spans the notch, so its
/// fragment past the tangent point ends at a recorded vertex pair
/// (Boundary), and the upper's outline passes through the fillet's
/// far end (a + r, r), so the arc's far end is a vertex pair too.
#[test]
#[ignore = "review probe"]
fn join2_r1_like_far_ends() {
    let mut lines = Vec::new();
    for (a, r) in [(3.0, 0.5), (3.0, 1.0), (2.0, 0.25)] {
        let lower = prism(
            &[
                (0.0, 0.0, 0.0),
                (a, 0.0, Q),
                (a + r, r, 0.0),
                (a + r, 2.0, 0.0),
                (a + 3.0, 2.0, 0.0),
                (a + 3.0, 0.0, 0.0),
                (a + 5.0, 0.0, 0.0),
                (a + 5.0, 4.0, 0.0),
                (0.0, 4.0, 0.0),
            ],
            0.0,
            1.0,
        );
        // V1: a plain rectangle over the notch (arc far end on the
        // upper's face, line fragment's far end a vertex pair).
        let rect = prism(
            &[
                (0.0, 0.0, 0.0),
                (a + 5.0, 0.0, 0.0),
                (a + 5.0, 4.0, 0.0),
                (0.0, 4.0, 0.0),
            ],
            1.0,
            1.0,
        );
        lines.extend(run(&format!("rect a={a} r={r}"), &rect, &lower));
        // V2: a V-notch cut from the upper's north side with its tip on
        // the fillet's far end: both far ends are vertex pairs.
        let vee = prism(
            &[
                (0.0, 0.0, 0.0),
                (a + 5.0, 0.0, 0.0),
                (a + 5.0, 4.0, 0.0),
                (a + r + 1.0, 4.0, 0.0),
                (a + r, r, 0.0),
                (a + r - 1.0, 4.0, 0.0),
                (0.0, 4.0, 0.0),
            ],
            1.0,
            1.0,
        );
        lines.extend(run(&format!("vee a={a} r={r}"), &vee, &lower));
        // V3: the upper an L whose reflex corner is the fillet's far end.
        let ell = prism(
            &[
                (0.0, 0.0, 0.0),
                (a + 5.0, 0.0, 0.0),
                (a + 5.0, 4.0, 0.0),
                (a + r, 4.0, 0.0),
                (a + r, r, 0.0),
                (0.0, r, 0.0),
            ],
            1.0,
            1.0,
        );
        lines.extend(run(&format!("ell a={a} r={r}"), &ell, &lower));
    }
    check(&lines);
}

/// **A tangent germ at a vertex-on-face site whose edge's far end
/// touches the partner past a gap**: a block with its south-west
/// vertical edge filleted, set in the inside corner of an L, its
/// south face on the floor and its west face on the wall. The arc
/// leaves the floor tangentially and its far end lies on the wall.
#[test]
#[ignore = "review probe"]
fn join2_r1_cove_corner() {
    let ell = prism(
        &[
            (-1.0, -1.0, 0.0),
            (6.0, -1.0, 0.0),
            (6.0, 0.0, 0.0),
            (0.0, 0.0, 0.0),
            (0.0, 6.0, 0.0),
            (-1.0, 6.0, 0.0),
        ],
        -1.0,
        3.0,
    );
    let mut lines = Vec::new();
    for (r, gap) in [(0.5, 0.0), (0.01, 0.0), (2.9, 0.0), (0.5, 0.25)] {
        let block = prism(
            &[
                (gap + r, 0.0, 0.0),
                (gap + 4.0, 0.0, 0.0),
                (gap + 4.0, 3.0, 0.0),
                (gap, 3.0, 0.0),
                (gap, r, Q),
            ],
            0.0,
            1.0,
        );
        lines.extend(run(&format!("cove r={r} gap={gap}"), &block, &ell));
        // The block flush with the L's top and bottom too.
        let flush = prism(
            &[
                (gap + r, 0.0, 0.0),
                (gap + 4.0, 0.0, 0.0),
                (gap + 4.0, 3.0, 0.0),
                (gap, 3.0, 0.0),
                (gap, r, Q),
            ],
            -1.0,
            3.0,
        );
        lines.extend(run(&format!("cove-flush r={r} gap={gap}"), &flush, &ell));
    }
    check(&lines);
}

/// **Two edges between one vertex pair**: the lower plate's south
/// side carries an arc from (1, 0) to (5, 0) (bulging out or in), the
/// upper's a straight edge through the same two points, or an arc
/// bulging the other way (a lens between the two).
#[test]
#[ignore = "review probe"]
fn join2_r1_two_edges_one_pair() {
    let mut lines = Vec::new();
    let outline = |bulge: f64| {
        vec![
            (0.0, 0.0, 0.0),
            (1.0, 0.0, bulge),
            (5.0, 0.0, 0.0),
            (6.0, 0.0, 0.0),
            (6.0, 4.0, 0.0),
            (0.0, 4.0, 0.0),
        ]
    };
    for (lb, ub) in [
        (0.3, 0.0),
        (-0.3, 0.0),
        (0.3, -0.3),
        (-0.3, 0.3),
        (0.3, 0.3),
        (0.41, 0.0),
        (-0.41, -0.2),
    ] {
        let lower = prism(&outline(lb), 0.0, 1.0);
        let upper = prism(&outline(ub), 1.0, 1.0);
        lines.extend(run(&format!("pair lower={lb} upper={ub}"), &upper, &lower));
    }
    check(&lines);
}

/// **A grid battery of tangent stacks**, shaped unlike the PR's: a
/// rounded lower plate (or the rounded L) under an axis-aligned upper
/// rectangle whose sides are drawn from the lower's special abscissae
/// and ordinates — corners, tangent points, arc midpoints, outside —
/// so its edges pass the tangent points, end on them, or miss them.
/// Prints one `outcome` line per op; fails on any wrong body.
#[test]
#[ignore = "review battery"]
fn join2_r1_grid() {
    let (w, h) = (6.0, 4.0);
    let mut lines = Vec::new();
    for r in [0.5, 1.0] {
        let m = r * (1.0 - core::f64::consts::FRAC_1_SQRT_2);
        let xs = [-1.0, 0.0, m, r, 3.0, w - r, w - m, w, w + 1.0];
        let ys = [-1.0, 0.0, m, r, 2.0, h - r, h - m, h, h + 1.0];
        let lower = slab(rounded(w, h, r), 0.0);
        for (i, &x0) in xs.iter().enumerate() {
            for &x1 in &xs[i + 1..] {
                for (j, &y0) in ys.iter().enumerate() {
                    for &y1 in &ys[j + 1..] {
                        let upper = prism(
                            &[(x0, y0, 0.0), (x1, y0, 0.0), (x1, y1, 0.0), (x0, y1, 0.0)],
                            1.0,
                            1.0,
                        );
                        lines.extend(run(
                            &format!("grid r={r} x=[{x0},{x1}] y=[{y0},{y1}]"),
                            &upper,
                            &lower,
                        ));
                    }
                }
            }
        }
    }
    for r in [0.5] {
        let m = r * (1.0 - core::f64::consts::FRAC_1_SQRT_2);
        let xs = [-1.0, 0.0, r, 3.0 - r, 3.0, 3.0 + m, 3.0 + r, 6.0 - r, 6.0];
        let lower = slab(ell_rounded(r), 0.0);
        for (i, &x0) in xs.iter().enumerate() {
            for &x1 in &xs[i + 1..] {
                for (j, &y0) in xs.iter().enumerate() {
                    for &y1 in &xs[j + 1..] {
                        let upper = prism(
                            &[(x0, y0, 0.0), (x1, y0, 0.0), (x1, y1, 0.0), (x0, y1, 0.0)],
                            1.0,
                            1.0,
                        );
                        lines.extend(run(
                            &format!("gridL r={r} x=[{x0},{x1}] y=[{y0},{y1}]"),
                            &upper,
                            &lower,
                        ));
                    }
                }
            }
        }
    }
    for l in &lines {
        println!("{l}");
    }
    let bad = lines
        .iter()
        .filter(|l| l.contains(" BAD ") || l.contains("WRONG") || l.contains("OK-UNMEASURED"))
        .count();
    assert_eq!(bad, 0, "wrong bodies");
}

/// **The unions main (`66bbdaa6bd`) builds sound and PR 3880's head
/// refuses**: sixteen grid poses the zip's ring-first order refuses
/// `ChordBetweenIsolatedPierces` (it takes a segment with BOTH ends at
/// unjoined pierce-ring vertices first), and the notched plate under an
/// L, which reaches the filed like-far-ends residue and is left loose
/// by the join (`UnpairedLooseEnds`) where main's own enumeration
/// zipped it in this operand order. Red at the head reviewed; each pose
/// is `(lower r, x0, x1, y0, y1, order)` with the lower plate A when
/// `order` is "QA".
#[test]
#[ignore = "review row: red at 17c254c99d"]
fn join2_r1_unions_main_builds_sound() {
    let (w, h) = (6.0, 4.0);
    let (m5, m1) = (
        0.5 * (1.0 - core::f64::consts::FRAC_1_SQRT_2),
        1.0 - core::f64::consts::FRAC_1_SQRT_2,
    );
    let plate_poses = [
        (0.5, 0.0, 0.5, m5, 0.5, "PA"),
        (0.5, 0.0, 0.5, m5, 0.5, "QA"),
        (0.5, m5, 0.5, 3.5, 4.0, "PA"),
        (0.5, 5.5, w - m5, 0.0, 0.5, "PA"),
        (1.0, 0.0, 1.0, m1, 1.0, "PA"),
        (1.0, 0.0, 1.0, m1, 1.0, "QA"),
        (1.0, 0.0, 3.0, m1, 3.0, "PA"),
        (1.0, 0.0, 3.0, m1, 3.0, "QA"),
        (1.0, m1, 1.0, 3.0, 4.0, "PA"),
        (1.0, m1, 3.0, 1.0, 4.0, "PA"),
        (1.0, m1, 3.0, 1.0, 4.0, "QA"),
        (1.0, 3.0, w - m1, 0.0, 3.0, "PA"),
        (1.0, 3.0, w - m1, 0.0, 3.0, "QA"),
        (1.0, 5.0, w - m1, 0.0, 1.0, "PA"),
    ];
    let mut failed = Vec::new();
    let mut one = |label: String, upper: Body<f64>, lower: Body<f64>, order: &str| {
        let (a, b) = if order == "PA" {
            (&upper, &lower)
        } else {
            (&lower, &upper)
        };
        let d = topo::test_support::flush_declarations(a, b, tol());
        let line = outcome(topo::union_with(a, b, &d, tol()), vol(a) + vol(b), tol());
        println!("{label} {order} U {line}");
        if !line.starts_with("OK SOUND") {
            failed.push(format!("{label} {order}: {line}"));
        }
    };
    for (r, x0, x1, y0, y1, order) in plate_poses {
        let upper = prism(
            &[(x0, y0, 0.0), (x1, y0, 0.0), (x1, y1, 0.0), (x0, y1, 0.0)],
            1.0,
            1.0,
        );
        one(
            format!("grid r={r} x=[{x0},{x1}] y=[{y0},{y1}]"),
            upper,
            slab(rounded(w, h, r), 0.0),
            order,
        );
    }
    for order in ["PA", "QA"] {
        let upper = prism(
            &[
                (3.0, 0.5, 0.0),
                (5.5, 0.5, 0.0),
                (5.5, 3.0, 0.0),
                (3.0, 3.0, 0.0),
            ],
            1.0,
            1.0,
        );
        one(
            "gridL r=0.5 x=[3,5.5] y=[0.5,3]".into(),
            upper,
            slab(ell_rounded(0.5), 0.0),
            order,
        );
    }
    for (a, r) in [(3.0, 0.5), (3.0, 1.0), (2.0, 0.25)] {
        let lower = prism(
            &[
                (0.0, 0.0, 0.0),
                (a, 0.0, Q),
                (a + r, r, 0.0),
                (a + r, 2.0, 0.0),
                (a + 3.0, 2.0, 0.0),
                (a + 3.0, 0.0, 0.0),
                (a + 5.0, 0.0, 0.0),
                (a + 5.0, 4.0, 0.0),
                (0.0, 4.0, 0.0),
            ],
            0.0,
            1.0,
        );
        let ell = prism(
            &[
                (0.0, 0.0, 0.0),
                (a + 5.0, 0.0, 0.0),
                (a + 5.0, 4.0, 0.0),
                (a + r, 4.0, 0.0),
                (a + r, r, 0.0),
                (0.0, r, 0.0),
            ],
            1.0,
            1.0,
        );
        one(format!("ell a={a} r={r}"), ell, lower, "QA");
    }
    assert!(failed.is_empty(), "main builds these sound: {failed:#?}");
}
