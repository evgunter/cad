//! Reviewer probes for the signed-radius unit (PR 7686, round 1).
//!
//! Written against the public surface the merge base `f2815a232a`
//! already has, so the same file builds on both trees and its printed
//! lines are a head-against-base differential:
//!
//! - `REVIEW_DUMP=<file>` appends every line a probe prints to `<file>`.
//!
//! The verb fixture below is a copy of `profile::test_support::arc_verb_loop`
//! (which the base does not have) with a pose added; it exists only so
//! the base can be driven through the same documents.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::float_cmp,
    clippy::type_complexity
)]

use std::io::Write as _;
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom::Surface;
use geom_core::sym::with_session;
use geom_core::{Interval, ParamSymbol, Point2, Real, Sym, SymBudget, Tol};
use profile::{Profile, ProfileLoop, Segment, SegmentDirections, SegmentKind, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

fn tol() -> Tol {
    Tol::witness()
}

type S = Sym<Interval>;

fn dump(line: &str) {
    println!("{line}");
    if let Ok(path) = std::env::var("REVIEW_DUMP") {
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        writeln!(f, "{line}").unwrap();
    }
}

/// The variant name of a refusal: its `Debug` up to the first payload.
fn variant<E: core::fmt::Debug>(e: &E) -> String {
    let s = format!("{e:?}");
    s.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':'))
        .next()
        .unwrap_or("")
        .to_owned()
}

/// The first 220 characters of a refusal.
fn short<E: core::fmt::Debug>(e: &E) -> String {
    format!("{e:?}").chars().take(220).collect()
}

/// A pose: every authored length times `s`, every authored point moved
/// by `(ox, oy)`.
#[derive(Clone, Copy, Debug)]
struct Pose {
    s: f64,
    ox: f64,
    oy: f64,
}

const HOME: Pose = Pose {
    s: 1.0,
    ox: 0.0,
    oy: 0.0,
};

/// One arc verb's loop through its door, `q` the quantity that fixes the
/// arc's radius, refusals as their `Debug`.
fn verb_loop<T: profile::ArcCarrierScalar>(
    verb: &str,
    q: T,
    pose: Pose,
    t: Tol,
) -> Result<ProfileLoop<T>, String> {
    let l = T::from_f64;
    let p = |x: f64, y: f64| Point2::new(l(pose.ox + pose.s * x), l(pose.oy + pose.s * y));
    let len = |x: f64| l(pose.s * x);
    let e = |e: profile::PathError<T>| format!("{e:?}");
    Ok(match verb {
        "circle" => profile::circle(p(0.3, 0.2), q, t)
            .map_err(e)?
            .loop_
            .into_loop(),
        "split circle" => profile::circle_split(p(1.0, 0.5), q, 3, l(0.25), t)
            .map_err(e)?
            .loop_
            .into_loop(),
        "bulge" | "bulge cw" => profile::Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .map_err(e)?
            .arc_to(
                profile::Bulge {
                    p: p(4.0, 4.0),
                    b: q,
                },
                t,
            )
            .map_err(e)?
            .line_to(p(0.0, 4.0), t)
            .map_err(e)?
            .line_to(profile::Start, t)
            .map_err(e)?
            .loop_
            .into_loop(),
        "via" | "via cw" => profile::Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .map_err(e)?
            .arc_to(
                profile::Via {
                    q: Point2::new(q, l(pose.oy + pose.s * 2.0)),
                    p: p(4.0, 4.0),
                },
                t,
            )
            .map_err(e)?
            .line_to(p(0.0, 4.0), t)
            .map_err(e)?
            .line_to(profile::Start, t)
            .map_err(e)?
            .loop_
            .into_loop(),
        "tangent" | "tangent cw" => {
            let far = if verb == "tangent" { 2.0 } else { -2.0 };
            profile::Open
                .at(p(0.0, 0.0))
                .line_to(p(2.0, 0.0), t)
                .map_err(e)?
                .tangent()
                .tangent_arc_to(Point2::new(l(pose.ox + pose.s * 3.0), q), t)
                .map_err(e)?
                .line_to(p(0.0, far), t)
                .map_err(e)?
                .line_to(profile::Start, t)
                .map_err(e)?
                .loop_
                .into_loop()
        }
        "onto" => profile::Open
            .at(p(0.0, 0.0))
            .toward(l(0.0), l(1.0), t)
            .map_err(e)?
            .line(q, t)
            .map_err(e)?
            .toward(l(1.0), l(0.0), t)
            .map_err(e)?
            .tangent_arc_to(p(0.25, 1.0), t)
            .map_err(e)?
            .arc_onto(profile::Start, t)
            .map_err(e)?
            .toward(l(-1.0), l(0.0), t)
            .map_err(e)?
            .loop_
            .into_loop(),
        "sweep" | "sweep cw" => {
            let side = if verb == "sweep" {
                profile::ArcSide::Left
            } else {
                profile::ArcSide::Right
            };
            profile::Open
                .at(p(0.0, 0.0))
                .angle(l(0.0), t)
                .map_err(e)?
                .arc_to(
                    profile::Sweep {
                        r: q,
                        side,
                        angle: l(core::f64::consts::FRAC_PI_2),
                    },
                    t,
                )
                .map_err(e)?
                .line_to(p(-1.0, if verb == "sweep" { 3.0 } else { -3.0 }), t)
                .map_err(e)?
                .line_to(profile::Start, t)
                .map_err(e)?
                .loop_
                .into_loop()
        }
        "fillet" => {
            let (east, north) = (0.0_f64, core::f64::consts::FRAC_PI_2);
            let (west, south) = (core::f64::consts::PI, -north);
            let _ = len;
            profile::Open
                .at(p(0.0, -1.0))
                .angle(l(east), t)
                .map_err(e)?
                .fillet(q, t)
                .map_err(e)?
                .at(p(1.0, 0.0), t)
                .map_err(e)?
                .angle(l(north), t)
                .map_err(e)?
                .fillet(q, t)
                .map_err(e)?
                .at(p(0.0, 1.0), t)
                .map_err(e)?
                .angle(l(west), t)
                .map_err(e)?
                .fillet(q, t)
                .map_err(e)?
                .at(p(-1.0, 0.0), t)
                .map_err(e)?
                .angle(l(south), t)
                .map_err(e)?
                .fillet(q, t)
                .map_err(e)?
                .to(profile::Start, t)
                .map_err(e)?
                .loop_
                .into_loop()
        }
        other => panic!("no such verb: {other}"),
    })
}

const VERBS: [(&str, f64, f64); 12] = [
    ("circle", 0.05, 0.005),
    ("split circle", 0.4, 0.04),
    ("bulge", 0.4, 0.05),
    ("bulge cw", -0.3, 0.05),
    ("via", 5.0, 0.1),
    ("via cw", 3.5, 0.1),
    ("tangent", 1.0, 0.1),
    ("tangent cw", -1.0, 0.1),
    ("onto", 2.0, 0.05),
    ("sweep", 1.0, 0.05),
    ("sweep cw", 1.0, 0.05),
    ("fillet", 0.25, 0.02),
];

fn in_session<R>(f: impl FnOnce() -> R) -> R {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let mut out = None;
    with_session(budget, || out = Some(f()));
    out.unwrap()
}

/// The stage a verb's unit slab reaches over `c ± half` at `Sym<Interval>`.
fn stage(verb: &str, c: f64, half: f64) -> String {
    let run = || {
        in_session(|| {
            let q: S = Sym::param(
                ParamSymbol::new(0x9b0),
                Interval::from_bounds(c - half, c + half),
            );
            let lp = match verb_loop::<S>(verb, q, HOME, tol()) {
                Ok(lp) => lp,
                Err(e) => return format!("path: {}", e.chars().take(220).collect::<String>()),
            };
            let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                Ok(profile) => profile,
                Err(e) => return format!("validation: {}", short(&e)),
            };
            let depth = <S as Real>::from_f64(1.0);
            let side = ExtrudeSide::Along;
            match extrude(&profile, Extrusion::Distance { depth, side }, tol()) {
                Err(e) => format!("door: {}", short(&e)),
                Ok(slab) => match topo::take::<S>(&std::sync::Arc::new(slab.body), tol()) {
                    Err(e) => format!("finish: {}", short(&e)),
                    Ok(_) => "finished".to_owned(),
                },
            }
        })
    };
    catch_unwind(AssertUnwindSafe(run)).unwrap_or_else(|p| {
        let msg = p
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_owned()))
            .unwrap_or_default();
        format!("PANIC: {}", msg.chars().take(200).collect::<String>())
    })
}

/// Claim 10: each verb's stage over a ladder of boxes, for the
/// head-against-base diff. Prints; a panic in a door is a line too.
#[test]
fn p10_stage_reached_by_each_verb_over_a_ladder_of_boxes() {
    let eps = tol().eps();
    for (verb, c, _) in VERBS {
        for half in [eps / 64.0, eps, 1e-8, 1e-7, 1e-6, 1e-4] {
            let got = stage(verb, c, half);
            let key: String = got.chars().take(120).collect();
            dump(&format!("STAGE {verb} half={half:e}: {key}"));
        }
    }
    // The bulge-beside-a-line row's own box (bulge 0.5 ± 0.02), and its
    // siblings at that relative width.
    for (verb, c, half) in [
        ("bulge", 0.5, 0.02),
        ("bulge cw", -0.5, 0.02),
        ("via", 5.0, 0.02),
        ("tangent", 1.0, 0.02),
        ("sweep", 1.0, 0.02),
        ("circle", 0.05, 0.005),
        ("split circle", 0.4, 0.02),
        ("fillet", 0.25, 0.02),
    ] {
        let got = stage(verb, c, half);
        let key: String = got.chars().take(120).collect();
        dump(&format!("STAGE {verb} c={c} half={half:e}: {key}"));
    }
}

// ---------------------------------------------------------------------------
// The f64 differential and the positive-radius sweep.

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn log(&mut self, lo: f64, hi: f64) -> f64 {
        10f64.powf(lo + (hi - lo) * self.unit())
    }
    fn sign(&mut self) -> f64 {
        if self.next() & 1 == 0 { 1.0 } else { -1.0 }
    }
}

fn fnv(h: &mut u64, bits: u64) {
    for b in bits.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

/// What one f64 document does, stage by stage: a line for the diff, and
/// the non-positive radii it stored.
fn f64_line(verb: &str, q: f64, pose: Pose) -> (String, Vec<f64>) {
    let mut bad = Vec::new();
    let mut h = 0xcbf2_9ce4_8422_2325_u64;
    let lp = match verb_loop::<f64>(verb, q, pose, tol()) {
        Ok(lp) => lp,
        Err(e) => {
            let v: String = e
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .next()
                .unwrap_or("")
                .to_owned();
            return (format!("path:{v}"), bad);
        }
    };
    for (seg, dir) in lp.segments().iter().zip(lp.directions()) {
        if let Segment::Arc(arc) = seg {
            for x in [arc.centre.x, arc.centre.y, arc.radius, arc.sweep] {
                fnv(&mut h, x.to_bits());
            }
            if !(arc.radius > 0.0) {
                bad.push(arc.radius);
            }
            // The stored sweep's sign and a radius's sign are one
            // construction's: a negative radius with radials recorded
            // would put the centre's far side on the vertex.
        }
        if let Some(SegmentDirections::Arc { start, end }) = dir {
            for x in [start.x, start.y, end.x, end.y] {
                fnv(&mut h, x.to_bits());
            }
        }
    }
    let stored = h;
    let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
        Ok(p) => p,
        Err(e) => {
            return (
                format!("stored:{stored:016x} validation:{}", variant(&e)),
                bad,
            );
        }
    };
    for seg in profile.loops()[0].segments() {
        if let SegmentKind::Arc { arc, radials, .. } = seg.kind {
            for x in [
                arc.centre.x,
                arc.centre.y,
                arc.radius,
                arc.sweep,
                radials[0].x,
                radials[0].y,
                radials[1].x,
                radials[1].y,
            ] {
                fnv(&mut h, x.to_bits());
            }
            if !(arc.radius > 0.0) {
                bad.push(arc.radius);
            }
        }
    }
    let validated = h;
    let side = ExtrudeSide::Along;
    let built = catch_unwind(AssertUnwindSafe(|| {
        extrude(
            &profile,
            Extrusion::Distance {
                depth: pose.s,
                side,
            },
            tol(),
        )
    }));
    let slab = match built {
        Err(_) => {
            return (
                format!("stored:{stored:016x} validated:{validated:016x} door:PANIC"),
                bad,
            );
        }
        Ok(Err(e)) => {
            return (
                format!(
                    "stored:{stored:016x} validated:{validated:016x} door:{}",
                    variant(&e)
                ),
                bad,
            );
        }
        Ok(Ok(slab)) => slab,
    };
    let body: &topo::Body<f64> = &slab.body;
    // Frames to 1e-12 relative, so the last bits the body's change moves
    // by design do not flood the diff; decisions and stored arcs are bit
    // for bit above.
    let coarse = |x: f64| (x * 1e12).round() as i64 as u64;
    let mut n = 0u64;
    for (face, _) in body.faces() {
        if let Surface::Cylinder { frame, .. } = body.face_surface(face).expect("a live face") {
            let u = frame.u_ref();
            for x in [u.x, u.y, u.z] {
                fnv(&mut h, coarse(x));
            }
            n += 1;
        }
    }
    (
        format!("stored:{stored:016x} validated:{validated:016x} built:{h:016x} walls:{n}"),
        bad,
    )
}

/// Claims 1 and 15: a generated set of f64 documents per verb, over
/// poses far from the origin and quantities down to the band. Every line
/// is dumped for the head-against-base diff; a stored radius that is not
/// positive fails the row.
#[test]
fn p15_f64_documents_stage_by_stage_and_no_stored_radius_is_not_positive() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut bad = Vec::new();
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    let poses = [
        HOME,
        Pose {
            s: 1.0,
            ox: 1.0e3,
            oy: -7.0e2,
        },
        Pose {
            s: 1.0,
            ox: 1.0e6,
            oy: 3.0e5,
        },
        Pose {
            s: 1.0,
            ox: -4.0e8,
            oy: 1.0e9,
        },
        Pose {
            s: 1.0e-3,
            ox: 0.0,
            oy: 0.0,
        },
        Pose {
            s: 1.0e3,
            ox: 0.0,
            oy: 0.0,
        },
        Pose {
            s: 1.0e6,
            ox: 5.0e6,
            oy: 5.0e6,
        },
    ];
    for (verb, c, _) in VERBS {
        for (k, pose) in poses.iter().enumerate() {
            for i in 0..60 {
                // The defining quantity: near its fixture value, and
                // pushed toward the degenerate end of its door.
                let q = match verb {
                    "circle" | "split circle" | "sweep" | "sweep cw" | "fillet" => {
                        pose.s * rng.log(-11.0, 2.0)
                    }
                    "bulge" | "bulge cw" => rng.sign() * rng.log(-13.0, 3.0),
                    // The through point's x: the chord is x = 4, so its
                    // offset from the chord line is `q − 4`.
                    "via" | "via cw" => pose.ox + pose.s * (4.0 + rng.sign() * rng.log(-12.0, 1.0)),
                    // The target's y: the departure line is y = 0.
                    "tangent" | "tangent cw" => pose.oy + pose.s * rng.sign() * rng.log(-12.0, 1.5),
                    "onto" => pose.s * rng.log(-3.0, 1.0),
                    _ => c,
                };
                let (line, nonpos) = f64_line(verb, q, *pose);
                let stage_key: String = line
                    .split(' ')
                    .map(|part| part.split(':').next().unwrap_or(""))
                    .next_back()
                    .unwrap_or("")
                    .to_owned();
                let ending = line.rsplit(' ').next().unwrap_or("").to_owned();
                let key = if stage_key == "walls" {
                    "built".to_owned()
                } else if ending.starts_with("path:")
                    || ending.starts_with("validation:")
                    || ending.starts_with("door:")
                {
                    ending
                } else {
                    stage_key
                };
                *tally.entry(format!("{verb} / {key}")).or_default() += 1;
                dump(&format!("F64 {verb} pose{k} #{i} q={q:e}: {line}"));
                for r in nonpos {
                    bad.push(format!("{verb} pose {pose:?} q={q:e}: radius {r:e}"));
                }
            }
        }
    }
    for (k, n) in &tally {
        println!("TALLY {k}: {n}");
    }
    assert!(
        bad.is_empty(),
        "a door stored a radius that is not positive:\n{}",
        bad.join("\n")
    );
}

/// Claim 4: a bulge boxed across zero, and one whose box only touches
/// it, through `arc_to(Bulge)` and validation, at `Interval` and at
/// `Sym<Interval>`: what each stage answers.
#[test]
fn p4_a_bulge_boxed_across_zero() {
    for (lo, hi) in [
        (-0.1, 0.3),
        (-1e-12, 1e-12),
        (0.0, 0.3),
        (1e-300, 0.3),
        (0.1, 0.3),
    ] {
        let at_interval = catch_unwind(AssertUnwindSafe(|| {
            let b = Interval::from_bounds(lo, hi);
            match verb_loop::<Interval>("bulge", b, HOME, tol()) {
                Err(e) => format!("path: {}", e.chars().take(160).collect::<String>()),
                Ok(lp) => {
                    let arc = lp.segments().iter().find_map(|s| match s {
                        Segment::Arc(a) => Some(*a),
                        Segment::Line => None,
                    });
                    let recorded = lp
                        .directions()
                        .iter()
                        .any(|d| matches!(d, Some(SegmentDirections::Arc { .. })));
                    let stored = arc.map_or("a line".to_owned(), |a| {
                        format!(
                            "radius [{:e}, {:e}] radials recorded: {recorded}",
                            geom_core::Bounds::lo(a.radius),
                            geom_core::Bounds::hi(a.radius)
                        )
                    });
                    match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                        Ok(_) => format!("{stored}; validates"),
                        Err(e) => format!("{stored}; validation: {}", short(&e)),
                    }
                }
            }
        }))
        .unwrap_or_else(|_| "PANIC".to_owned());
        dump(&format!("TIE Interval [{lo:e}, {hi:e}]: {at_interval}"));
        let c = 0.5 * (lo + hi);
        let at_sym = stage("bulge", c, 0.5 * (hi - lo));
        dump(&format!(
            "TIE Sym<Interval> [{lo:e}, {hi:e}]: {}",
            at_sym.chars().take(200).collect::<String>()
        ));
    }
}

/// Claim 3: a table arc whose stored radius is negative, or zero, never
/// reads on its carrier or clear of anything: validation refuses it or
/// escalates, at `f64` and at `Interval`.
#[test]
fn p3_a_table_arc_with_a_radius_that_is_not_positive() {
    use profile::RawLoop;
    for radius in [-1.0, 0.0, -1.0 + 1e-13] {
        for on_exactly in [true, false] {
            let half = geom_core::Arc2 {
                centre: Point2::new(0.0, 0.0),
                radius,
                sweep: core::f64::consts::PI,
            };
            // A vertex exactly on the mirror circle, or a rounding off it.
            let x = if on_exactly {
                1.0
            } else {
                1.0 + 2.0 * f64::EPSILON
            };
            let lp: ProfileLoop<f64> = RawLoop::new([
                (Point2::new(x, 0.0), Segment::Arc(half)),
                (Point2::new(-x, 0.0), Segment::Arc(half)),
            ]);
            let got = match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                Ok(_) => "VALIDATES".to_owned(),
                Err(e) => short(&e),
            };
            dump(&format!(
                "NEGATIVE f64 radius {radius:e} vertex x={x:e}: {got}"
            ));
            assert!(
                got != "VALIDATES" || radius > 0.0,
                "a table arc of radius {radius:e} validated"
            );
        }
    }
}

/// Claim 6: a table's written radial that is no unit, at the enclosure
/// scalars, where the frame mint keeps its bits or poisons: what the
/// extrude door does with it (a typed refusal, a body, or a panic).
#[test]
fn p6_a_tables_stretched_radial_at_the_enclosure_scalars() {
    use profile::RawLoop;
    fn run<T: geom_core::Decide + topo::AtRestPolicy>(lane: &str, stretch: f64) {
        let l = T::from_f64;
        let half = geom_core::Arc2 {
            centre: Point2::new(l(0.0), l(0.0)),
            radius: l(1.0),
            sweep: l(core::f64::consts::PI),
        };
        let got = catch_unwind(AssertUnwindSafe(|| {
            let lp: ProfileLoop<T> = RawLoop::new_directed([
                (
                    Point2::new(l(1.0), l(0.0)),
                    Segment::Arc(half),
                    Some(SegmentDirections::Arc {
                        start: geom_core::Vec2::new(l(1.0 + stretch), l(0.0)),
                        end: geom_core::Vec2::new(l(-1.0), l(0.0)),
                    }),
                ),
                (Point2::new(l(-1.0), l(0.0)), Segment::Arc(half), None),
            ]);
            let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                Ok(p) => p,
                Err(e) => return format!("validation: {}", short(&e)),
            };
            let side = ExtrudeSide::Along;
            match extrude(
                &profile,
                Extrusion::Distance {
                    depth: l(1.0),
                    side,
                },
                tol(),
            ) {
                Err(e) => format!("door: {}", short(&e)),
                Ok(_) => "builds".to_owned(),
            }
        }))
        .unwrap_or_else(|p| {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                .unwrap_or_default();
            format!("PANIC: {}", msg.chars().take(240).collect::<String>())
        });
        dump(&format!("STRETCH {lane} {stretch:e}: {got}"));
    }
    for part in [0.0, 1e-7, 0.1, 0.4] {
        let stretch = part * tol().eps();
        run::<f64>("f64", stretch);
        run::<Interval>("Interval", stretch);
        in_session(|| run::<S>("Sym<Interval>", stretch));
    }
}

/// Claim 4, the arm itself: a bulge leg that is the path's FIRST leg (no
/// junction ahead of it to escalate first), and the fixture door's
/// lowering, each with the bulge boxed across zero.
#[test]
fn p4b_a_tied_bulge_with_no_junction_ahead_of_it() {
    for (lo, hi) in [(-0.1, 0.3), (-1e-12, 1e-12), (0.1, 0.3)] {
        let got = catch_unwind(AssertUnwindSafe(|| {
            let l = Interval::from_f64;
            let p = |x: f64, y: f64| Point2::new(l(x), l(y));
            let e = |e: profile::PathError<Interval>| short(&e);
            let lp = profile::Open
                .at(p(4.0, 0.0))
                .arc_to(
                    profile::Bulge {
                        p: p(4.0, 4.0),
                        b: Interval::from_bounds(lo, hi),
                    },
                    tol(),
                )
                .map_err(e)?
                .line_to(p(0.0, 4.0), tol())
                .map_err(e)?
                .line_to(p(0.0, 0.0), tol())
                .map_err(e)?
                .line_to(profile::Start, tol())
                .map_err(e)?
                .loop_
                .into_loop();
            let arc = lp.segments().iter().find_map(|s| match s {
                Segment::Arc(a) => Some(*a),
                Segment::Line => None,
            });
            let recorded = lp
                .directions()
                .iter()
                .any(|d| matches!(d, Some(SegmentDirections::Arc { .. })));
            let stored = arc.map_or("a line".to_owned(), |a| {
                format!(
                    "radius [{:e}, {:e}] radials recorded: {recorded}",
                    geom_core::Bounds::lo(a.radius),
                    geom_core::Bounds::hi(a.radius)
                )
            });
            Ok::<_, String>(
                match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                    Ok(_) => format!("{stored}; VALIDATES"),
                    Err(e) => format!("{stored}; validation: {}", short(&e)),
                },
            )
        }))
        .map_or_else(
            |_| "PANIC".to_owned(),
            |r| r.unwrap_or_else(|e| format!("path: {e}")),
        );
        dump(&format!("TIE-FIRST Interval [{lo:e}, {hi:e}]: {got}"));
        let fixture = catch_unwind(AssertUnwindSafe(|| {
            let l = Interval::from_f64;
            let lp = profile::test_support::bulge_loop(vec![
                (Point2::new(l(0.0), l(0.0)), l(0.0)),
                (Point2::new(l(4.0), l(0.0)), Interval::from_bounds(lo, hi)),
                (Point2::new(l(4.0), l(4.0)), l(0.0)),
                (Point2::new(l(0.0), l(4.0)), l(0.0)),
            ]);
            let arc = lp.segments().iter().find_map(|s| match s {
                Segment::Arc(a) => Some(*a),
                Segment::Line => None,
            });
            let stored = arc.map_or("a line".to_owned(), |a| {
                format!(
                    "radius [{:e}, {:e}]",
                    geom_core::Bounds::lo(a.radius),
                    geom_core::Bounds::hi(a.radius)
                )
            });
            match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                Ok(_) => format!("{stored}; VALIDATES"),
                Err(e) => format!("{stored}; validation: {}", short(&e)),
            }
        }))
        .unwrap_or_else(|_| "PANIC".to_owned());
        dump(&format!("TIE-FIXTURE Interval [{lo:e}, {hi:e}]: {fixture}"));
    }
}

/// Claim 15, the detail: the f64 documents whose stage moved between
/// the merge base and the head, with each stage's refusal in full.
#[test]
fn p15b_the_f64_documents_that_moved_in_full() {
    let pose = |k: usize| match k {
        0 => HOME,
        1 => Pose {
            s: 1.0,
            ox: 1.0e3,
            oy: -7.0e2,
        },
        2 => Pose {
            s: 1.0,
            ox: 1.0e6,
            oy: 3.0e5,
        },
        5 => Pose {
            s: 1.0e3,
            ox: 0.0,
            oy: 0.0,
        },
        6 => Pose {
            s: 1.0e6,
            ox: 5.0e6,
            oy: 5.0e6,
        },
        _ => panic!("no such pose"),
    };
    for (verb, k, q) in [
        ("bulge", 0, -1.306_923_626_133_198e-8),
        ("bulge cw", 0, -1.363_230_105_576_11e-8),
        ("via", 0, 3.999_999_896_912_03e0),
        ("via cw", 0, 3.999_999_987_985_185_5e0),
        ("bulge cw", 6, -1.233_330_482_735_325_2e-3),
        ("bulge", 5, -3.706_648_103_120_864_3e-6),
        ("via", 5, 4.000_000_069_730_961_5e3),
        ("split circle", 6, 1.708_677_920_526_177_4e1),
        ("split circle", 6, 8.565_958_696_678_71e7),
        ("bulge", 1, -1.065_754_758_032_666e-7),
        ("bulge cw", 1, 9.190_941_974_346_471e-8),
        ("tangent", 6, 5.031_441_873_472_028e6),
        ("tangent cw", 2, 3.000_000_000_000_569e5),
        ("tangent cw", 5, 2.627_944_907_625_093_6e-2),
        ("via cw", 6, 1.631_507_862_826_352_4e7),
    ] {
        let p = pose(k);
        let got = match verb_loop::<f64>(verb, q, p, tol()) {
            Err(e) => format!("path: {}", e.chars().take(700).collect::<String>()),
            Ok(lp) => {
                let arcs: Vec<String> = lp
                    .segments()
                    .iter()
                    .filter_map(|s| match s {
                        Segment::Arc(a) => Some(format!("r={:e} sweep={:e}", a.radius, a.sweep)),
                        Segment::Line => None,
                    })
                    .collect();
                match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
                    Err(e) => format!("{arcs:?}; validation: {}", short(&e)),
                    Ok(profile) => {
                        let side = ExtrudeSide::Along;
                        match catch_unwind(AssertUnwindSafe(|| {
                            extrude(&profile, Extrusion::Distance { depth: p.s, side }, tol())
                        })) {
                            Err(_) => format!("{arcs:?}; door: PANIC"),
                            Ok(Err(e)) => format!("{arcs:?}; door: {}", short(&e)),
                            Ok(Ok(_)) => format!("{arcs:?}; built"),
                        }
                    }
                }
            }
        };
        dump(&format!("MOVED {verb} pose{k} q={q:e}: {got}"));
    }
}
