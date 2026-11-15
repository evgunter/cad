//! Reviewer probe (r1): sweep doors over a parameter box, at
//! `Sym<Interval>` and at bare `Interval`. Prints, per case, whether it
//! builds and the width each face's stored directions carry, and checks
//! that every member's `f64` datum lies in the boxed enclosure. Run on
//! the merge base and on the head, it is a differential.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::ops::Deref;

use geom_core::sym::with_session;
use geom_core::{
    Bounds, Decide, Interval, ParamSymbol, Point2, Point3, Real, RigidMotion, Sym, SymBudget, Tol,
    UnitVec3, Vec2, Vec3,
};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

type Datum<T> = Vec<(String, Vec3<T>, Option<Vec3<T>>)>;

fn data<T: Real>(body: &Body<T>) -> Datum<T> {
    body.faces()
        .filter_map(|(fk, _)| {
            let kind = format!("{:?}", topo::query::face_surface_kind(body, fk));
            topo::readback::face_pose(body, fk)
                .ok()
                .map(|p| (kind, p.axis, p.u_ref))
        })
        .collect()
}

fn p<T: Real>(x: f64, y: f64) -> Point2<T> {
    Point2::new(T::from_f64(x), T::from_f64(y))
}

fn rect<T: Real>(x0: f64, y0: f64, x1: T, y1: f64) -> ProfileLoop<T> {
    let z = T::zero;
    bulge_loop(vec![
        (p(x0, y0), z()),
        (Point2::new(x1, T::from_f64(y0)), z()),
        (Point2::new(x1, T::from_f64(y1)), z()),
        (p(x0, y1), z()),
    ])
}

fn hole<T: Real>() -> ProfileLoop<T> {
    let z = T::zero;
    bulge_loop(vec![
        (p(0.25, 0.25), z()),
        (p(0.25, 0.75), z()),
        (p(0.75, 0.75), z()),
        (p(0.75, 0.25), z()),
    ])
}

fn circle<T: Real>(cx: f64, cy: f64, r: T) -> ProfileLoop<T> {
    RawLoop::new([(
        Point2::new(T::from_f64(cx) + r, T::from_f64(cy)),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: p(cx, cy),
            radius: r,
            sweep: T::from_f64(core::f64::consts::TAU),
        }),
    )])
}

fn turned<T: Real>(angle: T) -> SketchPlane<T> {
    SketchPlane::from_frame(RigidMotion::rotation(UnitVec3::exact_x(), angle).unwrap())
}

fn sq<T: Real>(a: f64) -> ProfileLoop<T> {
    rect(-a, -a, T::from_f64(a), a)
}

fn short(e: impl core::fmt::Debug) -> String {
    let t = format!("{e:?}");
    t[..t.len().min(330)].to_string()
}

/// One case built with its one parameter `a`.
fn build<T: Decide + topo::AtRestPolicy>(case: &str, a: T) -> Result<Datum<T>, String>
where
    T::Leaf: Deref<Target = Body<T>>,
{
    let one = T::one;
    let lit = T::from_f64;
    let prof = |plane: SketchPlane<T>, loops: Vec<ProfileLoop<T>>| {
        Profile::new(plane, loops)
            .validate(tol())
            .map_err(|e| format!("PROFILE {}", short(e)))
    };
    let y_axis = || RevolveAxis {
        origin: p::<T>(0.0, 0.0),
        dir: Vec2::new(T::zero(), T::one()),
    };
    let dist = |depth: T, side| Extrusion::Distance { depth, side };
    let ex = |pr: profile::ValidatedProfile<T>, e: Extrusion<T>| -> Result<Datum<T>, String> {
        let b: sweep::Extruded<T> = extrude(&pr, e, tol()).map_err(short)?;
        let body: &Body<T> = b.body.deref();
        Ok(data::<T>(body))
    };
    let rv = |pr: profile::ValidatedProfile<T>, r: Revolution<T>| -> Result<Datum<T>, String> {
        let b: sweep::Revolved<T> = revolve(&pr, y_axis(), r, tol()).map_err(short)?;
        let body: &Body<T> = b.body.deref();
        Ok(data::<T>(body))
    };
    match case {
        "depth along" => ex(
            prof(SketchPlane::xy(), vec![rect(0.0, 0.0, one(), 1.0)])?,
            dist(a, ExtrudeSide::Along),
        ),
        "depth against" => ex(
            prof(SketchPlane::xy(), vec![rect(0.0, 0.0, one(), 1.0)])?,
            dist(a, ExtrudeSide::Against),
        ),
        "depth along, hole" => ex(
            prof(SketchPlane::xy(), vec![rect(0.0, 0.0, one(), 1.0), hole()])?,
            dist(a, ExtrudeSide::Along),
        ),
        "depth along, turned frame 0.3" => ex(
            prof(turned(lit(0.3)), vec![rect(0.0, 0.0, one(), 1.0)])?,
            dist(a, ExtrudeSide::Along),
        ),
        "vector +z boxed" => ex(
            prof(SketchPlane::xy(), vec![rect(0.0, 0.0, one(), 1.0)])?,
            Extrusion::Vector(Vec3::new(T::zero(), T::zero(), a)),
        ),
        "vector -z boxed" => ex(
            prof(SketchPlane::xy(), vec![rect(0.0, 0.0, one(), 1.0)])?,
            Extrusion::Vector(Vec3::new(T::zero(), T::zero(), T::zero() - a)),
        ),
        "vector exact (0,0,1.5)" => ex(
            prof(SketchPlane::xy(), vec![rect(0.0, 0.0, a, 1.0)])?,
            Extrusion::Vector(Vec3::new(T::zero(), T::zero(), lit(1.5))),
        ),
        "frame turned by box" => ex(
            prof(turned(a), vec![rect(0.0, 0.0, one(), 1.0)])?,
            dist(one(), ExtrudeSide::Along),
        ),
        "width box" => ex(
            prof(SketchPlane::xy(), vec![rect(0.0, 0.0, a, 1.0)])?,
            dist(one(), ExtrudeSide::Along),
        ),
        "hole radius box" => ex(
            prof(
                SketchPlane::xy(),
                vec![rect(-1.0, -1.0, one(), 1.0), circle(0.0, 0.0, a)],
            )?,
            dist(one(), ExtrudeSide::Along),
        ),
        "disc radius box" => ex(
            prof(SketchPlane::xy(), vec![circle(0.0, 0.0, a)])?,
            dist(one(), ExtrudeSide::Along),
        ),
        "revolve angle box" => rv(
            prof(SketchPlane::xy(), vec![rect(1.0, 0.0, lit(1.5), 1.0)])?,
            Revolution::Partial(a),
        ),
        "revolve angle box neg" => rv(
            prof(SketchPlane::xy(), vec![rect(1.0, 0.0, lit(1.5), 1.0)])?,
            Revolution::Partial(T::zero() - a),
        ),
        "revolve width box, quarter" => rv(
            prof(SketchPlane::xy(), vec![rect(1.0, 0.0, a, 1.0)])?,
            Revolution::Partial(lit(core::f64::consts::FRAC_PI_2)),
        ),
        "revolve full width box" => rv(
            prof(SketchPlane::xy(), vec![rect(1.0, 0.0, a, 1.0)])?,
            Revolution::Full,
        ),
        "revolve frame turned by box" => rv(
            prof(turned(a), vec![rect(1.0, 0.0, lit(1.5), 1.0)])?,
            Revolution::Partial(lit(1.0)),
        ),
        "loft top height box" => loft_body(
            &[vec![sq::<T>(1.0)], vec![sq::<T>(0.6)]],
            &[
                SketchPlane::xy(),
                SketchPlane::xy_at(Point3::new(T::zero(), T::zero(), a)),
            ],
            1,
            tol(),
        )
        .map(|l: sweep::Lofted<T>| {
            let body: &Body<T> = l.body.deref();
            data::<T>(body)
        })
        .map_err(short),
        "loft top tilt box" => loft_body(
            &[vec![sq::<T>(1.0)], vec![sq::<T>(0.6)]],
            &[
                SketchPlane::xy(),
                SketchPlane::from_frame(
                    RigidMotion::rotation(UnitVec3::exact_x(), a)
                        .unwrap()
                        .translated(Vec3::new(T::zero(), T::zero(), lit(1.5)))
                        .unwrap(),
                ),
            ],
            1,
            tol(),
        )
        .map(|l: sweep::Lofted<T>| {
            let body: &Body<T> = l.body.deref();
            data::<T>(body)
        })
        .map_err(short),
        other => panic!("no case {other}"),
    }
}

fn w<T: Bounds>(x: T) -> f64 {
    x.hi() - x.lo()
}

fn widths<T: Real + Bounds>(d: &Datum<T>) -> String {
    let mut out = String::new();
    for (i, (kind, axis, u)) in d.iter().enumerate() {
        let n = axis.norm();
        let aw = w(axis.x).max(w(axis.y)).max(w(axis.z));
        let (uw, un) = match u {
            Some(u) => {
                let m = u.norm();
                (w(u.x).max(w(u.y)).max(w(u.z)), format!("[{:.3e},{:.3e}]", m.lo() - 1.0, m.hi() - 1.0))
            }
            None => (0.0, "-".into()),
        };
        out += &format!(
            "\n      f{i} {kind}: axis w {aw:.2e} |axis|-1 [{:.3e},{:.3e}]  u w {uw:.2e} |u|-1 {un}",
            n.lo() - 1.0,
            n.hi() - 1.0
        );
    }
    out
}

/// Worst escape of a member's datum from the boxed enclosure (0 when
/// contained), and the truth's spread between members.
fn escape<T: Real + Bounds>(held: &Datum<T>, members: &[Datum<f64>]) -> (f64, f64, bool) {
    let mut worst = 0.0_f64;
    let mut spread = 0.0_f64;
    let mut shape = true;
    for m in members {
        if m.len() != held.len() {
            shape = false;
            continue;
        }
        for ((_, ha, hu), (_, ma, mu)) in held.iter().zip(m) {
            let mut one = |h: Vec3<T>, v: Vec3<f64>| {
                for (h, v) in [(h.x, v.x), (h.y, v.y), (h.z, v.z)] {
                    worst = worst.max(h.lo() - v).max(v - h.hi());
                }
            };
            one(*ha, *ma);
            if let (Some(hu), Some(mu)) = (hu, mu) {
                one(*hu, *mu);
            }
        }
        for ((_, a0, u0), (_, a1, u1)) in members[0].iter().zip(m) {
            spread = spread.max((*a0 - *a1).norm());
            if let (Some(u0), Some(u1)) = (u0, u1) {
                spread = spread.max((*u0 - *u1).norm());
            }
        }
    }
    (worst, spread, shape)
}

#[test]
fn r1_box_probe() {
    // (case, lo, hi)
    let eps = tol().eps();
    let cases: Vec<(&str, f64, f64)> = vec![
        ("depth along", 0.95, 1.05),
        ("depth against", 0.95, 1.05),
        ("depth along, hole", 0.95, 1.05),
        ("depth along, turned frame 0.3", 0.95, 1.05),
        ("vector +z boxed", 0.95, 1.05),
        ("vector -z boxed", 0.95, 1.05),
        ("vector exact (0,0,1.5)", 1.0, 1.0),
        ("frame turned by box", 0.30, 0.32),
        ("width box", 0.95, 1.05),
        ("width box", 1.0 - eps / 64.0, 1.0 + eps / 64.0),
        ("hole radius box", 0.49, 0.51),
        ("hole radius box", 0.5 - eps / 64.0, 0.5 + eps / 64.0),
        ("disc radius box", 0.5 - eps / 64.0, 0.5 + eps / 64.0),
        ("revolve angle box", 1.0, 1.02),
        ("revolve angle box neg", 1.0, 1.02),
        ("revolve angle box", 1.0 - eps / 64.0, 1.0 + eps / 64.0),
        ("revolve width box, quarter", 1.5 - eps / 64.0, 1.5 + eps / 64.0),
        ("revolve width box, quarter", 1.45, 1.55),
        ("revolve full width box", 1.5 - eps / 64.0, 1.5 + eps / 64.0),
        ("revolve full width box", 1.45, 1.55),
        ("revolve frame turned by box", 0.30, 0.32),
        ("loft top height box", 0.95, 1.05),
        ("loft top height box", 1.0 - eps / 64.0, 1.0 + eps / 64.0),
        ("loft top tilt box", 0.10, 0.12),
        ("loft top tilt box", 0.1 - eps / 64.0, 0.1 + eps / 64.0),
    ];
    for (i, (case, lo, hi)) in cases.into_iter().enumerate() {
        let members: Vec<Datum<f64>> = [0.0, 0.25, 0.5, 0.75, 1.0]
            .iter()
            .filter_map(|s| build::<f64>(case, lo + (hi - lo) * s).ok())
            .collect();
        println!("BOX {case} [{lo}, {hi}] (f64 members built: {}/5)", members.len());
        // Sym<Interval>
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        with_session(budget, || {
            let a: Sym<Interval> =
                Sym::param(ParamSymbol::new(0x9000 + i as u64), Interval::from_bounds(lo, hi));
            match build::<Sym<Interval>>(case, a) {
                Ok(d) => {
                    let (esc, spread, shape) = escape(&d, &members);
                    println!(
                        "   SYM ok faces {} escape {esc:.3e} truth-spread {spread:.3e} same-shape {shape}{}",
                        d.len(),
                        widths(&d)
                    );
                }
                Err(e) => println!("   SYM REFUSED {e}"),
            }
        });
        match build::<Interval>(case, Interval::from_bounds(lo, hi)) {
            Ok(d) => {
                let (esc, spread, shape) = escape(&d, &members);
                println!(
                    "   INT ok faces {} escape {esc:.3e} truth-spread {spread:.3e} same-shape {shape}{}",
                    d.len(),
                    widths(&d)
                );
            }
            Err(e) => println!("   INT REFUSED {e}"),
        }
    }
}
