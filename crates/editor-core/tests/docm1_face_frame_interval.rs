//! **DOCM-1 under the Interval lane** — DM1c's rows: a profile on a
//! derived frame is placed at the lane scalar under EVERY lift (its
//! 2-D structure record still `f64`-pinned), a loft section on a
//! derived frame refuses typed off `f64`, and a widened parameter the
//! frame's body reads recomputes the profile rather than serving the
//! nominal memo entry.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;
use crate::docm1_face_frame::lofted_on_face_frame;
use crate::fixture::{self, Recorder, ang, len, scl};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions,
    Evaluation, Expr, Node, NodeError, NodeErrorKind, NodeResult, ParamName, ProfileDoc,
    ProfileLift, RecipeNodeId, RoleSeg, UnitSym, ValuePayload, evaluate,
};
use geom_core::{Bounds, Interval, Tol};
use topo::{DatumValue, UnitVec3, validate_closed};

fn run(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<Interval>>,
    opts: &EvalOptions,
) -> Evaluation<Interval> {
    evaluate::<Interval>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

fn lifted(lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

/// **The corpus document's profile is placed at the lane scalar under
/// both lifts**: the profile on the derived frame evaluates green at
/// `Interval`, and the boss's bottom encloses the cap's height —
/// which is the frame's landed origin, not a placeholder.
#[test]
fn a_profile_on_a_derived_frame_is_placed_at_the_lane_scalar_under_every_lift() {
    let cd = corpus::face_sketch::document();
    let boss = cd.result.expect("the boss");
    let frame = cd
        .doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(cd.doc.node(*id), Some(Node::Datum(Datum::FaceFrame { .. }))))
        .expect("the derived frame");
    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let ev = run(&cd.doc, None, &lifted(lift));
        assert!(
            corpus::failures(&ev).is_empty(),
            "{lift:?}: {:?}",
            corpus::failures(&ev)
        );
        let ValuePayload::Datum(DatumValue::Frame { origin, u, v }) =
            &ev.value(frame).expect("the frame").payload
        else {
            panic!("a frame value");
        };
        assert!(
            origin.z.lo() <= 1.0 && 1.0 <= origin.z.hi(),
            "{lift:?}: origin {origin:?}"
        );
        let n = UnitVec3::get(*u).cross(UnitVec3::get(*v));
        assert!(
            n.z.lo() <= 1.0 && 1.0 <= n.z.hi(),
            "{lift:?}: outward normal {n:?}"
        );
        let body = corpus::body_of(&ev, boss);
        assert_eq!(validate_closed(body), Ok(()), "{lift:?}");
        let bottom = body
            .points()
            .map(|(_, p)| p.z.lo())
            .fold(f64::INFINITY, f64::min);
        assert!(
            (bottom - 1.0).abs() <= 1e-9,
            "{lift:?}: the boss stands on the cap: {bottom}"
        );
    }
}

/// **A loft section on a derived frame refuses typed off f64**, naming
/// the profile and the frame (the f64 half of the row evaluates, in
/// the f64 suite).
#[test]
fn a_section_on_a_derived_frame_refuses_derived_frame_section_at_interval() {
    let (doc, loft) = lofted_on_face_frame();
    let ev = run(&doc, None, &EvalOptions::default());
    let frame = doc
        .order()
        .iter()
        .copied()
        .find(|id| matches!(doc.node(*id), Some(Node::Datum(Datum::FaceFrame { .. }))))
        .expect("the derived frame");
    let Some(Node::Loft { profiles, .. }) = doc.node(loft) else {
        panic!("the loft");
    };
    let section = profiles[0];
    match ev.nodes.get(&loft) {
        Some(NodeResult::Failed(NodeError {
            kind:
                NodeErrorKind::DerivedFrameSection {
                    profile,
                    frame: named,
                },
            ..
        })) => {
            assert_eq!(*profile, section);
            assert_eq!(*named, frame);
        }
        other => panic!("the loft must refuse typed, got {other:?}"),
    }
    // The profile itself is fine: it is the SECTION use that has no
    // f64 placement, not the profile.
    assert!(matches!(ev.nodes.get(&section), Some(NodeResult::Ok(_))));
}

/// An exact unit box LIFTED by a document parameter `lift` (a rigid
/// transform along +z) whose declared distribution the box's body
/// value then carries, and a boss profile sketched on the lifted
/// body's top face.
///
/// The widened parameter enters through the TRANSFORM here so the
/// row measures the placement alone; the direct spelling — the
/// widened parameter IS the extrude's height — is
/// [`a_widened_extrude_height_carries_the_frame_at_one_tenth_eps`],
/// and the width at which a plain-`Interval` extrude of a widened
/// height certifies is measured by
/// [`an_interval_extrude_of_a_widened_height`] (green at ε/10,
/// refused at ε/8: a floor near ε/16, the one
/// `m10_3_driver_interval.rs` documents).
fn boxed_on_param(width: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("lift"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 0.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -width,
                hi: width,
            }),
        },
    });
    let (plane, profile) = r.profile_keeping(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let _ = plane;
    let cube = r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });
    let lifted = r.insert(Node::Transform {
        input: cube,
        translation: [
            len(0.0),
            len(0.0),
            Expr::param(ParamName::new("lift"), Dimension::Length),
        ],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(0.0),
    });
    // A rigid transform keeps its input's name table verbatim, so the
    // cap is still named by the extrude that minted it.
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: lifted,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::End)),
        spin: ang(0.0),
    }));
    // The document ends at the boss PROFILE: the row below measures the
    // profile's memo and its carried placement. An extrude ABOVE a
    // derived frame whose placement carries width is the open DM1c
    // asymmetry on the symbolic lane (PR #1829's item-1 diagnosis),
    // and is not what this row pins.
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.5)],
    )));
    (r.doc, boss_p)
}

/// **The memo recomputes a profile on a derived frame through the
/// UPSTREAM KEY**: widening the parameter the frame's body reads
/// re-keys the body, hence the frame, hence the profile
/// (`Node::inputs` of a profile names its frame, and a profile's
/// content key folds every input's key in), so the widened placement
/// cannot be served from the nominal memo entry. No placement feed of
/// its own is needed for that, and none exists.
///
/// The widening is SCALED TO THE ROW'S ε (±ε/8) because the hosted
/// matrix runs this row at every ε and the transform-lifted box must
/// certify at each: ε/8 is a width the lifted box's own certification
/// admits on plain `Interval` (the extrude's height is exact here),
/// not the extrude-height floor the ladder row measures.
#[test]
fn widening_the_frames_body_parameter_recomputes_the_profile() {
    let width = Tol::witness().eps() / 8.0;
    let (doc, boss_p) = boxed_on_param(width);
    let nominal = run(&doc, None, &EvalOptions::default());
    assert!(
        corpus::failures(&nominal).is_empty(),
        "{:?}",
        corpus::failures(&nominal)
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let widened = run(&doc, Some(&nominal), &opts);
    assert!(
        corpus::failures(&widened).is_empty(),
        "{:?}",
        corpus::failures(&widened)
    );
    // The box's frame, profile and extrude read no parameter and are
    // served from the memo; everything from the transform up — the
    // lifted body, the derived frame and the boss profile — recomputes.
    assert_eq!(widened.reused, 3, "the three parameter-free leaves");
    assert_eq!(widened.recomputed, doc.len() - 3);
    let ValuePayload::Profile(p) = &widened.value(boss_p).expect("the boss profile").payload else {
        panic!("a profile");
    };
    let z = p.validated.plane().placement.translation.z;
    assert!(
        z.hi() - z.lo() >= 1.9 * width,
        "the placement carries the widened height: {z:?}"
    );
}

/// **The measurement of record for a plain-`Interval` extrude of a
/// WIDENED height** (no frame anywhere): certifies at ε/10 and every
/// narrower width, refuses at ε/8 — a floor near ε/16, at every ε
/// row. The ladder is printed in full; the assertion is the floor.
#[test]
fn an_interval_extrude_of_a_widened_height() {
    let e = Tol::witness().eps();
    let at = |width: f64| -> bool {
        let mut r = Recorder::new();
        r.push(DocEdit::SetDocParam {
            name: ParamName::new("hh"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 1.0,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: if width > 0.0 {
                    Some(Distribution::Uniform {
                        lo: -width,
                        hi: width,
                    })
                } else {
                    None
                },
            },
        });
        let profile = r.profile(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![fixture::square(0.0, 0.0, 1.0)],
        );
        r.insert(Node::Extrude {
            profile,
            distance: Expr::param(ParamName::new("hh"), Dimension::Length),
        });
        let doc = r.doc;
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let opts = EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed))),
            ..EvalOptions::default()
        };
        let ev = run(&doc, None, &opts);
        let fails = corpus::failures(&ev);
        println!(
            "widened extrude height: eps={e:e} width={width:e} green={} {}",
            fails.is_empty(),
            fails.first().map(String::as_str).unwrap_or("")
        );
        fails.is_empty()
    };
    for div in [8.0, 10.0, 30.0, 100.0, 300.0, 1.0e3, 1.0e4, 1.0e6] {
        let green = at(e / div);
        if div >= 10.0 {
            assert!(
                green,
                "a widened extrude height of ε/{div} certifies on plain Interval"
            );
        } else {
            assert!(
                !green,
                "a widened extrude height of ε/{div} refuses on plain Interval"
            );
        }
    }
    assert!(at(0.0), "the exact height certifies");
}

/// **The direct spelling**: the widened parameter IS the extrude's
/// height, the derived frame sits on the cap it moves, the boss
/// profile is on the frame — green at every ε row at ε/10, the
/// placement carries the width, and the profile recomputes through
/// the upstream key.
#[test]
fn a_widened_extrude_height_carries_the_frame_at_one_tenth_eps() {
    let width = Tol::witness().eps() / 10.0;
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("h"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -width,
                hi: width,
            }),
        },
    });
    let profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 1.0)],
    );
    let cube = r.insert(Node::Extrude {
        profile,
        distance: Expr::param(ParamName::new("h"), Dimension::Length),
    });
    let frame = r.insert(Node::Datum(Datum::FaceFrame {
        at: cube,
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::End)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.25)],
    )));
    let doc = r.doc;

    let nominal = run(&doc, None, &EvalOptions::default());
    assert!(
        corpus::failures(&nominal).is_empty(),
        "nominal: {:?}",
        corpus::failures(&nominal)
    );
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let widened = run(&doc, Some(&nominal), &opts);
    assert!(
        corpus::failures(&widened).is_empty(),
        "a widened EXTRUDE HEIGHT of eps/10 certifies green: {:?}",
        corpus::failures(&widened)
    );
    let ValuePayload::Profile(p) = &widened.value(boss_p).expect("the boss profile").payload else {
        panic!("a profile");
    };
    let z = p.validated.plane().placement.translation.z;
    assert!(
        z.hi() - z.lo() >= 1.9 * width,
        "the placement carries the widened height: {z:?}"
    );
    assert_eq!(widened.reused, 2, "the two parameter-free leaves");
    assert_eq!(widened.recomputed, doc.len() - 2);
}

/// **`SectionScalar` answers by type**: `f64` hands its value across,
/// an interval and a dual answer `None` — decided by the impl, no
/// number inspected (an exact interval is still an interval).
#[test]
fn section_scalar_is_decided_by_the_type() {
    use editor_core::SectionScalar;
    assert_eq!(1.5_f64.pinned_f64(), Some(1.5));
    assert_eq!(Interval::from_bounds(1.0, 1.0).pinned_f64(), None);
    assert_eq!(geom_core::Dual64::constant(1.0).pinned_f64(), None);
}
