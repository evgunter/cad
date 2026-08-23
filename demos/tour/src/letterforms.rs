//! The shadow-silhouette solid (#91 C2): one solid whose orthographic
//! shadows are two, then three, different profiles — the tour's FIRST
//! `intersect`, and (for the 3-way) intersect-of-intersect, the
//! boolean-of-boolean lane.
//!
//! All three operands are extrudes of pure POLYGON letterforms (an
//! "H" on the xy plane extruded +z, a "T" on the yz plane extruded
//! +x, a "C" on the zx plane extruded +y — all three letter boxes the
//! same 3-unit height): every face a plane, every edge a line — the
//! operand gate (`reduce::gate_operand_kinds`) passes and no curved
//! geometry goes near a boolean at all (round letterforms are the M5
//! upgrade). A×Z was probed (#91, 2026-07-25)
//! and refused typed then — banked as the acceptance fixture for the
//! cookie-cutter role resolver's vertex-only-probing gap, closed by
//! #108; it now runs as its own tour stop (`az.rs`).
//!
//! THE DESIGN RULE (from the #91 build evidence, and it generalizes to
//! every boolean scene in this tour): operands must not share
//! coincident planes. The NAIVE letterforms — T's stem spanning
//! exactly the H's bar band (both y ∈ [1.25, 1.75]) — hit the pinned
//! flush-plane gap: the 2-way intersect "succeeds" tiers 1–2 but tier
//! 3′ refuses `DescriptionNotAdjacent` on the shared-plane seam edges,
//! and its 3-way consumer refuses `NonMaximalFaces` typed. Decoupling
//! the dimensions by 1/16 (invisible at montage scale, still
//! exact-dyadic) turns every tier green. Both variants run here — the
//! refusal IS demo material (the coincidence ladder made visible; the
//! intentionally-flush variant is exactly what M4 PR 5's Declare will
//! glue).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::{Point3, Vec3};
use pncad::profile::SketchPlane;
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanBody, BooleanResult, validate_pseudomanifold};

use crate::booleans::{check, expect_seamed, try_intersect};
use crate::paths::path_polygon;
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::validated;
use pncad::geom_core::Tol;

/// "H" sketch: xy plane at z = -0.25, extruded 3.5 (z ∈ [-0.25, 3.25] —
/// covering the full-height T).
fn h_prism<S: Scalar>(poly: &[(f64, f64)], tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(
        Point3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(-0.25)),
        Vec3::new(S::from_f64(1.0), S::from_f64(0.0), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(1.0), S::from_f64(0.0)),
    );
    extrude(
        &validated(plane, vec![path_polygon(poly, tol)], tol).expect("letterform profile"),
        Extrusion::Distance(S::from_f64(3.5)),
        tol,
    )
    .expect("extrude H")
    .body
}

/// "T" sketch: yz plane at x = -0.25, extruded 2.5 (x ∈ [-0.25, 2.25]).
fn t_prism<S: Scalar>(poly: &[(f64, f64)], tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(
        Point3::new(S::from_f64(-0.25), S::from_f64(0.0), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(1.0), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(1.0)),
    );
    extrude(
        &validated(plane, vec![path_polygon(poly, tol)], tol).expect("letterform profile"),
        Extrusion::Distance(S::from_f64(2.5)),
        tol,
    )
    .expect("extrude T")
    .body
}

/// Third letterform: a blocky "C" on the zx plane at y = -0.5,
/// extruded 4 along +y — H x T x C, the third shadow a real letter.
/// Every C plane is axis-aligned but shares NO carrier with any H/T
/// plane (x planes {-1/16, 3/8, 33/16} vs H's {0, 7/16, 1/2, 3/2,
/// 25/16, 2} and T's caps {-1/4, 9/4}; z planes {3/16, 13/16, 39/16,
/// 49/16} vs T's {1/8, 41/16, 21/8, 25/8} and H's caps {-1/4, 13/4})
/// — the no-coincident-planes rule, so the intersect-of-intersect
/// runs the clean lane. Sized so the y-shadow is the near-unclipped
/// C (only its 1/16 x-overshoot margins are trimmed), while the H
/// shadow stays COMPLETE: the C's spine spans the stem's z-band over
/// H's full-height left column, and its top arm spans every x at the
/// T-bar band.
fn c_prism<S: Scalar>(tol: Tol) -> Body<S> {
    let plane = SketchPlane::from_frame(
        Point3::new(S::from_f64(0.0), S::from_f64(-0.5), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(1.0)),
        Vec3::new(S::from_f64(1.0), S::from_f64(0.0), S::from_f64(0.0)),
    );
    // (z, x), counterclockwise; right-opening notch makes the C.
    let poly = [
        (0.1875, -0.0625),
        (3.0625, -0.0625),
        (3.0625, 2.0625),
        (2.4375, 2.0625),
        (2.4375, 0.375),
        (0.8125, 0.375),
        (0.8125, 2.0625),
        (0.1875, 2.0625),
    ];
    extrude(
        &validated(plane, vec![path_polygon(&poly, tol)], tol).expect("letterform profile"),
        Extrusion::Distance(S::from_f64(4.0)),
        tol,
    )
    .expect("extrude C")
    .body
}

/// NAIVE letterforms: textbook proportions, T stem = H bar band, both
/// letter boxes 3 tall (T: z ∈ [0.125, 3.125]).
const H_NAIVE: [(f64, f64); 12] = [
    (0.0, 0.0),
    (0.5, 0.0),
    (0.5, 1.25),
    (1.5, 1.25),
    (1.5, 0.0),
    (2.0, 0.0),
    (2.0, 3.0),
    (1.5, 3.0),
    (1.5, 1.75),
    (0.5, 1.75),
    (0.5, 3.0),
    (0.0, 3.0),
];
const T_NAIVE: [(f64, f64); 8] = [
    (1.25, 0.125),
    (1.75, 0.125),
    (1.75, 2.625),
    (3.25, 2.625),
    (3.25, 3.125),
    (-0.25, 3.125),
    (-0.25, 2.625),
    (1.25, 2.625),
];

/// DECOUPLED letterforms: every cross-operand-coincident plane pair
/// offset by 1/16 (T stem straddles H's bar planes; same-carrier
/// splits offset) — imperceptible, still exact-dyadic.
const H_DECOUPLED: [(f64, f64); 12] = [
    (0.0, 0.0),
    (0.5, 0.0),
    (0.5, 1.25),
    (1.5, 1.25),
    (1.5, 0.0625),
    (2.0, 0.0625),
    (2.0, 2.9375),
    (1.5625, 2.9375),
    (1.5625, 1.75),
    (0.4375, 1.75),
    (0.4375, 3.0),
    (0.0, 3.0),
];
const T_DECOUPLED: [(f64, f64); 8] = [
    (1.1875, 0.125),
    (1.8125, 0.125),
    (1.8125, 2.625),
    (3.25, 2.625),
    (3.25, 3.125),
    (-0.25, 3.125),
    (-0.25, 2.5625),
    (1.1875, 2.5625),
];

/// 2-way volume oracle for the decoupled pair: independent derivation
/// as ∫ len_x(H at y) · len_z(T at y) dy over the seven y-cells —
/// exactly 577/128 (dyadic; hand computation and the exact-fraction
/// script in the #91 revision pass agree).
const V_2WAY: f64 = 4.5078125;
/// 2-way volume oracle for the NAIVE pair (the declared-glue
/// after-shot), same derivation: len_x(H) is 1 on the legs bands and
/// 2 on the bar band y ∈ [1.25, 1.75]; len_z(T) is 0.5 off-stem and
/// 3.0 on the stem band (= the SAME y-band, that's the coincidence):
/// 1.25·0.5 + 0.5·6 + 1.25·0.5 = 17/4 exactly (dyadic).
const V_2WAY_NAIVE: f64 = 4.25;
/// 3-way (× C) volume: independent derivation as
/// ∬ |Y_H(x) ∩ Y_T(z)| over the C region (exact-fraction polygon
/// clipping over the 5×3 cell grid) — exactly 11461/4096 (dyadic).
const V_3WAY: f64 = 2.798095703125;

/// The coincident-plane variant, narrated as the M4 PR 5
/// before/after (the #91 review pin UPGRADED when the Declare wire
/// fired): UNDECLARED, the flush H × T refuses typed at the
/// coincidence door — post-retirement, value-equality never even
/// CLASSIFIES (rung (b)); the old "tiers 1–2 pass, tier 3′ refuses
/// DescriptionNotAdjacent" posture is gone because the op refuses
/// EARLIER and cleaner. DECLARED, the same geometry glues to the
/// exact silhouette volume.
fn narrate_naive<S: Scalar>(tol: Tol) {
    println!("   -- the coincidence ladder, made visible (before/after, M4 PR 5) --");
    // BEFORE: undeclared flush contact refuses typed, loudly.
    let (h, t) = (h_prism::<S>(&H_NAIVE, tol), t_prism::<S>(&T_NAIVE, tol));
    match try_intersect(&h, &t, tol) {
        Err(e) => {
            assert!(
                format!("{e:?}").contains("UndeclaredCoincidence"),
                "the naive refusal moved off the pinned \
                 UndeclaredCoincidence class: {e:?}"
            );
            println!(
                "   NAIVE H x T (T stem spanning exactly H's bar band, y in [1.25, 1.75]):\n\
                 \x20     refuses typed at the coincidence door (UndeclaredCoincidence):\n\
                 \x20     value-equality never glues (ladder rung (b), M4 PR 5 narrowing)"
            );
        }
        Ok(_) => panic!(
            "REGRESSION: the UNDECLARED coincident-plane H x T ran — \
             value-equality must never classify coincidence"
        ),
    }
    // AFTER: the SAME geometry with the flush contact DECLARED —
    // glued, certified, exact.
    let r =
        crate::booleans::try_intersect_declared(&h, &t, tol).expect("declared naive 2-way runs");
    let BooleanResult::Body(bb) = r else {
        panic!("declared naive 2-way cannot be empty");
    };
    let v = pncad::topo::mass_properties(&bb.body, tol)
        .expect("declared naive volume")
        .volume
        .f();
    assert!(
        (v - V_2WAY_NAIVE).abs() < 1e-9,
        "declared naive 2-way volume {v} vs {V_2WAY_NAIVE}"
    );
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "the declared naive 2-way certifies at the 3' gate"
    );
    println!(
        "      the SAME geometry with the flush contact DECLARED: glued, 3' \
         certified, volume {v} — coincidence is intent (recipe data), never a \
         value guess"
    );
}

/// Builds the decoupled 2-way and 3-way results.
pub(crate) fn build<S: Scalar>(tol: Tol) -> (BooleanBody<S>, BooleanBody<S>) {
    narrate_naive::<S>(tol);
    let two = expect_seamed(
        "decoupled H x T intersect",
        check(
            try_intersect(
                &h_prism::<S>(&H_DECOUPLED, tol),
                &t_prism::<S>(&T_DECOUPLED, tol),
                tol,
            ),
            V_2WAY,
            tol,
        ),
        V_2WAY,
    );
    let three = expect_seamed(
        "3-way intersect (result x diamond)",
        check(try_intersect(&two.body, &c_prism(tol), tol), V_3WAY, tol),
        V_3WAY,
    );
    (two, three)
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let (two, three) = build::<f64>(tol);
    // The shadow PROOF renders (standalone, not montage panels): the
    // 3-way solid viewed straight down each axis — orthographic, so
    // each frame IS the shadow: an H (z), a T (x), a C (y).
    let shadow = |axis: char, caption: &str, elev: f64, azim: f64| Stop {
        name: match axis {
            'z' => "silhouette3_shadow_z",
            'x' => "silhouette3_shadow_x",
            _ => "silhouette3_shadow_y",
        },
        caption: caption.to_string(),
        montage: false,
        story: "shadow proof: the 3-way solid viewed straight down one axis",
        ops: "same body as silhouette3; orthographic axis view",
        delta: 1e-2,
        note: None,
        view: View {
            elev,
            azim,
            up: 'z',
        },
        bodies: vec![SceneBody::seamed(
            match axis {
                'z' => "silhouette3_shadow_z",
                'x' => "silhouette3_shadow_x",
                _ => "silhouette3_shadow_y",
            },
            [0.80, 0.44, 0.30],
            three.body.clone(),
            three.contacts.clone(),
        )],
    };
    let shadows = vec![
        shadow('z', "z-shadow: H", 90.0, -90.0),
        shadow('x', "x-shadow: T", 0.0, 0.0),
        shadow(
            'y',
            "y-shadow: C (only its 1/16 x-overshoot margins are trimmed)",
            0.0,
            -90.0,
        ),
    ];
    let naive_note = "the NAIVE variant (coincident planes) is narrated above: tier 3' refusal + \
         typed 3-way refusal — the design rule is 'operands never share coincident \
         planes' (1/16 decoupling, invisible at this scale)";
    vec![
        Stop {
            name: "silhouette",
            caption: "silhouette (H x T)".to_string(),
            // Montage carries only the 3-way (#91 revision note 5);
            // this stop stays in the tour + standalone render.
            montage: false,
            story: "shadow-silhouette solid: its z-shadow is an H, its x-shadow is a T \
                    — the tour's first `intersect`",
            ops: "extrude H (xy sketch, +z) x extrude T (yz sketch, +x) -> 1 intersect node",
            delta: 1e-2,
            note: Some(format!(
                "volume {V_2WAY} (observed bit-exact, gated 1e-9); {naive_note}"
            )),
            view: View {
                elev: 24.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::seamed(
                "silhouette",
                [0.85, 0.62, 0.28],
                two.body,
                two.contacts,
            )],
        },
        Stop {
            name: "silhouette3",
            caption: "silhouette3 (H x T x C)".to_string(),
            montage: true,
            story: "three letter shadows: the H x T solid intersected AGAIN with a \
                    blocky C prism along +y — intersect-of-intersect, boolean-of-boolean",
            ops: "silhouette result x extrude C (zx sketch, +y) -> 1 more intersect node",
            delta: 1e-2,
            note: Some(format!(
                "volume {V_3WAY} (observed bit-exact, gated 1e-9); the C's planes \
                 are axis-aligned yet share NO carrier with any H/T plane, so the \
                 boolean-of-boolean runs the clean coplanarity-free lane"
            )),
            view: View {
                elev: 24.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::seamed(
                "silhouette3",
                [0.80, 0.44, 0.30],
                three.body,
                three.contacts,
            )],
        },
    ]
    .into_iter()
    .chain(shadows)
    .collect()
}
