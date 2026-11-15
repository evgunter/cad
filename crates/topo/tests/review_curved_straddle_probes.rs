//! Reviewer probes for PR 6979 (BOXFINISH curved-straddle): bodies with
//! a moving curved rim built by hand over a box (a cylinder, a cone
//! frustum, a sphere cap), point containment and aimed rays read over
//! the box against each member's own `f64` reading. A wrong answer is a
//! definite boxed answer some member definitely contradicts.
//!
//! Every row prints its tally and is `#[ignore]`d: run with
//! `--run-ignored only`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Decide, Interval, ParamSymbol, Point3, Real, Sym, SymBudget, Tol, Vec3};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{AtRestPolicy, Body, FaceKey, SolidContainment};

type Boxed = Sym<Interval>;

/// The wall a revolved body's two rims bound.
#[derive(Clone, Copy, Debug)]
enum Kind {
    /// A cylinder of radius `a`, from `z = 0` to `z = b`.
    Cylinder,
    /// A frustum of the cone with apex `(0, 0, a)` and `tan(half angle)
    /// = 0.5`, from `z = 1` to `z = b`.
    Frustum,
    /// The sphere of radius `a` about the origin below the plane
    /// `z = b`, and its disc.
    SphereCap,
}

const K: f64 = 0.5;

fn rims<T: Real>(kind: Kind, a: T, b: T) -> [(T, T); 2] {
    let (zero, one) = (T::zero(), T::one());
    match kind {
        Kind::Cylinder => [(zero, a), (b, a)],
        Kind::Frustum => {
            let k = T::from_f64(K);
            [(one, (one - a) * k), (b, (b - a) * k)]
        }
        Kind::SphereCap => [(b, (a * a - b * b).sqrt()), (b, (a * a - b * b).sqrt())],
    }
}

fn body<T: Decide + AtRestPolicy>(kind: Kind, a: T, b: T, tol: Tol) -> (Body<T>, FaceKey) {
    let (zero, one) = (T::zero(), T::one());
    let (x, z) = (Vec3::new(one, zero, zero), Vec3::new(zero, zero, one));
    let [(z0, r0), (z1, r1)] = rims(kind, a, b);
    let mut body = Body::<T>::new();
    let seed = body.mvfs(Point3::new(r0, zero, z0), true).unwrap();
    let wall = match kind {
        Kind::Cylinder => Surface::Cylinder {
            origin: Point3::origin(),
            axis: z,
            radius: a,
            u_ref: x,
        },
        Kind::Frustum => Surface::Cone {
            apex: Point3::new(zero, zero, a),
            axis: z,
            half_angle: T::from_f64(K.atan()),
            u_ref: x,
        },
        Kind::SphereCap => Surface::Sphere {
            center: Point3::origin(),
            radius: a,
            axis: z,
            u_ref: x,
        },
    };
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: wall,
            sense: true,
        },
    )
    .unwrap();
    let rim = |at: T, r: T, axis: Vec3<T>| {
        let circle = Curve3::circle(Point3::new(zero, zero, at), axis, r, x);
        EdgeCurveSpec::arc_of_circle(circle, T::zero(), T::tau()).unwrap()
    };
    let cap = |at: T, normal: Vec3<T>| FaceSurface::New {
        surface: Surface::Plane {
            origin: Point3::new(zero, zero, at),
            normal,
            u_ref: x,
        },
        sense: true,
    };
    if let Kind::SphereCap = kind {
        body.mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            rim(z0, r0, z * -one),
            cap(z0, z),
            tol,
        )
        .unwrap();
        return (body, seed.face);
    }
    let bottom = body
        .mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            rim(z0, r0, z),
            cap(z0, z * -one),
            tol,
        )
        .unwrap();
    let seam = body
        .mev_line(
            MevSite::Fan {
                he1: bottom.he_plus,
                he2: bottom.he_plus,
            },
            Point3::new(r1, zero, z1),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: seam.he_minus,
            he2: seam.he_minus,
        },
        rim(z1, r1, z * -one),
        cap(z1, z),
        tol,
    )
    .unwrap();
    (body, seed.face)
}

fn over<R>(a: (f64, f64), b: (f64, f64), read: impl FnOnce(Boxed, Boxed, Band, Tol) -> R) -> R {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the witness band");
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let axis = |name: &str, (lo, hi): (f64, f64)| {
        if lo == hi {
            Boxed::from_f64(lo)
        } else {
            Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id(name)),
                Interval::from_bounds(lo, hi),
                lo,
                hi,
            )
        }
    };
    geom_core::sym::with_session(budget, || {
        read(axis("review_probe_a", a), axis("review_probe_b", b), band, tol)
    })
    .0
}

fn members((lo, hi): (f64, f64)) -> Vec<f64> {
    if lo == hi {
        return vec![lo];
    }
    [0.0, 1e-3, 0.25, 0.5, 0.75, 1.0 - 1e-3, 1.0]
        .iter()
        .map(|u| lo + u * (hi - lo))
        .collect()
}

/// Nominal values and the boxes driven over each.
fn cases(kind: Kind) -> (f64, f64) {
    match kind {
        Kind::Cylinder => (1.0, 1.0),
        Kind::Frustum => (0.0, 2.0),
        Kind::SphereCap => (1.0, 0.3),
    }
}

fn boxes(kind: Kind) -> Vec<((f64, f64), (f64, f64))> {
    let (a, b) = cases(kind);
    let mut out = Vec::new();
    for w in [1e-7, 1e-4, 1e-2, 0.1] {
        let s = |m: f64| (m - w * m.abs().max(1.0), m + w * m.abs().max(1.0));
        out.push((s(a), (b, b)));
        out.push(((a, a), s(b)));
        out.push((s(a), s(b)));
    }
    out
}

/// Points about the nominal member's rims: on them, off them radially
/// and axially at several scales, at three angles.
fn points(kind: Kind) -> Vec<[f64; 3]> {
    let (a, b) = cases(kind);
    let mut out = Vec::new();
    for (z, r) in rims::<f64>(kind, a, b) {
        for ang in [0.7_f64, 2.3, 4.4] {
            for dr in [0.0, 3e-9, -3e-9, 1e-6, -1e-6, 1e-4, -1e-4, 1e-2, -1e-2, 0.05, -0.05] {
                for dz in [0.0, 3e-9, -3e-9, 1e-6, -1e-6, 1e-3, -1e-3, 0.03, -0.03] {
                    out.push([(r + dr) * ang.cos(), (r + dr) * ang.sin(), z + dz]);
                }
            }
        }
    }
    out
}

fn name(c: &Result<SolidContainment, topo::PointInSolidError>) -> String {
    match c {
        Ok(c) => format!("{c:?}"),
        Err(_) => "Err".into(),
    }
}

fn containment(kind: Kind) {
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    let mut wrong = Vec::new();
    for (ab, bb) in boxes(kind) {
        let pts = points(kind);
        let boxed: Vec<String> = pts
            .chunks(30)
            .flat_map(|chunk| {
                over(ab, bb, |a, b, band, tol| {
                    let (body, _) = body::<Boxed>(kind, a, b, tol);
                    let iv = Boxed::from_f64;
                    chunk
                        .iter()
                        .map(|p| {
                            name(&topo::point_in_solid(
                                &body,
                                Point3::new(iv(p[0]), iv(p[1]), iv(p[2])),
                                band,
                                tol,
                            ))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut each: Vec<Vec<String>> = vec![Vec::new(); pts.len()];
        for &a in &members(ab) {
            for &b in &members(bb) {
                let (body, _) = body::<f64>(kind, a, b, tol);
                for (k, p) in pts.iter().enumerate() {
                    each[k].push(name(&topo::point_in_solid(
                        &body,
                        Point3::new(p[0], p[1], p[2]),
                        band,
                        tol,
                    )));
                }
            }
        }
        for (k, p) in pts.iter().enumerate() {
            let x = &boxed[k];
            let disagree = x != "Err" && each[k].iter().any(|m| m != "Err" && m != x);
            let unanimous = each[k].iter().all(|m| m == &each[k][0]) && each[k][0] != "Err";
            let key = format!(
                "boxed {x} / members {}",
                if unanimous { each[k][0].clone() } else { "mixed".into() }
            );
            *tally.entry(key).or_default() += 1;
            if disagree {
                wrong.push(format!("{kind:?} a {ab:?} b {bb:?} p {p:?}: boxed {x}, members {:?}", each[k]));
            }
        }
    }
    println!("{kind:?} containment, ε {:e}: {tally:#?}", Tol::witness().eps());
    for w in wrong.iter().take(40) {
        println!("WRONG {w}");
    }
    assert!(wrong.is_empty(), "{} wrong containment answers", wrong.len());
}

fn aims(kind: Kind) {
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    let mut wrong = Vec::new();
    let (a0, b0) = cases(kind);
    for (ab, bb) in boxes(kind) {
        let mut draws = Vec::new();
        for (zr, (z, r)) in rims::<f64>(kind, a0, b0).into_iter().enumerate() {
            if zr == 1 && matches!(kind, Kind::SphereCap) {
                break;
            }
            for ang in [0.7_f64, 2.3, 4.4] {
                let (c, s) = (ang.cos(), ang.sin());
                for inset in [0.0, 1e-8, -1e-5, 1e-2, -1e-2] {
                    for dz in [0.0, 1e-8, -1e-5] {
                        let aim = [(r + inset) * c, (r + inset) * s, z + dz];
                        // the caps' normal, the radial, and tilts between,
                        // shallow and near-tangent.
                        for (nr, nz) in [
                            (0.0, 1.0),
                            (0.0, -1.0),
                            (1.0, 0.0),
                            (-1.0, 0.0),
                            (1.0, 1.0),
                            (1.0, -1.0),
                            (1e-3, 1.0),
                            (1.0, 1e-3),
                            (1.0, -1e-3),
                            (-1e-3, 1.0),
                        ] {
                            for nt in [0.0, 0.3] {
                                let n = [nr * c - nt * s, nr * s + nt * c, nz];
                                let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                                draws.push((aim, [n[0] / l, n[1] / l, n[2] / l]));
                            }
                        }
                    }
                }
            }
        }
        let boxed: Vec<String> = draws
            .chunks(60)
            .flat_map(|chunk| {
                over(ab, bb, |a, b, band, tol| {
                    let (body, wall) = body::<Boxed>(kind, a, b, tol);
                    let iv = Boxed::from_f64;
                    chunk
                        .iter()
                        .map(|&(p, n)| {
                            topo::test_support::ray_at(
                                &body,
                                wall,
                                Point3::new(iv(p[0]), iv(p[1]), iv(p[2])),
                                Vec3::new(iv(n[0]), iv(n[1]), iv(n[2])),
                                band,
                            )
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        for (k, r) in boxed.iter().enumerate() {
            let class = if r.contains("Side(In)") {
                wrong.push(format!("{kind:?} a {ab:?} b {bb:?} {:?}: {r}", draws[k]));
                "In".to_string()
            } else if r.contains("Side(Out)") {
                "Out".into()
            } else if let Some(i) = r.find("predicate: Some(\"") {
                let rest = &r[i + 17..];
                format!("aside {}", &rest[..rest.find('"').unwrap_or(rest.len())])
            } else {
                format!("aside {}", &r[..r.len().min(40)])
            };
            *tally.entry(class).or_default() += 1;
        }
    }
    println!("{kind:?} aims, ε {:e}: {tally:#?}", Tol::witness().eps());
    for w in wrong.iter().take(20) {
        println!("WRONG {w}");
    }
    assert!(wrong.is_empty(), "{} wrong sides", wrong.len());
}

/// Each body builds at f64 and reads Out from far away and In at its
/// centre: the builder's own sanity.
#[test]
#[ignore = "reviewer probe"]
fn probe_bodies_are_solids() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for (kind, inside) in [
        (Kind::Cylinder, [0.1, 0.2, 0.5]),
        (Kind::Frustum, [0.1, 0.2, 1.5]),
        (Kind::SphereCap, [0.1, 0.2, -0.3]),
    ] {
        let (a, b) = cases(kind);
        let (body, wall) = body::<f64>(kind, a, b, tol);
        let i = topo::point_in_solid(&body, Point3::new(inside[0], inside[1], inside[2]), band, tol);
        let o = topo::point_in_solid(&body, Point3::new(5.0, 0.1, 0.2), band, tol);
        let s = topo::test_support::each_aim(&body, band);
        println!("{kind:?}: inside {i:?} outside {o:?} aims {s:?}");
        assert_eq!(i, Ok(SolidContainment::In), "{kind:?}");
        assert_eq!(o, Ok(SolidContainment::Out), "{kind:?}");
        let _ = wall;
    }
}

#[test]
#[ignore = "reviewer probe"]
fn probe_containment_cylinder() {
    containment(Kind::Cylinder);
}
#[test]
#[ignore = "reviewer probe"]
fn probe_containment_frustum() {
    containment(Kind::Frustum);
}
#[test]
#[ignore = "reviewer probe"]
fn probe_containment_sphere_cap() {
    containment(Kind::SphereCap);
}
#[test]
#[ignore = "reviewer probe"]
fn probe_aims_cylinder() {
    aims(Kind::Cylinder);
}
#[test]
#[ignore = "reviewer probe"]
fn probe_aims_frustum() {
    aims(Kind::Frustum);
}
#[test]
#[ignore = "reviewer probe"]
fn probe_aims_sphere_cap() {
    aims(Kind::SphereCap);
}
