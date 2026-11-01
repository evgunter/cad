//! **What the feature tree's rows say, read as values**: the rows are
//! a pure function of the recipe and the landed run, so they are
//! asserted here rather than looked at. Where a row is DRAWN — every
//! node row in one column — is the pane's, and its test is there
//! (`pane::features`, `no_node_row_is_drawn_under_another`).

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::geom_core::Tol;

/// **A measure's row reads the landed run, as every row's badge
/// does**: while the document has moved past the landed evaluation,
/// the row shows what that run measured on the document it ran over,
/// and the new value arrives with the run that measures it.
///
/// Red if the tree blanks a value while a run is outstanding, or keeps
/// the old one once the new run has landed.
#[test]
fn a_measure_row_shows_the_landed_value_until_the_next_run_lands() {
    use pncad::document::{Dimension, MeasurePrimitive, Node, SitedRef};
    use pncad::prelude::StableName;
    use pncad::select::{CapEnd, EntityKind, RoleSeg};
    use viewer::props::{Computed, SlotValue};
    use viewer::session::{DocSession, SessionOp};
    use viewer::tree::Readout;

    let tol = Tol::witness();
    // The plate is `thickness / 2` deep, and the measure reads it back.
    let (mut doc, _, extrude) = common::parametric_plate(tol);
    let cap = |end| {
        SitedRef::new(
            extrude,
            StableName {
                kind: EntityKind::Face,
                node: extrude,
                path: vec![RoleSeg::Cap(end)],
            },
        )
    };
    let measure = common::insert_into(
        &mut doc,
        Node::measure(&MeasurePrimitive::Distance {
            a: cap(CapEnd::Start),
            b: cap(CapEnd::End),
        }),
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let measured = |session: &DocSession| {
        common::row_of(&session.tree_rows(), measure)
            .readout
            .clone()
    };
    let reading = |metres| {
        Some(Readout::Value(Computed {
            canonical: metres,
            dimension: Dimension::Length,
        }))
    };
    let landed = reading(0.004);
    assert_eq!(measured(&session), landed);

    session.perform(SessionOp::SetVariable {
        var: common::thickness_var(session.committed_doc()),
        value: SlotValue::Continuous(0.012),
    });
    assert!(session.busy(), "the premise: the document has moved on");
    assert_eq!(
        measured(&session),
        landed,
        "the row is the landed picture's until the next one lands"
    );
    session.pump();
    assert_eq!(measured(&session), reading(0.006));
}
