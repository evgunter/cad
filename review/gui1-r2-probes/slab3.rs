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
    // Zero direction on EVERY axis; origin strictly BELOW the x slab,
    // inside the y and z slabs. The doc says the d=0-origin-outside
    // prune "is exact"; on this side it does not prune at all.
    let b = B{min:[10.0,-1.0,-1.0], max:[20.0,1.0,1.0]};
    println!("below-slab, all-zero dir: {:?}", slab_enter([0.0,0.0,0.0],[0.0,0.0,0.0],&b));
    println!("above-slab, all-zero dir: {:?}", slab_enter([30.0,0.0,0.0],[0.0,0.0,0.0],&b));
    // Same, with only the x direction zero and the others unconstrained
    // by a zero-direction-inside-slab axis:
    println!("below-slab, dir=(0,0,0) but y,z inside: {:?}",
        slab_enter([0.0,0.5,0.5],[0.0,0.0,0.0],&b));
}
