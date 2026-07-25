//! The cross-lap joint (#91 C1, Evan's pick): two beams, each notched
//! half-depth by a subtract — each notched beam is itself a boolean
//! result, so the joint is #90's boolean-of-boolean payload made
//! visible. Geometry is the `issue86_double_subtract` crossing-slots
//! class promoted to real joint proportions.
//!
//! M4 PR 5 status (the crosslap wire did NOT fire — refusal moved
//! one stage deeper, honestly): UNDECLARED, the mated union now
//! refuses at the COINCIDENCE door (rung (b) — value equality never
//! classifies); DECLARED, classification opens but the union still
//! refuses typed at the JOIN — the mate is a pure REST contact (the
//! notches interlock exactly, interiors disjoint), the M3 envelope's
//! boundary-on-boundary class (iii), which is a join-stage gap, not
//! a classification gap (the corner-table legs glue because they
//! OVERLAP into the top; the crosslap has no overlap to seam).
//! Both refusals are narrated; the joint ships as two mated bodies;
//! the kernel wire (`crosslap_rest.rs`) stays armed on the join
//! frontier.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Vec3};
use topo::BooleanBody;

use crate::bool_bodies::slab;
use crate::booleans::{check, describe, expect_seamed, try_subtract, try_union};
use crate::{SceneBody, Stop, View};

/// Beam cross-section 0.5 x 0.5, length 4; notches half-depth (0.25),
/// exactly beam-width wide, at the crossing x,y ∈ [1.75, 2.25]. All
/// notch cutters overshoot so no cutter plane coincides with a beam
/// plane except the intended crossing walls.
const NOTCH_VOL: f64 = 0.5 * 0.5 * 0.25;
const BEAM_VOL: f64 = 4.0 * 0.5 * 0.5;

/// Beam A runs along x, notched from the TOP at the crossing.
fn beam_a() -> BooleanBody<f64> {
    let beam = slab((0.0, 4.0), (1.75, 2.25), (0.0, 0.5));
    let cutter = slab((1.75, 2.25), (1.5, 2.5), (0.25, 0.75));
    expect_seamed(
        "beam A notch subtract",
        check(try_subtract(&beam, &cutter), BEAM_VOL - NOTCH_VOL),
        BEAM_VOL - NOTCH_VOL,
    )
}

/// Beam B runs along y, notched from the BOTTOM — the two half-depth
/// notches interlock.
fn beam_b() -> BooleanBody<f64> {
    let beam = slab((1.75, 2.25), (0.0, 4.0), (0.0, 0.5));
    let cutter = slab((1.5, 2.5), (1.75, 2.25), (-0.25, 0.25));
    expect_seamed(
        "beam B notch subtract",
        check(try_subtract(&beam, &cutter), BEAM_VOL - NOTCH_VOL),
        BEAM_VOL - NOTCH_VOL,
    )
}

pub fn stops() -> Vec<Stop> {
    let a = beam_a();
    let b = beam_b();

    // The UNDECLARED union refuses at the coincidence door (rung (b);
    // post-PR 5 value equality never even classifies).
    let expected = 2.0 * (BEAM_VOL - NOTCH_VOL);
    let naive = check(try_union(&a.body, &b.body), expected);
    let refusal = describe(&naive, expected);
    if !matches!(naive, crate::booleans::Verdict::Refused(_)) {
        panic!(
            "the UNDECLARED mated union no longer refuses ({refusal}) — \
             value-equality must never glue (ladder rung (b)); regression"
        );
    }
    println!("   mated-union WITHOUT declarations: {refusal}");
    // DECLARED, classification opens — and the union still refuses
    // typed at the JOIN: the mate is a pure REST contact (interiors
    // disjoint), the M3 envelope's boundary-on-boundary class (iii).
    // A join-stage gap, distinct from PR 5's declared-classification
    // opener; the kernel wire stays armed on it.
    let declared = check(
        crate::booleans::try_union_declared(&a.body, &b.body),
        expected,
    );
    let declared_refusal = describe(&declared, expected);
    if !matches!(declared, crate::booleans::Verdict::Refused(_)) {
        panic!(
            "the DECLARED mated union now builds ({declared_refusal}) — the \
             crosslap_rest.rs wire should have fired; upgrade this stop to ship \
             the glued union"
        );
    }
    println!(
        "   mated-union WITH the mate declared: {declared_refusal}\n\
         \x20  — classification OPENED (M4 PR 5); the remaining refusal is the \
         join-stage boundary-on-boundary REST gap (M3 envelope iii): no overlap \
         to seam, every mate segment lies ON existing edges"
    );

    // Exploded: beam B lifted by a rigid transform (#84 — every moved
    // edge witness is re-minted, and the moved body revalidates).
    let lift = Affine3::translation(Vec3::new(0.0, 0.0, 1.25));
    let b_lifted = topo::transform_rigid(&b.body, &lift).expect("lift beam B");

    let note = format!(
        "each beam is a boolean RESULT (notch subtract, volume {} — observed \
         bit-exact, gated 1e-9); undeclared the mate refuses at the coincidence \
         door; DECLARED (M4 PR 5) classification opens and the join-stage REST gap \
         refuses typed ({declared_refusal}) — shipped as two mated bodies, wire armed",
        BEAM_VOL - NOTCH_VOL
    );
    vec![
        Stop {
            name: "crosslap",
            caption: "cross-lap (assembled)".to_string(),
            montage: true,
            story: "cross-lap joint, assembled: two half-depth-notched beams interlocked \
                    — mated flush, shipped as two bodies (declared classification opened \
                    with M4 PR 5; the glued union waits on the join-stage REST lane)",
            ops: "2 x (extrude beam, extrude cutter -> subtract); mate by construction",
            delta: 1e-2,
            note: Some(note.clone()),
            view: View {
                elev: 22.0,
                azim: -60.0,
                up: 'z',
            },
            bodies: vec![
                SceneBody::seamed(
                    "crosslap_a",
                    [0.72, 0.53, 0.30],
                    a.body.clone(),
                    a.contacts.clone(),
                ),
                SceneBody::seamed(
                    "crosslap_b",
                    [0.55, 0.42, 0.65],
                    b.body.clone(),
                    b.contacts.clone(),
                ),
            ],
        },
        Stop {
            name: "crosslap_exploded",
            caption: "cross-lap (exploded)".to_string(),
            montage: true,
            story: "the same joint exploded: beam B lifted by a rigid transform \
                    (re-minted witnesses, #84), the interlocking notches visible",
            ops: "transform_rigid(beam B, +1.25 z) — transform witnesses re-minted",
            delta: 1e-2,
            note: None,
            view: View {
                elev: 22.0,
                azim: -60.0,
                up: 'z',
            },
            bodies: vec![
                SceneBody::seamed("crosslap_exp_a", [0.72, 0.53, 0.30], a.body, a.contacts),
                // The lifted copy is a TRANSFORM result, not a boolean
                // result — its contacts don't survive the move; it
                // validates through the plain tier-3 gate.
                SceneBody::plain_planar("crosslap_exp_b", [0.55, 0.42, 0.65], b_lifted),
            ],
        },
    ]
}
