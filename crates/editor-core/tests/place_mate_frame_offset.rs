//! **A mate frame is a base composed with an offset** (ASSEMBLY.md A3,
//! A11 (5); `[ev]` #3920): the part base or the side's own head face,
//! composed with a `Placement` written in the base's frame.
//!
//! The scenes are `p2_gauges`'s two literal blocks: a 3 x 3 x 1 base
//! and a 1 x 1 x 3 top, each with a corner at its own origin. A cap
//! face's frame stands at the cap's centre; the base's upper cap runs
//! its axis along +z and its reference along +x, which the witness
//! ladder puts on local +Y. The top's lower cap seats on it, outward
//! normals opposed, so with no offset the top's own origin lands at
//! `[1, 1, 1]`, and every offset below is a dyadic length: the frames
//! compared are exact.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::p2_gauges::{Parts, block, body_of, cut, literal, min_corner, parts, set_gauge};
use crate::wire::doctored;

use editor_core::{
    Alignment, AxisSense, ContactClass, Dimension, DocEdit, DocParam, DocParamValue, DocumentId,
    EditError, EvalOptions, Expr, Frame, FrameSite, MateFrame, MatePrimitive, MateRole, MateSide,
    Node, ParamName, PersistError, Placement, ProfileDoc, ProfileProgram, RecipeNodeId, RigidArg,
    SitedFace, SlotId, SnapshotError, SplitError, Step, apply, load, save,
};
use fixture::resolver::with_resolver;
use fixture::round_trip::{composed, same_up_to_ids};
use fixture::{ang, gate, head, insert, len, run, scl, solve, step, step_with};
use geom_core::Tol;

// ---- substrate ----

fn slide() -> ParamName {
    ParamName::from_static("slide")
}

/// A rigid step that only translates, by `t`.
fn shift(t: [Expr; 3]) -> Step {
    Step::Rigid {
        translation: t,
        axis: [0.0, 0.0, 1.0].map(scl),
        angle: ang(0.0),
    }
}

/// The shift along the base cap's local +Y — its reference, world +x —
/// by the document's `slide`.
fn slid_by_the_parameter() -> Placement {
    shift([len(0.0), Expr::param(slide(), Dimension::Length), len(0.0)]).into()
}

/// "Seat the top on the base": the top's lower cap (the mover) on its
/// own face, the base's upper cap on its face composed with `offset`,
/// outward normals opposed.
fn seat(top: SitedFace, base: SitedFace, offset: Placement) -> Node<ProfileProgram> {
    Node::Mate {
        a: top,
        b: base,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::from_face(),
            b: MateFrame::on_face(offset),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// The two-block scene: the base, the top, and the top seated on the
/// base through `offset` — the mate inserted through the store's reach,
/// which resolves its face sides at the door. `prelude` runs on the
/// empty document first (a parameter's declaration).
fn seated(
    label: &str,
    offset: Placement,
    prelude: impl FnOnce(ProfileDoc) -> ProfileDoc,
) -> (Parts, ProfileDoc, [RecipeNodeId; 3]) {
    let p = parts(label);
    let doc = prelude(ProfileDoc::empty(DocumentId::derive(label), Tol::witness()));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let opts = p.opts();
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
    let (doc, m) = step_with(
        doc,
        DocEdit::InsertNode {
            node: Box::new(seat(head(p.top_cap(top)), head(p.base_cap(base)), offset)),
        },
        &reach,
    );
    (p, doc, [base, top, m.expect("the seat is minted")])
}

fn declare_slide(doc: ProfileDoc, value: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParam {
            name: slide(),
            value: DocParam::continuous(Dimension::Length, value),
        },
    )
    .0
}

/// Where the top's own origin landed.
fn top_corner(doc: &ProfileDoc, o: &EvalOptions, top: RecipeNodeId) -> [f64; 3] {
    min_corner(&body_of(&run(doc, o), top))
}

// ---- Row 1: a face side's offset follows the face ----

/// **A face side with an in-plane offset follows the face through a
/// part edit.** Slid half a unit along the base cap's reference, the
/// top sits at `[1.5, 1, 1]`; the base grown to height 2 lifts it to
/// `[1.5, 1, 2]` — the slide kept, the face followed — and the gate
/// certifies the seat both times.
#[test]
fn a_face_side_with_an_in_plane_offset_follows_the_face_through_a_part_edit() {
    let label = "place-mfo-follow";
    let (mut p, doc, [base, top, m]) =
        seated(label, literal([0.0, 0.5, 0.0]), std::convert::identity);
    let o = p.opts();
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(m),
        Some(MateRole::Determining)
    );
    assert_eq!(
        top_corner(&doc, &o, top),
        [1.5, 1.0, 1.0],
        "slid along the face"
    );
    let ev = run(&doc, &o);
    assert!(gate(&doc, &ev).is_ok(), "{:?}", gate(&doc, &ev).err());

    // The base part grows on disk; its names hold, its pin moves.
    let (base_doc, base_body) = block(&format!("{label}-base"), 3.0, 1.0);
    assert_eq!(base_body, p.base_body, "the same base document");
    let (grown, _) = step(
        base_doc,
        DocEdit::SetParam {
            node: base_body,
            slot: SlotId::Distance,
            expr: len(2.0),
        },
    );
    let new_ref = p.store.insert(grown, Tol::witness());
    let o = with_resolver(p.store.clone());
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = step_with(
        doc,
        DocEdit::UpdateReference {
            node: base,
            new_pin: new_ref.pin,
        },
        &reach,
    );
    assert_eq!(
        top_corner(&doc, &o, top),
        [1.5, 1.0, 2.0],
        "the top follows the grown face, still slid"
    );
    let ev = run(&doc, &o);
    assert!(gate(&doc, &ev).is_ok(), "{:?}", gate(&doc, &ev).err());
}

// ---- Row 2: a parameter drives an offset ----

/// **A parameter drives an offset, and the solved pose moves.** The
/// base cap's offset reads the document's `slide`: at 0.5 the top
/// sits at x = 1.5, and one value edit to 1.0 moves it to 2.0 — the
/// evaluation fed the first as its memo re-keys the mate and the top
/// and serves neither from the memo. The parameter is a reading edge
/// for both doors that check one: a mate naming an undeclared
/// parameter refuses at insert, and one reading an angle parameter
/// as a length refuses, each naming the offset's own slot.
#[test]
fn a_parameter_drives_an_offset_and_the_solved_pose_moves() {
    let label = "place-mfo-param";
    let (p, doc, [base, top, m]) =
        seated(label, slid_by_the_parameter(), |d| declare_slide(d, 0.5));
    let o = p.opts();
    let slot = SlotId::MateFrameStep {
        side: MateSide::B,
        step: 0,
        arg: RigidArg::Translation(editor_core::Axis3::Y),
    };
    assert!(
        doc.node(m).expect("the mate").slots().contains(&slot),
        "the offset's components are the mate's slots"
    );
    let first = run(&doc, &o);
    assert_eq!(min_corner(&body_of(&first, top)), [1.5, 1.0, 1.0]);
    let (moved, _) = step(
        doc.clone(),
        DocEdit::SetDocParamValue {
            name: slide(),
            value: DocParamValue::Continuous(1.0),
        },
    );
    let second = editor_core::evaluate::<f64>(
        &moved,
        Some(&first),
        &editor_core::CancelToken::new(),
        &o,
        Tol::witness(),
    );
    assert_eq!(
        min_corner(&body_of(&second, top)),
        [2.0, 1.0, 1.0],
        "the pose moved"
    );
    for (id, what) in [(m, "the mate"), (top, "the top")] {
        assert_ne!(
            first.value(id).map(|v| v.content_key),
            second.value(id).map(|v| v.content_key),
            "{what} re-keys on the parameter its offset reads"
        );
    }
    assert_eq!(
        first.value(base).map(|v| v.content_key),
        second.value(base).map(|v| v.content_key),
        "the base reads nothing that moved"
    );

    // The two parameter doors, at the insert.
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (unmated, _) = step(doc.clone(), DocEdit::DeleteNode { id: m });
    let (unmated, _) = step(
        unmated,
        DocEdit::SetDocParam {
            name: ParamName::from_static("tilt"),
            value: DocParam::continuous(Dimension::Angle, 0.0),
        },
    );
    let named = |name: &'static str, dim| -> Placement {
        shift([
            len(0.0),
            Expr::param(ParamName::from_static(name), dim),
            len(0.0),
        ])
        .into()
    };
    let refused = |offset| {
        apply(
            &unmated,
            &DocEdit::InsertNode {
                node: Box::new(seat(head(p.top_cap(top)), head(p.base_cap(base)), offset)),
            },
            Tol::witness(),
            &reach,
        )
        .expect_err("the insert refuses")
    };
    let err = refused(named("nowhere", Dimension::Length));
    assert!(
        matches!(&err, EditError::SlotUnknownDocParam { slot: s, .. } if *s == slot),
        "{err:?}"
    );
    let err = refused(named("tilt", Dimension::Length));
    assert!(
        matches!(&err, EditError::SlotDocParamDimension { slot: s, .. } if *s == slot),
        "{err:?}"
    );
}

// ---- Row 3: authored vectors are the part base with one literal step ----

/// **An authored side is the part base with one literal step, and
/// gives today's frame bit for bit.** The authored door builds exactly
/// the witness ladder's `point_at` frame as one literal step on the
/// part base, and a frame coincidence of two skew authored sides
/// solves to that frame times the inverse of the other — the
/// representative the coset table formed from the two vector frames —
/// to the last bit.
#[test]
fn an_authored_side_is_the_part_base_with_one_literal_step_bit_for_bit() {
    let tol = Tol::witness();
    let point_at = |o: [f64; 3], a: [f64; 3], r: [f64; 3]| {
        let eye = geom_core::Point3::from_array(o);
        geom_core::linalg::frame::point_at_frame(
            eye,
            eye + geom_core::Vec3::from_array(a),
            geom_core::Vec3::from_array(r),
            tol,
        )
        .expect("a definite frame")
        .to_affine()
    };
    let (oa, aa, ra) = ([0.25, -0.5, 0.75], [3.0, -4.0, 12.0], [0.0, 1.0, 0.0]);
    let (ob, ab, rb) = ([1.0, 2.0, -0.125], [-1.0, 2.0, 2.0], [1.0, 0.0, 0.0]);
    let a = MateFrame::authored(oa, aa, ra, tol).expect("a frame");
    let b = MateFrame::authored(ob, ab, rb, tol).expect("a frame");
    for (frame, o, ax, r) in [(&a, oa, aa, ra), (&b, ob, ab, rb)] {
        let want = Placement::literal(&Frame::from_affine(point_at(o, ax, r)));
        assert!(
            frame.base == editor_core::FrameBase::Part && frame.offset.bit_eq(&want),
            "{frame:?}"
        );
    }

    let label = "place-mfo-bits";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), tol);
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    // No offset of its own, so the base stays the group's root.
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Mate {
                a: head(p.base_cap(base)),
                b: head(p.top_cap(top)),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a,
                    b,
                    primitive: MatePrimitive::FrameCoincidence,
                    sense: AxisSense::Aligned,
                    clocking: None,
                },
            }),
        },
    );
    let placed = solve(&doc, &o, tol).placement(&doc, top).expect("placed");
    let want = Frame::from_affine(point_at(oa, aa, ra) * point_at(ob, ab, rb).inverse());
    assert!(placed.bit_eq(&want), "{placed:?} vs {want:?}");
}

// ---- Row 4: an improper literal step refuses ----

/// **An improper or non-rigid literal step refuses at the edit door,
/// and an improper one at load**, named by its side and step. A mirror
/// in a face side's offset and a scale in it are refused before the
/// mate's admission composes them; a mirror written into a saved file
/// refuses at the load door as a gauge's would.
#[test]
fn an_improper_literal_step_refuses_at_the_door_and_at_load() {
    let label = "place-mfo-improper";
    let (p, doc, [base, top, m]) = seated(label, literal([0.0, 0.5, 0.0]), std::convert::identity);
    let at = FrameSite::MateStep {
        side: MateSide::B,
        index: 0,
    };
    let opts = p.opts();
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
    let (unmated, _) = step(doc.clone(), DocEdit::DeleteNode { id: m });
    let with = |columns: [[f64; 3]; 3]| {
        apply(
            &unmated,
            &DocEdit::InsertNode {
                node: Box::new(seat(
                    head(p.top_cap(top)),
                    head(p.base_cap(base)),
                    Placement::literal(&Frame {
                        columns,
                        translation: [0.0; 3],
                    }),
                )),
            },
            Tol::witness(),
            &reach,
        )
        .expect_err("refused")
    };
    let mirror = [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let err = with(mirror);
    assert!(
        matches!(&err, EditError::ImproperPlacement { at: a, .. } if *a == at),
        "{err:?}"
    );
    let err = with([[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    assert!(
        matches!(&err, EditError::NonRigidPlacement { at: a, .. } if *a == at),
        "{err:?}"
    );

    let text = save(&doc, &[], Tol::witness()).expect("saves");
    load(&text, Tol::witness()).expect("the admitted document loads");
    let mirrored = doctored(&text, |wire| {
        wire["snapshot"]["nodes"][m.0.to_string()]["Mate"]["alignment"]["b"]["offset"]["steps"]
            [0]["Literal"]["columns"] = serde_json::json!(mirror);
    });
    let err = load(&mirrored, Tol::witness()).expect_err("a mirror refuses at load");
    assert!(
        matches!(
            &err,
            PersistError::Snapshot(SnapshotError::PlacementImproper { at: a, .. }) if *a == at
        ),
        "{err:?}"
    );
}

// ---- Row 5: a Rest side set back from its face ----

/// **A `Rest` side set back along the normal declares a contact the
/// gate refutes.** The offset is any rigid motion and the solve places
/// the top a quarter unit above the base, as asked; the contact class
/// says which offsets are legal, and the at-rest gate refutes the
/// declared rest.
#[test]
fn a_rest_side_set_back_along_the_normal_is_refuted_by_the_gate() {
    let (p, doc, [_, top, m]) = seated(
        "place-mfo-setback",
        literal([0.0, 0.0, 0.25]),
        std::convert::identity,
    );
    let o = p.opts();
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(m),
        Some(MateRole::Determining)
    );
    assert_eq!(
        top_corner(&doc, &o, top),
        [1.0, 1.0, 1.25],
        "placed as asked"
    );
    let ev = run(&doc, &o);
    let err = gate(&doc, &ev).expect_err("the set-back rest is refuted");
    let editor_core::AssemblyError::AtRest { findings } = &err else {
        panic!("{err:?}");
    };
    assert!(
        findings.iter().any(|f| matches!(
            &f.attribution,
            editor_core::Attribution::Refuted(d) if d.mate == m
        )),
        "{err:?}"
    );
}

// ---- Row 6: the offset and its parameter across the seam ----

/// **The offset and its parameter cross split and inline.** A cut
/// seat whose face offset reads `slide` carries the parameter into the
/// part, inline merges it back, and `inline(split(d))` is `d` up to
/// node ids — the offset included — with the top where it was. A kept
/// mate reading `slide` beside a cut one that does not keeps the
/// parameter in the remainder alone; both reading it is the uncut
/// reference split refuses.
#[test]
fn the_offset_and_its_parameter_cross_split_and_inline() {
    let label = "place-mfo-seam";
    let (p, doc, [base, top, m]) =
        seated(label, slid_by_the_parameter(), |d| declare_slide(d, 0.5));
    let o = p.opts();
    let before = top_corner(&doc, &o, top);
    let out = editor_core::split(
        &doc,
        &cut(&[base, top, m]),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("split refused: {e}"));
    assert!(
        out.part.params().contains_key(&slide()),
        "the cut seat's parameter goes with it"
    );
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &crate::p2_gauges::resolver(store),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("inline refused: {e}"));
    let (map, steps) = composed(&doc, &out, &back);
    same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("inline(split(d)) is d up to node ids:\n{e}"));
    assert_eq!(
        top_corner(&back.doc, &o, top_image(&map, top)),
        before,
        "the top where it was"
    );

    // A kept declaring mate reading `slide`: a third block on its own
    // gauge, its lower cap on the top's upper cap slid by `slide`.
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 8.0])));
    let (doc, k) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, k, Some(g));
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = step_with(
        doc,
        DocEdit::InsertNode {
            node: Box::new(seat(
                head(p.top_cap(k)),
                head(p.top_upper_cap(top)),
                slid_by_the_parameter(),
            )),
        },
        &reach,
    );
    let err = editor_core::split(
        &doc,
        &cut(&[base, top, m]),
        DocumentId::derive(&format!("{label}-part2")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect_err("both sides read `slide`");
    assert!(
        matches!(&err, SplitError::UncutParamReference { param, .. } if *param == slide()),
        "{err:?}"
    );
    // The cut seat re-authored to read nothing: the parameter stays.
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: m });
    let (doc, plain) = step_with(
        doc,
        DocEdit::InsertNode {
            node: Box::new(seat(
                head(p.top_cap(top)),
                head(p.base_cap(base)),
                literal([0.0, 0.5, 0.0]),
            )),
        },
        &reach,
    );
    let out = editor_core::split(
        &doc,
        &cut(&[base, top, plain.expect("minted")]),
        DocumentId::derive(&format!("{label}-part3")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("split refused: {e}"));
    assert!(
        !out.part.params().contains_key(&slide()),
        "nothing cut reads it"
    );
    assert!(
        out.remainder.params().contains_key(&slide()),
        "the kept mate's parameter stays"
    );
}

fn top_image(map: &editor_core::NodeMap, top: RecipeNodeId) -> RecipeNodeId {
    *map.get(&top).expect("the top is carried")
}

// ---- Row 7: a slot edit at a frame step is admitted as an insert is ----

/// **A slot edit that reaches a mate's frame offset asks the mate's
/// admission.** A rigid step turning about +z is admitted; zeroing its
/// axis one component at a time through `SetParam` is admitted until
/// the last zero leaves no direction, which refuses `MateRefused`
/// carrying `FrameUnevaluated` — what the insert of that mate refuses
/// with. `SetExpression` at the same address refuses alike.
#[test]
fn a_slot_edit_at_a_frame_step_is_admitted_as_the_insert_is() {
    let turn: Placement = Step::Rigid {
        translation: [len(0.0), len(0.5), len(0.0)],
        axis: [0.0, 0.0, 1.0].map(scl),
        angle: ang(0.0),
    }
    .into();
    let (p, doc, [_, _, m]) = seated("place-mfo-slot", turn, std::convert::identity);
    let opts = p.opts();
    let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
    let axis = |ax| SlotId::MateFrameStep {
        side: MateSide::B,
        step: 0,
        arg: RigidArg::RotationAxis(ax),
    };
    let doc = apply(
        &doc,
        &DocEdit::SetParam {
            node: m,
            slot: axis(editor_core::Axis3::X),
            expr: scl(0.0),
        },
        Tol::witness(),
        &reach,
    )
    .expect("x is already zero")
    .doc;
    let zeroed = |doc: &ProfileDoc, edit: DocEdit<ProfileProgram>| {
        apply(doc, &edit, Tol::witness(), &reach).expect_err("no direction is left")
    };
    let refused = |err: &EditError| {
        matches!(
            err,
            EditError::MateRefused { fault, .. }
                if matches!(**fault, editor_core::MateFault::FrameUnevaluated { side: MateSide::B, .. })
        )
    };
    let err = zeroed(
        &doc,
        DocEdit::SetParam {
            node: m,
            slot: axis(editor_core::Axis3::Z),
            expr: scl(0.0),
        },
    );
    assert!(refused(&err), "{err:?}");
    let err = zeroed(
        &doc,
        DocEdit::SetExpression {
            path: editor_core::ExprPath {
                node: m,
                slot: axis(editor_core::Axis3::Z),
                path: vec![],
            },
            expr: scl(0.0),
        },
    );
    assert!(refused(&err), "{err:?}");
}

// ---- Row 8: a mate's alignment compares by bits ----

/// **Two mates differing only in a signed zero are two nodes to D7.**
/// A literal frame offset at `+0.0` and at `-0.0`, a rider at `+0.0`
/// and `-0.0`: each pair is equal by value and different by bits, and
/// `Node::bit_eq` says so, as it does for a gauge's literal step.
#[test]
fn a_mates_alignment_compares_by_bits() {
    let cap = |node, end| {
        head(editor_core::StableName {
            kind: editor_core::EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![editor_core::RoleSeg::Cap(end)],
        })
    };
    let mate = |offset: Placement, clocking: Option<f64>| Node::<ProfileProgram>::Mate {
        a: cap(1, editor_core::CapEnd::Start),
        b: cap(2, editor_core::CapEnd::End),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::from_face(),
            b: MateFrame::on_face(offset),
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking,
        },
    };
    let plus = literal([0.0, 0.5, 0.0]);
    let minus = literal([-0.0, 0.5, 0.0]);
    for (what, x, y) in [
        (
            "offset",
            mate(plus.clone(), None),
            mate(minus.clone(), None),
        ),
        (
            "rider",
            mate(plus.clone(), Some(0.0)),
            mate(plus.clone(), Some(-0.0)),
        ),
        (
            "gauge",
            Node::gauge(None, plus.clone()),
            Node::gauge(None, minus.clone()),
        ),
    ] {
        assert_eq!(x, y, "{what}: equal by value");
        assert!(!x.bit_eq(&y), "{what}: different by bits");
        assert!(x.bit_eq(&x.clone()), "{what}: equal to itself");
    }
}
