//! Reviewer probe (reach-dual3985-r1): the editor's round boss / slab /
//! plate unions in every member order, and their two-member prefixes,
//! digested for the base/head differential, each body's edges against
//! the closed form and its volume against the closed form.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::emit_shared_rim_several::permutations;
use crate::fixture::{frame, insert, len};
use core::f64::consts::PI;
use editor_core::{ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId};
use geom_core::Tol;
use std::hash::{Hash, Hasher};
use std::io::Write;

fn disc(doc: ProfileDoc, lp: LoopProgram, z0: f64, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, p) = insert(doc, Node::Profile(ProfileProgram { plane, loops: vec![lp], ids: Vec::new() }));
    insert(doc, Node::Extrude { profile: p, distance: len(dz), side: ExtrudeSide::Along })
}

#[test]
fn zz_probe_editor_boss_slab_plate() {
    let (r, cx, cy) = (0.6f64, 1.5f64, 1.0f64);
    let doc = ProfileDoc::empty_derived("round_boss_slab", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, boss) = disc(doc, LoopProgram::circle_split(cx, cy, r, 3, 0.0).unwrap(), 0.44, 1.8);
    let (doc, slab) = block(doc, (1.4, 1.6), (-1.0, 3.0), 0.5, 1.5);
    let ids = [plate, boss, slab];
    // closed forms, f < 0 inside
    let f_plate = |x: f64, y: f64, z: f64| (-x).max(x - 3.0).max(-y).max(y - 2.0).max(-z).max(z - 1.0);
    let f_boss = move |x: f64, y: f64, z: f64| {
        (((x - cx).powi(2) + (y - cy).powi(2)).sqrt() - r).max(0.44 - z).max(z - 2.24)
    };
    let f_slab = |x: f64, y: f64, z: f64| (1.4 - x).max(x - 1.6).max(-1.0 - y).max(y - 3.0).max(0.5 - z).max(z - 2.0);
    let fs: [&dyn Fn(f64, f64, f64) -> f64; 3] = [&f_plate, &f_boss, &f_slab];
    let mut lines = Vec::new();
    let mut orders: Vec<Vec<usize>> = permutations(&[0, 1, 2]);
    for a in 0..3 {
        for b in 0..3 {
            if a != b {
                orders.push(vec![a, b]);
            }
        }
    }
    for order in orders {
        let (d, n) = insert(
            doc.clone(),
            Node::Union { members: order.iter().map(|&i| ids[i]).collect(), declare: Vec::new() },
        );
        let ev = run(&d);
        let what = format!("order {order:?}");
        if let Some(e) = failure(&ev, n) {
            lines.push(format!("{what}\tREFUSED\t{}", format!("{e:?}").chars().take(120).collect::<String>()));
            continue;
        }
        let body = body_of(&ev, n);
        let f = |x: f64, y: f64, z: f64| order.iter().map(|&i| fs[i](x, y, z)).fold(f64::MAX, f64::min);
        let mut worst = 0.0f64;
        let mut digest = Vec::new();
        for (_, e) in body.edges() {
            let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else { continue };
            let (t0, t1) = c.params();
            let mut pts = Vec::new();
            for k in 0..=8 {
                let p = c.carrier().eval(t0 + (t1 - t0) * f64::from(k) / 8.0);
                worst = worst.max(f(p.x, p.y, p.z).abs());
                pts.push(format!("{:?},{:?},{:?}", p.x, p.y, p.z));
            }
            digest.push(pts.join(";"));
        }
        digest.sort();
        let mut h = std::collections::hash_map::DefaultHasher::new();
        digest.hash(&mut h);
        // Monte Carlo of the closed form over the union's box
        let (mut s, mut inside, nn) = (0x2545F4914F6CDD1Du64, 0usize, 200_000usize);
        for _ in 0..nn {
            let mut u = || {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                (s >> 11) as f64 / (1u64 << 53) as f64
            };
            let (x, y, z) = (u() * 3.0, -1.0 + u() * 4.0, u() * 2.24);
            if f(x, y, z) < 0.0 {
                inside += 1;
            }
        }
        let bv = 3.0 * 4.0 * 2.24;
        let mc = inside as f64 / nn as f64 * bv;
        let v = topo::mass_properties(body, Tol::witness()).map(|p| p.volume);
        let _ = PI;
        lines.push(format!(
            "{what}\tBUILT\thash={:016x}\tvol={v:?}\tmc={mc:.4}\tworst={worst:.2e}\t{}",
            h.finish(),
            if worst > 1e-7 { "EDGE_OFF_BOUNDARY" } else { "ok" }
        ));
    }
    let path = std::env::var("PROBE_OUT").unwrap_or_else(|_| "/tmp/probe_out".into());
    let mut fh = std::fs::File::create(format!("{path}.editor")).unwrap();
    for l in &lines {
        writeln!(fh, "{l}").unwrap();
    }
}
