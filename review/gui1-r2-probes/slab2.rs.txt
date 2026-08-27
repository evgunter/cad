fn widen_down(v: f64) -> f64 { v.next_down().next_down().next_down().next_down() }
fn widen_up(v: f64) -> f64 { v.next_up().next_up().next_up().next_up() }
fn axis_interval(o: f64, d: f64, lo: f64, hi: f64) -> Option<(f64, f64)> {
    let inv = 1.0 / d;
    let t0 = (lo - o) * inv;
    let t1 = (hi - o) * inv;
    if t0.is_nan() || t1.is_nan() { return None; }
    let (near, far) = if t0 <= t1 { (t0, t1) } else { (t1, t0) };
    Some((widen_down(near), widen_up(far)))
}
#[derive(Clone,Copy,Debug)]
struct B { min:[f64;3], max:[f64;3] }
fn slab_enter(o:[f64;3], d:[f64;3], b:&B) -> Option<f64> {
    let mut t_min = 0.0f64; let mut t_max = f64::INFINITY;
    for a in 0..3 {
        if let Some((near,far)) = axis_interval(o[a], d[a], b.min[a], b.max[a]) {
            if near > t_min { t_min = near; }
            if far < t_max { t_max = far; }
        }
    }
    if t_min <= t_max { Some(t_min) } else { None }
}
fn main() {
    println!("next_down(+inf) = {:e}", f64::INFINITY.next_down());
    println!("next_up(-inf)   = {:e}", f64::NEG_INFINITY.next_up());
    println!("widen_down(+inf)= {:e}", widen_down(f64::INFINITY));
    println!("widen_up(-inf)  = {:e}", widen_up(f64::NEG_INFINITY));

    // ---- Case S: SUBNORMAL direction component; 1/d overflows to +inf.
    // Box x in [1e-300, 2e-300]; origin x = 0; d.x = 5e-324 (min subnormal).
    // True t range on x: [1e-300/5e-324, 2e-300/5e-324] ~= [2.0e23, 4.0e23] -- finite.
    let d = [5e-324f64, 1.0, 0.0];
    let o = [0.0f64, 0.0, 0.0];
    let bb = B { min: [1e-300, 0.0, -1.0], max: [2e-300, 1e30, 1.0] };
    println!("S: 1/d.x = {:e}", 1.0/d[0]);
    // Exact-ish true t via logs / scaled division (avoid overflow):
    // t = x / d  computed as (x * 2^1074) / (d * 2^1074) with d*2^1074 = 1.0
    let scale = 2f64.powi(1074);
    let tlo = (bb.min[0]*scale) / (d[0]*scale);
    let thi = (bb.max[0]*scale) / (d[0]*scale);
    println!("S: true x-slab t range = [{:e}, {:e}]", tlo, thi);
    println!("S: slab_enter = {:?}  <-- expect Some(...) for conservativeness", slab_enter(o,d,&bb));
    // verify a true intersection point at t* in the middle
    let tstar = (tlo+thi)*0.5;
    let px = (d[0]*scale)*(tstar/scale); // = d.x * t*, computed without underflow
    println!("S: t*={:e}, x(t*)={:e} in [{:e},{:e}]? {}", tstar, px, bb.min[0], bb.max[0],
        px>=bb.min[0] && px<=bb.max[0]);
    println!("S: y(t*)={:e} in [0,1e30]? {}", tstar, tstar>=0.0 && tstar<=1e30);

    // ---- Case O: overflow of (lo - o).
    let o2 = [1.7e308f64, 0.0, 0.0];
    let d2 = [-1e300f64, 1.0, 0.0];
    let b2 = B { min: [-1.7e308, 0.0, -1.0], max: [-1.6e308, 1e9, 1.0] };
    let tstar2 = 3.35e8f64;
    // x(t*) computed without overflow: o + d*t = 1.7e308 - 1e300*3.35e8
    //   = 1.7e308 - 3.35e308  -> compute as (1.7 - 3.35)e308
    let xt = 1.7e308 - 3.35e8*1e300_f64.min(f64::MAX); // still overflows; do it in halves
    let half = d2[0]*(tstar2*0.5); // -1.675e308, representable
    let xt2 = (o2[0] + half) + half;
    println!("O: x(t*) via halves = {:e} (naive {:e})", xt2, xt);
    println!("O: in [{:e},{:e}]? {}", b2.min[0], b2.max[0], xt2>=b2.min[0]&&xt2<=b2.max[0]);
    println!("O: y(t*)={:e} in [0,1e9]? {}", tstar2, tstar2>=0.0&&tstar2<=1e9);
    println!("O: slab_enter = {:?}  <-- expect Some(...) for conservativeness", slab_enter(o2,d2,&b2));
}
