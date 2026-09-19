//! **Review probes for `edit/param-ref-one-convention`** (lane
//! `paramref-rv`): what the unit's own convention guard,
//! `display_contract::the_two_doors_spell_the_four_param_ref_refusals_with_the_same_four_names`,
//! does not measure.
//!
//! That row compares the SET of four `Debug` names at each door with
//! the `{address} x {fact}` product. A set has no opinion about which
//! arm carries which member, so the convention's actual content — the
//! ADDRESS word of a name is the address that arm is raised at — is
//! not in it. Measured: renaming the edit door's slot pair to the
//! payload spelling and its payload pair to the slot spelling leaves
//! every one of `display_contract`'s 29 rows green, the convention row
//! included, because the four names are the same four.
//!
//! PROBE 1 below is the missing half, and it goes red on that swap. It
//! ties each name to its address through the one place the address is
//! externally visible: the rendered sentence, which names a slot at a
//! slot arm and a payload expression at a payload arm, at both doors.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{Dimension, EditError, ParamName, RecipeNodeId, SlotId, SnapshotError};

/// The address word a name claims, and the address its rendered
/// sentence actually reports, for all eight arms.
///
/// A slot-addressed refusal names the slot it was raised at; a
/// payload-addressed one says "payload expression" and has no slot to
/// name. So a name whose address word is `Slot` owes a sentence that
/// names a slot, and one whose address word is `Payload` owes a
/// sentence that says "payload expression" — which is what makes
/// `{Slot,Payload}` a convention rather than four interchangeable
/// tokens spelled the same at both doors.
#[test]
fn rv_each_param_ref_name_reports_the_address_its_name_claims() {
    fn variant_of<T: core::fmt::Debug>(value: &T) -> String {
        format!("{value:?}")
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect()
    }

    /// `(what the arm renders, the arm's `Debug` name)` for one arm.
    fn arm<T: core::fmt::Debug + core::fmt::Display>(value: &T) -> (String, String) {
        (value.to_string(), variant_of(value))
    }

    let node = RecipeNodeId(5);
    let name = ParamName::new("width");

    let arms: Vec<(&str, (String, String))> = vec![
        (
            "edit",
            arm(&EditError::SlotUnknownDocParam {
                name: name.clone(),
                node,
                slot: SlotId::Radius,
            }),
        ),
        (
            "edit",
            arm(&EditError::SlotDocParamDimension {
                name: name.clone(),
                node,
                slot: SlotId::Radius,
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            }),
        ),
        (
            "edit",
            arm(&EditError::PayloadUnknownDocParam {
                name: name.clone(),
                node,
            }),
        ),
        (
            "edit",
            arm(&EditError::PayloadDocParamDimension {
                name: name.clone(),
                node,
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            }),
        ),
        (
            "load",
            arm(&SnapshotError::SlotUnknownDocParam {
                node,
                slot: SlotId::Radius,
                name: name.clone(),
            }),
        ),
        (
            "load",
            arm(&SnapshotError::SlotDocParamDimension {
                node,
                slot: SlotId::Radius,
                name: name.clone(),
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            }),
        ),
        (
            "load",
            arm(&SnapshotError::PayloadUnknownDocParam {
                node,
                name: name.clone(),
            }),
        ),
        (
            "load",
            arm(&SnapshotError::PayloadDocParamDimension {
                node,
                name,
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            }),
        ),
    ];

    for (door, (rendered, variant)) in &arms {
        let says_slot = rendered.contains("slot ");
        let says_payload = rendered.contains("payload expression");
        if let Some(fact) = variant.strip_prefix("Slot") {
            assert!(
                says_slot && !says_payload,
                "the {door} door's {variant} claims a SLOT address (fact {fact}) but renders \
                 {rendered:?}"
            );
        } else if let Some(fact) = variant.strip_prefix("Payload") {
            assert!(
                says_payload && !says_slot,
                "the {door} door's {variant} claims a PAYLOAD address (fact {fact}) but renders \
                 {rendered:?}"
            );
        } else {
            panic!("{door} door: {variant} does not open with an address word");
        }
    }
}
