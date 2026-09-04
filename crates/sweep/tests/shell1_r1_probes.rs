//! SHELL-1 — R1 blinded review probes (probe branch only; NOT part of
//! the PR under review). One audit, run over fixtures the PR's own rows
//! do not draw:
//!
//! - the face channels partition the live faces; the edge/vertex
//!   channels partition as survivors ⊎ inner twins;
//! - `dead` is EXACT: as a set it equals every key the record or the
//!   operand mentions that no longer resolves, with no duplicates, and
//!   nothing in it resolves;
//! - every `inner*` source column is the operand arena, in order;
//! - every ring row: result edge on the ring in cycle order, source a
//!   boundary edge of the designated chart in the OPERAND, and the pair
//!   present verbatim in `inner_edges`;
//! - the record is a function of the construction (Debug-equal twice).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, LoopKey, ShellError, Shelled, VertexKey};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

const FIT_TOL: f64 = 1e-6;
/// tan(pi/8): the 3-4-5 belly arc of the tour teapot, centred ON the axis (a sphere zone).
const BULGE: f64 = 0.414_213_562_373_095_1;

fn polygon(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

/// A closed meridian with a per-vertex bulge (an arc leaves a vertex
/// whose bulge is nonzero).
fn bulged(pts: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        pts.iter()
            .map(|&(x, y, b)| ProfileVertex::new(p2(x, y), b))
            .collect(),
    )
}

fn revolved_loop(lp: ProfileLoop<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("meridian revolves")
    .body
}

fn revolved_full(pts: &[(f64, f64)]) -> Body<f64> {
    revolved_loop(polygon(pts))
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("profile validates");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("profile extrudes")
        .body
}

fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn plane_chart_at_z(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-12
                        && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

fn loop_edges(body: &Body<f64>, lk: LoopKey) -> Vec<EdgeKey> {
    let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("loop").boundary else {
        return Vec::new();
    };
    body.loop_cycle(first)
        .expect("cycle")
        .into_iter()
        .map(|he| body.get_half_edge(he).expect("he").edge)
        .collect()
}

fn loop_vertices(body: &Body<f64>, lk: LoopKey) -> Vec<VertexKey> {
    let LoopBoundary::Cycle { first } = body.get_loop(lk).expect("loop").boundary else {
        return Vec::new();
    };
    body.loop_cycle(first)
        .expect("cycle")
        .into_iter()
        .map(|he| body.get_half_edge(he).expect("he").start)
        .collect()
}

fn face_loops(body: &Body<f64>, f: FaceKey) -> Vec<LoopKey> {
    let d = body.get_face(f).expect("face");
    core::iter::once(d.outer)
        .chain(d.rings.iter().copied())
        .collect()
}

fn sorted_dedup<K: Ord + Copy>(v: &[K]) -> (Vec<K>, bool) {
    let mut s = v.to_vec();
    s.sort();
    let n = s.len();
    s.dedup();
    let dups = s.len() != n;
    (s, dups)
}

/// The whole audit, against the operand AND the result.
fn audit(what: &str, source: &Body<f64>, chart: &[FaceKey], shelled: &Shelled<f64>) {
    let (body, rec) = (&shelled.body, &shelled.naming);
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "{what}: tier 3"
    );

    // ---- outer: every undesignated operand face, arena order, (k,k), live.
    let expect_outer: Vec<FaceKey> = source
        .faces()
        .map(|(k, _)| k)
        .filter(|k| !chart.contains(k))
        .collect();
    let outer_src: Vec<FaceKey> = rec.outer.iter().map(|&(_, s)| s).collect();
    assert_eq!(
        outer_src, expect_outer,
        "{what}: outer's source column is the undesignated operand faces in arena order"
    );
    for &(r, s) in &rec.outer {
        assert_eq!(r, s, "{what}: a survivor keeps its key");
        assert!(
            body.get_face(r).is_some(),
            "{what}: outer {r:?} does not resolve"
        );
    }

    // ---- inner*: source columns are the operand arenas, in order.
    let src_faces: Vec<FaceKey> = source.faces().map(|(k, _)| k).collect();
    let src_edges: Vec<EdgeKey> = source.edges().map(|(k, _)| k).collect();
    let src_vertices: Vec<VertexKey> = source.vertices().map(|(k, _)| k).collect();
    assert_eq!(
        rec.inner.iter().map(|&(_, s)| s).collect::<Vec<_>>(),
        src_faces,
        "{what}: inner"
    );
    assert_eq!(
        rec.inner_edges.iter().map(|&(_, s)| s).collect::<Vec<_>>(),
        src_edges,
        "{what}: inner_edges"
    );
    assert_eq!(
        rec.inner_vertices
            .iter()
            .map(|&(_, s)| s)
            .collect::<Vec<_>>(),
        src_vertices,
        "{what}: inner_vertices"
    );
    // result columns injective
    assert!(
        !sorted_dedup(&rec.inner.iter().map(|&(r, _)| r).collect::<Vec<_>>()).1,
        "{what}: inner result dup"
    );
    assert!(
        !sorted_dedup(&rec.inner_edges.iter().map(|&(r, _)| r).collect::<Vec<_>>()).1,
        "{what}: inner_edges result dup"
    );
    assert!(
        !sorted_dedup(
            &rec.inner_vertices
                .iter()
                .map(|&(r, _)| r)
                .collect::<Vec<_>>()
        )
        .1,
        "{what}: inner_vertices result dup"
    );
    // a twin never aliases a survivor key
    for &(r, _) in &rec.inner {
        assert!(
            source.get_face(r).is_none(),
            "{what}: twin face {r:?} aliases an operand key"
        );
    }
    for &(r, _) in &rec.inner_edges {
        assert!(
            source.get_edge(r).is_none(),
            "{what}: twin edge {r:?} aliases an operand key"
        );
    }
    for &(r, _) in &rec.inner_vertices {
        assert!(
            source.get_vertex(r).is_none(),
            "{what}: twin vertex {r:?} aliases an operand key"
        );
    }

    // ---- faces partition.
    let mut named: Vec<FaceKey> = rec.outer.iter().map(|&(r, _)| r).collect();
    for &(twin, _) in &rec.inner {
        if body.get_face(twin).is_some() {
            named.push(twin);
        }
    }
    for rim in &rec.rims {
        named.push(rim.rim);
        named.extend(rim.holes.iter().map(|&(f, _)| f));
    }
    let (named_s, dups) = sorted_dedup(&named);
    assert!(!dups, "{what}: a face is named twice: {named:?}");
    let (live, _) = sorted_dedup(&body.faces().map(|(k, _)| k).collect::<Vec<_>>());
    assert_eq!(
        named_s, live,
        "{what}: face channels do not partition the live faces"
    );

    // ---- edges / vertices partition: survivor xor twin.
    let twin_e: Vec<EdgeKey> = rec.inner_edges.iter().map(|&(r, _)| r).collect();
    for (e, _) in body.edges() {
        assert!(
            twin_e.contains(&e) != source.get_edge(e).is_some(),
            "{what}: edge {e:?} partition"
        );
    }
    let twin_v: Vec<VertexKey> = rec.inner_vertices.iter().map(|&(r, _)| r).collect();
    for (v, _) in body.vertices() {
        assert!(
            twin_v.contains(&v) != source.get_vertex(v).is_some(),
            "{what}: vertex {v:?} partition"
        );
    }

    // ---- dead: EXACT. mentioned ∧ ¬resolves == dead, as sets, no dups.
    let mentioned_f: Vec<FaceKey> = src_faces
        .iter()
        .copied()
        .chain(rec.inner.iter().map(|&(r, _)| r))
        .chain(
            rec.rims
                .iter()
                .flat_map(|r| r.holes.iter().map(|&(f, _)| f)),
        )
        .collect();
    let (expect_dead_f, _) = sorted_dedup(
        &mentioned_f
            .iter()
            .copied()
            .filter(|&f| body.get_face(f).is_none())
            .collect::<Vec<_>>(),
    );
    let (dead_f, dup_f) = sorted_dedup(&rec.dead.faces);
    assert!(
        !dup_f,
        "{what}: dead.faces has duplicates: {:?}",
        rec.dead.faces
    );
    assert_eq!(
        dead_f, expect_dead_f,
        "{what}: dead.faces is not exactly the retired mentioned faces"
    );
    let mentioned_e: Vec<EdgeKey> = src_edges
        .iter()
        .copied()
        .chain(twin_e.iter().copied())
        .collect();
    let (expect_dead_e, _) = sorted_dedup(
        &mentioned_e
            .iter()
            .copied()
            .filter(|&e| body.get_edge(e).is_none())
            .collect::<Vec<_>>(),
    );
    let (dead_e, dup_e) = sorted_dedup(&rec.dead.edges);
    assert!(!dup_e, "{what}: dead.edges has duplicates");
    assert_eq!(
        dead_e, expect_dead_e,
        "{what}: dead.edges is not exactly the retired mentioned edges"
    );
    let mentioned_v: Vec<VertexKey> = src_vertices
        .iter()
        .copied()
        .chain(twin_v.iter().copied())
        .collect();
    let (expect_dead_v, _) = sorted_dedup(
        &mentioned_v
            .iter()
            .copied()
            .filter(|&v| body.get_vertex(v).is_none())
            .collect::<Vec<_>>(),
    );
    let (dead_v, dup_v) = sorted_dedup(&rec.dead.vertices);
    assert!(!dup_v, "{what}: dead.vertices has duplicates");
    assert_eq!(
        dead_v, expect_dead_v,
        "{what}: dead.vertices is not exactly the retired mentioned vertices"
    );

    // ---- rims.
    let mut seen_sources: Vec<FaceKey> = Vec::new();
    for (i, rim) in rec.rims.iter().enumerate() {
        assert!(!rim.sources.is_empty(), "{what}: rim {i} names no source");
        for s in &rim.sources {
            assert!(
                chart.contains(s),
                "{what}: rim {i} source {s:?} was not designated"
            );
            assert!(!seen_sources.contains(s), "{what}: {s:?} in two rims");
            seen_sources.push(*s);
        }
        assert!(
            rim.sources.contains(&rim.rim),
            "{what}: rim {i} face is not the chart's survivor"
        );
        let data = body
            .get_face(rim.rim)
            .unwrap_or_else(|| panic!("{what}: rim {i} face does not resolve"));
        assert!(
            data.rings.contains(&rim.ring),
            "{what}: rim {i}'s ring is not a ring of its face"
        );
        // ring rows in cycle order
        let on_ring = loop_edges(body, rim.ring);
        assert_eq!(
            rim.ring_edges.iter().map(|&(r, _)| r).collect::<Vec<_>>(),
            on_ring,
            "{what}: ring_edges result column is not the ring cycle"
        );
        let on_ring_v = loop_vertices(body, rim.ring);
        assert_eq!(
            rim.ring_vertices
                .iter()
                .map(|&(r, _)| r)
                .collect::<Vec<_>>(),
            on_ring_v,
            "{what}: ring_vertices result column is not the ring cycle"
        );
        // sources bound the chart in the OPERAND
        let mut bounding_e: Vec<EdgeKey> = Vec::new();
        let mut bounding_v: Vec<VertexKey> = Vec::new();
        for &f in &rim.sources {
            for lk in face_loops(source, f) {
                bounding_e.extend(loop_edges(source, lk));
                bounding_v.extend(loop_vertices(source, lk));
            }
        }
        for &(r, s) in &rim.ring_edges {
            assert!(
                bounding_e.contains(&s),
                "{what}: ring edge source {s:?} does not bound the chart in the operand"
            );
            assert!(
                rec.inner_edges.contains(&(r, s)),
                "{what}: ring row {r:?}<-{s:?} absent from inner_edges verbatim"
            );
        }
        for &(r, s) in &rim.ring_vertices {
            assert!(
                bounding_v.contains(&s),
                "{what}: ring vertex source {s:?} does not bound the chart in the operand"
            );
            assert!(
                rec.inner_vertices.contains(&(r, s)),
                "{what}: ring vertex row absent from inner_vertices verbatim"
            );
        }
        // holes
        for &(promoted, src_ring) in &rim.holes {
            let d = body
                .get_face(promoted)
                .unwrap_or_else(|| panic!("{what}: promoted {promoted:?} does not resolve"));
            assert_eq!(
                d.rings,
                vec![src_ring],
                "{what}: promoted face's ring is the row's"
            );
            // The docs call `src_ring` a SOURCE key. Measured below (r1_hole_rows_...):
            // on the slit cap it is a loop `kemr` minted in the RESULT.
            let in_operand = rim.sources.iter().any(|&f| {
                source
                    .get_face(f)
                    .expect("src face")
                    .rings
                    .contains(&src_ring)
            });
            println!(
                "[{what}] hole row ring {src_ring:?}: resolves in the operand as a designated face's ring = {in_operand}"
            );
        }
    }
    // every designated chart got a rim row
    for f in chart {
        assert!(
            seen_sources.contains(f),
            "{what}: designated {f:?} appears in no rim row"
        );
    }
    // an undesignated operand face never dies
    for f in &expect_outer {
        assert!(
            body.get_face(*f).is_some(),
            "{what}: undesignated {f:?} died"
        );
    }
}

fn build(what: &str, source: &Body<f64>, chart: &[FaceKey], t: f64) -> Shelled<f64> {
    topo::shell_open(source, t, chart, FIT_TOL, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: must build: {e}"))
}

fn audit_twice(what: &str, source: &Body<f64>, chart: &[FaceKey], t: f64) -> Shelled<f64> {
    let a = build(what, source, chart, t);
    let b = build(what, source, chart, t);
    assert_eq!(
        format!("{:?}", a.naming),
        format!("{:?}", b.naming),
        "{what}: D9"
    );
    audit(what, source, chart, &a);
    println!(
        "[{what}] F={} E={} V={} rims={} dead=({},{},{}) holes={}",
        a.body.faces().count(),
        a.body.edges().count(),
        a.body.vertices().count(),
        a.naming.rims.len(),
        a.naming.dead.faces.len(),
        a.naming.dead.edges.len(),
        a.naming.dead.vertices.len(),
        a.naming.rims.iter().map(|r| r.holes.len()).sum::<usize>()
    );
    a
}

#[test]
fn r1_box_sealed_cup_and_tube() {
    let body = extruded(
        vec![polygon(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)])],
        4.0,
    );
    let top = plane_chart_at_z(&body, 4.0);
    let bottom = plane_chart_at_z(&body, 0.0);
    audit_twice("box sealed", &body, &[], 0.25);
    audit_twice("box cup", &body, &top, 0.25);
    let both: Vec<FaceKey> = top.iter().chain(&bottom).copied().collect();
    let tube = audit_twice("box tube", &body, &both, 0.25);
    assert_eq!(tube.naming.rims.len(), 2);
    // designation order is the rim order
    assert_eq!(tube.naming.rims[0].sources, top);
    assert_eq!(tube.naming.rims[1].sources, bottom);
    let rev: Vec<FaceKey> = bottom.iter().chain(&top).copied().collect();
    let tube2 = audit_twice("box tube reversed", &body, &rev, 0.25);
    assert_eq!(tube2.naming.rims[0].sources, bottom);
}

#[test]
fn r1_vessel_cup_and_both_caps() {
    let body = revolved_full(&[(0.0, 0.0), (0.5, 0.0), (0.5, 0.4), (0.0, 0.4)]);
    let top = plane_chart_at_y(&body, 0.4);
    let bottom = plane_chart_at_y(&body, 0.0);
    assert_eq!(top.len(), 2);
    audit_twice("vessel sealed", &body, &[], 0.05);
    audit_twice("vessel cup", &body, &top, 0.05);
    let both: Vec<FaceKey> = top.iter().chain(&bottom).copied().collect();
    let t = audit_twice("vessel both caps (two seamed charts)", &body, &both, 0.05);
    assert_eq!(t.naming.rims.len(), 2);
    assert_eq!(t.body.shells().count(), 1);
}

#[test]
fn r1_annular_tube_cup() {
    let body = revolved_full(&[(0.30, 0.0), (0.50, 0.0), (0.50, 0.40), (0.30, 0.40)]);
    let top = plane_chart_at_y(&body, 0.40);
    let bottom = plane_chart_at_y(&body, 0.0);
    audit_twice("tube sealed", &body, &[], 0.05);
    let c = audit_twice("tube cup (slit cap)", &body, &top, 0.05);
    assert_eq!(c.naming.rims[0].holes.len(), 1);
    let both: Vec<FaceKey> = top.iter().chain(&bottom).copied().collect();
    // Both caps of an annular tube disconnect the inner wall from the
    // outer: the standing gate, recorded rather than audited.
    match topo::shell_open(&body, 0.05, &both, FIT_TOL, Tol::witness()) {
        Err(ShellError::OpenFacesDisconnect { components, .. }) => {
            println!("[tube both] disconnect gate: {components}")
        }
        Err(e) => panic!("[tube both] other refusal: {e}"),
        Ok(s) => audit("tube both caps", &body, &both, &s),
    }
}

#[test]
fn r1_p6_one_holed_slab() {
    let (w, d, h, t) = (1.0, 0.8, 0.3, 0.04);
    let s = 0.15;
    let (cx, cy) = (0.5, 0.4);
    let outer = polygon(&[(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]);
    let hole = polygon(&[
        (cx - s, cy - s),
        (cx + s, cy - s),
        (cx + s, cy + s),
        (cx - s, cy + s),
    ]);
    let body = extruded(vec![outer, hole], h);
    let top = plane_chart_at_z(&body, h);
    assert_eq!(top.len(), 1);
    audit_twice("p6 slab sealed", &body, &[], t);
    let c = audit_twice("p6 slab cup", &body, &top, t);
    assert_eq!(c.naming.rims[0].holes.len(), 1);
}

#[test]
fn r1_p2_counterbore_is_still_the_disconnect_refusal() {
    let (ro, rb, h, depth, t) = (0.5, 0.2, 0.4, 0.2, 0.05);
    let body = revolved_full(&[
        (0.0, 0.0),
        (ro, 0.0),
        (ro, h),
        (rb, h),
        (rb, h - depth),
        (0.0, h - depth),
    ]);
    let chart = plane_chart_at_y(&body, h);
    match topo::shell_open(&body, t, &chart, FIT_TOL, Tol::witness()) {
        Err(ShellError::OpenFacesDisconnect { components, .. }) => {
            println!("[p2] refused at the disconnect gate: {components}");
        }
        Err(e) => panic!("[p2] other refusal: {e}"),
        Ok(s) => {
            audit("p2 counterbore", &body, &chart, &s);
        }
    }
}

/// A cap worn by MORE than two faces: a mid-cap station on the
/// meridian revolves to a circle on the cap, so the plane is worn by
/// (disc + annulus) × (two seam halves).
#[test]
fn r1_chart_worn_by_several_faces() {
    let (r, h) = (0.5, 0.4);
    let body = revolved_full(&[(0.0, 0.0), (r, 0.0), (r, h), (0.5 * r, h), (0.0, h)]);
    let top = plane_chart_at_y(&body, h);
    println!("[several] cap chart has {} faces", top.len());
    match topo::shell_open(&body, 0.05, &top, FIT_TOL, Tol::witness()) {
        Err(e) => println!("[several] REFUSED typed: {e}"),
        Ok(s) => {
            audit("several-face cap", &body, &top, &s);
            let s2 = build("several-face cap", &body, &top, 0.05);
            assert_eq!(format!("{:?}", s.naming), format!("{:?}", s2.naming));
            println!(
                "[several] BUILT; rims={} dead faces={}",
                s.naming.rims.len(),
                s.naming.dead.faces.len()
            );
        }
    }
}

/// A bellied pot (sphere-zone wall through the axial door), sealed and
/// opened at the mouth.
#[test]
fn r1_bellied_pot() {
    // (r,0) -> arc to (r,h): bulge on the wall segment.
    let body = revolved_loop(bulged(&[
        (0.0, 0.0, 0.0),
        (4.0 / 64.0, 0.0, 0.0),
        (4.0 / 64.0, 1.0 / 64.0, BULGE),
        (3.0 / 64.0, 8.0 / 64.0, 0.0),
        (0.0, 8.0 / 64.0, 0.0),
    ]));
    let top = plane_chart_at_y(&body, 8.0 / 64.0);
    println!(
        "[belly] faces {} mouth chart {}",
        body.faces().count(),
        top.len()
    );
    audit_twice("belly sealed", &body, &[], 1.0 / 128.0);
    for i in 0..3 {
        match topo::shell_open(&body, 1.0 / 128.0, &top, FIT_TOL, Tol::witness()) {
            Ok(s) => {
                println!("[belly cup] build {i}: OK F={}", s.body.faces().count());
                audit("belly cup", &body, &top, &s);
            }
            Err(e) => panic!("[belly cup] build {i}: REFUSED {e}"),
        }
    }
}

/// A vase opened at its BOTTOM (the lift running the other way).
#[test]
fn r1_vase_bottom_open() {
    let body = revolved_full(&[
        (0.0, 0.0),
        (0.21, 0.0),
        (0.21, 0.07),
        (0.34, 0.07),
        (0.34, 0.19),
        (0.11, 0.19),
        (0.11, 0.31),
        (0.0, 0.31),
    ]);
    let bottom = plane_chart_at_y(&body, 0.0);
    audit_twice("vase bottom", &body, &bottom, 0.02);
}

/// **FINDING.** `RimNaming::holes` is documented as `(promoted face,
/// SOURCE ring loop)`. On the extruded holed square the loop key is the
/// operand's; on the revolve's annular (slit) cap the operand face has
/// NO ring — the ring is minted by `kemr` in the result during
/// `canonicalize_chart`, so the key names nothing in the operand.
#[test]
fn r1_hole_rows_source_key_claim_is_false_on_the_slit_cap() {
    let body = revolved_full(&[(0.30, 0.0), (0.50, 0.0), (0.50, 0.40), (0.30, 0.40)]);
    let top = plane_chart_at_y(&body, 0.40);
    assert_eq!(top.len(), 1);
    assert!(
        body.get_face(top[0]).unwrap().rings.is_empty(),
        "the slit cap carries no ring in the operand"
    );
    let s = build("tube cup", &body, &top, 0.05);
    let (_, ring) = s.naming.rims[0].holes[0];
    assert!(s.body.get_loop(ring).is_some(), "it resolves in the RESULT");
    assert!(
        body.get_loop(ring).is_none() || !body.get_face(top[0]).unwrap().rings.contains(&ring),
        "the docs' 'source key' claim would hold — retract the finding"
    );
    println!("[finding] holes[0].1 = {ring:?} is a result-minted loop, not a source key");

    let slab = extruded(
        vec![
            polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 0.8), (0.0, 0.8)]),
            polygon(&[(0.35, 0.25), (0.65, 0.25), (0.65, 0.55), (0.35, 0.55)]),
        ],
        0.3,
    );
    let top = plane_chart_at_z(&slab, 0.3);
    let s = build("slab cup", &slab, &top, 0.04);
    let (_, ring) = s.naming.rims[0].holes[0];
    assert!(
        slab.get_face(top[0]).unwrap().rings.contains(&ring),
        "on the slab it IS the operand's ring"
    );
}
