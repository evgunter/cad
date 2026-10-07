//! PR 4246 second-review probes (scratch, not for merge).
//!
//! `graze_at_a_smooth_vertex`: a closed small circle on a sphere, split
//! into two arcs at a smooth vertex `V`, and a great-circle path whose
//! ends both lie outside the circle's cap (so the true parity is even,
//! whatever the path does near the circle) and which grazes the circle
//! within `e` of `V`, missing it by `g`. Each arc carries the circle on
//! its own `u_ref`, so the two arcs' roots differ only by rounding. A
//! decided `Some(true)` is a decided-wrong parity.
//!
//! Run: `cargo nextest run -p topo --run-ignored only review2_probes --no-capture`.

use super::*;
use geom_core::{Band, Point3, Vec3};
use std::cell::Cell;

thread_local! {
    /// Read the meeting on the pre-fix margin `r·ρ − |D|`.
    pub(crate) static OLD_MARGIN: Cell<bool> = const { Cell::new(false) };
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Tally {
    wrong: usize,
    right: usize,
    none: usize,
    err: usize,
}

impl Tally {
    fn add(&mut self, r: Result<Option<bool>, geom_core::Indeterminate>) {
        match r {
            Ok(Some(true)) => self.wrong += 1,
            Ok(Some(false)) => self.right += 1,
            Ok(None) => self.none += 1,
            Err(_) => self.err += 1,
        }
    }
}

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

#[test]
#[ignore = "reviewer evidence: prints a table"]
fn graze_at_a_smooth_vertex() {
    let gs: Vec<f64> = {
        let mut v = vec![0.0];
        for k in [1e-16, 1e-15, 1e-14, 1e-13, 1e-12, 1e-11, 1e-10] {
            v.push(k);
            v.push(-k);
        }
        v
    };
    let es: Vec<f64> = {
        let mut v = Vec::new();
        for k in [1e-10, 3e-10, 1e-9, 3e-9, 1e-8, 3e-8, 1e-7, 3e-7, 1e-6, 1e-5] {
            v.push(k);
            v.push(-k);
        }
        v
    };
    let mut first_wrong: Option<String> = None;
    let mut total_new = Tally::default();
    let mut total_old = Tally::default();
    let mode = std::env::var("R2_JITTER").unwrap_or_else(|_| "ulp".into());
    eprintln!("jitter mode {mode}");
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for r in [1e-3, 1.0, 1e3] {
            for off in [0.0, 1e3] {
                let mut rng = Rng(0x9e3779b97f4a7c15);
                let (mut new, mut old) = (Tally::default(), Tally::default());
                for _ in 0..30 {
                    let z = rng.unit();
                    let x0 = rng.unit().cross(z).normalize();
                    let q = [x0, z.cross(x0), z];
                    let o = Point3::origin() + rng.unit() * off;
                    let phi = rng.range(0.3, 1.2);
                    let beta = rng.range(0.3, 2.5);
                    let jdir = rng.unit();
                    let size = match mode.as_str() {
                        "none" => 0.0,
                        "ulp" => 4.0 * f64::EPSILON * (off + r),
                        _ => 1e-3 * eps,
                    };
                    for &g in &gs {
                        for &e in &es {
                            let (path, arcs) = case(q, o, r, phi, beta, g * r, e * r, (jdir, size));
                            OLD_MARGIN.with(|c| c.set(false));
                            let got = path_parity(&path, &arcs, None, band);
                            if got == Ok(Some(true)) && first_wrong.is_none() {
                                first_wrong = Some(format!(
                                    "eps {eps:e} r {r:e} off {off:e} phi {phi} beta {beta} g/r {g:e} e/r {e:e} q {q:?} o {o:?}"
                                ));
                            }
                            new.add(got);
                            OLD_MARGIN.with(|c| c.set(true));
                            old.add(path_parity(&path, &arcs, None, band));
                            OLD_MARGIN.with(|c| c.set(false));
                        }
                    }
                }
                eprintln!(
                    "eps {eps:e} r {r:e} off {off:e}: new {new:?} | old {old:?}"
                );
                for (t, s) in [(&mut total_new, new), (&mut total_old, old)] {
                    t.wrong += s.wrong;
                    t.right += s.right;
                    t.none += s.none;
                    t.err += s.err;
                }
            }
        }
    }
    eprintln!("TOTAL new {total_new:?} | old {total_old:?}");
    eprintln!("first decided-wrong: {first_wrong:?}");
}

/// The asserting form at this run's band, scale 1, no offset, the
/// second arc's carrier 4 ulp off the first's: no decided parity may be
/// odd (both ends lie outside the circle's cap). Red on 9a074c81 at
/// ε 1e-9; green with the pre-fix margin (`R2_OLD=1`).
#[test]
fn a_graze_at_a_smooth_vertex_never_decides_a_wrong_parity() {
    let band = super::cone_islands::band();
    OLD_MARGIN.with(|c| c.set(std::env::var("R2_OLD").is_ok()));
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let mut wrong = Vec::new();
    let mut decided = 0usize;
    for _ in 0..30 {
        let z = rng.unit();
        let x0 = rng.unit().cross(z).normalize();
        let q = [x0, z.cross(x0), z];
        let o = Point3::origin() + rng.unit() * 0.0;
        let phi = rng.range(0.3, 1.2);
        let beta = rng.range(0.3, 2.5);
        let jdir = rng.unit();
        for g in [0.0, 1e-16, -1e-16, 1e-15, -1e-15, 1e-14, -1e-14, 1e-13, -1e-13] {
            for e in [1e-10, -1e-10, 1e-9, -1e-9, 1e-8, -1e-8, 1e-7, -1e-7] {
                let (path, arcs) =
                    case(q, o, 1.0, phi, beta, g, e, (jdir, 4.0 * f64::EPSILON));
                match path_parity(&path, &arcs, None, band) {
                    Ok(Some(true)) => wrong.push((phi, beta, g, e)),
                    Ok(Some(false)) => decided += 1,
                    _ => {}
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} decided-wrong parities ({decided} decided right); first {:?}",
        wrong.len(),
        wrong.first()
    );
}
