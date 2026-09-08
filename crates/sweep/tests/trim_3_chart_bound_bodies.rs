//! **TRIM-3 PR-1 acceptance on REAL BODIES** — `topo::chart_boundary`
//! and `MetredBound::certifies_outside` on bodies built through the
//! public constructors (`sweep::extrude`, `sweep::revolve`), every
//! certified cell checked against an INDEPENDENT 3-D oracle: profile
//! segment loci and an even-odd region walk over bulge chains, which
//! shares no code with the description under test.
//!
//! **Provenance.** The oracle, the fixtures and the grid driver are
//! adopted from review lane **trim3-r1**'s probe file (rows p1–p18 of
//! its report), with review lane **trim3-r2**'s two additions: an
//! arc-bounded cap must actually reach the `Envelope` arm (or the
//! whole `chart_edge` envelope path stays unpinned), and a VERTICAL
//! planar wall must describe through the consumer's RE-CHART, which is
//! the configuration `clearance.rs` will feed and which no hand-built
//! row reaches. The dual's adjudication is what put both here; the
//! shapes are the reviewers'.
//!
//! It lives in `sweep` and not beside the `topo` rows because the
//! constructors do: `topo` cannot depend on `sweep`.
//!
//! ε posture: no ε literal. The band is the run's own; the oracle's
//! `DELTA` is a GEOMETRIC skin around the loci, not a tolerance, and
//! it only ever moves a cell from `In`/`Out` into `Near`, which the
//! comparison then declines to judge.

#![cfg(feature = "interval")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::cast_precision_loss,
    clippy::float_cmp
)]

use std::f64::consts::{PI, TAU};

use geom::Surface;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Real, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{
    Body, ChartBound, ChartEdge, ChartLoop, FaceKey, LoopBoundary, MetredRect, chart_boundary,
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}
fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}
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
    let (bx, by) = s.b;
    let l = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
    let u = ((bx - ax) / l, (by - ay) / l);
    let nrm = (-u.1, u.0);
    let b = s.bulge;
    let apothem = l * (1.0 - b * b) / (4.0 * b);
    let c = (
        0.5 * (ax + bx) + nrm.0 * apothem,
        0.5 * (ay + by) + nrm.1 * apothem,
    );
    let r = l * (1.0 + b * b) / (4.0 * b.abs());
    let start = (ay - c.1).atan2(ax - c.0);
    let sweep = 4.0 * b.atan();
    (c, r, start, sweep)
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

/// The consumer's plane re-chart (`clearance.rs` `in_plane_axis` /
/// `chart_frame`), replicated.
fn rechart(s: &Surface<Interval>) -> Option<Surface<Interval>> {
    let Surface::Plane { origin, normal, .. } = s else {
        return None;
    };
    let unit = |k: usize| {
        let (z, o) = (Interval::zero(), Interval::one());
        match k {
            0 => Vec3::new(o, z, z),
            1 => Vec3::new(z, o, z),
            _ => Vec3::new(z, z, o),
        }
    };
    let mut best = (f64::NEG_INFINITY, 0usize);
    for k in 0..3 {
        let lo = normal.cross(unit(k)).norm().lo();
        if lo.total_cmp(&best.0) == core::cmp::Ordering::Greater {
            best = (lo, k);
        }
    }
    let u = normal.cross(unit(best.1)).normalize();
    Some(Surface::Plane {
        origin: *origin,
        normal: *normal,
        u_ref: u,
    })
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

fn kind(s: &Surface<Interval>) -> &'static str {
    match s {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        _ => "other",
    }
}

fn arms(s: &Surface<Interval>) -> Option<(Interval, Interval)> {
    match s {
        Surface::Plane { .. } => Some((Interval::one(), Interval::one())),
        Surface::Cylinder { radius, .. } => Some((*radius, Interval::one())),
        _ => None,
    }
}

// ------------------------------------------------------------------ //
// The grid comparison
// ------------------------------------------------------------------ //

struct GridStats {
    cells: usize,
    certified: usize,
    loose: usize,
    violations: usize,
    near_only: usize,
    off_branch: usize,
}

fn describe_line(b: &ChartBound<Interval>) -> String {
    let mut segs = 0;
    let mut envs = 0;
    for lp in &b.loops {
        for e in &lp.edges {
            match e {
                ChartEdge::Segment { .. } => segs += 1,
                ChartEdge::Envelope { .. } => envs += 1,
            }
        }
    }
    format!(
        "loops={} (rings={}) segments={} envelopes={} hull=u[{:.4},{:.4}] v[{:.4},{:.4}] period={:?}",
        b.loops.len(),
        b.loops.iter().filter(|l| l.ring).count(),
        segs,
        envs,
        b.hull.u_min.lo(),
        b.hull.u_max.hi(),
        b.hull.v_min.lo(),
        b.hull.v_max.hi(),
        b.period.map(mid)
    )
}

/// Runs `certifies_outside` over an `n×n` grid of cells covering
/// `(u0,u1)×(v0,v1)` in CHART units, checks every certified cell against
/// the 3-D oracle on a 5×5 interior lattice. Returns the stats.
fn grid(
    bound: &ChartBound<Interval>,
    chart: &Surface<Interval>,
    a: (Interval, Interval),
    window: ((f64, f64), (f64, f64)),
    n: usize,
    oracle: &Oracle,
    label: &str,
) -> GridStats {
    // On a periodic chart a cell whose u-range has a lift inside the
    // hull is OFF the walk's branch: the contract says nothing there.
    let branch = bound.period.map(|p| {
        let p = mid(p);
        (bound.hull.u_max.hi() - p, bound.hull.u_min.lo() + p)
    });
    let metred = bound.metred(a);
    let b = band();
    let sf = surface_f64(chart);
    let (au, av) = (mid(a.0), mid(a.1));
    let ((u0, u1), (v0, v1)) = window;
    let mut st = GridStats {
        cells: 0,
        certified: 0,
        loose: 0,
        violations: 0,
        near_only: 0,
        off_branch: 0,
    };
    let du = (u1 - u0) / n as f64;
    let dv = (v1 - v0) / n as f64;
    for i in 0..n {
        for j in 0..n {
            let cu = (u0 + i as f64 * du, u0 + (i + 1) as f64 * du);
            let cv = (v0 + j as f64 * dv, v0 + (j + 1) as f64 * dv);
            st.cells += 1;
            let rect = MetredRect::new(cu.0 * au, cu.1 * au, cv.0 * av, cv.1 * av);
            let cert = metred.certifies_outside(rect, b);
            // Oracle over the interior lattice.
            let mut any_in = false;
            let mut any_near = false;
            let mut all_out = true;
            for p in 0..5 {
                for q in 0..5 {
                    let u = cu.0 + (0.1 + 0.2 * p as f64) * (cu.1 - cu.0);
                    let v = cv.0 + (0.1 + 0.2 * q as f64) * (cv.1 - cv.0);
                    match oracle(sf.eval(u, v)) {
                        Cls::In => {
                            any_in = true;
                            all_out = false;
                        }
                        Cls::Near => {
                            any_near = true;
                            all_out = false;
                        }
                        Cls::Out => {}
                    }
                }
            }
            let on_branch = branch.is_none_or(|(lo, hi)| cu.0 >= lo - 1e-9 && cu.1 <= hi + 1e-9);
            if cert {
                st.certified += 1;
                if any_in && !on_branch {
                    st.off_branch += 1;
                } else if any_in {
                    st.violations += 1;
                    eprintln!(
                        "  !!! VIOLATION {label}: cell u[{:.5},{:.5}] v[{:.5},{:.5}] certified outside but the oracle finds material",
                        cu.0, cu.1, cv.0, cv.1
                    );
                } else if any_near {
                    st.near_only += 1;
                }
            } else if all_out {
                st.loose += 1;
            }
        }
    }
    eprintln!(
        "  {label}: cells={} certified={} loose(all-out, not certified)={} near-only={} off-branch-material-certified={} VIOLATIONS={}",
        st.cells, st.certified, st.loose, st.near_only, st.off_branch, st.violations
    );
    st
}

fn window_of(b: &ChartBound<Interval>, pad: f64) -> ((f64, f64), (f64, f64)) {
    let (u0, u1) = (b.hull.u_min.lo(), b.hull.u_max.hi());
    let (v0, v1) = (b.hull.v_min.lo(), b.hull.v_max.hi());
    let pu = (0.25 * (u1 - u0)).max(pad);
    let pv = (0.25 * (v1 - v0)).max(pad);
    ((u0 - pu, u1 + pu), (v0 - pv, v1 + pv))
}

/// Describes `face` in `chart`, prints the outcome, and runs the grid
/// if the chart has exact arms and an oracle is supplied.
fn probe_face(
    body: &Body<Interval>,
    face: FaceKey,
    chart: &Surface<Interval>,
    oracle: Option<&Oracle>,
    label: &str,
) -> (Option<ChartBound<Interval>>, Option<GridStats>) {
    match chart_boundary(body, face, chart, band()) {
        Err(e) => {
            eprintln!("{label} [{}]: REFUSED: {e}", kind(chart));
            (None, None)
        }
        Ok(b) => {
            eprintln!("{label} [{}]: {}", kind(chart), describe_line(&b));
            let stats = match (arms(chart), oracle) {
                (Some(a), Some(o)) => Some(grid(&b, chart, a, window_of(&b, 0.3), 32, o, label)),
                _ => None,
            };
            (Some(b), stats)
        }
    }
}

// ------------------------------------------------------------------ //
// Fixtures
// ------------------------------------------------------------------ //

fn profile_of(loops: &[Vec<((f64, f64), f64)>]) -> profile::ValidatedProfile<Interval> {
    let lps: Vec<ProfileLoop<Interval>> = loops
        .iter()
        .map(|l| {
            ProfileLoop::new(
                l.iter()
                    .map(|(p, b)| ProfileVertex::new(p2(p.0, p.1), iv(*b)))
                    .collect(),
            )
        })
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
    for faces in &t.side_faces {
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
    let _ = &t.walls;
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

fn run_extrude(name: &str, loops: &[Vec<((f64, f64), f64)>], h: f64) -> Vec<GridStats> {
    eprintln!("=== extrude fixture: {name} (h = {h}) ===");
    let vp = profile_of(loops);
    let t = extrude(&vp, Extrusion::Distance(iv(h)), Tol::witness()).unwrap();
    let mut all = Vec::new();
    for (face, oracle, fname) in extrude_oracles(&t.body, &t, loops, h) {
        let s = face_surface(&t.body, face);
        let label = format!("{name}/{fname}/{face:?}");
        let (_, st) = probe_face(&t.body, face, &s, Some(&oracle), &format!("{label} stored"));
        all.extend(st);
        if let Some(rc) = rechart(&s) {
            let (_, st) = probe_face(
                &t.body,
                face,
                &rc,
                Some(&oracle),
                &format!("{label} rechart"),
            );
            all.extend(st);
        }
    }
    all
}

fn run_revolve(
    name: &str,
    loops: &[Vec<((f64, f64), f64)>],
    rev: Revolution<Interval>,
    theta: Option<f64>,
) -> Vec<GridStats> {
    eprintln!("=== revolve fixture: {name} (theta = {theta:?}) ===");
    let vp = profile_of(loops);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(iv(0.0), iv(1.0)),
    };
    let t = revolve(&vp, axis, rev, Tol::witness()).unwrap();
    let mut all = Vec::new();
    for (face, oracle, fname) in revolve_oracles(&t.body, &t, loops, theta) {
        let s = face_surface(&t.body, face);
        let label = format!("{name}/{fname}/{face:?}");
        let (_, st) = probe_face(&t.body, face, &s, Some(&oracle), &format!("{label} stored"));
        all.extend(st);
        if let Some(rc) = rechart(&s) {
            let (_, st) = probe_face(
                &t.body,
                face,
                &rc,
                Some(&oracle),
                &format!("{label} rechart"),
            );
            all.extend(st);
        }
    }
    all
}

fn assert_sound(stats: &[GridStats]) {
    let v: usize = stats.iter().map(|s| s.violations).sum();
    let c: usize = stats.iter().map(|s| s.certified).sum();
    let faces_described = stats.len();
    eprintln!("  => faces gridded={faces_described} certified cells={c} violations={v}");
    assert_eq!(v, 0, "a certified cell held material");
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

/// Wider than τ, so a spurious period on the plane chart lands a ring
/// copy in the material.
fn wide_slab_with_hole() -> Vec<Vec<((f64, f64), f64)>> {
    vec![
        vec![
            ((0.0, 0.0), 0.0),
            ((8.0, 0.0), 0.0),
            ((8.0, 2.0), 0.0),
            ((0.0, 2.0), 0.0),
        ],
        vec![
            ((0.5, 0.5), 0.0),
            ((1.0, 0.5), 0.0),
            ((1.0, 1.0), 0.0),
            ((0.5, 1.0), 0.0),
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

// ------------------------------------------------------------------ //
// C1 / C4 / C5 — real bodies against the oracle
// ------------------------------------------------------------------ //

#[test]
fn p1_l_extrude_is_sound_on_every_face() {
    assert_sound(&run_extrude("L", &l_profile(), 1.5));
}

#[test]
fn p2_slab_with_two_holes_is_sound_on_every_face() {
    assert_sound(&run_extrude("slab2holes", &slab_with_holes(), 1.0));
}

#[test]
fn p3_bumped_block_is_sound_on_every_face() {
    assert_sound(&run_extrude("bumped", &bumped_block(), 1.0));
}

#[test]
fn p4_notched_block_quarter_arc_is_sound() {
    assert_sound(&run_extrude("notched", &notched_block(), 1.0));
}

#[test]
fn p5_disc_is_sound() {
    assert_sound(&run_extrude("disc", &disc(), 1.0));
}

#[test]
fn p6_quarter_annulus_positive_revolve_is_sound() {
    assert_sound(&run_revolve(
        "annulus+90",
        &annulus_rect(),
        Revolution::Partial(iv(PI / 2.0)),
        Some(PI / 2.0),
    ));
}

#[test]
fn p7_quarter_annulus_negative_revolve_is_sound() {
    assert_sound(&run_revolve(
        "annulus-90",
        &annulus_rect(),
        Revolution::Partial(iv(-PI / 2.0)),
        Some(-PI / 2.0),
    ));
}

#[test]
fn p8_three_quarter_annulus_negative_revolve_is_sound() {
    assert_sound(&run_revolve(
        "annulus-270",
        &annulus_rect(),
        Revolution::Partial(iv(-3.0 * PI / 2.0)),
        Some(-3.0 * PI / 2.0),
    ));
}

#[test]
fn p9_full_revolve_walls_describe_or_refuse() {
    // Measures C4: a full revolve's closed wall (seam chain).
    assert_sound(&run_revolve(
        "annulus-full",
        &annulus_rect(),
        Revolution::Full,
        None,
    ));
}

#[test]
fn p10_ball_partial_and_full() {
    // A ball's sphere wall meets both poles, so `chart_boundary` must
    // REFUSE it; whatever else describes must still be sound.
    assert_sound(&run_revolve(
        "ball+90",
        &ball(),
        Revolution::Partial(iv(PI / 2.0)),
        Some(PI / 2.0),
    ));
    assert_sound(&run_revolve("ball-full", &ball(), Revolution::Full, None));
}

#[test]
fn p11_spool_partial_and_full() {
    assert_sound(&run_revolve(
        "spool+90",
        &spool(),
        Revolution::Partial(iv(PI / 2.0)),
        Some(PI / 2.0),
    ));
    assert_sound(&run_revolve("spool-full", &spool(), Revolution::Full, None));
}

#[test]
fn p12_wide_slab_with_hole_is_sound() {
    // Under `TRIM3_MUTANT=m17_plane_period` this must go RED: the ring
    // copy at u + τ lands in the material.
    assert_sound(&run_extrude("wide", &wide_slab_with_hole(), 1.0));
    // The pointed version: the hole's centre lifted by τ in u is material.
    let loops = wide_slab_with_hole();
    let vp = profile_of(&loops);
    let t = extrude(&vp, Extrusion::Distance(iv(1.0)), Tol::witness()).unwrap();
    let s = face_surface(&t.body, t.bottom);
    let b = chart_boundary(&t.body, t.bottom, &s, band()).unwrap();
    eprintln!(
        "wide/bottom loops={} period={:?}",
        b.loops.len(),
        b.period.map(mid)
    );
    // Hole centre in the stored chart: find it from the ring's edges.
    let ring = b.loops.iter().find(|l| l.ring).unwrap();
    let (mut cu, mut cv) = (0.0, 0.0);
    for e in &ring.edges {
        cu += mid(e.a().x) / ring.edges.len() as f64;
        cv += mid(e.a().y) / ring.edges.len() as f64;
    }
    let m = b.metred((Interval::one(), Interval::one()));
    let lifted = MetredRect::new(cu + TAU - 0.05, cu + TAU + 0.05, cv - 0.05, cv + 0.05);
    let sf = surface_f64(&s);
    let p = sf.eval(cu + TAU, cv);
    eprintln!(
        "hole centre ({cu:.3},{cv:.3}); lifted cell centre maps to {:?}",
        (p.x, p.y, p.z)
    );
    assert!(
        !m.certifies_outside(lifted, band()),
        "a cell one period from the hole is material on a plane"
    );
}

// ------------------------------------------------------------------ //
// C1 contract edge: the description is branch-relative
// ------------------------------------------------------------------ //

#[test]
fn p13_off_branch_cells_of_a_cylinder_face_are_certified_outside() {
    // A quarter annulus's outer cylinder: shift the window by ±τ and
    // count certified cells whose model point IS material. This is the
    // contract's stated scope, not a defect — the description is
    // BRANCH-RELATIVE, and it is recorded here to show that the
    // consumer's root rule (which never hands it an off-branch cell)
    // is load-bearing rather than convenient. The grid driver routes
    // these into `off_branch` and not into `violations`, which is the
    // same distinction spelled in code.
    let loops = annulus_rect();
    let vp = profile_of(&loops);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(iv(0.0), iv(1.0)),
    };
    let t = revolve(&vp, axis, Revolution::Partial(iv(PI / 2.0)), Tol::witness()).unwrap();
    let mut off_branch_material_certified = 0usize;
    for (face, oracle, _) in revolve_oracles(&t.body, &t, &loops, Some(PI / 2.0)) {
        let s = face_surface(&t.body, face);
        if !matches!(s, Surface::Cylinder { .. }) {
            continue;
        }
        let Ok(b) = chart_boundary(&t.body, face, &s, band()) else {
            continue;
        };
        let ((u0, u1), (v0, v1)) = window_of(&b, 0.3);
        for k in [-1.0, 1.0] {
            let st = grid(
                &b,
                &s,
                arms(&s).unwrap(),
                ((u0 + k * TAU, u1 + k * TAU), (v0, v1)),
                16,
                &oracle,
                &format!("off-branch k={k} {face:?}"),
            );
            off_branch_material_certified += st.off_branch;
        }
    }
    eprintln!("off-branch material cells certified outside: {off_branch_material_certified}");
    assert!(
        off_branch_material_certified > 0,
        "expected the branch-relative contract to show: material at u ± τ reads outside"
    );
}

// ------------------------------------------------------------------ //
// C2 — fat intervals only lose cells; the band at any K
// ------------------------------------------------------------------ //

fn widen(b: &ChartBound<Interval>, w: f64) -> ChartBound<Interval> {
    let wp = |p: Point2<Interval>| {
        Point2::new(
            Interval::from_bounds(p.x.lo() - w, p.x.hi() + w),
            Interval::from_bounds(p.y.lo() - w, p.y.hi() + w),
        )
    };
    ChartBound {
        loops: b
            .loops
            .iter()
            .map(|lp| ChartLoop {
                ring: lp.ring,
                edges: lp
                    .edges
                    .iter()
                    .map(|e| match *e {
                        ChartEdge::Segment { a, b } => ChartEdge::Segment { a: wp(a), b: wp(b) },
                        ChartEdge::Envelope { a, b, image, slack } => ChartEdge::Envelope {
                            a: wp(a),
                            b: wp(b),
                            image: wp(image),
                            slack,
                        },
                    })
                    .collect(),
            })
            .collect(),
        hull: b.hull,
        period: b.period,
    }
}

#[test]
fn p14_fat_intervals_only_lose_cells() {
    let loops = bumped_block();
    let vp = profile_of(&loops);
    let t = extrude(&vp, Extrusion::Distance(iv(1.0)), Tol::witness()).unwrap();
    let b = band();
    let mut lost_total = 0usize;
    let mut gained_total = 0usize;
    for (face, _, _) in extrude_oracles(&t.body, &t, &loops, 1.0) {
        let s = face_surface(&t.body, face);
        let Some(a) = arms(&s) else { continue };
        let Ok(desc) = chart_boundary(&t.body, face, &s, b) else {
            continue;
        };
        let ((u0, u1), (v0, v1)) = window_of(&desc, 0.3);
        let n = 24;
        let (au, av) = (mid(a.0), mid(a.1));
        let base = desc.metred(a);
        for w in [1e-9, 1e-7, 1e-4, 1e-2] {
            let fat = widen(&desc, w).metred(a);
            let (mut lost, mut gained) = (0, 0);
            for i in 0..n {
                for j in 0..n {
                    let cu = (
                        u0 + i as f64 * (u1 - u0) / n as f64,
                        u0 + (i + 1) as f64 * (u1 - u0) / n as f64,
                    );
                    let cv = (
                        v0 + j as f64 * (v1 - v0) / n as f64,
                        v0 + (j + 1) as f64 * (v1 - v0) / n as f64,
                    );
                    let rect = MetredRect::new(cu.0 * au, cu.1 * au, cv.0 * av, cv.1 * av);
                    let (c0, c1) = (
                        base.certifies_outside(rect, b),
                        fat.certifies_outside(rect, b),
                    );
                    if c0 && !c1 {
                        lost += 1;
                    }
                    if !c0 && c1 {
                        gained += 1;
                        eprintln!(
                            "  !!! widening by {w} GAINED a certified cell on {face:?}: {rect:?}"
                        );
                    }
                }
            }
            eprintln!(
                "{face:?} [{}] widen {w}: lost={lost} gained={gained}",
                kind(&s)
            );
            lost_total += lost;
            gained_total += gained;
        }
    }
    eprintln!("fat-interval probe: lost={lost_total} gained={gained_total}");
    assert_eq!(gained_total, 0);
}

// ------------------------------------------------------------------ //
// C1 falsification attempt: loops that TOUCH a chart singularity
// (cone apex, sphere pole) without crossing it
// ------------------------------------------------------------------ //

/// A triangle touching the axis along (0,1)→(0,0): its slanted edge
/// sweeps a CONE whose face loop touches the apex.
fn axis_triangle() -> Vec<Vec<((f64, f64), f64)>> {
    vec![vec![
        ((0.0, 0.0), 0.0),
        ((1.0, 0.0), 0.0),
        ((0.0, 1.0), 0.0),
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

fn singular_probe(name: &str, loops: &[Vec<((f64, f64), f64)>], theta: f64) -> (usize, usize) {
    eprintln!("=== singular-touching fixture: {name} (theta = {theta}) ===");
    let vp = profile_of(loops);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(iv(0.0), iv(1.0)),
    };
    let t = revolve(&vp, axis, Revolution::Partial(iv(theta)), Tol::witness()).unwrap();
    let (mut singular_refusals, mut violations) = (0usize, 0usize);
    for (face, oracle, fname) in revolve_oracles(&t.body, &t, loops, Some(theta)) {
        let s = face_surface(&t.body, face);
        let label = format!("{name}/{fname}/{face:?} [{}]", kind(&s));
        match chart_boundary(&t.body, face, &s, band()) {
            Err(topo::PcurveMintError::SingularChartJoint { .. }) => {
                singular_refusals += 1;
                eprintln!("{label}: REFUSED at the chart singularity");
            }
            Err(e) => eprintln!("{label}: REFUSED: {e}"),
            Ok(b) => {
                eprintln!("{label}: {}", describe_line(&b));
                let a = arms(&s).unwrap_or((Interval::one(), Interval::one()));
                let st = grid(&b, &s, a, window_of(&b, 0.3), 32, &oracle, &label);
                violations += st.violations;
            }
        }
    }
    (singular_refusals, violations)
}

/// **The cone apex.** A triangle touching the axis sweeps a cone whose
/// loop runs through the apex, where the chart's first channel has no
/// lever. Adopted from review lane trim3-r1 (its p17), which measured
/// the case reaching the WRAP fence — the right refusal for the wrong
/// reason; it now refuses at the singularity, one check earlier.
#[test]
fn p17_a_cone_face_touching_the_apex_refuses_at_the_singularity() {
    let (refusals, violations) = singular_probe("axis-triangle+90", &axis_triangle(), PI / 2.0);
    assert!(
        refusals >= 1,
        "the apex-touching cone face must refuse SingularChartJoint"
    );
    assert_eq!(
        violations, 0,
        "no face of this body may certify a cell that holds material"
    );
}

/// **The sphere pole.** A quarter disc revolved 90° sweeps a sphere
/// octant whose loop touches the pole. Adopted from review lane
/// trim3-r1 (its p18), which measured the pre-fix behaviour: the loop
/// was ACCEPTED, its chord polygon was a triangle where the region is
/// a rectangle, and **210 of 750 certified cells held material**.
#[test]
fn p18_a_sphere_face_touching_the_pole_refuses_at_the_singularity() {
    let (refusals, violations) = singular_probe("quarter-disc+90", &quarter_disc(), PI / 2.0);
    assert!(
        refusals >= 1,
        "the pole-touching sphere face must refuse SingularChartJoint"
    );
    assert_eq!(
        violations, 0,
        "no face of this body may certify a cell that holds material"
    );
}

// ------------------------------------------------------------------ //
// The two shapes review lane trim3-r2 named: the envelope arm must be
// REACHED, and a vertical wall must describe through the RE-CHART
// ------------------------------------------------------------------ //

/// Counts the described faces and the `Envelope` edges among them, on
/// both the stored chart and the consumer's re-chart.
fn describe_census(body: &Body<Interval>, faces: &[FaceKey]) -> (usize, usize, usize, usize) {
    let (mut described, mut envelopes, mut rechart_planes, mut vertical_planes) = (0, 0, 0, 0);
    for &face in faces {
        let s = face_surface(body, face);
        if let Ok(b) = chart_boundary(body, face, &s, band()) {
            described += 1;
            envelopes += b
                .loops
                .iter()
                .flat_map(|l| l.edges.iter())
                .filter(|e| matches!(e, ChartEdge::Envelope { .. }))
                .count();
        }
        // A VERTICAL plane: its normal is horizontal, which is exactly
        // the family whose stored `u_ref` is sign-hulled at the
        // interval scalar (the spec's refutation 2).
        if let Surface::Plane { normal, .. } = &s
            && normal.z.hi().abs() < 1e-9
            && normal.z.lo().abs() < 1e-9
        {
            vertical_planes += 1;
            if let Some(rc) = rechart(&s)
                && chart_boundary(body, face, &rc, band()).is_ok()
            {
                rechart_planes += 1;
            }
        }
    }
    (described, envelopes, vertical_planes, rechart_planes)
}

/// **The envelope arm is reached on a real body.** `chart_edge`'s
/// `Envelope` path — the span enclosure, the control hull, the stored
/// certificate's `slack` — is what the hand-built rows can only
/// simulate. Review lane trim3-r2 measured that three plausible
/// defects in it (the image as the chord box, the straightness gate
/// without its carrier check, the slack dropped) left all nine
/// hand-built rows green. This row, with `p3`/`p4`'s oracle grids, is
/// what stands against them.
#[test]
fn r2_the_envelope_arm_is_reached_and_sound_on_an_arc_bounded_cap() {
    for (name, loops) in [("bumped", bumped_block()), ("notched", notched_block())] {
        let vp = profile_of(&loops);
        let t = extrude(&vp, Extrusion::Distance(iv(1.0)), Tol::witness()).unwrap();
        let faces: Vec<FaceKey> = t.body.faces().map(|(k, _)| k).collect();
        let (described, envelopes, _, _) = describe_census(&t.body, &faces);
        eprintln!("{name}: described={described} envelope edges={envelopes}");
        assert!(
            envelopes > 0,
            "{name}: an arc-bounded cap must produce at least one Envelope edge,              or the whole chart_edge envelope path is unpinned"
        );
        // And the oracle says every certified cell is honest.
        assert_sound(&run_extrude(name, &loops, 1.0));
    }
}

/// **A vertical planar wall describes through the consumer's
/// re-chart.** Review lane trim3-r2 measured `chart_boundary` refusing
/// 3 of 6 planar side walls of an extruded L on their STORED chart —
/// `Escalated{pcurve_loop_continuity}`, the sign-hulled `u_ref` the
/// spec's refutation 2 names — and no row in the unit describing one
/// through a re-chart, which is exactly what `clearance.rs`'s
/// `window_of` will pass. The re-chart is replicated here from that
/// function (`in_plane_axis` / `chart_frame`).
#[test]
fn r2_a_vertical_planar_wall_describes_through_the_rechart() {
    let loops = l_profile();
    let vp = profile_of(&loops);
    let t = extrude(&vp, Extrusion::Distance(iv(1.5)), Tol::witness()).unwrap();
    let faces: Vec<FaceKey> = t.body.faces().map(|(k, _)| k).collect();
    let (_, _, vertical, via_rechart) = describe_census(&t.body, &faces);
    eprintln!("L extrude: vertical planar walls={vertical} described via re-chart={via_rechart}");
    assert!(vertical >= 6, "an extruded L has six vertical walls");
    assert_eq!(
        vertical, via_rechart,
        "EVERY vertical planar wall must describe through the consumer's re-chart"
    );
    // Sound on every face, stored chart and re-chart alike.
    assert_sound(&run_extrude("L-rechart", &loops, 1.5));
}
