//! **The at-rest census's idealized/realized differential** (PERF-PLAN
//! §4.4; the census's `Candidates`, the boolean sweep's
//! `m5_pr8_bvh_diff` one door over). The same product body goes
//! through the census under both strategies, and the pins are:
//!
//! 1. **Bit-equal**: the error vector — kinds, entities, order,
//!    witnesses — is `Debug`-byte-identical through either strategy.
//!    The K-funnel's verdict log is NOT compared, for the reason the
//!    boolean twin scrubs it: it records which predicates ran, and a
//!    pre-filter exists to run fewer.
//! 2. **Superset**: per sweep, the realized examined set holds every
//!    idealized ACCEPTED pair (a pair whose examination pushed a
//!    finding) — no finding is ever lost to the filter.
//! 3. **Order**: per sweep, the realized examined sequence is the
//!    idealized one with pairs removed and nothing reordered — the
//!    decision order the verdict order and the error order ride.
//! 4. **Not vacuous**: the adversarial rows and the corpus as a whole
//!    prune something, so the pins above are exercised on real
//!    candidate loss.
//!
//! The adversarial rows are deliberately NOT axis-aligned bricks
//! (PERF-PLAN §2.1's gap): a torus resting on a cylinder's cap, a
//! three-arc boss standing on a plate, two cylinders tangent along a
//! line, a NURBS-walled loft with a brick flush on its cap, and an
//! L-shaped face with an edge that crosses its box through the notch
//! (a candidate the box keeps and the exact predicate rejects). The
//! corpus rows run every registered document, and one fin row runs the
//! heat sink at 10 fins — the 40- and 160-fin bodies' order pin rides
//! on the goldens (`perf12_census_goldens`), which run them at every ε
//! row. A planted row shows the comparator can go red.
//!
//! The comparators (restriction of the examined sequence, lost accepted
//! pairs, the sweep roster) are [`SweepPairs`]'s and [`CensusTrace`]'s
//! own; `census.rs`'s unit rows read the same ones, and their `pin` is
//! this file's over an operator-built body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use corpus::{documents, eval, failures};
use editor_core::{
    Datum, Dimension, DocEdit, DocParam, Expr, LoopProgram, Node, ParamName, ProfileDoc,
    ProfileProgram, TubeWindow, apply, product_recorded,
};
use fixture::{Recorder, band, frame, len, xy_frame};
use geom_core::Tol;
use sweep::test_support::{PRISM_SQUARE, PRISM_TRAPEZOID};
use topo::{
    CensusStrategy, CensusTrace, EntityId, PlantedDegradation, RegionLane, census_traces,
    census_traces_planted,
};

/// One strategy's run: the error vector rendered, and the trace.
struct Run {
    errors: Vec<String>,
    trace: CensusTrace,
}

fn run(name: &str, doc: &ProfileDoc, strategy: CensusStrategy) -> Run {
    let tol = Tol::witness();
    let ev = eval::<f64>(doc);
    let bad = failures(&ev);
    assert!(
        bad.is_empty(),
        "{name}: evaluation failed:\n{}",
        bad.join("\n")
    );
    let product = product_recorded(doc, &ev, tol)
        .unwrap_or_else(|e| panic!("{name}: the product gathers: {e:?}"));
    let (errors, trace) = census_traces(
        &product.body,
        &product.contacts,
        band(),
        tol,
        Some(RegionLane::certified()),
        strategy,
    );
    Run {
        errors: errors.iter().map(|e| format!("{e:?}")).collect(),
        trace,
    }
}

/// The three pins on one document; returns the number of pairs the
/// filter pruned, summed over the sweeps.
fn pin(name: &str, doc: &ProfileDoc) -> usize {
    let real = run(name, doc, CensusStrategy::Realized);
    let ideal = run(name, doc, CensusStrategy::Idealized);
    assert_eq!(
        real.errors, ideal.errors,
        "{name}: the error vector differs between the realized and idealized census"
    );
    let mut pruned = 0;
    for ((sweep, r), (_, i)) in real.trace.sweeps().iter().zip(ideal.trace.sweeps().iter()) {
        assert!(
            r.is_restriction_of(i),
            "{name}/{sweep}: the realized sweep's order is not the exact order restricted"
        );
        let lost = r.lost_accepted(i);
        assert!(
            lost.is_empty(),
            "{name}/{sweep}: the filter pruned pairs the exact sweep decided against: {lost:?}"
        );
        assert_eq!(
            r.accepted, i.accepted,
            "{name}/{sweep}: the accepted pairs differ"
        );
        pruned += i.examined.len() - r.examined.len();
    }
    pruned
}

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite")
}

/// A solid ring torus (R = 2, r = 0.5, axis z through the origin)
/// resting on the top cap of a three-arc cylinder (radius 3, z ∈
/// [-1.5, -0.5]): the torus's lowest circle lies on the planar cap.
fn torus_on_cylinder() -> ProfileDoc {
    let mut r = Recorder::new();
    let spine = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scalar(0.0), scalar(0.0), scalar(1.0)],
    }));
    r.insert(Node::Tube {
        spine,
        u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
        major_radius: len(2.0),
        window: TubeWindow::Full,
        minor_radius: len(0.5),
    });
    let plane = r.insert(frame([0.0, 0.0, -1.5], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let disc = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::circle_split(0.0, 0.0, 3.0, 3, 0.0).unwrap()],
    }));
    r.insert(Node::Extrude {
        profile: disc,
        distance: len(1.0),
    });
    r.doc
}

/// A 3×3×0.8 plate and a three-arc boss (r = 0.35) standing on it,
/// undeclared: the boss's cap rests flush on the plate's top, and the
/// boss's arc-joint vertices lie in the plate's top face.
fn boss_on_plate() -> ProfileDoc {
    let mut r = Recorder::new();
    let plate_plane = r.insert(xy_frame());
    let plate = r.insert(Node::Profile(ProfileProgram {
        plane: plate_plane,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]).unwrap(),
        ],
    }));
    r.insert(Node::Extrude {
        profile: plate,
        distance: len(0.8),
    });
    let boss_plane = r.insert(frame([0.0, 0.0, 0.8], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let boss = r.insert(Node::Profile(ProfileProgram {
        plane: boss_plane,
        loops: vec![LoopProgram::circle_split(1.2, 1.7, 0.35, 3, 0.0).unwrap()],
    }));
    r.insert(Node::Extrude {
        profile: boss,
        distance: len(1.0),
    });
    r.doc
}

/// Two unit-radius cylinders (z ∈ [0, 1]) whose walls are tangent
/// along the line x = 1: curved × curved, touching at a curved seat.
fn tangent_cylinders() -> ProfileDoc {
    let mut r = Recorder::new();
    for cx in [0.0, 2.0] {
        let plane = r.insert(xy_frame());
        let disc = r.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::circle_split(cx, 0.0, 1.0, 3, 0.0).unwrap()],
        }));
        r.insert(Node::Extrude {
            profile: disc,
            distance: len(1.0),
        });
    }
    r.doc
}

/// `loft_prism`'s NURBS-walled loft (squares at z = 0 and 2, a
/// trapezoid at z = 1) with a brick standing flush on its top cap.
fn loft_with_brick() -> ProfileDoc {
    let mut r = Recorder::new();
    let section = |r: &mut Recorder, z: f64, pts: [(f64, f64); 4]| {
        let plane = r.insert(frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        r.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::polygon(pts).unwrap()],
        }))
    };
    let bottom = section(&mut r, 0.0, PRISM_SQUARE);
    let middle = section(&mut r, 1.0, PRISM_TRAPEZOID);
    let top = section(&mut r, 2.0, PRISM_SQUARE);
    r.insert(Node::Loft {
        profiles: vec![bottom, middle, top],
        v_degree: Expr::count(2),
    });
    let brick_plane = r.insert(frame([0.0, 0.0, 2.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let brick = r.insert(Node::Profile(ProfileProgram {
        plane: brick_plane,
        loops: vec![
            LoopProgram::polygon([(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]).unwrap(),
        ],
    }));
    r.insert(Node::Extrude {
        profile: brick,
        distance: len(1.0),
    });
    r.doc
}

/// An L-shaped prism (the notch at x, y > 1) and a small brick whose
/// bottom edges lie in the L's top plane INSIDE the L's box but inside
/// the notch: the box keeps every edge×face pair, the exact region
/// test rejects them all.
fn grazing_notch() -> ProfileDoc {
    let mut r = Recorder::new();
    let l_plane = r.insert(xy_frame());
    let l = r.insert(Node::Profile(ProfileProgram {
        plane: l_plane,
        loops: vec![
            LoopProgram::polygon([
                (0.0, 0.0),
                (2.0, 0.0),
                (2.0, 1.0),
                (1.0, 1.0),
                (1.0, 2.0),
                (0.0, 2.0),
            ])
            .unwrap(),
        ],
    }));
    r.insert(Node::Extrude {
        profile: l,
        distance: len(1.0),
    });
    let brick_plane = r.insert(frame([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let brick = r.insert(Node::Profile(ProfileProgram {
        plane: brick_plane,
        loops: vec![
            LoopProgram::polygon([(1.25, 1.25), (1.75, 1.25), (1.75, 1.75), (1.25, 1.75)]).unwrap(),
        ],
    }));
    r.insert(Node::Extrude {
        profile: brick,
        distance: len(1.0),
    });
    r.doc
}

fn adversarial() -> Vec<(&'static str, ProfileDoc)> {
    vec![
        ("torus_on_cylinder", torus_on_cylinder()),
        ("boss_on_plate", boss_on_plate()),
        ("tangent_cylinders", tangent_cylinders()),
        ("loft_with_brick", loft_with_brick()),
        ("grazing_notch", grazing_notch()),
    ]
}

#[test]
fn adversarial_rows_bit_equal_superset_and_ordered() {
    let mut total = 0;
    for (name, doc) in adversarial() {
        let pruned = pin(name, &doc);
        assert!(
            pruned > 0,
            "{name}: the filter pruned nothing — the row exercises no candidate loss"
        );
        total += pruned;
    }
    assert!(total > 0);
}

#[test]
fn the_grazing_notch_keeps_the_candidate_the_exact_predicate_rejects() {
    // The brick's four bottom edges against the L's top face: every
    // pair survives the filter (the brick sits inside the L's box) and
    // every one is rejected exactly (the notch is not the face).
    let doc = grazing_notch();
    let real = run("grazing_notch", &doc, CensusStrategy::Realized);
    let ideal = run("grazing_notch", &doc, CensusStrategy::Idealized);
    assert_eq!(real.errors, ideal.errors);
    assert!(
        real.trace.ef.examined.len() >= 4,
        "the brick's four bottom edges lie in the L's top face's box and survive the filter"
    );
    assert!(
        real.trace.ef.examined.len() < ideal.trace.ef.examined.len(),
        "the faces whose boxes the brick clears are pruned"
    );
    assert!(real.trace.ef.accepted.is_empty(), "nothing touches");
    assert!(
        real.errors.is_empty(),
        "the notch scene is at rest with no contact: {:?}",
        real.errors
    );
}

/// The corpus heat sink with its fin count driven to `fins`.
fn heatsink_at(fins: i64) -> ProfileDoc {
    let entry = documents()
        .into_iter()
        .find(|d| d.name == "heat_sink")
        .expect("the corpus carries the heat sink");
    apply(
        &entry.doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("fins"),
            value: DocParam::Count { value: fins },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the fin count is a document parameter")
    .doc
}

#[test]
fn the_ten_fin_heat_sink_is_bit_equal_superset_and_ordered() {
    let pruned = pin("heat_sink@10", &heatsink_at(10));
    assert!(pruned > 0, "eleven solids prune");
}

/// The comparator can go red: one face box planted empty, and the
/// superset pin MUST report the loss (`m5_pr8_bvh_diff`'s pin 3).
#[test]
fn planted_degradation_is_caught() {
    let doc = boss_on_plate();
    let tol = Tol::witness();
    let ev = eval::<f64>(&doc);
    let product = product_recorded(&doc, &ev, tol).expect("the boss gathers");
    let (_, ideal) = census_traces(
        &product.body,
        &product.contacts,
        band(),
        tol,
        Some(RegionLane::certified()),
        CensusStrategy::Idealized,
    );
    let &(_, EntityId::Face(face)) = ideal
        .vf
        .accepted
        .first()
        .expect("the boss's arc joints rest in the plate's top face")
    else {
        panic!("a vf pair names a face");
    };
    let (_, real) = census_traces_planted(
        &product.body,
        &product.contacts,
        band(),
        tol,
        Some(RegionLane::certified()),
        CensusStrategy::Realized,
        PlantedDegradation { face },
    );
    let lost = real.vf.lost_accepted(&ideal.vf);
    assert!(
        lost.iter().any(|&(_, f)| f == EntityId::Face(face)),
        "a planted-empty face box must lose its accepted pairs (got {lost:?})"
    );
}

#[test]
fn corpus_rows_bit_equal_superset_and_ordered() {
    let mut total = 0;
    for d in documents() {
        total += pin(d.name, &d.doc);
    }
    assert!(total > 0, "the corpus exercises the filter");
}
