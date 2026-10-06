//! Review probes for PR 4074 (`Pinches`): chart-level shapes beyond the
//! PR's two unit rows. Each `ok` row asserts a watertight chart: every
//! walk edge used once, every other directed edge paired with its
//! reverse, every triangle CCW, and the triangle area equal to the
//! face's shoelace area.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::{PatchVertex, triangulate_chart};
use crate::types::TessellateError;
use std::collections::HashMap;
use topo::FaceKey;

type Chart = (Vec<Vec<u32>>, Vec<Vec<[f64; 2]>>);

fn chart(walks: &[&[[f64; 2]]]) -> Chart {
    let mut next = 0u32;
    let mut ids = Vec::new();
    let mut polys = Vec::new();
    for w in walks {
        ids.push((next..next + w.len() as u32).collect());
        next += w.len() as u32;
        polys.push(w.to_vec());
    }
    (ids, polys)
}

/// Same chart, but the walk entries whose index is in `alias` take the
/// id of the entry the map names (one vertex met twice).
fn alias(mut c: Chart, from: u32, to: u32) -> Chart {
    for l in &mut c.0 {
        for id in l.iter_mut() {
            if *id == from {
                *id = to;
            }
        }
    }
    c
}

fn shoelace(p: &[[f64; 2]]) -> f64 {
    let n = p.len();
    (0..n)
        .map(|i| p[i][0] * p[(i + 1) % n][1] - p[(i + 1) % n][0] * p[i][1])
        .sum::<f64>()
        / 2.0
}

fn run(c: &Chart) -> Result<Vec<[u32; 3]>, TessellateError> {
    let tris = triangulate_chart(FaceKey::default(), &c.0, &c.1)?;
    Ok(tris
        .into_iter()
        .map(|t| {
            t.map(|v| match v {
                PatchVertex::Shared(id) => id,
                other => panic!("{other:?}"),
            })
        })
        .collect())
}

fn watertight(c: &Chart) -> usize {
    let tris = run(c).unwrap_or_else(|e| panic!("refused: {e:?}"));
    let mut at: HashMap<u32, [f64; 2]> = HashMap::new();
    let mut walk_edges: HashMap<(u32, u32), u32> = HashMap::new();
    for (ids, p) in c.0.iter().zip(&c.1) {
        for i in 0..ids.len() {
            at.entry(ids[i]).or_insert(p[i]);
            *walk_edges.entry((ids[i], ids[(i + 1) % ids.len()])).or_insert(0) += 1;
        }
    }
    let mut edges: HashMap<(u32, u32), u32> = HashMap::new();
    let mut area = 0.0;
    for t in &tris {
        let [a, b, cc] = t.map(|i| at[&i]);
        let s = ((b[0] - a[0]) * (cc[1] - a[1]) - (b[1] - a[1]) * (cc[0] - a[0])) / 2.0;
        assert!(s > 0.0, "triangle {t:?} not CCW (area {s})");
        area += s;
        for k in 0..3 {
            *edges.entry((t[k], t[(k + 1) % 3])).or_insert(0) += 1;
        }
    }
    for (&(a, b), &n) in &edges {
        assert_eq!(n, 1, "directed edge {a}->{b} used {n} times");
        let rev = edges.contains_key(&(b, a));
        let walk = walk_edges.contains_key(&(a, b));
        assert!(rev ^ walk, "edge {a}->{b}: reverse {rev}, walk {walk}");
    }
    for e in walk_edges.keys() {
        assert!(edges.contains_key(e), "walk edge {e:?} unused");
    }
    let want: f64 = c.1.iter().map(|p| shoelace(p)).sum();
    assert!((area - want).abs() < 1e-9 * want.abs().max(1.0), "area {area} vs {want}");
    tris.len()
}

fn refuses(c: &Chart) -> bool {
    matches!(run(c), Err(TessellateError::PinchWedge { .. }))
}

const O: [f64; 2] = [0.0, 0.0];

fn polar(deg: f64, r: f64) -> [f64; 2] {
    let t = deg.to_radians();
    [r * t.cos(), r * t.sin()]
}

/// Three cones: a square less a notch to its bottom edge and two inner
/// notches, all three tips on the origin, so the one outer loop passes
/// the origin three times (ids 2, 5, 8).
fn three(crossed: bool) -> Chart {
    let (b, c) = if crossed {
        ([[1.0, 3.0], [3.0, 1.0]], [[-3.0, 1.0], [-1.0, 3.0]])
    } else {
        ([[-3.0, 1.0], [-1.0, 3.0]], [[1.0, 3.0], [3.0, 1.0]])
    };
    let w = [
        [-4.0, -4.0], [-1.0, -4.0], O, b[0], b[1], O, c[0], c[1], O,
        [1.0, -4.0], [4.0, -4.0], [4.0, 4.0], [-4.0, 4.0],
    ];
    chart(&[&w])
}

#[test]
fn p1_three_cones_mesh_watertight() {
    let n = watertight(&three(false));
    assert!(n > 0);
}

#[test]
fn p1x_three_cones_crossed_refuse() {
    assert!(refuses(&three(true)));
}

/// Two passes; the first's corner reflex (`~290°`), the second's
/// `wedge` degrees wide.
fn reflex_thin(wedge: f64) -> Chart {
    let base = [0.5, -3.0];
    let a = (-3.0f64).atan2(0.5).to_degrees() + 360.0; // ~279.46°
    let w = [
        [-3.0, -3.0], [-0.5, -3.0], O, polar(330.0, 2.0), polar(a + wedge, 2.0), O,
        base, [3.0, -3.0], [3.0, 3.0], [-3.0, 3.0],
    ];
    chart(&[&w])
}

#[test]
fn p2_reflex_and_thin_wedges_mesh_watertight() {
    for wedge in [20.0, 1.0, 0.5, 1e-2, 1e-4] {
        let n = watertight(&reflex_thin(wedge));
        assert!(n > 0, "wedge {wedge}");
    }
}

/// Ring touching the outer loop's notch tip at the origin: the outer
/// pass (id 2) and the ring's pass (id 7) are distinct vertices.
fn ring_touch() -> Chart {
    let outer = [[-3.0, -3.0], [-1.0, -3.0], O, [1.0, -3.0], [3.0, -3.0], [3.0, 3.0], [-3.0, 3.0]];
    let ring = [O, [-1.0, 2.0], [1.0, 2.0]];
    chart(&[&outer, &ring])
}

#[test]
fn p3_ring_touching_outer_loop_at_a_pinch() {
    // Report, not assert: what the lane does with two loops meeting.
    let r = run(&ring_touch());
    eprintln!("P3 ring touching outer at a pinch: {:?}", r.map(|t| t.len()));
}

/// Three passes, the first two the SAME vertex (one cone holding two
/// corners of the face) and the third another vertex: a non-crossing
/// face, ids 2, 2, 8.
#[test]
fn p4_one_vertex_twice_and_another_once() {
    let c = alias(three(false), 5, 2);
    let r = run(&c);
    eprintln!("P4 ids 2,2,8 (non-crossing): {:?}", r.as_ref().map(Vec::len));
    assert!(matches!(r, Err(TessellateError::PinchWedge { .. })), "{r:?}");
}

/// Control: all three passes the same vertex (one cone) meshes.
#[test]
fn p4c_one_vertex_three_times_meshes() {
    let c = alias(alias(three(false), 5, 2), 8, 2);
    let tris = run(&c).unwrap();
    assert!(!tris.is_empty());
}

/// The pinch on the chart's convex hull: a C whose two passes at the
/// origin hold the left and right sectors, the notch between them
/// `gap` degrees wide, and the hull's exterior below the origin.
fn hull_c(gap: f64) -> Chart {
    let w = [
        O, polar(90.0 + gap / 2.0, 3.0), polar(90.0 - gap / 2.0, 3.0), O,
        [3.0, 1.0], [3.0, 5.0], [-3.0, 5.0], [-3.0, 1.0],
    ];
    chart(&[&w])
}

#[test]
fn p5_pinch_on_hull_and_corners_adjacent_in_rotation() {
    for gap in [30.0, 1.0, 1e-3, 1e-6] {
        let n = watertight(&hull_c(gap));
        assert!(n >= 2, "gap {gap}");
    }
}
