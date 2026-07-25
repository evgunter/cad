//! The shadow-silhouette solid (#91 C2): one solid whose orthographic
//! shadows are two, then three, different profiles — the tour's FIRST
//! `intersect`, and (for the 3-way) intersect-of-intersect, the
//! boolean-of-boolean lane.
//!
//! Both letterform operands are extrudes of pure POLYGON profiles (an
//! "H" on the xy plane extruded +z, a "T" on the yz plane extruded
//! +x): every face a plane, every edge a line — `gate_planar` passes,
//! no curved geometry near a boolean (round letterforms are the M5
//! upgrade). A×Z was probed (#91, 2026-07-25) and refuses typed today
//! — banked as the acceptance fixture for the cookie-cutter role
//! resolver's vertex-only-probing gap; do not attempt it here.
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

use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanBody, BooleanResult, validate_pseudomanifold};

use crate::booleans::{check, expect_seamed, try_intersect};
use crate::{SceneBody, Stop, View};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn validated(plane: SketchPlane<f64>, poly: &[(f64, f64)]) -> ValidatedProfile<f64> {
    let lp = ProfileLoop::polygon(poly.iter().map(|&(x, y)| p2(x, y)).collect::<Vec<_>>());
    Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .expect("letterform profile")
}

/// "H" sketch: xy plane at z = -0.25, extruded 2.5 (z ∈ [-0.25, 2.25]).
fn h_prism(poly: &[(f64, f64)]) -> Body<f64> {
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, -0.25),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    extrude(&validated(plane, poly), Extrusion::Distance(2.5))
        .expect("extrude H")
        .body
}

/// "T" sketch: yz plane at x = -0.25, extruded 2.5 (x ∈ [-0.25, 2.25]).
fn t_prism(poly: &[(f64, f64)]) -> Body<f64> {
    let plane = SketchPlane::from_frame(
        Point3::new(-0.25, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    extrude(&validated(plane, poly), Extrusion::Distance(2.5))
        .expect("extrude T")
        .body
}

/// Third silhouette: 45°-chamfer diamond on the zx plane at y = -0.5,
/// extruded 4 along +y — its crossing is coplanarity-free by
/// construction, so the intersect-of-intersect runs the clean lane.
fn diamond_prism() -> Body<f64> {
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let poly = [
        (1.0, -0.8125),
        (2.8125, 1.0),
        (1.0, 2.8125),
        (-0.8125, 1.0),
    ];
    extrude(&validated(plane, &poly), Extrusion::Distance(4.0))
        .expect("extrude diamond")
        .body
}

/// NAIVE letterforms: textbook proportions, T stem = H bar band.
const H_NAIVE: [(f64, f64); 12] = [
    (0.0, 0.0), (0.5, 0.0), (0.5, 1.25), (1.5, 1.25), (1.5, 0.0), (2.0, 0.0),
    (2.0, 3.0), (1.5, 3.0), (1.5, 1.75), (0.5, 1.75), (0.5, 3.0), (0.0, 3.0),
];
const T_NAIVE: [(f64, f64); 8] = [
    (1.25, 0.25), (1.75, 0.25), (1.75, 1.5), (3.25, 1.5),
    (3.25, 2.0), (-0.25, 2.0), (-0.25, 1.5), (1.25, 1.5),
];

/// DECOUPLED letterforms: every cross-operand-coincident plane pair
/// offset by 1/16 (T stem straddles H's bar planes; same-carrier
/// splits offset) — imperceptible, still exact-dyadic.
const H_DECOUPLED: [(f64, f64); 12] = [
    (0.0, 0.0), (0.5, 0.0), (0.5, 1.25), (1.5, 1.25), (1.5, 0.0625), (2.0, 0.0625),
    (2.0, 2.9375), (1.5625, 2.9375), (1.5625, 1.75), (0.4375, 1.75), (0.4375, 3.0), (0.0, 3.0),
];
const T_DECOUPLED: [(f64, f64); 8] = [
    (1.1875, 0.25), (1.8125, 0.25), (1.8125, 1.5), (3.25, 1.5),
    (3.25, 2.0), (-0.25, 2.0), (-0.25, 1.4375), (1.1875, 1.4375),
];

/// 2-way volume oracle for the decoupled pair (exact dyadic, derived
/// in the #91 build evidence).
const V_2WAY: f64 = 3.111328125;
/// 3-way volume, pinned exact-dyadic from the same verified build.
const V_3WAY: f64 = 3.008056640625;

/// The naive variant, narrated: tiers 1–2 pass, tier 3′ refuses on the
/// coincident-plane seam edges, and the 3-way consumer refuses typed.
fn narrate_naive() {
    println!("   -- the coincidence ladder, made visible --");
    let r = try_intersect(&h_prism(&H_NAIVE), &t_prism(&T_NAIVE)).expect("naive 2-way runs");
    let BooleanResult::Body(bb) = r else {
        panic!("naive 2-way cannot be empty");
    };
    let t3 = validate_pseudomanifold(&bb.body, &bb.contacts);
    println!(
        "   NAIVE H x T (T stem spanning exactly H's bar band, y in [1.25, 1.75]):\n\
         \x20     tiers 1-2 pass, but tier 3' refuses on the shared-plane seam edges:\n\
         \x20     {}",
        match &t3 {
            Ok(()) => "Ok — the flush-plane gap CLOSED; update this narration".to_string(),
            Err(errs) => format!("{} errors, all {:?}-class", errs.len(),
                errs.first().map(|e| format!("{e:?}").chars().take(24).collect::<String>())),
        }
    );
    match try_intersect(&bb.body, &diamond_prism()) {
        Err(e) => println!(
            "      and its 3-way consumer refuses typed: {e:?}\n\
             \x20     — value-equality never glues (ladder rung (b)); M4 PR 5's Declare is\n\
             \x20     what makes INTENTIONAL flush contact glue"
        ),
        Ok(_) => println!("      3-way on the naive result now succeeds — update this narration"),
    }
}

/// Builds the decoupled 2-way and 3-way results.
fn build() -> (BooleanBody<f64>, BooleanBody<f64>) {
    narrate_naive();
    let two = expect_seamed(
        "decoupled H x T intersect",
        check(try_intersect(&h_prism(&H_DECOUPLED), &t_prism(&T_DECOUPLED)), V_2WAY),
        V_2WAY,
    );
    let three = expect_seamed(
        "3-way intersect (result x diamond)",
        check(try_intersect(&two.body, &diamond_prism()), V_3WAY),
        V_3WAY,
    );
    (two, three)
}

pub fn stops() -> Vec<Stop> {
    let (two, three) = build();
    let naive_note =
        "the NAIVE variant (coincident planes) is narrated above: tier 3' refusal + \
         typed 3-way refusal — the design rule is 'operands never share coincident \
         planes' (1/16 decoupling, invisible at this scale)";
    vec![
        Stop {
            name: "silhouette",
            caption: "silhouette (H x T)".to_string(),
            story: "shadow-silhouette solid: its z-shadow is an H, its x-shadow is a T \
                    — the tour's first `intersect`",
            ops: "extrude H (xy sketch, +z) x extrude T (yz sketch, +x) -> 1 intersect node",
            delta: 1e-2,
            note: Some(format!("volume exact {V_2WAY}; {naive_note}")),
            view: View { elev: 24.0, azim: -50.0, up: 'z' },
            bodies: vec![SceneBody::seamed(
                "silhouette",
                [0.85, 0.62, 0.28],
                two.body,
                two.contacts,
            )],
        },
        Stop {
            name: "silhouette3",
            caption: "silhouette3 (+ diamond)".to_string(),
            story: "three shadows: the H x T solid intersected AGAIN with a 45-degree \
                    diamond prism along +y — intersect-of-intersect, boolean-of-boolean",
            ops: "silhouette result x extrude diamond (zx sketch, +y) -> 1 more intersect node",
            delta: 1e-2,
            note: Some(format!(
                "volume exact {V_3WAY}; the chamfer crossing is coplanarity-free, so \
                 the boolean-of-boolean runs the clean lane"
            )),
            view: View { elev: 24.0, azim: -50.0, up: 'z' },
            bodies: vec![SceneBody::seamed(
                "silhouette3",
                [0.80, 0.44, 0.30],
                three.body,
                three.contacts,
            )],
        },
    ]
}
