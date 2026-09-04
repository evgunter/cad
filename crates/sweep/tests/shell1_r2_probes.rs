//! SHELL-1 — R2 blinded review probes (lane shell-1-r2).
//!
//! Probe branch only; NOT part of the PR under review. Fixtures the
//! shipped rows do not draw:
//!
//! - Q1: the key space of `RimNaming::holes`' second column (the docs
//!   say "source key") on a hole that is BORN in the surgery vs one
//!   that already exists in the operand;
//! - Q2: coverage on a ONE-SQUARE-HOLED slab (`shellfix1_r1_probes` P6);
//! - Q3: coverage on TWO designated charts that BOTH carry holes;
//! - Q4: the ring rows' sources are boundary edges of the designated
//!   chart in the OPERAND, on the holed fixtures;
//! - Q5: a designated chart worn by several faces named in a
//!   non-arena order (which face becomes the rim);
//! - Q6: determinism across the holed fixtures.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, LoopKey, ShellNaming, VertexKey};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

const FIT_TOL: f64 = 1e-6;

fn polygon(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    )
}

fn extruded(loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("profile validates");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("profile extrudes")
        .body
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

fn loop_edges(body: &Body<f64>, r#loop: LoopKey) -> Vec<EdgeKey> {
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).expect("the loop").boundary else {
        return Vec::new();
    };
    body.loop_cycle(first)
        .expect("the cycle")
        .into_iter()
        .map(|he| body.get_half_edge(he).expect("the half-edge").edge)
        .collect()
}

fn boundary_edges(body: &Body<f64>, faces: &[FaceKey]) -> Vec<EdgeKey> {
    let mut out = Vec::new();
    for &face in faces {
        let data = body.get_face(face).expect("the face");
        for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
            for edge in loop_edges(body, lk) {
                if !out.contains(&edge) {
                    out.push(edge);
                }
            }
        }
    }
    out
}

/// The coverage assertions of the shipped row, re-run on a fixture it
/// does not draw. Returns nothing; panics on a gap.
fn assert_covers(what: &str, source: &Body<f64>, body: &Body<f64>, record: &ShellNaming) {
    let mut named: Vec<FaceKey> = record.outer.iter().map(|&(r, _)| r).collect();
    for &(twin, _) in &record.inner {
        if body.get_face(twin).is_some() {
            named.push(twin);
        } else {
            assert!(
                record.dead.faces.contains(&twin),
                "{what}: the twin {twin:?} neither resolves nor is retired"
            );
        }
    }
    for rim in &record.rims {
        named.push(rim.rim);
        named.extend(rim.holes.iter().map(|&(f, _)| f));
    }
    let mut live: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    named.sort();
    let before = named.len();
    named.dedup();
    assert_eq!(before, named.len(), "{what}: a face is named twice");
    live.sort();
    assert_eq!(named, live, "{what}: the face channels do not cover");

    let twins: Vec<EdgeKey> = record.inner_edges.iter().map(|&(r, _)| r).collect();
    for (edge, _) in body.edges() {
        let twin = twins.contains(&edge);
        let survivor = source.get_edge(edge).is_some();
        assert!(twin != survivor, "{what}: {edge:?} is not exactly one of a twin/survivor");
    }
    let vtwins: Vec<VertexKey> = record.inner_vertices.iter().map(|&(r, _)| r).collect();
    for (vertex, _) in body.vertices() {
        let twin = vtwins.contains(&vertex);
        let survivor = source.get_vertex(vertex).is_some();
        assert!(
            twin != survivor,
            "{what}: {vertex:?} is not exactly one of a twin/survivor"
        );
    }
    for &face in &record.dead.faces {
        assert!(body.get_face(face).is_none(), "{what}: a retired face resolves");
    }
    for &edge in &record.dead.edges {
        assert!(body.get_edge(edge).is_none(), "{what}: a retired edge resolves");
    }
    for &vertex in &record.dead.vertices {
        assert!(
            body.get_vertex(vertex).is_none(),
            "{what}: a retired vertex resolves"
        );
    }
    // Ring rows: on the ring, live twins, sources in the operand.
    for rim in &record.rims {
        let on_ring = loop_edges(body, rim.ring);
        assert_eq!(rim.ring_edges.len(), on_ring.len(), "{what}: one row per ring edge");
        let bounding = boundary_edges(source, &rim.sources);
        for &(result, src) in &rim.ring_edges {
            assert!(on_ring.contains(&result), "{what}: a ring row's edge is off the ring");
            assert!(twins.contains(&result), "{what}: a ring edge is not an inner twin");
            assert!(
                bounding.contains(&src),
                "{what}: a ring row's source {src:?} did not bound the designated chart"
            );
        }
        for &(_, src) in &rim.ring_vertices {
            assert!(
                source.get_vertex(src).is_some(),
                "{what}: a ring vertex's source is not an operand vertex"
            );
        }
    }
}

/// A `w x d x h` slab with one square through-hole (P6's fixture).
fn holed_slab(w: f64, d: f64, h: f64, s: f64) -> Body<f64> {
    let (cx, cy) = (0.5 * w, 0.5 * d);
    extruded(
        vec![
            polygon(&[(0.0, 0.0), (w, 0.0), (w, d), (0.0, d)]),
            polygon(&[
                (cx - s, cy - s),
                (cx + s, cy - s),
                (cx + s, cy + s),
                (cx - s, cy + s),
            ]),
        ],
        h,
    )
}

// ---------------------------------------------------------------------
// Q1/Q2/Q4 — the one-square-holed slab, opened at its top.
// ---------------------------------------------------------------------

#[test]
fn q2_one_holed_slab_record_covers_and_names() {
    let (w, d, h, t, s) = (1.0, 0.8, 0.3, 0.04, 0.15);
    let tol = Tol::witness();
    let source = holed_slab(w, d, h, s);
    let top = plane_chart_at_z(&source, h);
    assert_eq!(top.len(), 1, "one holed mouth");
    let shelled = topo::shell_open(&source, t, &top, FIT_TOL, tol)
        .unwrap_or_else(|e| panic!("[q2] the one-holed slab is in scope and refused: {e}"));
    let (body, record) = (&shelled.body, &shelled.naming);
    assert_covers("q2 one-holed slab", &source, body, record);

    assert_eq!(record.rims.len(), 1);
    let rim = &record.rims[0];
    assert_eq!(rim.holes.len(), 1, "[q2] one promoted rim");
    let (promoted, named_ring) = rim.holes[0];
    println!(
        "[q2] holes row: promoted={promoted:?} ring={named_ring:?}; \
         resolves in operand: {}; resolves in result: {}",
        source.get_loop(named_ring).is_some(),
        body.get_loop(named_ring).is_some()
    );
    // The type's doc calls this column a SOURCE key.
    assert!(
        source.get_loop(named_ring).is_some(),
        "[q2] the holes row's loop does not resolve in the operand"
    );
    // And the rim itself carries exactly the ring the row names.
    let data = body.get_face(rim.rim).expect("the rim resolves");
    println!("[q2] rim rings: {:?}, row ring {:?}", data.rings, rim.ring);
}

// ---------------------------------------------------------------------
// Q1 — the SAME row on a hole that is BORN in the surgery: the
// revolve's annular cap, whose ring `canonicalize_chart` mints by
// `kemr`. If the doc's "source key" holds, this must resolve in the
// operand too.
// ---------------------------------------------------------------------

fn tube(ri: f64, ro: f64, h: f64) -> Body<f64> {
    use geom_core::Vec2;
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = polygon(&[(ri, 0.0), (ro, 0.0), (ro, h), (ri, h)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the annular meridian is valid");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("it revolves")
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

#[test]
fn q1_holes_second_column_key_space() {
    let tol = Tol::witness();
    let (ri, ro, h, t) = (0.30, 0.50, 0.40, 0.05);
    let source = tube(ri, ro, h);
    let chart = plane_chart_at_y(&source, h);
    let shelled = topo::shell_open(&source, t, &chart, FIT_TOL, tol).expect("the tube opens");
    let (body, record) = (&shelled.body, &shelled.naming);
    let rim = &record.rims[0];
    let (promoted, named_ring) = rim.holes[0];
    let in_operand = source.get_loop(named_ring).is_some();
    let in_result = body.get_loop(named_ring).is_some();
    println!(
        "[q1] annular cap holes row: promoted={promoted:?} ring={named_ring:?}; \
         in operand: {in_operand}; in result: {in_result}"
    );
    // The operand's own loop keys, for contrast.
    let operand_loops: Vec<LoopKey> = source.faces().map(|(_, f)| f.outer).collect();
    println!("[q1] a sample of operand loop keys: {:?}", &operand_loops[..2.min(operand_loops.len())]);
    assert!(
        in_operand,
        "[q1] `RimNaming::holes`' second column is documented as a SOURCE key \
         but {named_ring:?} does not resolve in the operand — it was minted by \
         `kemr` during the chart reduction, so this column's key space depends \
         on the fixture"
    );
}

// ---------------------------------------------------------------------
// Q3 — TWO designated charts, both carrying holes: a holed slab opened
// at BOTH caps. Neither shipped row designates two holed charts.
// ---------------------------------------------------------------------

#[test]
fn q3_two_holed_charts_at_once() {
    let (w, d, h, t, s) = (1.0, 0.8, 0.4, 0.04, 0.15);
    let tol = Tol::witness();
    let source = holed_slab(w, d, h, s);
    let mut chart = plane_chart_at_z(&source, h);
    chart.extend(plane_chart_at_z(&source, 0.0));
    assert_eq!(chart.len(), 2, "two holed mouths");
    match topo::shell_open(&source, t, &chart, FIT_TOL, tol) {
        Err(e) => println!("[q3] two holed charts refused: {e}"),
        Ok(shelled) => {
            let (body, record) = (&shelled.body, &shelled.naming);
            println!(
                "[q3] BUILT: rims {}, holes {:?}, dead f/e/v {}/{}/{}",
                record.rims.len(),
                record.rims.iter().map(|r| r.holes.len()).collect::<Vec<_>>(),
                record.dead.faces.len(),
                record.dead.edges.len(),
                record.dead.vertices.len()
            );
            assert_covers("q3 two holed charts", &source, body, record);
            assert_eq!(record.rims.len(), 2, "[q3] one row per designated chart");
            for rim in &record.rims {
                for &(_, named_ring) in &rim.holes {
                    assert!(
                        source.get_loop(named_ring).is_some(),
                        "[q3] a holes row's loop is not an operand key"
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// Q5 — a chart worn by several faces, DESIGNATED IN REVERSE ARENA
// ORDER: which face is the rim, and is `sources` the designation order
// or the arena order?
// ---------------------------------------------------------------------

fn vessel(r: f64, h: f64) -> Body<f64> {
    use geom_core::Vec2;
    use sweep::{Revolution, RevolveAxis, revolve};
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![polygon(&[(0.0, 0.0), (r, 0.0), (r, h), (0.0, h)])],
    )
    .validate(Tol::witness())
    .expect("the meridian is valid");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("it revolves")
    .body
}

#[test]
fn q5_designation_order_picks_the_rim() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.4, 0.05);
    let source = vessel(r, h);
    let mut chart = plane_chart_at_y(&source, h);
    assert_eq!(chart.len(), 2);
    let forward = topo::shell_open(&source, t, &chart, FIT_TOL, tol).expect("forward");
    chart.reverse();
    let reversed = topo::shell_open(&source, t, &chart, FIT_TOL, tol).expect("reversed");
    println!(
        "[q5] forward rim {:?} sources {:?}; reversed rim {:?} sources {:?}",
        forward.naming.rims[0].rim,
        forward.naming.rims[0].sources,
        reversed.naming.rims[0].rim,
        reversed.naming.rims[0].sources
    );
    println!(
        "[q5] forward outer {:?}",
        forward.naming.outer.iter().map(|&(a, _)| a).collect::<Vec<_>>()
    );
    println!(
        "[q5] dead faces forward {:?} reversed {:?}",
        forward.naming.dead.faces, reversed.naming.dead.faces
    );
    assert_covers("q5 forward", &source, &forward.body, &forward.naming);
    assert_covers("q5 reversed", &source, &reversed.body, &reversed.naming);
}

// ---------------------------------------------------------------------
// Q6 — determinism on a HOLED fixture (the shipped row builds the cup,
// which has no promoted rim).
// ---------------------------------------------------------------------

#[test]
fn q6_determinism_on_the_holed_slab() {
    let (w, d, h, t, s) = (1.0, 0.8, 0.3, 0.04, 0.15);
    let tol = Tol::witness();
    let build = || {
        let source = holed_slab(w, d, h, s);
        let top = plane_chart_at_z(&source, h);
        topo::shell_open(&source, t, &top, FIT_TOL, tol)
            .expect("the slab opens")
            .naming
    };
    let (a, b) = (build(), build());
    assert_eq!(a.outer, b.outer);
    assert_eq!(a.inner, b.inner);
    assert_eq!(a.inner_edges, b.inner_edges);
    assert_eq!(a.inner_vertices, b.inner_vertices);
    assert_eq!(a.dead.faces, b.dead.faces);
    assert_eq!(a.dead.edges, b.dead.edges);
    assert_eq!(a.dead.vertices, b.dead.vertices);
    for (x, y) in a.rims.iter().zip(&b.rims) {
        assert_eq!(x.sources, y.sources);
        assert_eq!(x.rim, y.rim);
        assert_eq!(x.ring, y.ring);
        assert_eq!(x.ring_edges, y.ring_edges);
        assert_eq!(x.ring_vertices, y.ring_vertices);
        assert_eq!(x.holes, y.holes);
    }
}

// ---------------------------------------------------------------------
// Q7 — is `dead` EXACT and COMPLETE? Every face/edge/vertex key that
// was ever alive in the result is an operand key, an inner twin, or a
// promoted rim face; `dead` must be exactly that set minus the live
// one, with no duplicates.
// ---------------------------------------------------------------------

fn assert_dead_is_exact(what: &str, source: &Body<f64>, body: &Body<f64>, record: &ShellNaming) {
    let mut ever_f: Vec<FaceKey> = source.faces().map(|(k, _)| k).collect();
    ever_f.extend(record.inner.iter().map(|&(t, _)| t));
    for rim in &record.rims {
        ever_f.extend(rim.holes.iter().map(|&(f, _)| f));
    }
    let mut want: Vec<FaceKey> = ever_f
        .iter()
        .copied()
        .filter(|k| body.get_face(*k).is_none())
        .collect();
    want.sort();
    want.dedup();
    let mut got = record.dead.faces.clone();
    let n = got.len();
    got.sort();
    got.dedup();
    assert_eq!(n, got.len(), "{what}: dead.faces has a duplicate");
    assert_eq!(got, want, "{what}: dead.faces is not exactly the retired set");

    let mut ever_e: Vec<EdgeKey> = source.edges().map(|(k, _)| k).collect();
    ever_e.extend(record.inner_edges.iter().map(|&(t, _)| t));
    let mut want: Vec<EdgeKey> = ever_e
        .iter()
        .copied()
        .filter(|k| body.get_edge(*k).is_none())
        .collect();
    want.sort();
    want.dedup();
    let mut got = record.dead.edges.clone();
    let n = got.len();
    got.sort();
    got.dedup();
    assert_eq!(n, got.len(), "{what}: dead.edges has a duplicate");
    assert_eq!(got, want, "{what}: dead.edges is not exactly the retired set");

    let mut ever_v: Vec<VertexKey> = source.vertices().map(|(k, _)| k).collect();
    ever_v.extend(record.inner_vertices.iter().map(|&(t, _)| t));
    let mut want: Vec<VertexKey> = ever_v
        .iter()
        .copied()
        .filter(|k| body.get_vertex(*k).is_none())
        .collect();
    want.sort();
    want.dedup();
    let mut got = record.dead.vertices.clone();
    let n = got.len();
    got.sort();
    got.dedup();
    assert_eq!(n, got.len(), "{what}: dead.vertices has a duplicate");
    assert_eq!(got, want, "{what}: dead.vertices is not exactly the retired set");

    for rim in &record.rims {
        let data = body.get_face(rim.rim).expect("the rim resolves");
        assert!(
            data.rings.contains(&rim.ring),
            "{what}: the row's ring is not a ring of the rim"
        );
    }
}

#[test]
fn q7_dead_is_exact_on_every_arm() {
    let tol = Tol::witness();
    let boxed = extruded(
        vec![polygon(&[(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)])],
        4.0,
    );
    let top = plane_chart_at_z(&boxed, 4.0);
    let bottom = plane_chart_at_z(&boxed, 0.0);
    let both: Vec<FaceKey> = top.iter().chain(&bottom).copied().collect();
    let vessel_body = vessel(0.5, 0.4);
    let vessel_chart = plane_chart_at_y(&vessel_body, 0.4);
    let tube_body = tube(0.30, 0.50, 0.40);
    let tube_chart = plane_chart_at_y(&tube_body, 0.40);
    let slab = holed_slab(1.0, 0.8, 0.3, 0.15);
    let slab_chart = plane_chart_at_z(&slab, 0.3);
    let cases: Vec<(&str, Body<f64>, Vec<FaceKey>, f64)> = vec![
        ("sealed box", boxed.clone(), Vec::new(), 0.25),
        ("box cup", boxed.clone(), top, 0.25),
        ("box tube", boxed, both, 0.25),
        ("revolved cup", vessel_body, vessel_chart, 0.05),
        ("annular cup", tube_body, tube_chart, 0.05),
        ("holed slab", slab, slab_chart, 0.04),
    ];
    for (what, source, chart, t) in cases {
        let shelled = topo::shell_open(&source, t, &chart, FIT_TOL, tol)
            .unwrap_or_else(|e| panic!("{what}: {e}"));
        assert_dead_is_exact(what, &source, &shelled.body, &shelled.naming);
        assert_covers(what, &source, &shelled.body, &shelled.naming);
        println!(
            "[q7] {what}: dead f/e/v {}/{}/{}, rims {}",
            shelled.naming.dead.faces.len(),
            shelled.naming.dead.edges.len(),
            shelled.naming.dead.vertices.len(),
            shelled.naming.rims.len()
        );
    }
}
