//! Review probe for PR 7264, not for merge: prints one validation verdict
//! per case of a fixed corpus and then fails, so hosted CI shows the lines.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Decide, Interval, Point2, Real, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};

fn verdict<T: Decide>(loops: Vec<ProfileLoop<T>>) -> String {
    match Profile::new(SketchPlane::xy(), loops).validate(Tol::witness()) {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("{e:?}").chars().take(230).collect(),
    }
}

/// Prints each line as it is made, so a panic part-way loses nothing.
struct Out(usize);

impl Out {
    fn push(&mut self, line: String) {
        println!("P7264|{line}");
        self.0 += 1;
    }
}

struct Lcg(u64);

impl Lcg {
    fn next(&mut self, m: u64) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) % m
    }
}

fn poly_f(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)))
}

fn poly_i(pts: &[(f64, f64)], h: f64) -> ProfileLoop<Interval> {
    ProfileLoop::polygon(pts.iter().map(|&(x, y)| {
        Point2::new(
            Interval::from_bounds(x - h, x + h),
            Interval::from_bounds(y - h, y + h),
        )
    }))
}

fn boxed(pts: &[(f64, f64, f64)]) -> ProfileLoop<Interval> {
    ProfileLoop::polygon(pts.iter().map(|&(x, y, h)| {
        Point2::new(
            Interval::from_bounds(x - h, x + h),
            Interval::from_bounds(y - h, y + h),
        )
    }))
}

#[test]
fn zz_rev_7264_probe_prints_its_verdicts() {
    let mut out = Out(0);

    // G: grid polygons, scaled and offset; f64, point boxes, widened boxes.
    let scales = [1.0, 1e-3, 1e4, 1e7, 1e10];
    let mut rng = Lcg(7264);
    for i in 0..2500usize {
        let n = 3 + rng.next(4) as usize;
        let s = scales[i % 5];
        let (ox, oy) = if (i / 5) % 2 == 0 {
            (0.0, 0.0)
        } else {
            (s * 0.1, s * 0.7)
        };
        let pts: Vec<(f64, f64)> = (0..n)
            .map(|_| {
                let (gx, gy) = (rng.next(5) as f64, rng.next(5) as f64);
                (gx * s + ox, gy * s + oy)
            })
            .collect();
        out.push(format!("G|{i}|f64|{}", verdict(vec![poly_f(&pts)])));
        out.push(format!("G|{i}|iv0|{}", verdict(vec![poly_i(&pts, 0.0)])));
        if i < 750 {
            for h in [1e-12, 1e-9, 1e-6, 1e-3] {
                out.push(format!(
                    "G|{i}|iv{h:e}|{}",
                    verdict(vec![poly_i(&pts, h * s)])
                ));
            }
        }
    }

    // N: a short leg leaving a long one at a small angle, forward or
    // doubled back, its far end `h` off the long leg's carrier.
    let mut k = 0usize;
    for long in [1.0, 100.0, 1e5] {
        for short_frac in [1e-3, 0.1, 0.9, 1.0, 1.5] {
            for h in [
                3e-10, 9e-10, 1.1e-9, 2e-9, 5e-9, 9e-9, 1.1e-8, 3e-8, 1e-7, 1e-6,
            ] {
                for back in [false, true] {
                    for far in [0.0, 1e5] {
                        let short: f64 = long * short_frac;
                        let run = (short * short - h * h).sqrt();
                        let a = (far, far);
                        let b = (far + long, far);
                        let c = if back {
                            (far + long - run, far + h)
                        } else {
                            (far + long + run, far + h)
                        };
                        let d = (far + 0.5 * long, far + long);
                        for (name, pts) in [
                            ("abc", vec![a, b, c]),
                            ("bca", vec![b, c, a]),
                            ("cab", vec![c, a, b]),
                            ("cba", vec![c, b, a]),
                            ("abcd", vec![a, b, c, d]),
                            ("dabc", vec![d, a, b, c]),
                            ("cdab", vec![c, d, a, b]),
                        ] {
                            out.push(format!("N|{k}|{name}|f64|{}", verdict(vec![poly_f(&pts)])));
                            out.push(format!(
                                "N|{k}|{name}|iv0|{}",
                                verdict(vec![poly_i(&pts, 0.0)])
                            ));
                        }
                        k += 1;
                    }
                }
            }
        }
    }

    // A: grid chains with arcs, so the curved arms are in the differential.
    let bulges = [0.0, 0.0, 0.0, 0.3, -0.3, 1.0];
    let mut rng = Lcg(72640);
    for i in 0..800usize {
        let n = 2 + rng.next(4) as usize;
        let chain: Vec<(Point2<f64>, f64)> = (0..n)
            .map(|_| {
                let (gx, gy) = (rng.next(5) as f64, rng.next(5) as f64);
                (Point2::new(gx, gy), bulges[rng.next(6) as usize])
            })
            .collect();
        let lifted: Vec<(Point2<Interval>, Interval)> = chain
            .iter()
            .map(|&(p, b)| {
                (
                    Point2::new(
                        <Interval as Real>::from_f64(p.x),
                        <Interval as Real>::from_f64(p.y),
                    ),
                    <Interval as Real>::from_f64(b),
                )
            })
            .collect();
        out.push(format!("A|{i}|f64|{}", verdict(vec![bulge_loop(chain)])));
        out.push(format!("A|{i}|iv0|{}", verdict(vec![bulge_loop(lifted)])));
    }

    // S: named shapes over a box of half-width `h` at Interval.
    for h in [0.0, 1e-12, 1e-9, 1e-6, 1e-3, 0.05] {
        let z = 0.0;
        let shapes: Vec<(&str, Vec<(f64, f64, f64)>)> = vec![
            (
                "rect_boxed_corner",
                vec![(0.0, 0.0, z), (1.0, 0.0, z), (1.0, 1.0, h), (0.0, 1.0, z)],
            ),
            (
                "rect_all_boxed",
                vec![(0.0, 0.0, h), (1.0, 0.0, h), (1.0, 1.0, h), (0.0, 1.0, h)],
            ),
            (
                "straight_joint_boxed_mid",
                vec![
                    (0.0, 0.0, z),
                    (0.5, 0.0, h),
                    (1.0, 0.0, z),
                    (1.0, 1.0, z),
                    (0.0, 1.0, z),
                ],
            ),
            (
                "straight_joint_exact_mid_boxed_ends",
                vec![
                    (0.0, 0.0, h),
                    (0.5, 0.0, z),
                    (1.0, 0.0, h),
                    (1.0, 1.0, z),
                    (0.0, 1.0, z),
                ],
            ),
            (
                "spike_doubled_back_boxed_tip",
                vec![(0.0, 0.0, z), (2.0, 0.0, h), (1.0, 0.0, z), (1.0, 1.0, z)],
            ),
            (
                "spike_doubled_back_boxed_return",
                vec![(0.0, 0.0, z), (2.0, 0.0, z), (1.0, 0.0, h), (1.0, 1.0, z)],
            ),
            (
                "spike_closing_pair",
                vec![(1.0, 0.0, z), (1.0, 1.0, z), (0.0, 0.0, z), (2.0, 0.0, h)],
            ),
            (
                "spike_past_start_closing_pair",
                vec![(0.0, 0.0, h), (2.0, 0.0, z), (2.0, 1.0, z), (-1.0, 0.0, z)],
            ),
            (
                "acute_triangle",
                vec![(0.0, 0.0, z), (4.0, 0.0, h), (0.0, 1.0, z)],
            ),
            (
                "acute_triangle_all_boxed",
                vec![(0.0, 0.0, h), (4.0, 0.0, h), (0.0, 1.0, h)],
            ),
            (
                "bowtie",
                vec![(0.0, 0.0, h), (1.0, 1.0, h), (1.0, 0.0, h), (0.0, 1.0, h)],
            ),
            (
                "vertex_on_far_edge",
                vec![
                    (0.0, 0.0, z),
                    (2.0, 0.0, z),
                    (2.0, 2.0, z),
                    (1.0, 0.0, h),
                    (0.0, 2.0, z),
                ],
            ),
            (
                "degenerate_triangle_collinear",
                vec![(0.0, 0.0, h), (1.0, 0.0, h), (2.0, 0.0, h)],
            ),
            ("two_lines", vec![(0.0, 0.0, h), (1.0, 0.0, h)]),
            ("one_point", vec![(0.0, 0.0, h)]),
            ("needle", vec![(0.0, 0.0, z), (1.0, 0.0, h), (0.0, 1e-7, z)]),
            (
                "far_rect",
                vec![
                    (1e6, 1e6, h),
                    (1e6 + 1.0, 1e6, h),
                    (1e6 + 1.0, 1e6 + 1.0, h),
                    (1e6, 1e6 + 1.0, h),
                ],
            ),
        ];
        for (name, pts) in shapes {
            out.push(format!("S|{name}|{h:e}|{}", verdict(vec![boxed(&pts)])));
        }
        let two_seg_arc = bulge_loop(vec![
            (
                Point2::new(
                    Interval::from_bounds(-h, h),
                    <Interval as Real>::from_f64(0.0),
                ),
                <Interval as Real>::from_f64(0.0),
            ),
            (
                Point2::new(
                    Interval::from_bounds(1.0 - h, 1.0 + h),
                    <Interval as Real>::from_f64(0.0),
                ),
                <Interval as Real>::from_f64(1.0),
            ),
        ]);
        out.push(format!("S|half_disc|{h:e}|{}", verdict(vec![two_seg_arc])));
    }

    panic!(
        "probe, not for merge: {} verdict lines printed above",
        out.0
    );
}
