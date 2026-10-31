//! The project-box enclosure (#91 C3): one part carrying the tour's
//! longest boolean-of-boolean chain — cavity subtract, then 6 vent
//! through-slots (each a two-ring tunnel seam through a wall), then 4
//! round screw bosses standing ON the floor, each union gluing its
//! cap-on-floor contact on its decided zero (D10), then a through-bore
//! down each boss and out through the floor: 15 sequential ops, every
//! one against the closed-form volume oracle (a volume + Seamed-kind
//! gate per op; tier 3′ with the result's contact records runs once,
//! on the FINAL body, in `crate::run_body`).
//!
//! A six-turn compression spring stands on one boss, around its bore:
//! ONE `sweep_body` along the whole coil, the long-turn helical sweep,
//! against the Pappus–Guldin volume `A·L` ([`standing_spring`]).
//!
//! Retires the abstract `openbox` stop (this is the cavity story with
//! a real part around it).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};
use pncad::sweep::ExtrudeSide;

use pncad::geom::NurbsCurve3;
use pncad::geom_core::Point3;
use pncad::geom_core::linalg::frame::path_start_frame;
use pncad::prelude::{ConstructedLoop, circle_split};
use pncad::topo::{AtRestBody, BooleanBody};

use crate::bool_bodies::slab;
use crate::booleans::{check, expect_seamed, finished, try_subtract, try_union};
use crate::scalar::Scalar;
use crate::{SceneBody, TourStop, View};
use pncad::authoring::{p2, p3, polygon, validated};
use pncad::geom_core::{RigidMotion, Tol};
use pncad::profile::SketchPlane;
use pncad::sweep::{Extrusion, LoftError, extrude, sweep_body};

/// The bosses' radius (m).
pub(crate) const BOSS_R: f64 = 0.1875;

/// The floor's top (m): walls and floor are 0.25 thick.
pub(crate) const FLOOR_TOP: f64 = 0.25;

/// The bosses' span in `z`: standing on the floor, [`FLOOR_TOP`] to
/// the boss tops.
pub(crate) const BOSS_Z: (f64, f64) = (FLOOR_TOP, 0.875);

/// The bores' radius (m).
pub(crate) const BORE_R: f64 = 0.09375;

/// The boss axes, `(x, y)`, each boss's bore on the same axis.
pub(crate) const BOSS_AXES: [(f64, f64); 4] = [
    (0.625, 0.625),
    (0.625, 1.375),
    (2.375, 0.625),
    (2.375, 1.375),
];

/// The height of material a bore removes: the boss's column, from the
/// floor's underside (`z = 0`) to the boss top.
pub(crate) const BORED_HEIGHT: f64 = BOSS_Z.1;

/// The boss the spring stands on: a far corner's, which the section
/// passes over, so the spring rides whole with the below half.
pub const SPRING_AXIS: (f64, f64) = BOSS_AXES[0];

/// The coil's mean radius (m): its wire clears the bore inside and
/// the boss's rim outside.
const COIL_R: f64 = 0.14;

/// The wire's half-side (m): a square section.
pub const WIRE_H: f64 = 0.025;

/// The coil's rise per turn (m).
const PITCH: f64 = 0.08;

/// The spring's render colour: steel.
pub(crate) const SPRING_COLOR: [f64; 3] = [0.62, 0.62, 0.66];

/// The coil's turns.
const TURNS: u32 = 6;

/// The coil's stations per turn, and its spine's exact points per turn.
pub const PER_TURN: u32 = 32;

/// The coil's spine: a degree-3 interpolant through [`PER_TURN`] exact
/// helix points a turn, about [`SPRING_AXIS`], starting at angle 0 at
/// height `z0`.
fn coil(z0: f64) -> NurbsCurve3<f64> {
    let n = TURNS * PER_TURN;
    let points: Vec<Point3<f64>> = (0..=n)
        .map(|k| {
            let s = f64::from(k) / f64::from(n);
            let a = TAU * f64::from(TURNS) * s;
            Point3::new(
                COIL_R.mul_add(a.cos(), SPRING_AXIS.0),
                COIL_R.mul_add(a.sin(), SPRING_AXIS.1),
                (PITCH * f64::from(TURNS)).mul_add(s, z0),
            )
        })
        .collect();
    NurbsCurve3::interpolate(&points, 3).expect("the coil's spine interpolates")
}

/// The square wire section, centred on the spine.
pub fn square_wire(tol: Tol) -> ConstructedLoop<f64> {
    let h = WIRE_H;
    polygon(&[(-h, -h), (h, -h), (h, h), (-h, h)], tol).expect("the wire section is a square")
}

/// `wire` swept along the coil at `per_turn` stations a turn, from the
/// plane normal to the spine's start tangent (`path_start_frame`).
///
/// The coil starts at the height that puts the square section's lowest
/// corner on the boss top. The frame's axes do not depend on that
/// height, so it is read off the coil at `z0 = 0`. That corner is the
/// spring's lowest point, because every corner climbs along the coil.
/// With `α` the helix angle and `φ` the frame's roll from the Frenet
/// frame, whose normal is horizontal and whose binormal rises at
/// `cos α`, a corner sits `WIRE_H cos α (±sin φ ± cos φ)` above the
/// spine. The roll turns at `τ = sin α cos α / R` per unit of arc, so
/// that height changes at most `√2 WIRE_H cos α τ` per unit of arc,
/// while the spine climbs `sin α`. Every corner therefore rises at no
/// less than `sin α (1 − √2 WIRE_H cos² α / R)`, which is positive:
/// `√2 WIRE_H / R ≈ 0.25`. [`standing_spring`] asserts the walls'
/// convex hulls clear the boss top as well.
pub fn spring<S: Scalar>(
    wire: &[ConstructedLoop<f64>],
    per_turn: u32,
    tol: Tol,
) -> Result<pncad::topo::Body<S>, LoftError> {
    let start_frame = |path: &NurbsCurve3<f64>| {
        let (t0, _) = path.domain();
        let (p, d) = path.ders1(t0);
        path_start_frame(p, d, tol).expect("the coil's start tangent fixes a frame")
    };
    let at_zero = start_frame(&coil(0.0));
    let depth = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
        .into_iter()
        .map(|(u, v)| {
            -at_zero
                .transform_point(Point3::new(u * WIRE_H, v * WIRE_H, 0.0))
                .z
        })
        .fold(f64::NEG_INFINITY, f64::max);
    let path = coil(BOSS_Z.1 + depth);
    let stations = usize::try_from(TURNS * per_turn + 1).expect("a station count");
    // The wire and the spine are built in `f64`, so they enter `S` as
    // the computed values they are, and the start frame is minted at
    // `S` from the spine's.
    let wire: Vec<_> = wire.iter().map(|lp| lp.lift_computed()).collect();
    let (t0, _) = path.domain();
    let (p, d) = path.ders1(t0);
    let place = SketchPlane::from_frame(
        path_start_frame(p.map(S::from_computed), d.map(S::from_computed), tol)
            .expect("the coil's start tangent fixes a frame"),
    );
    sweep_body::<S>(&wire, place, &path, stations, 3, tol).map(|l| l.body)
}

/// The continuum volume of a wire of cross-section `area` swept along
/// the helix: `area · L`, `L = TURNS · √((2π COIL_R)² + PITCH²)`.
/// Exact for any section whose centroid is on the spine, swept in a
/// frame normal to it (Pappus–Guldin: the section's first moment about
/// the spine is zero, so curvature drops out, and roll about the
/// tangent leaves the Jacobian alone).
pub fn coil_volume(area: f64) -> f64 {
    area * f64::from(TURNS) * (TAU * COIL_R).hypot(PITCH)
}

/// The kernel's coil against [`coil_volume`]: the body is a stack of
/// station-to-station skinned slabs, not the continuum tube, and falls
/// SHORT of it (the gap is negative) by a discretization gap, measured
/// 4.1e-6 at [`PER_TURN`] stations a turn and 5.4e-5 at half that. The
/// spine's own length is 1.8e-6 short of the helix's, and that is the
/// floor: at twice [`PER_TURN`] the gap reads 2.0e-6.
pub const COIL_GAP: f64 = 6e-6;

/// The round-wire coil BODY's volume (m³) and area (m²): the slab
/// stack's own, not the continuum tube's, as an independent quadrature
/// of its patches measures them
/// (`tests/projectbox.rs`'s `the_round_coils_own_quadrature_is_the_pinned_oracle`
/// re-derives both from the body on every test run). Where wall 1 is
/// retired, the kernel's certified bracket is held to these.
pub const ROUND_COIL_V: f64 = 0.010_405_810_587_853;
pub const ROUND_COIL_A: f64 = 0.836_392_790_468_53;

/// The spring: a square-wire compression spring of [`TURNS`] turns,
/// one `sweep_body` along the whole coil, standing on [`SPRING_AXIS`]'s
/// boss. Returns the body and its relative gap to [`coil_volume`].
///
/// Round wire is the natural section, and it was wall 1: the swept
/// `circle_split` wire builds, and its volume meets the tour's printed
/// precision at every ε (0.01040581 ± 2.1e-7 at the default ε). The wall is RETIRED, kept live: it asserts the bracket
/// holds the body's own [`ROUND_COIL_V`] and [`ROUND_COIL_A`]. Neither bracket holds [`coil_volume`]`(π WIRE_H²)`,
/// the continuum tube's: the body is a stack of slabs, as the square
/// wire's is, and the retired wall asserts the same slab gap against
/// it, inside [`COIL_GAP`] (measured −4.11e-6).
fn standing_spring(tol: Tol) -> (pncad::topo::Body<f64>, f64) {
    let round: Vec<ConstructedLoop<f64>> = vec![
        circle_split(p2(0.0, 0.0), WIRE_H, 4, 0.0, tol)
            .expect("a round wire")
            .into(),
    ];
    let round = spring::<f64>(&round, PER_TURN, tol).expect("the round-wire coil builds");
    let round = pncad::topo::AtRestBody::validate(pncad::topo::Body::clone(&round), tol)
        .unwrap_or_else(|e| panic!("the round-wire coil passes tier 3: {e:?}"));
    let measured = pncad::topo::mass_properties(&round, tol, crate::printed())
        .unwrap_or_else(|e| panic!("wall 1 (the round-wire spring's volume) refuses: {e:?}"));
    assert_eq!(
        measured.stop,
        pncad::topo::Stop::Met,
        "wall 1 (the round-wire spring's volume) stops short of the printed precision at \
         ε = {:e}: the wall MOVED; re-derive it",
        tol.eps()
    );
    let props = measured.enclosure;
    let want = coil_volume(core::f64::consts::PI * WIRE_H * WIRE_H);
    let gap = (props.volume - want) / want;
    assert!(
        (props.volume - ROUND_COIL_V).abs() <= props.volume_pad
            && (props.surface_area - ROUND_COIL_A).abs() <= props.area_pad,
        "the round-wire coil's V = {} ± {:.1e} and A = {} ± {:.1e} against the body's own \
         {ROUND_COIL_V} and {ROUND_COIL_A}",
        props.volume,
        props.volume_pad,
        props.surface_area,
        props.area_pad
    );
    assert!(
        (-COIL_GAP..0.0).contains(&gap),
        "the round-wire coil's volume {} against A·L = {want}: relative gap {gap:.3e}",
        props.volume
    );
    println!(
        "   wall 1 RETIRED — the round-wire spring's volume {} ± {:.1e} (A·L = {want}, gap \
         {gap:.3e})",
        props.volume, props.volume_pad
    );

    let body = spring::<f64>(&[square_wire(tol)], PER_TURN, tol).expect("the coil builds");
    // The pad bound below is a claim about the kernel's precision, so it
    // asks for the width it needs: a volume half-width of at most
    // `want·1e-6` is a mean displacement of `2e-6·want/A` over the
    // coil's area `A ≈ 8·WIRE_H·L`, and `want = 4·WIRE_H²·L`, so
    // `1e-6·WIRE_H` — or ε, the finest width offered, where that is
    // coarser.
    let pad_width = pncad::quantity::Length::from_meters((1e-6 * WIRE_H).max(tol.eps()));
    let props = pncad::topo::mass_properties(
        &crate::booleans::at_rest(&body, tol),
        tol,
        pncad::topo::Settle::Resolution(pad_width),
    )
    .expect("the coil's volume");
    let want = coil_volume(4.0 * WIRE_H * WIRE_H);
    let gap = (props.enclosure.volume - want) / want;
    assert!(
        (-COIL_GAP..0.0).contains(&gap) && props.enclosure.volume_pad <= want * 1e-6,
        "the coil's volume {} ± {:.1e} against A·L = {want}: relative gap {gap:.3e}",
        props.enclosure.volume,
        props.enclosure.volume_pad
    );

    // It stands: its lowest point is a start-cap corner on the boss
    // top, over the boss's material between the bore and the rim.
    let (low, foot) = body
        .vertex_points()
        .map(|(_, p)| p)
        .map(|p| (p.z, (p.x - SPRING_AXIS.0).hypot(p.y - SPRING_AXIS.1)))
        .fold((f64::INFINITY, 0.0), |a, b| if b.0 < a.0 { b } else { a });
    assert!(
        (low - BOSS_Z.1).abs() <= 1e-12 && BORE_R < foot && foot < BOSS_R,
        "the spring's lowest corner sits at z = {low}, {foot} off the boss axis: \
         on the boss top (z = {}) between the bore ({BORE_R}) and the rim ({BOSS_R})",
        BOSS_Z.1
    );
    // …and nothing between the corners dips below it: each wall is a
    // NURBS patch with positive weights, so it lies in the convex hull
    // of its control points, and those all clear the boss top. (The
    // caps are planar and bounded by the corners above.)
    let mut walls = 0;
    for (_, surface) in body.surfaces() {
        if let pncad::geom::Surface::Nurbs(patch) = surface {
            walls += 1;
            let lowest = patch
                .control()
                .iter()
                .map(|p| p.z)
                .fold(f64::INFINITY, f64::min);
            assert!(
                patch.weights().iter().all(|&w| w > 0.0) && lowest >= BOSS_Z.1 - 1e-12,
                "a wall's control net reaches z = {lowest}, below the boss top {}",
                BOSS_Z.1
            );
        }
    }
    assert_eq!(walls, 4, "the square wire's four walls");
    (body, gap)
}

/// A rod of radius `r` on the vertical axis through `(cx, cy)`, from
/// `z.0` to `z.1`, finished as a boolean operand.
fn rod<S: Scalar>(cx: f64, cy: f64, r: f64, z: (f64, f64), tol: Tol) -> AtRestBody<S> {
    let circle = pncad::profile::circle(p2(cx, cy), S::from_f64(r), tol)
        .expect("the rod radius is positive")
        .into();
    let plane = SketchPlane::from_frame(RigidMotion::axes_xy(p3(0.0, 0.0, z.0)));
    let profile = validated(plane, vec![circle], tol).expect("the rod profile validates");
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: S::from_f64(z.1 - z.0),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .expect("extrude the rod")
    .body;
    finished("the rod", body, tol)
}

/// Builds the 15-op enclosure chain, generic (the Probe sweep runs the
/// same ops); returns the final body and its closed-form volume.
pub(crate) fn build<S: Scalar>(tol: Tol) -> (BooleanBody<S>, f64) {
    // Outer shell 3 x 2 x 1.5, walls/floor 0.25.
    let outer: AtRestBody<S> = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.5), tol);
    let cavity = slab((0.25, 2.75), (0.25, 1.75), (0.25, 2.0), tol);
    let mut vol = 9.0 - 2.5 * 1.5 * 1.25;
    let mut acc: BooleanBody<S> = expect_seamed(
        "cavity subtract",
        check(try_subtract(&outer, &cavity, tol), vol, tol),
        vol,
    );
    let mut ops = 1;

    // Vent through-slots: 3 per long wall, cut clean through both wall
    // faces (two-ring tunnel seams); cutters overshoot into air.
    let xs = [(0.5, 0.875), (1.3125, 1.6875), (2.125, 2.5)];
    for &x in &xs {
        for y in [(-0.25, 0.5), (1.5, 2.25)] {
            let cutter = slab(x, y, (0.5, 1.25), tol);
            vol -= 0.375 * 0.25 * 0.75;
            acc = expect_seamed(
                "vent slot",
                check(try_subtract(&acc.body, &cutter, tol), vol, tol),
                vol,
            );
            ops += 1;
        }
    }

    // Interior screw bosses: 4 cylinders standing on the floor.
    for (cx, cy) in BOSS_AXES {
        vol += PI * BOSS_R * BOSS_R * (BOSS_Z.1 - BOSS_Z.0);
        acc = expect_seamed(
            "boss union",
            check(
                try_union(&acc.body, &rod(cx, cy, BOSS_R, BOSS_Z, tol), tol),
                vol,
                tol,
            ),
            vol,
        );
        ops += 1;
    }

    // A through-bore down each boss's axis and out through the floor,
    // overshooting into air at both ends.
    for (cx, cy) in BOSS_AXES {
        vol -= PI * BORE_R * BORE_R * BORED_HEIGHT;
        let bore = rod(cx, cy, BORE_R, (-0.125, BOSS_Z.1 + 0.25), tol);
        acc = expect_seamed(
            "boss bore",
            check(try_subtract(&acc.body, &bore, tol), vol, tol),
            vol,
        );
        ops += 1;
    }
    assert_eq!(ops, 15);
    (acc, vol)
}

/// The tour stop — the whole enclosure and, beside it, the same body
/// sectioned. ONE cell: the section is not a second part, it is this
/// part with its interior shown, and two independently-scaled panels
/// are the one arrangement that stops a reader laying the halves back
/// onto the whole.
pub fn stop(tol: Tol) -> TourStop {
    let (acc, vol) = build::<f64>(tol);
    let (spring, spring_gap) = standing_spring(tol);
    let spring = finished("the spring", spring, tol);
    let (section_bodies, section_note) = crate::cutaway::sectioned_beside(&acc.body, &spring, tol);
    let note = format!(
        "15 sequential boolean nodes on ONE part (subtract -> 6 tunnel subtracts -> \
         4 boss unions -> 4 bore subtracts), volume within 1e-9 of the closed-form \
         oracle after every op, final V = {vol} (each boss adds pi R^2 x {boss_h}, \
         R = {BOSS_R}; each bore removes pi r^2 x {BORED_HEIGHT}, r = {BORE_R}); \
         each boss stands on the floor, and its union glues the cap-on-floor contact \
         on its decided zero. \
         SPRING: a {TURNS}-turn square-wire coil (mean radius {COIL_R}, wire {wire}, pitch \
         {PITCH}) as ONE sweep_body along a degree-3 interpolant of the helix, from \
         path_start_frame; its volume within {spring_gap:.2e} (relative) of A x L, the \
         Pappus-Guldin volume of the continuum coil (gap cap {COIL_GAP:.0e}: the body is \
         station-to-station skinned slabs); its lowest point a start-cap corner exactly on \
         the boss top, between the bore and the rim. \
         SECTIONED: {section_note}; the spring split by the same plane lies wholly below \
         it and rides with the below half",
        boss_h = BOSS_Z.1 - BOSS_Z.0,
        wire = 2.0 * WIRE_H,
    );
    TourStop {
        name: "projectbox",
        caption: "project box — whole, and sectioned".to_string(),
        montage: true,
        story: "electronics enclosure: cavity, 6 vent through-slots, 4 round floor \
                bosses, 4 through-bores — the tour's longest boolean-of-boolean chain \
                — and beside it the SAME body split by a tilted plane through two \
                bored bosses and pulled apart, a machinist's section whose boss \
                sections are rings around the bores, showing what the whole one hides: \
                a six-turn compression spring standing on a far boss, around its bore",
        ops: "extrude the shell, 7 slab cutters, 4 boss rods, 4 bore rods -> 15 sequential \
              subtract/union nodes; sweep::sweep_body(square wire, 6-turn helix \
              interpolant, path_start_frame) -> the spring; topo::split(tilted plane) of \
              the box -> 2 bodies, of the spring -> below only; 3 transform nodes; \
              topo::plane_section(same plane) -> regions with holes",
        delta: 1e-2,
        note: Some(note),
        // The SECTION's camera, not the box's. A merged cell has one
        // camera, and the two subjects do not want the same one: the
        // box alone was framed at elev 33 / azim -125, which puts the
        // cut faces 2.5 degrees off EDGE-ON (foreshortening 0.044) and
        // makes a machinist's section into a pair of slivers. The
        // cutaway's own choice — "the section normal ~48 degrees off
        // the view direction" (#91 revision note 6) — is the demanding
        // constraint, so it wins: at elev 20 / azim 55 the section
        // normal is 44.8 degrees off the view, foreshortening 0.705,
        // and the below half's cut faces the camera with the cavity,
        // vents and boss sections open to it. The whole box is a box
        // from any azimuth; the section is only a section from this
        // one.
        view: View {
            elev: 20.0,
            azim: 55.0,
            up: 'z',
        },
        bodies: core::iter::once(SceneBody::seamed(
            "projectbox",
            [0.40, 0.60, 0.72],
            acc.body.into_body(),
            acc.contacts,
        ))
        .chain(core::iter::once(SceneBody::plain(
            "spring",
            SPRING_COLOR,
            spring.into_body(),
        )))
        .chain(section_bodies)
        .collect(),
    }
}
