//! Review probes for PR 5567 (`flux/torus-spiric-ring`): independent
//! checks of the bodies whose torus face carries a spiric ring.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::germ_plane_torus_chord::{Solid, boxed, ring, tori_of_one_face_pair};
use crate::germ_plane_torus_frame::{SIX, six, torus};
use crate::germ_plane_torus_spiric::torus_in_box;
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, BooleanOp};

/// Faces of `b` on a torus with a ring, and whether a spiric is on any of its loops.
pub(crate) fn ringed_spiric_tori(b: &topo::Body<f64>) -> usize {
    let mut n = 0;
    for (fk, f) in b.faces() {
        let s = b.get_surface(f.surface).unwrap();
        if !matches!(s, geom::Surface::Torus { .. }) || f.rings.is_empty() {
            continue;
        }
        let spiric = b.half_edges().any(|(_, he)| {
            b.get_loop(he.parent_loop).map(|l| l.face) == Some(fk) && {
                let e = b.get_edge(he.edge).unwrap();
                b.get_curve_geom(e.curve)
                    .and_then(|c| c.certified())
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Spiric { .. }))
            }
        });
        if spiric {
            n += 1;
        }
    }
    n
}

fn poses() -> Vec<(&'static str, (f64, f64), (f64, f64), (f64, f64))> {
    let (x, w) = (0.0053, 0.277);
    vec![
        ("M3", (0.0, 1.0), (-1.0, 1.0), (1.2, 2.8)),
        ("P4", (-1.0, 1.0), (0.2, 1.0), (1.2, 2.8)),
        ("B1", (-0.1, 0.1), (-0.1, 0.1), (-2.0, 2.0)),
        ("B2", (-0.05, 0.05), (0.25, 0.35), (1.0, 3.0)),
        ("the tie bar", (x - w, x + w), (-w, w), (-2.0, 2.0)),
    ]
}

fn want(op: BooleanOp, swap: bool, t: f64, p: f64, meet: f64) -> f64 {
    let (x, y) = if swap { (p, t) } else { (t, p) };
    match op {
        BooleanOp::Union => x + y - meet,
        BooleanOp::Intersect => meet,
        BooleanOp::Subtract => x - meet,
    }
}

fn mesh_va(b: &topo::Body<f64>) -> Option<(f64, f64)> {
    let m = match mesh::tessellate(b, 2e-4, Tol::witness()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("REVIEW mesh refused {e:?}");
            return None;
        }
    };
    let (mut v, mut a) = (0.0, 0.0);
    for p in &m.patches {
        for t in &p.triangles {
            let [p0, p1, p2] = t.map(|i| m.positions[i as usize]);
            let (e1, e2) = (p1 - p0, p2 - p0);
            a += 0.5 * e1.cross(e2).norm();
            v += (p0 - Point3::origin()).dot((p1 - Point3::origin()).cross(p2 - Point3::origin()))
                / 6.0;
        }
    }
    Some((v, a))
}

/// Every built op on a ringed spiric torus face: the volume within its pad
/// of the box oracle, operand orders agreeing, and the tessellation's volume
/// and area beside the measured ones.
#[test]
fn review_5567_pads_orders_and_mesh() {
    let tol = Tol::witness();
    let mut tori = vec![("the donut".to_string(), torus(2.0, 0.5))];
    tori.extend(tori_of_one_face_pair(2.0, 0.5));
    let tv = 2.0 * core::f64::consts::PI.powi(2) * 2.0 * 0.25;
    let (mut ringed, mut worst_pad, mut worst_mesh_v, mut worst_mesh_a, mut nomesh) =
        (0, 0.0f64, 0.0f64, 0.0f64, 0);
    let mut fails = Vec::new();
    for (name, t) in tori {
        let t = ring(t, (2.0, 0.5));
        for (pose, xs, ys, zs) in poses() {
            let p: Solid = boxed(xs, ys, zs);
            let meet = torus_in_box((2.0, 0.5), xs, ys, zs);
            let mut by_op: Vec<(BooleanOp, f64, f64)> = Vec::new();
            for ((label, r), &(_, op, swap)) in six(&t.body, &p.body).into_iter().zip(&SIX) {
                let what = format!("{name}, {pose}, {label}");
                let Ok(r) = r else {
                    eprintln!("REVIEW {what}: refused {:?}", r.err());
                    continue;
                };
                let b = r.body().unwrap().body.clone();
                let rings = ringed_spiric_tori(&b);
                let mp = topo::mass_properties(&b, tol).unwrap();
                let w = want(op, swap, tv, p.volume, meet);
                let err = (mp.volume - w).abs();
                if rings > 0 {
                    ringed += 1;
                    worst_pad = worst_pad.max(err / mp.volume_pad.max(1e-300));
                    if err > mp.volume_pad + 1e-12 * w {
                        fails.push(format!(
                            "{what}: |{} - {w}| = {err:e} > pad {:e}",
                            mp.volume, mp.volume_pad
                        ));
                    }
                    match mesh_va(&b) {
                        Some((mv, ma)) => {
                            worst_mesh_v = worst_mesh_v.max((mv - mp.volume).abs() / w);
                            worst_mesh_a =
                                worst_mesh_a.max((ma - mp.surface_area).abs() / mp.surface_area);
                            if (mv - mp.volume).abs() > 1e-3 * w
                                || (ma - mp.surface_area).abs() > 1e-3 * mp.surface_area
                            {
                                fails.push(format!(
                                    "{what}: mesh v {mv} a {ma} vs {} {}",
                                    mp.volume, mp.surface_area
                                ));
                            }
                        }
                        None => nomesh += 1,
                    }
                }
                for (o, v, pad) in &by_op {
                    if *o == op
                        && op != BooleanOp::Subtract
                        && (v - mp.volume).abs() > pad + mp.volume_pad
                    {
                        fails.push(format!("{what}: operand orders {v} vs {}", mp.volume));
                    }
                }
                by_op.push((op, mp.volume, mp.volume_pad));
            }
        }
    }
    eprintln!(
        "REVIEW ringed bodies {ringed}, worst err/pad {worst_pad:e}, worst mesh v rel {worst_mesh_v:e}, a rel {worst_mesh_a:e}, unmeshed {nomesh}"
    );
    assert!(ringed > 0);
    assert!(fails.is_empty(), "{fails:#?}");
}

/// B1, B2 and P4 on the donut carried and turned measure as at home.
#[test]
fn review_5567_moved_ringed_bodies_measure_as_at_home() {
    let tol = Tol::witness();
    let m = Affine3::translation(Vec3::new(1.3, -0.7, 0.4))
        * Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(0.3, -0.5, 0.8).normalize(),
            0.7,
        );
    let motion = geom_core::test_support::rigid(&m, tol);
    let mv = |b: &AtRestBody<f64>| {
        finished(
            "moved",
            topo::transform_rigid(b, &motion, tol).unwrap(),
            tol,
        )
    };
    let t = torus(2.0, 0.5);
    let mut fails = Vec::new();
    for (pose, xs, ys, zs) in poses() {
        let p = boxed(xs, ys, zs);
        let home = six(&t, &p.body);
        let far = six(&mv(&t), &mv(&p.body));
        for ((label, h), (_, f)) in home.into_iter().zip(far) {
            let (Ok(h), Ok(f)) = (h, f) else {
                eprintln!("REVIEW moved {pose} {label}: one side refused");
                continue;
            };
            let hb = h.body().unwrap().body.clone();
            let fb = f.body().unwrap().body.clone();
            let (a, b) = (
                topo::mass_properties(&hb, tol),
                topo::mass_properties(&fb, tol),
            );
            match (a, b) {
                (Ok(a), Ok(b)) => {
                    eprintln!(
                        "REVIEW moved {pose} {label}: rings {} {}, {} ± {:e} vs {} ± {:e}",
                        ringed_spiric_tori(&hb),
                        ringed_spiric_tori(&fb),
                        a.volume,
                        a.volume_pad,
                        b.volume,
                        b.volume_pad
                    );
                    if (a.volume - b.volume).abs() > a.volume_pad + b.volume_pad {
                        fails.push(format!("{pose} {label}"));
                    }
                }
                (a, b) => eprintln!("REVIEW moved {pose} {label}: {:?} / {:?}", a.err(), b.err()),
            }
        }
    }
    assert!(fails.is_empty(), "{fails:#?}");
}

/// B1 and B2 at the interval scalar.
#[test]
fn review_5567_ringed_bodies_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::{Bounds, Interval, Point2};
    let tol = Tol::witness();
    let t: AtRestBody<Interval> = finished(
        "the torus",
        sweep::test_support::revolved_about_y_at::<Interval>(
            vec![
                (Point2::new(iv(2.0), iv(-0.5)), iv(1.0)),
                (Point2::new(iv(2.0), iv(0.5)), iv(1.0)),
            ],
            sweep::Revolution::Full,
            tol,
        ),
        tol,
    );
    let tv = 2.0 * core::f64::consts::PI.powi(2) * 2.0 * 0.25;
    let mut fails = Vec::new();
    for (pose, xs, ys, zs) in poses() {
        let p: AtRestBody<Interval> = finished(
            "bar",
            sweep::test_support::brick::<Interval>(xs, ys, zs, tol),
            tol,
        );
        let pv = (xs.1 - xs.0) * (ys.1 - ys.0) * (zs.1 - zs.0);
        let meet = torus_in_box((2.0, 0.5), xs, ys, zs);
        for &(label, op, swap) in &SIX {
            let (x, y) = if swap { (&p, &t) } else { (&t, &p) };
            let out = match op {
                BooleanOp::Union => topo::union(x, y, tol),
                BooleanOp::Intersect => topo::intersect(x, y, tol),
                BooleanOp::Subtract => topo::subtract(x, y, tol),
            };
            let out = match out {
                Ok(o) => o,
                Err(e) => {
                    eprintln!("REVIEW iv {pose} {label}: refused {e:?}");
                    continue;
                }
            };
            let b = &out.body().unwrap().body;
            let g = topo::validate_geometric(b, tol);
            let mp = topo::mass_properties(b, tol);
            let w = want(op, swap, tv, pv, meet);
            match (g, mp) {
                (Ok(()), Ok(mp)) => {
                    let s = mp.volume_pad + 1e-12 * w;
                    eprintln!(
                        "REVIEW iv {pose} {label}: [{}, {}] pad {:e} truth {w}",
                        mp.volume.lo(),
                        mp.volume.hi(),
                        mp.volume_pad
                    );
                    if !(mp.volume.lo() - s <= w && w <= mp.volume.hi() + s) {
                        fails.push(format!("{pose} {label}"));
                    }
                }
                (g, mp) => eprintln!("REVIEW iv {pose} {label}: tier3 {g:?} props {:?}", mp.err()),
            }
        }
    }
    assert!(fails.is_empty(), "{fails:#?}");
}

/// Dump of every built op's (volume, pad, area) bits over the donut poses
/// on the five tori, for a merge-base differential.
#[test]
fn review_5567_dump_bits() {
    let tol = Tol::witness();
    let mut tori = vec![("the donut".to_string(), torus(2.0, 0.5))];
    tori.extend(tori_of_one_face_pair(2.0, 0.5));
    let (x, w) = (0.0053, 0.277);
    let mut ps: Vec<(&str, Solid)> = vec![(
        "S1",
        crate::germ_plane_torus_chord::half(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x()),
    )];
    for (n, xs, ys, zs) in [
        ("M3", (0.0, 1.0), (-1.0, 1.0), (1.2, 2.8)),
        ("P4", (-1.0, 1.0), (0.2, 1.0), (1.2, 2.8)),
        ("the spiric box", (0.3, 1.0), (-1.0, 1.0), (-3.0, 3.0)),
        ("B1", (-0.1, 0.1), (-0.1, 0.1), (-2.0, 2.0)),
        ("B2", (-0.05, 0.05), (0.25, 0.35), (1.0, 3.0)),
        ("the tie bar", (x - w, x + w), (-w, w), (-2.0, 2.0)),
    ] {
        ps.push((n, boxed(xs, ys, zs)));
    }
    let mut out = String::new();
    for (name, t) in &tori {
        for (pose, p) in &ps {
            for (label, r) in six(t, &p.body) {
                let line = match r {
                    Ok(r) => match r.body() {
                        Some(b) => match topo::mass_properties(&b.body, tol) {
                            Ok(mp) => format!(
                                "B {:016x} {:016x} {:016x} {:016x}",
                                mp.volume.to_bits(),
                                mp.volume_pad.to_bits(),
                                mp.surface_area.to_bits(),
                                mp.area_pad.to_bits()
                            ),
                            Err(e) => format!("P {e:?}"),
                        },
                        None => "empty".into(),
                    },
                    Err(e) => format!(
                        "E {}",
                        format!("{e:?}").chars().take(160).collect::<String>()
                    ),
                };
                out += &format!("{name} | {pose} | {label} | {line}\n");
            }
        }
    }
    let path = std::env::var("REVIEW_DUMP").unwrap_or("/tmp/review_dump.txt".into());
    std::fs::write(path, out).unwrap();
}

/// A lone sense flip on a ringed spiric torus face: does tier 3 refuse it?
#[test]
fn review_5567_flip_a_ringed_face() {
    let tol = Tol::witness();
    let t = torus(2.0, 0.5);
    let (x, w) = (0.0053, 0.277);
    for (pose, xs, ys, zs) in [
        ("S1-ish box", (0.3, 9.0), (-9.0, 9.0), (-9.0, 9.0)),
        ("B1", (-0.1, 0.1), (-0.1, 0.1), (-2.0, 2.0)),
        ("B2", (-0.05, 0.05), (0.25, 0.35), (1.0, 3.0)),
        ("the tie bar", (x - w, x + w), (-w, w), (-2.0, 2.0)),
    ] {
        let p = boxed(xs, ys, zs);
        for (label, r) in six(&t, &p.body) {
            let Ok(r) = r else { continue };
            let b = r.body().unwrap().body.clone();
            for (fk, f) in b.faces() {
                let s = b.get_surface(f.surface).unwrap();
                if !matches!(s, geom::Surface::Torus { .. }) {
                    continue;
                }
                let spiric = b.half_edges().any(|(_, he)| {
                    b.get_loop(he.parent_loop).map(|l| l.face) == Some(fk)
                        && b.get_curve_geom(b.get_edge(he.edge).unwrap().curve)
                            .and_then(|c| c.certified())
                            .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Spiric { .. }))
                });
                if !spiric {
                    continue;
                }
                let mut m: topo::Body<f64> = (*b).clone();
                m.set_face_sense(fk, !f.sense).unwrap();
                let g = topo::validate_geometric(&m, tol);
                let mp = topo::mass_properties(&m, tol).map(|p| p.volume);
                eprintln!(
                    "REVIEW flip {pose} {label} face rings={} : tier3 {} ; volume {:?} (unflipped {:?})",
                    f.rings.len(),
                    match &g {
                        Ok(()) => "PASSES".to_string(),
                        Err(e) => format!(
                            "{:?}",
                            e.iter()
                                .map(|e| format!("{e:?}").chars().take(60).collect::<String>())
                                .collect::<Vec<_>>()
                        ),
                    },
                    mp.ok(),
                    topo::mass_properties(&b, tol).map(|p| p.volume).ok()
                );
            }
        }
    }
}
