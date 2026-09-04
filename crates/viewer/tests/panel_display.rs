//! **How the panel WRITES a number, as values rather than as pixels.**
//!
//! The display decisions land in `props` and in the pure functions the
//! panel reads it through, and each is a function of a document, so
//! each is asserted here rather than looked at:
//!
//! * the three components of a 3-vector are ONE panel row
//!   (`props::group_rows`),
//! * a value is shown in the unit its literal — or, for a document
//!   parameter, its DECLARATION — remembers, and authored back through
//!   the same factor (`rendering_unit` / `in_written` / `from_written`),
//!   at a drag tick in that same unit (`app::FieldWriting`),
//! * a range probe's reading is written in the unit the SEARCH ran in
//!   (`BoundsReading`),
//! * changing the unit and changing the number are separate operations,
//!   and neither performs the other (`SetSlotUnit` vs `SetSlot`),
//! * and the ONE value field says the number without the unit, shows a
//!   driven slot's source, and routes typed text to the door it means
//!   (`field_text` / `field_edit`).
//!
//! The pixels are not tested here and are not the claim; what is
//! claimed is that the panel is drawing from the right numbers.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{
    Axis3, Datum, Dimension, Doc, DocEdit, DocParam, Expr, Node, ParamName, ProfileProgram, SlotId,
    VectorSlot,
};
use pncad::geom_core::Tol;
use pncad::prelude::{DEG, IN, MM, PI, RAD};
use pncad::quantity::{UnitDef, WrittenLength, unit_by_symbol};
use viewer::props::{
    self, SlotGroup, SlotUnitFault, SlotValue, from_written, in_written, rendering_unit,
};
use viewer::session::{BoundsTarget, DocSession, Refusal, Selection, SessionOp};

/// A document holding one datum plane — six slots, two vector families.
fn plane_doc(tol: Tol) -> (Doc<ProfileProgram>, pncad::document::RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-display", tol);
    common::inserted(
        &doc,
        Node::Datum(Datum::Plane {
            origin: [common::len(0.001), common::len(0.002), common::len(0.003)],
            normal: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
        }),
        tol,
    )
}

/// **A datum plane is two rows, not six** — and the rows it is made of
/// are exactly the slots the node has, in the vocabulary's own order.
///
/// The second half is the property that matters more than the count:
/// grouping is a LAYOUT, so it may bundle rows and may not invent,
/// drop, reorder or duplicate one.
#[test]
fn a_vectors_three_components_are_one_panel_row() {
    let tol = Tol::witness();
    let (doc, plane) = plane_doc(tol);
    let rows = props::slot_rows(&doc, plane);
    let groups = props::group_rows(rows.clone());

    assert_eq!(rows.len(), 6, "a plane carries origin xyz + normal xyz");
    assert_eq!(groups.len(), 2, "shown as two vectors");
    match &groups[0] {
        SlotGroup::Vector { family, rows } => {
            assert_eq!(*family, VectorSlot::Origin);
            assert_eq!(
                rows.each_ref().map(|row| row.slot),
                Axis3::ALL.map(SlotId::Origin),
                "components in x/y/z order"
            );
        }
        other => panic!("expected the origin vector, got {other:?}"),
    }
    assert!(matches!(
        &groups[1],
        SlotGroup::Vector {
            family: VectorSlot::Normal,
            ..
        }
    ));
    // The flattening is the whole set, once each, in order.
    let flattened: Vec<SlotId> = groups
        .iter()
        .flat_map(SlotGroup::rows)
        .map(|row| row.slot)
        .collect();
    let original: Vec<SlotId> = rows.iter().map(|row| row.slot).collect();
    assert_eq!(flattened, original, "grouping is a layout, not a filter");
}

/// A node with no vector slots groups into scalars, one apiece — the
/// arm that says grouping does not fire where there is nothing to
/// group.
#[test]
fn scalar_slots_stay_one_row_each() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let groups = props::slot_groups(&doc, extrude);
    assert_eq!(groups.len(), 1);
    assert!(matches!(groups[0], SlotGroup::Scalar(_)));
}

/// **A family missing a component degrades to scalars rather than
/// drawing a vector with a hole in it.**
///
/// Two components of a family cannot arise from a well-formed node —
/// `Node::slots` emits all three — so the input is synthesised here
/// directly from rows. That is the point: the rule is stated on the
/// FOLD, so it holds whatever produced the rows, and a `Node::slots`
/// postcondition break shows up as the components the node actually
/// has instead of as a panic or a silently dropped row.
#[test]
fn an_incomplete_vector_family_degrades_to_scalar_rows() {
    let tol = Tol::witness();
    let (doc, plane) = plane_doc(tol);
    let mut rows = props::slot_rows(&doc, plane);
    rows.retain(|row| row.slot != SlotId::Origin(Axis3::Z));
    let groups = props::group_rows(rows);
    // origin x, origin y as scalars; the complete normal as a vector.
    assert_eq!(groups.len(), 3);
    assert!(matches!(
        groups[0],
        SlotGroup::Scalar(ref row) if row.slot == SlotId::Origin(Axis3::X)
    ));
    assert!(matches!(
        groups[1],
        SlotGroup::Scalar(ref row) if row.slot == SlotId::Origin(Axis3::Y)
    ));
    assert!(matches!(
        groups[2],
        SlotGroup::Vector {
            family: VectorSlot::Normal,
            ..
        }
    ));
}

/// **The written unit is the literal's own, and the canonical fallback
/// is a real row rather than "no unit".**
#[test]
fn a_slot_is_written_in_the_unit_its_literal_remembers() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-units", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::literal_with_unit(0.008, Dimension::Length, MM.def())
                .expect("8 mm is a length"),
        },
        tol,
    );
    let rows = props::slot_rows(&doc, extrude);
    let row = rows.first().expect("the extrude has a distance");
    assert_eq!(row.unit, Some(MM.def()), "the stored unit is remembered");
    let unit = rendering_unit(row.dimension, row.unit).expect("a length row");
    assert_eq!(unit, MM.def());
    // 0.008 m shown as 8 mm — and back to the identical bits.
    let shown = in_written(0.008, unit);
    assert!((shown - 8.0).abs() < 1e-12, "shown as {shown}");
    assert_eq!(from_written(shown, unit).to_bits(), 0.008_f64.to_bits());

    // A literal authored canonically NAMES the canonical row rather
    // than remembering nothing: `m`'s factor is exactly 1.0, so the
    // number shown is the number stored, and the row says which unit
    // that is instead of leaving a reader to decide.
    let (doc, plain) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.008),
        },
        tol,
    );
    let rows = props::slot_rows(&doc, plain);
    let row = rows.first().expect("the extrude has a distance");
    assert_eq!(
        row.unit.map(|u| u.symbol()),
        Some("m"),
        "a literal always names its notation"
    );
    let unit = rendering_unit(row.dimension, row.unit);
    assert_eq!(unit.map(|u| u.symbol()), Some("m"));
    assert_eq!(
        in_written(0.008, unit.expect("a length row")).to_bits(),
        0.008_f64.to_bits()
    );

    // `None` reaches `rendering_unit` only for a value NOBODY WROTE — a
    // slot driven by an expression — and the unit chosen there is the
    // CANONICAL one, which is what `unparse` renders such a value in
    // too. The two used to disagree: this function's predecessor chose
    // half-turns for an unmarked angle while the text formatter chose
    // radians, so one stored value read two ways.
    let unit = rendering_unit(Dimension::Angle, None);
    assert_eq!(unit.map(|u| u.symbol()), Some("rad"));
    let shown = in_written(core::f64::consts::TAU, unit.expect("an angle row"));
    assert!(
        (shown - core::f64::consts::TAU).abs() < 1e-12,
        "a full turn shown as {shown}"
    );

    // Half-turns are still a row a user can PICK — the notation this
    // editor says angles in, and the creation forms' angle default. It
    // is now named by the literal rather than supplied by the reader.
    let turn = Expr::literal_with_unit(core::f64::consts::TAU, Dimension::Angle, PI.def())
        .expect("a full turn in half-turns");
    let unit = turn.display_unit().expect("a literal names its unit");
    assert_eq!(unit.symbol(), "pi rad");
    assert!((in_written(core::f64::consts::TAU, unit) - 2.0).abs() < 1e-12);

    // A Scalar names the DIMENSIONLESS row: the empty symbol, factor
    // exactly 1.0. It is a row so that no literal has to decline to
    // name a notation — and the picker still offers nothing for it,
    // because a choice between one option is not a choice.
    let dimensionless = rendering_unit(Dimension::Scalar, None).expect("the dimensionless row");
    assert_eq!(dimensionless.symbol(), "");
    assert_eq!(dimensionless.factor(), 1.0);
    assert_eq!(
        common::scl(0.5).display_unit().map(|u| u.symbol()),
        Some(""),
        "a scalar literal names it too"
    );
    assert!(props::unit_options(Dimension::Scalar).is_empty());
    // A Count has no unit at all: an instance count is a number, and
    // the table has no row for it.
    assert_eq!(rendering_unit(Dimension::Count, None), None);
    assert!(props::unit_options(Dimension::Count).is_empty());
}

/// The picker's options are the closed table's rows OF THAT DIMENSION,
/// read from the table rather than listed here — so a unit added to
/// `quantity` is offered the day it lands, and no dimension is ever
/// offered another's units.
#[test]
fn the_unit_options_are_the_tables_rows_of_that_dimension() {
    let lengths: Vec<&str> = props::unit_options(Dimension::Length)
        .iter()
        .map(|u| u.symbol())
        .collect();
    let angles: Vec<&str> = props::unit_options(Dimension::Angle)
        .iter()
        .map(|u| u.symbol())
        .collect();
    for row in pncad::quantity::UNITS {
        // The dimensionless row is the ONE the picker does not offer,
        // and deliberately: it is the only unit a `Scalar` can be
        // written in, and a choice between one option is not a choice.
        // It exists so that a literal can NAME its notation, not so
        // that a user can pick it.
        if row.quantity() == pncad::quantity::UnitQuantity::Scalar {
            assert_eq!(row.symbol(), "");
            assert!(props::unit_options(Dimension::Scalar).is_empty());
            continue;
        }
        let listed = lengths.contains(&row.symbol()) || angles.contains(&row.symbol());
        assert!(listed, "{} is offered nowhere", row.symbol());
        assert!(
            !(lengths.contains(&row.symbol()) && angles.contains(&row.symbol())),
            "{} is offered under both dimensions",
            row.symbol()
        );
    }
    assert!(angles.contains(&"pi rad"), "the half-turn row is offered");
    assert!(!lengths.contains(&"deg"));
}

/// **Editing the number does not rewrite how the number is written.**
///
/// The rule `slot_edit` exists to keep: a drag that canonicalized the
/// literal would lose the user's chosen notation silently, once, on the
/// first touch.
#[test]
fn a_value_edit_keeps_the_slots_rendering_unit() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-unit-keep", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::literal_with_unit(0.008, Dimension::Length, MM.def())
                .expect("8 mm is a length"),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    let outcome = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.012),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let row = props::slot_rows(session.doc(), extrude)
        .into_iter()
        .next()
        .expect("the distance row");
    assert_eq!(row.value, Ok(SlotValue::Continuous(0.012)));
    assert_eq!(row.unit, Some(MM.def()), "still written in mm");

    // And a gesture, which captures the unit when it opens rather than
    // re-reading the scratch document its own previews are writing.
    session.perform(SessionOp::BeginGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
    session.perform(SessionOp::PreviewGesture { value: 0.02 });
    let outcome = session.perform(SessionOp::CommitGesture);
    assert_eq!(outcome.committed.len(), 1, "one edit per gesture");
    let row = props::slot_rows(session.doc(), extrude)
        .into_iter()
        .next()
        .expect("the distance row");
    assert_eq!(row.value, Ok(SlotValue::Continuous(0.02)));
    assert_eq!(row.unit, Some(MM.def()), "a drag does not canonicalize");
}

/// **Changing how it is written moves no bits.**
///
/// The half-turn row is the case that makes the point: 90° and 0.5π are
/// the same canonical value to within the last ulp, so the ONLY thing
/// that distinguishes them is the stored notation — which is why
/// rewriting the notation must not touch the value.
#[test]
fn changing_the_display_unit_leaves_the_value_bit_identical() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-unit-switch", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.008),
        },
        tol,
    );
    let (doc, placed) = common::inserted(
        &doc,
        Node::Transform {
            input: extrude,
            translation: [common::len(0.0), common::len(0.0), common::len(0.0)],
            rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
            rotation_angle: Expr::literal_with_unit(
                core::f64::consts::FRAC_PI_2,
                Dimension::Angle,
                DEG.def(),
            )
            .expect("a right angle"),
        },
        tol,
    );
    let before = props::slot_rows(&doc, placed)
        .into_iter()
        .find(|row| row.slot == SlotId::RotationAngle)
        .expect("the rotation angle");
    assert_eq!(before.unit, Some(DEG.def()));

    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::SetSlotUnit {
        node: placed,
        slot: SlotId::RotationAngle,
        unit: PI.def(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "one edit, one undo step");
    let after = props::slot_rows(session.doc(), placed)
        .into_iter()
        .find(|row| row.slot == SlotId::RotationAngle)
        .expect("the rotation angle");
    assert_eq!(after.unit, Some(PI.def()), "written in half-turns now");
    let radians = after.value.clone().expect("a value").as_f64();
    assert_eq!(
        radians.to_bits(),
        before.value.expect("a value").as_f64().to_bits(),
        "the notation moved; the value did not"
    );
    // And that notation is what the user asked for: a right angle,
    // written as half a π.
    let shown = in_written(
        radians,
        rendering_unit(after.dimension, after.unit).expect("an angle row"),
    );
    assert!((shown - 0.5).abs() < 1e-15, "shown as {shown}");

    // There is no "clear the notation" spelling: the canonical unit is
    // named by naming it, so moving a literal back to radians is the
    // same kind of edit as moving it to degrees.
    session.perform(SessionOp::SetSlotUnit {
        node: placed,
        slot: SlotId::RotationAngle,
        unit: RAD.def(),
    });
    let after = props::slot_rows(session.doc(), placed)
        .into_iter()
        .find(|row| row.slot == SlotId::RotationAngle)
        .expect("the rotation angle");
    assert_eq!(
        after.unit.map(|u| u.symbol()),
        Some("rad"),
        "the literal now names radians, rather than declining to name anything"
    );
    assert_eq!(
        rendering_unit(after.dimension, after.unit).map(|u| u.symbol()),
        Some("rad"),
        "and the panel renders what the literal says, with nothing to infer"
    );
}

/// The two typed refusals a unit change can raise, each named rather
/// than swallowed: a computed slot has no authored notation, and a unit
/// of the wrong dimension is not a unit for that slot.
#[test]
fn a_unit_change_refuses_typed_on_a_computed_slot_and_a_foreign_unit() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    // The parametric fixture's distance is DRIVEN by `thickness`.
    let fault = props::slot_unit_edit(&doc, extrude, SlotId::Distance, MM.def())
        .expect_err("a computed slot has no written unit");
    assert!(
        matches!(fault, SlotUnitFault::NotALiteral { .. }),
        "{fault:?}"
    );

    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::SetSlotUnit {
        node: extrude,
        slot: SlotId::Distance,
        unit: MM.def(),
    });
    assert!(matches!(outcome.refusal, Some(Refusal::SlotUnit(_))));
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");

    // A degree is not a length. Asserted through the table's own row so
    // the test does not depend on a `UnitDef` this crate built.
    let deg: UnitDef = unit_by_symbol("deg").expect("deg is a table row");
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-unit-refuse", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, plain) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: common::len(0.008),
        },
        tol,
    );
    let fault = props::slot_unit_edit(&doc, plain, SlotId::Distance, deg)
        .expect_err("a length is not written in degrees");
    assert!(
        matches!(fault, SlotUnitFault::Dimension { .. }),
        "{fault:?}"
    );
    // And a slot the node does not carry.
    let fault = props::slot_unit_edit(&doc, plain, SlotId::Radius, MM.def())
        .expect_err("an extrude has no radius");
    assert!(
        matches!(fault, SlotUnitFault::NoExpression { .. }),
        "{fault:?}"
    );
    let _ = DocEdit::<ProfileProgram>::DeleteNode { id: plain };
}

/// **The value field says the number, and the picker says the unit.**
///
/// The two sat adjacent saying the same thing, which is what Ev's
/// report was about; the field is the half that gives it up, because
/// the picker is also the door that CHANGES the unit.
#[test]
fn the_field_shows_a_bare_literals_number_without_its_unit() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-field", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::literal_with_unit(0.008, Dimension::Length, MM.def())
                .expect("8 mm is a length"),
        },
        tol,
    );
    let row = props::slot_rows(&doc, extrude)
        .into_iter()
        .next()
        .expect("the distance row");
    assert_eq!(props::field_text(&row), "8");
    assert_eq!(rendering_unit(row.dimension, row.unit), Some(MM.def()));
    // The SOURCE the row carries is the whole literal, unit and all —
    // it is what the expression door would read back — but that is not
    // what a literal's field shows.
    assert_eq!(row.source.as_deref(), Some("8 mm"));
}

/// A DRIVEN slot shows what drives it. Nothing else could be shown:
/// its number is a consequence, and the text is what an edit revises.
#[test]
fn the_field_shows_a_driven_slots_source() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "thickness * 2.0 + 1 mm".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let row = props::slot_rows(session.doc(), extrude)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the distance row");
    assert_eq!(props::field_text(&row), "thickness * 2.0 + 1 mm");
    assert_eq!(row.source.as_deref(), Some("thickness * 2.0 + 1 mm"));
}

/// **What typed text MEANS**, which is the whole of the single field's
/// contract: bare digits are a number and everything else is source.
#[test]
fn the_field_reads_digits_as_a_number_and_everything_else_as_source() {
    use props::FieldEdit;
    assert_eq!(props::field_edit("8"), FieldEdit::Number(8.0));
    assert_eq!(props::field_edit(" 8.5 "), FieldEdit::Number(8.5));
    assert_eq!(props::field_edit("-4"), FieldEdit::Number(-4.0));
    assert_eq!(props::field_edit("1e-3"), FieldEdit::Number(1e-3));
    assert_eq!(props::field_edit(""), FieldEdit::Empty);
    assert_eq!(props::field_edit("   "), FieldEdit::Empty);
    for source in ["25 in", "thickness", "8 * 2", "-w", "sin(30 deg)", "8mm"] {
        assert_eq!(
            props::field_edit(source),
            FieldEdit::Expression(source.to_owned()),
            "{source}"
        );
    }
}

/// **The unit-authoring rule, end to end.** A typed literal that
/// carries a unit is authoring the notation as well as the number, and
/// it does so through the ONE door that already has those semantics —
/// so the field and the picker agree afterwards without either being
/// told about the other.
#[test]
fn a_typed_literal_with_a_unit_authors_the_display_unit_too() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-field-unit", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::literal_with_unit(0.008, Dimension::Length, MM.def())
                .expect("8 mm is a length"),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    // `25 in` is not a number, so the field routes it to the
    // expression door — where the parser's own literal semantics both
    // convert it and remember the unit.
    assert_eq!(
        props::field_edit("25 in"),
        props::FieldEdit::Expression("25 in".to_owned())
    );
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "25 in".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let row = props::slot_rows(session.doc(), extrude)
        .into_iter()
        .next()
        .expect("the distance row");
    assert_eq!(
        row.unit.map(|u| u.symbol()),
        Some("in"),
        "the picker now says what the field said"
    );
    assert_eq!(props::field_text(&row), "25");
    assert_eq!(
        row.driver,
        props::SlotDriver::Literal,
        "still a bare number"
    );
    assert_eq!(row.value, Ok(SlotValue::Continuous(25.0 * 0.0254)));

    // And a bare number typed after it leaves that notation alone —
    // the numeric door re-attaches the stored unit, exactly as before.
    let outcome = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::of(Dimension::Length, from_written(2.0, IN.def())),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let row = props::slot_rows(session.doc(), extrude)
        .into_iter()
        .next()
        .expect("the distance row");
    assert_eq!(row.unit.map(|u| u.symbol()), Some("in"));
    assert_eq!(props::field_text(&row), "2");
}

/// **A parameter declared in millimetres reads in millimetres**, and a
/// number authored against that row lands as the millimetres it says.
///
/// The row is the panel's whole account of a document parameter, so
/// this is the claim a canonical-unit row breaks: `50 mm` shown as
/// `0.05` is the panel saying metres about a parameter nobody wrote in
/// metres. The value that CROSSES `props` stays canonical either way —
/// the conversion is the field's, the same one a slot's does.
///
/// What is asserted HERE is the half `props` answers: the row carries
/// the declaration's notation, and writing a number through the
/// value-only door does not rewrite it. The field's arithmetic over
/// that row is `viewer::app`'s and is asserted in
/// `a_parameter_field_is_written_the_way_its_declaration_says` below.
#[test]
fn a_millimetre_parameter_reads_and_authors_in_millimetres() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-param-unit", tol);
    let name = ParamName::new("base_r");
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::written_length(WrittenLength::in_unit(50.0, MM)),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let row = |session: &DocSession| {
        props::param_rows(session.doc())
            .into_iter()
            .find(|row| row.name == name)
            .expect("the parameter row")
    };

    let before = row(&session);
    let unit = rendering_unit(before.dimension, before.unit).expect("a length parameter");
    assert_eq!(
        unit.symbol(),
        "mm",
        "the row is written in what declared it"
    );
    assert_eq!(
        before.value,
        SlotValue::Continuous(0.05),
        "stored canonical"
    );

    // A value edit through the panel's own door: the number moves, the
    // notation beside it does not.
    let outcome = session.perform(SessionOp::SetParam {
        name: name.clone(),
        value: SlotValue::of(before.dimension, from_written(60.0, unit)),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let after = row(&session);
    assert_eq!(
        rendering_unit(after.dimension, after.unit).map(|u| u.symbol()),
        Some("mm"),
        "writing a number does not rewrite how the number is written"
    );
    assert_eq!(after.value, SlotValue::Continuous(0.06));
}

/// A `Count` parameter names no unit — an instance count is a number,
/// not a quantity — and the panel's rendering rule agrees.
#[test]
fn a_count_parameter_has_no_written_unit() {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-param-count", tol);
    let name = ParamName::new("holes");
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::Count { value: 6 },
        },
        tol,
    );
    let row = props::param_rows(&doc)
        .into_iter()
        .find(|row| row.name == name)
        .expect("the parameter row");
    assert_eq!(row.unit, None);
    assert_eq!(rendering_unit(row.dimension, row.unit), None);
    assert_eq!(row.value, SlotValue::Count(6));
}

/// **A parameter's range reading is written in the unit the search
/// used**, and this row establishes that by RUNNING the search: it
/// probes, then reads the sentence the panel would draw.
///
/// A reading in any other unit describes a search that did not happen —
/// the panel saying metres about a range found in millimetres. The
/// agreement is not two reads of one document that happen to match: the
/// probe stores the unit it stepped by (`BoundsReading::unit`) and the
/// reading is written in that stored unit, so a hand-built `Bounds`
/// beside a hand-picked unit would assert nothing about the pairing.
/// Both parameters below drive the same feature and differ only in the
/// notation they were DECLARED in.
#[test]
fn a_parameters_range_reads_in_the_unit_it_was_searched_in() {
    let reading = |value: DocParam| {
        let tol = Tol::witness();
        let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-param-range", tol);
        let name = ParamName::new("thickness");
        let (doc, _) = common::edited(
            &doc,
            DocEdit::SetDocParam {
                name: name.clone(),
                value,
            },
            tol,
        );
        let (doc, profile) = common::framed_square(&doc, 0.04, tol);
        let (doc, _) = common::inserted(
            &doc,
            Node::Extrude {
                profile,
                distance: Expr::param(name.clone(), Dimension::Length),
            },
            tol,
        );
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let outcome = session.perform(SessionOp::ProbeBounds {
            target: BoundsTarget::Param { name },
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        let reading = session.bounds().expect("a probe landed").clone();
        (reading.unit.map(|u| u.symbol()), reading.wording())
    };

    // Declared in millimetres: the search stepped by one millimetre and
    // the sentence says millimetres.
    let (unit, mm) = reading(DocParam::written_length(WrittenLength::in_unit(8.0, MM)));
    assert_eq!(unit, Some("mm"), "the search ran in the declared unit");
    assert!(mm.contains(" mm"), "{mm}");
    assert!(!mm.contains(" m,") && !mm.contains(" m "), "{mm}");

    // The same parameter declared canonically: same document, same
    // search, a different sentence — so the reading follows the
    // DECLARATION and not the dimension.
    let (unit, m) = reading(DocParam::continuous(Dimension::Length, 0.008));
    assert_eq!(unit, Some("m"));
    assert!(m.contains(" m"), "{m}");
    assert!(!m.contains(" mm"), "{m}");
}

/// **Loud skip.** The row below needs `viewer::app`, which is not in a
/// default-feature build; say so rather than letting the run report one
/// fewer test and nothing else. Its seat is the hosted row
/// `cargo nextest run -p viewer --features app`
/// (`.github/workflows/ci.yml`).
///
/// **This row closes no gate and cannot fail** — its payload is its
/// NAME in the PASS list. It names the row by hand, so a second
/// `app`-gated row added to this file leaves the marker quietly
/// incomplete.
#[cfg(not(feature = "app"))]
#[test]
fn app_lane_skipped_parameter_field_units_not_checked_here() {
    println!(
        "SKIPPED (no --features app): a_parameter_field_is_written_the_way_its_declaration_says \
         does not run - the parameter field's shown number, its authored number and its drag \
         tick are unchecked in this build."
    );
}

/// **A parameter field is shown, scrubbed and authored in the unit its
/// DECLARATION names** — `app::FieldWriting`, the value the panel
/// builds one field from.
///
/// Every assertion here can fail for the reason this unit was filed
/// about. A writing that ignored the declaration shows `0.05`; one that
/// inverted the conversion authors `60000`; one that picked its tick
/// before its unit steps a millimetre field by half a metre; one that
/// took the length constant for every dimension moves an angle field a
/// quarter turn per three thousand pixels.
///
/// **What no value test in this crate reaches** is the widget itself:
/// the panel's `DragValue` lives inside a private `ViewerBehavior`
/// method over an `egui::Ui`, and this crate carries no headless egui
/// harness, so "the field calls this" is held by the two call sites
/// being one line each rather than by a row here.
#[cfg(feature = "app")]
#[test]
fn a_parameter_field_is_written_the_way_its_declaration_says() {
    use viewer::app::FieldWriting;

    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("panel-param-field", tol);
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetDocParam {
            name: ParamName::new("thickness"),
            value: DocParam::written_length(WrittenLength::in_unit(8.0, MM)),
        },
        tol,
    );
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetDocParam {
            name: ParamName::new("in_metres"),
            value: DocParam::continuous(Dimension::Length, 0.008),
        },
        tol,
    );
    let rows = props::param_rows(&doc);
    let writing = |name: &str| {
        let row = rows
            .iter()
            .find(|row| row.name == ParamName::new(name))
            .expect("the parameter row");
        FieldWriting::of(row.dimension, row.unit)
    };

    // The millimetre declaration: 8, dragged half a millimetre at a
    // time, authored back into metres.
    let mm = writing("thickness");
    assert_eq!(mm.unit.map(|u| u.symbol()), Some("mm"));
    assert_eq!(mm.shown(0.008), 8.0);
    assert_eq!(mm.authored(60.0), 0.06);
    assert_eq!(mm.tick, 0.5, "half a millimetre, in what the field says");

    // The same dimension declared canonically: the SAME half
    // millimetre, spelled in metres because that is what this field
    // says.
    let m = writing("in_metres");
    assert_eq!(m.unit.map(|u| u.symbol()), Some("m"));
    assert_eq!(m.shown(0.008), 0.008);
    assert_eq!(m.tick, 0.0005);
    assert_eq!(
        m.authored(m.tick),
        mm.authored(mm.tick),
        "one gesture over a length is one step, however the field is written"
    );

    // A slot field written the same way is written the SAME way — the
    // two rows reach `FieldWriting` from different documents' facts (a
    // literal's remembered unit, a declaration's) and must not come to
    // two answers.
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::literal_with_unit(0.008, Dimension::Length, MM.def())
                .expect("8 mm is a length"),
        },
        tol,
    );
    let slot = props::slot_rows(&doc, extrude)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the distance row");
    assert_eq!(FieldWriting::of(slot.dimension, slot.unit), mm);

    // Each dimension keeps its own tick, and a count keeps whole
    // numbers: the mistakes `drag_tick`'s branch and the `Count` arm
    // exist to answer.
    let canonical = |dimension| FieldWriting::of(dimension, None).tick;
    let length = canonical(Dimension::Length);
    assert_ne!(canonical(Dimension::Angle), length);
    assert_ne!(canonical(Dimension::Scalar), length);
    assert_eq!(canonical(Dimension::Count), 1.0);
}
