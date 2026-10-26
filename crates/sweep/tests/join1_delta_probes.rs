//! JOIN-1 fix-pass delta review: probes of the `AlongEdge` lane, the
//! minted edge-edge record and the restated scaffold. Every building
//! pose is judged by [`judged_with_seams`] against a closed-form
//! volume (its arcs' areas extrapolated, [`chord_area`]), which also
//! reads the seams a restated scaffold could hide: an edge between two
//! faces of ONE planar surface key (a face cut in two), an edge between
//! two faces whose planes coincide (a smooth planar seam), and a
//! zero-length open edge. `#[ignore]`d batteries print one line per
//! pose so two trees can be diffed.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::type_complexity
)]

use geom_core::Point3;
use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::test_support::finished;
use sweep::{Extrusion, extrude};
use topo::AtRestBody;

use crate::common::differential::{Verdict, area, clip_convex, judged_with_seams, outcome, seams};

fn tol() -> Tol {
    Tol::witness()
}

/// A z-prism over a bulge loop `(x, y, bulge)`, shifted by `(dx, dy)`,
/// spanning `z`.
fn zprism(pts: &[(f64, f64, f64)], d: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z.0));
    let lp = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x + d.0, y + d.1), b))
            .collect(),
    );
    let p = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    let prism = extrude(
        &p,
        Extrusion::Distance {
            depth: z.1 - z.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("the prism", prism, tol())
}

/// The loop polygonised, each bulge arc at `n` chords.
fn polygon(pts: &[(f64, f64, f64)], d: (f64, f64), n: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..pts.len() {
        let (x0, y0, b) = pts[i];
        let (x1, y1, _) = pts[(i + 1) % pts.len()];
        out.push((x0 + d.0, y0 + d.1));
        if b == 0.0 {
            continue;
        }
        // Bulge b: included angle θ = 4·atan(b), CCW for b > 0.
        let theta = 4.0 * b.atan();
        let (mx, my) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        let (cx_, cy_) = (x1 - x0, y1 - y0);
        let chord = (cx_ * cx_ + cy_ * cy_).sqrt();
        let r = chord / (2.0 * (theta / 2.0).sin());
        // centre: left of the chord for a CCW arc of < π
        let h = r * (theta / 2.0).cos();
        let (nx, ny) = (-cy_ / chord, cx_ / chord);
        let (ccx, ccy) = (mx + nx * h, my + ny * h);
        let a0 = (y0 - ccy).atan2(x0 - ccx);
        for k in 1..n {
            let t = a0 + theta * (k as f64) / (n as f64);
            out.push((ccx + r.abs() * t.cos() + d.0, ccy + r.abs() * t.sin() + d.1));
        }
    }
    out
}

/// **The area of loop `a` against loop `b` shifted by `d`** (`b`
/// `None`: `a`'s own), Richardson-extrapolated from its chord areas at
/// 1024 and 2048 chords per arc. A chord polygon's area is short of the
/// arcs' by `c/n² + O(n⁻³)`, so `(4·A(2n) − A(n))/3` is off by
/// `O(n⁻³)`: under 1e-10 here, against the segments and lenses' closed
/// forms and against the same extrapolation at 4096 chords.
fn chord_area(a: &[(f64, f64, f64)], b: Option<(&[(f64, f64, f64)], (f64, f64))>) -> f64 {
    let at = |n| {
        let pa = polygon(a, (0.0, 0.0), n);
        match b {
            None => area(&pa),
            Some((b, d)) => area(&clip_convex(&pa, &polygon(b, d, n))).max(0.0),
        }
    };
    (4.0 * at(2048) - at(1024)) / 3.0
}

/// Bulge of a CCW arc of included angle `deg`.
fn bulge(deg: f64) -> f64 {
    (deg.to_radians() / 4.0).tan()
}

/// Convex arc-bounded profiles on and around the circle of radius 0.5
/// at the origin: shared arcs, arcs of other radii through the same
/// two points, an arc against the straight chord through its ends.
fn arc_shapes() -> Vec<(&'static str, Vec<(f64, f64, f64)>)> {
    vec![
        // upper half disc: arc (0.5,0) → (−0.5,0) CCW, chord back.
        ("half", vec![(0.5, 0.0, bulge(180.0)), (-0.5, 0.0, 0.0)]),
        // lower half disc.
        ("lower", vec![(-0.5, 0.0, bulge(180.0)), (0.5, 0.0, 0.0)]),
        // a shallower arc (radius 0.707) through the same two points.
        ("shallow", vec![(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, 0.0)]),
        // a quarter sector of the same circle.
        (
            "quarter",
            vec![(0.0, 0.0, 0.0), (0.5, 0.0, bulge(90.0)), (0.0, 0.5, 0.0)],
        ),
        // a square holding the half disc's chord.
        (
            "square",
            vec![
                (-0.5, -1.0, 0.0),
                (0.5, -1.0, 0.0),
                (0.5, 0.0, 0.0),
                (-0.5, 0.0, 0.0),
            ],
        ),
        // the triangle over the chord.
        (
            "tri",
            vec![(0.5, 0.0, 0.0), (0.0, 0.5, 0.0), (-0.5, 0.0, 0.0)],
        ),
        // a lens: two arcs of the radius-0.707 circle family.
        (
            "lens",
            vec![(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, bulge(90.0))],
        ),
    ]
}

#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_delta_arc_battery() {
    let shapes = arc_shapes();
    let offsets = [(0.0, 0.0), (0.5, 0.0), (-0.5, 0.0), (0.0, 0.5), (1.0, 0.0)];
    let za = (0.0, 2.0);
    let zbs = [(0.0, 2.0), (1.0, 3.0), (2.0, 4.0), (-1.0, 1.0), (0.5, 1.5)];
    let only = std::env::var("J1D_SHAPE").ok();
    for (na, pa) in &shapes {
        if only.as_deref().is_some_and(|o| o != *na) {
            continue;
        }
        let a = zprism(pa, (0.0, 0.0), za);
        let va = chord_area(pa, None) * (za.1 - za.0);
        for (nb, pb) in &shapes {
            for &d in &offsets {
                let ab = chord_area(pa, Some((pb, d)));
                for &zb in &zbs {
                    let b = zprism(pb, d, zb);
                    let vb = chord_area(pb, None) * (zb.1 - zb.0);
                    let vi = ab * (za.1.min(zb.1) - za.0.max(zb.0)).max(0.0);
                    for (op, want) in [("U", va + vb - vi), ("S", va - vi), ("I", vi)] {
                        let res = match op {
                            "U" => topo::union(&a, &b, tol()),
                            "S" => topo::subtract(&a, &b, tol()),
                            _ => topo::intersect(&a, &b, tol()),
                        };
                        println!(
                            "ARC {na} {nb} d={d:?} zb={zb:?} {op} => {}",
                            judged_with_seams(res, Some(want), tol()).line
                        );
                    }
                }
            }
        }
    }
}

/// Brick against brick: corners, edges and faces shared, caps resting
/// and overlapping, every op in both orders.
#[test]
#[ignore = "differential battery; run with --ignored"]
fn join1_delta_brick_battery() {
    use sweep::test_support::brick;
    let coords = [-0.5, 0.0, 0.5, 1.0, 1.5];
    let mut ranges = Vec::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            ranges.push((coords[i], coords[j]));
        }
    }
    let a = finished(
        "the unit brick",
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let ov = |p: (f64, f64), q: (f64, f64)| (p.1.min(q.1) - p.0.max(q.0)).max(0.0);
    let zs = [(1.0, 2.0), (0.0, 1.0), (0.5, 1.5), (-1.0, 0.0), (0.0, 0.5)];
    for &x in &ranges {
        for &y in &ranges {
            for &z in &zs {
                let b = finished("the brick", brick(x, y, z, tol()), tol());
                let vb = (x.1 - x.0) * (y.1 - y.0) * (z.1 - z.0);
                let vi = ov((0.0, 1.0), x) * ov((0.0, 1.0), y) * ov((0.0, 1.0), z);
                for (order, l, r) in [("AB", &a, &b), ("BA", &b, &a)] {
                    let (vl, vr) = if order == "AB" { (1.0, vb) } else { (vb, 1.0) };
                    for (op, want) in [("U", vl + vr - vi), ("S", vl - vi), ("I", vi)] {
                        let res = match op {
                            "U" => topo::union(l, r, tol()),
                            "S" => topo::subtract(l, r, tol()),
                            _ => topo::intersect(l, r, tol()),
                        };
                        println!(
                            "BRICK x={x:?} y={y:?} z={z:?} {order} {op} => {}",
                            judged_with_seams(res, Some(want), tol()).line
                        );
                    }
                }
            }
        }
    }
}

/// Two lens prisms on one profile (two 90° arcs of radius 0.707 through
/// (±0.5, 0)), overlapping in `z ∈ [0, 1]`: the union is the lens prism over `z ∈ [−1, 2]`. **RED on 21b7f289**:
/// the merge parent of JOIN-1's fix-pass head (`da396111f`, main)
/// builds it sound; the head refuses `Join(SectionLoopMixed)`, the
/// guard `boolean::join::loop_roles` documents as a kernel defect that
/// "no row reaches". The head with the fix-pass delta reverted refuses
/// the same way: the move predates the fix pass.
#[test]
fn overlapping_lens_prisms_union_builds() {
    let lens = [(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, bulge(90.0))];
    let a = zprism(&lens, (0.0, 0.0), (0.0, 2.0));
    let b = zprism(&lens, (0.0, 0.0), (-1.0, 1.0));
    let want = chord_area(&lens, None) * 3.0;
    let r = topo::union(&a, &b, tol());
    if let Err(e) = &r {
        println!("[lens] {e:?}");
    }
    let j = judged_with_seams(r, Some(want), tol());
    println!("[lens] {}", j.line);
    assert_eq!(j.verdict, Verdict::Sound, "{}", j.line);
    let r = topo::union(&a, &b, tol()).unwrap();
    sweep::test_support::assert_legal_operand("lens ∪", &r.body().unwrap().body, tol());
}

/// **An edge lying in a face or along an edge is recorded** (D1): the
/// triangle prism's apex edge `x = 0, y = 0.5` against a partner
/// standing on `y = 0.5` over `z ∈ [0.5, 1.5]`. The partner's cap edges
/// bound the apex edge's span on it, and the union records the span
/// against what holds it: the partner's face `y = 0.5` (`tri`, `half`),
/// or its corner edge along the apex (`quarter`). Tier 3′ confirms the
/// record in each: all planar, `tri tri` passes, and a curved partner
/// refuses only on the cross-solid backstop
/// (`census-cross-solid-curved-pairs-undecidable-on-shell-results`).
#[test]
fn an_apex_edge_in_the_partners_face_is_recorded() {
    let tri = [(0.5, 0.0, 0.0), (0.0, 0.5, 0.0), (-0.5, 0.0, 0.0)];
    let half = [(0.5, 0.0, bulge(180.0)), (-0.5, 0.0, 0.0)];
    let quarter = [(0.0, 0.0, 0.0), (0.5, 0.0, bulge(90.0)), (0.0, 0.5, 0.0)];
    let a = zprism(&tri, (0.0, 0.0), (0.0, 2.0));
    let on_apex = |p: geom_core::Point3<f64>| p.x.abs() < 1e-12 && (p.y - 0.5).abs() < 1e-12;
    for (name, pb, along) in [
        ("tri", &tri[..], false),
        ("half", &half[..], false),
        ("quarter", &quarter[..], true),
    ] {
        let b = zprism(pb, (0.0, 0.5), (0.5, 1.5));
        let r = topo::union(&a, &b, tol()).unwrap();
        let bb = r.body().unwrap();
        let body = &bb.body;
        let ends = |e| {
            let d = body.get_edge(e).unwrap();
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            (
                p(body.get_half_edge(d.he_plus).unwrap().start),
                p(body.half_edge_end(d.he_plus).unwrap()),
            )
        };
        let apex = |e| {
            let (p, q) = ends(e);
            on_apex(p) && on_apex(q) && (p.z - q.z).abs() > 2.0 - 1e-12
        };
        let holder = |c| match c {
            topo::Cell::Face(f) => match body.get_surface(body.get_face(f).unwrap().surface) {
                Some(geom::Surface::Plane { origin, normal, .. }) => {
                    !along && normal.x.abs() < 1e-12 && (origin.y - 0.5).abs() < 1e-12
                }
                _ => false,
            },
            topo::Cell::Edge(g) => {
                let (p, q) = ends(g);
                along && on_apex(p) && on_apex(q) && (p.z - q.z).abs() > 1.0 - 1e-12
            }
            topo::Cell::Vertex(_) => false,
        };
        let spans = bb
            .contacts
            .cell_pairs()
            .filter(|&(x, y)| match (x, y) {
                (topo::Cell::Edge(e), c) | (c, topo::Cell::Edge(e)) => apex(e) && holder(c),
                _ => false,
            })
            .count();
        assert_eq!(
            spans, 1,
            "tri ∪ {name}: one record of the apex edge's span against what holds it, in {:?}",
            bb.contacts
        );
        // Tier 3′ confirms the record: no finding is a stale record or
        // an unrecorded touch. A curved partner's only findings are the
        // cross-solid backstop's.
        match topo::validate_pseudomanifold(body, &bb.contacts, tol()) {
            Ok(()) => assert_eq!(name, "tri", "tri ∪ {name}: the backstop refuses"),
            Err(es) => assert!(
                name != "tri"
                    && es
                        .iter()
                        .all(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })),
                "tri ∪ {name}: tier 3′: {es:?}"
            ),
        }
    }
}

/// The hexagon ∪ box corner-edge pose (`join1_r1_rows`) builds on the
/// head with the minted chord restated on a smooth seam between two
/// coplanar faces of different keys (`y = −0.5`). The row reads that
/// seam and then uses the union as an OPERAND, with a brick across the
/// seam: what a caller does next with the body. **RED on 21b7f289**:
/// every op refuses `CoplanarNeighbours` (the maximal-faces gate),
/// against DESIGN.md's "every boolean output is a legal boolean
/// operand" and "a same-sense cosurface adjacency it has no licence for
/// refuses at the op that would create it". Main refuses the union.
#[test]
fn the_seam_body_is_an_operand() {
    use sweep::test_support::brick;
    let hex: [(f64, f64, f64); 6] = [
        (0.5, 0.0, 0.0),
        (0.25, 0.5, 0.0),
        (-0.25, 0.5, 0.0),
        (-0.5, 0.0, 0.0),
        (-0.25, -0.5, 0.0),
        (0.25, -0.5, 0.0),
    ];
    let a = zprism(&hex, (0.0, 0.0), (0.0, 2.0));
    let b = finished(
        "the box",
        brick((-0.5, -0.25), (-0.5, -0.25), (-1.0, 3.0), tol()),
        tol(),
    );
    // The continuation is one carrier by margin: the union glues it,
    // and the merge stage glues the two faces.
    let r = match topo::union(&a, &b, tol()).unwrap() {
        topo::BooleanResult::Body(bb) => bb,
        topo::BooleanResult::Empty => panic!("empty"),
    };
    let (sk, cp, z) = seams(&r.body);
    println!("[seam] hex ∪ box: samekey={sk} coplanar={cp} zero={z}");
    let vr = topo::test_support::pinned_volume("the measured body", &r.body, tol()).volume;
    assert!((vr - 1.71875).abs() < 1e-9, "{vr}");
    let c = finished(
        "the brick",
        brick((-0.4, -0.1), (-0.7, -0.3), (0.5, 1.5), tol()),
        tol(),
    );
    let lines = [
        (
            "∪",
            outcome(topo::union(&r.body, &c, tol()), vr + 0.12 - 0.06, tol()),
        ),
        (
            "∖",
            outcome(topo::subtract(&r.body, &c, tol()), vr - 0.06, tol()),
        ),
        (
            "∩",
            outcome(topo::intersect(&r.body, &c, tol()), 0.06, tol()),
        ),
    ];
    for (op, l) in &lines {
        println!("[seam] (hex ∪ box) {op} brick: {l}");
    }
    assert!(
        lines.iter().all(|(_, l)| l.starts_with("OK SOUND")),
        "{lines:#?}"
    );
}

/// The mechanism rows' peg ∪ collar bodies (`join1_mechanisms`), the
/// walls a `Rest` and the flush caps (peg disc, collar annulus)
/// continuations, which the merge stage glues. Read the shipped body's seams, and use it as an operand
/// against a disjoint brick (no contact at all: only the operand gates
/// run on it): both build, and both are operands.
#[test]
fn the_peg_collar_unions_are_operands() {
    use crate::mate2_common::{collar_at, peg_at};
    use sweep::test_support::brick;
    let far = finished(
        "the far brick",
        brick((5.0, 6.0), (5.0, 6.0), (0.0, 1.0), tol()),
        tol(),
    );
    let mut lines = Vec::new();
    for (what, h) in [("proud", 1.5), ("flush", 1.0)] {
        let c = finished("the collar", collar_at(0.0), tol());
        let p = finished("the peg", peg_at(0.0, 1.0, h), tol());
        let r = match topo::union(&c, &p, tol()).unwrap() {
            topo::BooleanResult::Body(bb) => bb,
            topo::BooleanResult::Empty => panic!("empty"),
        };
        let (sk, cp, z) = seams(&r.body);
        let vr = topo::test_support::pinned_volume("the measured body", &r.body, tol()).volume;
        let next = outcome(topo::union(&r.body, &far, tol()), vr + 1.0, tol());
        let l = format!("{what}: samekey={sk} coplanar={cp} zero={z}; with a far brick: {next}");
        println!("[peg] {l}");
        lines.push(l);
    }
    assert!(lines.iter().all(|l| l.contains("OK SOUND")), "{lines:#?}");
}
