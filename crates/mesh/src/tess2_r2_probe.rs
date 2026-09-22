//! TESS-2 reviewer (r2) probe: a deterministic bilinear-rational census
//! dumped for an exact referee — certified `muu/muv/mvv`, and the
//! 61x61 sampler's per-component argmax so the sampler's own error can
//! be measured against the exact truth at that point.

use geom::NurbsSurface;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use topo::FaceKey;

use crate::nurbs_cert::tests::nurbs_face_bound;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 11
    }
    fn unit(&mut self) -> f64 {
        (self.next() as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
}

fn hex(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}

#[test]
fn r2_bilinear_census_dump() {
    let n: usize = std::env::var("R2_CENSUS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(300);
    let mut r = Lcg(0x7e55_2025_0922_0002);
    let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
    for t in 0..n {
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for _ in 0..4 {
            control.push(Point3::new(
                r.range(-2.0, 2.0),
                r.range(-2.0, 2.0),
                r.range(-2.0, 2.0),
            ));
            weights.push(10f64.powf(r.range(-2.0, 2.0)));
        }
        let s = NurbsSurface::new(kv.clone(), kv.clone(), control.clone(), weights.clone())
            .unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
        let list = |v: &[f64]| v.iter().map(|x| hex(*x)).collect::<Vec<_>>().join(",");
        let pts: Vec<f64> = control.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
        // The sampler, re-spelled with its argmax recorded (61x61 grid,
        // as `r1_random_rational_soundness_sweep` runs it).
        let grid = 60u32;
        let mut best = [(0.0f64, 0.0f64, 0.0f64); 3];
        for i in 0..=grid {
            for j in 0..=grid {
                let u = f64::from(i) / f64::from(grid);
                let v = f64::from(j) / f64::from(grid);
                let jet = s.ders(u, v);
                let vals = [jet.duu.norm(), jet.duv.norm(), jet.dvv.norm()];
                for (k, val) in vals.iter().enumerate() {
                    if *val > best[k].2 {
                        best[k] = (u, v, *val);
                    }
                }
            }
        }
        println!(
            "TRIAL {t} W {} P {} MUU {} MUV {} MVV {} ARG {} {} {} {} {} {} {} {} {}",
            list(&weights),
            list(&pts),
            hex(b.muu),
            hex(b.muv),
            hex(b.mvv),
            hex(best[0].0),
            hex(best[0].1),
            hex(best[0].2),
            hex(best[1].0),
            hex(best[1].1),
            hex(best[1].2),
            hex(best[2].0),
            hex(best[2].1),
            hex(best[2].2),
        );
    }
}
