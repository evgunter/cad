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
//! # The one word with no reading
//!
//! `profile` names one expression inside a profile PROGRAM, and its
//! address is completed by two integers and an argument role that the
//! word does not carry. There is nothing to answer with, so it
//! answers nothing — the same stop the forward map makes one level
//! out, where the word says the slot is a profile program's and the
//! rest of the address is in the refusal's prose.

use pncad::document::{Axis3, SlotId};

/// The [`SlotId`] a stable slot word names, or `None` for a word
/// outside the alphabet.
///
/// Exhaustive over [`crate::tags::slot_id_tag`]'s values by test: a
/// slot the kernel adds gains a word there, and a word with no arm
/// here is a slot a caller can read off a refusal and cannot write
/// back at.
pub fn slot_from_word(word: &str) -> Option<SlotId> {
    // The seven vector families spell their component INTO the word,
    // exactly as the forward map does — `origin` alone names three
    // slots, and a caller handing one back must say which.
    let slot = match word {
        "origin_x" => SlotId::Origin(Axis3::X),
        "origin_y" => SlotId::Origin(Axis3::Y),
        "origin_z" => SlotId::Origin(Axis3::Z),
        "normal_x" => SlotId::Normal(Axis3::X),
        "normal_y" => SlotId::Normal(Axis3::Y),
        "normal_z" => SlotId::Normal(Axis3::Z),
        "direction_x" => SlotId::Direction(Axis3::X),
        "direction_y" => SlotId::Direction(Axis3::Y),
        "direction_z" => SlotId::Direction(Axis3::Z),
        "u_x" => SlotId::U(Axis3::X),
        "u_y" => SlotId::U(Axis3::Y),
        "u_z" => SlotId::U(Axis3::Z),
        "v_x" => SlotId::V(Axis3::X),
        "v_y" => SlotId::V(Axis3::Y),
        "v_z" => SlotId::V(Axis3::Z),
        "translation_x" => SlotId::Translation(Axis3::X),
        "translation_y" => SlotId::Translation(Axis3::Y),
        "translation_z" => SlotId::Translation(Axis3::Z),
        "rotation_axis_x" => SlotId::RotationAxis(Axis3::X),
        "rotation_axis_y" => SlotId::RotationAxis(Axis3::Y),
        "rotation_axis_z" => SlotId::RotationAxis(Axis3::Z),
        "distance" => SlotId::Distance,
        "radius" => SlotId::Radius,
        "chamfer_distance" => SlotId::ChamferDistance,
        "shell_thickness" => SlotId::ShellThickness,
        "revolve_angle" => SlotId::RevolveAngle,
        "spin" => SlotId::Spin,
        "tube_major_radius" => SlotId::TubeMajorRadius,
        "tube_minor_radius" => SlotId::TubeMinorRadius,
        "tube_window_start" => SlotId::TubeWindowStart,
        "tube_window_end" => SlotId::TubeWindowEnd,
        "tube_wall" => SlotId::TubeWall,
        "rotation_angle" => SlotId::RotationAngle,
        "spacing" => SlotId::Spacing,
        "step" => SlotId::Step,
        "count" => SlotId::Count,
        "instance" => SlotId::Instance,
        "v_degree" => SlotId::VDegree,
        "stations" => SlotId::Stations,
        _ => return None,
    };
    Some(slot)
}
