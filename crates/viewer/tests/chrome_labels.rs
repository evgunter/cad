//! **What the chrome calls things.**
//!
//! Two of the names a user reads are pure functions of state rather
//! than pixels, so they are pinned here: the toolbar's name for the
//! open document, and the initial layout's shape. The rest of the
//! chrome's wording lives inside widget calls and is not testable
//! without a window; this suite claims only what it can see.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

test_utils::loud_skip_marker!(
    feature = "app",
    row = app_lane_skipped_no_chrome_coverage_here,
    absent = "coverage of the chrome's labels",
);

#[cfg(feature = "app")]
mod chrome {
    use std::path::Path;

    use viewer::app::{Pane, document_name, initial_layout, model_stack};

    #[test]
    fn the_toolbar_names_the_open_document_not_the_program() {
        assert_eq!(
            document_name(Some(Path::new("/tmp/gallery/diefillet.pncad"))),
            "diefillet",
            "the file stem, without the directory or the extension"
        );
        assert_eq!(
            document_name(None),
            "untitled",
            "a document with no path of its own still has a name to show"
        );
        // A path that is all directory has no stem; the label must still
        // say something rather than come up empty.
        assert_eq!(document_name(Some(Path::new("/"))), "untitled");
    }

    #[test]
    fn the_starting_layout_stacks_features_over_properties() {
        let tree = initial_layout();
        for pane in [Pane::Viewport, Pane::Features, Pane::Properties, Pane::View] {
            assert!(tree.tiles.find_pane(&pane).is_some(), "{pane:?} has a tile");
        }
        // Features and Properties share one container — the tile the
        // `Model` tab title and the content-driven split both key off.
        assert!(
            model_stack(&tree.tiles).is_some(),
            "the starting layout stacks the two panes the user reads together"
        );
    }

    /// **A disabled toolbar control says what its own operation
    /// refuses.** Three controls are held to it: Create on the
    /// New-document form, Undo and Redo.
    ///
    /// Each is drawn from the refusal a click would have been answered
    /// with — `Refusal::empty_name` and `Refusal::nothing_to_step` are
    /// the predicates the buttons gate on AND the doors refuse on — so
    /// the words beside the greyed button and the words on the status
    /// line are one value rather than two spellings of one condition.
    ///
    /// Asserted at the value the toolbar reads, which is the standing
    /// `gesture_table.rs`'s `a_closed_door_says_what_its_own_operation
    /// _refuses` holds the cancel doors to: what a widget call does
    /// with the value needs a window, and the drift this row exists to
    /// catch is in the value.
    ///
    /// The undo half is asserted **at the root with a redo waiting**,
    /// which is the state that makes the direction load-bearing: a
    /// sentence naming both directions is false there of the one the
    /// reader can still click.
    #[test]
    fn a_disabled_toolbar_control_says_what_its_own_operation_refuses() {
        use crate::common;
        use pncad::geom_core::Tol;
        use viewer::props::SlotValue;
        use viewer::session::{DocSession, Refusal, SessionOp, Step};

        let tol = Tol::witness();
        let (doc, _profile, _extrude) = common::parametric_plate(tol);
        let mut session = DocSession::inline(doc, tol);

        // Create, over the text the form's field holds. A blank name
        // is refused whether or not it is whitespace, and the trim is
        // the rule's rather than each caller's.
        for typed in ["", "   "] {
            let blocked = Refusal::empty_name(typed).expect("a blank name greys Create");
            let refused = session
                .perform(SessionOp::NewDocument {
                    name: typed.to_owned(),
                })
                .refusal
                .expect("and the door refuses the same name");
            assert_eq!(
                blocked.to_string(),
                refused.to_string(),
                "Create: the disabled control's words"
            );
            assert_eq!(
                format!("{blocked:?}"),
                format!("{refused:?}"),
                "Create: and the same refusal, not merely the same sentence"
            );
        }
        assert!(
            Refusal::empty_name(" plate ").is_none(),
            "a name with text in it leaves the button live"
        );

        // One edit, then undo it: the cursor is at the root and the
        // branch it left is waiting.
        session.perform(SessionOp::SetParam {
            name: common::thickness_param(),
            value: SlotValue::Continuous(0.010),
        });
        assert!(
            session.perform(SessionOp::Undo).refusal.is_none(),
            "the edit undoes"
        );

        let blocked = Refusal::nothing_to_step(session.history(), Step::Undo)
            .expect("at the root there is nothing to undo");
        let refused = session
            .perform(SessionOp::Undo)
            .refusal
            .expect("and the op refuses the step the button would have sent");
        assert_eq!(
            blocked.to_string(),
            refused.to_string(),
            "Undo: the disabled control's words"
        );
        assert_eq!(
            format!("{blocked:?}"),
            format!("{refused:?}"),
            "Undo: and the same refusal, not merely the same sentence"
        );
        assert!(
            Refusal::nothing_to_step(session.history(), Step::Redo).is_none(),
            "the Redo button beside it is live — which a sentence naming both directions denies"
        );

        // And the other direction, at the tip of the branch.
        assert!(
            session.perform(SessionOp::Redo).refusal.is_none(),
            "the redo lands"
        );
        let blocked = Refusal::nothing_to_step(session.history(), Step::Redo)
            .expect("at the tip there is nothing to redo");
        let refused = session
            .perform(SessionOp::Redo)
            .refusal
            .expect("and the op refuses");
        assert_eq!(
            blocked.to_string(),
            refused.to_string(),
            "Redo: the disabled control's words"
        );
        assert_eq!(
            format!("{blocked:?}"),
            format!("{refused:?}"),
            "Redo: and the same refusal, not merely the same sentence"
        );
        assert_ne!(
            Refusal::NothingToDo {
                direction: Step::Undo
            }
            .to_string(),
            refused.to_string(),
            "and the two buttons do not share one sentence"
        );
    }
}
