// Independent re-implementation of the PR's axis_interval/slab_enter, plain f64.
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
    // ---- Probe A: d = +-0.0, origin strictly outside the slab.
    let b = B{min:[10.0,-1.0,-1.0], max:[20.0,1.0,1.0]};
    println!("A1 (o.x=0 below slab, d.x=+0): {:?}", slab_enter([0.0,0.0,0.0],[0.0,1.0,0.0],&b));
    println!("A2 (o.x=30 above slab, d.x=+0): {:?}", slab_enter([30.0,0.0,0.0],[0.0,1.0,0.0],&b));
    println!("A3 (o.x=0 below, d.x=-0): {:?}", slab_enter([0.0,0.0,0.0],[-0.0,1.0,0.0],&b));
    println!("A4 (o.x=30 above, d.x=-0): {:?}", slab_enter([30.0,0.0,0.0],[-0.0,1.0,0.0],&b));

    // ---- Probe B: overflow of (lo - o): does a TRUE intersection get pruned?
    let o = [1.7e308, 0.0, 0.0];
    let d = [-1e300, 1.0, 0.0];
    let bx_lo = -1.7e308; let bx_hi = -1.6e308;
    println!("B: naive (lo-o)={} (hi-o)={}", bx_lo - o[0], bx_hi - o[0]);
    let tl: f64 = bx_lo/d[0] - o[0]/d[0];
    let th: f64 = bx_hi/d[0] - o[0]/d[0];
    let (tmin_true, tmax_true) = (th.min(tl), th.max(tl));
    println!("B: true t range approx [{}, {}]", tmin_true, tmax_true);
    let ylo = 0.0; let yhi = 1e9;
    let bb = B{min:[bx_lo, ylo, -1.0], max:[bx_hi, yhi, 1.0]};
    println!("B: slab_enter = {:?}", slab_enter(o,d,&bb));
    let tstar = (tmin_true + tmax_true)/2.0;
    let px = o[0] + d[0]*tstar; let py = o[1] + d[1]*tstar; let pz = o[2] + d[2]*tstar;
    println!("B: point at t*={} is ({},{},{}) inside? x:{} y:{} z:{}", tstar, px,py,pz,
        px>=bb.min[0]&&px<=bb.max[0], py>=bb.min[1]&&py<=bb.max[1], pz>=bb.min[2]&&pz<=bb.max[2]);
}
