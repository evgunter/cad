//! PR 4246 third-review probes (scratch, not for merge): a graze at a
//! smooth vertex with one root placed at the vertex, the graze sized
//! from the band, under carrier jitter of a few ulp and of ε/2.

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

#[allow(clippy::too_many_arguments)]
fn case(
    q: [Vec3<f64>; 3],
    o: Point3<f64>,
    r: f64,
    phi: f64,
    beta: f64,
    g: f64,
    e: f64,
    jitter: (Vec3<f64>, f64),
) -> (Path<f64>, Vec<LoopArc<f64>>) {
    let w = |v: Vec3<f64>| q[0] * v.x + q[1] * v.y + q[2] * v.z;
    let at = |v: Vec3<f64>| o + w(v);
    let m = Vec3::new(-phi.sin(), 0.0, phi.cos());
    let t_hat = Vec3::new(phi.cos(), 0.0, phi.sin());
    let mt = m.cross(t_hat);
    let u0 = t_hat * 0.7f64.cos() - mt * 0.7f64.sin();
    let piece = CircleArc {
        centre: o,
        axis: w(m),
        radius: r,
        u_ref: w(u0),
        span: (0.0, 1.4),
    };
    let path = Path {
        pieces: vec![Piece::Arc(piece)],
        arrival: piece.tangent(1.4),
    };
    let c = r * phi.cos() + g;
    let a = (r * r - c * c).sqrt();
    let centre = at(Vec3::new(c, 0.0, 0.0));
    let x = w(Vec3::unit_x());
    let sv = core::f64::consts::FRAC_PI_2 + e / a;
    let u1 = w(Vec3::unit_y());
    let u2 = w(Vec3::new(0.0, beta.cos(), beta.sin()));
    let pi = core::f64::consts::PI;
    let arcs = vec![
        LoopArc::Conic {
            centre,
            axis: x,
            u_ref: u1,
            a,
            b: a,
            span: (sv - pi, sv),
        },
        LoopArc::Conic {
            centre: centre + jitter.0 * jitter.1,
            axis: (x + jitter.0.cross(x) * (jitter.1 / r)).normalize(),
            u_ref: u2,
            a,
            b: a,
            span: (sv - beta, sv - beta + pi),
        },
    ];
    (path, arcs)
}

/// The grazes and vertex offsets every sweep reads, relative to `r`: the
/// grazes run from inside every band to outside it, where the meeting is
/// decided and the parity must read even.

fn unit(seed: &mut u64) -> Vec3<f64> {
    let mut next = || {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        (*seed >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    };
    loop {
        let v = Vec3::new(next(), next(), next());
        let n = v.norm();
        if n > 0.2 && n < 1.0 {
            return v / n;
        }
    }
}

#[test]
#[ignore = "reviewer evidence"]
fn rp3_root_at_the_vertex() {
    let mut seed = 0x2545f4914f6cdd1du64;
    let mut report = String::new();
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        for r in [1e-3, 1.0, 1e3] {
            for off in [0.0, 1e3] {
                for (jname, jmag) in [("none", 0.0), ("4ulp", 4.0 * f64::EPSILON * (off + r)), ("eps/2", 0.5 * eps)] {
                    let (mut wrong, mut decided, mut total) = (0usize, 0usize, 0usize);
                    let mut first = None;
                    for _ in 0..6 {
                        let z = unit(&mut seed);
                        let x0 = unit(&mut seed).cross(z).normalize();
                        let q = [x0, z.cross(x0), z];
                        let o = Point3::origin() + unit(&mut seed) * off;
                        let phi = 0.7;
                        let beta = 1.3;
                        let jitter = (unit(&mut seed), jmag);
                        for gk in [1.01, 1.5, 3.0, 10.0, 100.0, 1e4, 1e6] {
                            let gap = gk * band.escalate();
                            let h = (2.0 * gap * r / phi.cos()).sqrt();
                            if h > 0.3 * r {
                                continue;
                            }
                            for sgn in [1.0, -1.0] {
                                for k in -16..=0 {
                                    for m in [1.0, 3.0] {
                                        for ss in [1.0, -1.0] {
                                            let s = ss * m * 10f64.powi(k) * h;
                                            let e = sgn * h + s;
                                            total += 1;
                                            let (path, arcs) = case(q, o, r, phi, beta, -gap, e, jitter);
                                            match path_parity(&path, &arcs, None, band) {
                                                Ok(Some(true)) => {
                                                    wrong += 1;
                                                    first.get_or_insert(format!("g/esc {gk} e/h {} s/h {:e}", e / h, s / h));
                                                }
                                                Ok(Some(false)) => decided += 1,
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let line = format!("eps {eps:e} r {r:e} off {off:e} jitter {jname}: wrong {wrong} decided-right {decided} of {total}; first {first:?}\n");
                    report.push_str(&line);
                }
            }
        }
    }
    eprintln!("{report}");
    std::fs::write(std::env::var("RP_OUT").unwrap_or_else(|_| "/tmp/rp3_root.txt".into()), report).unwrap();
}
