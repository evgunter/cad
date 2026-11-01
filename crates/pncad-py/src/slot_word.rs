//! **The slot alphabet read INWARD** — a slot's stable word back to
//! the [`SlotId`] it names.
//!
//! [`crate::tags::slot_id_tag`] writes the word a refusal publishes;
//! this reads the same word off a caller. The two are ONE alphabet and
//! must stay one: a refusal that answers `slot` and a door that takes
//! one are the same address written in the two directions, so a
//! spelling accepted here that no refusal answers in — or a word a
//! refusal answers with that is not accepted here — breaks the
//! round trip a caller retries on.
//!
//! # Why it is not in `tags.rs`
//!
//! That module holds the tag FUNCTIONS — `(..) -> &'static str` and
//! nothing else — and is read as a TABLE by the guard that pins every
//! word this crate can put on the wire. A function answering a
//! `SlotId` is not one of those, and a reader enumerating tag values
//! would stop at it rather than enumerate it. Sited here, the guard
//! reads the forward map whole and this file is checked against the
//! inventory that guard pins.
//!
//! # The words with no reading
//!
//! `program` names one expression inside a profile PROGRAM, and its
//! address is completed by two integers and an argument role that the
//! word does not carry; `station_slide` and `station_spin` name one move
//! of a loft section's station, completed by the section and the
//! move's place in the station; and `value_slide` and `value_spin` a
//! value's number, completed by the constraint; `side` names one side of a constraint,
//! completed by the constraint and the side; a loft's `section`, a
//! union's or a `⊔`'s `member` and a measure's `measured` name one
//! entry of a list, completed by its position; `index` names
//! one index of an indexed read, completed by the seat it reads at and
//! the index's position. There is nothing to answer with, so each
//! answers nothing —
//! the same stop the forward map makes one level out, where the word
//! says which kind of slot it is and the rest of the address is in the
//! refusal's prose.

use pncad::document::{Axis3, OperandSlot, SlotId};

/// The [`SlotId`] a stable slot word names, or `None` for a word
/// outside the alphabet.
///
/// Exhaustive over [`crate::tags::slot_id_tag`]'s values by test: a
/// slot the kernel adds gains a word there, and a word with no arm
/// here is a slot a caller can read off a refusal and cannot write
/// back at.
pub fn slot_from_word(word: &str) -> Option<SlotId> {
    // The vector family spells its component INTO the word, exactly as
    // the forward map does — `origin` alone names three slots, and a
    // caller handing one back must say which.
    let slot = match word {
        "origin_x" => SlotId::Origin(Axis3::X),
        "origin_y" => SlotId::Origin(Axis3::Y),
        "origin_z" => SlotId::Origin(Axis3::Z),
        "distance" => SlotId::Distance,
        "radius" => SlotId::Radius,
        "chamfer_distance" => SlotId::ChamferDistance,
        "shell_thickness" => SlotId::ShellThickness,
        "revolve_angle" => SlotId::RevolveAngle,
        "axis_heading" => SlotId::AxisHeading,
        "tube_major_radius" => SlotId::TubeMajorRadius,
        "tube_minor_radius" => SlotId::TubeMinorRadius,
        "tube_window_start" => SlotId::TubeWindowStart,
        "tube_window_end" => SlotId::TubeWindowEnd,
        "tube_wall" => SlotId::TubeWall,
        "v_degree" => SlotId::VDegree,
        "stations" => SlotId::Stations,
        // The operands, by their field's word (D10: an operand is a
        // slot written through the same door).
        "profile" => SlotId::Operand(OperandSlot::Profile),
        "path" => SlotId::Operand(OperandSlot::Path),
        "axis" => SlotId::Operand(OperandSlot::Axis),
        "frame" => SlotId::Operand(OperandSlot::Frame),
        "target" => SlotId::Operand(OperandSlot::Target),
        "source" => SlotId::Operand(OperandSlot::Source),
        "tool" => SlotId::Operand(OperandSlot::Tool),
        "from" => SlotId::Operand(OperandSlot::From),
        "cut" => SlotId::Operand(OperandSlot::Cut),
        "members" => SlotId::Operand(OperandSlot::Members),
        "input" => SlotId::Operand(OperandSlot::Input),
        "of" => SlotId::Operand(OperandSlot::Of),
        "selection" => SlotId::Operand(OperandSlot::Selection),
        "open" => SlotId::Operand(OperandSlot::Open),
        "face" => SlotId::Operand(OperandSlot::Face),
        "body" => SlotId::Operand(OperandSlot::Body),
        "plane" => SlotId::Operand(OperandSlot::Plane),
        "target_space" => SlotId::Operand(OperandSlot::Space),
        _ => return None,
    };
    Some(slot)
}
