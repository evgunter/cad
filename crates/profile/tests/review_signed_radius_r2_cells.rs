//! Reviewer probe (signed radius, r2), claim 10: the validation stage
//! each arc verb reaches over a box at `Sym<Interval>` with the
//! registered-identity door shut (the fillet with it open), at ±ε/64
//! and ±1e-6, spelled on the path API the head and the merge base
//! share so the same file runs on both. Prints; asserts nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::sym::with_session_rules;
use geom_core::{Interval, ParamSymbol, Point2, Sym, SymBudget, SymRules, Tol};
use profile::{Profile, ProfileLoop, SketchPlane};

type Lane = Sym<Interval>;

#[allow(clippy::too_many_lines)]
fn verb_loop<T: profile::ArcCarrierScalar>(verb: &str, q: T, t: geom_core::Tol) -> ProfileLoop<T> {
    let l = T::from_f64;
    let p = |x: f64, y: f64| Point2::new(l(x), l(y));
    match verb {
        "circle" => profile::circle(p(0.3, 0.2), q, t)
            .unwrap()
            .loop_
            .into_loop(),
        "split circle" => profile::circle_split(p(1.0, 0.5), q, 3, l(0.25), t)
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
                    q: Point2::new(q, l(2.0)),
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
                .tangent_arc_to(Point2::new(l(3.0), q), t)
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
            .toward(l(0.0), l(1.0), t)
            .unwrap()
            .line(q, t)
            .unwrap()
            .toward(l(1.0), l(0.0), t)
            .unwrap()
            .tangent_arc_to(p(0.25, 1.0), t)
            .unwrap()
            .arc_onto(profile::Start, t)
            .unwrap()
            .toward(l(-1.0), l(0.0), t)
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
                .angle(l(0.0), t)
                .unwrap()
                .arc_to(
                    profile::Sweep {
                        r: q,
                        side,
                        angle: l(core::f64::consts::FRAC_PI_2),
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
                .angle(l(east), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(1.0, 0.0), t)
                .unwrap()
                .angle(l(north), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(0.0, 1.0), t)
                .unwrap()
                .angle(l(west), t)
                .unwrap()
                .fillet(q, t)
                .unwrap()
                .at(p(-1.0, 0.0), t)
                .unwrap()
                .angle(l(south), t)
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

const CELLS: [(&str, f64); 6] = [
    ("circle", 0.05),
    ("bulge", 0.4),
    ("tangent", 1.0),
    ("onto", 2.0),
    ("fillet", 0.25),
    ("via", 5.0),
];

#[test]
fn review_r2_cells_validation_stage_over_a_box() {
    let tol = Tol::witness();
    for (verb, c) in CELLS {
        for h in [tol.eps() / 64.0, 1e-6] {
            let rules = if verb == "fillet" {
                SymRules::shipped()
            } else {
                SymRules::shipped_without_the_door()
            };
            let got = std::thread::spawn(move || {
                with_session_rules(
                    SymBudget {
                        max_terms: 4096,
                        max_degree: 128,
                    },
                    rules,
                    || {
                        let q = Sym::param(
                            ParamSymbol::new(test_utils::symbol_id("q")),
                            Interval::from_bounds(c - h, c + h),
                        );
                        let lp: ProfileLoop<Lane> = verb_loop(verb, q, tol);
                        match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol) {
                            Ok(_) => "validates".to_owned(),
                            Err(e) => format!("{e:?}").chars().take(160).collect(),
                        }
                    },
                )
                .0
            })
            .join()
            .map_err(|e| {
                e.downcast_ref::<String>()
                    .map(|s| s.chars().take(160).collect::<String>())
                    .unwrap_or_default()
            });
            println!("CELL {verb} ±{h:e}: {got:?}");
        }
    }
}
