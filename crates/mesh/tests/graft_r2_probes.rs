//! Review probes (origin-graft-r2) for PR 3413: a graft forwards
//! key-carrying provenance into the destination arena.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::float_cmp
)]

use crate::common;
use common::*;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::RawLoop;
use topo::Body;

fn tol() -> Tol {
    Tol::witness()
}

fn split_seam_donut(fracs: &[f64]) -> (Body<f64>, topo::EdgeKey, Vec<topo::EdgeKey>) {
    let mut body = donut();
    let (seam, edge) = body.edges().next().unwrap();
    let curve = body
        .get_curve_geom(edge.curve)
        .unwrap()
        .certified()
        .unwrap()
        .clone();
    let _ = curve;
    let mut minted = Vec::new();
    for f in fracs {
        let c = body.get_edge(seam).unwrap().curve;
        let (t0, t1) = body.get_curve_geom(c).unwrap().certified().unwrap().params();
        let c = body
            .split_edge(seam, t0 + f * (t1 - t0), tol())
            .expect("split");
        minted.push(c.new_edge);
    }
    (body, seam, minted)
}

fn boxed(x: f64, y: f64, z: f64, s: f64) -> Body<f64> {
    let b = sweep::extrude(
        &validated(vec![profile::ProfileLoop::<f64>::polygon([
            Point2::new(x, y),
            Point2::new(x + s, y),
            Point2::new(x + s, y + s),
            Point2::new(x, y + s),
        ])]),
        sweep::Extrusion::Distance(s),
        tol(),
    )
    .unwrap()
    .body;
    if z == 0.0 {
        b
    } else {
        topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.0, 0.0, z)), tol()).unwrap()
    }
}

fn volume(b: &Body<f64>) -> Result<f64, topo::MassPropsError> {
    topo::mass_properties(b, tol()).map(|m| m.volume)
}

type Out = Result<Option<Result<f64, topo::MassPropsError>>, topo::BooleanError>;

fn run(
    f: fn(&Body<f64>, &Body<f64>, Tol) -> Result<topo::BooleanResult<f64>, topo::BooleanError>,
    a: &Body<f64>,
    b: &Body<f64>,
) -> (Out, Option<Result<usize, String>>, usize) {
    match f(a, b, tol()) {
        Err(e) => (Err(e), None, 0),
        Ok(r) => match r.body() {
            None => (Ok(None), None, 0),
            Some(bb) => (
                Ok(Some(volume(&bb.body))),
                Some(
                    mesh::tessellate(&bb.body, 0.1, tol())
                        .map(|m| m.positions.len())
                        .map_err(|e| format!("{e:?}")),
                ),
                bb.naming.graft_dead_edges.len(),
            ),
        },
    }
}

/// Every edge whose split lineage roots at a key that resolves nowhere,
/// grouped by that root.
fn dead_roots(b: &Body<f64>) -> std::collections::BTreeMap<topo::EdgeKey, Vec<topo::EdgeKey>> {
    let mut m = std::collections::BTreeMap::<_, Vec<_>>::new();
    for (e, _) in b.edges() {
        let r = b.split_root(e, |_| false).expect("no cycle");
        if b.get_edge(r).is_none() {
            m.entry(r).or_default().push(e);
        }
    }
    m
}

fn roots(b: &Body<f64>) -> Vec<(topo::EdgeKey, topo::EdgeKey)> {
    b.edges()
        .map(|(e, _)| (e, b.split_root(e, |_| false).expect("no cycle")))
        .collect()
}

/// Booleans of a (multiply) split-seam donut with a far box and an
/// overlapping box, all three ops, both orders.
#[test]
fn r2_booleans_both_orders() {
    let v_donut = volume(&donut()).unwrap();
    for fracs in [&[0.5][..], &[0.5, 0.5, 0.5][..]] {
        let (src, _, _) = split_seam_donut(fracs);
        let far = boxed(10.0, 10.0, 0.0, 1.0);
        // A box straddling the donut's outer rim (x from 2.2 to 2.8).
        let near = boxed(2.2, -0.3, -0.3, 0.6);
        for (tag, other) in [("far", &far), ("near", &near)] {
            for (name, f) in [
                ("union", topo::union::<f64> as fn(&_, &_, _) -> _),
                ("subtract", topo::subtract::<f64>),
                ("intersect", topo::intersect::<f64>),
            ] {
                let ab = run(f, other, &src);
                let ba = run(f, &src, other);
                println!("R2 {fracs:?} {name}({tag}, donut): {ab:?}");
                println!("R2 {fracs:?} {name}(donut, {tag}): {ba:?}");
            }
        }
        let u1 = run(topo::union::<f64>, &far, &src);
        let u2 = run(topo::union::<f64>, &src, &far);
        let bits = |o: &Out| match o {
            Ok(Some(Ok(v))) => Some(v.to_bits()),
            _ => None,
        };
        assert!(bits(&u1.0).is_some(), "{u1:?}");
        assert_eq!(bits(&u1.0), bits(&u2.0));
        assert!((u1.0.unwrap().unwrap().unwrap() - (1.0 + v_donut)).abs() < 1e-9);
    }
}

/// Graft a split donut into a ball, then graft THAT into a far box's
/// body (a third arena); graft a second placed copy too. Each copy's
/// seam pieces must share a root, the copies must not share one, and
/// everything measures and meshes.
#[test]
fn r2_graft_chains_and_copies() {
    let v_donut = volume(&donut()).unwrap();
    let v_ball = volume(&ball()).unwrap();
    let (src, _, _) = split_seam_donut(&[0.5, 0.5, 0.5]);
    let src_roots: std::collections::BTreeSet<_> =
        roots(&src).into_iter().map(|(_, r)| r).collect();
    let mut held = ball();
    topo::graft_disjoint(&mut held, &src, tol()).unwrap();
    let mut third = boxed(10.0, 10.0, 0.0, 1.0);
    topo::graft_disjoint_all(&mut third, &held, tol()).unwrap();
    let moved = topo::transform_rigid(
        &src,
        &Affine3::translation(Vec3::new(-20.0, 0.0, 0.0)),
        tol(),
    )
    .unwrap();
    topo::graft_disjoint(&mut third, &moved, tol()).unwrap();
    let v = volume(&third);
    let m = mesh::tessellate(&third, 0.1, tol()).map(|m| m.positions.len());
    println!("R2 chain: V = {v:?} want {}, mesh {m:?}", 1.0 + v_ball + 2.0 * v_donut);
    assert!((v.unwrap() - (1.0 + v_ball + 2.0 * v_donut)).abs() < 1e-9);
    assert!(m.is_ok());
    assert!(topo::validate(&third).is_ok(), "{:?}", topo::validate(&third));
    // Roots: in src, the split pieces root at the seam (live). In the
    // third body, each copy's pieces root at that copy's seam image.
    let r3: std::collections::BTreeMap<_, Vec<_>> =
        roots(&third)
            .into_iter()
            .fold(Default::default(), |mut m, (e, r)| {
                m.entry(r).or_default().push(e);
                m
            });
    let multi: Vec<_> = r3.iter().filter(|(_, v)| v.len() > 1).collect();
    println!("R2 chain: src root groups {}, third multi-piece root groups {multi:?}", src_roots.len());
    assert_eq!(multi.len(), 2, "one seam lineage per donut copy");
    for (r, _) in &multi {
        assert!(third.get_edge(**r).is_some(), "the seam root is live");
    }
}

/// A split lineage whose kept-key parent is killed, grafted twice
/// (src -> mid -> dst): dead roots stay dead, shared by siblings, never
/// aliased by later inserts, and survive clone / transform / revert.
#[test]
fn r2_dead_roots_survive_regrafts_and_later_inserts() {
    let mut src = boxed(0.0, 0.0, 0.0, 1.0);
    let e0 = src.edges().next().unwrap().0;
    let param = |b: &Body<f64>, e, f: f64| {
        let c = b.get_edge(e).unwrap().curve;
        let (t0, t1) = b.get_curve_geom(c).unwrap().certified().unwrap().params();
        t0 + f * (t1 - t0)
    };
    let e1 = src.split_edge(e0, param(&src, e0, 0.5), tol()).unwrap().new_edge;
    let _e2 = src.split_edge(e1, param(&src, e1, 0.5), tol()).unwrap().new_edge;
    let _e3 = src.split_edge(e0, param(&src, e0, 0.5), tol()).unwrap().new_edge;
    let he0 = src.get_edge(e0).unwrap().he_plus;
    src.kev(he0).expect("kill the first child");
    let d_src = dead_roots(&src);
    println!("R2 src dead roots: {d_src:?}");
    assert_eq!(d_src.len(), 1);
    assert!(topo::validate(&src).is_ok());

    let mut mid = boxed(5.0, 0.0, 0.0, 1.0);
    topo::graft_disjoint(&mut mid, &src, tol()).unwrap();
    let d_mid = dead_roots(&mid);
    println!("R2 mid dead roots: {d_mid:?}");
    assert_eq!(d_mid.len(), 1);
    assert_eq!(d_mid.values().next().unwrap().len(), d_src.values().next().unwrap().len());
    assert!(topo::validate(&mid).is_ok(), "{:?}", topo::validate(&mid));

    let mut dst = boxed(9.0, 0.0, 0.0, 1.0);
    // Graft mid (two solids) into dst twice: two copies of the lineage.
    topo::graft_disjoint_all(&mut dst, &mid, tol()).unwrap();
    topo::graft_disjoint_all(&mut dst, &mid, tol()).unwrap();
    let d_dst = dead_roots(&dst);
    println!("R2 dst dead roots: {d_dst:?}");
    assert_eq!(d_dst.len(), 2, "one dead root per copy");
    assert!(d_dst.values().all(|v| v.len() == 3));
    assert!(topo::validate(&dst).is_ok(), "{:?}", topo::validate(&dst));
    let dead: Vec<topo::EdgeKey> = d_dst.keys().copied().collect();

    // Clone / transform / revert keep the lineage key-for-key.
    let before = roots(&dst);
    assert_eq!(roots(&dst.clone()), before);
    match topo::transform_rigid(&dst, &Affine3::translation(Vec3::new(0.0, 3.0, 0.0)), tol()) {
        Ok(moved) => assert_eq!(roots(&moved), before),
        Err(e) => println!("R2 transform of the kev'd body refused (geometry, expected): {e:?}"),
    }
    let rev = dst.revert().unwrap();
    assert_eq!(roots(&rev), before);

    // Later inserts: split many edges of dst; no new key may equal a
    // dead root, and the dead roots must stay dead.
    let mut n = 0;
    for _ in 0..3 {
        let keys: Vec<_> = dst.edges().map(|(k, _)| k).collect();
        for e in keys {
            let Some(p) = dst.get_edge(e).and_then(|ed| {
                dst.get_curve_geom(ed.curve)
                    .and_then(|c| c.certified())
                    .map(|c| {
                        let (t0, t1) = c.params();
                        0.5 * (t0 + t1)
                    })
            }) else {
                continue;
            };
            if let Ok(c) = dst.split_edge(e, p, tol()) {
                assert!(!dead.contains(&c.new_edge), "a later insert took a dead key");
                n += 1;
            }
        }
    }
    println!("R2 {n} later splits in dst");
    for d in &dead {
        assert!(dst.get_edge(*d).is_none());
    }
    assert_eq!(dead_roots(&dst).len(), 2);
    assert!(topo::validate(&dst).is_ok(), "{:?}", topo::validate(&dst));
}

/// Planar overlapping booleans where B's edges are cut by the reduction
/// (a kept-key parent dies pre-graft), with and without a user split on
/// B beforehand; all three ops, both orders.
#[test]
fn r2_planar_piercing_booleans() {
    let a = boxed(0.0, 0.0, 0.0, 1.0);
    let b = {
        let b = boxed(0.5, 0.25, 0.0, 0.5);
        topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.25, 0.0, 0.25)), tol()).unwrap()
    };
    let mut b_split = b.clone();
    for _ in 0..3 {
        let keys: Vec<_> = b_split.edges().map(|(k, _)| k).collect();
        for e in keys.into_iter().take(4) {
            let c = b_split.get_edge(e).unwrap().curve;
            let (t0, t1) = b_split.get_curve_geom(c).unwrap().certified().unwrap().params();
            let _ = b_split.split_edge(e, 0.5 * (t0 + t1), tol());
        }
    }
    for (tag, bb) in [("plain", &b), ("presplit", &b_split)] {
        for (name, f) in [
            ("union", topo::union::<f64> as fn(&_, &_, _) -> _),
            ("subtract", topo::subtract::<f64>),
            ("intersect", topo::intersect::<f64>),
        ] {
            for (order, (x, y)) in [("A,B", (&a, bb)), ("B,A", (bb, &a))] {
                let r = f(x, y, tol());
                match &r {
                    Ok(res) => match res.body() {
                        Some(body) => {
                            let v = volume(&body.body);
                            let m = mesh::tessellate(&body.body, 0.1, tol()).map(|m| m.positions.len());
                            let cyc = body
                                .body
                                .edges()
                                .filter(|(e, _)| body.body.split_root(*e, |_| false).is_err())
                                .count();
                            let dead = dead_roots(&body.body);
                            let val = topo::validate(&body.body).is_ok();
                            println!(
                                "R2P {tag} {name}({order}): V={v:?} mesh={m:?} dead_rows={} dead_roots={} cycles={cyc} valid={val} kind={:?}",
                                body.naming.graft_dead_edges.len(),
                                dead.len(),
                                body.kind
                            );
                            // Each dead root in the result must be a
                            // graft_dead_edges result key (or dead in A).
                            let rows: std::collections::BTreeSet<_> =
                                body.naming.graft_dead_edges.iter().map(|&(_, d)| d).collect();
                            let unexplained: Vec<_> =
                                dead.keys().filter(|k| !rows.contains(k)).collect();
                            println!("R2P   dead roots with no graft_dead_edges row: {unexplained:?}");
                            assert_eq!(cyc, 0);
                        }
                        None => println!("R2P {tag} {name}({order}): empty"),
                    },
                    Err(e) => println!("R2P {tag} {name}({order}): Err {e:?}"),
                }
            }
        }
    }
}
