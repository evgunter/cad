//! **The tube door made visible** — `pncad::sweep::tube_along_arc`, the
//! world-coordinate torus door (M6-3 Leg F, the Evan-ratified rider
//! on the #175 thread).
//!
//! # Why this is a scene and not "another torus"
//!
//! The sheave's groove and the fairy lantern's three stem tubes already
//! put ring-torus walls on the montage — but both arrive by
//! `revolve`, which reconstructs the tube radius from the profile's
//! bulge arcs (the lily's stored `minor_radius` sits 3.9e-16 below
//! the authored 0.060; the review donut drifted 56 ulps). This door
//! takes the INTENT parameters directly — spine centre, axis,
//! reference direction, major radius, an arc window, minor radius —
//! and stores them, with no profile → bulge → radius arithmetic
//! anywhere on the path.
//!
//! The scene is deliberately a WINDOWED tube rather than the full
//! donut, so that every intent parameter is on screen: the major
//! radius is the ring's own radius, the minor radius is the pipe's,
//! and the window `[t0, t1]` is the gap plus the two wedge caps that
//! close it. A full ring would show two of the three.
//!
//! Construction is `sweep/tests/m6_tube.rs`'s wedge, constant for
//! constant (that suite pins the census — two half-tube walls plus
//! two planar caps — the Pappus volume, and the bit-exact
//! `minor_radius` retirement with `==`, not a tolerance).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom::Surface;
use pncad::geom_core::{Point3, Vec3};
use pncad::sweep::{TubeWindow, tube_along_arc};

use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// Major radius (`m6_tube.rs::R`).
const R: f64 = 2.0;
/// Minor radius (`m6_tube.rs::MINOR`).
const MINOR: f64 = 0.5;
/// The window's start angle, radians about the spine axis from
/// `u_ref` (`m6_tube.rs::tube_window_and_refusal_doors`).
const T0: f64 = 0.25;
/// The window's end angle.
const T1: f64 = 1.75;

/// The tube-door stop.
pub fn stops(tol: Tol) -> Vec<Stop> {
    let tube = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Arc { t0: T0, t1: T1 },
        MINOR,
        tol,
    )
    .expect("the wedge builds")
    .body;

    // The door's headline claim, EXECUTED on the scene body rather
    // than quoted from the test suite: the number the caller gave is
    // the number the body stores, bit for bit. (`==`, not a
    // tolerance — that is the whole point of the retirement.)
    let mut stored: Vec<f64> = Vec::new();
    for (_, face) in tube.faces() {
        if let Some(Surface::Torus { minor_radius, .. }) = tube.get_surface(face.surface) {
            stored.push(*minor_radius);
        }
    }
    assert_eq!(stored.len(), 2, "a windowed tube has two half-tube walls");
    assert!(
        stored.iter().all(|r| r.to_bits() == MINOR.to_bits()),
        "the tube door STORES the authored minor radius BIT-exactly \
         (got {stored:?}, authored {MINOR}) — if this ever drifts, the \
         56-ulp reconstruction the door retired has come back"
    );

    // Pappus on the wedge: the swept area is the full disc pi*r^2 and
    // the centroid travels R*(t1 - t0).
    let span = T1 - T0;
    let pappus = core::f64::consts::PI * MINOR * MINOR * R * span;

    vec![Stop {
        name: "tube_along_arc",
        caption: "tube_along_arc (intent parameters, stored)".to_string(),
        // Standalone since the montage-v2 curation (Evan, #218 follow-up):
        // the cell's content — bit-exact STORED intent parameters — is
        // interesting for how it works, not visually; without that context
        // it reads as one more partial revolve. The scene, its assertions
        // and its standalone render stay fully alive.
        montage: false,
        story: "a ring-torus tube built from its INTENT parameters — spine centre at \
                the origin, spine axis +y, reference direction +x, major radius 2, \
                minor radius 0.5, traversed over the window [0.25, 1.75] rad. Every \
                one of those is visible: the ring's radius, the pipe's radius, and the \
                window as the gap the two planar wedge caps close",
        ops: "sweep::tube_along_arc(origin, +y, +x, R = 2, Arc{t0 = 0.25, t1 = 1.75}, \
              r = 0.5)",
        // The lily's finding 13 in miniature: delta is a CHORD budget
        // spent per RING radius, not per feature size, so a 2 m spine
        // burns it fast. Measured on this body: 2e-3 -> 172k triangles
        // at 0.004% mesh-volume error, 6e-3 -> 58k at 0.011%,
        // 1e-2 -> 23k at 0.028%. 1e-2 is the montage's own working
        // budget (cutaway, projectbox) and the facets it leaves are
        // the point of the kernel lane, not a defect.
        delta: 1e-2,
        note: Some(format!(
            "sweep/tests/m6_tube.rs's wedge, constant for constant. NO semantic fork: \
             the body is assembled by the revolve's own partial-wedge machinery fed a \
             directly constructed swept traversal, so classification, sense derivation, \
             seam placement, the ring-torus convention (R > r > 0) and the whole-body \
             pcurve mint are the revolve's code, not copies. What differs is what gets \
             STORED — both half-tube walls carry minor_radius == {MINOR} BIT-EXACTLY \
             (asserted here with ==, on this scene body), retiring the profile -> bulge \
             -> radius reconstruction drift the lily measured at 3.9e-16 and the review \
             donut at 56 ulps. Census: 2 half-tube walls + 2 planar caps; volume by \
             Pappus pi*r^2*R*(t1 - t0) = {pappus:.9} m^3"
        )),
        // The ring lies in the world XZ plane (its axis is +y), so a
        // camera near the y-axis sees the window face-on and a camera
        // in the XZ plane sees it edge-on and unreadable. azim 72
        // puts the eye 18 degrees off +y — the window's angular
        // extent still reads as an angle — and elev 28 tilts enough
        // that the tube's roundness and the near cap's disc are both
        // shaded rather than flat.
        view: View {
            elev: 28.0,
            azim: 72.0,
            up: 'z',
        },
        bodies: vec![SceneBody::plain("tube_along_arc", [0.80, 0.68, 0.32], tube)],
    }]
}
