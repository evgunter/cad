//! PROBE (BOXFINISH M3, measurement lane; never merged). Every row here
//! prints and asserts nothing, so every row is `#[ignore]`d and run by
//! name with `--ignored --nocapture` and `BOXFINISH_TRACE=1` (or `2`,
//! which also prints the stored data of a finish that passes).
//!
//! Dials are env vars so a width bisects without a rebuild:
//! `M3_THETA_DEG`, `M3_DELTA_DEG`, `M3_RADIUS`, `M3_DEPTH`, `M3_AXIS`
//! (`x`|`y`|`z`), `M3_TAKER` (`none`|`union`), `M3_LEAVES`,
//! `M3_SCALE` (the cut plate's box as a fraction of the tour's study),
//! `M3_FORM` (`poly`|`rect`|`pen`), `M3_HALF`, `M3_OFFSET_HALF`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;
use std::sync::Arc;

use crate::fixture::{self, Recorder, ang, len};

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{
    DEFAULT_MAX_DEPTH, DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS, DriveConfig, drive,
};
use editor_core::{
    Bodies, BodyRead, Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node,
    Operand, ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, TargetAxis,
    UnitSym, VarName,
};
use geom_core::{Arc2, Interval, Point2, RigidMotion, Sym, Tol, UnitVec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

fn env_f64(name: &str, default: f64) -> f64 {
    std::env::var(name).map_or(default, |v| v.parse().expect("a number"))
}

fn env_str(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_owned())
}

fn declare(r: &mut Recorder, name: &'static str, dim: Dimension, value: f64, dist: Distribution) {
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(name),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim,
            value,
            display_unit: UnitSym::canonical_for(dim),
            distribution: Some(dist),
        }),
    });
}

fn uniform(half: f64) -> Distribution {
    Distribution::Uniform {
        lo: -half,
        hi: half,
    }
}

fn named(name: &'static str, dim: Dimension) -> Formula {
    Formula::named(VarName::from_static(name), dim)
}

/// Drives `doc` over its declared box and prints the receipt, the
/// accounting and each refused leaf's reason class (the first three in
/// full).
fn report(label: &str, doc: &ProfileDoc) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let max_leaves = env_f64("M3_LEAVES", 32.0) as usize;
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let started = std::time::Instant::now();
    let verdict = drive(
        doc,
        &analyzed,
        &DriveConfig {
            max_depth: DEFAULT_MAX_DEPTH,
            max_leaves,
            ..DriveConfig::default()
        },
        &editor_core::Stop::default(),
        Tol::witness(),
    );
    let v = match verdict {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[m3] {label}: the drive refused: {e:?}");
            return;
        }
    };
    let acc = v.accounting();
    eprintln!(
        "[m3] {label}: max_leaves={max_leaves} receipt={:?} decisions={:?} dt_s={:.2}",
        v.receipt(),
        v.decisions(),
        started.elapsed().as_secs_f64()
    );
    eprintln!(
        "[m3] {label}: mass certified={:?} refused={:?} unanalyzed={:?}",
        acc.certified, acc.refused, acc.unanalyzed
    );
    let mut classes = std::collections::BTreeMap::new();
    for leaf in v.refused() {
        *classes.entry(format!("{:?}", leaf.reason.class())).or_insert(0usize) += 1;
    }
    eprintln!("[m3] {label}: refused classes {classes:?}");
    for leaf in v.refused().iter().take(3) {
        let text = format!("{:?}", leaf.reason);
        eprintln!(
            "[m3] {label}:   refused {}",
            text.chars().take(1200).collect::<String>()
        );
    }
}

fn axis_of(name: &str) -> TargetAxis {
    match name {
        "x" => TargetAxis::X,
        "y" => TargetAxis::Y,
        "z" => TargetAxis::Z,
        other => panic!("M3_AXIS is x, y or z: {other}"),
    }
}

/// **Item 1, through the document**: a disc of radius `M3_RADIUS`
/// centred at the origin of its own sketch, extruded `M3_DEPTH`, and one
/// copy of it in the world spun about the world's `M3_AXIS` by the angle
/// variable `theta` = `M3_THETA_DEG` ± `M3_DELTA_DEG`. A document's
/// extrude is built on its profile's own `xy` (`eval::wire`), so the
/// turn reaches the body as a boxed motion of the leaf.
fn turned_disc() -> ProfileDoc {
    let theta = env_f64("M3_THETA_DEG", 30.0).to_radians();
    let delta = env_f64("M3_DELTA_DEG", 1.0).to_radians();
    let radius = env_f64("M3_RADIUS", 5e-3);
    let depth = env_f64("M3_DEPTH", radius);
    let mut r = Recorder::new();
    declare(&mut r, "theta", Dimension::Angle, theta, uniform(delta));
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
            radius: len(radius),
        }],
        ids: Vec::new(),
        on: None,
    }));
    let disc = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(depth),
    });
    let axis = axis_of(&env_str("M3_AXIS", "y"));
    let copy = r.in_world_by(
        disc,
        fixture::turned_to([0.0, 0.0, 0.0], &[(axis, named("theta", Dimension::Angle))]),
    );
    // A taker for the turned copy: the union of it and a second copy
    // standing clear of it, unturned (a one-member union runs nothing).
    if env_str("M3_TAKER", "none") == "union" {
        let clear = r.in_world_by(disc, fixture::turned_to([20.0 * radius, 0.0, 0.0], &[]));
        let u = r.insert(Node::Union {
            // The union is built in its first member's space: with the
            // turned copy first, the body that meets the boxed turn is
            // the clear one (its centre swings); `M3_ORDER=clear` puts
            // the clear copy first, so the turned copy meets it about
            // its own centre.
            members: Bodies::Spelled(if env_str("M3_ORDER", "copy") == "clear" {
                vec![clear.into(), copy.into()]
            } else {
                vec![copy.into(), clear.into()]
            }),
        });
        eprintln!(
            "[m3] turned disc nodes: disc={} copy={} clear={} union={}",
            disc.0, copy.0, clear.0, u.0
        );
    }
    r.assemble_world_copies();
    r.doc
}

#[test]
#[ignore = "probe: prints, asserts nothing; BOXFINISH_TRACE=1 cargo test m3_turned_disc -- --ignored --nocapture"]
fn m3_turned_disc() {
    report("turned disc", &turned_disc());
}

/// **Item 1, through the doors**: the extrude door handed a sketch plane
/// that is itself turned by `theta` about `M3_AXIS`, then taken
/// (`AtRestPolicy::take`: the finish at the taker with the switch on, the
/// held finish with it off).
fn door<T>(label: &str, theta: T)
where
    T: geom_core::Decide + topo::AtRestPolicy,
{
    let tol = Tol::witness();
    let radius = env_f64("M3_RADIUS", 5e-3);
    let depth = env_f64("M3_DEPTH", radius);
    let c = |x: f64| T::from_f64(x);
    let axis = match env_str("M3_AXIS", "y").as_str() {
        "x" => UnitVec3::<T>::exact_x(),
        "y" => UnitVec3::<T>::exact_y(),
        _ => UnitVec3::<T>::exact_z(),
    };
    let turn = RigidMotion::rotation(axis, theta).expect("a turn");
    eprintln!("[m3] {label}: the plane's frame {turn:?}");
    let rim = RawLoop::new([(
        Point2::new(c(radius), c(0.0)),
        Segment::Arc(Arc2 {
            centre: Point2::new(c(0.0), c(0.0)),
            radius: c(radius),
            sweep: c(TAU),
        }),
    )]);
    let profile = match Profile::new(SketchPlane::from_frame(turn), vec![rim]).validate(tol) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[m3] {label}: the profile refused: {e:?}");
            return;
        }
    };
    let out = extrude(
        &profile,
        Extrusion::Distance {
            depth: c(depth),
            side: ExtrudeSide::Along,
        },
        tol,
    );
    match out {
        Err(e) => {
            let text = format!("{e:?}");
            eprintln!(
                "[m3] {label}: the DOOR refused: {}",
                text.chars().take(3000).collect::<String>()
            );
        }
        Ok(out) => match T::take(&Arc::new(out.body), tol) {
            Ok(_) => eprintln!("[m3] {label}: built and TAKEN finished"),
            Err(e) => {
                let text = format!("{e:?}");
                eprintln!(
                    "[m3] {label}: the TAKE refused: {}",
                    text.chars().take(3000).collect::<String>()
                );
            }
        },
    }
}

#[test]
#[ignore = "probe: prints, asserts nothing; BOXFINISH_TRACE=1 cargo test m3_turned_plane_door -- --ignored --nocapture"]
fn m3_turned_plane_door() {
    let theta = env_f64("M3_THETA_DEG", 30.0).to_radians();
    let delta = env_f64("M3_DELTA_DEG", 1.0).to_radians();
    let (lo, hi) = (theta - delta, theta + delta);
    door::<Interval>("door Interval", Interval::from_bounds(lo, hi));
    let budget = geom_core::SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    };
    let ((), counts) = geom_core::sym::with_session(budget, || {
        let th = Sym::param_over(
            geom_core::ParamSymbol::new(7),
            Interval::from_bounds(lo, hi),
            lo,
            hi,
        );
        door::<Sym<Interval>>("door Sym<Interval>", th);
    });
    eprintln!("[m3] door Sym<Interval>: counts {counts:?}");
}

/// **Item 2**: the tour's cut plate (`demos/tour/src/plate.rs`,
/// `author` with `cut`), without its measure and assertion: the blank,
/// two hole extrudes each placed on the blank's space by an unmated
/// bundle with no value, two subtractions, the cut part placed in the
/// world and made the product. The box is `M3_SCALE` of the tour's study
/// (±0.05 mm on the half spacing, σ = 0.01 mm on each radius).
fn cut_plate() -> ProfileDoc {
    let s = env_f64("M3_SCALE", 1.0);
    let mut r = Recorder::new();
    declare(
        &mut r,
        "half_spacing",
        Dimension::Length,
        3.1e-3 / 2.0,
        uniform(5.0e-5 * s),
    );
    for n in ["hole_a_r", "hole_b_r"] {
        declare(
            &mut r,
            n,
            Dimension::Length,
            1.25e-3,
            Distribution::Normal { sigma: 1.0e-5 * s },
        );
    }
    let plate_profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([
                (-4.0e-3, -2.0e-3),
                (4.0e-3, -2.0e-3),
                (4.0e-3, 2.0e-3),
                (-4.0e-3, 2.0e-3),
            ])
            .expect("finite plate corners"),
        ],
        ids: Vec::new(),
        on: None,
    }));
    let blank = r.insert(Node::Extrude {
        profile: plate_profile.into(),
        distance: len(1.0e-3),
    });
    let hole = |r: &mut Recorder, centre: Formula, radius: &'static str| {
        let profile = r.insert(Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [centre, Formula::zero(Dimension::Length)],
                radius: named(radius, Dimension::Length),
            }],
            ids: Vec::new(),
            on: None,
        }));
        let hole = r.insert(Node::Extrude {
            profile: profile.into(),
            distance: len(1.0e-3),
        });
        let space = r
            .doc
            .operand_space(&Operand::Node(blank))
            .expect("the blank is in a space");
        r.push(DocEdit::place(
            BodyRead::plain(Operand::Node(hole)),
            Operand::Var(space),
            editor_core::Placed::unmated(Vec::new()).bundle(),
        ))
        .expect("a placement mints an id")
    };
    let hole_a = hole(
        &mut r,
        Formula::neg(named("half_spacing", Dimension::Length)).expect("a length"),
        "hole_a_r",
    );
    let hole_b = hole(&mut r, named("half_spacing", Dimension::Length), "hole_b_r");
    // `M3_CUT=0`: the tour's study document's shape (item 3): the holes
    // left as placed extrudes, the blank the product.
    if env_str("M3_CUT", "1") == "0" {
        let placed = r.place(blank);
        r.push(DocEdit::SetProduct {
            product: Some(Operand::Node(placed)),
        });
        eprintln!(
            "[m3] uncut plate nodes: blank={} hole_a(placed)={} hole_b(placed)={} placed={}",
            blank.0, hole_a.0, hole_b.0, placed.0
        );
        return r.doc;
    }
    let drilled = r.insert(Node::Subtract {
        from: blank.into(),
        tool: hole_a.into(),
    });
    let part = r.insert(Node::Subtract {
        from: drilled.into(),
        tool: hole_b.into(),
    });
    let placed = r.place(part);
    r.push(DocEdit::SetProduct {
        product: Some(Operand::Node(placed)),
    });
    eprintln!(
        "[m3] cut plate nodes: blank={} hole_a(placed)={} hole_b(placed)={} drilled={} part={} placed={}",
        blank.0, hole_a.0, hole_b.0, drilled.0, part.0, placed.0
    );
    r.doc
}

#[test]
#[ignore = "probe: prints, asserts nothing; BOXFINISH_TRACE=1 cargo test m3_cut_plate -- --ignored --nocapture"]
fn m3_cut_plate() {
    report("cut plate", &cut_plate());
}

/// **Item 4**: a rectangle whose WIDTH is `1 ± M3_HALF`, its left edge
/// at `x0 = 0 ± M3_OFFSET_HALF` (no variable when that is 0), depth 1
/// fixed. `M3_FORM`: `poly` (four corners as formulas), `rect` (the
/// rectangle verb, which is centred), `pen` (headings and lengths: every
/// line's direction is a constant of the program).
fn wide_slab() -> ProfileDoc {
    let half = env_f64("M3_HALF", 0.05);
    let off = env_f64("M3_OFFSET_HALF", 0.0);
    let mut r = Recorder::new();
    declare(&mut r, "w", Dimension::Length, 1.0, uniform(half));
    let w = || named("w", Dimension::Length);
    let x0 = || {
        if off == 0.0 {
            len(0.0)
        } else {
            named("x0", Dimension::Length)
        }
    };
    if off != 0.0 {
        declare(&mut r, "x0", Dimension::Length, 0.0, uniform(off));
    }
    let x1 = || {
        if off == 0.0 {
            w()
        } else {
            Formula::add(x0(), w()).expect("a length")
        }
    };
    let quarter = || ang(core::f64::consts::FRAC_PI_2);
    let lp = match env_str("M3_FORM", "poly").as_str() {
        "poly" => LoopProgram::polygon_expr([
            [x0(), len(0.0)],
            [x1(), len(0.0)],
            [x1(), len(1.0)],
            [x0(), len(1.0)],
        ]),
        "rect" => LoopProgram::Rectangle {
            centre: [x0(), len(0.0)],
            width: w(),
            height: len(1.0),
        },
        "pen" => LoopProgram::Chain(vec![
            ProgramStep::At([x0(), len(0.0)]),
            ProgramStep::Angle(ang(0.0)),
            ProgramStep::Line(w()),
            ProgramStep::Turn(quarter()),
            ProgramStep::Line(len(1.0)),
            ProgramStep::Turn(quarter()),
            ProgramStep::Line(w()),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]),
        other => panic!("M3_FORM is poly, rect or pen: {other}"),
    };
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![lp],
        ids: Vec::new(),
        on: None,
    }));
    let _: RecipeNodeId = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    r.doc
}

#[test]
#[ignore = "probe: prints, asserts nothing; BOXFINISH_TRACE=2 cargo test m3_wide_slab -- --ignored --nocapture"]
fn m3_wide_slab() {
    report("wide slab", &wide_slab());
}
