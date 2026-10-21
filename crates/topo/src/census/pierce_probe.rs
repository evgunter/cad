//! Printing probes of the pierce lane, ignored: far origins, tilted
//! edges and edges in the plane, and a thin notch or strip.
use super::rows::{notch_read, read_ends, strip_read, tilted_ends};
use super::*;

/// Poses per grid cell.
const PROBE_N: usize = 200;

/// Attack grid: far offsets up to 1e3 m, tilts, three ε, both scalars.
#[test]
#[ignore = "review probe, prints only: cargo nextest run -p topo --run-ignored all --no-capture -E 'test(/probe_/)'"]
fn probe_grid() {
    let mut r = test_utils::fuzz::start("probe_grid");
    let n = PROBE_N;
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
                let depth = depth_below_top(o, nrm, ends).unwrap_or(f64::NAN);
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
    let n = PROBE_N;
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

#[test]
#[ignore = "review probe, prints only: cargo nextest run -p topo --run-ignored all --no-capture -E 'test(/probe_/)'"]
fn probe_notch() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    for (w, far) in [
        (2e-2, (30.0, 30.0)),
        (2e-3, (30.0, 30.0)),
        (2e-4, (30.0, 30.0)),
        (2e-5, (30.0, 30.0)),
        (1e-4, (0.0, 0.0)),
        (1e-5, (0.0, 0.0)),
    ] {
        for off in [1.5e-11, 1e-10] {
            let f = notch_read::<f64>(w, far, off, band);
            let i = notch_read::<Interval>(w, far, off, band);
            println!(
                "[probe] notch w {w:e} (half-width {:e}) far {far:?} off {off:e}: f64 {:?}  Interval {:?}",
                0.1 * w,
                f.as_ref().map(|x| x.0),
                i.as_ref().map(|x| x.0)
            );
            if let Ok((_, errs)) = &f {
                for e in errs {
                    println!("     {e}");
                }
            }
        }
    }
    for (h, far) in [
        (2e-2, (30.0, 30.0)),
        (2e-4, (30.0, 30.0)),
        (2e-5, (30.0, 30.0)),
        (1e-5, (0.0, 0.0)),
        (1e-6, (0.0, 0.0)),
    ] {
        for off in [1.5e-11, 1e-10] {
            let f = strip_read::<f64>(h, far, off, band);
            let i = strip_read::<Interval>(h, far, off, band);
            println!(
                "[probe] strip h {h:e} far {far:?} off {off:e}: f64 {:?}  Interval {:?}",
                f.as_ref().map(|x| x.0),
                i.as_ref().map(|x| x.0)
            );
        }
    }
}
