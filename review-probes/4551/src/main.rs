use geom_core::interval::certification::Certification;
use geom_core::{Bounds, Interval, KnotVector};

struct R(u64);
impl R {
    fn next(&mut self) -> u64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; self.0 }
    fn f(&mut self) -> f64 { (self.next() >> 11) as f64 / (1u64 << 53) as f64 }
    fn n(&mut self, k: u64) -> u64 { self.next() % k }
}
fn h(x: f64) -> String { format!("{:016x}", x.to_bits()) }
fn up(x: f64, k: i64) -> f64 { let mut v = x; for _ in 0..k.abs() { v = if k > 0 { v.next_up() } else { v.next_down() }; } v }

fn val(r: &mut R) -> f64 {
    let s = if r.n(2) == 0 { 1.0 } else { -1.0 };
    match r.n(9) {
        0 => 0.0 * s,
        1 => s * r.f() * 1e-310,
        2 => s * (1.0 + r.f()) * 1e300,
        3 => s * (1.0 + 0.79 * r.f()) * 1e308,
        4 => s * f64::MIN_POSITIVE * r.f(),
        5 => s * 1e3 * (1.0 + r.f()),
        _ => s * r.f(),
    }
}
fn iv(r: &mut R) -> (f64, f64) {
    let a = val(r);
    let b = match r.n(5) { 0 => a, 1 => up(a, 1 + r.n(4) as i64), 2 => val(r), 3 => a + r.f() * a.abs().max(1e-300), _ => a + r.f() };
    if a <= b { (a, b) } else { (b, a) }
}
fn lam(r: &mut R) -> (f64, f64) {
    let (a, b) = match r.n(10) {
        0 => (0.5, 0.5),
        1 => (up(0.5, -(r.n(5) as i64)), up(0.5, r.n(5) as i64)),
        2 => (-0.0, r.f()),
        3 => (r.f(), 1.0),
        4 => (up(0.0, -(r.n(3) as i64)), up(1.0, r.n(3) as i64)),
        5 => (-0.05 * r.f(), r.f()),
        6 => (r.f(), 1.0 + 0.05 * r.f()),
        7 => (0.0, 0.0),
        8 => (1.0, 1.0),
        _ => { let a = r.f(); let b = r.f(); (a, b) }
    };
    if a <= b { (a, b) } else { (b, a) }
}

fn main() {
    let mode = std::env::args().nth(1).unwrap();
    let mut r = R(0x1234_5678_9abc_def1);
    if mode == "between" {
        for _ in 0..200000 {
            let (x, y, l) = (iv(&mut r), iv(&mut r), lam(&mut r));
            let res = Interval::between(Interval::from_bounds(x.0, x.1), Interval::from_bounds(y.0, y.1), Interval::from_bounds(l.0, l.1));
            println!("{} {} {} {} {} {} {} {} {}", h(x.0), h(x.1), h(y.0), h(y.1), h(l.0), h(l.1), res.is_certified() as u8, h(res.lo()), h(res.hi()));
        }
        // poison checks
        let one = Interval::from_bounds(0.0, 1.0);
        for (tag, a, b, c) in [("x", Interval::refused(), one, one), ("y", one, Interval::refused(), one), ("l", one, one, Interval::refused()),
                               ("inf", Interval::from_bounds(0.0, f64::INFINITY), one, one), ("lout", one, one, Interval::from_bounds(1.5, 2.0)),
                               ("nan_l", one, one, Interval::from_bounds(f64::NAN, 0.5))] {
            let res = Interval::between(a, b, c);
            eprintln!("POISON {tag}: certified={}", res.is_certified());
        }
    } else {
        // blossom
        for case in 0..30000 {
            let p = 1 + r.n(8) as usize;
            let nint = r.n(5) as usize;
            let mut interior: Vec<f64> = Vec::new();
            for _ in 0..nint {
                let v = match r.n(4) { 0 => (r.n(8) as f64) / 8.0, 1 => 0.1 * (1 + r.n(9)) as f64, _ => r.f() };
                if v <= 0.0 || v >= 1.0 { continue; }
                let m = 1 + r.n(p as u64) as usize;
                for _ in 0..m { interior.push(v); }
            }
            // close knots: an ulp-wide span sometimes
            if r.n(6) == 0 { let v = 0.3 + 0.4 * r.f(); interior.push(v); interior.push(up(v, 1 + r.n(3) as i64)); }
            interior.sort_by(f64::total_cmp);
            // cap multiplicity at p
            let mut capped: Vec<f64> = Vec::new();
            for &v in &interior { if capped.iter().filter(|&&w| w == v).count() < p { capped.push(v); } }
            let (d0, d1) = if r.n(3) == 0 { (-3.0, 1e3) } else { (0.0, 1.0) };
            let mut knots = vec![d0; p + 1];
            knots.extend(capped.iter().map(|v| d0 + (d1 - d0) * v));
            knots.extend(vec![d1; p + 1]);
            let Ok(kv) = KnotVector::clamped(knots.clone(), p) else { continue };
            let n = kv.control_count();
            let coeffs: Vec<Interval> = (0..n).map(|_| { let a = 2.0 * r.f() - 1.0; let a = a * if r.n(4)==0 {1e3} else {1.0}; let w = match r.n(3) { 0 => 0.0, 1 => a.abs() * 1e-15, _ => 1e-3 * r.f() }; Interval::from_bounds(a, a + w) }).collect();
            let pair = kv.with_coeffs(&coeffs).unwrap();
            let spans: Vec<usize> = (kv.first_span()..=kv.last_span()).filter(|&s| kv.span_is_nonempty(s)).collect();
            let s = spans[r.n(spans.len() as u64) as usize];
            let (lo, hi) = (knots[s], knots[s + 1]);
            let mut args = Vec::new();
            for _ in 0..p {
                let a = match r.n(8) {
                    0 => (lo, hi),
                    1 => (lo, lo),
                    2 => (hi, hi),
                    3 => { let v = lo + (hi - lo) * r.f(); (v, up(v, 1 + r.n(4) as i64).min(hi)) }
                    4 => (up(lo, -(1 + r.n(4) as i64)), up(lo, r.n(3) as i64).min(hi)), // pokes below
                    5 => (up(hi, -(r.n(3) as i64)).max(lo), up(hi, 1 + r.n(4) as i64)), // pokes above
                    _ => { let v = lo + (hi - lo) * r.f(); (v, v) }
                };
                args.push(a);
            }
            let w = pair.span(s).unwrap();
            let ai: Vec<Interval> = args.iter().map(|&(a, b)| Interval::from_bounds(a, b)).collect();
            let res = w.blossom(&ai).unwrap();
            let window = &coeffs[s - p..=s];
            let ks: Vec<String> = knots.iter().map(|&k| h(k)).collect();
            let cs: Vec<String> = window.iter().map(|c| format!("{}:{}", h(c.lo()), h(c.hi()))).collect();
            let as_: Vec<String> = args.iter().map(|&(a, b)| format!("{}:{}", h(a), h(b))).collect();
            println!("{case}|{p}|{s}|{}|{}|{}|{}|{}|{}", ks.join(","), cs.join(","), as_.join(","), res.is_certified() as u8, h(res.lo()), h(res.hi()));
            // restrict check row
            if r.n(4) == 0 {
                let a = lo + (hi - lo) * r.f() * 0.5; let b = if r.n(2)==0 { hi } else { a + (hi - a) * r.f() };
                let row = w.restrict((Interval::from_bounds(a, a), Interval::from_bounds(b, b))).unwrap();
                let rs: Vec<String> = row.iter().map(|c| format!("{}:{}:{}", c.is_certified() as u8, h(c.lo()), h(c.hi()))).collect();
                println!("R|{p}|{s}|{}|{}|{}:{}|{}", ks.join(","), cs.join(","), h(a), h(b), rs.join(","));
            }
            // refusals
            let bad_count = w.blossom(&ai[..p - 1]);
            assert!(matches!(bad_count, Err(geom_core::spline::BlossomError::ArgumentCount { .. })), "count");
            let mut out = ai.clone();
            out[0] = Interval::from_bounds(up(hi, 1), up(hi, 2));
            assert!(matches!(w.blossom(&out), Err(geom_core::spline::BlossomError::OutsideSpan { index: 0, .. })), "outside hi");
            out[0] = Interval::from_bounds(up(lo, -2), up(lo, -1));
            assert!(matches!(w.blossom(&out), Err(geom_core::spline::BlossomError::OutsideSpan { index: 0, .. })), "outside lo");
            out[0] = Interval::refused();
            let rr = w.blossom(&out).unwrap();
            assert!(!rr.is_certified(), "refused arg");
        }
        eprintln!("refusal asserts held");
    }
}
