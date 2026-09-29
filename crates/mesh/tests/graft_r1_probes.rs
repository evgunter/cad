//! origin-graft-r1 review probes (PR 3413): a graft forwards provenance
//! keys into the destination. Every row prints what it measured.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::float_cmp
)]

use crate::common;
use common::*;
use geom::Curve3;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::RawLoop;
use topo::Body;

fn split_donut(fracs: &[f64]) -> Result<Body<f64>, String> {
    let tol = Tol::witness();
    let mut body = donut();
    let (seam, edge) = body.edges().next().unwrap();
    let curve = body.get_curve_geom(edge.curve).unwrap().certified().unwrap().clone();
    assert!(matches!(curve.carrier(), Curve3::Circle { radius, .. } if (*radius - 0.5).abs() < 1e-12));
    for f in fracs {
        // Split whichever current piece of the original seam holds the parameter.
        let (t0, t1) = curve.params();
        let t = t0 + f * (t1 - t0);
        let target = body
            .edges()
            .filter(|(e, _)| body.split_root(*e, |_| false) == Ok(seam))
            .find(|(_, ed)| {
                let (a, b) = body.get_curve_geom(ed.curve).unwrap().certified().unwrap().params();
                a.min(b) < t && t < a.max(b)
            })
            .map(|(e, _)| e)
            .ok_or_else(|| format!("no piece holds {f}"))?;
        body.split_edge(target, t, tol).map_err(|e| format!("{e:?}"))?;
    }
    Ok(body)
}

fn vol(b: &Body<f64>) -> Result<f64, String> {
    topo::mass_properties(b, Tol::witness()).map(|m| m.volume).map_err(|e| format!("{e:?}"))
}

fn meshn(b: &Body<f64>) -> Result<usize, String> {
    mesh::tessellate(b, 0.1, Tol::witness()).map(|m| m.positions.len()).map_err(|e| format!("{e:?}"))
}

fn boxy(x0: f64, y0: f64, x1: f64, y1: f64, h: f64) -> Body<f64> {
    sweep::extrude(
        &validated(vec![profile::ProfileLoop::<f64>::polygon([
            Point2::new(x0, y0),
            Point2::new(x1, y0),
            Point2::new(x1, y1),
            Point2::new(x0, y1),
        ])]),
        sweep::Extrusion::Distance(h),
        Tol::witness(),
    )
    .unwrap()
    .body
}

type Op = fn(&Body<f64>, &Body<f64>, Tol) -> Result<topo::BooleanResult<f64>, topo::BooleanError>;

fn run(op: Op, a: &Body<f64>, b: &Body<f64>) -> (String, Option<Body<f64>>) {
    match op(a, b, Tol::witness()) {
        Err(e) => (format!("Err({e:?})"), None),
        Ok(r) => match r.body() {
            None => ("Empty".into(), None),
            Some(bb) => {
                let v = vol(&bb.body);
                let m = meshn(&bb.body);
                let val = topo::validate(&bb.body).map_err(|e| e.len());
                (
                    format!(
                        "V={:?} bits={:?} mesh={m:?} validate={val:?} splitroots_ok={:?}",
                        v,
                        v.as_ref().ok().map(|x| x.to_bits()),
                        bb.body.edges().all(|(e, _)| bb.body.split_root(e, |_| false).is_ok())
                    ),
                    Some(bb.body.clone()),
                )
            }
        },
    }
}

#[test]
fn g1_both_orders_many_split_sets() {
    let far = || boxy(10.0, 10.0, 11.0, 11.0, 1.0);
    for fracs in [&[0.5][..], &[0.25, 0.5, 0.75], &[0.5, 0.25], &[0.1, 0.9, 0.5, 0.3]] {
        let d = match split_donut(fracs) {
            Ok(d) => d,
            Err(e) => {
                println!("G1 {fracs:?}: fixture failed {e}");
                continue;
            }
        };
        let (s1, _) = run(topo::union, &far(), &d);
        let (s2, _) = run(topo::union, &d, &far());
        println!("G1 {fracs:?} union(far,d): {s1}\nG1 {fracs:?} union(d,far): {s2}");
        // Both operands split.
        let d2 = {
            let d = split_donut(&[0.3, 0.6]).unwrap();
            topo::transform_rigid(&d, &Affine3::translation(Vec3::new(20.0, 0.0, 0.0)), Tol::witness()).unwrap()
        };
        let (s3, _) = run(topo::union, &d2, &d);
        let (s4, _) = run(topo::union, &d, &d2);
        println!("G1 {fracs:?} union(d2,d): {s3}\nG1 {fracs:?} union(d,d2): {s4}");
        let (s5, _) = run(topo::subtract, &d, &far());
        let (s6, _) = run(topo::subtract, &far(), &d);
        let (s7, _) = run(topo::intersect, &d, &far());
        println!("G1 {fracs:?} d-far: {s5}\nG1 {fracs:?} far-d: {s6}\nG1 {fracs:?} d&far: {s7}");
    }
}

#[test]
fn g2_void_and_overlap() {
    let tol = Tol::witness();
    let lift = Affine3::translation(Vec3::new(0.0, 0.0, 3.0));
    let d = topo::transform_rigid(&split_donut(&[0.25, 0.5]).unwrap(), &lift, tol).unwrap();
    let plain = topo::transform_rigid(&donut(), &lift, tol).unwrap();
    let big = || boxy(-4.0, -4.0, 4.0, 4.0, 6.0);
    println!("G2 V(big)={:?} V(donut)={:?}", vol(&big()), vol(&plain));
    for (name, op) in [("subtract", topo::subtract as Op), ("union", topo::union as Op), ("intersect", topo::intersect as Op)] {
        let (a, _) = run(op, &big(), &d);
        let (b, _) = run(op, &big(), &plain);
        println!("G2 {name}(big, split donut): {a}\nG2 {name}(big, plain donut): {b}");
    }
    // A box cutting through the donut's ring (x from 1.8 to 3, z band around 3).
    let cut = || boxy(1.8, -1.0, 3.0, 1.0, 6.0);
    for (name, op) in [("subtract", topo::subtract as Op), ("union", topo::union as Op), ("intersect", topo::intersect as Op)] {
        let (a, _) = run(op, &cut(), &d);
        let (b, _) = run(op, &d, &cut());
        let (c, _) = run(op, &cut(), &plain);
        println!("G2 {name}(cut, split): {a}\nG2 {name}(split, cut): {b}\nG2 {name}(cut, plain): {c}");
    }
}

#[test]
fn g3_graft_twice_and_derived_bodies() {
    let tol = Tol::witness();
    let d = split_donut(&[0.25, 0.5, 0.75]).unwrap();
    let vd = vol(&donut()).unwrap();
    let mut held = ball();
    let vb = vol(&held).unwrap();
    topo::graft_disjoint(&mut held, &d, tol).unwrap();
    println!("G3 held: V={:?} want {} mesh={:?} validate={:?}", vol(&held), vb + vd, meshn(&held), topo::validate(&held).map_err(|e| e.len()));
    // Graft the held body again into a third body (not empty).
    let mut third = boxy(10.0, 10.0, 11.0, 11.0, 1.0);
    topo::graft_disjoint_all(&mut third, &held, tol).unwrap();
    println!("G3 third: V={:?} want {} mesh={:?} validate={:?}", vol(&third), 1.0 + vb + vd, meshn(&third), topo::validate(&third).map_err(|e| e.len()));
    // And once more, onto an existing solid.
    let mut fourth = boxy(20.0, 20.0, 21.0, 21.0, 1.0);
    let s0 = fourth.solids().next().unwrap().0;
    let targets = vec![s0; third.solids().count()];
    topo::graft_disjoint_all_onto_keyed(&mut fourth, &targets, &third, tol).unwrap();
    println!("G3 fourth(onto): V={:?} want {} mesh={:?}", vol(&fourth), 2.0 + vb + vd, meshn(&fourth));
    // Transform / revert / clone of a destination.
    let moved = topo::transform_rigid(&third, &Affine3::translation(Vec3::new(0.3, -7.0, 2.0)), tol);
    println!("G3 transformed third: {:?}", moved.as_ref().map(|m| (vol(m), meshn(m))).map_err(|e| format!("{e:?}")));
    let rev = third.revert();
    println!("G3 reverted third: {:?}", rev.as_ref().map(|m| vol(m)).map_err(|e| format!("{e:?}")));
    // Unions of an already-grafted result, both orders.
    let far = || boxy(10.0, 10.0, 11.0, 11.0, 1.0);
    let (_, u) = run(topo::union, &far(), &d);
    let u = u.unwrap();
    let far2 = || boxy(-10.0, -10.0, -9.0, -9.0, 1.0);
    let (a, ab) = run(topo::union, &u, &far2());
    let (b, bb) = run(topo::union, &far2(), &u);
    println!("G3 union(u, far2): {a}\nG3 union(far2, u): {b}");
    if let (Some(ab), Some(bb)) = (ab, bb) {
        let (c, _) = run(topo::union, &ab, &bb.clone());
        println!("G3 union(those two) (overlapping identical): {c}");
    }
}

#[test]
fn g4_plain_pair_order_baseline() {
    let d2 = topo::transform_rigid(&donut(), &Affine3::translation(Vec3::new(20.0, 0.0, 0.0)), Tol::witness()).unwrap();
    let (a, _) = run(topo::union, &d2, &donut());
    let (b, _) = run(topo::union, &donut(), &d2);
    println!("G4 plain union(d2,d): {a}\nG4 plain union(d,d2): {b}");
}
