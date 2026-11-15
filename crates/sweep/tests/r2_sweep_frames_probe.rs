//! Reviewer probe (review-sweep-frames r2): an `f64` face dump over a
//! battery of sweeps, written to compile against both the merge base
//! and the reviewed head so the two dumps can be diffed. Every planar
//! face prints its outward normal (chart normal folded by the face's
//! sense), its `u_ref`, its origin, and the largest distance of any of
//! its loop vertices from its plane; every curved face its kind. The
//! dump lands at `$R2_DUMP` when set; the row itself asserts only what
//! must hold at either revision: every vertex on its face's plane, and
//! every planar outward normal of unit length.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use std::fmt::Write as _;

use geom::Surface;
use geom_core::{Point2, Point3, RigidMotion, Tol, UnitVec3, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, loft_body, revolve};
use topo::Body;
use topo::entity::LoopBoundary;

fn tol() -> Tol {
    Tol::witness()
}

fn p(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A rectangle `[x0, x1] × [y0, y1]`, counterclockwise unless `cw`.
fn rect(x0: f64, y0: f64, x1: f64, y1: f64, cw: bool) -> ProfileLoop<f64> {
    let mut v = vec![(p(x0, y0), 0.0), (p(x1, y0), 0.0), (p(x1, y1), 0.0), (p(x0, y1), 0.0)];
    if cw {
        v.reverse();
    }
    bulge_loop(v)
}

/// A D: a line and one bulged edge.
fn d_shape(cw: bool) -> ProfileLoop<f64> {
    let mut v = vec![(p(0.0, 0.0), 0.0), (p(2.0, 0.0), 0.6), (p(2.0, 1.0), 0.0), (p(0.0, 1.0), 0.0)];
    if cw {
        // reversed traversal; bulges move with their segment
        v = vec![(p(0.0, 1.0), 0.0), (p(2.0, 1.0), -0.6), (p(2.0, 0.0), 0.0), (p(0.0, 0.0), 0.0)];
    }
    bulge_loop(v)
}

fn tilted() -> SketchPlane<f64> {
    let axis = UnitVec3::new(Vec3::new(0.3, -0.7, 0.648), "probe", geom_core::Band::linear(tol()).unwrap()).unwrap();
    SketchPlane::from_frame(
        RigidMotion::rotation_then_translation(axis, 1.1, Vec3::new(3.0, -2.0, 5.0)).unwrap(),
    )
}

fn far_sketch() -> SketchPlane<f64> {
    SketchPlane::xy_at(Point3::new(0.0, 0.0, 0.0))
}

fn dump(out: &mut String, name: &str, body: &Body<f64>) {
    let _ = writeln!(out, "== {name}");
    let mut worst = 0.0_f64;
    for (fk, face) in body.faces() {
        let surf = body.face_surface(fk).expect("live face");
        match surf {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => {
                let s = if face.sense { 1.0 } else { -1.0 };
                let n = *normal * s;
                let mut resid = 0.0_f64;
                let mut verts = Vec::new();
                for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                    let lp = body.loops().find(|(k, _)| *k == lk).unwrap().1;
                    if let LoopBoundary::Cycle { first } = lp.boundary {
                        for he in body.loop_cycle(first).unwrap() {
                            let q = body.half_edge_start_point(he).unwrap();
                            verts.push(q);
                            resid = resid.max(((q - *origin).dot(*normal)).abs() / normal.norm());
                        }
                    }
                }
                worst = worst.max(resid);
                assert!(
                    (n.norm() - 1.0).abs() < 1e-12,
                    "{name}: a unit plane normal, {}",
                    n.norm()
                );
                assert!(resid < 1e-9, "{name}: every vertex on its plane, {resid:e}");
                // An independent outward test: the face's vertex mean,
                // pushed a hair along the outward normal, then compare
                // with the oriented Newell normal of the outer loop.
                let lp = body.loops().find(|(k, _)| *k == face.outer).unwrap().1;
                let mut newell = Vec3::new(0.0, 0.0, 0.0);
                if let LoopBoundary::Cycle { first } = lp.boundary {
                    let cyc = body.loop_cycle(first).unwrap();
                    let pts: Vec<Point3<f64>> =
                        cyc.iter().map(|&h| body.half_edge_start_point(h).unwrap()).collect();
                    for i in 0..pts.len() {
                        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
                        newell = newell
                            + Vec3::new(
                                (a.y - b.y) * (a.z + b.z),
                                (a.z - b.z) * (a.x + b.x),
                                (a.x - b.x) * (a.y + b.y),
                            );
                    }
                }
                let agree = if newell.norm() > 1e-12 {
                    format!("{:+.6}", newell.dot(n) / newell.norm())
                } else {
                    "deg".into()
                };
                let _ = writeln!(
                    out,
                    "plane out=({:+.15e},{:+.15e},{:+.15e}) u=({:+.15e},{:+.15e},{:+.15e}) o=({:+.6e},{:+.6e},{:+.6e}) loopnewell·out={agree} nv={}",
                    n.x, n.y, n.z, u_ref.x, u_ref.y, u_ref.z, origin.x, origin.y, origin.z, verts.len()
                );
            }
            Surface::Cylinder { axis, u_ref, .. } => {
                let _ = writeln!(
                    out,
                    "cyl sense={} axis=({:+.15e},{:+.15e},{:+.15e}) u=({:+.15e},{:+.15e},{:+.15e})",
                    face.sense, axis.x, axis.y, axis.z, u_ref.x, u_ref.y, u_ref.z
                );
            }
            other => {
                let kind = format!("{other:?}");
                let _ = writeln!(out, "other {} sense={}", &kind[..kind.find(' ').unwrap_or(kind.len()).min(12)], face.sense);
            }
        }
    }
    let _ = writeln!(out, "worst_resid={worst:e}");
}

fn ext(name: &str, out: &mut String, plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>, e: Extrusion<f64>) {
    let profile = match Profile::new(plane, loops).validate(tol()) {
        Ok(p) => p,
        Err(e) => {
            let _ = writeln!(out, "== {name}\nINVALID {e:?}");
            return;
        }
    };
    match extrude(&profile, e, tol()) {
        Ok(b) => dump(out, name, &b.body),
        Err(err) => {
            let _ = writeln!(out, "== {name}\nREFUSED {err:?}");
        }
    }
}

fn rev(name: &str, out: &mut String, plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>, r: Revolution<f64>) {
    let profile = match Profile::new(plane, loops).validate(tol()) {
        Ok(p) => p,
        Err(e) => {
            let _ = writeln!(out, "== {name}\nINVALID {e:?}");
            return;
        }
    };
    let axis = RevolveAxis {
        origin: p(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    match revolve(&profile, axis, r, tol()) {
        Ok(b) => dump(out, name, &b.body),
        Err(err) => {
            let _ = writeln!(out, "== {name}\nREFUSED {err:?}");
        }
    }
}

#[test]
fn r2_face_dump_over_the_sweep_battery() {
    let mut out = String::new();
    let along = |depth| Extrusion::Distance {
        depth,
        side: ExtrudeSide::Along,
    };
    let against = |depth| Extrusion::Distance {
        depth,
        side: ExtrudeSide::Against,
    };
    for (cw, tag) in [(false, "ccw"), (true, "cw")] {
        ext(&format!("rect {tag} along"), &mut out, SketchPlane::xy(), vec![rect(0.0, 0.0, 2.0, 1.0, cw)], along(1.5));
        ext(&format!("rect {tag} against"), &mut out, SketchPlane::xy(), vec![rect(0.0, 0.0, 2.0, 1.0, cw)], against(1.5));
        ext(&format!("d {tag} along tilted"), &mut out, tilted(), vec![d_shape(cw)], along(0.7));
        ext(&format!("d {tag} against tilted"), &mut out, tilted(), vec![d_shape(cw)], against(0.7));
        ext(
            &format!("holed {tag} along"),
            &mut out,
            SketchPlane::xy(),
            vec![rect(0.0, 0.0, 4.0, 3.0, cw), rect(1.0, 1.0, 2.0, 2.0, !cw)],
            along(1.0),
        );
        ext(
            &format!("holed {tag} tilted against"),
            &mut out,
            tilted(),
            vec![rect(0.0, 0.0, 4.0, 3.0, cw), rect(1.0, 1.0, 2.0, 2.0, !cw)],
            against(1.0),
        );
    }
    // Vector extrusions: along the normal, against it, slightly tilted.
    ext("vec up", &mut out, SketchPlane::xy(), vec![rect(0.0, 0.0, 2.0, 1.0, false)], Extrusion::Vector(Vec3::new(0.0, 0.0, 2.0)));
    ext("vec down", &mut out, SketchPlane::xy(), vec![rect(0.0, 0.0, 2.0, 1.0, false)], Extrusion::Vector(Vec3::new(0.0, 0.0, -2.0)));
    ext("vec tilt", &mut out, SketchPlane::xy(), vec![d_shape(false)], Extrusion::Vector(Vec3::new(4e-10, -3e-10, 2.0)));
    ext("vec tilt down", &mut out, SketchPlane::xy(), vec![d_shape(false)], Extrusion::Vector(Vec3::new(4e-10, -3e-10, -2.0)));
    // A profile far from its sketch origin.
    ext("far 1e6", &mut out, far_sketch(), vec![rect(1e6, 1e6, 1e6 + 2.0, 1e6 + 1.0, false)], along(1.0));
    // Lofts: an offset, a turned top, a flipped top.
    let sq = |a: f64| rect(-a, -a, a, a, false);
    for (name, top) in [
        ("loft up", SketchPlane::xy_at(Point3::new(0.3, -0.2, 2.0))),
        ("loft turned", {
            let axis = UnitVec3::new(Vec3::new(1.0, 0.2, 0.0), "probe", geom_core::Band::linear(tol()).unwrap()).unwrap();
            SketchPlane::from_frame(
                RigidMotion::rotation_then_translation(axis, 0.2, Vec3::new(0.0, 0.0, 2.0)).unwrap(),
            )
        }),
    ] {
        match loft_body(&[vec![sq(1.0)], vec![sq(0.6)]], &[SketchPlane::xy(), top], 1, tol()) {
            Ok(l) => dump(&mut out, name, &l.body),
            Err(e) => {
                let _ = writeln!(out, "== {name}\nREFUSED {e:?}");
            }
        }
    }
    // Revolves: a band off the axis, a band touching it, angles of both
    // signs and near a full turn, on the plain and the tilted sketch.
    for (theta, tag) in [(FRAC_PI_2, "+90"), (-FRAC_PI_2, "-90"), (2.0 * PI - 1e-3, "near full"), (-(2.0 * PI - 1e-3), "-near full")] {
        rev(&format!("band {tag}"), &mut out, SketchPlane::xy(), vec![rect(1.0, 0.0, 1.5, 1.0, false)], Revolution::Partial(theta));
        rev(&format!("touch {tag}"), &mut out, SketchPlane::xy(), vec![rect(0.0, 0.0, 1.5, 1.0, false)], Revolution::Partial(theta));
        rev(&format!("band cw {tag} tilted"), &mut out, tilted(), vec![rect(1.0, 0.0, 1.5, 1.0, true)], Revolution::Partial(theta));
    }
    rev("band full", &mut out, SketchPlane::xy(), vec![rect(1.0, 0.0, 1.5, 1.0, false)], Revolution::Full);
    if let Ok(path) = std::env::var("R2_DUMP") {
        std::fs::write(path, &out).unwrap();
    }
}

/// The washer the merge base's `m10_9_the_revolve_carriers_state_their_rim_identity`
/// built, under the shipped rules: print the session's decision counts
/// so the head's (no registration) and the base's (registration) can be
/// compared. A lost identity shows as `numeric` up and `symbolic` down.
#[test]
fn r2_washer_counts_under_the_shipped_rules() {
    use geom_core::sym::with_session_rules;
    use geom_core::{Interval, ParamSymbol, Real, Sym, SymBudget, SymRules};
    type S = Sym<Interval>;
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    for (full, tag) in [(true, "full"), (false, "partial")] {
        let (ok, counts) = with_session_rules(budget, SymRules::shipped(), || {
            let lit = |v: f64| S::from_f64(v);
            let eps = Tol::witness().eps();
            let r0: S = Sym::param(
                ParamSymbol::new(test_utils::symbol_id("r0")),
                Interval::from_bounds(1.0 - eps / 64.0, 1.0 + eps / 64.0),
            );
            let (zero, one, two) = (lit(0.0), lit(1.0), lit(2.0));
            let loop_: ProfileLoop<S> = bulge_loop(vec![
                (Point2::new(r0, zero), zero),
                (Point2::new(two, zero), zero),
                (Point2::new(two, one), zero),
                (Point2::new(r0, one), zero),
            ]);
            let vp = Profile::new(SketchPlane::xy(), vec![loop_])
                .validate(Tol::witness())
                .expect("the washer validates over its box");
            let axis = RevolveAxis {
                origin: Point2::new(zero, zero),
                dir: Vec2::new(zero, one),
            };
            let r = if full {
                Revolution::Full
            } else {
                Revolution::Partial(lit(1.0))
            };
            match revolve(&vp, axis, r, Tol::witness()) {
                Ok(_) => "ok".to_string(),
                Err(e) => format!("{e:?}"),
            }
        });
        println!("R2WASHER {tag} {ok} {counts:?}");
    }
}

/// A boxed VECTOR extrusion, which no head row builds: print each
/// face's stored normal and `u_ref` enclosure, and check every member's
/// `f64` datum (the box's ends and middle) lies inside it.
#[test]
fn r2_a_boxed_vector_extrusion() {
    use geom_core::sym::with_session;
    use geom_core::{Bounds, Interval, ParamSymbol, Real, Sym, SymBudget};
    type S = Sym<Interval>;
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let datum = |s: &Surface<S>| -> Option<[Vec3<S>; 2]> {
        match s {
            Surface::Plane { normal, u_ref, .. } => Some([*normal, *u_ref]),
            Surface::Cylinder { axis, u_ref, .. } => Some([*axis, *u_ref]),
            _ => None,
        }
    };
    let datum64 = |s: &Surface<f64>| -> Option<[Vec3<f64>; 2]> {
        match s {
            Surface::Plane { normal, u_ref, .. } => Some([*normal, *u_ref]),
            Surface::Cylinder { axis, u_ref, .. } => Some([*axis, *u_ref]),
            _ => None,
        }
    };
    for (lo, hi, tag) in [(1.95, 2.05, "up"), (-2.05, -1.95, "down")] {
        let members: Vec<Vec<[Vec3<f64>; 2]>> = [lo, 0.5 * (lo + hi), hi]
            .iter()
            .map(|&z| {
                let pr = Profile::new(SketchPlane::xy(), vec![d_shape(false)]).validate(tol()).unwrap();
                let b = extrude(&pr, Extrusion::Vector(Vec3::new(0.0, 0.0, z)), tol()).unwrap();
                b.body.faces().filter_map(|(f, _)| datum64(b.body.face_surface(f).unwrap())).collect()
            })
            .collect();
        with_session(budget, || {
            let lit = |v: f64| S::from_f64(v);
            let z: S = Sym::param(ParamSymbol::new(0x72_02), Interval::from_bounds(lo, hi));
            let pts = [(0.0, 0.0, 0.0), (2.0, 0.0, 0.6), (2.0, 1.0, 0.0), (0.0, 1.0, 0.0)];
            let lp: ProfileLoop<S> = bulge_loop(pts.iter().map(|&(x, y, b)| (Point2::new(lit(x), lit(y)), lit(b))).collect());
            let pr = match Profile::new(SketchPlane::<S>::xy(), vec![lp]).validate(tol()) {
                Ok(p) => p,
                Err(e) => {
                    println!("R2VEC {tag} INVALID {e:?}");
                    return;
                }
            };
            match extrude(&pr, Extrusion::Vector(Vec3::new(lit(0.0), lit(0.0), z)), tol()) {
                Ok(b) => {
                    let held: Vec<[Vec3<S>; 2]> =
                        b.body.faces().filter_map(|(f, _)| datum(b.body.face_surface(f).unwrap())).collect();
                    for (i, h) in held.iter().enumerate() {
                        let w = |v: Vec3<S>| (v.x.hi() - v.x.lo()).max(v.y.hi() - v.y.lo()).max(v.z.hi() - v.z.lo());
                        let mut contained = true;
                        for m in &members {
                            for k in 0..2 {
                                for (hh, mm) in [(h[k].x, m[i][k].x), (h[k].y, m[i][k].y), (h[k].z, m[i][k].z)] {
                                    if !(hh.lo() - 1e-14 <= mm && mm <= hh.hi() + 1e-14) {
                                        contained = false;
                                    }
                                }
                            }
                        }
                        println!("R2VEC {tag} face {i} normal-width {:e} u_ref-width {:e} contains-members {contained}", w(h[0]), w(h[1]));
                        assert!(contained, "R2VEC {tag} face {i} holds every member");
                    }
                }
                Err(e) => println!("R2VEC {tag} REFUSED {e:?}"),
            }
        });
    }
}
