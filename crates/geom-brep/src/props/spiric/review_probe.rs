//! Reviewer probes (PR 4554): the cap's vector-area term against an
//! oracle built from `Curve3::Spiric`'s own `eval`/`deriv` (Gauss–Legendre
//! on ½∫(P − ref) × P′ dt), random frames, both signs of `d`, both
//! traversals, partial arcs; and near-tangent cuts.
#![allow(clippy::unwrap_used, clippy::panic, clippy::cast_precision_loss)]
use super::*;
use geom::Curve3;
use geom_core::{Point3, Vec3};

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, a: f64, b: f64) -> f64 { a + (b - a) * self.next() }
}

fn unit(v: Vec3<f64>) -> Vec3<f64> { v * (1.0 / v.dot(v).sqrt()) }

/// ½∫(P − ref) × P′ dt by 5-point Gauss–Legendre over `n` pieces.
fn oracle(c: &Curve3<f64>, refp: Point3<f64>, t0: f64, t1: f64, n: usize) -> [f64; 3] {
    let xs = [0.0, -0.5384693101056831, 0.5384693101056831, -0.906179845938664, 0.906179845938664];
    let ws = [0.5688888888888889, 0.47862867049936647, 0.47862867049936647, 0.23692688505618908, 0.23692688505618908];
    let h = (t1 - t0) / n as f64;
    let mut acc = [0.0; 3];
    for i in 0..n {
        let m = t0 + h * (i as f64 + 0.5);
        for (x, w) in xs.iter().zip(ws) {
            let t = m + 0.5 * h * x;
            let p = c.eval(t) - refp;
            let d = c.deriv(t);
            let cr = p.cross(d);
            acc[0] += 0.25 * h * w * cr.x;
            acc[1] += 0.25 * h * w * cr.y;
            acc[2] += 0.25 * h * w * cr.z;
        }
    }
    acc
}

fn q(c: &Curve3<f64>, refp: Point3<f64>, t0: f64, t1: f64, forward: bool) -> SpiricEdgeQ {
    let Curve3::Spiric { center, axis, u_ref, major_radius, minor_radius, offset } = *c else { panic!() };
    let iv = |v: Vec3<f64>| [pt(v.x), pt(v.y), pt(v.z)];
    SpiricEdgeQ {
        centre: iv(center - refp),
        axis: iv(axis),
        normal: iv(u_ref),
        major: pt(major_radius),
        minor: pt(minor_radius),
        offset: pt(offset),
        t0: pt(t0),
        t1: pt(t1),
        forward,
        wall: None,
    }
}

fn random_case(g: &mut Lcg, tangent_gap: Option<f64>) -> (Curve3<f64>, Point3<f64>, f64, f64) {
    let big = g.range(0.5, 3.0);
    let r = big * g.range(0.1, 0.8);
    let gap = big - r;
    let d = match tangent_gap {
        None => gap * g.range(0.02, 0.98) * if g.next() < 0.5 { -1.0 } else { 1.0 },
        // f_min = √((R−r)² − d²) = target.
        Some(fm) => (gap * gap - fm * fm).sqrt() * if g.next() < 0.5 { -1.0 } else { 1.0 },
    };
    let axis = unit(Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)));
    let w = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
    let u_ref = unit(w - axis * w.dot(axis));
    let center = Point3::new(g.range(-2.0, 2.0), g.range(-2.0, 2.0), g.range(-2.0, 2.0));
    let c = Curve3::Spiric { center, axis, u_ref, major_radius: big, minor_radius: r, offset: d };
    let refp = Point3::new(g.range(-2.0, 2.0), g.range(-2.0, 2.0), g.range(-2.0, 2.0));
    let t0 = g.range(-4.0, 4.0);
    let t1 = t0 + g.range(0.05, 6.2);
    (c, refp, t0, t1)
}

#[test]
fn review_cap_vector_area_encloses_an_eval_deriv_oracle() {
    let mut g = Lcg(0x4554);
    let mut worst = 0.0_f64;
    for case in 0..400 {
        let (c, refp, t0, t1) = random_case(&mut g, None);
        let want = oracle(&c, refp, t0, t1, 4000);
        for forward in [true, false] {
            let va = spiric_vector_area(&q(&c, refp, t0, t1, forward), 64);
            let sgn = if forward { 1.0 } else { -1.0 };
            for k in 0..3 {
                let w = sgn * want[k];
                assert!(va[k].is_certified(), "case {case}: refused {va:?}");
                let miss = (va[k].lo() - w).max(w - va[k].hi());
                worst = worst.max(miss);
                assert!(miss <= 1e-12, "case {case} fwd {forward} k {k}: [{}, {}] misses {w} by {miss}; {c:?} t {t0}..{t1}", va[k].lo(), va[k].hi());
            }
        }
    }
    println!("[review] worst miss (≤0 is inside) {worst:e}");
}

#[test]
fn review_near_tangent_at_1024_pieces() {
    for fm in [1e-2, 1e-3, 1e-4, 1e-6, 1e-8] {
        let mut g = Lcg(7);
        for _ in 0..20 {
            let (c, refp, _, _) = random_case(&mut g, Some(fm));
            // Spans across t = π, where f is smallest.
            let (t0, t1) = (2.0, 4.5);
            let want = oracle(&c, refp, t0, t1, 200_000);
            let va = spiric_vector_area(&q(&c, refp, t0, t1, true), 1024);
            let wid = (0..3).map(|k| va[k].hi() - va[k].lo()).fold(0.0, f64::max);
            let ok = (0..3).all(|k| va[k].is_certified());
            let inside = (0..3).all(|k| va[k].lo() <= want[k] + 1e-9 && want[k] - 1e-9 <= va[k].hi());
            println!("[review] f_min {fm:e}: certified {ok} width {wid:e} contains-oracle(±1e-9) {inside}");
            if ok { assert!(inside); }
        }
    }
}

#[test]
fn review_pieces_tile_exactly_and_non_thin_endpoints() {
    // A non-thin endpoint bracket must still enclose; the pad is the
    // only thing covering the bracket.
    let mut g = Lcg(99);
    for case in 0..200 {
        let (c, refp, t0, t1) = random_case(&mut g, None);
        let hw = 1e-6;
        let mut e = q(&c, refp, t0, t1, true);
        e.t0 = Interval::from_bounds(t0 - hw, t0 + hw);
        e.t1 = Interval::from_bounds(t1 - hw, t1 + hw);
        let va = spiric_vector_area(&e, 64);
        for (a, b) in [(t0 - hw, t1 - hw), (t0 + hw, t1 + hw), (t0 - hw, t1 + hw), (t0 + hw, t1 - hw)] {
            let want = oracle(&c, refp, a, b, 2000);
            for k in 0..3 {
                assert!(va[k].lo() <= want[k] + 1e-12 && want[k] - 1e-12 <= va[k].hi(),
                    "case {case} k {k}: [{}, {}] vs {}", va[k].lo(), va[k].hi(), want[k]);
            }
        }
    }
}

fn wall_face(spirics: Vec<SpiricEdgeQ>, meridians: Vec<MeridianQ>) -> SpiricFaceQ {
    SpiricFaceQ {
        closed_va: [Interval::zero(); 3],
        spirics,
        lever: [Interval::zero(); 3],
        perimeter: 20.0,
        wall: Some(WallChartQ { major: pt(1.2), minor: pt(0.275), rims: vec![], meridians }),
    }
}

fn wall_edge(d: f64, t0: f64, t1: f64) -> SpiricEdgeQ {
    SpiricEdgeQ {
        centre: [Interval::zero(); 3],
        axis: [pt(0.0), pt(0.0), pt(1.0)],
        normal: [pt(1.0), pt(0.0), pt(0.0)],
        major: pt(1.2),
        minor: pt(0.275),
        offset: pt(d),
        t0: pt(t0),
        t1: pt(t1),
        forward: true,
        wall: Some(WallImageQ { sense: 1.0, v0: Interval::zero(), env: Interval::zero() }),
    }
}

#[test]
fn review_the_lift_closure_check_refuses_open_and_admits_closed() {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let eps = geom_core::Tol::witness().eps();
    let tau = 2.0 * core::f64::consts::PI;
    let run = |f: &SpiricFaceQ| spiric_face_rounds::<f64>(f, eps, band, RoundWindow::SCHEDULE);
    // Closed: a whole oval (Δu = 0, Δv = 2π) and a meridian back.
    let closed = wall_face(vec![wall_edge(0.05, 0.0, tau)], vec![MeridianQ { dv: pt(-tau), env: Interval::zero() }]);
    let c = run(&closed);
    println!("[review] closed: {:?}", c.as_ref().map(|_| "ok"));
    assert!(c.is_ok(), "{c:?}");
    // Open in v: the oval alone.
    let open_v = wall_face(vec![wall_edge(0.05, 0.0, tau)], vec![]);
    let o = run(&open_v);
    println!("[review] open v: {o:?}");
    assert!(matches!(o, Err(PropsError::NotIsoRectangle { .. })), "{o:?}");
    // Open in u: an asymmetric arc (Δu ≈ 0.0107) closed in v by a meridian.
    let open_u = wall_face(vec![wall_edge(0.05, 0.3, 2.1)], vec![MeridianQ { dv: pt(-1.8), env: Interval::zero() }]);
    let o = run(&open_u);
    println!("[review] open u: {o:?}");
    assert!(matches!(o, Err(PropsError::NotIsoRectangle { .. })), "{o:?}");
}

#[test]
fn review_near_tangent_cap_schedule() {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let eps = geom_core::Tol::witness().eps();
    let tau = 2.0 * core::f64::consts::PI;
    for fm in [1e-1, 1e-2, 1e-3, 1e-4] {
        let (big, r) = (1.0, 0.5);
        let d = (0.25_f64 - fm * fm).sqrt();
        let e = SpiricEdgeQ {
            centre: [Interval::zero(); 3],
            axis: [pt(0.0), pt(0.0), pt(1.0)],
            normal: [pt(1.0), pt(0.0), pt(0.0)],
            major: pt(big), minor: pt(r), offset: pt(d),
            t0: pt(-core::f64::consts::PI), t1: pt(core::f64::consts::PI),
            forward: true, wall: None,
        };
        let face = SpiricFaceQ { closed_va: [Interval::zero(); 3], spirics: vec![e], lever: [pt(0.1), pt(0.0), pt(0.0)], perimeter: 10.0, wall: None };
        let _ = tau;
        let out = spiric_face_rounds::<f64>(&face, eps, band, RoundWindow::SCHEDULE);
        println!("[review] cap f_min {fm:e}: {:?}", out.map(|o| match o { RoundOutcome::Converged(b) => format!("converged area {:?}", b.area), RoundOutcome::Open{round, ..} => format!("open {round}") }));
    }
}
