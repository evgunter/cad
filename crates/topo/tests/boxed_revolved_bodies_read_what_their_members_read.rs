//! **Boxed revolved bodies read what their members read, or nothing**:
//! point containment and aimed rays about a moving curved rim, over a
//! box, against each member's own `f64` reading. The bodies are built
//! by hand through the public Euler doors at the symbolic box scalar: a
//! cylinder, a cone frustum and a sphere cap, each a wall between rims
//! that are whole-turn circles on one seam. A wrong answer is a
//! definite boxed answer that a member definitely contradicts, or a
//! boxed `Side(In)` on a body every member of which is a solid.
//!
//! **These rows are tripwires, held today by refusals elsewhere.** Each
//! cap of these bodies closes on one whole-turn circle, which a planar
//! face's in-face walk refuses over a box (`CorruptLoop`;
//! `work/inside/a-whole-turn-circle-cap-refuses-corrupt-loop-over-a-box-where-f64-answers.md`),
//! so a read that needs a cap's crossing reads nothing, and no mutant
//! of the ray's edge arms (`solid_contain::edge_gap`, the
//! curved arm, `reaches_root_in_band`) turns any of them red today. The
//! day that walk reads, the curved edge's reading is first live over a
//! box, and these are where a wrong answer would show. They were
//! written as the probes of PR 6979's review.

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
            frame: geom::Frame::new(Point3::origin(), z, x),
            radius: a,
        },
        Kind::Frustum => Surface::Cone {
            frame: geom::Frame::new(Point3::new(zero, zero, a), z, x),
            half_angle: T::from_f64(K.atan()),
        },
        Kind::SphereCap => Surface::Sphere {
            frame: geom::Frame::new(Point3::origin(), z, x),
            radius: a,
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
            frame: geom::Frame::new(Point3::new(zero, zero, at), normal, x),
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
        read(
            axis("review_probe_a", a),
            axis("review_probe_b", b),
            band,
            tol,
        )
    })
    .0
}

fn members((lo, hi): (f64, f64)) -> Vec<f64> {
    if lo == hi {
        return vec![lo];
    }
    [0.0, 0.5, 1.0].iter().map(|u| lo + u * (hi - lo)).collect()
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
    let s = |m: f64, w: f64| (m - w * m.abs().max(1.0), m + w * m.abs().max(1.0));
    vec![
        (s(a, 1e-2), (b, b)),
        ((a, a), s(b, 1e-2)),
        (s(a, 1e-2), s(b, 1e-2)),
        (s(a, 1e-7), s(b, 1e-7)),
    ]
}

/// Points about the nominal member's rims: on them, off them radially
/// and axially at several scales, at three angles.
fn points(kind: Kind) -> Vec<[f64; 3]> {
    let (a, b) = cases(kind);
    let mut out = Vec::new();
    for (z, r) in rims::<f64>(kind, a, b) {
        for ang in [0.7_f64] {
            for dr in [0.0, 3e-9, 1e-4, -0.05] {
                for dz in [0.0, 1e-6, -0.03] {
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
                if unanimous {
                    each[k][0].clone()
                } else {
                    "mixed".into()
                }
            );
            *tally.entry(key).or_default() += 1;
            if disagree {
                wrong.push(format!(
                    "{kind:?} a {ab:?} b {bb:?} p {p:?}: boxed {x}, members {:?}",
                    each[k]
                ));
            }
        }
    }
    println!(
        "{kind:?} containment, ε {:e}: {tally:#?}",
        Tol::witness().eps()
    );
    for w in wrong.iter().take(40) {
        println!("WRONG {w}");
    }
    assert!(
        wrong.is_empty(),
        "{} wrong containment answers",
        wrong.len()
    );
}

fn aims(kind: Kind) {
    aims_at(kind, &[0.7]);
}

fn aims_at(kind: Kind, angles: &[f64]) {
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    let mut wrong = Vec::new();
    let (a0, b0) = cases(kind);
    for (ab, bb) in boxes(kind) {
        let mut draws = Vec::new();
        for (zr, (z, r)) in rims::<f64>(kind, a0, b0).into_iter().enumerate() {
            if zr == 1 && matches!(kind, Kind::SphereCap) {
                break;
            }
            for &ang in angles {
                let (c, s) = (ang.cos(), ang.sin());
                for inset in [0.0, 1e-8, -1e-2] {
                    for dz in [0.0, -1e-5] {
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
                            // Every other normal is tilted along the rim.
                            let nt = 0.3 * (draws.len() % 2) as f64;
                            let n = [nr * c - nt * s, nr * s + nt * c, nz];
                            let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                            draws.push((aim, [n[0] / l, n[1] / l, n[2] / l]));
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

/// **Each body is a solid at `f64`**: a point at its middle is `In`
/// and one far off is `Out`. The builder's own check, so the rows
/// below read bodies and not a builder's slip.
#[test]
fn each_body_is_a_solid_at_f64() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for (kind, inside) in [
        (Kind::Cylinder, [0.1, 0.2, 0.5]),
        (Kind::Frustum, [0.1, 0.2, 1.5]),
        (Kind::SphereCap, [0.1, 0.2, -0.3]),
    ] {
        let (a, b) = cases(kind);
        let (body, _) = body::<f64>(kind, a, b, tol);
        let at =
            |p: [f64; 3]| topo::point_in_solid(&body, Point3::new(p[0], p[1], p[2]), band, tol);
        assert_eq!(at(inside), Ok(SolidContainment::In), "{kind:?}");
        assert_eq!(at([5.0, 0.1, 0.2]), Ok(SolidContainment::Out), "{kind:?}");
    }
}

/// **Point containment about a boxed body's rims answers as every
/// member does, or refuses**: points on and off each rim of the nominal
/// member, radially and axially from three bands to a twentieth, over
/// boxes a hundredth wide on either dimension and on both and a
/// ten-millionth wide on both, each against the members at its ends
/// and middle read at `f64`. Red on a definite
/// boxed answer a member definitely contradicts, and on nothing else; a
/// tripwire (module docs).
#[test]
fn containment_about_a_boxed_rim_answers_as_its_members_do() {
    for kind in [Kind::Cylinder, Kind::Frustum, Kind::SphereCap] {
        containment(kind);
    }
}

/// **Aimed rays about a boxed frustum's and a boxed sphere cap's rims
/// read no side no member has**: aims on and beside each rim, about the
/// cap's normal, the radial, their negatives, and tilts between down to
/// a thousandth (shallow to the cap, near-tangent to the wall). The
/// cylinder's aims are `an_aimed_ray_beside_a_boxed_cylinders_moving_rim`'s.
/// Red on a boxed `Side(In)` and on nothing else; a tripwire (module
/// docs).
#[test]
fn aims_about_a_boxed_frustum_and_sphere_cap_read_no_wrong_side() {
    for kind in [Kind::Frustum, Kind::SphereCap] {
        aims(kind);
    }
}

/// **Aimed rays beside the seam's vertices read no side no
/// member has**: where a cylinder's and a frustum's seam meets each
/// rim, a vertex between a curved and a planar face, a tenth of a
/// micron round from it. Red
/// on a boxed `Side(In)` and on nothing else; a tripwire (module docs).
#[test]
fn aims_beside_a_boxed_seams_vertices_read_no_wrong_side() {
    for kind in [Kind::Cylinder, Kind::Frustum] {
        aims_at(kind, &[1e-7]);
    }
}
