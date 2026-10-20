//! Printing probes of the pierce lane over far origins, tilted edges
//! and edges in the plane (ignored), and the tilted-edge fixtures the
//! in-plane row reads.
use super::*;

pub(super) fn tilted_ends(pose: &Pose, normal: Vec3<f64>, tilt: f64) -> [Point3<f64>; 2] {
    let [(x0, y0), (x1, y1)] = [PROFILE[0], PROFILE[1]];
    let along = normal.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    let aim = Point3::new(
        x0 + (x1 - x0) * pose.across,
        y0 + (y1 - y0) * pose.across,
        1.0 - pose.depth,
    );
    let half = Vec3::new(0.0, 0.0, 0.5) + along * (0.5 * tilt);
    [
        aim - half + normal * pose.off,
        aim + half - normal * pose.off,
    ]
}

fn exact_depth_ends(o: Point3<f64>, n: Vec3<f64>, [p0, p1]: [Point3<f64>; 2]) -> f64 {
    let n = [n.x, n.y, n.z].map(big);
    let r = |p: Point3<f64>| {
        big_dot(&[p.x, p.y, p.z].map(big), &n) - big_dot(&[o.x, o.y, o.z].map(big), &n)
    };
    let (r0, r1) = (r(p0), r(p1));
    let lever = &r0 - &r1;
    if lever == BigInt::from(0) {
        return f64::NAN;
    }
    let num = (big(1.0) - big(p0.z)) * &lever - (big(p1.z) - big(p0.z)) * &r0;
    let den = lever << 1074_usize;
    let (num, den) = if den < BigInt::from(0) {
        (-num, -den)
    } else {
        (num, den)
    };
    let head = |x: &BigInt| {
        let shift = x.bits().saturating_sub(62);
        (i64::try_from(x >> shift).unwrap() as f64, shift as i32)
    };
    let ((n, sn), (d, sd)) = (head(&num), head(&den));
    n / d * 2f64.powi(sn - sd)
}

pub(super) fn read_ends<T: Decide + crate::props::AtRestPolicy>(
    o: Point3<f64>,
    ends: [Point3<f64>; 2],
    band: Band,
) -> Result<Found, String> {
    std::panic::catch_unwind(|| {
        let mut prism =
            crate::test_support_fixtures::prism_z::<T>(&PROFILE, 0.0, 1.0, Tol::witness());
        let face = prism.side_faces[0];
        let s = prism.body.get_face(face).unwrap().surface;
        let lift = |p: Point3<f64>| p.map(T::from_f64);
        match prism.body.surfaces.get_mut(s).unwrap() {
            geom::Surface::Plane { origin, .. } => *origin = lift(o),
            _ => panic!(),
        }
        let geo = snapshot(&prism.body);
        let f = geo.faces.iter().find(|f| f.key == face).unwrap();
        let e = segment((lift(ends[0]), lift(ends[1])));
        let declared = Declared::index(&geo, &ContactRecords::default());
        let mut errors = Vec::new();
        pair_edge_face(&prism.body, &geo, &declared, band, None, &e, f, &mut errors);
        classify(&errors)
    })
    .map_err(|p| {
        p.downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_default()
    })
}

fn classify(errors: &[ValidationError]) -> Found {
    if errors.iter().any(|e| {
        matches!(
            e,
            ValidationError::CensusEscalated { .. } | ValidationError::CensusUnsupported { .. }
        )
    }) {
        Found::Refused
    } else if errors.is_empty() {
        Found::Nothing
    } else {
        Found::Pierce
    }
}

fn probe_n() -> usize {
    std::env::var("PROBE_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(200)
}

/// Attack grid: far offsets up to 1e3 m, tilts, three ε, both scalars.
#[test]
#[ignore = "review probe, prints only: cargo nextest run -p topo --run-ignored all --no-capture -E 'test(/probe_/)'"]
fn probe_grid() {
    let mut r = test_utils::fuzz::start("probe_grid");
    let n = probe_n();
    for scale in [3.0, 30.0, 300.0, 1000.0] {
        for eps in EPS {
            let band = Band::new(eps, K * eps).unwrap();
            let (mut read, mut wrong_f, mut wrong_i, mut ref_f, mut ref_i, mut panics) =
                (0, 0, 0, 0, 0, 0);
            let mut ex = Vec::new();
            for _ in 0..n {
                let sign = |r: &mut Rng| if r.below(2) == 0 { 1.0 } else { -1.0 };
                let pose = Pose {
                    far: (r.range(-scale, scale), r.range(-scale, scale)),
                    depth: sign(&mut r) * r.log_uniform(3.0 * K * eps, 1e-2),
                    off: r.log_uniform(1.05 * K * eps, 0.1),
                    across: r.range(0.2, 0.8),
                };
                let tilt = r.range(-0.4, 0.4);
                let (o, nrm) = plane(&pose);
                let ends = tilted_ends(&pose, nrm, tilt);
                let depth = exact_depth_ends(o, nrm, ends);
                let m = 2.0 * band.escalate();
                let truth = if depth >= m {
                    Found::Pierce
                } else if depth <= -m {
                    Found::Nothing
                } else {
                    continue;
                };
                read += 1;
                for (is_f, got) in [
                    (true, read_ends::<f64>(o, ends, band)),
                    (false, read_ends::<Interval>(o, ends, band)),
                ] {
                    match got {
                        Err(msg) => {
                            panics += 1;
                            ex.push(format!("PANIC f64={is_f} {pose:?} tilt {tilt}: {msg}"));
                        }
                        Ok(Found::Refused) => {
                            if is_f {
                                ref_f += 1;
                            } else {
                                ref_i += 1;
                            }
                        }
                        Ok(g) if g != truth => {
                            if is_f {
                                wrong_f += 1;
                            } else {
                                wrong_i += 1;
                            }
                            ex.push(format!(
                                "WRONG f64={is_f} {pose:?} tilt {tilt} depth {depth:e} got {g:?}"
                            ));
                        }
                        Ok(_) => {}
                    }
                }
            }
            println!(
                "[probe] scale {scale} eps {eps:e}: read {read} wrong f64 {wrong_f} Interval \
                 {wrong_i} refused f64 {ref_f} Interval {ref_i} panics {panics}"
            );
            for x in ex.iter().take(4) {
                println!("   {x}");
            }
        }
    }
}

/// Lever-AtMost attack: an edge (almost) in the plane, the origin far.
#[test]
#[ignore = "review probe, prints only: cargo nextest run -p topo --run-ignored all --no-capture -E 'test(/probe_/)'"]
fn probe_lever_atmost() {
    let mut r = test_utils::fuzz::start("probe_atmost");
    let n = probe_n();
    for scale in [1.0, 30.0, 1000.0, 1e4, 1e5] {
        for eps in EPS {
            let band = Band::new(eps, K * eps).unwrap();
            let (mut pf, mut pi, mut tot) = (0, 0, 0);
            let mut ex = Vec::new();
            for _ in 0..n {
                let pose = Pose {
                    far: (r.range(-scale, scale), r.range(-scale, scale)),
                    depth: r.range(-0.3, 0.3),
                    off: if r.below(3) == 0 {
                        0.0
                    } else {
                        r.range(-3.0 * eps, 3.0 * eps)
                    },
                    across: r.range(0.2, 0.8),
                };
                let tilt = r.range(-0.4, 0.4);
                let (o, nrm) = plane(&pose);
                let ends = tilted_ends(&pose, nrm, tilt);
                tot += 1;
                if let Err(m) = read_ends::<f64>(o, ends, band) {
                    pf += 1;
                    ex.push(format!("f64 {pose:?} tilt {tilt}: {m}"));
                }
                if let Err(m) = read_ends::<Interval>(o, ends, band) {
                    pi += 1;
                    ex.push(format!("Interval {pose:?} tilt {tilt}: {m}"));
                }
            }
            println!(
                "[probe] atmost scale {scale} eps {eps:e}: {tot} poses, panics f64 {pf} Interval {pi}"
            );
            for x in ex.iter().take(3) {
                println!("   {x}");
            }
        }
    }
}
