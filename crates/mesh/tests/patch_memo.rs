//! **The patch memo answers the same mesh, and its key is complete** —
//! the rows `tessellate_with` is built under.
//!
//! Two shapes of row. The first is the bit-identity pin: over the D9
//! goldens' corpus, a mesh answered through the memo is byte-for-byte
//! the plain door's, on the first picture (every face a miss) and on
//! the second (every face a hit), and a δ change misses everything;
//! after each picture the memo holds exactly the picture's faces.
//!
//! The second is the differential on the key, through the body's own
//! doors: for each lane, one input the lane reads is changed and the
//! face MISSES; something the lane does not read is changed and the
//! face HITS. The inputs no public door can change in isolation — a
//! chord parameter without its carrier, ε without the process — are
//! pinned at the key itself, in `memo`'s unit rows, which state the
//! per-lane read list as a table.
//!
//! What a hit here means: the memo answered the face, so the lane did
//! not run. What the pin above adds: the answer it gave was right.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::Surface;
use geom_brep::{Pcurve, PcurveCache};
use geom_core::{Band, Point2, Tol};
use mesh::{PatchMemo, Tessellation, tessellate, tessellate_with};
use profile::{ProfileLoop, RawLoop};
use sweep::{Extrusion, extrude};
use topo::{Body, FaceKey, FaceSurface, HalfEdgeKey};

use crate::common;
use crate::d9_mesh_goldens;
use common::*;

const DELTA: f64 = 0.05;

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

/// Every byte of a mesh, the goldens' way.
fn digest(m: &mesh::Mesh) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    fnv(&mut h, m.positions.len() as u64);
    for p in &m.positions {
        fnv(&mut h, p.x.to_bits());
        fnv(&mut h, p.y.to_bits());
        fnv(&mut h, p.z.to_bits());
    }
    fnv(&mut h, m.patches.len() as u64);
    for q in &m.patches {
        fnv(&mut h, format!("{:?}", q.face).len() as u64);
        for b in format!("{:?}", q.face).bytes() {
            fnv(&mut h, u64::from(b));
        }
        fnv(&mut h, q.triangles.len() as u64);
        for t in &q.triangles {
            fnv(&mut h, u64::from(t[0]));
            fnv(&mut h, u64::from(t[1]));
            fnv(&mut h, u64::from(t[2]));
        }
    }
    fnv(&mut h, m.boundaries.len() as u64);
    for b in &m.boundaries {
        fnv(&mut h, b.points.len() as u64);
        for id in &b.points {
            fnv(&mut h, u64::from(*id));
        }
    }
    h
}

/// The corpus that meshes: name and body, one per lane and chart.
fn corpus() -> Vec<(&'static str, Body<f64>)> {
    let (above, _) = d9_mesh_goldens::tilted_halves();
    vec![
        ("l_prism", l_prism()),
        ("holed_prism", holed_prism()),
        ("wedge", wedge()),
        ("rounded_prism", rounded_prism()),
        ("ball", ball()),
        ("cone", cone()),
        ("washer", washer()),
        ("donut", donut()),
        ("tilted_above", above),
        ("loft_prism", d9_mesh_goldens::nurbs_bodies::loft_prism()),
    ]
}

fn through(memo: &mut PatchMemo, body: &Body<f64>, delta: f64) -> Tessellation {
    tessellate_with(body, delta, Tol::witness(), memo).expect("the body meshes through the memo")
}

#[test]
fn the_memo_answers_the_plain_doors_mesh_on_a_miss_and_on_a_hit() {
    for (name, body) in corpus() {
        let plain = digest(&tessellate(&body, DELTA, Tol::witness()).expect("the body meshes"));
        let faces = body.faces().count();
        let mut memo = PatchMemo::new();

        let first = through(&mut memo, &body, DELTA);
        assert_eq!(
            digest(&first.mesh),
            plain,
            "{name}: first picture (all misses)"
        );
        assert_eq!(
            (memo.hits(), memo.misses()),
            (0, faces),
            "{name}: first picture"
        );
        assert_eq!(first.keys.len(), faces, "{name}: one digest per face");
        memo.end_picture();
        assert_eq!(
            memo.len(),
            faces,
            "{name}: the memo holds the picture's faces"
        );

        let second = through(&mut memo, &body, DELTA);
        assert_eq!(
            digest(&second.mesh),
            plain,
            "{name}: second picture (all hits)"
        );
        assert_eq!(
            (memo.hits(), memo.misses()),
            (faces, 0),
            "{name}: second picture"
        );
        assert_eq!(
            second.keys, first.keys,
            "{name}: the same faces digest the same"
        );
        memo.end_picture();
        assert_eq!(memo.len(), faces, "{name}: still one picture's worth");

        // δ is in every lane's key: a finer picture misses everything,
        // and closing it drops the coarser one.
        let finer = through(&mut memo, &body, DELTA / 2.0);
        assert_eq!(
            digest(&finer.mesh),
            digest(&tessellate(&body, DELTA / 2.0, Tol::witness()).unwrap()),
            "{name}: the finer picture is the plain door's too"
        );
        assert_eq!(
            (memo.hits(), memo.misses()),
            (0, faces),
            "{name}: a δ change misses everything"
        );
        memo.end_picture();
        assert_eq!(memo.len(), faces, "{name}: the coarser picture was evicted");
    }
}

#[test]
fn two_bodies_that_share_a_face_share_its_entry() {
    // The L prism's two inner walls (y = 1 over x ∈ [1, 2], x = 1 over
    // y ∈ [1, 2]) are, plane, corners and outward side alike, the
    // holed prism's two matching hole walls: the memo keys by content,
    // so the two bodies' eighteen faces are sixteen entries.
    let a = l_prism();
    let b = holed_prism();
    let mut memo = PatchMemo::new();
    through(&mut memo, &a, DELTA);
    through(&mut memo, &b, DELTA);
    assert_eq!(
        memo.misses(),
        8 + 10 - 2,
        "two of b's faces were answered from a's"
    );
    memo.end_picture();
    assert_eq!(memo.len(), 16);
}

#[test]
fn a_kept_tessellation_survives_the_picture_and_an_unkept_one_does_not() {
    let a = l_prism();
    let b = washer();
    let mut memo = PatchMemo::new();
    let ta = through(&mut memo, &a, DELTA);
    let tb = through(&mut memo, &b, DELTA);
    memo.end_picture();
    let both = a.faces().count() + b.faces().count();
    assert_eq!(memo.len(), both);
    // The next picture reuses `a` whole (as a node-level hit would) and
    // never asks about `b`.
    memo.keep(&ta.keys);
    memo.end_picture();
    assert_eq!(
        memo.len(),
        a.faces().count(),
        "kept faces stay; b's were dropped"
    );
    let again = through(&mut memo, &a, DELTA);
    assert_eq!(
        memo.hits(),
        a.faces().count(),
        "a's faces are still answered"
    );
    assert_eq!(again.keys, ta.keys);
    let _ = tb;
}

/// The face of `body` on a surface `pick` accepts, by arena order.
fn face_where(body: &Body<f64>, pick: impl Fn(&Surface<f64>) -> bool) -> FaceKey {
    body.faces()
        .find(|(_, f)| pick(body.get_surface(f.surface).unwrap()))
        .map(|(k, _)| k)
        .expect("a face on such a surface")
}

/// The face-arena position of `fk`, which is the digest's index.
fn position_of(body: &Body<f64>, fk: FaceKey) -> usize {
    body.faces().position(|(k, _)| k == fk).unwrap()
}

/// Mesh `before` into a memo, close the picture, mesh `after` through
/// it, and answer which faces (by arena position) were misses.
fn misses_between(before: &Body<f64>, after: &Body<f64>) -> Vec<usize> {
    let mut memo = PatchMemo::new();
    let first = through(&mut memo, before, DELTA);
    memo.end_picture();
    let second = through(&mut memo, after, DELTA);
    // A miss is a face whose digest is not one the first picture
    // stored; the counters agree with that reading.
    let stored: std::collections::HashSet<_> = first.keys.iter().collect();
    let missed: Vec<usize> = second
        .keys
        .iter()
        .enumerate()
        .filter(|(_, d)| !stored.contains(d))
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        memo.misses(),
        missed.len(),
        "the counters and the digests agree"
    );
    assert_eq!(memo.hits(), second.keys.len() - missed.len());
    // And the answer is the plain door's, whatever was reused.
    assert_eq!(
        digest(&second.mesh),
        digest(&tessellate(after, DELTA, Tol::witness()).unwrap()),
        "the memo-assisted mesh of the edited body is the fresh one"
    );
    missed
}

#[test]
fn arena_keys_are_not_in_the_key_a_reminted_surface_key_hits_on_every_lane() {
    for (name, body) in corpus() {
        let mut after = body.clone();
        let faces: Vec<FaceKey> = after.faces().map(|(k, _)| k).collect();
        for fk in faces {
            let surface = after
                .get_surface(after.get_face(fk).unwrap().surface)
                .unwrap()
                .clone();
            after
                .set_face_surface(fk, FaceSurface::New(surface))
                .expect("the same surface under a new key attaches");
        }
        assert!(
            misses_between(&body, &after).is_empty(),
            "{name}: every surface key moved and no face missed"
        );
    }
}

#[test]
fn a_moved_vertex_misses_exactly_the_faces_that_read_its_chords() {
    // The L prism with one profile vertex moved: the two walls at that
    // vertex and both caps read a changed chord position; the other
    // four walls read nothing that moved. Planar lane throughout.
    let moved = {
        let lp = ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(2.0, 0.0),
            p2(2.0, 1.0),
            p2(1.0, 1.0),
            p2(1.0, 2.125),
            p2(0.0, 2.0),
        ]);
        extrude(
            &validated(vec![lp]),
            Extrusion::Distance(1.0),
            Tol::witness(),
        )
        .unwrap()
        .body
    };
    let base = l_prism();
    let missed = misses_between(&base, &moved);
    assert_eq!(missed.len(), 4, "two walls and two caps: {missed:?}");
    assert_eq!(base.faces().count(), 8);
}

#[test]
fn the_planar_lane_reads_neither_the_stored_plane_nor_the_sense() {
    let base = holed_prism();
    let fk = face_where(&base, |s| matches!(s, Surface::Plane { .. }));
    // A rotated stored frame: the planar lane derives its own from the
    // boundary (planar.rs, #284), so this is not an input it reads.
    let mut rotated = base.clone();
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *rotated
        .get_surface(rotated.get_face(fk).unwrap().surface)
        .unwrap()
    else {
        panic!("a plane")
    };
    rotated
        .set_face_surface(
            fk,
            FaceSurface::New(Surface::Plane {
                origin,
                normal,
                u_ref: normal.cross(u_ref),
            }),
        )
        .unwrap();
    assert!(
        misses_between(&base, &rotated).is_empty(),
        "a rotated stored frame hits"
    );

    let mut flipped = base.clone();
    let sense = flipped.get_face(fk).unwrap().sense;
    flipped.set_face_sense(fk, !sense).unwrap();
    assert!(
        misses_between(&base, &flipped).is_empty(),
        "a flipped sense hits on a planar face"
    );
}

#[test]
fn the_curved_lane_misses_when_its_chart_or_sense_changes_and_nothing_else_does() {
    let base = rounded_prism();
    let fk = face_where(&base, |s| matches!(s, Surface::Cylinder { .. }));
    let at = position_of(&base, fk);

    // The chart's u reference rotated a quarter turn about the axis:
    // the same cylinder, the same rims and meridians, the same chord
    // points — a surface parameter the walk reads and the chords do
    // not.
    let mut rotated = base.clone();
    let Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref,
    } = *rotated
        .get_surface(rotated.get_face(fk).unwrap().surface)
        .unwrap()
    else {
        panic!("a cylinder")
    };
    rotated
        .set_face_surface(
            fk,
            FaceSurface::New(Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref: axis.cross(u_ref),
            }),
        )
        .unwrap();
    assert_eq!(
        misses_between(&base, &rotated),
        vec![at],
        "only the re-charted wall misses"
    );

    let mut flipped = base.clone();
    let sense = flipped.get_face(fk).unwrap().sense;
    flipped.set_face_sense(fk, !sense).unwrap();
    assert_eq!(
        misses_between(&base, &flipped),
        vec![at],
        "only the re-sensed wall misses"
    );
}

#[test]
fn the_trimmed_lane_misses_when_a_pcurve_changes_and_hits_when_a_plane_does() {
    let (base, _) = d9_mesh_goldens::tilted_halves();
    let wall = face_where(&base, |s| matches!(s, Surface::Cylinder { .. }));
    let at = position_of(&base, wall);
    let tol = Tol::witness();

    // Every pcurve of the wall's loop with its u shifted by a full
    // turn: the same points on the periodic chart, a consistent
    // polygon, different bytes. (One edge shifted alone would open the
    // polygon by 2π and the lane would refuse the wall typed; a
    // refusal proves the lane read the pcurve but shows no hit.)
    let surface = base
        .get_surface(base.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    let mut moved = base.clone();
    let mut shifted_any = false;
    let wall_half_edges: Vec<HalfEdgeKey> = base
        .half_edges()
        .filter(|(_, he)| base.get_loop(he.parent_loop).unwrap().face == wall)
        .map(|(hek, _)| hek)
        .collect();
    for hek in wall_half_edges {
        let Some(cache) = base.pcurve(hek) else {
            continue;
        };
        let cache: PcurveCache<f64> = cache.clone();
        let shifted = match cache.pcurve().clone() {
            Pcurve::Harmonic { p0, pa, pb, pl } => Pcurve::Harmonic {
                p0: Point2::new(p0.x + TAU, p0.y),
                pa,
                pb,
                pl,
            },
            Pcurve::IsoLine { p0, pl } => Pcurve::IsoLine {
                p0: Point2::new(p0.x + TAU, p0.y),
                pl,
            },
            Pcurve::IsoArc {
                p0,
                pd,
                t0,
                angle,
                breaks,
            } => Pcurve::IsoArc {
                p0: Point2::new(p0.x + TAU, p0.y),
                pd,
                t0,
                angle,
                breaks,
            },
            Pcurve::Fitted(_) | Pcurve::General(_) => panic!("an analytic chart carries no fit"),
        };
        let (t0, t1) = cache.params();
        let he = base.get_half_edge(hek).unwrap();
        let edge = base.get_edge(he.edge).unwrap();
        let carrier = base
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap()
            .carrier()
            .clone();
        let recertified = PcurveCache::certify(
            shifted.clone(),
            t0,
            t1,
            &carrier,
            &surface,
            shifted.chart_box(t0, t1),
            Band::linear(tol).unwrap(),
        )
        .expect("the shifted pcurve certifies on the periodic chart");
        moved.attach_pcurve(hek, recertified);
        shifted_any = true;
    }
    assert!(shifted_any, "the wall's loop carries stored pcurves");
    assert_eq!(
        misses_between(&base, &moved),
        vec![at],
        "only the wall reads its pcurves"
    );

    // The cap's stored plane rotated: the wall reads its own chart and
    // its neighbours' chords, not a neighbour's plane.
    let cap = face_where(&base, |s| matches!(s, Surface::Plane { .. }));
    let mut rotated = base.clone();
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = *rotated
        .get_surface(rotated.get_face(cap).unwrap().surface)
        .unwrap()
    else {
        panic!("a plane")
    };
    rotated
        .set_face_surface(
            cap,
            FaceSurface::New(Surface::Plane {
                origin,
                normal,
                u_ref: normal.cross(u_ref),
            }),
        )
        .unwrap();
    assert!(
        misses_between(&base, &rotated).is_empty(),
        "no lane reads a stored plane"
    );
}

#[test]
fn the_trimmed_nurbs_lane_misses_when_its_surface_changes() {
    let base = d9_mesh_goldens::nurbs_bodies::loft_prism();
    let fk = face_where(&base, |s| matches!(s, Surface::Nurbs(_)));
    let at = position_of(&base, fk);
    // The same net under a new key hits (the row above, on every
    // lane); the fit itself moved by one weight misses. A weight is
    // a surface parameter the lane reads and the chords, which come
    // from the edges' carriers, do not.
    let Surface::Nurbs(net) = base
        .get_surface(base.get_face(fk).unwrap().surface)
        .unwrap()
    else {
        panic!("a nurbs")
    };
    let mut weights = net.weights().to_vec();
    weights[0] *= 1.0 + 1.0 / 64.0;
    let moved = geom::NurbsSurface::new(
        net.knots_u().clone(),
        net.knots_v().clone(),
        net.control().to_vec(),
        weights,
    )
    .expect("a reweighted net");
    let mut after = base.clone();
    after
        .set_face_surface(
            fk,
            FaceSurface::New(Surface::Nurbs(std::sync::Arc::new(moved))),
        )
        .unwrap();
    // The chord pass reads a NURBS face's certified bound to size the
    // chords of its edges (`chords::nurbs_tighten`), so the wall's
    // reweighting moves the chord points of every edge it shares —
    // and every face across those edges reads those points. The
    // expected misses are therefore the wall AND its edge-neighbours;
    // the wall opposite, which shares no edge with it, hits.
    let wall_edges: std::collections::HashSet<_> = base
        .half_edges()
        .filter(|(_, he)| base.get_loop(he.parent_loop).unwrap().face == fk)
        .map(|(_, he)| he.edge)
        .collect();
    let mut expected: Vec<usize> = base
        .faces()
        .enumerate()
        .filter(|(_, (other, _))| {
            base.half_edges().any(|(_, he)| {
                base.get_loop(he.parent_loop).unwrap().face == *other
                    && wall_edges.contains(&he.edge)
            })
        })
        .map(|(i, _)| i)
        .collect();
    expected.sort_unstable();
    assert!(expected.contains(&at));
    assert_eq!(
        expected.len(),
        5,
        "four neighbours and the wall, of six faces"
    );
    assert_eq!(
        misses_between(&base, &after),
        expected,
        "the wall and its edge-neighbours miss"
    );
}
