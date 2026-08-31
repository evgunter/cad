//! The cross-lap joint (#91 C1, Evan's pick): two beams, each notched
//! half-depth by a subtract — each notched beam is itself a boolean
//! result, so the joint is #90's boolean-of-boolean payload made
//! visible. Geometry is the `issue86_double_subtract` crossing-slots
//! class promoted to real joint proportions.
//!
//! M5 S1 status: the declared mated union BUILDS through the join-stage
//! declared-REST zip — the contact patches (notch floor/ceiling and
//! the four flush walls) are removed as interior and the seam is
//! fused, volume exactly 2·(BEAM_VOL − NOTCH_VOL). The stop ships the
//! GLUED union (watertight STL + STEP exported by the tour like every
//! stop body). UNDECLARED, the mate still refuses at the coincidence
//! door (rung (b) — value equality never classifies; the ladder is
//! law) — that refusal stays narrated: it is the declared/undeclared
//! contrast the joint exists to demonstrate.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::{Affine3, Vec3};
use pncad::topo::BooleanBody;

use crate::bool_bodies::slab;
use crate::booleans::{check, describe, expect_seamed, try_subtract, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// Beam cross-section 0.5 x 0.5, length 4; notches half-depth (0.25),
/// exactly beam-width wide, at the crossing x,y ∈ [1.75, 2.25]. All
/// notch cutters overshoot so no cutter plane coincides with a beam
/// plane except the intended crossing walls.
const NOTCH_VOL: f64 = 0.5 * 0.5 * 0.25;
const BEAM_VOL: f64 = 4.0 * 0.5 * 0.5;

/// Beam A runs along x, notched from the TOP at the crossing.
fn beam_a<S: Scalar>(tol: Tol) -> BooleanBody<S> {
    let beam: pncad::topo::Body<S> = slab((0.0, 4.0), (1.75, 2.25), (0.0, 0.5), tol);
    let cutter = slab((1.75, 2.25), (1.5, 2.5), (0.25, 0.75), tol);
    expect_seamed(
        "beam A notch subtract",
        check(try_subtract(&beam, &cutter, tol), BEAM_VOL - NOTCH_VOL, tol),
        BEAM_VOL - NOTCH_VOL,
    )
}

/// Beam B runs along y, notched from the BOTTOM — the two half-depth
/// notches interlock.
fn beam_b<S: Scalar>(tol: Tol) -> BooleanBody<S> {
    let beam: pncad::topo::Body<S> = slab((1.75, 2.25), (0.0, 4.0), (0.0, 0.5), tol);
    let cutter = slab((1.5, 2.5), (1.75, 2.25), (-0.25, 0.25), tol);
    expect_seamed(
        "beam B notch subtract",
        check(try_subtract(&beam, &cutter, tol), BEAM_VOL - NOTCH_VOL, tol),
        BEAM_VOL - NOTCH_VOL,
    )
}

/// The joint's boolean work, generic (the Probe sweep runs the same
/// ops): both notched beams, the naive-union refusal pin, the DECLARED
/// glued union (M5 S1 — the REST zip), and the lifted exploded copy.
/// Returns the undeclared refusal's narration string for the f64 stop
/// captions.
pub(crate) fn build<S: Scalar>(
    tol: Tol,
) -> (
    BooleanBody<S>,
    BooleanBody<S>,
    BooleanBody<S>,
    pncad::topo::Body<S>,
    String,
) {
    let a = beam_a::<S>(tol);
    let b = beam_b::<S>(tol);

    // The UNDECLARED union refuses at the coincidence door (rung (b);
    // post-PR 5 value equality never even classifies).
    let expected = 2.0 * (BEAM_VOL - NOTCH_VOL);
    let naive = check(try_union(&a.body, &b.body, tol), expected, tol);
    let refusal = describe(&naive, expected);
    if !matches!(naive, crate::booleans::Verdict::Refused(_)) {
        panic!(
            "the UNDECLARED mated union no longer refuses ({refusal}) — \
             value-equality must never glue (ladder rung (b)); regression"
        );
    }
    println!("   mated-union WITHOUT declarations: {refusal}");
    // DECLARED, the union BUILDS (M5 S1): the join-stage REST zip
    // removes the coincident contact patches and fuses the seam —
    // exact dyadic volume additivity (interiors disjoint).
    let glued = expect_seamed(
        "declared mated union (M5 S1 REST zip)",
        check(
            crate::booleans::try_union_declared(&a.body, &b.body, tol),
            expected,
            tol,
        ),
        expected,
    );
    println!(
        "   mated-union WITH the mate declared: GLUED (volume {expected} exactly) — \
         the M5 S1 declared-REST zip; the former join-stage refusal is retired \
         (crosslap_rest.rs pins both doors)"
    );

    // Exploded: beam B lifted by a rigid transform (#84 — every moved
    // edge witness is re-minted, and the moved body revalidates).
    let lift = Affine3::translation(Vec3::new(
        S::from_f64(0.0),
        S::from_f64(0.0),
        S::from_f64(1.25),
    ));
    let b_lifted = pncad::topo::transform_rigid(&b.body, &lift, tol).expect("lift beam B");
    (a, b, glued, b_lifted, refusal)
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let (a, _b, glued, b_lifted, refusal) = build::<f64>(tol);
    let note = format!(
        "each beam is a boolean RESULT (notch subtract, volume {} — observed \
         bit-exact, gated 1e-9); undeclared the mate refuses at the coincidence \
         door ({refusal}); DECLARED, the M5 S1 REST zip GLUES the joint — one \
         watertight body, volume {} exactly (2·(beam − notch); interiors \
         disjoint, nothing discarded)",
        BEAM_VOL - NOTCH_VOL,
        2.0 * (BEAM_VOL - NOTCH_VOL)
    );
    vec![
        Stop {
            name: "crosslap",
            caption: "cross-lap (glued)".to_string(),
            // Off the sheet: `twopeg` shows this cell's whole claim —
            // a glued union across a declared coincident PLANAR
            // contact — and adds two CYLINDRICAL contacts on a shape
            // that is more part-like. The cross-lap keeps its
            // standalone render, its narration, and every non-sheet
            // role (corpus, latency, exports).
            montage: false,
            story: "cross-lap joint, glued: two half-depth-notched beams interlocked and \
                    UNIONED into one body through the declared-REST zip (M5 S1) — the \
                    contact patches are interior now; only the seam edges remain",
            ops: "2 x (extrude beam, extrude cutter -> subtract); declared mate -> union",
            delta: 1e-2,
            note: Some(note.clone()),
            view: View {
                elev: 22.0,
                azim: -60.0,
                up: 'z',
            },
            bodies: vec![SceneBody::seamed(
                "crosslap_glued",
                [0.72, 0.53, 0.30],
                glued.body.clone(),
                glued.contacts.clone(),
            )],
        },
        Stop {
            name: "crosslap_exploded",
            caption: "cross-lap (exploded)".to_string(),
            // Off the sheet with its own glued cell (see above);
            // `twopeg_apart` is the apart framing the sheet now
            // carries.
            montage: false,
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
                SceneBody::plain("crosslap_exp_b", [0.55, 0.42, 0.65], b_lifted),
            ],
        },
    ]
}
