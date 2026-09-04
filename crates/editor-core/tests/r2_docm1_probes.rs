//! **Review lane R2's probes for DOCM-1.** Independent rows for C1,
//! C2 and the amendment's C7 emphasis (ii). Not part of the unit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{self, Recorder, ang, len};

use editor_core::{
    CancelToken, CapEnd, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Expr,
    Node, ParamName, ProfileDoc, RecipeNodeId, RoleSeg, UnitSym, ValuePayload, evaluate,
};
use geom_core::{Tol, Vec3};
use topo::{DatumValue, UnitVec3, readback};

fn ev64(doc: &ProfileDoc, prior: Option<&editor_core::Evaluation<f64>>) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn frame_of(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
) -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    let ValuePayload::Datum(DatumValue::Frame { origin, u, v }) =
        &ev.value(node).expect("the frame evaluated").payload
    else {
        panic!("a frame value");
    };
    (
        *origin - geom_core::Point3::origin(),
        UnitVec3::get(*u),
        UnitVec3::get(*v),
    )
}

// ---------------------------------------------------------------------
// C1 — a DIFFERENT body and a DIFFERENT edit than the unit's A1.
// A doc-PARAMETER drives the box's height (A1 uses SetParam on the
// slot), the frame carries a non-zero spin, and the boss is a second
// extrude on it.
// ---------------------------------------------------------------------

fn param_box_with_boss() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("h"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 1.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: None,
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
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
        spin: ang(0.4),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.25)],
    )));
    let boss = r.insert(Node::Extrude {
        profile: boss_p,
        distance: len(0.5),
    });
    (r.doc, cube, frame, boss)
}

#[test]
fn r2_c1_a_doc_param_edit_moves_the_frame_and_recomputes_exactly_the_cone() {
    let (doc, _cube, frame, boss) = param_box_with_boss();
    let ev = ev64(&doc, None);
    assert!(corpus::failures(&ev).is_empty(), "{:?}", corpus::failures(&ev));
    let (o, _, _) = frame_of(&ev, frame);
    assert!((o.z - 1.0).abs() <= 1e-12, "origin {o:?}");

    let bumped = editor_core::apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("h"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 2.5,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            },
        },
        Tol::witness(),
    )
    .expect("a doc-param edit applies")
    .doc;
    let ev2 = ev64(&bumped, Some(&ev));
    assert!(
        corpus::failures(&ev2).is_empty(),
        "{:?}",
        corpus::failures(&ev2)
    );
    let (o2, _, _) = frame_of(&ev2, frame);
    assert!((o2.z - 2.5).abs() <= 1e-12, "the frame followed: {o2:?}");
    let bottom = corpus::body_of(&ev2, boss)
        .points()
        .map(|(_, p)| p.z)
        .fold(f64::INFINITY, f64::min);
    assert!((bottom - 2.5).abs() <= 1e-12, "the boss followed: {bottom}");

    // The cone of the parameter: the box extrude and everything above.
    // The frame's plane node and the box's own profile read no `h`.
    assert_eq!(ev2.reused, 2, "the two parameter-free leaves reuse");
    assert_eq!(ev2.recomputed, bumped.len() - 2, "exactly the cone");
}

// ---------------------------------------------------------------------
// C2 — handedness, on a sense-`false` face, at a spin the unit did not
// choose, checked WITHOUT restating the implementation's formula.
// ---------------------------------------------------------------------

/// A washer (two plane annuli of opposite sense), its revolve node.
fn washer() -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane, p) = fixture::on_frame_keeping(
        ProfileDoc::empty_derived("r2_washer", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let (doc, axis) = fixture::insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    fixture::insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    )
}

#[test]
fn r2_c2_spin_turns_right_handed_about_the_outward_normal_on_a_reversed_face() {
    let theta = 0.3_f64;
    let (doc, node) = washer();
    let ev = ev64(&doc, None);
    let ValuePayload::Body(body) = &ev.value(node).expect("the washer").payload else {
        panic!("a body");
    };
    let mut seen = [false, false];
    let planar: Vec<_> = body
        .faces()
        .filter(|(k, _)| {
            readback::face_carrier_kind(body, *k) == Ok(geom_brep::SurfaceKind::Plane)
        })
        .map(|(k, f)| (k, f.sense))
        .collect();
    for (key, sense) in planar {
        let pose = readback::face_pose(body, key).expect("a plane has a pose");
        let name = editor_core::all_faces(&ev, node)
            .into_iter()
            .find(|n| {
                matches!(
                    ev.value(node).expect("evaluated").name_table.lookup(n),
                    Some(editor_core::Entry::Unique(e))
                        if e.key == editor_core::EntityKey::Face(key)
                )
            })
            .expect("named");
        let (doc2, frame) = fixture::insert(
            doc.clone(),
            Node::Datum(Datum::FaceFrame {
                at: node,
                face: name,
                spin: ang(theta),
            }),
        );
        let ev2 = ev64(&doc2, None);
        let (_, u, v) = frame_of(&ev2, frame);
        let n = u.cross(v);
        let u_ref = pose.u_ref.expect("a plane fixes u_ref");
        let axis = pose.axis;

        // 1. The outward normal is the SENSE times the chart axis.
        let want_n = if sense { axis } else { -axis };
        for (a, b) in [(n.x, want_n.x), (n.y, want_n.y), (n.z, want_n.z)] {
            assert!((a - b).abs() <= 1e-12, "n {n:?} vs {want_n:?}");
        }
        // 2. The turn is by theta: cos from the projection...
        assert!(
            (u.dot(u_ref) - theta.cos()).abs() <= 1e-12,
            "|u·u_ref| = {} want cos = {}",
            u.dot(u_ref),
            theta.cos()
        );
        // 3. ...and RIGHT-HANDED ABOUT n, not about the chart axis:
        //    (u_ref × u)·n = +sin θ. On a sense-false face this has
        //    the OPPOSITE sign to (u_ref × u)·axis, which is what
        //    separates "about the outward normal" from "about the
        //    chart axis" — the two agree on every sense-true face.
        let s = u_ref.cross(u);
        assert!(
            (s.dot(n) - theta.sin()).abs() <= 1e-12,
            "sense={sense}: (u_ref×u)·n = {} want +sin θ = {}",
            s.dot(n),
            theta.sin()
        );
        if !sense {
            assert!(
                (s.dot(axis) + theta.sin()).abs() <= 1e-12,
                "on a reversed face the turn is the other way about the CHART axis"
            );
        }
        seen[usize::from(sense)] = true;
    }
    assert_eq!(seen, [true, true], "both senses exercised");
}

// ---------------------------------------------------------------------
// C7 emphasis (ii) — the PR's kernel claim, checked on its own: does
// an Interval extrude of a WIDENED height really refuse at every eps?
// No frame anywhere in this document.
// ---------------------------------------------------------------------

#[cfg(feature = "interval")]
#[test]
fn r2_c7ii_an_interval_extrude_of_a_widened_height() {
    for div in [8.0, 10.0, 30.0, 100.0, 300.0, 1.0e3, 1.0e4, 1.0e6] {
        c7ii_at(Tol::witness().eps() / div);
    }
    c7ii_at(0.0);
}

#[cfg(feature = "interval")]
fn c7ii_at(width: f64) {
    use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
    use geom_core::Interval;
    use std::sync::Arc;

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
    let cube = r.insert(Node::Extrude {
        profile,
        distance: Expr::param(ParamName::new("hh"), Dimension::Length),
    });
    let doc = r.doc;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let ev = evaluate::<Interval>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    let failures = corpus::failures(&ev);
    println!(
        "R2 c7(ii): eps={:e} width={width:e} green={} failures={}",
        Tol::witness().eps(),
        failures.is_empty(),
        failures.len()
    );
    if !failures.is_empty() {
        println!("    {}", failures[0]);
    }
    let _ = cube;
    assert!(ev.nodes.len() >= 3);
}

/// **The Transform detour was not forced.** The unit's widening row
/// (`docm1_face_frame_interval.rs:138`) lifts an exact box by a
/// widened rigid `Transform` because, it says, "an interval extrude
/// re-certifies its carriers against a band scaled to the row's ε, and
/// a height bracket of ANY width leaves that band". This row is the
/// direct spelling — the widened parameter IS the extrude's height,
/// the frame is on the cap it moves — at ε/10 rather than ε/8, and it
/// is green at every ε row while still carrying the width into the
/// derived frame's placement and recomputing the profile.
#[cfg(feature = "interval")]
#[test]
fn r2_c7ii_a_widened_extrude_height_carries_the_frame_at_one_tenth_eps() {
    use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
    use geom_core::{Bounds, Interval};
    use std::sync::Arc;

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
        face: fixture::fname(cube, RoleSeg::Cap(CapEnd::Top)),
        spin: ang(0.0),
    }));
    let boss_p = r.insert(Node::Profile(fixture::desc(
        frame,
        vec![fixture::square(0.0, 0.0, 0.25)],
    )));
    let doc = r.doc;

    let nominal = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
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
    let widened = evaluate::<Interval>(&doc, Some(&nominal), &CancelToken::new(), &opts, Tol::witness());
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
