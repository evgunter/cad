//! REVIEW (PR 6226, `splitdoor/rims-seam-orientation`): an oracle for
//! the rims-on-tangent-walls bodies that shares no code with
//! `sheets_part_on_slanted_rims`. Each op's volume is held to a grid
//! integral of the analytic solids, not to the closed form, and
//! `point_in_solid` to the analytic membership on a lattice and at points
//! in the shared lens and by the pinch. It runs on the nine tilt pairs of
//! `rims_crossing_on_tangent_walls_build_every_op` and on each pair's
//! mirror in `y`, `(−t_a, −t_b)`, which reverses the handedness.
//!
//! Run: `cargo nextest run -p sweep --run-ignored all review_6226`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;
use topo::boolean::{BooleanError, BooleanOp, BooleanResult, SweepStrategy, boolean_op_with};

const H: f64 = 3.0;

fn tol() -> Tol {
    Tol::witness()
}

fn rod(r: f64, cx: f64) -> Body<f64> {
    let rim = RawLoop::new((0..2).map(|k| {
        let at = FRAC_PI_2 + PI * f64::from(k);
        (
            Point2::new(cx + r * at.cos(), r * at.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(cx, 0.0),
                radius: r,
                sweep: PI,
            }),
        )
    }));
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![rim])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 2.0 * H,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    let lift = Affine3::translation(Vec3::new(0.0, 0.0, -H));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&lift, tol()), tol()).unwrap()
}

fn op(o: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Result<BooleanResult<f64>, BooleanError> {
    let a = topo::test_support::finished("a", a.clone(), tol());
    let b = topo::test_support::finished("b", b.clone(), tol());
    boolean_op_with(
        o,
        &a,
        &b,
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        tol(),
    )
}

fn cut(body: &Body<f64>, tilt: f64, below: bool) -> Body<f64> {
    let z = if below { (-10.0, 0.0) } else { (0.0, 10.0) };
    let brick = sweep::test_support::brick((-10.0, 10.0), (-10.0, 10.0), z, tol());
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), tilt);
    let half = topo::transform_rigid(&brick, &geom_core::test_support::rigid(&turn, tol()), tol())
        .unwrap();
    match op(BooleanOp::Intersect, body, &half) {
        Ok(BooleanResult::Body(b)) => b.body.into_body(),
        other => panic!("cutting a rod at tilt {tilt}: {other:?}"),
    }
}

/// The analytic solids, and each point's clearance from every surface
/// that bounds either.
struct Rims {
    ta: f64,
    tb: f64,
}

impl Rims {
    fn in_a(&self, p: [f64; 3]) -> bool {
        let [x, y, z] = p;
        (x - 1.0).powi(2) + y * y < 1.0 && z > -H && z < y * self.ta.tan()
    }
    fn in_b(&self, p: [f64; 3]) -> bool {
        let [x, y, z] = p;
        x * x + y * y < 4.0 && z < H && z > y * self.tb.tan()
    }
    fn clearance(&self, p: [f64; 3]) -> f64 {
        let [x, y, z] = p;
        let plane = |t: f64| (z - y * t.tan()).abs() * t.cos();
        [
            (((x - 1.0).powi(2) + y * y).sqrt() - 1.0).abs(),
            ((x * x + y * y).sqrt() - 2.0).abs(),
            (z - H).abs(),
            (z + H).abs(),
            plane(self.ta),
            plane(self.tb),
        ]
        .into_iter()
        .fold(f64::INFINITY, f64::min)
    }
    fn member(&self, name: &str, p: [f64; 3]) -> bool {
        let (a, b) = (self.in_a(p), self.in_b(p));
        match name {
            "A−B" => a && !b,
            "A∩B" => a && b,
            "A∪B" => a || b,
            "B−A" => b && !a,
            _ => unreachable!(),
        }
    }
    /// The volume of `name` by a midpoint grid over the `(x, y)` square,
    /// each column's `z` length in closed form.
    fn grid_volume(&self, name: &str) -> f64 {
        let n = 1600;
        let h = 4.0 / f64::from(n);
        let (sa, sb) = (self.ta.tan(), self.tb.tan());
        let mut v = 0.0;
        for i in 0..n {
            let x = -2.0 + (f64::from(i) + 0.5) * h;
            for j in 0..n {
                let y = -2.0 + (f64::from(j) + 0.5) * h;
                let ia = (x - 1.0).powi(2) + y * y < 1.0;
                let ib = x * x + y * y < 4.0;
                // A: z in (−H, min(H, y·sa)); B: z in (max(−H, y·sb), H).
                let a = if ia {
                    (-H, (y * sa).min(H))
                } else {
                    (0.0, 0.0)
                };
                let b = if ib {
                    ((y * sb).max(-H), H)
                } else {
                    (0.0, 0.0)
                };
                let len = |(lo, hi): (f64, f64)| (hi - lo).max(0.0);
                let both = len((a.0.max(b.0), a.1.min(b.1)));
                let (la, lb) = (len(a), len(b));
                v += match name {
                    "A−B" => la - both,
                    "A∩B" => both,
                    "A∪B" => la + lb - both,
                    "B−A" => lb - both,
                    _ => unreachable!(),
                };
            }
        }
        v * h * h
    }
}

fn probes(r: &Rims) -> Vec<[f64; 3]> {
    let mut out = Vec::new();
    let (nx, ny, nz) = (9, 9, 9);
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                let f = |m: i32, n: i32, lo: f64, hi: f64, off: f64| {
                    lo + (hi - lo) * (f64::from(m) + off) / f64::from(n)
                };
                out.push([
                    f(i, nx, -2.3, 2.3, 0.37),
                    f(j, ny, -2.3, 2.3, 0.61),
                    f(k, nz, -3.3, 3.3, 0.53),
                ]);
            }
        }
    }
    // The shared lens and the pinch: between the two planes, on the side
    // where they part, and just outside them.
    let (sa, sb) = (r.ta.tan(), r.tb.tan());
    let s = if sa > sb { 1.0 } else { -1.0 };
    for &(x, yy) in &[
        (1.0, 0.5),
        (1.0, 0.9),
        (1.5, 0.4),
        (1.9, 0.2),
        (1.97, 0.1),
        (0.3, 0.6),
    ] {
        let y = s * yy;
        let (lo, hi) = ((y * sb).min(y * sa), (y * sb).max(y * sa));
        for w in [-0.3, 0.1, 0.5, 0.9, 1.3] {
            out.push([x, y, lo + w * (hi - lo)]);
        }
        // The other side of the ruling, where the planes cross over.
        let y = -y;
        out.push([x, y, 0.5 * (y * sa + y * sb)]);
    }
    out
}

fn check(tilts: (f64, f64), fails: &mut Vec<String>) {
    let r = Rims {
        ta: tilts.0,
        tb: tilts.1,
    };
    let a = cut(&rod(1.0, 1.0), r.ta, true);
    let b = cut(&rod(2.0, 0.0), r.tb, false);
    let shared = 2.0 / 3.0 * (r.ta.tan() - r.tb.tan()).abs();
    let (va, vb) = (3.0 * PI, 12.0 * PI);
    let band = geom_core::Band::linear(tol()).unwrap();
    let ps = probes(&r);
    for (o, name, x, y, closed) in [
        (BooleanOp::Subtract, "A−B", &a, &b, va - shared),
        (BooleanOp::Intersect, "A∩B", &a, &b, shared),
        (BooleanOp::Union, "A∪B", &a, &b, va + vb - shared),
        (BooleanOp::Subtract, "B−A", &b, &a, vb - shared),
    ] {
        let what = format!("tilts {tilts:?} {name}");
        let grid = r.grid_volume(name);
        if (grid - closed).abs() > 2e-3 * closed.max(1.0) {
            fails.push(format!("{what}: grid {grid} vs closed form {closed}"));
        }
        let res = match op(o, x, y) {
            Ok(BooleanResult::Body(res)) => res,
            other => {
                fails.push(format!("{what}: {other:?}"));
                continue;
            }
        };
        let body = &res.body;
        if let Err(e) = topo::validate_geometric(body, tol()) {
            fails.push(format!("{what}: tier 3: {e:?}"));
        }
        let m = topo::test_support::pinned_volume(&what, body, tol());
        if (m.volume - grid).abs() > 2e-3 * grid.max(1.0) {
            fails.push(format!("{what}: volume {} vs grid {grid}", m.volume));
        }
        let (mut read, mut skipped) = (0, 0);
        for p in &ps {
            if r.clearance(*p) < 1e-3 {
                continue;
            }
            let want = r.member(name, *p);
            let q = Point3::new(p[0], p[1], p[2]);
            match topo::point_in_solid(body, q, band, tol()) {
                Ok(topo::SolidContainment::In) if want => read += 1,
                Ok(topo::SolidContainment::Out) if !want => read += 1,
                Ok(got) => fails.push(format!("{what}: {p:?} reads {got:?}, wants in={want}")),
                Err(e) => {
                    skipped += 1;
                    if skipped <= 3 {
                        fails.push(format!("{what}: {p:?} unread: {e:?}"));
                    }
                }
            }
        }
        eprintln!(
            "{what}: volume {} grid {grid}; {read} probes agree",
            m.volume
        );
    }
}

#[test]
#[ignore = "review oracle, ~minutes; run with --run-ignored"]
fn review_6226_every_op_both_handednesses_against_the_analytic_solids() {
    let nine = [
        (0.0, 0.3),
        (0.0, 1.0),
        (0.2, 0.5),
        (-0.3, 0.6),
        (0.45, 0.5),
        (0.0, -0.5),
        (0.3, -0.4),
        (0.5, 0.2),
        (0.5, 0.45),
    ];
    let mut fails = Vec::new();
    for (ta, tb) in nine {
        check((ta, tb), &mut fails);
        check((-ta, -tb), &mut fails);
    }
    assert!(
        fails.is_empty(),
        "{} failures:\n{}",
        fails.len(),
        fails.join("\n")
    );
}
