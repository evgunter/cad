//! **The differential batteries' shared pieces**: the polygon oracles
//! their closed-form volumes are derived from, the one `outcome` line a
//! battery prints per pose, the reflex-corner pose, and the near-tangent
//! corner poses ([`NtPrism`]) the census probe and its pin share. Derived without
//! the kernel, as `super::oracles` is, and printed in one format, so
//! two batteries' lines can be diffed against each other.
//!
//! **Deliberately not absorbed**, and the whole of it:
//! `super::oracles`' closed forms, which are swept-solid volumes with
//! nothing polygonal to share; the per-battery operands of
//! `join1_r1_probes.rs` (prisms, rods, revolves), each one battery's;
//! and `join_reflex_wedge_probes.rs`' fan-wedge `Pose`, whose operands
//! are both parametrized by wedge angle, which [`ReflexPose`]'s one
//! fixed corner is not.

use geom_core::{Point3, Tol};
use sweep::test_support::finished;
use topo::test_support::{FaceGeometry, describe_as_intersections, prism_ops, prism_z};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

/// The signed area of a simple polygon (positive counter-clockwise).
pub fn area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    (0..n)
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            a.0 * b.1 - a.1 * b.0
        })
        .sum::<f64>()
        / 2.0
}

/// `p` wound counter-clockwise.
pub fn ccw(mut p: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    if area(&p) < 0.0 {
        p.reverse();
    }
    p
}

/// `∫ x dA`, `∫ y dA` over a simple polygon (signed by its winding).
pub fn moments(p: &[(f64, f64)]) -> (f64, f64) {
    let n = p.len();
    let (mut mx, mut my) = (0.0, 0.0);
    for i in 0..n {
        let (a, b) = (p[i], p[(i + 1) % n]);
        let c = a.0 * b.1 - b.0 * a.1;
        mx += (a.0 + b.0) * c;
        my += (a.1 + b.1) * c;
    }
    (mx / 6.0, my / 6.0)
}

/// Sutherland–Hodgman of `poly` against the axis rectangle `x × y`.
pub fn clip_rect(poly: &[(f64, f64)], x: (f64, f64), y: (f64, f64)) -> Vec<(f64, f64)> {
    let rect = [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)];
    clip_convex(poly, &rect)
}

/// Sutherland–Hodgman of `poly` against the convex CCW polygon `clipper`.
pub fn clip_convex(poly: &[(f64, f64)], clipper: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = poly.to_vec();
    let m = clipper.len();
    for k in 0..m {
        let (a, b) = (clipper[k], clipper[(k + 1) % m]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let inp = std::mem::take(&mut out);
        let n = inp.len();
        for i in 0..n {
            let (s, e) = (inp[i], inp[(i + 1) % n]);
            let (ds, de) = (side(s), side(e));
            let isect = || {
                let t = ds / (ds - de);
                (s.0 + t * (e.0 - s.0), s.1 + t * (e.1 - s.1))
            };
            match (ds >= 0.0, de >= 0.0) {
                (true, true) => out.push(e),
                (true, false) => out.push(isect()),
                (false, true) => {
                    out.push(isect());
                    out.push(e);
                }
                (false, false) => {}
            }
        }
    }
    out
}

/// The signed area of the triangle `origin, a, b` and its first
/// moments `(area, ∫x, ∫y)`.
fn chord_triangle(a: (f64, f64), b: (f64, f64)) -> (f64, f64, f64) {
    let area = (a.0 * b.1 - a.1 * b.0) / 2.0;
    (area, area * (a.0 + b.0) / 3.0, area * (a.1 + b.1) / 3.0)
}

/// The signed area of the circular sector of radius `r` from `a`'s
/// direction to `b`'s, the turn read in `(−π, π]`, and its first
/// moments `(area, ∫x, ∫y)`.
fn sector(r: f64, a: (f64, f64), b: (f64, f64)) -> (f64, f64, f64) {
    let turn = (a.0 * b.1 - a.1 * b.0).atan2(a.0 * b.0 + a.1 * b.1);
    let from = a.1.atan2(a.0);
    let to = from + turn;
    let r3 = r * r * r / 3.0;
    (
        r * r * turn / 2.0,
        r3 * (to.sin() - from.sin()),
        r3 * (from.cos() - to.cos()),
    )
}

/// **The area of the disc of radius `r` about the origin against the
/// polygon `poly`**, closed form ([`disc_clip_moments`]).
pub fn disc_clip_area(r: f64, poly: &[(f64, f64)]) -> f64 {
    disc_clip_moments(r, poly).0
}

/// **The disc of radius `r` about the origin against the polygon
/// `poly`: its area and first moments `(area, ∫x, ∫y)`**, closed form.
/// Each edge contributes what its own chord or arc sweeps about the
/// origin — a chord triangle where the edge is inside the disc, a
/// sector where it is outside, and the split of the two at the edge's
/// crossings, which are the roots of a quadratic. The sum over a closed
/// polygon is the enclosed region's, signed by the winding.
pub fn disc_clip_moments(r: f64, poly: &[(f64, f64)]) -> (f64, f64, f64) {
    let n = poly.len();
    let mut total = (0.0, 0.0, 0.0);
    let mut add = |(a, x, y): (f64, f64, f64)| {
        total = (total.0 + a, total.1 + x, total.2 + y);
    };
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let d = (b.0 - a.0, b.1 - a.1);
        let ra = a.0.hypot(a.1);
        let (qa, qb, qc) = (
            d.0 * d.0 + d.1 * d.1,
            2.0 * (a.0 * d.0 + a.1 * d.1),
            a.0 * a.0 + a.1 * a.1 - r * r,
        );
        let disc = qb * qb - 4.0 * qa * qc;
        // The crossing parameters inside the edge, in order.
        let hits: Vec<f64> = if qa <= 0.0 || disc <= 0.0 {
            Vec::new()
        } else {
            let root = disc.sqrt();
            [(-qb - root) / (2.0 * qa), (-qb + root) / (2.0 * qa)]
                .into_iter()
                .filter(|t| *t > 0.0 && *t < 1.0)
                .collect()
        };
        let at = |t: f64| (a.0 + t * d.0, a.1 + t * d.1);
        let piece = |p: (f64, f64), q: (f64, f64), inside: bool| {
            if inside {
                chord_triangle(p, q)
            } else {
                sector(r, p, q)
            }
        };
        let mut from = a;
        let mut inside = ra <= r;
        for t in hits {
            let p = at(t);
            add(piece(from, p, inside));
            from = p;
            inside = !inside;
        }
        add(piece(from, b, inside));
    }
    total
}

/// **One pose's line**: the refusal, or the body's tiers 2 and 3′, the
/// at-rest certificate, whether it is a legal operand (it unites with a
/// far brick, `sweep::test_support::assert_legal_operand`'s question)
/// and its volume against `want` ([`judged`]). `SOUND` is all five;
/// anything else that builds is `BAD`.
pub fn outcome(r: Result<BooleanResult<f64>, BooleanError>, want: f64, tol: Tol) -> String {
    judged(r, Some(want), tol).line
}

/// The volume claim a `SOUND` line makes: the closed form lies within
/// this of the body's certified volume enclosure.
pub const VOLUME_CLAIM: f64 = 1e-7;

/// The widest certified volume enclosure (its half-width, `volume_pad`)
/// a `SOUND` line accepts: no wider than [`VOLUME_CLAIM`], so a
/// certificate that loosens cannot pass a volume it does not pin.
pub const VOLUME_PAD_BUDGET: f64 = 1e-7;

/// What [`judged`] read of a pose.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The op refused.
    Refused,
    /// An empty result where the closed form is empty.
    EmptyOk,
    /// An empty result where it is not.
    EmptyWrong,
    /// An empty result, with no closed form.
    EmptyUnjudged,
    /// A body passing every check.
    Sound,
    /// A body passing every check but the volume, with no closed form.
    Unjudged,
    /// A body failing a check.
    Bad,
    /// A body whose volume could not be measured.
    Unmeasured,
}

/// One pose as [`judged`] read it: its line, the volume the result
/// measured (zero for an empty one), and the verdict.
#[derive(Clone, Debug)]
pub struct Judgement {
    /// The printed line.
    pub line: String,
    /// The measured volume.
    pub volume: Option<f64>,
    /// The measured enclosure's half-width (`volume_pad`), where a
    /// volume was measured.
    pub pad: Option<f64>,
    /// The verdict the line prints.
    pub verdict: Verdict,
}

/// [`outcome`] where a pose may have no closed form (`want` `None`),
/// with the volume and the verdict. The volume passes where `want` lies
/// within [`VOLUME_CLAIM`] of the measured enclosure, `v ± volume_pad`,
/// and that enclosure is no wider than [`VOLUME_PAD_BUDGET`]; a nonzero
/// pad is printed. A body passing the other four checks against no
/// closed form reads `UNJUDGED`, never `SOUND`.
pub fn judged(
    r: Result<BooleanResult<f64>, BooleanError>,
    want: Option<f64>,
    tol: Tol,
) -> Judgement {
    judge(r, want, tol, true, VOLUME_PAD_BUDGET)
}

/// [`judged`] against an enclosure no wider than `pad_budget` in
/// place of [`VOLUME_PAD_BUDGET`]: for a body whose certified enclosure
/// is known not to meet the shared budget at every ε, the caller names
/// its own ceiling, and why, at its site.
pub fn judged_within(
    r: Result<BooleanResult<f64>, BooleanError>,
    want: Option<f64>,
    tol: Tol,
    pad_budget: f64,
) -> Judgement {
    judge(r, want, tol, true, pad_budget)
}

/// [`judged`], with `extra` one more condition a `SOUND` line needs and
/// `pad_budget` the widest enclosure it accepts.
fn judge(
    r: Result<BooleanResult<f64>, BooleanError>,
    want: Option<f64>,
    tol: Tol,
    extra: bool,
    pad_budget: f64,
) -> Judgement {
    let want_col = want.map_or("want=none".to_string(), |w| format!("want={w:.9}"));
    let of = |line: String, volume, verdict| Judgement {
        line,
        volume,
        pad: None,
        verdict,
    };
    match r {
        Err(e) => {
            let s = format!("{e:?}");
            let cut: String = s.chars().take(110).collect();
            of(format!("ERR {cut}"), None, Verdict::Refused)
        }
        Ok(r) => match r.body() {
            None => match want {
                None => of("EMPTY unjudged".into(), Some(0.0), Verdict::EmptyUnjudged),
                Some(w) if w.abs() < 1e-9 => of("EMPTY ok".into(), Some(0.0), Verdict::EmptyOk),
                Some(w) => of(
                    format!("EMPTY WRONG want={w}"),
                    Some(0.0),
                    Verdict::EmptyWrong,
                ),
            },
            Some(bb) => {
                let t2 = topo::validate_closed(&bb.body).is_ok();
                let t3 = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol).is_ok();
                let cert = topo::validate_geometric_certificate(&bb.body, tol).is_ok();
                let far = finished(
                    "the far brick",
                    sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol),
                    tol,
                );
                let operand = topo::union(&bb.body, &far, tol).is_ok();
                match topo::mass_properties(
                    &bb.body,
                    tol,
                    topo::test_support::pinned_resolution(tol),
                )
                .map(|m| (m.enclosure.volume, m.enclosure.volume_pad))
                {
                    Ok((v, pad)) => {
                        let verdict = match want {
                            _ if !(t2 && t3 && cert && operand && extra) => Verdict::Bad,
                            _ if pad > pad_budget => Verdict::Bad,
                            None => Verdict::Unjudged,
                            Some(w) if (v - w).abs() <= VOLUME_CLAIM + pad => Verdict::Sound,
                            Some(_) => Verdict::Bad,
                        };
                        let word = match verdict {
                            Verdict::Sound => "SOUND",
                            Verdict::Unjudged => "UNJUDGED",
                            _ => "BAD",
                        };
                        let pad_col = if pad > 0.0 {
                            format!(" pad={pad:.1e}")
                        } else {
                            String::new()
                        };
                        Judgement {
                            pad: Some(pad),
                            ..of(
                                format!(
                                    "OK {word} t2={t2} t3p={t3} cert={cert} operand={operand} \
                                     v={v:.9}{pad_col} {want_col}"
                                ),
                                Some(v),
                                verdict,
                            )
                        }
                    }
                    Err(e) => of(
                        format!(
                            "OK-UNMEASURED t2={t2} t3p={t3} cert={cert} operand={operand} {e:?}"
                        ),
                        None,
                        Verdict::Unmeasured,
                    ),
                }
            }
        },
    }
}

/// The seams a restated scaffold could hide: (edges between two faces
/// of one planar surface key, edges between two faces on one plane,
/// zero-length open edges).
pub fn seams(body: &Body<f64>) -> (usize, usize, usize) {
    let (mut same_key, mut coplanar, mut zero) = (0, 0, 0);
    for (_, e) in body.edges() {
        let ends = (
            body.get_half_edge(e.he_plus).map(|h| h.start),
            body.half_edge_end(e.he_plus),
        );
        if let (Some(a), Some(b)) = ends
            && a != b
        {
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            if (p(a) - p(b)).norm() < 1e-9 {
                zero += 1;
            }
        }
        let (Some(f1), Some(f2)) = (
            body.face_of_half_edge(e.he_plus),
            body.face_of_half_edge(e.he_minus),
        ) else {
            continue;
        };
        if f1 == f2 {
            continue;
        }
        let (k1, k2) = (
            body.get_face(f1).unwrap().surface,
            body.get_face(f2).unwrap().surface,
        );
        let plane = |k| match body.get_surface(k) {
            Some(geom::Surface::Plane { origin, normal, .. }) => Some((*origin, *normal)),
            _ => None,
        };
        if let (Some((o1, n1)), Some((o2, n2))) = (plane(k1), plane(k2)) {
            if k1 == k2 {
                same_key += 1;
            } else if n1.cross(n2).norm() < 1e-9 && (o2 - o1).dot(n1).abs() < 1e-9 {
                coplanar += 1;
            }
        }
    }
    (same_key, coplanar, zero)
}

/// [`judged`], and where the op built, the body's [`seams`] as columns.
/// A face cut in two (`samekey`) or a zero-length edge (`zero`) reads
/// `BAD`; two faces on one plane (`coplanar`) is printed only.
pub fn judged_with_seams(
    r: Result<BooleanResult<f64>, BooleanError>,
    want: Option<f64>,
    tol: Tol,
) -> Judgement {
    let seams = r
        .as_ref()
        .ok()
        .and_then(BooleanResult::body)
        .map(|bb| seams(&bb.body));
    let clean = seams.is_none_or(|(sk, _, z)| sk == 0 && z == 0);
    let mut j = judge(r, want, tol, clean, VOLUME_PAD_BUDGET);
    if let Some((sk, cp, z)) = seams {
        j.line = format!("{} samekey={sk} coplanar={cp} zero={z}", j.line);
    }
    j
}

/// A built body that [`outcome`] reads `SOUND` against `want` and that
/// meshes, the mesh checked closed and consistent.
///
/// # Panics
///
/// On a refusal, an empty result, a mesh refusal or failed check, or any
/// line but `SOUND`.
pub fn assert_sound_and_meshed(
    label: &str,
    r: Result<BooleanResult<f64>, BooleanError>,
    want: f64,
    tol: Tol,
) {
    let Ok(Some(bb)) = r.as_ref().map(BooleanResult::body) else {
        panic!("{label}: wanted a body, got {r:?}");
    };
    let mesh = mesh::tessellate(&bb.body, 5e-3, tol)
        .unwrap_or_else(|e| panic!("{label}: the mesh: {e:?}"));
    mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{label}: the mesh: {e:?}"));
    assert_meshed_geometry(label, &bb.body, &mesh, 5e-3);
    let line = outcome(r, want, tol);
    assert!(line.starts_with("OK SOUND"), "{label}: {line}");
}

/// **Every op in both operand orders** between `a` (volume `va`) and
/// `b` (`vb`), which overlap in `vab`: ∪, ∩ and ∖ run `a`-first then
/// `b`-first, each with its closed-form volume, named `"A ∪ B"`,
/// `"B ∪ A"`, `"A ∩ B"`, `"B ∩ A"`, `"A ∖ B"` and `"B ∖ A"`.
#[allow(clippy::type_complexity)] // (op, result, closed form) per run
pub fn every_op_both_orders(
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    (va, vb, vab): (f64, f64, f64),
    tol: Tol,
) -> Vec<(&'static str, Result<BooleanResult<f64>, BooleanError>, f64)> {
    vec![
        ("A ∪ B", topo::union(a, b, tol), va + vb - vab),
        ("B ∪ A", topo::union(b, a, tol), va + vb - vab),
        ("A ∩ B", topo::intersect(a, b, tol), vab),
        ("B ∩ A", topo::intersect(b, a, tol), vab),
        ("A ∖ B", topo::subtract(a, b, tol), va - vab),
        ("B ∖ A", topo::subtract(b, a, tol), vb - vab),
    ]
}

/// The reflex corner's `a` profile: the 315° corner at the origin, all
/// of `[−2, 2]²` but the wedge `0 ≤ y ≤ x`.
pub const REFLEX_A: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 2.0),
    (-2.0, 2.0),
    (-2.0, -2.0),
    (2.0, -2.0),
    (2.0, 0.0),
];

/// The reflex probe's twelve `b` profiles, by name: a vertex or an
/// edge of each on the corner.
pub const REFLEX_PROFILES: [(&str, [(f64, f64); 4]); 12] = [
    ("sqQ1", [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]),
    ("sqQ2", [(-1.0, 0.0), (0.0, 0.0), (0.0, 1.0), (-1.0, 1.0)]),
    ("sqQ3", [(-1.0, -1.0), (0.0, -1.0), (0.0, 0.0), (-1.0, 0.0)]),
    ("sqQ4", [(0.0, -1.0), (1.0, -1.0), (1.0, 0.0), (0.0, 0.0)]),
    ("dUp", [(0.0, 0.0), (0.5, 0.5), (0.0, 1.0), (-0.5, 0.5)]),
    (
        "dLeft",
        [(0.0, 0.0), (-0.5, 0.5), (-1.0, 0.0), (-0.5, -0.5)],
    ),
    (
        "dDown",
        [(0.0, 0.0), (-0.5, -0.5), (0.0, -1.0), (0.5, -0.5)],
    ),
    ("dRight", [(0.0, 0.0), (0.5, -0.5), (1.0, 0.0), (0.5, 0.5)]),
    ("eBot", [(-0.5, 0.0), (0.5, 0.0), (0.5, 1.0), (-0.5, 1.0)]),
    ("eTop", [(-0.5, -1.0), (0.5, -1.0), (0.5, 0.0), (-0.5, 0.0)]),
    ("eLeft", [(0.0, -0.5), (1.0, -0.5), (1.0, 0.5), (0.0, 0.5)]),
    (
        "eRight",
        [(-1.0, -0.5), (0.0, -0.5), (0.0, 0.5), (-1.0, 0.5)],
    ),
];

/// The reflex probe's four ops, in the order [`ReflexPose::want`]
/// holds their volumes.
pub const REFLEX_OPS: [&str; 4] = ["I", "U", "S_ab", "S_ba"];

/// One reflex pose: `a` the 315° prism `prism_z(REFLEX_A)`,
/// z ∈ [0, 1]; `b` a `prism_ops` prism over z ∈ (1, 3) sheared
/// `z' = z + sx·x + sy·y`, so its bottom cap passes through `a`'s
/// corner `(0, 0, 1)`; and the closed-form volumes of [`REFLEX_OPS`].
pub struct ReflexPose {
    pub a: AtRestBody<f64>,
    pub b: AtRestBody<f64>,
    pub want: [f64; 4],
}

/// The pose of the named profile, turned `rot` degrees about the
/// corner, at shear `(sx, sy)` (not both zero).
pub fn reflex_pose(profile: &str, rot: f64, sx: f64, sy: f64, tol: Tol) -> ReflexPose {
    let a_prof = ccw(REFLEX_A.to_vec());
    let a = prism_z::<f64>(&a_prof, 0.0, 1.0, tol).body;
    let (c, s) = (rot.to_radians().cos(), rot.to_radians().sin());
    let prof = ccw(REFLEX_PROFILES
        .iter()
        .find(|(n, _)| *n == profile)
        .expect("a probe profile")
        .1
        .iter()
        .map(|&(x, y)| (c * x - s * y, s * x + c * y))
        .collect());
    let mut b = Body::<f64>::new();
    prism_ops(
        &mut b,
        &prof,
        (1.0, 3.0),
        |x, y, z| Point3::new(x, y, z + sx * x + sy * y),
        FaceGeometry::Certified,
        tol,
    );
    describe_as_intersections(&mut b, tol);
    // The overlap's height over (x, y) in both profiles is
    // min(1, max(0, −L)), L = sx·x + sy·y (b's cap may pass below a's
    // floor): ∫(0 − L)⁺ − ∫(−1 − L)⁺.
    let both = clip_convex(&a_prof, &prof);
    let len = (sx * sx + sy * sy).sqrt();
    let (ux, uy) = (-sy / len, sx / len);
    let (ix, iy) = (-sx / len, -sy / len);
    let big = 100.0;
    let below = |lvl: f64| {
        // ∫ over both ∩ {L < lvl} of (lvl − L).
        let (ox, oy) = (sx * lvl / (len * len), sy * lvl / (len * len));
        let half = ccw(vec![
            (ox + ux * big, oy + uy * big),
            (ox - ux * big, oy - uy * big),
            (ox - ux * big + ix * big, oy - uy * big + iy * big),
            (ox + ux * big + ix * big, oy + uy * big + iy * big),
        ]);
        let low = clip_convex(&both, &half);
        if low.len() < 3 {
            return 0.0;
        }
        let (mx, my) = moments(&low);
        lvl * area(&low) - (sx * mx + sy * my)
    };
    let vi = below(0.0) - below(-1.0);
    let (va, vb) = (area(&a_prof), area(&prof) * 2.0);
    ReflexPose {
        a: finished("the reflex corner", a, tol),
        b: finished("the sheared strut", b, tol),
        want: [vi, va + vb - vi, va - vi, vb - vi],
    }
}

/// One of [`REFLEX_OPS`] on a pose.
pub fn reflex_run(p: &ReflexPose, op: &str, tol: Tol) -> Result<BooleanResult<f64>, BooleanError> {
    match op {
        "I" => topo::intersect(&p.a, &p.b, tol),
        "U" => topo::union(&p.a, &p.b, tol),
        "S_ab" => topo::subtract(&p.a, &p.b, tol),
        _ => topo::subtract(&p.b, &p.a, tol),
    }
}

type V3 = [f64; 3];

fn dot3(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross3(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit3(a: V3) -> V3 {
    let s = 1.0 / dot3(a, a).sqrt();
    a.map(|c| c * s)
}

/// One prism of the near-tangent corner poses: its counter-clockwise
/// profile, its convex pieces (an oracle clips each), its height, and
/// the corner `v` the cube's near face is laid through.
pub struct NtPrism {
    pub name: &'static str,
    pub profile: Vec<(f64, f64)>,
    pub pieces: Vec<Vec<(f64, f64)>>,
    pub h: f64,
    pub v: V3,
}

impl NtPrism {
    /// The prism as an operand.
    pub fn body(&self, tol: Tol) -> AtRestBody<f64> {
        AtRestBody::validate(
            topo::test_support::prism::<f64>(&self.profile, self.h, tol).body,
            tol,
        )
        .expect("a near-tangent prism is a valid operand")
    }

    /// The corner's three edge directions: the profile's edge in, the
    /// profile's edge out, and the vertical edge into the prism.
    pub fn corner_edges(&self) -> [V3; 3] {
        let n = self.profile.len();
        let i = self
            .profile
            .iter()
            .position(|&(x, y)| (x - self.v[0]).abs() < 1e-12 && (y - self.v[1]).abs() < 1e-12)
            .expect("the corner is a profile vertex");
        let (p, a, b) = (
            self.profile[i],
            self.profile[(i + n - 1) % n],
            self.profile[(i + 1) % n],
        );
        let vz = if self.v[2] > 0.5 { -1.0 } else { 1.0 };
        [
            [a.0 - p.0, a.1 - p.1, 0.0],
            [b.0 - p.0, b.1 - p.1, 0.0],
            [0.0, 0.0, vz],
        ]
    }

    /// The near face's unit normal for the pose `nt e{edge} a{a} d{d}`:
    /// `a + 1/4` sixteenths of a turn about the corner's edge `edge`,
    /// tilted `d` along it, so the face's plane lies `d` rad off that
    /// edge. Normalized twice: JOIN's census measured the twice-rounded
    /// normal, and a sub-ulp change of it moves which poses fire.
    pub fn near_normal(&self, edge: usize, a: f64, d: f64) -> V3 {
        let e = unit3(self.corner_edges()[edge]);
        let (p1, p2) = nt_basis(e);
        let al = std::f64::consts::TAU * (a + 0.25) / 16.0;
        unit3(unit3(
            [0, 1, 2].map(|k| p1[k] * al.cos() + p2[k] * al.sin() + e[k] * d),
        ))
    }
}

/// An orthonormal pair completing the unit `m` to a right-handed frame.
pub fn nt_basis(m: V3) -> (V3, V3) {
    let m = unit3(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit3(cross3(seed, m));
    (u, cross3(m, u))
}

/// The cube `v + a u + b w + c m`, a, b ∈ [−2, 2], c ∈ [0, 4], `(u, w)`
/// [`nt_basis`] of the unit `m`: its near face holds `v` with outward
/// normal `−m`.
pub fn nt_cube(v: V3, m: V3, tol: Tol) -> AtRestBody<f64> {
    let (u, w) = nt_basis(m);
    let cube = topo::test_support::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
            let p = [0, 1, 2].map(|k| v[k] + (u[k] * a + (w[k] * b + m[k] * c)));
            Point3::new(p[0], p[1], p[2])
        },
        tol,
    );
    AtRestBody::validate(cube, tol).expect("a mapped cube is a valid operand")
}

/// The near-tangent census's prisms, by name.
pub fn nt_prisms() -> Vec<NtPrism> {
    let l = vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let l_pieces = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
    ];
    let mirror = |p: &[(f64, f64)]| -> Vec<(f64, f64)> {
        let mut q: Vec<(f64, f64)> = p.iter().map(|&(x, y)| (-x, y)).collect();
        q.reverse();
        q
    };
    let prism = |name, profile: Vec<(f64, f64)>, pieces, v| NtPrism {
        name,
        profile,
        pieces,
        h: 1.0,
        v,
    };
    vec![
        prism("Ltop", l.clone(), l_pieces.clone(), [1.0, 1.0, 1.0]),
        prism("Lbot", l.clone(), l_pieces.clone(), [1.0, 1.0, 0.0]),
        prism(
            "Lmirror",
            mirror(&l),
            l_pieces.iter().map(|p| mirror(p)).collect(),
            [-1.0, 1.0, 1.0],
        ),
        prism(
            "notch307",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)],
            vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 2.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0)],
            ],
            [2.0, 1.0, 1.0],
        ),
        prism(
            "shallow200",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6), (0.0, 1.0)],
            vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.6), (0.0, 1.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6)],
            ],
            [2.0, 0.6, 1.0],
        ),
        prism("convex", l, l_pieces, [2.0, 0.0, 1.0]),
        prism(
            "vee300",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)],
            vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.5), (0.0, 4.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5)],
            ],
            [2.0, 0.5, 1.0],
        ),
        prism(
            "asym",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0), (0.0, 1.5)],
            vec![
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.5)],
                vec![(1.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0)],
            ],
            [1.0, 1.0, 1.0],
        ),
        nt_wedge("w345", 345.0),
        nt_wedge("w60", 60.0),
    ]
}

/// The `alpha`° wedge about the z axis, corner `(0, 0, 1)`, its
/// profile on the square of half-side 2 (`join_pierce_runs_sweep`'s).
fn nt_wedge(name: &'static str, alpha: f64) -> NtPrism {
    let at = |t: f64| {
        let t = t.to_radians();
        let r = 2.0 / t.cos().abs().max(t.sin().abs());
        (r * t.cos(), r * t.sin())
    };
    let mut profile = vec![(0.0, 0.0), at(0.0)];
    let mut c = 45.0;
    while c < alpha {
        profile.push(at(c));
        c += 90.0;
    }
    profile.push(at(alpha));
    let pieces = (1..profile.len() - 1)
        .map(|k| vec![profile[0], profile[k], profile[k + 1]])
        .collect();
    NtPrism {
        name,
        profile,
        pieces,
        h: 1.0,
        v: [0.0, 0.0, 1.0],
    }
}

fn add3(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub3(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn scale3(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}

/// A half-space `n·x ≤ d`.
pub type NtPlane = (V3, f64);

/// A convex polytope as its faces, each counter-clockwise seen from
/// outside.
pub type NtPoly = Vec<Vec<V3>>;

impl NtPrism {
    /// Each convex piece of the prism as half-spaces.
    pub fn piece_planes(&self) -> Vec<Vec<NtPlane>> {
        self.pieces
            .iter()
            .map(|poly| {
                let n = poly.len();
                let mut out: Vec<NtPlane> = (0..n)
                    .map(|i| {
                        let (p, q) = (poly[i], poly[(i + 1) % n]);
                        let nn = [q.1 - p.1, -(q.0 - p.0), 0.0];
                        (nn, nn[0] * p.0 + nn[1] * p.1)
                    })
                    .collect();
                out.push(([0.0, 0.0, 1.0], self.h));
                out.push(([0.0, 0.0, -1.0], 0.0));
                out
            })
            .collect()
    }

    /// The volume the prism shares with [`nt_cube`]`(self.v, m)`: each
    /// convex piece clipped by the cube's faces, independent of the
    /// kernel.
    pub fn overlap(&self, m: V3) -> f64 {
        let cube = nt_cube_poly(self.v, m);
        self.piece_planes()
            .iter()
            .map(|pl| nt_poly_volume(&pl.iter().fold(cube.clone(), |acc, &p| nt_clip(&acc, p))))
            .sum()
    }
}

/// [`nt_cube`] as a polytope.
pub fn nt_cube_poly(v: V3, m: V3) -> NtPoly {
    let (u, w) = nt_basis(m);
    let p = |a: f64, b: f64, c: f64| add3(v, add3(scale3(u, a), add3(scale3(w, b), scale3(m, c))));
    let c = |i: usize| {
        let (a, b, z) = ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64);
        p(-2.0 + 4.0 * a, -2.0 + 4.0 * b, 4.0 * z)
    };
    let quads = [
        [0, 2, 3, 1],
        [4, 5, 7, 6],
        [0, 1, 5, 4],
        [2, 6, 7, 3],
        [0, 4, 6, 2],
        [1, 3, 7, 5],
    ];
    quads
        .iter()
        .map(|q| q.iter().map(|&i| c(i)).collect())
        .collect()
}

/// `poly` clipped by the half-space `n·x ≤ d`, the cut capped.
pub fn nt_clip(poly: &NtPoly, (n, d): NtPlane) -> NtPoly {
    let mut out: NtPoly = Vec::new();
    let mut cap: Vec<V3> = Vec::new();
    for f in poly {
        let mut g = Vec::new();
        for i in 0..f.len() {
            let (p, q) = (f[i], f[(i + 1) % f.len()]);
            let (sp, sq) = (dot3(n, p) - d, dot3(n, q) - d);
            if sp <= 0.0 {
                g.push(p);
            }
            if (sp < 0.0 && sq > 0.0) || (sp > 0.0 && sq < 0.0) {
                let t = sp / (sp - sq);
                let x = add3(p, scale3(sub3(q, p), t));
                g.push(x);
                cap.push(x);
            }
            if sp == 0.0 {
                cap.push(p);
            }
        }
        if g.len() >= 3 {
            out.push(g);
        }
    }
    if cap.len() >= 3 {
        let c = scale3(
            cap.iter().fold([0.0; 3], |a, &b| add3(a, b)),
            1.0 / cap.len() as f64,
        );
        let (e1, e2) = nt_basis(unit3(n));
        let mut pts: Vec<(f64, V3)> = cap
            .iter()
            .map(|&p| {
                let r = sub3(p, c);
                (dot3(r, e2).atan2(dot3(r, e1)), p)
            })
            .collect();
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        pts.dedup_by(|a, b| dot3(sub3(a.1, b.1), sub3(a.1, b.1)) < 1e-24);
        out.push(pts.into_iter().map(|p| p.1).collect());
    }
    out
}

/// The polytope's volume, its divergence sum taken about one of its own
/// vertices: about the world origin, a sliver's volume drowns in its
/// faces' rounding.
pub fn nt_poly_volume(poly: &NtPoly) -> f64 {
    let Some(&o) = poly.first().and_then(|f| f.first()) else {
        return 0.0;
    };
    poly.iter()
        .map(|f| {
            (1..f.len() - 1)
                .map(|i| dot3(sub3(f[0], o), cross3(sub3(f[i], o), sub3(f[i + 1], o))))
                .sum::<f64>()
        })
        .sum::<f64>()
        / 6.0
}

/// The body's mass properties at the pinned resolution: its volume and
/// its surface area, read off the B-rep.
pub fn body_props(body: &Body<f64>) -> topo::MassProperties<f64> {
    topo::test_support::pinned_volume(
        "the meshed body",
        &sweep::test_support::at_rest(body, Tol::witness()),
        Tol::witness(),
    )
}

/// [`body_props`]' volume.
pub fn body_volume(body: &Body<f64>) -> f64 {
    body_props(body).volume
}

/// **The mesh's geometry, against the body's.** Its volume within the
/// chordal deficit of the body's, both read off the B-rep
/// (`mesh::validate::check_volume`), and every sphere- and
/// cylinder-face triangle wound outward ([`assert_outward`]).
pub fn assert_meshed_geometry(label: &str, body: &Body<f64>, mesh: &mesh::Mesh, delta: f64) {
    let p = body_props(body);
    mesh::validate::check_volume(mesh, p.volume, p.surface_area, delta)
        .unwrap_or_else(|e| panic!("{label}: the mesh encloses the body, got {e:?}"));
    assert_outward(label, body, mesh);
}

/// **Every sphere- and cylinder-face triangle of `mesh` winds outward.**
/// A triangle's normal, as the mesh winds it, must point the way its
/// face's outward normal does at its centroid: off the sphere's centre,
/// or off the cylinder's axis, reversed on a `sense: false` face. A
/// triangle folded over the chord beside it, or a face meshed inside
/// out, is a closed manifold mesh that [`mesh::validate::check_mesh`]
/// passes; this is the geometry it does not read. Faces on other
/// surfaces are not read. It spells the outward direction itself rather
/// than calling the mesher's `TrimmedFace::outward` on purpose: it is
/// the independent oracle the mesher's census is checked against, and
/// it reads every face the mesh has, curved-lane faces included.
pub fn assert_outward(label: &str, body: &Body<f64>, mesh: &mesh::Mesh) {
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).expect("the patch's face");
        let radial = |p: Point3<f64>| match *body.get_surface(face.surface).expect("its surface") {
            geom::Surface::Sphere { center, .. } => Some(p - center),
            geom::Surface::Cylinder { origin, axis, .. } => {
                let d = p - origin;
                Some(d - axis * d.dot(axis))
            }
            _ => None,
        };
        for tri in &patch.triangles {
            let [a, b, c] = tri.map(|i| mesh.positions[i as usize]);
            let centroid = Point3::new(
                (a.x + b.x + c.x) / 3.0,
                (a.y + b.y + c.y) / 3.0,
                (a.z + b.z + c.z) / 3.0,
            );
            let Some(out) = radial(centroid) else { break };
            let out = if face.sense { out } else { -out };
            let normal = (b - a).cross(c - a);
            assert!(
                normal.dot(out) > 0.0,
                "{label}: face {:?} has a triangle {tri:?} wound against it (cosine {})",
                patch.face,
                normal.dot(out) / (normal.norm() * out.norm())
            );
        }
    }
}
