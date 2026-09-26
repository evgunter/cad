//! **A certified torus root and its consumer agree, or the consumer
//! refuses.**
//!
//! `line_torus_roots` certifies a root COUNT, not a root's accuracy to
//! the band: at a near-perpendicular pose the constructed root can land
//! a hair off the tube. The crossing layer places every landing point
//! with [`super::curved_face_placement`], whose carrier test runs
//! before any chart work, so a torus face on a skeletal body is enough
//! to reach it. The row holds the contract `wall_crossing` rests on:
//! for every certified root, either the landing point's elevation off
//! the tube decides `Zero`, or the placement answers `OffCarrier` —
//! which the crossing layer turns into its typed door, never into
//! "outside this face's trim".

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::{Band, Margin, Point3, Sign, Tol, Vec3};

use super::{CurvedPlacement, curved_face_placement};
use crate::boolean::solid_contain::{TorusRoots, line_torus_roots, torus_elevation};
use crate::validate::decide;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A deterministic stream in `[0, 1)` (a 64-bit LCG): poses nobody
/// chose, replayed identically on every run.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
}

/// For each shape, lines through the tube region, directions drawn on
/// the sphere and then squashed toward the tube's midplane (`d.y *=
/// 1e-4`, the near-perpendicular family the defect was found in), plus
/// the same count left unsquashed so the Ferrari arm (`e = d·a ≠ 0`)
/// is exercised too.
#[test]
fn every_certified_root_lands_on_the_tube_or_the_consumer_refuses() {
    let (center, axis) = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let mut rng = Lcg(0x6765_726d_746f_7275);
    let (mut roots, mut contradicted, mut ferrari) = (0usize, 0usize, 0usize);
    let mut definite = 0usize;
    let mut displaced = 0usize;
    // The two measured poses (PR 3265 review): the long edges of a
    // near-perpendicular framed bar through the donut (`R = 2`,
    // `r = 0.5`), and a rod lying in its tube. Each certified a root
    // the landing test then put definitely off the tube.
    let measured: Vec<(Point3<f64>, Vec3<f64>)> = {
        let mut v = Vec::new();
        let poses = [
            (
                Point3::new(
                    -2.109_637_800_205_744_5,
                    0.170_221_792_550_834_86,
                    1.920_645_887_674_835_4,
                ),
                Vec3::new(
                    -0.990_360_666_876_138_8,
                    1.376_996_009_986_983_2e-4,
                    0.138_512_564_568_957,
                ),
                [1e-6, 1e-3],
                -4.6,
            ),
            (
                Point3::new(
                    -1.647_779_393_496_495_5,
                    0.270_473_450_406_354_4,
                    1.497_185_557_815_917,
                ),
                Vec3::new(
                    -0.980_697_285_232_897,
                    7.766_907_203_911_848e-5,
                    -0.195_532_167_952_848_92,
                ),
                [1e-5, 1e-5],
                -3.6,
            ),
        ];
        // Each edge line is anchored where the bar's profile sits, as the
        // extruded edge's own carrier is: a root's rounding depends on
        // the line's origin, so the pose is replayed as the body has it.
        for (o, d, ws, t0) in poses {
            let d = d.normalize();
            let o = o + d * t0;
            let u = d.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
            let w_ = d.cross(u);
            for w in ws {
                for (a, b) in [
                    (-1.0, -1.0),
                    (1.0, -1.0),
                    (1.0, 1.0),
                    (-1.0, 1.0),
                    (0.0, 0.0),
                ] {
                    v.push((o + u * (a * w / 2.0) + w_ * (b * w / 2.0), d));
                }
            }
        }
        v
    };
    for (big_r, r) in [(2.0, 0.5), (0.8, 0.5), (1.0, 0.3)] {
        // A prism side face relabelled a torus: its boundary is a
        // walkable loop that lies nowhere near the tube, so the boundary
        // pre-pass answers nothing and the placement reaches its carrier
        // test — the step this row is about.
        let prism = crate::fixtures::raw_prism(3, Tol::witness());
        let face = prism.face_side[0];
        let mut body = prism.body;
        let surface = geom::Surface::Torus {
            center,
            axis,
            major_radius: big_r,
            minor_radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        body.set_face_surface(face, crate::euler::FaceSurface::New(surface.clone()))
            .unwrap();
        let drawn = (0..400).map(|k| {
            let o = Point3::new(
                rng.range(-(big_r + r), big_r + r),
                rng.range(-r, r),
                rng.range(-(big_r + r), big_r + r),
            );
            let mut d = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
            if k % 2 == 0 {
                d.y *= 1e-4;
            }
            (k, o, d)
        });
        let lines: Vec<(usize, Point3<f64>, Vec3<f64>)> = if big_r == 2.0 {
            measured
                .iter()
                .map(|&(o, d)| (0, o, d))
                .chain(drawn)
                .collect()
        } else {
            drawn.collect()
        };
        for (k, o, d) in lines {
            if d.norm() < 1e-3 {
                continue;
            }
            let d = d.normalize();
            let got = line_torus_roots(o, d, center, axis, big_r, r, band());
            let Ok(TorusRoots::Certified { count, ts }) = got else {
                continue;
            };
            if k % 2 == 1 {
                ferrari += 1;
            }
            for &t in &ts[..count] {
                roots += 1;
                let p = o + d * t;
                let on_tube = decide(
                    "bool_curved_contain_carrier",
                    Margin::of(torus_elevation(center, axis, big_r, r, p)),
                    band(),
                );
                let placed = curved_face_placement(&body, face, p, band());
                match (on_tube, placed) {
                    (Ok(Sign::Zero), Ok(CurvedPlacement::OffCarrier)) => panic!(
                        "R = {big_r}, r = {r}: a root the elevation puts ON the tube was \
                         placed off it, at {p:?}"
                    ),
                    (Ok(Sign::Zero), _) => {}
                    // The root is contradicted: the consumer must be told
                    // so, never handed a trim verdict it would read as a
                    // sibling face's crossing.
                    (_, Ok(CurvedPlacement::OffCarrier)) => {
                        contradicted += 1;
                        definite += 1;
                    }
                    (_, Err(_)) => contradicted += 1,
                    (on, placed) => panic!(
                        "R = {big_r}, r = {r}: a certified root off the tube ({on:?}) was \
                         placed as {placed:?} at {p:?} — the consumer would read it as a \
                         trim verdict"
                    ),
                }
                // The same landing point pushed definitely off the tube
                // (twice the escalation width along the tube normal): the
                // shape a contradicted root has. It must be placed
                // `OffCarrier`, never as a trim verdict.
                let n = geom_brep::implicit_gradient(&surface, p).normalize();
                let q = p + n * (2.0 * band().escalate());
                match curved_face_placement(&body, face, q, band()) {
                    Ok(CurvedPlacement::OffCarrier) => displaced += 1,
                    other => panic!(
                        "R = {big_r}, r = {r}: a point definitely off the tube was placed \
                         {other:?} at {q:?} — a contradicted root would be read as a trim \
                         verdict"
                    ),
                }
            }
        }
    }
    println!(
        "{roots} certified roots, {contradicted} contradicted ({definite} definitely off the \
         tube), {ferrari} Ferrari-arm poses"
    );
    assert!(
        roots > 1000,
        "the lattice must reach the root lane: {roots}"
    );
    assert!(
        ferrari > 100,
        "the Ferrari arm must be exercised: {ferrari}"
    );
    // Since the root search's accuracy fix (#3255) no certified root in
    // this lattice lands off the tube at any band the run matrix draws;
    // the measured poses that did (44 at 1e-9, 460 at 1e-12 before it)
    // now land on it. The consumer's contract is what this row still
    // holds, and the displaced probes below keep it executed.
    assert_eq!(
        displaced, roots,
        "every root's displaced landing point must have been placed"
    );
}
