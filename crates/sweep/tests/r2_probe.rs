#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::Surface;
use geom_core::{Affine3, Interval, Mat3, Point2, Real, Tol, Vec3, Bounds};
use profile::ProfileLoop;
use profile::test_support::bulge_loop;
use sweep::{Lofted, loft_body};

fn wall_values<T: geom_core::spline::SpanLocate, R>(lofted: &Lofted<T>, f: impl Fn(T) -> R) -> Vec<R> {
    let mut out = Vec::new();
    for face in lofted.side_faces.iter().flatten() {
        let face = lofted.body.get_face(*face).expect("a built face");
        match lofted.body.get_surface(face.surface).expect("a described face") {
            Surface::Nurbs(n) => {
                for p in n.control() { out.extend([f(p.x), f(p.y), f(p.z)]); }
                out.extend(n.weights().iter().map(|w| f(*w)));
                out.extend(n.knot_values_v().values().iter().map(|k| f(*k)));
            }
            other => panic!("{other:?}"),
        }
    }
    out
}

fn misses(enclosing: &Lofted<Interval>, sample: &Lofted<f64>) -> Vec<(usize, f64, f64, f64)> {
    let b = wall_values(enclosing, |x| (x.lo(), x.hi()));
    let v = wall_values(sample, |x| x);
    assert_eq!(b.len(), v.len());
    b.iter().zip(&v).enumerate().filter(|(_, ((lo, hi), v))| !(lo <= *v && *v <= hi))
        .map(|(i, ((lo, hi), v))| (i, *lo, *v, *hi)).collect()
}

/// Rotation by `deg` about the world y axis: plane x-axis (cos,0,-sin), normal (sin,0,cos).
fn fan(deg: f64) -> Affine3<f64> {
    let (s, c) = deg.to_radians().sin_cos();
    Affine3::from_parts(
        Mat3::from_cols(Vec3::new(c, 0.0, -s), Vec3::new(0.0, 1.0, 0.0), Vec3::new(s, 0.0, c)),
        Vec3::new(0.0, 0.0, 0.0),
    )
}

/// Triangle A=(t,0) [on the fan axis at t=0], B=(-s,1), C=(-s,-1), CCW.
fn tri<T: Real>(t: T, s: f64) -> Vec<ProfileLoop<T>> {
    let p = |x: T, y: f64| (Point2::new(x, T::from_f64(y)), T::zero());
    vec![bulge_loop(vec![p(t, 0.0), p(T::from_f64(-s), 1.0), p(T::from_f64(-s), -1.0)])]
}

#[test]
fn r2_pinned_row_abstains_over_a_box() {
    let degs = [0.0, 20.0, 40.0];
    let sizes = [1.0, 1.0, 3.0];
    let w = Tol::witness().eps() / 256.0;
    for (name, box_) in [("straddle", Interval::from_bounds(-w, w)), ("touch0", Interval::from_bounds(0.0, w))] {
        let secs: Vec<_> = sizes.iter().map(|s| tri(box_, *s)).collect();
        let places: Vec<Affine3<Interval>> = degs.iter().map(|d| fan(*d).map(Interval::from_f64)).collect();
        let enc = loft_body::<Interval>(&secs, &places, 2, Tol::witness());
        let enc = match enc { Ok(e) => e, Err(e) => { println!("{name}: interval refuses {e:?}"); continue; } };
        println!("{name}: interval params {:?}", enc.section_params);
        for t in [box_.lo(), 0.0, box_.hi()] {
            let secs: Vec<_> = sizes.iter().map(|s| tri(t, *s)).collect();
            let places: Vec<_> = degs.iter().map(|d| fan(*d)).collect();
            let m = loft_body::<f64>(&secs, &places, 2, Tol::witness()).expect("member lofts");
            let miss = misses(&enc, &m);
            println!("{name}: t={t:e} member params {:?}; {} of {} wall values outside, first {:?}",
                m.section_params, miss.len(), wall_values(&m, |x| x).len(), miss.first());
        }
    }
}

fn tri_y<T: Real>(y: T, s: f64) -> Vec<ProfileLoop<T>> {
    let p = |x: T, y: T| (Point2::new(x, y), T::zero());
    vec![bulge_loop(vec![p(T::zero(), y), p(T::from_f64(-s), T::from_f64(1.0)), p(T::from_f64(-s), T::from_f64(-1.0))])]
}

/// Row A sits on the fan axis at local (0, t + i*delta): every member's row moves by delta per
/// section, but at Interval the chords' lower ends are 0 (the boxes overlap), so the row abstains.
#[test]
fn r2_overlapping_row_abstains_over_a_box() {
    let degs = [0.0, 20.0, 40.0];
    let sizes = [1.0, 1.0, 3.0];
    let e = Tol::witness().eps(); println!("eps {e:e}"); for (w, delta) in [(e/256.0, e/512.0), (e/64.0, e/128.0), (e/16.0, e/32.0), (e*4.0, e*2.0), (e*64.0, e*32.0)] {
        let secs: Vec<_> = sizes.iter().enumerate().map(|(i, s)| tri_y(Interval::from_bounds(-w, w) + Interval::from_f64(i as f64 * delta), *s)).collect();
        let places: Vec<Affine3<Interval>> = degs.iter().map(|d| fan(*d).map(Interval::from_f64)).collect();
        let enc = match loft_body::<Interval>(&secs, &places, 2, Tol::witness()) {
            Ok(e) => e,
            Err(e) => { println!("w={w:e} d={delta:e}: interval refuses {e:?}"); continue; }
        };
        println!("w={w:e} d={delta:e}: interval params {:?}", enc.section_params);
        for t in [-w, 0.0, w] {
            let secs: Vec<_> = sizes.iter().enumerate().map(|(i, s)| tri_y(t + i as f64 * delta, *s)).collect();
            let places: Vec<_> = degs.iter().map(|d| fan(*d)).collect();
            match loft_body::<f64>(&secs, &places, 2, Tol::witness()) {
                Ok(m) => {
                    let miss = misses(&enc, &m);
                    println!("   t={t:e} member params {:?}; {} of {} wall values outside, worst {:?}",
                        m.section_params, miss.len(), wall_values(&m, |x| x).len(),
                        miss.iter().max_by(|a, b| (a.2 - (a.1+a.3)/2.0).abs().total_cmp(&(b.2 - (b.1+b.3)/2.0).abs())));
                }
                Err(e) => println!("   t={t:e} member refuses {e:?}"),
            }
        }
    }
}

fn line_curve<T: Real + geom_core::spline::SpanLocate>(a: geom_core::Point3<T>, b: geom_core::Point3<T>) -> geom::NurbsCurve3<T> {
    let two = core::num::NonZeroUsize::new(2).unwrap();
    geom::NurbsCurve3::new(
        geom_core::spline::Knots::new(vec![(T::zero(), two), (T::one(), two)], 1).unwrap(),
        vec![a, b], vec![T::one(); 2]).unwrap()
}

/// Public `skin` at Interval: row 1's boxes overlap across sections, so its total's lower end is 0
/// and it abstains; every member's row 1 moves by delta, so every f64 member counts it.
#[test]
fn r2_skin_abstains_on_an_undecided_row() {
    use geom_core::Point3;
    let zs = [0.0, 1.0, 3.0];
    let w = 1e-3; let delta = 5e-4;
    let i = Interval::from_f64;
    let secs: Vec<_> = zs.iter().enumerate().map(|(k, z)| line_curve(
        Point3::new(i(0.0), i(0.0), i(*z)),
        Point3::new(i(1.0), Interval::from_bounds(-w, w) + i(k as f64 * delta), i(0.0)))).collect();
    let enc = sweep::skin(&secs, 2).expect("interval skin");
    for t in [-w, 0.0, w] {
        let secs: Vec<_> = zs.iter().enumerate().map(|(k, z)| line_curve(
            Point3::new(0.0, 0.0, *z), Point3::new(1.0, t + k as f64 * delta, 0.0))).collect();
        let m = sweep::skin(&secs, 2).expect("f64 skin");
        let mut out = 0; let mut worst = 0.0f64;
        for (pe, pm) in enc.control().iter().zip(m.control()) {
            for (e, v) in [(pe.x, pm.x), (pe.y, pm.y), (pe.z, pm.z)] {
                if !(e.lo() <= v && v <= e.hi()) { out += 1; worst = worst.max((v - e.lo()).abs().min((v - e.hi()).abs())); }
            }
        }
        println!("t={t:e}: {out} of {} pole coords outside the Interval skin; worst distance {worst:e}", 3 * m.control().len());
        let mid = enc.control()[1]; let mm = m.control()[1];
        println!("   pole 1: interval z [{}, {}] member z {}", mid.z.lo(), mid.z.hi(), mm.z);
    }
}

fn quad<T: Real>(h: T) -> Vec<ProfileLoop<T>> {
    let corner = |x: T, y: f64| (Point2::new(x, T::from_f64(y)), T::zero());
    vec![bulge_loop(vec![corner(T::from_f64(-1.0), -1.0), corner(T::one(), -1.0), corner(h, 1.0), corner(T::from_f64(-1.0), 1.0)])]
}
fn at_height<T: Real>(z: T) -> Affine3<T> { Affine3::translation(Vec3::new(T::zero(), T::zero(), z)) }

#[test]
fn r2_unbounded_inputs() {
    let i = Interval::from_f64;
    for (what, h) in [("point inf", i(f64::INFINITY)), ("[1.2, inf]", Interval::from_bounds(1.2, f64::INFINITY)),
                      ("[1.2, MAX]", Interval::from_bounds(1.2, f64::MAX)), ("NaI", i(f64::NAN))] {
        let r = loft_body::<Interval>(&[quad(i(1.2)), quad(h)], &[at_height(i(0.0)), at_height(i(1.0))], 1, Tol::witness());
        println!("section {what}: {:?}", r.map(|_| ()));
        let r = loft_body::<Interval>(&[quad(i(1.2)), quad(i(1.2))], &[at_height(i(0.0)), at_height(h)], 1, Tol::witness());
        println!("placement {what}: {:?}", r.map(|_| ()));
    }
    let r = loft_body::<f64>(&[quad(1.2), quad(f64::INFINITY)], &[at_height(0.0), at_height(1.0)], 1, Tol::witness());
    println!("f64 section inf: {:?}", r.map(|_| ()));
    let r = loft_body::<f64>(&[quad(1.2), quad(1.2)], &[at_height(0.0), at_height(f64::NAN)], 1, Tol::witness());
    println!("f64 placement nan: {:?}", r.map(|_| ()));
}
