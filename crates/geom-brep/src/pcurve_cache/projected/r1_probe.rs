//! Reviewer R1's probe for PR 6806: dumps `Meters::at_height` and
//! `Meters::certified` readings at `Interval` on random sheet boxes, for
//! an independent decimal oracle (`r1_oracle.py`). Not a gate.
use super::Meters;
use geom_core::{Bounds, Interval};
use std::io::Write;

struct Rng(u64);
impl Rng {
    fn unit(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.unit()
    }
    fn log(&mut self, a: f64, b: f64) -> f64 {
        (a.ln() + (b.ln() - a.ln()) * self.unit()).exp()
    }
}

#[test]
#[ignore = "probe: R1_OUT=<csv> cargo nextest run --run-ignored all r1_probe"]
fn r1_dump() {
    let out = std::env::var("R1_OUT").unwrap_or_else(|_| "/dev/null".into());
    let n: usize = std::env::var("R1_N").ok().and_then(|s| s.parse().ok()).unwrap_or(200_000);
    let seed: u64 = std::env::var("R1_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0x9e37_79b9_7f4a_7c15);
    let mut f = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    let mut s = Rng(seed);
    let pt = |x: f64| Interval::from_bounds(x, x);
    let positive = |l: Interval| l.is_certified() && l.lo() > 0.0;
    let mut reads = 0;
    for _ in 0..n {
        let minor = s.range(0.2, 2.0);
        let kind = s.unit();
        let major = if kind < 0.3 {
            minor
        } else if kind < 0.5 {
            minor * (1.0 - s.log(1e-14, 1e-2))
        } else if kind < 0.6 {
            minor * (1.0 + s.log(1e-16, 1e-13))
        } else if kind < 0.85 {
            minor * s.range(0.05, 0.99)
        } else {
            -minor * s.range(0.05, 0.99)
        };
        let h2 = ((minor - major) * (minor + major)).max(0.0);
        let h = h2.sqrt();
        let sgn = if s.unit() < 0.5 { -1.0 } else { 1.0 };
        let seed_kind = s.unit();
        let z_c = if seed_kind < 0.35 {
            // beside the cusp / the apex
            h + s.log(1e-13, 1e-1) * minor
        } else if seed_kind < 0.55 {
            // straddling h
            h + s.range(-1.0, 1.0) * s.log(1e-14, 1e-2) * minor
        } else if seed_kind < 0.75 {
            // near the top of the tube
            minor * (1.0 - s.log(1e-15, 1e-2))
        } else {
            s.range(h, minor)
        };
        let w = (minor * minor - z_c * z_c).max(0.0).sqrt();
        let rho_inner = (z_c * z_c - h2) / (major + w);
        let rho_c = match (s.unit() * 4.0) as u32 {
            0 => major + w,  // this sheet's outer branch
            1 => (w - major).max(0.0), // the other sheet's circle at this height
            2 => 0.0,
            _ => rho_inner,
        };
        let scale = s.log(1e-17, 1e-1) * minor;
        let zw = s.log(1e-17, 1e-1) * minor;
        let rho_lo = (rho_c - scale * s.unit()).max(0.0);
        let rho_hi = rho_lo + scale * s.unit() * 2.0;
        let z0 = z_c - zw * s.unit();
        let z1 = z0 + zw * s.unit() * 2.0;
        let (z0, z1) = if sgn < 0.0 { (-z1, -z0) } else { (z0, z1) };
        let m = Meters::Spindle {
            major: pt(major),
            minor: pt(minor),
        };
        let ah = m.at_height((pt(rho_lo), pt(rho_hi)), (z0, z1), positive);
        let f_sup = f64::NAN; // the oracle checks at_height alone
        let _ = f_sup;
        if let Some(r) = ah {
            reads += 1;
            writeln!(
                f,
                "{major:?},{minor:?},{rho_lo:?},{rho_hi:?},{z0:?},{z1:?},{:?},{}",
                r.hi(),
                r.is_certified()
            )
            .unwrap();
        }
    }
    eprintln!("r1_dump: {reads} readings of {n}");
}
