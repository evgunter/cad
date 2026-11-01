//! **The cell door on REAL BODIES** — `topo::cell_region` on faces
//! built through the public constructors (`sweep::extrude`,
//! `sweep::revolve`), every decided cell checked against an INDEPENDENT
//! model-space oracle: profile segment loci and an even-odd region walk
//! over bulge chains, read off each lattice point's own 3-D position,
//! which shares no code with the door under test.
//!
//! For every face a grid of cells covers its carrier window in its
//! stored chart. A cell the door calls `Outside` must hold no lattice
//! point the oracle puts in the face; one it calls `Inside` must hold
//! none the oracle puts out of it. Every fixture also states which
//! verdicts it must reach, so a door that answered `Undecided`
//! everywhere, or skipped its witness, goes red.
//!
//! It lives in `sweep` and not beside the `topo` rows because the
//! constructors do: `topo` cannot depend on `sweep`.
//!
//! ε posture: no ε literal. The band is the run's own; the oracle's
//! `DELTA` is a GEOMETRIC skin around the loci, not a tolerance, and
//! it only ever moves a lattice point from `In`/`Out` into `Near`,
//! which the comparison then declines to judge.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::float_cmp
)]

use std::f64::consts::{PI, TAU};
use std::time::Instant;
use sweep::ExtrudeSide;

use crate::common::approx::band;
use crate::common::interval::{iv, p2, v2};
use geom::Surface;
use geom_core::{Bounds, Interval, Point2, Point3, Tol};
use profile::{Profile, ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::cell_region::{CellRegion, cell_region};
use topo::{Body, FaceKey, LoopBoundary};

fn mid(x: Interval) -> f64 {
    0.5 * (x.lo() + x.hi())
}

// ------------------------------------------------------------------ //
// The oracle: profile-segment loci and regions, in 2-D
// ------------------------------------------------------------------ //

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cls {
    In,
    Out,
    Near,
}

const DELTA: f64 = 1e-7;

#[derive(Clone, Copy, Debug)]
struct Seg {
    a: (f64, f64),
    b: (f64, f64),
    bulge: f64,
}

fn segs_of(verts: &[((f64, f64), f64)]) -> Vec<Seg> {
    let n = verts.len();
    (0..n)
        .map(|i| Seg {
            a: verts[i].0,
            b: verts[(i + 1) % n].0,
            bulge: verts[i].1,
        })
        .collect()
}

/// (center, radius, start angle, signed sweep) of a bulged segment.
fn arc_of(s: &Seg) -> ((f64, f64), f64, f64, f64) {
    let (ax, ay) = s.a;
    let arc = geom_core::Arc2::from_chord(Point2::new(ax, ay), Point2::new(s.b.0, s.b.1), s.bulge);
    let c = (arc.centre.x, arc.centre.y);
    let start = (ay - c.1).atan2(ax - c.0);
    (c, arc.radius, start, arc.sweep)
}

/// Is the 2-D point on this segment's locus (line or arc), strictly
/// inside its parameter range?
fn on_locus(s: &Seg, p: (f64, f64)) -> Cls {
    if s.bulge == 0.0 {
        let (ax, ay) = s.a;
        let (bx, by) = s.b;
        let ex = bx - ax;
        let ey = by - ay;
        let l2 = ex * ex + ey * ey;
        let t = ((p.0 - ax) * ex + (p.1 - ay) * ey) / l2;
        let px = ax + t * ex;
        let py = ay + t * ey;
        let d = ((p.0 - px).powi(2) + (p.1 - py).powi(2)).sqrt();
        let l = l2.sqrt();
        if d > DELTA {
            return Cls::Out;
        }
        let ends = DELTA / l;
        if t < -ends || t > 1.0 + ends {
            Cls::Out
        } else if t < ends || t > 1.0 - ends {
            Cls::Near
        } else {
            Cls::In
        }
    } else {
        let (c, r, start, sweep) = arc_of(s);
        let d = ((p.0 - c.0).powi(2) + (p.1 - c.1).powi(2)).sqrt();
        if (d - r).abs() > DELTA {
            return Cls::Out;
        }
        let ang = (p.1 - c.1).atan2(p.0 - c.0);
        let mut rel = ang - start;
        // Normalize into the sweep's direction.
        if sweep > 0.0 {
            rel = rel.rem_euclid(TAU);
        } else {
            rel = -((-rel).rem_euclid(TAU));
        }
        let ends = DELTA / r;
        let (lo, hi) = if sweep > 0.0 {
            (0.0, sweep)
        } else {
            (sweep, 0.0)
        };
        // rel ∈ [0, τ) or (−τ, 0]; the arc covers [lo, hi].
        let inside = rel > lo + ends && rel < hi - ends;
        let near = (rel - lo).abs() <= ends
            || (rel - hi).abs() <= ends
            || (rel - lo - TAU.copysign(sweep)).abs() <= ends;
        if inside {
            Cls::In
        } else if near {
            Cls::Near
        } else {
            Cls::Out
        }
    }
}

/// Even-odd point-in-region for a bulge chain (|bulge| ≤ 1 only).
fn in_region(segs: &[Seg], p: (f64, f64)) -> Cls {
    // Near any locus ⇒ Near.
    for s in segs {
        if on_locus(s, p) != Cls::Out {
            return Cls::Near;
        }
        // Near an endpoint?
        for q in [s.a, s.b] {
            if ((p.0 - q.0).powi(2) + (p.1 - q.1).powi(2)).sqrt() <= DELTA {
                return Cls::Near;
            }
        }
    }
    // A bulged segment's chord is no locus, but the parity below cannot
    // place a point on it: within the skin of a chord, Near.
    for s in segs {
        if s.bulge == 0.0 {
            continue;
        }
        let chord = Seg { bulge: 0.0, ..*s };
        if on_locus(&chord, p) != Cls::Out {
            return Cls::Near;
        }
    }
    // Chord polygon parity.
    let mut inside = false;
    for s in segs {
        let (ax, ay) = s.a;
        let (bx, by) = s.b;
        if (ay > p.1) != (by > p.1) {
            let x = ax + (p.1 - ay) * (bx - ax) / (by - ay);
            if p.0 < x {
                inside = !inside;
            }
        }
    }
    // Lunes.
    for s in segs {
        if s.bulge == 0.0 {
            continue;
        }
        assert!(s.bulge.abs() <= 1.0, "oracle handles minor arcs only");
        let (c, r, _, _) = arc_of(s);
        let d = ((p.0 - c.0).powi(2) + (p.1 - c.1).powi(2)).sqrt();
        if d < r {
            // Apex side of the chord: the RIGHT of A→B for positive bulge.
            let (ax, ay) = s.a;
            let (bx, by) = s.b;
            let cross = (bx - ax) * (p.1 - ay) - (by - ay) * (p.0 - ax);
            let apex_side = if s.bulge > 0.0 {
                cross < 0.0
            } else {
                cross > 0.0
            };
            if apex_side {
                inside = !inside;
            }
        }
    }
    if inside { Cls::In } else { Cls::Out }
}

/// A profile with holes: in the outer, not in any hole.
fn in_profile(outer: &[Seg], holes: &[Vec<Seg>], p: (f64, f64)) -> Cls {
    match in_region(outer, p) {
        Cls::Out => return Cls::Out,
        Cls::Near => return Cls::Near,
        Cls::In => {}
    }
    for h in holes {
        match in_region(h, p) {
            Cls::In => return Cls::Out,
            Cls::Near => return Cls::Near,
            Cls::Out => {}
        }
    }
    Cls::In
}

fn range_cls(x: f64, lo: f64, hi: f64) -> Cls {
    if x < lo - DELTA || x > hi + DELTA {
        Cls::Out
    } else if x < lo + DELTA || x > hi - DELTA {
        Cls::Near
    } else {
        Cls::In
    }
}

fn both(a: Cls, b: Cls) -> Cls {
    match (a, b) {
        (Cls::Out, _) | (_, Cls::Out) => Cls::Out,
        (Cls::Near, _) | (_, Cls::Near) => Cls::Near,
        _ => Cls::In,
    }
}

type Oracle = Box<dyn Fn(Point3<f64>) -> Cls>;

// ------------------------------------------------------------------ //
// Body helpers
// ------------------------------------------------------------------ //

fn surface_f64(s: &Surface<Interval>) -> Surface<f64> {
    s.map_scalar(mid)
}

fn face_surface(body: &Body<Interval>, face: FaceKey) -> Surface<Interval> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

/// Midpoints of every edge of the face's outer loop, in 3-D.
fn edge_mids(body: &Body<Interval>, face: FaceKey) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    let f = body.get_face(face).unwrap();
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let lp = body.get_loop(lk).unwrap();
        let LoopBoundary::Cycle { first } = lp.boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let e = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
            let topo::CurveGeom::Certified(c) = body.get_curve_geom(e.curve).unwrap() else {
                panic!("uncertified curve")
            };
            let (t0, t1) = c.params();
            let t = (t0 + t1) * iv(0.5);
            let p = c.carrier().eval(t);
            out.push(Point3::new(mid(p.x), mid(p.y), mid(p.z)));
        }
    }
    out
}

// ------------------------------------------------------------------ //
// The grid comparison
// ------------------------------------------------------------------ //

#[derive(Default)]
struct GridStats {
    cells: usize,
    inside: usize,
    outside: usize,
    violations: usize,
}

/// Points of the face's boundary, in 3-D: every vertex and nine
/// stations along every edge.
fn boundary_points(body: &Body<Interval>, face: FaceKey) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    let f = body.get_face(face).unwrap();
    let flat = |p: Point3<Interval>| Point3::new(mid(p.x), mid(p.y), mid(p.z));
    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let e = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
            let topo::CurveGeom::Certified(c) = body.get_curve_geom(e.curve).unwrap() else {
                panic!("uncertified curve")
            };
            let (t0, t1) = (mid(c.params().0), mid(c.params().1));
            for k in 0..=8 {
                let t = t0 + (t1 - t0) * f64::from(k) / 8.0;
                out.push(flat(c.carrier().eval(iv(t))));
            }
        }
    }
    out
}

/// The face's carrier window in its stored chart: the boundary's own
/// extent, padded, on an affine coordinate, and the coordinate's whole
/// canonical range on an angular one (the cone's slant both ways round
/// the apex, as `clearance` takes it).
fn window(body: &Body<Interval>, face: FaceKey, s: &Surface<f64>) -> ((f64, f64), (f64, f64)) {
    let pts = boundary_points(body, face);
    let span = |xs: Vec<f64>| {
        let (lo, hi) = xs
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), &x| {
                (l.min(x), h.max(x))
            });
        let pad = (0.25 * (hi - lo)).max(0.3);
        (lo - pad, hi + pad)
    };
    match *s {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let w = normal.cross(u_ref);
            (
                span(pts.iter().map(|p| (*p - origin).dot(u_ref)).collect()),
                span(pts.iter().map(|p| (*p - origin).dot(w)).collect()),
            )
        }
        Surface::Cylinder { origin, axis, .. } => (
            (0.0, TAU),
            span(pts.iter().map(|p| (*p - origin).dot(axis)).collect()),
        ),
        Surface::Cone { apex, .. } => {
            let reach = pts.iter().map(|p| (*p - apex).norm()).fold(0.0, f64::max) + 0.3;
            ((0.0, TAU), (-reach, reach))
        }
        Surface::Sphere { .. } => ((0.0, TAU), (-PI / 2.0, PI / 2.0)),
        Surface::Torus { .. } => ((0.0, TAU), (0.0, TAU)),
        _ => panic!("no window for a free-form carrier"),
    }
}

/// Asks the door about an `n×n` grid of cells covering the face's
/// window, and checks every decided cell against the oracle on a 5×5
/// interior lattice.
fn grid(body: &Body<Interval>, face: FaceKey, n: usize, oracle: &Oracle, label: &str) -> GridStats {
    let s = surface_f64(&face_surface(body, face));
    let ((u0, u1), (v0, v1)) = window(body, face, &s);
    let b = band();
    let mut st = GridStats::default();
    let du = (u1 - u0) / n as f64;
    let dv = (v1 - v0) / n as f64;
    let started = Instant::now();
    for i in 0..n {
        for j in 0..n {
            let cu = (u0 + i as f64 * du, u0 + (i + 1) as f64 * du);
            let cv = (v0 + j as f64 * dv, v0 + (j + 1) as f64 * dv);
            st.cells += 1;
            let verdict = cell_region(body, face, cu, cv, b);
            let (mut any_in, mut any_out) = (false, false);
            for p in 0..5 {
                for q in 0..5 {
                    let u = cu.0 + (0.1 + 0.2 * p as f64) * (cu.1 - cu.0);
                    let v = cv.0 + (0.1 + 0.2 * q as f64) * (cv.1 - cv.0);
                    match oracle(s.eval(u, v)) {
                        Cls::In => any_in = true,
                        Cls::Out => any_out = true,
                        Cls::Near => {}
                    }
                }
            }
            let wrong = match verdict {
                CellRegion::Outside => {
                    st.outside += 1;
                    any_in
                }
                CellRegion::Inside => {
                    st.inside += 1;
                    any_out
                }
                CellRegion::Undecided => false,
            };
            if wrong {
                st.violations += 1;
                eprintln!(
                    "  !!! VIOLATION {label}: cell u[{:.5},{:.5}] v[{:.5},{:.5}] decided \
                     {verdict:?} against the oracle",
                    cu.0, cu.1, cv.0, cv.1
                );
            }
        }
    }
    let per_cell = started.elapsed().as_secs_f64() / st.cells as f64;
    eprintln!(
        "  {label} [{}]: cells={} inside={} outside={} VIOLATIONS={} ({:.1} µs/cell)",
        s.kind().name(),
        st.cells,
        st.inside,
        st.outside,
        st.violations,
        per_cell * 1e6
    );
    st
}

/// One face's grid, by kind.
struct Graded {
    kind: &'static str,
    stats: GridStats,
}

fn run_extrude(name: &str, loops: &[Vec<((f64, f64), f64)>], h: f64) -> Vec<Graded> {
    eprintln!("=== extrude fixture: {name} (h = {h}) ===");
    let vp = profile_of(loops);
    let t = extrude(
        &vp,
        Extrusion::Distance {
            depth: iv(h),
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap();
    extrude_oracles(&t.body, &t, loops, h)
        .into_iter()
        .map(|(face, oracle, fname)| Graded {
            kind: face_surface(&t.body, face).kind().name(),
            stats: grid(
                &t.body,
                face,
                24,
                &oracle,
                &format!("{name}/{fname}/{face:?}"),
            ),
        })
        .collect()
}

fn run_revolve(
    name: &str,
    loops: &[Vec<((f64, f64), f64)>],
    rev: Revolution<Interval>,
    theta: Option<f64>,
) -> Vec<Graded> {
    run_revolve_at(name, loops, rev, theta, 24)
}

fn run_revolve_at(
    name: &str,
    loops: &[Vec<((f64, f64), f64)>],
    rev: Revolution<Interval>,
    theta: Option<f64>,
    n: usize,
) -> Vec<Graded> {
    eprintln!("=== revolve fixture: {name} (theta = {theta:?}) ===");
    let vp = profile_of(loops);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: v2(0.0, 1.0),
    };
    let t = revolve(&vp, axis, rev, Tol::witness()).unwrap();
    revolve_oracles(&t.body, &t, loops, theta)
        .into_iter()
        .map(|(face, oracle, fname)| Graded {
            kind: face_surface(&t.body, face).kind().name(),
            stats: grid(
                &t.body,
                face,
                n,
                &oracle,
                &format!("{name}/{fname}/{face:?}"),
            ),
        })
        .collect()
}

/// No decided cell contradicts the oracle, on any face.
fn assert_sound(graded: &[Graded]) {
    let v: usize = graded.iter().map(|g| g.stats.violations).sum();
    assert_eq!(v, 0, "a decided cell contradicted the oracle");
}

/// Every face of `kind` decides cells both ways: some `Inside`, some
/// `Outside`.
fn assert_decides(graded: &[Graded], kind: &str) {
    let of_kind: Vec<&Graded> = graded.iter().filter(|g| g.kind == kind).collect();
    assert!(!of_kind.is_empty(), "the fixture has a {kind} face");
    for g in of_kind {
        assert!(
            g.stats.inside > 0 && g.stats.outside > 0,
            "a {kind} face decides cells both ways: inside={} outside={}",
            g.stats.inside,
            g.stats.outside
        );
    }
}

// ------------------------------------------------------------------ //
// Fixtures
// ------------------------------------------------------------------ //

fn profile_of(loops: &[Vec<((f64, f64), f64)>]) -> profile::ValidatedProfile<Interval> {
    let lps: Vec<ProfileLoop<Interval>> = loops
        .iter()
        .map(|l| bulge_loop(l.iter().map(|(p, b)| (p2(p.0, p.1), iv(*b))).collect()))
        .collect();
    Profile::new(SketchPlane::<Interval>::xy(), lps)
        .validate(Tol::witness())
        .unwrap()
}

/// Extrude oracle: caps at z=0/h use the profile region; walls use the
/// segment locus found by the face's edge midpoints.
fn extrude_oracles(
    body: &Body<Interval>,
    t: &sweep::Extruded<Interval>,
    loops: &[Vec<((f64, f64), f64)>],
    h: f64,
) -> Vec<(FaceKey, Oracle, &'static str)> {
    let outer = segs_of(&loops[0]);
    let holes: Vec<Vec<Seg>> = loops[1..].iter().map(|l| segs_of(l)).collect();
    let mut out: Vec<(FaceKey, Oracle, &'static str)> = Vec::new();
    for (face, z, name) in [(t.bottom, 0.0, "bottom"), (t.top, h, "top")] {
        let outer = outer.clone();
        let holes = holes.clone();
        out.push((
            face,
            Box::new(move |p: Point3<f64>| {
                both(range_cls(p.z, z, z), in_profile(&outer, &holes, (p.x, p.y)))
            }),
            name,
        ));
    }
    let all_segs: Vec<Seg> = loops.iter().flat_map(|l| segs_of(l)).collect();
    for faces in &t.side_faces() {
        for &face in faces {
            let mids = edge_mids(body, face);
            let seg = all_segs
                .iter()
                .copied()
                .find(|s| mids.iter().all(|m| on_locus(s, (m.x, m.y)) != Cls::Out))
                .expect("every wall lies on one profile segment");
            out.push((
                face,
                Box::new(move |p: Point3<f64>| {
                    both(range_cls(p.z, 0.0, h), on_locus(&seg, (p.x, p.y)))
                }),
                "wall",
            ));
        }
    }
    out
}

/// Revolve about the sketch y-axis: (r, y, φ) with φ = atan2(−z, x).
fn cyl_coords(p: Point3<f64>) -> (f64, f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y, (-p.z).atan2(p.x))
}

fn phi_cls(phi: f64, theta: Option<f64>, r: f64) -> Cls {
    let Some(theta) = theta else {
        return Cls::In;
    };
    let (lo, hi) = if theta > 0.0 {
        (0.0, theta)
    } else {
        (theta, 0.0)
    };
    // Bring phi to the nearest representative of the band.
    let mut best = Cls::Out;
    for k in [-1.0, 0.0, 1.0] {
        let c = range_cls(phi + k * TAU, lo, hi);
        if r < 1e-9 {
            return Cls::Near;
        }
        best = match (best, c) {
            (Cls::In, _) | (_, Cls::In) => Cls::In,
            (Cls::Near, _) | (_, Cls::Near) => Cls::Near,
            _ => Cls::Out,
        };
    }
    best
}

fn revolve_oracles(
    body: &Body<Interval>,
    t: &sweep::Revolved<Interval>,
    loops: &[Vec<((f64, f64), f64)>],
    theta: Option<f64>,
) -> Vec<(FaceKey, Oracle, &'static str)> {
    let outer = segs_of(&loops[0]);
    let holes: Vec<Vec<Seg>> = loops[1..].iter().map(|l| segs_of(l)).collect();
    let all_segs: Vec<Seg> = loops.iter().flat_map(|l| segs_of(l)).collect();
    let mut out: Vec<(FaceKey, Oracle, &'static str)> = Vec::new();
    let mut walls = std::collections::HashSet::new();
    let _ = &t.walls();
    for (face, _) in body.faces() {
        let mids = edge_mids(body, face);
        let seg = all_segs.iter().copied().find(|s| {
            mids.iter().all(|m| {
                let (r, y, _) = cyl_coords(*m);
                on_locus(s, (r, y)) != Cls::Out
            })
        });
        if let Some(seg) = seg {
            walls.insert(face);
            out.push((
                face,
                Box::new(move |p: Point3<f64>| {
                    let (r, y, phi) = cyl_coords(p);
                    both(phi_cls(phi, theta, r), on_locus(&seg, (r, y)))
                }),
                "wall",
            ));
        }
    }
    // Wedge caps: every other face.
    for (face, _) in body.faces() {
        if walls.contains(&face) {
            continue;
        }
        let outer = outer.clone();
        let holes = holes.clone();
        let th = theta.expect("a full revolve has no caps");
        out.push((
            face,
            Box::new(move |p: Point3<f64>| {
                let (r, y, phi) = cyl_coords(p);
                // On the φ = 0 or φ = θ half-plane (not the opposite one).
                let on0 = r < DELTA || range_cls(phi, 0.0, 0.0) != Cls::Out;
                let on_th = r < DELTA
                    || [-1.0, 0.0, 1.0]
                        .iter()
                        .any(|k| range_cls(phi + k * TAU, th, th) != Cls::Out);
                if !(on0 || on_th) {
                    return Cls::Out;
                }
                in_profile(&outer, &holes, (r, y))
            }),
            "cap",
        ));
    }
    out
}

// ------------------------------------------------------------------ //
// Profiles
// ------------------------------------------------------------------ //

fn l_profile() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![
        ((0.0, 0.0), 0.0),
        ((1.0, 0.0), 0.0),
        ((1.0, 0.5), 0.0),
        ((0.5, 0.5), 0.0),
        ((0.5, 1.0), 0.0),
        ((0.0, 1.0), 0.0),
    ]]
}

fn slab_with_holes() -> Vec<Vec<((f64, f64), f64)>> {
    vec![
        vec![
            ((0.0, 0.0), 0.0),
            ((3.0, 0.0), 0.0),
            ((3.0, 2.0), 0.0),
            ((0.0, 2.0), 0.0),
        ],
        // A square hole well inside.
        vec![
            ((0.5, 0.5), 0.0),
            ((1.0, 0.5), 0.0),
            ((1.0, 1.0), 0.0),
            ((0.5, 1.0), 0.0),
        ],
        // A hole 0.05 from the outer's right wall.
        vec![
            ((2.6, 0.5), 0.0),
            ((2.95, 0.5), 0.0),
            ((2.95, 1.5), 0.0),
            ((2.6, 1.5), 0.0),
        ],
    ]
}

fn bumped_block() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![
        ((0.0, 0.0), 0.0),
        ((2.0, 0.0), 0.0),
        ((2.0, 0.5), 0.0),
        ((1.5, 0.5), 1.0),
        ((0.5, 0.5), 0.0),
    ]]
}

/// A quarter-circle bump (bulge tan(π/8)): an extruded arc face of
/// 90°, and a cap with a minor-arc envelope.
fn notched_block() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![
        ((0.0, 0.0), 0.0),
        ((2.0, 0.0), 0.0),
        ((2.0, 1.0), 0.0),
        ((1.5, 1.0), (PI / 8.0).tan()),
        ((0.5, 1.0), 0.0),
        ((0.0, 1.0), 0.0),
    ]]
}

fn disc() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![((-0.5, 0.0), 1.0), ((0.5, 0.0), 1.0)]]
}

fn annulus_rect() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![
        ((1.0, 0.0), 0.0),
        ((2.0, 0.0), 0.0),
        ((2.0, 1.0), 0.0),
        ((1.0, 1.0), 0.0),
    ]]
}

fn ball() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![((0.0, -1.0), 1.0), ((0.0, 1.0), 0.0)]]
}

/// The spool: an annular meridian whose outer wall is a 60° arc
/// (test_support's shape), bore on-axis at 0.5.
fn spool() -> Vec<Vec<((f64, f64), f64)>> {
    let b = (PI / 3.0 / 4.0).tan();
    vec![vec![
        ((0.5, 0.0), 0.0),
        ((2.0, 0.0), 0.0),
        ((1.5 + 0.5 * (PI / 3.0).cos(), 0.5 * (PI / 3.0).sin()), 0.0),
        ((0.5, 0.5 * (PI / 3.0).sin()), 0.0),
    ]]
    .into_iter()
    .map(|mut l| {
        // (2,0) → arc of 60° about (1.5,0) radius 0.5, CCW: bulge +b.
        l[1].1 = b;
        l
    })
    .collect()
}

/// A square plate with a round hole: the hole is two half arcs.
fn plate_with_round_hole() -> Vec<Vec<((f64, f64), f64)>> {
    vec![
        vec![
            ((0.0, 0.0), 0.0),
            ((2.0, 0.0), 0.0),
            ((2.0, 2.0), 0.0),
            ((0.0, 2.0), 0.0),
        ],
        vec![((0.6, 1.0), 1.0), ((1.4, 1.0), 1.0)],
    ]
}

/// A trapezoid off the axis whose slanted edge sweeps a CONE that keeps
/// clear of its apex.
fn frustum_section() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![
        ((1.0, 0.0), 0.0),
        ((2.0, 0.0), 0.0),
        ((1.5, 1.0), 0.0),
        ((1.0, 1.0), 0.0),
    ]]
}

/// A quarter disc about the origin: its arc sweeps a SPHERE octant
/// whose loop touches the pole.
fn quarter_disc() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![
        ((0.0, 0.0), 0.0),
        ((1.0, 0.0), (PI / 8.0).tan()),
        ((0.0, 1.0), 0.0),
    ]]
}

// ------------------------------------------------------------------ //
// Real bodies against the oracle
// ------------------------------------------------------------------ //

#[test]
fn an_l_extrude_is_sound_on_every_face() {
    let g = run_extrude("L", &l_profile(), 1.5);
    assert_sound(&g);
    assert_decides(&g, "plane");
}

#[test]
fn a_slab_with_two_holes_is_sound_on_every_face() {
    let g = run_extrude("slab2holes", &slab_with_holes(), 1.0);
    assert_sound(&g);
    assert_decides(&g, "plane");
}

#[test]
fn a_bumped_block_is_sound_on_every_face() {
    let g = run_extrude("bumped", &bumped_block(), 1.0);
    assert_sound(&g);
    assert_decides(&g, "cylinder");
}

#[test]
fn a_notched_block_is_sound_on_every_face() {
    let g = run_extrude("notched", &notched_block(), 1.0);
    assert_sound(&g);
    assert_decides(&g, "plane");
    assert_decides(&g, "cylinder");
}

#[test]
fn a_disc_is_sound() {
    let g = run_extrude("disc", &disc(), 1.0);
    assert_sound(&g);
    assert_decides(&g, "plane");
    assert_decides(&g, "cylinder");
}

/// **A plate with a round hole drops the cells inside the hole.** The
/// hole is two half arcs, so no chord polygon of it has an interior; a
/// cell in the hole meets no edge on its carrier, and its witness reads
/// `Outside`.
///
/// **The witness row.** A cell at the hole's centre and a cell deep in
/// the material are both boundary-free, and only the witness tells
/// them apart: a door that skipped it answers both the same, and one of
/// the two assertions below goes red.
#[test]
fn a_plate_with_a_round_hole_drops_the_cells_in_the_hole() {
    let loops = plate_with_round_hole();
    let g = run_extrude("round-hole", &loops, 0.5);
    assert_sound(&g);
    assert_decides(&g, "plane");
    let vp = profile_of(&loops);
    let t = extrude(
        &vp,
        Extrusion::Distance {
            depth: iv(0.5),
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap();
    let s = surface_f64(&face_surface(&t.body, t.bottom));
    let Surface::Plane {
        origin,
        normal,
        u_ref,
    } = s
    else {
        panic!("the bottom cap is a plane")
    };
    let w = normal.cross(u_ref);
    let chart = |x: f64, y: f64| {
        let p = Point3::new(x, y, 0.0) - origin;
        (p.dot(u_ref), p.dot(w))
    };
    let cell = |(u, v): (f64, f64), r: f64| ((u - r, u + r), (v - r, v + r));
    let (hu, hv) = cell(chart(1.0, 1.0), 0.1);
    assert_eq!(
        cell_region(&t.body, t.bottom, hu, hv, band()),
        CellRegion::Outside,
        "a cell at the hole's centre is out of the plate"
    );
    let (mu, mv) = cell(chart(0.3, 0.3), 0.1);
    assert_eq!(
        cell_region(&t.body, t.bottom, mu, mv, band()),
        CellRegion::Inside,
        "a cell deep in the material is in the plate"
    );
}

#[test]
fn a_quarter_annulus_revolved_either_way_is_sound() {
    for (name, theta) in [
        ("annulus+90", PI / 2.0),
        ("annulus-90", -PI / 2.0),
        ("annulus-270", -3.0 * PI / 2.0),
    ] {
        let g = run_revolve(
            name,
            &annulus_rect(),
            Revolution::Partial(iv(theta)),
            Some(theta),
        );
        assert_sound(&g);
        assert_decides(&g, "cylinder");
    }
}

#[test]
fn a_full_revolve_is_sound() {
    let g = run_revolve("annulus-full", &annulus_rect(), Revolution::Full, None);
    assert_sound(&g);
    // The whole-turn walls have only their rims for a boundary, which
    // the window's padding stays too near to clear: they decide inside.
    assert!(
        g.iter()
            .filter(|f| f.kind == "cylinder")
            .all(|f| f.stats.inside > 0),
        "every whole-turn wall decides cells inside"
    );
    assert_decides(&g, "plane");
}

/// **A cone face decides cells**: the frustum's slanted wall, a quarter
/// turn of it, drops the cells of the other three quarters and of the
/// mirror nappe.
#[test]
fn a_cone_wall_decides_its_cells() {
    let g = run_revolve(
        "frustum+90",
        &frustum_section(),
        Revolution::Partial(iv(PI / 2.0)),
        Some(PI / 2.0),
    );
    assert_sound(&g);
    assert_decides(&g, "cone");
}

/// **A cell that reaches a cone's apex is kept.** The double cone is
/// connected only through its apex, so a patch round the apex can be
/// boundary-free and still hold points of the face and of the other
/// nappe. On the frustum's wall, which keeps clear of its apex, a cell
/// round the apex meets no edge, and its station lies off the face: a
/// door that skipped the apex would call it `Outside` from that one
/// station. It reads `Undecided`; the same cell moved off the axis
/// decides.
#[test]
fn a_cell_reaching_a_cone_apex_is_kept() {
    let vp = profile_of(&frustum_section());
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: v2(0.0, 1.0),
    };
    let t = revolve(&vp, axis, Revolution::Partial(iv(PI / 2.0)), Tol::witness()).unwrap();
    let (face, _) = t
        .body
        .faces()
        .find(|(f, _)| matches!(face_surface(&t.body, *f), Surface::Cone { .. }))
        .expect("the frustum has a cone wall");
    let s = surface_f64(&face_surface(&t.body, face));
    let Surface::Cone { apex, .. } = s else {
        unreachable!("a cone")
    };
    // A cell straddling the slant origin, at the far side of the turn
    // from the wall's quarter: the apex is its only feature.
    let at_apex = cell_region(&t.body, face, (PI, PI + 0.4), (-0.2, 0.2), band());
    assert!(
        s.eval(PI + 0.2, 0.0).distance(apex) < 1e-9,
        "the cell holds the apex"
    );
    assert_eq!(
        at_apex,
        CellRegion::Undecided,
        "a cell round the apex is kept"
    );
    let off = cell_region(&t.body, face, (PI, PI + 0.4), (0.6, 1.0), band());
    assert_eq!(
        off,
        CellRegion::Outside,
        "the same azimuths off the apex, away from the wall, drop"
    );
}

/// **A sphere face decides cells**, both poles on its loops. Only the
/// partial revolve: a full one splits the sphere into two halves
/// between meridians, which the oracle's `φ` band cannot tell apart.
#[test]
fn a_ball_is_sound_and_its_sphere_decides_its_cells() {
    let g = run_revolve(
        "ball+90",
        &ball(),
        Revolution::Partial(iv(PI / 2.0)),
        Some(PI / 2.0),
    );
    assert_sound(&g);
    assert_decides(&g, "sphere");
}

/// **A torus face decides cells**: the spool's arc wall.
#[test]
fn a_spool_is_sound_and_its_torus_decides_its_cells() {
    // The torus wall spans a quarter turn by a sixth of the tube: at
    // 48 cells a side its middle cells clear the edges.
    let g = run_revolve_at(
        "spool+90",
        &spool(),
        Revolution::Partial(iv(PI / 2.0)),
        Some(PI / 2.0),
        48,
    );
    assert_sound(&g);
    assert_decides(&g, "torus");
    assert_sound(&run_revolve("spool-full", &spool(), Revolution::Full, None));
}

/// **The pole-touching sphere octant.** A quarter disc revolved 90°
/// sweeps a sphere octant whose loop has the pole as a vertex. A chord
/// polygon drawn through the pole's arbitrary azimuth was a triangle
/// where the region is a rectangle, and 210 of 750 cells certified
/// outside held material. The door reads no azimuth: a cell at the pole
/// meets the vertex and is kept, and every other cell decides on the
/// sphere as it does anywhere.
#[test]
fn a_sphere_octant_touching_the_pole_is_sound() {
    let g = run_revolve(
        "quarter-disc+90",
        &quarter_disc(),
        Revolution::Partial(iv(PI / 2.0)),
        Some(PI / 2.0),
    );
    assert_sound(&g);
    assert_decides(&g, "sphere");
}

// ------------------------------------------------------------------ //
// Section curves: faces bounded by ellipses and spiric ovals
// ------------------------------------------------------------------ //

/// A constraint of a half, `≤ 0` inside, in metres near its zero.
type Constraint = Box<dyn Fn(Point3<f64>) -> f64>;

/// The oracle on a face of a split half: a point of the face's carrier
/// is in the face when it satisfies every OTHER constraint of the half
/// (each face of these halves is its carrier's whole meeting with the
/// closed half), and not judged within the skin of any.
fn section_oracle(constraints: Vec<Constraint>, own: usize) -> Oracle {
    Box::new(move |p| {
        let mut near = false;
        for (k, c) in constraints.iter().enumerate() {
            if k == own {
                continue;
            }
            let g = c(p);
            if g > DELTA {
                return Cls::Out;
            }
            if g > -DELTA {
                near = true;
            }
        }
        if near { Cls::Near } else { Cls::In }
    })
}

/// The halves of `body` split by the plane through `origin` normal to
/// `normal`, each with the sign of its side of the plane.
fn split_halves(
    body: Body<Interval>,
    origin: [f64; 3],
    normal: [f64; 3],
) -> Vec<(Body<Interval>, f64)> {
    let tol = Tol::witness();
    let at_rest = topo::test_support::finished("the operand", body, tol);
    let pt = |a: [f64; 3]| Point3::new(iv(a[0]), iv(a[1]), iv(a[2]));
    let plane = topo::test_support::split_plane(
        pt(origin),
        geom_core::Vec3::new(iv(normal[0]), iv(normal[1]), iv(normal[2])),
        tol,
    );
    let split = topo::split(&at_rest, &plane, tol).expect("the split builds at Interval");
    [(split.above, 1.0), (split.below, -1.0)]
        .into_iter()
        .filter_map(|(part, side)| match part {
            topo::SplitPart::Body(b) => Some((b.into_body(), side)),
            topo::SplitPart::Empty => None,
        })
        .collect()
}

/// Whether some edge of `face` lies on a carrier `pick` names.
fn bounded_by(
    body: &Body<Interval>,
    face: FaceKey,
    pick: fn(&geom::Curve3<Interval>) -> bool,
) -> bool {
    let f = body.get_face(face).unwrap();
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|l| match body.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => body.loop_cycle(first),
            LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .any(|he| {
            let e = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
            body.get_curve_geom(e.curve)
                .unwrap()
                .certified()
                .is_some_and(|c| pick(c.carrier()))
        })
}

/// **An oblique section of a cylinder.** The unit cylinder `z ∈ [0, 2]`
/// split by the plane through `(0, 0, 1)` normal to `(1, 0, 2)`: each
/// half's cut face is a plane bounded by an ellipse, and its wall is
/// bounded by an elliptic arc. Both decide cells both ways against the
/// oracle, cells beside the ellipse included: a door that called every
/// elliptic edge clear would decide the cells it straddles from one
/// station, and one that read the conic's annulus too narrow would drop
/// a cell the ellipse passes through.
#[test]
fn an_oblique_cylinder_section_decides_beside_its_ellipse() {
    let tol = Tol::witness();
    let circle = profile::RawLoop::new([(
        p2(1.0, 0.0),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: p2(0.0, 0.0),
            radius: iv(1.0),
            sweep: iv(TAU),
        }),
    )]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![circle])
        .validate(tol)
        .unwrap();
    let t = extrude(
        &vp,
        Extrusion::Distance {
            depth: iv(2.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap();
    let (origin, normal) = ([0.0, 0.0, 1.0], [1.0, 0.0, 2.0]);
    let mut faces = 0usize;
    for (half, side) in split_halves(t.body, origin, normal) {
        for (face, _) in half.faces() {
            if !bounded_by(&half, face, |c| matches!(c, geom::Curve3::Ellipse { .. })) {
                continue;
            }
            faces += 1;
            let own = match surface_f64(&face_surface(&half, face)) {
                Surface::Cylinder { .. } => 0,
                Surface::Plane { .. } => 3,
                ref other => panic!("an ellipse on a {other:?}"),
            };
            let constraints: Vec<Constraint> = vec![
                Box::new(|p| p.x.hypot(p.y) - 1.0),
                Box::new(|p| -p.z),
                Box::new(|p| p.z - 2.0),
                Box::new(move |p| {
                    -side * ((p.x - origin[0]) * normal[0] + (p.z - origin[2]) * normal[2])
                        / 5f64.sqrt()
                }),
            ];
            let oracle = section_oracle(constraints, own);
            let label = format!("oblique-section/{face:?}");
            let st = grid(&half, face, 32, &oracle, &label);
            assert_eq!(
                st.violations, 0,
                "{label}: a decided cell contradicts the oracle"
            );
            assert!(
                st.inside > 0 && st.outside > 0,
                "{label}: the face decides cells both ways: inside={} outside={}",
                st.inside,
                st.outside
            );
        }
    }
    assert_eq!(
        faces, 4,
        "each half's cut face and wall are bounded by the ellipse"
    );
}
