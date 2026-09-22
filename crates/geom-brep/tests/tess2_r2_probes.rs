//! TESS-2 reviewer (r2) probes: dump every `PatchCell` of a battery of
//! described surfaces so an exact-rational referee (Python `fractions`)
//! can check that each enclosure contains the DESCRIBED patch's partial
//! at points inside the cell. Not a gate; a dump.

use geom::surfaces::nurbs::NurbsSurface;
use geom_brep::patch_bound::{PatchBoundError, PatchCell, patch_cells, patch_cells_refined};
use geom_core::Point3;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::knots::KnotVector;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 11
    }
    fn unit(&mut self) -> f64 {
        (self.next() as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % (n as u64)) as usize
    }
}

fn hex(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}

fn iv(i: &RingInterval) -> String {
    if i.is_poison() {
        "poison poison".to_string()
    } else {
        format!("{} {}", hex(i.lo()), hex(i.hi()))
    }
}

fn dump(name: &str, s: &NurbsSurface<f64>, arm: &str, splits: usize, cells: &[PatchCell]) {
    println!(
        "SURFACE {name} {} {} {} {}",
        s.knots_u().degree(),
        s.knots_v().degree(),
        s.control_counts().0,
        s.control_counts().1
    );
    let list = |v: &[f64]| v.iter().map(|x| hex(*x)).collect::<Vec<_>>().join(",");
    println!("KU {}", list(s.knots_u().knots()));
    println!("KV {}", list(s.knots_v().knots()));
    println!("W {}", list(s.weights()));
    let pts: Vec<f64> = s.control().iter().flat_map(|p| [p.x, p.y, p.z]).collect();
    println!("P {}", list(&pts));
    println!("SPLITS {splits} ARM {arm}");
    for c in cells {
        let comp = |a: &[RingInterval; 3]| a.iter().map(iv).collect::<Vec<_>>().join(" ");
        println!(
            "CELL {} {} {} {} | {} | {} | {} | {} | {}",
            hex(c.u.0),
            hex(c.u.1),
            hex(c.v.0),
            hex(c.v.1),
            comp(&c.s_u),
            comp(&c.s_v),
            comp(&c.s_uu),
            comp(&c.s_uv),
            comp(&c.s_vv)
        );
    }
    println!("END");
}

fn mk_kv(r: &mut Lcg, p: usize, spans: usize, mult: usize) -> KnotVector {
    let mut k = vec![0.0; p + 1];
    for i in 1..spans {
        // Irregular interior knots so the ratios are not dyadic.
        let t = (i as f64 + r.range(-0.3, 0.3)) / spans as f64;
        for _ in 0..mult {
            k.push(t);
        }
    }
    k.extend(vec![1.0; p + 1]);
    KnotVector::clamped(k, p).unwrap()
}

fn battery(seed: u64, count: usize, integral: bool, tag: &str) {
    let mut r = Lcg(seed);
    let mut made = 0usize;
    let mut idx = 0usize;
    while made < count {
        idx += 1;
        let pu = 1 + r.below(3);
        let pv = 1 + r.below(3);
        let spans_u = if pu >= 2 { 1 + r.below(3) } else { 1 };
        let spans_v = if pv >= 2 { 1 + r.below(3) } else { 1 };
        // Multiplicity up to p-1: the C1 gate's edge.
        let mu = if pu >= 2 { 1 + r.below(pu - 1) } else { 1 };
        let mv = if pv >= 2 { 1 + r.below(pv - 1) } else { 1 };
        let kv_u = mk_kv(&mut r, pu, spans_u, mu);
        let kv_v = mk_kv(&mut r, pv, spans_v, mv);
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let offset = match r.below(4) {
            0 => 1.0e3,
            1 => -37.5,
            _ => 0.0,
        };
        let scale = match r.below(3) {
            0 => 1.0e-3,
            1 => 1.0e2,
            _ => 1.0,
        };
        let wexp = match r.below(3) {
            0 => 2.0,
            1 => 3.0,
            _ => 1.0,
        };
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for _ in 0..nu * nv {
            control.push(Point3::new(
                offset + scale * r.range(-2.0, 2.0),
                offset + scale * r.range(-2.0, 2.0),
                offset + scale * r.range(-2.0, 2.0),
            ));
            weights.push(if integral {
                1.0
            } else {
                10f64.powf(r.range(-wexp, wexp))
            });
        }
        let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
        let name = format!("{tag}{idx}");
        let splits_choice = [0usize, 3, 5, 32][r.below(4)];
        let res = if splits_choice == 0 {
            patch_cells(&s)
        } else {
            patch_cells_refined(&s, splits_choice)
        };
        match res {
            Ok(cells) => {
                let arm = if integral { "integral" } else { "rational" };
                dump(&name, &s, arm, splits_choice, &cells);
                made += 1;
            }
            Err(e) => println!("REFUSED {name} {e:?}"),
        }
    }
}

#[test]
fn r2_dump_rational_battery() {
    battery(0x5eed_0001, 40, false, "R");
}

#[test]
fn r2_dump_integral_battery() {
    battery(0x5eed_0002, 25, true, "I");
}

/// Reachability of `RefinedWeightLostPositivity`: the min subnormal
/// weight, whose convex combination rounds DOWN to zero.
#[test]
fn r2_refined_weight_lost_positivity_reachability() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let ctl: Vec<Point3<f64>> = (0..9).map(|i| Point3::new(i as f64, 0.0, 0.0)).collect();
    for w in [5e-324f64, 1e-320, 1e-300, 1e-200, 1e-2, 1e-30] {
        let mut ws = vec![w; 9];
        ws[4] = 1e2; // a wide RATIO in every case
        let s = NurbsSurface::new(kv.clone(), kv.clone(), ctl.clone(), ws).unwrap();
        let r = patch_cells(&s);
        println!(
            "weight {w:e} vs 1e2: {}",
            match &r {
                Ok(c) => format!("ok {} cells", c.len()),
                Err(e) => format!("{e:?}"),
            }
        );
        if w >= 1e-300 {
            assert!(!matches!(
                r,
                Err(PatchBoundError::RefinedWeightLostPositivity)
            ));
        }
    }
}
