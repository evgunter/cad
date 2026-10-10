use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{PlaneNurbsLimbs, SsiTube};
use geom_core::interval::Interval;
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Point3, Real, Tol, Vec3};

struct Sec { k: Vec<f64>, p: usize, ctl: Vec<(f64,f64)>, w: Vec<f64>, vw: [f64;2] }

fn decades(w: [f64;6]) -> Sec {
    Sec { k: vec![0.,0.,0.,0.,0.2,0.7,1.,1.,1.,1.], p:3,
      ctl: vec![(0.,0.),(0.5,0.8),(1.0,1.0),(1.6,0.3),(2.0,0.9),(2.4,0.2)], w: w.to_vec(), vw:[1.,1.] }
}
fn fams() -> Vec<(&'static str, Sec, f64)> {
    let multi = |w: Vec<f64>, vw: [f64;2]| Sec { k: vec![0.,0.,0.,0.,0.13,0.29,0.41,0.58,0.77,1.,1.,1.,1.], p:3,
        ctl: vec![(0.,0.),(0.3,0.7),(0.8,0.9),(1.2,0.2),(1.7,0.8),(2.1,-0.1),(2.6,0.6),(3.0,0.1),(3.3,0.9)], w, vw };
    let steep = Sec { k: vec![0.,0.,0.,0.31,0.62,1.,1.,1.], p:2,
        ctl: vec![(0.,0.),(0.4,1.1),(1.2,1.3),(1.9,0.2),(2.6,0.9)], w: vec![1., 0.004, 250., 0.01, 1.], vw:[1.,1.] };
    vec![
        ("d3", decades([1.,0.03,30.,0.5,8.,1.]), 0.0),
        ("d2", decades([1.,0.1,10.,0.5,3.,1.]), 0.0),
        ("m100", multi(vec![1.,0.02,40.,0.05,9.,0.01,70.,0.3,1.], [1.,1.]), 0.0),
        ("m100v7", multi(vec![1.,0.02,40.,0.05,9.,0.01,70.,0.3,1.], [1.,7.]), 0.0),
        ("steep", steep, 0.0),
        ("d3_a0.2", decades([1.,0.03,30.,0.5,8.,1.]), 0.2),
        ("d3_a0.05", decades([1.,0.03,30.,0.5,8.,1.]), 0.05),
        ("m100_a0.1", multi(vec![1.,0.02,40.,0.05,9.,0.01,70.,0.3,1.], [1.,1.]), 0.1),
    ]
}
fn wall<T: Real>(s: &Sec) -> NurbsSurface<T> {
    let ku = KnotVector::clamped(s.k.clone(), s.p).unwrap();
    let kv = KnotVector::clamped(vec![0.,0.,1.,1.], 1).unwrap();
    let f = T::from_f64; let (mut c, mut w) = (vec![], vec![]);
    for (i,(x,y)) in s.ctl.iter().enumerate() { for (j,z) in [0.0,1.0].iter().enumerate() {
        c.push(Point3::new(f(*x),f(*y),f(*z))); w.push(s.w[i]*s.vw[j]); } }
    NurbsSurface::new(ku, kv, c, w).unwrap()
}
fn sec(s: &Sec) -> NurbsCurve3<f64> {
    NurbsCurve3::new(KnotVector::clamped(s.k.clone(), s.p).unwrap(),
      s.ctl.iter().map(|(x,y)| Point3::new(*x,*y,0.0)).collect(), s.w.clone()).unwrap()
}
fn ruling<T: Real>(x: f64, y: f64) -> NurbsCurve3<T> {
    let f = T::from_f64;
    NurbsCurve3::new(KnotVector::clamped(vec![0.,0.,1.,1.],1).unwrap(),
      vec![Point3::new(f(x),f(y),f(0.)), Point3::new(f(x),f(y),f(1.))], vec![1.,1.]).unwrap()
}
fn plane<T: Real>(x: f64, y: f64, n: (f64,f64)) -> Surface<T> {
    let f = T::from_f64;
    Surface::Plane { origin: Point3::new(f(x),f(y),f(0.)), normal: Vec3::new(f(n.0),f(n.1),f(0.)), u_ref: Vec3::new(f(0.),f(0.),f(1.)) }
}
fn main() {
    let band = Band::linear(Tol::witness()).unwrap();
    let only: Option<String> = std::env::args().nth(1);
    for (name, s, alpha) in fams() {
        if let Some(o) = &only { if o != name { continue; } }
        let c = sec(&s);
        for i in 1..20 {
            let u0 = f64::from(i)/20.0 + 0.003;
            let (q, t) = (c.eval(u0), c.deriv(u0));
            let tn = (t.x*t.x+t.y*t.y).sqrt(); let (tx,ty) = (t.x/tn, t.y/tn);
            let (nx, ny) = (-ty, tx);
            let n = (alpha.cos()*tx + alpha.sin()*nx, alpha.cos()*ty + alpha.sin()*ny);
            // alpha=0: transverse (normal = tangent). alpha->pi/2 near tangent. Use complement:
            let n = if alpha > 0.0 { (alpha.sin()*tx + alpha.cos()*nx, alpha.sin()*ty + alpha.cos()*ny) } else { n };
            let t0 = std::time::Instant::now();
            let r: Result<PlaneNurbsLimbs<f64>,_> = geom_brep::plane_nurbs_limbs(&ruling(q.x,q.y), &plane(q.x,q.y,n), &wall::<f64>(&s), 1.0, band);
            let dt = t0.elapsed().as_secs_f64();
            match r { Ok(l) => { let SsiTube::Chart{rung,pad_u,pad_v} = l.tube else { panic!() };
                println!("{name},{i},f64,ok,{:e},{:e},{:e},{:e},{},{:e},{dt:.4}", l.hull_sup, rung, pad_u, pad_v, l.tube_boxes, l.tube_transversality); }
              Err(e) => println!("{name},{i},f64,ERR {e:?},,,,,,,{dt:.4}") }
            let t0 = std::time::Instant::now();
            let r: Result<PlaneNurbsLimbs<Interval>,_> = geom_brep::plane_nurbs_limbs(&ruling(q.x,q.y), &plane(q.x,q.y,n), &wall::<Interval>(&s), Interval::one(), band);
            let dt = t0.elapsed().as_secs_f64();
            match r { Ok(l) => { let SsiTube::Chart{rung,pad_u,pad_v} = l.tube else { panic!() };
                println!("{name},{i},itv,ok,{:e},{:e},{:e},{:e},{},{:e},{dt:.4}", l.hull_sup.hi(), rung.hi(), pad_u.lo(), pad_v.lo(), l.tube_boxes, l.tube_transversality.lo()); }
              Err(e) => println!("{name},{i},itv,ERR {e:?},,,,,,,{dt:.4}") }
        }
    }
}
