//! origin-graft-r1 review probes (PR 3413).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use std::collections::BTreeMap;

use geom_core::Tol;
use slotmap::Key;

use crate::body::Body;
use crate::entity::*;
use crate::euler::{MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::instance::graft_disjoint_all_keyed;
use crate::provenance::Provenance;
use crate::test_support_fixtures::{declined_cube, holed_block};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum K {
    He(HalfEdgeKey),
    L(LoopKey),
    E(EdgeKey),
    S(ShellKey),
    So(SolidKey),
}

fn payload(p: &Provenance) -> Vec<K> {
    let mev = |s: &MevSite| match *s {
        MevSite::Fan { he1, he2 } => vec![K::He(he1), K::He(he2)],
        MevSite::Lone { r#loop } => vec![K::L(r#loop)],
    };
    match p {
        Provenance::Primordial { .. } | Provenance::Mvfs => vec![],
        Provenance::Mev { site } | Provenance::MevNull { site, .. } => mev(site),
        Provenance::Mef { site } => match *site {
            MefSite::Chords { he1, he2 } => vec![K::He(he1), K::He(he2)],
            MefSite::Lone { r#loop } => vec![K::L(r#loop)],
        },
        Provenance::Kemr { he1, he2 } => vec![K::He(*he1), K::He(*he2)],
        Provenance::Mekr { site } => match *site {
            MekrSite::Cycles { target, ring } => vec![K::He(target), K::He(ring)],
            MekrSite::EmptyRing { target, ring } => vec![K::He(target), K::L(ring)],
            MekrSite::EmptyTarget { target, ring } => vec![K::L(target), K::He(ring)],
            MekrSite::BothEmpty { target, ring } => vec![K::L(target), K::L(ring)],
        },
        Provenance::Mfkrh { ring } => vec![K::L(*ring)],
        Provenance::SplitEdge { edge } => vec![K::E(*edge)],
        Provenance::Movefac { shell } => vec![K::S(*shell)],
        Provenance::MoveShells { solid } => vec![K::So(*solid)],
    }
}

fn live(b: &Body<f64>, k: K) -> bool {
    match k {
        K::He(k) => b.get_half_edge(k).is_some(),
        K::L(k) => b.get_loop(k).is_some(),
        K::E(k) => b.get_edge(k).is_some(),
        K::S(k) => b.get_shell(k).is_some(),
        K::So(k) => b.get_solid(k).is_some(),
    }
}

/// Every record of `src` vs its image in `dst`: live payload keys map to
/// the image; dead ones map to a key dead in `dst`, consistently.
fn check(src: &Body<f64>, dst: &Body<f64>, keys: &crate::instance::GraftKeys) -> (usize, usize, usize) {
    let mut img: BTreeMap<K, K> = BTreeMap::new();
    for (e, ed) in src.edges() {
        let de = keys.map.edges[e];
        img.insert(K::E(e), K::E(de));
        let d = dst.get_edge(de).unwrap();
        img.insert(K::He(ed.he_plus), K::He(d.he_plus));
        img.insert(K::He(ed.he_minus), K::He(d.he_minus));
    }
    for (f, fd) in src.faces() {
        let df = dst.get_face(keys.map.faces[f]).unwrap();
        img.insert(K::L(fd.outer), K::L(df.outer));
        for (a, b) in fd.rings.iter().zip(df.rings.iter()) {
            img.insert(K::L(*a), K::L(*b));
        }
    }
    for (s, _) in src.shells() {
        img.insert(K::S(s), K::S(keys.map.shells[s]));
    }
    for ((s, _), &d) in src.solids().zip(keys.solids()) {
        img.insert(K::So(s), K::So(d));
    }
    let mut dead_img: BTreeMap<K, K> = BTreeMap::new();
    let (mut n_live, mut n_dead, mut n_rec) = (0, 0, 0);
    let mut pairs: Vec<(Option<&Provenance>, Option<&Provenance>)> = Vec::new();
    for (e, _) in src.edges() {
        pairs.push((src.edge_provenance.get(e), dst.edge_provenance.get(keys.map.edges[e])));
    }
    for (e, ed) in src.edges() {
        let d = dst.get_edge(keys.map.edges[e]).unwrap();
        pairs.push((src.half_edge_provenance.get(ed.he_plus), dst.half_edge_provenance.get(d.he_plus)));
        pairs.push((src.half_edge_provenance.get(ed.he_minus), dst.half_edge_provenance.get(d.he_minus)));
    }
    for (v, _) in src.vertices() {
        pairs.push((src.vertex_provenance.get(v), dst.vertex_provenance.get(keys.map.vertices[v])));
    }
    for (f, fd) in src.faces() {
        let df = dst.get_face(keys.map.faces[f]).unwrap();
        pairs.push((src.face_provenance.get(f), dst.face_provenance.get(keys.map.faces[f])));
        pairs.push((src.loop_provenance.get(fd.outer), dst.loop_provenance.get(df.outer)));
        for (a, b) in fd.rings.iter().zip(df.rings.iter()) {
            pairs.push((src.loop_provenance.get(*a), dst.loop_provenance.get(*b)));
        }
    }
    for (s, _) in src.shells() {
        pairs.push((src.shell_provenance.get(s), dst.shell_provenance.get(keys.map.shells[s])));
    }
    for ((s, _), &d) in src.solids().zip(keys.solids()) {
        pairs.push((src.solid_provenance.get(s), dst.solid_provenance.get(d)));
    }
    for (ps, pd) in pairs {
        let (ps, pd) = (ps.expect("src record"), pd.expect("dst record"));
        n_rec += 1;
        let (ks, kd) = (payload(ps), payload(pd));
        assert_eq!(std::mem::discriminant(ps), std::mem::discriminant(pd));
        assert_eq!(ks.len(), kd.len());
        for (a, b) in ks.into_iter().zip(kd) {
            if live(src, a) {
                n_live += 1;
                assert_eq!(img.get(&a), Some(&b), "live payload {a:?} maps to its image");
            } else {
                n_dead += 1;
                assert!(!live(dst, b), "dead payload {a:?} -> {b:?} must not resolve in dst");
                let prev = *dead_img.entry(a).or_insert(b);
                assert_eq!(prev, b, "one dst key per dead src key");
                assert!(!img.values().any(|v| *v == b), "dead image is no live image");
            }
        }
    }
    (n_rec, n_live, n_dead)
}

fn split_mid(b: &mut Body<f64>, e: EdgeKey) -> EdgeKey {
    let c = b.get_edge(e).unwrap().curve;
    let (t0, t1) = b.get_curve_geom(c).unwrap().certified().unwrap().params();
    b.split_edge(e, 0.5 * (t0 + t1), Tol::witness()).unwrap().new_edge
}

fn history_src() -> Body<f64> {
    let tol = Tol::witness();
    let mut src = holed_block::<f64>(3.0, &[1.5], tol);
    // Split lineages with dead ancestors.
    let e0 = src.edges().nth(3).unwrap().0;
    let e1 = split_mid(&mut src, e0);
    let _e2 = split_mid(&mut src, e1);
    let _e3 = split_mid(&mut src, e0);
    let he0 = src.get_edge(e0).unwrap().he_plus;
    src.kev(he0).expect("first child dies");
    // A second shell under the solid, then MoveShells.
    let first = src.solids().next().unwrap().0;
    crate::instance::graft_disjoint_all_onto_keyed(&mut src, &[first], &declined_cube::<f64>(tol).body, tol).unwrap();
    let moved = src.shells_of_solid(first).unwrap()[1];
    src.move_shells_to_new_solid(&[moved]).unwrap();
    src
}

#[test]
fn r1_every_payload_forwarded_or_tombstoned() {
    let tol = Tol::witness();
    let src = history_src();
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    let recs = src.solid_provenance.values()
        .chain(src.shell_provenance.values())
        .chain(src.face_provenance.values())
        .chain(src.loop_provenance.values())
        .chain(src.half_edge_provenance.values())
        .chain(src.edge_provenance.values())
        .chain(src.vertex_provenance.values());
    for p in recs {
        let s = format!("{p:?}");
        *kinds.entry(s.split([' ', '{']).next().unwrap().to_string()).or_default() += 1;
    }
    println!("R1 src record kinds: {kinds:?}");
    // A destination with its own history so no key coincides.
    let mut dst = holed_block::<f64>(5.0, &[1.5, 3.5], tol);
    let before_he = dst.half_edges().count();
    let keys = graft_disjoint_all_keyed(&mut dst, &src, tol).unwrap();
    let (n, l, d) = check(&src, &dst, &keys);
    println!("R1 first graft: {n} records, {l} live payload keys, {d} dead payload keys; dead_edges={:?}", keys.map.dead_edges);
    assert!(d > 0);
    assert_eq!(dst.half_edges().count(), before_he + src.half_edges().count(), "arena counts unaffected by tombstones");
    println!("R1 validate(dst) = {:?}", crate::validate::validate(&dst).map_err(|e| e.len()));
    // Graft the destination into a third body; the check holds again.
    let mut third = declined_cube::<f64>(tol).body;
    let keys2 = graft_disjoint_all_keyed(&mut third, &dst, tol).unwrap();
    let (n2, l2, d2) = check(&dst, &third, &keys2);
    println!("R1 second graft: {n2} records, {l2} live, {d2} dead; validate={:?}", crate::validate::validate(&third).map_err(|e| e.len()));
    // Hammer the arena after the graft: many inserts; no dead key comes alive.
    let dead: Vec<EdgeKey> = keys2.map.dead_edges.values().copied().collect();
    for _ in 0..20 {
        let c = declined_cube::<f64>(tol).body;
        graft_disjoint_all_keyed(&mut third, &c, tol).unwrap();
        let es: Vec<EdgeKey> = third.edges().map(|(e, _)| e).collect();
        for e in es.into_iter().take(5) {
            let _ = split_mid(&mut third, e);
        }
    }
    for k in &dead {
        assert!(third.get_edge(*k).is_none(), "{k:?} came alive");
    }
    let idx: std::collections::BTreeSet<u64> = dead.iter().map(|k| k.data().as_ffi() & 0xffff_ffff).collect();
    let reused = third.edges().filter(|(e, _)| idx.contains(&(e.data().as_ffi() & 0xffff_ffff))).count();
    println!("R1 dead edge keys {dead:?}: still dead after 20 grafts+100 splits; {reused} live edges now sit in their slot indices");
    // Clone/transform/revert of the third body keep the records' meaning.
    let rev = third.revert().unwrap();
    for (e, _) in third.edges() {
        assert_eq!(third.split_root(e, |_| false), rev.split_root(e, |_| false));
    }
    println!("R1 revert keeps every split root");
}

/// Kept-mode graft (onto an existing solid): the destination solid keeps
/// its record, and a MoveShells in the SOURCE... (no solid record crosses).
#[test]
fn r1_onto_graft_forwards_too() {
    let tol = Tol::witness();
    let src = history_src();
    let mut dst = holed_block::<f64>(5.0, &[1.5, 3.5], tol);
    let s0 = dst.solids().next().unwrap().0;
    let rec0 = dst.solid_provenance.get(s0).cloned();
    let targets = vec![s0; src.solids().count()];
    let keys = crate::instance::graft_disjoint_all_onto_keyed(&mut dst, &targets, &src, tol).unwrap();
    assert_eq!(dst.solid_provenance.get(s0).cloned(), rec0);
    // Records other than solids: reuse `check` minus the solid pairs by
    // checking edges' split roots.
    for (e, _) in src.edges() {
        let r = src.split_root(e, |_| false).unwrap();
        let want = keys.map.edges.get(r).copied().or_else(|| keys.map.dead_edges.get(&r).copied());
        assert_eq!(dst.split_root(keys.map.edges[e], |_| false).ok(), want);
    }
    println!("R1 onto-graft: roots forwarded; validate={:?}", crate::validate::validate(&dst).map_err(|e| e.len()));
}
