//! Reviewer probe (signed radius, r2): an `f64` differential over the
//! arc verbs, head against merge base. Each verb's loop is built at a
//! spread of its defining quantity, validated and extruded; the
//! validated profile's and the body's `Debug` are hashed and written to
//! `$REVIEW_OUT` (one line per case). Written against the public API
//! that both trees share, so the same file compiles on both; `#[ignore]`
//! because it asserts nothing on its own: the diff of the two outputs is
//! the reading. Run:
//! `REVIEW_OUT=/path cargo nextest run -p sweep --run-ignored only -E 'test(review_r2)'`
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;

use geom_core::{Arc2, Point2, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

fn tol() -> Tol {
    Tol::witness()
}

fn h(s: &str) -> u64 {
    let mut d = DefaultHasher::new();
    s.hash(&mut d);
    d.finish()
}

/// The head's `profile::test_support::arc_verb_loop`, re-spelled on the
/// public path API so the base compiles it too.
#[allow(clippy::too_many_lines)]
fn verb_loop(verb: &str, q: f64, t: Tol) -> ProfileLoop<f64> {
    let p = |x: f64, y: f64| Point2::new(x, y);
    match verb {
        "circle" => profile::circle(p(0.3, 0.2), q, t)
            .unwrap()
            .loop_
            .into_loop(),
        "split circle" => profile::circle_split(p(1.0, 0.5), q, 3, 0.25, t)
            .unwrap()
            .loop_
            .into_loop(),
        "bulge" | "bulge cw" => profile::Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .unwrap()
            .arc_to(
                profile::Bulge {
                    p: p(4.0, 4.0),
                    b: q,
                },
                t,
            )
            .unwrap()
            .line_to(p(0.0, 4.0), t)
            .unwrap()
            .line_to(profile::Start, t)
            .unwrap()
            .loop_
            .into_loop(),
        "via" | "via cw" => profile::Open
            .at(p(0.0, 0.0))
            .line_to(p(4.0, 0.0), t)
            .unwrap()
            .arc_to(
                profile::Via {
                    q: Point2::new(q, 2.0),
                    p: p(4.0, 4.0),
                },
                t,
            )
            .unwrap()
            .line_to(p(0.0, 4.0), t)
            .unwrap()
            .line_to(profile::Start, t)
            .unwrap()
            .loop_
            .into_loop(),
        "tangent" | "tangent cw" => {
            let far = if verb == "tangent" { 2.0 } else { -2.0 };
            profile::Open
                .at(p(0.0, 0.0))
                .line_to(p(2.0, 0.0), t)
                .unwrap()
                .tangent()
                .tangent_arc_to(Point2::new(3.0, q), t)
                .unwrap()
                .line_to(p(0.0, far), t)
                .unwrap()
                .line_to(profile::Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        "onto" => profile::Open
            .at(p(0.0, 0.0))
            .toward(0.0, 1.0, t)
            .unwrap()
            .line(q, t)
            .unwrap()
            .toward(1.0, 0.0, t)
            .unwrap()
            .tangent_arc_to(p(0.25, 1.0), t)
            .unwrap()
            .arc_onto(profile::Start, t)
            .unwrap()
            .toward(-1.0, 0.0, t)
            .unwrap()
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
                .angle(0.0, t)
                .unwrap()
                .arc_to(
                    profile::Sweep {
                        r: q,
                        side,
                        angle: core::f64::consts::FRAC_PI_2,
                    },
                    t,
                )
                .unwrap()
                .line_to(p(-1.0, if verb == "sweep" { 3.0 } else { -3.0 }), t)
                .unwrap()
                .line_to(profile::Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        "fillet" => {
            let (east, north) = (0.0_f64, core::f64::consts::FRAC_PI_2);
            let (west, south) = (core::f64::consts::PI, -north);
            profile::Open
                .at(p(0.0, -1.0))
                .angle(east, t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(1.0, 0.0), t)
                .unwrap()
                .angle(north, t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(0.0, 1.0), t)
                .unwrap()
                .angle(west, t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(-1.0, 0.0), t)
                .unwrap()
                .angle(south, t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .to(profile::Start, t)
                .unwrap()
                .loop_
                .into_loop()
        }
        other => panic!("no such verb: {other}"),
    }
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

fn case(lp: ProfileLoop<f64>) -> String {
    let profile = Profile::new(SketchPlane::xy(), vec![lp]);
    match profile.validate(tol()) {
        Err(e) => format!("validate-err {}", h(&format!("{e:?}"))),
        Ok(v) => {
            let vh = h(&format!("{v:?}"));
            let ext = extrude(
                &v,
                Extrusion::Distance {
                    depth: 1.0,
                    side: ExtrudeSide::Along,
                },
                tol(),
            );
            match ext {
                Err(e) => format!("validated {vh} extrude-err {}", h(&format!("{e:?}"))),
                Ok(x) => format!("validated {vh} body {}", h(&format!("{:?}", x.body))),
            }
        }
    }
}

#[test]
#[ignore = "a reviewer differential: its output is diffed head against base"]
fn review_r2_f64_verbs_differential() {
    let out = std::env::var("REVIEW_OUT").expect("REVIEW_OUT names the output file");
    let mut f = std::fs::File::create(out).unwrap();
    for (verb, c, half) in VERBS {
        for k in 0..80 {
            // A spread past the verb's own box, irrational steps so the
            // rounding of each member differs.
            let q = c + half * 6.0 * (f64::from(k) * 1.618_033_988_749_895).sin();
            let got = std::panic::catch_unwind(|| case(verb_loop(verb, q, tol())))
                .unwrap_or_else(|_| "panic".to_owned());
            writeln!(f, "{verb}\t{k}\t{:016x}\t{got}", q.to_bits()).unwrap();
        }
    }
    // Hand-written tables: a negative stored radius, which no verb
    // writes and the table door can.
    let tables: Vec<(&str, ProfileLoop<f64>)> = vec![
        (
            "neg circle",
            RawLoop::new([(
                Point2::new(1.75, -0.5),
                Segment::Arc(Arc2 {
                    centre: Point2::new(0.25, -0.5),
                    radius: -1.5,
                    sweep: core::f64::consts::TAU,
                }),
            )]),
        ),
        (
            "neg halves",
            RawLoop::new([
                (
                    Point2::new(1.0, 0.0),
                    Segment::Arc(Arc2 {
                        centre: Point2::new(0.0, 0.0),
                        radius: -1.0,
                        sweep: core::f64::consts::PI,
                    }),
                ),
                (
                    Point2::new(-1.0, 0.0),
                    Segment::Arc(Arc2 {
                        centre: Point2::new(0.0, 0.0),
                        radius: -1.0,
                        sweep: core::f64::consts::PI,
                    }),
                ),
            ]),
        ),
        (
            "neg halves off",
            RawLoop::new([
                (
                    Point2::new(1.0 + 3e-10, 0.0),
                    Segment::Arc(Arc2 {
                        centre: Point2::new(0.0, 0.0),
                        radius: -1.0,
                        sweep: core::f64::consts::PI,
                    }),
                ),
                (
                    Point2::new(-1.0, 0.0),
                    Segment::Arc(Arc2 {
                        centre: Point2::new(0.0, 0.0),
                        radius: -1.0,
                        sweep: core::f64::consts::PI,
                    }),
                ),
            ]),
        ),
    ];
    for (name, lp) in tables {
        let profile = Profile::new(SketchPlane::xy(), vec![lp]);
        let got = std::panic::catch_unwind(|| format!("{:?}", profile.validate(tol())))
            .unwrap_or_else(|_| "panic".to_owned());
        let short: String = got.chars().take(400).collect();
        writeln!(f, "{name}\t{short}").unwrap();
    }
}
