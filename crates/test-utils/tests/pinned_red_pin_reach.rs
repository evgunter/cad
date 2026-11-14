//! **What a `pinned_red::row` pin can see**, read on three of the pins
//! PR 6934 set at ε 1e-6: each probe hands the pin a stop main does not
//! produce today and says whether the pin goes red on it. The `says`
//! arrays are the pins' own, copied from
//! `crates/sweep/tests/parallel_cylinder_join.rs`,
//! `crates/sweep/tests/verbs_germarms2.rs` and
//! `crates/editor-core/tests/revalue_corpus.rs` at `5a50b4459`.
//!
//! The pin is a containment test over its pieces, so a stop that says
//! MORE than main's passes it: a pin over a joined list of failing runs
//! cannot see a further run fail, and a pin that leaves a number out
//! cannot see that number move.
#![allow(clippy::panic)]

const SEAMS_SAYS: [&str; 6] = [
    "tipped 4.6875e-7 about Vec3 { x: 1.0, y: 0.0, z: 0.0 } | U ab | OK BAD t2=true t3p=true cert=true operand=true v=8.853981355 want=8.853981634",
    "tipped 4.6875e-7 about Vec3 { x: 1.0, y: 0.0, z: 0.0 } | U ba | OK BAD t2=true t3p=true cert=true operand=true v=8.853981433 want=8.853981634",
    "tipped 4.6875e-7 about Vec3 { x: 1.0, y: 0.0, z: 0.0 } | S ba | OK BAD t2=true t3p=true cert=true operand=true v=2.570796126 want=2.570796327",
    "tipped 4.6875e-7 about Vec3 { x: 0.0, y: 1.0, z: 0.0 } | U ab | OK BAD t2=true t3p=true cert=true operand=true v=8.853981433 want=8.853981634",
    "tipped 4.6875e-7 about Vec3 { x: 0.0, y: 1.0, z: 0.0 } | U ba | OK BAD t2=true t3p=true cert=true operand=true v=8.853981433 want=8.853981634",
    "tipped 4.6875e-7 about Vec3 { x: 0.0, y: 1.0, z: 0.0 } | S ba | OK BAD t2=true t3p=true cert=true operand=true v=2.570796126 want=2.570796327",
];

/// A seventh run turning `BAD`, at another tip and a volume far off its
/// closed form, keeps `seams_lying_on_the_walls_read_the_axes_across_the_region`'s
/// pin green: the row panics with the failing lines joined, and the pin
/// asks only that the six it names be among them.
#[test]
fn a_seventh_wrong_volume_keeps_the_seams_pin_green() {
    let mut lines = SEAMS_SAYS.to_vec();
    lines.push(
        "tipped 1.5625e-7 about Vec3 { x: 1.0, y: 0.0, z: 0.0 } | S ab | OK BAD t2=true t3p=true cert=true operand=true v=1.000000000 want=2.570796327",
    );
    let message = lines.join("\n");
    test_utils::pinned_red::row(
        1e-6,
        &[1e-6],
        "the seams on the walls tipped 1.5·zero / 3.2",
        &SEAMS_SAYS,
        || panic!("{message}"),
    );
}

/// The rod's enclosure widening a millionfold keeps
/// `a_rod_through_a_drum_wall_builds_at_its_sliced_volume`'s pin green:
/// the pin says `pad=` with no digits, so the one number that makes the
/// line `BAD` is the one it does not read.
#[test]
fn a_pad_grown_a_millionfold_keeps_the_rods_pin_green() {
    test_utils::pinned_red::row(
        1e-6,
        &[1e-6],
        "the rod through the drum's wall",
        &[
            "drum ∪ rod: OK BAD t2=true t3p=true cert=true operand=true v=1.667873349 pad=",
            "want=1.667873349",
        ],
        || {
            panic!(
                "drum ∪ rod: OK BAD t2=true t3p=true cert=true operand=true v=1.667873349 pad=3.7e-1 want=1.667873349"
            )
        },
    );
}

const BOSS_SAYS: [&str; 3] = [
    "boss_union: the decisions the builds answered with different signs are not the owed ones",
    "\"period_near_mark\", 0.0, 8.881784197001252e-16)], mode: Some(InStep), seen: {",
    "left: [\"period_near_mark\", \"period_near_mark\"]\n right: []",
];

fn boss_message(flips: &str, names: &str) -> String {
    format!(
        "boss_union: the decisions the builds answered with different signs are not the owed \
         ones, in order ([]) — a row owes none unless it names them — Found {{ flips: [{flips}], \
         mode: Some(InStep), seen: {{}} }}\n  left: [{names}]\n right: []"
    )
}

/// A third `period_near_mark` flip turns `revalue_boss_union`'s pin red:
/// the names list is held whole.
#[test]
#[should_panic(expected = "it stopped otherwise")]
fn a_third_flip_turns_the_boss_union_pin_red() {
    let message = boss_message(
        "(1, \"period_near_mark\", 0.0, 8.881784197001252e-16), (2, \"period_near_mark\", 0.0, 8.881784197001252e-16), (3, \"period_near_mark\", 0.0, 8.881784197001252e-16)",
        "\"period_near_mark\", \"period_near_mark\", \"period_near_mark\"",
    );
    test_utils::pinned_red::row(1e-6, &[1e-6], "boss_union in step", &BOSS_SAYS, || {
        panic!("{message}")
    });
}

/// The first of the two flips moving off its exact zero keeps the pin
/// green: only the last flip's margin is spelled.
#[test]
fn the_first_flips_margin_is_not_read_by_the_boss_union_pin() {
    let message = boss_message(
        "(1, \"period_near_mark\", 5.0e-7, 8.881784197001252e-16), (2, \"period_near_mark\", 0.0, 8.881784197001252e-16)",
        "\"period_near_mark\", \"period_near_mark\"",
    );
    test_utils::pinned_red::row(1e-6, &[1e-6], "boss_union in step", &BOSS_SAYS, || {
        panic!("{message}")
    });
}
