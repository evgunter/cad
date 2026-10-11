//! Reviewer probes for PR 4746 (seam regions in tier 3's check 6).
//!
//! `review_4746_flip_regions`: random booleans of balls (random chart
//! poles), slab dimples, balls against tilted bricks and three-ball
//! chains. For every body: the unflipped body must not be refused with
//! `CurvedSenseInverted` on a face whose own loop is `Unencoded`
//! (regression); then for every seam region (faces of one surface entity
//! reached across shared edges, the harness's own BFS) with an
//! `Unencoded` member, every subset of members (all subsets up to 4
//! members, else singles + all) is flipped. A pass is classified by an
//! independent oracle: the region's boundary carries both poles
//! (sampled geometry) — the disclosed residual — or not (a FAILURE).
//!
//! Knobs: `R4746_SEED` (xorshift seed, default R2's 0xFEEDFACE),
//! `R4746_N` (iterations per family, default 25).
#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::panic,
    clippy::expect_used
)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Affine3, Point3, Tol, Vec3};
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, brick, finished};
use topo::{Body, FaceKey, ValidationError};
use topo::{FaceSurface, MefSite};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn dir(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            );
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
}

fn tol() -> Tol {
    Tol::witness()
}

fn own_unencoded(body: &Body<f64>, face: FaceKey) -> bool {
    let f = body.faces().find(|(k, _)| *k == face).unwrap().1;
    let Some(s @ Surface::Sphere { .. }) = body.get_surface(f.surface) else {
        return false;
    };
    let Ok(loops) = core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .map(|l| topo::props::loop_edges(body, l).map(|(e, _)| e))
        .collect::<Result<Vec<_>, _>>()
    else {
        return false;
    };
    let loops: Vec<&[_]> = loops.iter().map(Vec::as_slice).collect();
    let band = geom_core::Band::linear(tol()).unwrap();
    geom_brep::props::boundary_material_sign_loops(s, &loops, band)
        == Ok(geom_brep::props::MaterialSign::Unencoded)
}

/// The harness's own seam regions: BFS over same-surface-entity faces
/// across shared edges.
fn regions(body: &Body<f64>) -> Vec<Vec<FaceKey>> {
    let mut seen: Vec<FaceKey> = Vec::new();
    let mut out = Vec::new();
    for (k, f) in body.faces() {
        if !matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })) || seen.contains(&k)
        {
            continue;
        }
        let mut reg = vec![k];
        let mut i = 0;
        while i < reg.len() {
            let g = body.faces().find(|(kk, _)| *kk == reg[i]).unwrap().1;
            for l in core::iter::once(g.outer).chain(g.rings.iter().copied()) {
                let (_, hes) = topo::props::loop_edges(body, l).unwrap();
                for h in hes {
                    let o = body.face_of_half_edge(body.mate(h).unwrap()).unwrap();
                    let of = body.faces().find(|(kk, _)| *kk == o).unwrap().1;
                    if of.surface == f.surface && !reg.contains(&o) {
                        reg.push(o);
                    }
                }
            }
            i += 1;
        }
        seen.extend(reg.iter().copied());
        out.push(reg);
    }
    out
}

/// Whether point `p` lies on the circle arc of `e` (sampled oracle).
fn on_arc(e: &geom_brep::props::LoopEdge<f64>, p: Point3<f64>) -> bool {
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = e.carrier
    else {
        return false;
    };
    let a = axis.normalize();
    let u = u_ref.normalize();
    let v = a.cross(u);
    let q = p - center;
    if q.dot(a).abs() > 1e-7 || ((q - a * q.dot(a)).norm() - radius).abs() > 1e-7 {
        return false;
    }
    let tau = 2.0 * std::f64::consts::PI;
    let mut t = q.dot(v).atan2(q.dot(u));
    while t < e.t0 - 1e-9 {
        t += tau;
    }
    while t > e.t0 + tau {
        t -= tau;
    }
    t <= e.t1 + 1e-9 || (t - tau) >= e.t0 - 1e-9 && (t - tau) <= e.t1 + 1e-9
}

/// Number of chart poles on the region's boundary, and the boundary's
/// edge count (0 = no boundary).
fn boundary_poles(body: &Body<f64>, reg: &[FaceKey]) -> (usize, usize) {
    let f0 = body.faces().find(|(k, _)| *k == reg[0]).unwrap().1;
    let Some(&Surface::Sphere {
        center,
        radius,
        axis,
        ..
    }) = body.get_surface(f0.surface)
    else {
        unreachable!()
    };
    let up = axis.normalize();
    let mut bnd = Vec::new();
    for &m in reg {
        let g = body.faces().find(|(kk, _)| *kk == m).unwrap().1;
        for l in core::iter::once(g.outer).chain(g.rings.iter().copied()) {
            let (edges, hes) = topo::props::loop_edges(body, l).unwrap();
            for (e, h) in edges.into_iter().zip(hes) {
                let o = body.face_of_half_edge(body.mate(h).unwrap()).unwrap();
                if !reg.contains(&o) {
                    bnd.push(e);
                }
            }
        }
    }
    let n = [up, -up]
        .iter()
        .filter(|&&d| bnd.iter().any(|e| on_arc(e, center + d * radius)))
        .count();
    (n, bnd.len())
}

#[derive(Default, Debug)]
struct Stats {
    bodies: usize,
    unflipped_refused_other: usize,
    flips: usize,
    named: usize,
    refused_other: usize,
    pass_residual_both_poles: usize,
    pass_lone_face: usize,
    pass_no_boundary: usize,
    regions_with_unenc: usize,
    region_sizes: Vec<usize>,
}

fn probe(label: &str, body: &Body<f64>, st: &mut Stats, bad: &mut Vec<String>) {
    st.bodies += 1;
    let t = tol();
    match topo::validate_geometric(body, t) {
        Ok(()) => {}
        Err(errs) => {
            let regress: Vec<_> = errs
                .iter()
                .filter(|e| matches!(e, ValidationError::CurvedSenseInverted { face } if own_unencoded(body, *face)))
                .collect();
            if !regress.is_empty() {
                bad.push(format!(
                    "{label}: UNFLIPPED body refused by the region read: {regress:?}"
                ));
            } else {
                st.unflipped_refused_other += 1;
            }
            return;
        }
    }
    for reg in regions(body) {
        let unenc: Vec<FaceKey> = reg
            .iter()
            .copied()
            .filter(|&k| own_unencoded(body, k))
            .collect();
        if unenc.is_empty() {
            continue;
        }
        st.regions_with_unenc += 1;
        st.region_sizes.push(reg.len());
        let (poles, nb) = boundary_poles(body, &reg);
        let n = reg.len();
        let mut sets: Vec<Vec<FaceKey>> = Vec::new();
        if n <= 4 {
            for mask in 1..(1u32 << n) {
                sets.push(
                    (0..n)
                        .filter(|i| mask & (1 << i) != 0)
                        .map(|i| reg[i])
                        .collect(),
                );
            }
        } else {
            for &k in &reg {
                sets.push(vec![k]);
            }
            sets.push(reg.clone());
        }
        for set in sets {
            st.flips += 1;
            let fb = set.iter().fold(body.clone(), |b, &k| {
                b.flipped_face_sense_for_tests(k).unwrap()
            });
            match topo::validate_geometric(&fb, t) {
                Ok(()) => {
                    let whole = set.len() == n;
                    if nb == 0 {
                        st.pass_no_boundary += 1;
                        bad.push(format!(
                            "{label}: flip {set:?} of a boundaryless region PASSES"
                        ));
                    } else if whole && n == 1 && poles == 2 {
                        st.pass_lone_face += 1;
                    } else if whole && poles == 2 {
                        st.pass_residual_both_poles += 1;
                        bad.push(format!(
                            "{label}: RESIDUAL flip of whole region {set:?} (n={n}, both poles on boundary) passes, measures {:?} vs {:?}",
                            topo::mass_properties(&fb, t).map(|m| m.volume),
                            topo::mass_properties(body, t).map(|m| m.volume)
                        ));
                    } else {
                        bad.push(format!(
                            "{label}: SILENT flip {set:?} of region n={n} poles_on_boundary={poles} measures {:?} vs {:?}",
                            topo::mass_properties(&fb, t).map(|m| m.volume),
                            topo::mass_properties(body, t).map(|m| m.volume)
                        ));
                    }
                }
                Err(errs) => {
                    let mut all = true;
                    for &f in &set {
                        if own_unencoded(body, f) && poles < 2 && nb > 0 {
                            let named = errs.iter().any(|e| matches!(e, ValidationError::CurvedSenseInverted { face } if *face == f));
                            all &= named;
                            if !named {
                                bad.push(format!(
                                    "{label}: flip {set:?} refused but {f:?} not named: {errs:?}"
                                ));
                            }
                        }
                    }
                    if errs
                        .iter()
                        .any(|e| matches!(e, ValidationError::CurvedSenseInverted { .. }))
                        && all
                    {
                        st.named += 1;
                    } else {
                        st.refused_other += 1;
                        eprintln!(
                            "OTHER {label}: flip {set:?} n={n} poles={poles} nb={nb}: {errs:?}"
                        );
                    }
                }
            }
        }
    }
}

fn env(name: &str, d: u64) -> u64 {
    std::env::var(name).ok().map_or(d, |s| {
        if let Some(h) = s.strip_prefix("0x") {
            u64::from_str_radix(h, 16).unwrap()
        } else {
            s.parse().unwrap()
        }
    })
}

fn body_of<E>(r: Result<topo::boolean::BooleanResult<f64>, E>) -> Option<Body<f64>> {
    r.ok().and_then(|r| r.body().map(|b| Body::clone(&b.body)))
}

#[test]
fn review_4746_flip_regions() {
    let t = tol();
    let mut g = Rng(env("R4746_SEED", 0xFEED_FACE));
    let n = env("R4746_N", 25) as usize;
    let fams = std::env::var("R4746_FAM").unwrap_or_else(|_| "balls,slab,brick,three".into());
    let mut st = Stats::default();
    let mut bad = Vec::new();
    if fams.contains("balls") {
        for i in 0..n {
            let rb = g.range(0.5, 1.2);
            let d = g.range(0.4, 1.0 + rb - 0.2);
            let off = g.dir() * d;
            let a = finished(
                "a",
                ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), g.dir(), t),
                t,
            );
            let b = finished("b", ball_poled(rb, off, g.dir(), t), t);
            for (name, r) in [
                ("union", topo::boolean::union(&a, &b, t)),
                ("inter", topo::boolean::intersect(&a, &b, t)),
                ("a-b", topo::boolean::subtract(&a, &b, t)),
                ("b-a", topo::boolean::subtract(&b, &a, t)),
            ] {
                if let Some(body) = body_of(r) {
                    probe(&format!("balls#{i} {name}"), &body, &mut st, &mut bad);
                }
            }
        }
    }
    if fams.contains("slab") {
        for i in 0..n {
            let r = g.range(0.3, 0.9);
            let dip = g.range(0.05, 0.9 * r);
            let c = Vec3::new(g.range(1.2, 2.8), g.range(1.2, 2.8), 1.0 + r - dip);
            let slab = finished("slab", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), t), t);
            let ball = finished(
                "ball",
                ball_poled(
                    r,
                    c,
                    Vec3::new(0.0, 0.0, if i % 2 == 0 { 1.0 } else { -1.0 }),
                    t,
                ),
                t,
            );
            for (name, res) in [
                ("slab-ball", topo::boolean::subtract(&slab, &ball, t)),
                ("slab+ball", topo::boolean::union(&slab, &ball, t)),
                ("slab^ball", topo::boolean::intersect(&slab, &ball, t)),
                ("ball-slab", topo::boolean::subtract(&ball, &slab, t)),
            ] {
                if let Some(body) = body_of(res) {
                    probe(&format!("slab#{i} {name}"), &body, &mut st, &mut bad);
                }
            }
        }
    }
    if fams.contains("brick") {
        // A brick whose face plane holds the ball's centre or passes near
        // it, the ball's pole along the brick's face normal or in its plane.
        for i in 0..n {
            let r = 1.0;
            let off = g.range(-0.6, 0.6);
            let polar = match i % 3 {
                0 => Vec3::new(1.0, 0.0, 0.0),
                1 => Vec3::new(0.0, 1.0, 0.0),
                _ => Vec3::new(0.0, g.range(-1.0, 1.0), g.range(-1.0, 1.0)).normalize(),
            };
            let ball = finished("ball", ball_poled(r, Vec3::new(0.0, 0.0, 0.0), polar, t), t);
            let block = finished("block", brick((-3.0, off), (-2.0, 2.0), (-2.0, 2.0), t), t);
            for (name, res) in [
                ("ball+brick", topo::boolean::union(&ball, &block, t)),
                ("ball-brick", topo::boolean::subtract(&ball, &block, t)),
                ("brick-ball", topo::boolean::subtract(&block, &ball, t)),
                ("ball^brick", topo::boolean::intersect(&ball, &block, t)),
            ] {
                if let Some(body) = body_of(res) {
                    probe(
                        &format!("brick#{i} off={off:.3} {name}"),
                        &body,
                        &mut st,
                        &mut bad,
                    );
                }
            }
            // A slot: two planes both through the polar axis region.
            let w = g.range(0.1, 0.7);
            let slot = finished("slot", brick((-w, w), (-2.0, 2.0), (-2.0, 2.0), t), t);
            for (name, res) in [
                ("ball-slot", topo::boolean::subtract(&ball, &slot, t)),
                ("ball^slot", topo::boolean::intersect(&ball, &slot, t)),
            ] {
                if let Some(body) = body_of(res) {
                    probe(
                        &format!("slot#{i} w={w:.3} {name}"),
                        &body,
                        &mut st,
                        &mut bad,
                    );
                }
            }
        }
    }
    if fams.contains("three") {
        for i in 0..n {
            let a = finished(
                "a",
                ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), g.dir(), t),
                t,
            );
            let rb = g.range(0.5, 1.0);
            let b = finished(
                "b",
                ball_poled(rb, g.dir() * g.range(0.5, 1.3), g.dir(), t),
                t,
            );
            let rc = g.range(0.4, 0.9);
            let c = finished(
                "c",
                ball_poled(rc, g.dir() * g.range(0.5, 1.3), g.dir(), t),
                t,
            );
            let Ok(ab) = topo::boolean::union(&a, &b, t) else {
                continue;
            };
            let Some(ab) = ab.body().map(|b| b.body.clone()) else {
                continue;
            };
            for (name, res) in [
                ("ab+c", topo::boolean::union(&ab, &c, t)),
                ("ab-c", topo::boolean::subtract(&ab, &c, t)),
            ] {
                if let Some(body) = body_of(res) {
                    probe(&format!("three#{i} {name}"), &body, &mut st, &mut bad);
                }
            }
        }
    }
    let _ = Affine3::<f64>::translation(Vec3::new(0.0, 0.0, 0.0));
    let mut h = std::collections::BTreeMap::new();
    for &z in &st.region_sizes {
        *h.entry(z).or_insert(0) += 1;
    }
    st.region_sizes.clear();
    eprintln!("STATS {st:?} sizes {h:?}");
    let fails: Vec<_> = bad.iter().filter(|b| !b.contains("RESIDUAL")).collect();
    for b in &bad {
        eprintln!("  {b}");
    }
    assert!(fails.is_empty(), "{} failures", fails.len());
}

// Copied from the PR's seam_siblings_flipped_together.rs.
/// A face of one sphere split from pole to pole along the meridian
/// through the middle of its lune, by `mef`: the region it belongs to
/// then holds three faces across three seams.
fn split_along_a_meridian(
    body: &mut Body<f64>,
    face: FaceKey,
    clear_of: (Vec3<f64>, f64),
    tol: Tol,
) {
    let f = body.faces().find(|(k, _)| *k == face).unwrap().1.clone();
    let Some(&Surface::Sphere {
        center,
        radius,
        axis,
        ..
    }) = body.get_surface(f.surface)
    else {
        panic!("not a sphere face");
    };
    let pole = axis.normalize();
    let (edges, hes) = topo::props::loop_edges(body, f.outer).unwrap();
    let points: Vec<_> = body.vertex_points().collect();
    let at = |h: topo::HalfEdgeKey| {
        let v = body.get_half_edge(h).unwrap().start;
        points.iter().find(|(k, _)| *k == v).unwrap().1
    };
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let north = center + pole * radius;
    let south = center - pole * radius;
    let he1 = *hes
        .iter()
        .find(|&&h| near(at(h), north))
        .expect("a pole vertex");
    let he2 = *hes
        .iter()
        .find(|&&h| near(at(h), south))
        .expect("a pole vertex");
    let mut seam = None;
    let mut inside = None;
    for e in &edges {
        let Curve3::Circle {
            center: c,
            axis: n,
            radius: r,
            u_ref,
        } = e.carrier
        else {
            continue;
        };
        if near(c, center) && (r - radius).abs() < 1e-9 && n.dot(pole).abs() < 1e-9 {
            seam = Some(n.normalize());
        } else {
            let mid = 0.5 * (e.t0 + e.t1);
            inside = Some(c + u_ref * (r * mid.cos()) + n.cross(u_ref) * (r * mid.sin()));
        }
    }
    let n = seam.expect("a seam meridian on the loop");
    let w = if n.dot(inside.expect("a section arc on the loop") - center) > 0.0 {
        n
    } else {
        -n
    };
    // The lune's interior directions, from one seam half to the other;
    // the first meridian clear of the cut.
    let half = n.cross(pole);
    let meridian = |phi: f64| w * phi.cos() + half * phi.sin();
    let clear = |dir: Vec3<f64>| {
        (1..32).all(|i| {
            let t = PI * f64::from(i) / 32.0;
            let q = center + (pole * t.cos() + dir * t.sin()) * radius;
            (q - Point3::new(0.0, 0.0, 0.0) - clear_of.0).norm() > clear_of.1 + 0.05
        })
    };
    let dir = (-8..=8)
        .map(|k| meridian(f64::from(k) * PI / 20.0))
        .find(|&d| clear(d))
        .expect("a meridian across the lune clear of the cut");
    let circle = Curve3::Circle {
        center,
        axis: pole.cross(dir),
        radius,
        u_ref: pole,
    };
    body.mef(
        MefSite::Chords { he1, he2 },
        EdgeCurveSpec::arc_of_circle(circle, 0.0, PI)
            .unwrap()
            .at_rest_in_chart(f.surface, false),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
}

/// Split every region member that is Unencoded along a meridian (as the
/// PR's helper does), up to `times` times, and split each sphere edge of
/// the body at a random parameter; probe every stage.
fn split_family(
    label: &str,
    body: &Body<f64>,
    others: &[(Vec3<f64>, f64)],
    g: &mut Rng,
    st: &mut Stats,
    bad: &mut Vec<String>,
) {
    let t = tol();
    let mut cur = body.clone();
    for round in 0..3 {
        let Some(face) = cur
            .faces()
            .map(|(k, _)| k)
            .find(|&k| own_unencoded(&cur, k))
        else {
            return;
        };
        let mut next = cur.clone();
        let mut done = false;
        for &o in others {
            let mut tryb = next.clone();
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                split_along_a_meridian(&mut tryb, face, o, t);
                tryb
            }));
            if let Ok(b) = r {
                if topo::validate_geometric(&b, t).is_ok() {
                    next = b;
                    done = true;
                    break;
                }
            }
        }
        if !done {
            return;
        }
        probe(&format!("{label} mef{round}"), &next, st, bad);
        // split_edge on every edge of every sphere face, at random t.
        let mut se = next.clone();
        let edges: Vec<(topo::EdgeKey, f64, f64)> = se
            .faces()
            .filter(|(_, f)| matches!(se.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .flat_map(|(_, f)| {
                let (edges, hes) = topo::props::loop_edges(&se, f.outer).unwrap();
                edges
                    .into_iter()
                    .zip(hes)
                    .map(|(e, h)| (se.get_half_edge(h).unwrap().edge, e.t0, e.t1))
                    .collect::<Vec<_>>()
            })
            .collect();
        let mut seen = Vec::new();
        for (ek, t0, t1) in edges {
            if seen.contains(&ek) {
                continue;
            }
            seen.push(ek);
            let at = t0 + (t1 - t0) * g.range(0.1, 0.9);
            let _ = se.split_edge(ek, at, t);
        }
        if topo::validate_geometric(&se, t).is_ok() {
            probe(&format!("{label} mef{round}+split_edge"), &se, st, bad);
        } else {
            eprintln!(
                concat!(
                    "NOTE {} mef{}: split_edge left joinable vertices (harness artefact): {:?}"
                ),
                label,
                round,
                topo::validate_geometric(&se, t).err().map(|e| e.len())
            );
            let _ = (format!(
                "{label} mef{round}: split_edge made the body invalid: {:?}",
                topo::validate_geometric(&se, t)
            ));
        }
        cur = next;
    }
}

#[test]
fn review_4746_split_regions() {
    let t = tol();
    let mut g = Rng(env("R4746_SEED", 0xFEED_FACE));
    let n = env("R4746_N", 25) as usize;
    let mut st = Stats::default();
    let mut bad = Vec::new();
    for i in 0..n {
        let rb = g.range(0.5, 1.2);
        let d = g.range(0.4, 1.0 + rb - 0.2);
        let off = g.dir() * d;
        let a = finished(
            "a",
            ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), g.dir(), t),
            t,
        );
        let b = finished("b", ball_poled(rb, off, g.dir(), t), t);
        let others = [(off, rb), (Vec3::new(0.0, 0.0, 0.0), 1.0)];
        for (name, r) in [
            ("union", topo::boolean::union(&a, &b, t)),
            ("inter", topo::boolean::intersect(&a, &b, t)),
            ("a-b", topo::boolean::subtract(&a, &b, t)),
        ] {
            if let Some(body) = body_of(r) {
                split_family(
                    &format!("split#{i} {name}"),
                    &body,
                    &others,
                    &mut g,
                    &mut st,
                    &mut bad,
                );
            }
        }
    }
    let mut h = std::collections::BTreeMap::new();
    for &z in &st.region_sizes {
        *h.entry(z).or_insert(0) += 1;
    }
    st.region_sizes.clear();
    eprintln!("STATS {st:?} sizes {h:?}");
    for b in &bad {
        eprintln!("  {}", &b[..b.len().min(500)]);
    }
    let fails: Vec<_> = bad.iter().filter(|b| !b.contains("RESIDUAL")).collect();
    assert!(fails.is_empty(), "{} failures", fails.len());
}

/// The disclosed residual, built: the lone lune of ball ∪ brick (both
/// poles on its loop, its region boundary the great circle x = 0) split
/// by `mef` along the meridian through (1, 0, 0). The region now holds
/// two faces and its boundary carries both poles: flipping both.
#[test]
fn review_4746_residual_witness() {
    let t = tol();
    let ball = finished(
        "ball",
        ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), t),
        t,
    );
    let block = finished("block", brick((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), t), t);
    let mut body = Body::clone(
        &topo::boolean::union(&ball, &block, t)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let lune: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect();
    assert_eq!(lune.len(), 1);
    let f = body.faces().find(|(k, _)| *k == lune[0]).unwrap().1.clone();
    let (_, hes) = topo::props::loop_edges(&body, f.outer).unwrap();
    let points: Vec<_> = body.vertex_points().collect();
    let at = |h: topo::HalfEdgeKey| {
        let v = body.get_half_edge(h).unwrap().start;
        points.iter().find(|(k, _)| *k == v).unwrap().1
    };
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let he1 = *hes
        .iter()
        .find(|&&h| near(at(h), Point3::new(0.0, 1.0, 0.0)))
        .unwrap();
    let he2 = *hes
        .iter()
        .find(|&&h| near(at(h), Point3::new(0.0, -1.0, 0.0)))
        .unwrap();
    let pole = Vec3::new(0.0, 1.0, 0.0);
    let circle = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: pole.cross(Vec3::new(1.0, 0.0, 0.0)),
        radius: 1.0,
        u_ref: pole,
    };
    body.mef(
        MefSite::Chords { he1, he2 },
        EdgeCurveSpec::arc_of_circle(circle, 0.0, PI)
            .unwrap()
            .at_rest_in_chart(f.surface, false),
        FaceSurface::Inherit,
        t,
    )
    .unwrap();
    topo::validate_geometric(&body, t).expect("the split boss is valid");
    let v0 = topo::mass_properties(&body, t).unwrap().volume;
    eprintln!("split boss volume {v0} (want {})", 48.0 + 2.0 * PI / 3.0);
    let faces: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect();
    assert_eq!(faces.len(), 2);
    for k in &faces {
        eprintln!("face {k:?} own unencoded: {}", own_unencoded(&body, *k));
    }
    for set in [vec![faces[0]], vec![faces[1]], faces.clone()] {
        let fb = set.iter().fold(body.clone(), |b, &k| {
            b.flipped_face_sense_for_tests(k).unwrap()
        });
        eprintln!(
            "flip {set:?}: tier3 {:?} volume {:?}",
            topo::validate_geometric(&fb, t),
            topo::mass_properties(&fb, t).map(|m| m.volume)
        );
    }
}

/// Claim 7: the flipped lone lune is the dimpled brick that brick − ball
/// builds directly. Compare the two bodies' sphere faces and volumes.
#[test]
fn review_4746_lune_flip_is_the_dimple() {
    let t = tol();
    let ball = finished(
        "ball",
        ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), t),
        t,
    );
    let block = finished("block", brick((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), t), t);
    let boss = Body::clone(
        &topo::boolean::union(&ball, &block, t)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let dimple = Body::clone(
        &topo::boolean::subtract(&block, &ball, t)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let describe = |b: &Body<f64>| {
        b.faces()
            .filter(|(_, f)| matches!(b.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .map(|(_, f)| {
                let (_, hes) = topo::props::loop_edges(b, f.outer).unwrap();
                let pts: Vec<_> = b.vertex_points().collect();
                let mut ends: Vec<String> = hes
                    .iter()
                    .map(|&h| {
                        let v = b.get_half_edge(h).unwrap().start;
                        let p = pts.iter().find(|(k, _)| *k == v).unwrap().1;
                        format!("({:.3},{:.3},{:.3})", p.x, p.y, p.z)
                    })
                    .collect();
                ends.sort();
                (f.sense, f.rings.len(), ends)
            })
            .collect::<Vec<_>>()
    };
    let lune = boss
        .faces()
        .find(|(_, f)| matches!(boss.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .unwrap()
        .0;
    let flipped = boss.flipped_face_sense_for_tests(lune).unwrap();
    eprintln!("flipped boss sphere faces: {:?}", describe(&flipped));
    eprintln!("brick - ball sphere faces: {:?}", describe(&dimple));
    eprintln!(
        "volumes: flipped {:?} direct {:?} closed {}",
        topo::mass_properties(&flipped, t).map(|m| m.volume),
        topo::mass_properties(&dimple, t).map(|m| m.volume),
        48.0 - 2.0 * PI / 3.0
    );
    eprintln!(
        "tier3 flipped {:?} direct {:?}",
        topo::validate_geometric(&flipped, t),
        topo::validate_geometric(&dimple, t)
    );
}

/// Near-great boundaries: a brick face plane `x = off` beside the ball's
/// polar axis (pole along y, in the plane), off in the band and just past it.
#[test]
fn review_4746_near_polar_planes() {
    let t = tol();
    let mut st = Stats::default();
    let mut bad = Vec::new();
    for &off in &[
        0.0, 1e-13, -1e-13, 1e-11, -1e-11, 1e-9, -1e-9, 3e-9, 1e-8, -1e-8, 1e-7, 1e-6, -1e-6, 1e-4,
    ] {
        for pole in [Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.6, 0.8)] {
            let ball = finished(
                "ball",
                ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), pole, t),
                t,
            );
            let block = finished("block", brick((-3.0, off), (-2.0, 2.0), (-2.0, 2.0), t), t);
            let slot = finished("slot", brick((-0.3, off), (-2.0, 2.0), (-2.0, 2.0), t), t);
            for (name, res) in [
                ("ball+brick", topo::boolean::union(&ball, &block, t)),
                ("ball-brick", topo::boolean::subtract(&ball, &block, t)),
                ("brick-ball", topo::boolean::subtract(&block, &ball, t)),
                ("ball^brick", topo::boolean::intersect(&ball, &block, t)),
                ("ball-slot", topo::boolean::subtract(&ball, &slot, t)),
                ("ball^slot", topo::boolean::intersect(&ball, &slot, t)),
            ] {
                match body_of(res) {
                    Some(body) => probe(
                        &format!("near off={off:e} pole={pole:?} {name}"),
                        &body,
                        &mut st,
                        &mut bad,
                    ),
                    None => eprintln!("near off={off:e} {name}: no body"),
                }
            }
        }
    }
    let mut h = std::collections::BTreeMap::new();
    for &z in &st.region_sizes {
        *h.entry(z).or_insert(0) += 1;
    }
    st.region_sizes.clear();
    eprintln!("STATS {st:?} sizes {h:?}");
    for b in &bad {
        eprintln!("  {}", &b[..b.len().min(500)]);
    }
    let fails: Vec<_> = bad.iter().filter(|b| !b.contains("RESIDUAL")).collect();
    assert!(fails.is_empty(), "{} failures", fails.len());
}

/// Claim 8b: PAIRS[0] (#1 union) of the PR's row, both seam siblings
/// flipped: tier 3 refuses, `mass_properties` alone measures.
#[test]
fn review_4746_case1_mass_properties_without_tier3() {
    let t = tol();
    let v = |x: f64, y: f64, z: f64| Vec3::new(x, y, z);
    let a = finished(
        "a",
        ball_poled(
            1.0,
            v(0.0, 0.0, 0.0),
            v(
                -0.6847601426471676,
                0.7225669750674681,
                -0.09487103659009685,
            ),
            t,
        ),
        t,
    );
    let off = v(
        0.34687804548123913,
        -0.11616117805350519,
        1.3291277707606621,
    );
    let b = finished(
        "b",
        ball_poled(
            0.9356108706211743,
            off,
            v(-0.6379874658023172, -0.5755013108457552, 0.5116348646192466),
            t,
        ),
        t,
    );
    let body = Body::clone(
        &topo::boolean::union(&a, &b, t)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    for reg in regions(&body) {
        let unenc: Vec<_> = reg
            .iter()
            .copied()
            .filter(|&k| own_unencoded(&body, k))
            .collect();
        if unenc.len() < 2 {
            continue;
        }
        let fb = reg.iter().fold(body.clone(), |b, &k| {
            b.flipped_face_sense_for_tests(k).unwrap()
        });
        eprintln!(
            "region {reg:?}: unflipped {:?}; flipped mass_properties {:?}; tier3 {:?}",
            topo::mass_properties(&body, t).map(|m| m.volume),
            topo::mass_properties(&fb, t).map(|m| m.volume),
            topo::validate_geometric(&fb, t)
        );
    }
}

/// Dump of one near-polar pose (run at CAD_TOLERANCE_EPS=1e-12):
/// brick − ball, face plane `x = R4746_OFF`, pole `(0, 0.6, 0.8)`.
#[test]
fn review_4746_near_polar_dump() {
    let t = tol();
    let off = std::env::var("R4746_OFF")
        .ok()
        .map_or(1e-8, |s| s.parse().unwrap());
    let pole = Vec3::new(0.0, 0.6, 0.8);
    let ball = finished(
        "ball",
        ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), pole, t),
        t,
    );
    let block = finished("block", brick((-3.0, off), (-2.0, 2.0), (-2.0, 2.0), t), t);
    let Some(body) = body_of(topo::boolean::subtract(&block, &ball, t)) else {
        eprintln!("no body");
        return;
    };
    let pts: Vec<_> = body.vertex_points().collect();
    let north = Point3::new(0.0, 0.6, 0.8);
    let south = Point3::new(0.0, -0.6, -0.8);
    for (k, f) in body.faces() {
        let Some(Surface::Sphere { axis, .. }) = body.get_surface(f.surface) else {
            continue;
        };
        let (edges, hes) = topo::props::loop_edges(&body, f.outer).unwrap();
        eprintln!(
            "face {k:?} surface {:?} axis {axis:?} sense {} rings {} own_unencoded {}",
            f.surface,
            f.sense,
            f.rings.len(),
            own_unencoded(&body, k)
        );
        for (e, h) in edges.iter().zip(&hes) {
            let v = body.get_half_edge(*h).unwrap().start;
            let p = pts.iter().find(|(kk, _)| *kk == v).unwrap().1;
            let o = body.face_of_half_edge(body.mate(*h).unwrap()).unwrap();
            let carrier = match e.carrier {
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => format!(
                    "circle c=({:.2e},{:.2e},{:.2e}) n=({:.3},{:.3},{:.3}) r={radius:.6}",
                    center.x, center.y, center.z, axis.x, axis.y, axis.z
                ),
                _ => "other".into(),
            };
            eprintln!(
                "   start ({:.3e},{:.3e},{:.3e}) |p-N|={:.2e} |p-S|={:.2e} mate face {o:?} {carrier} t=[{:.4},{:.4}] fwd={} onN={} onS={}",
                p.x,
                p.y,
                p.z,
                (p - north).norm(),
                (p - south).norm(),
                e.t0,
                e.t1,
                e.forward,
                on_arc(e, north),
                on_arc(e, south)
            );
        }
    }
    for reg in regions(&body) {
        eprintln!(
            "region {reg:?} poles-on-boundary {:?}",
            boundary_poles(&body, &reg)
        );
        let fb = reg.iter().fold(body.clone(), |b, &k| {
            b.flipped_face_sense_for_tests(k).unwrap()
        });
        eprintln!("  flip all: tier3 {:?}", topo::validate_geometric(&fb, t));
    }
}
