//! The shadow-silhouette solid (#91 C2): one solid whose orthographic
//! shadows are two, then three, different profiles — the tour's FIRST
//! `intersect`, and (for the 3-way) intersect-of-intersect, the
//! boolean-of-boolean lane.
//!
//! All three operands are extrudes of pure POLYGON letterforms drawn
//! in one block, x ∈ [0, 2], y ∈ [0, 3], z ∈ [0, 3]: an "H" on the xy
//! plane extruded +z, a "T" on the yz plane extruded +x, a "C" on the
//! zx plane extruded +y. Every face is a plane and every edge a line,
//! so the operand gate (`reduce::gate_operand_pairs`) passes and no
//! curved geometry goes near a boolean (round letterforms are the M5
//! upgrade).
//!
//! The letters meet where a person drawing them would make them meet:
//! the T's stem spans exactly the H's bar band (y ∈ [1.25, 1.75]), and
//! every letter's box is the block's, so their outer walls share
//! carriers. Coincidence is intent, so those contacts are DECLARED
//! ([`crate::booleans::try_intersect_declared`]); the same operands
//! undeclared refuse typed at the coincidence door, and the scene
//! narrates that refusal before it builds.
//!
//! The 3-way is built `C ∩ (H × T)`. The other operand order refuses
//! `JoinDesync` on the same declared contacts — a live wall probe in
//! [`stops`], filed as
//! `work/join/declared-flush-intersect-refuses-in-one-operand-order.md`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::profile::SketchPlane;
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanBody, BooleanError};

use crate::booleans::{check, expect_seamed, try_intersect, try_intersect_declared};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::{p3, polygon, validated};
use pncad::geom_core::{OrthoFrame, Tol};

/// One letterform prism: `outline` on `frame`, extruded `depth` along
/// the frame's normal.
fn letter<S: Scalar>(
    frame: OrthoFrame<S>,
    outline: &[(f64, f64)],
    depth: f64,
    tol: Tol,
) -> Body<S> {
    let outline = polygon(outline, tol).expect("letterform outline");
    extrude(
        &validated(SketchPlane::from_frame(frame), vec![outline], tol).expect("letterform profile"),
        Extrusion::Distance(S::from_f64(depth)),
        tol,
    )
    .expect("extrude letterform")
    .body
}

/// "H", (x, y) on the xy plane at z = 0: legs 1/2 wide, bar
/// y ∈ [1.25, 1.75]; extruded 3 along +z.
const H: [(f64, f64); 12] = [
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

/// "T", (y, z) on the yz plane at x = 0: bar z ∈ [2.5, 3] across the
/// block, stem on the H's bar band; extruded 2 along +x.
const T: [(f64, f64); 8] = [
    (1.25, 0.0),
    (1.75, 0.0),
    (1.75, 2.5),
    (3.0, 2.5),
    (3.0, 3.0),
    (0.0, 3.0),
    (0.0, 2.5),
    (1.25, 2.5),
];

/// "C", (z, x) on the zx plane at y = 0, counterclockwise: spine
/// x ∈ [0, 0.5], arms z ∈ [0, 0.5] and [2.5, 3] opening to +x;
/// extruded 3 along +y.
const C: [(f64, f64); 8] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 2.0),
    (2.5, 2.0),
    (2.5, 0.5),
    (0.5, 0.5),
    (0.5, 2.0),
    (0.0, 2.0),
];

fn h_prism<S: Scalar>(tol: Tol) -> Body<S> {
    letter(OrthoFrame::axes_xy(p3(0.0, 0.0, 0.0)), &H, 3.0, tol)
}

fn t_prism<S: Scalar>(tol: Tol) -> Body<S> {
    letter(OrthoFrame::axes_yz(p3(0.0, 0.0, 0.0)), &T, 2.0, tol)
}

fn c_prism<S: Scalar>(tol: Tol) -> Body<S> {
    letter(OrthoFrame::axes_zx(p3(0.0, 0.0, 0.0)), &C, 3.0, tol)
}

/// H × T volume, ∫ len_x(H at y) · len_z(T at y) dy (each prism spans
/// the other's extrusion): len_x(H) is 1 on the leg bands and 2 on the
/// bar band; len_z(T) is 1/2 off the stem and 3 on it, the same band —
/// 2.5·1·0.5 + 0.5·2·3 = 17/4.
const V_2WAY: f64 = 4.25;

/// H × T × C volume, by z-slab; a slab's (x, y) cross-section is
/// H ∩ (T's y-band there) ∩ (C's x-band there):
///
/// | z | T's y-band | C's x-band | area | slab |
/// |---|---|---|---|---|
/// | [0, 1/2] | stem | [0, 2] | 2 · 1/2 = 1 | 1/2 |
/// | [1/2, 5/2] | stem | [0, 1/2] | 1/2 · 1/2 = 1/4 | 1/2 |
/// | [5/2, 3] | [0, 3] | [0, 2] | all of H, 7/2 | 7/4 |
///
/// — 11/4.
const V_3WAY: f64 = 2.75;

/// Builds the 2-way and 3-way results, narrating the undeclared
/// refusal first; also hands back the C prism the 3-way consumed.
pub(crate) fn build<S: Scalar>(tol: Tol) -> (BooleanBody<S>, BooleanBody<S>, Body<S>) {
    let (h, t, c) = (h_prism::<S>(tol), t_prism::<S>(tol), c_prism::<S>(tol));
    match try_intersect(&h, &t, tol) {
        Err(e @ BooleanError::UndeclaredCoincidence { .. }) => println!(
            "   H x T UNDECLARED refuses typed at the coincidence door ({:?}): \
             value-equality never glues; the scene declares its contacts",
            e.kind()
        ),
        other => panic!(
            "the undeclared flush H x T must refuse UndeclaredCoincidence, got {:?}",
            other.map(|_| "a result")
        ),
    }
    let two = expect_seamed(
        "declared H x T intersect",
        check(try_intersect_declared(&h, &t, tol), V_2WAY, tol),
        V_2WAY,
    );
    let three = expect_seamed(
        "declared C x (H x T) intersect",
        check(try_intersect_declared(&c, &two.body, tol), V_3WAY, tol),
        V_3WAY,
    );
    (two, three, c)
}

/// Area of `body`'s orthographic shadow down world axis `w` (0 = x,
/// 1 = y, 2 = z), exact for these letters: every face lies on a plane
/// of the block grid below, so each grid cell is wholly in or out of
/// the shadow, and a cell is in iff some cell centre along the ray is
/// strictly inside the body.
fn shadow_area(body: &Body<f64>, w: usize, tol: Tol) -> f64 {
    const GRID: [f64; 8] = [0.0, 0.5, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0];
    const EXTENT: [f64; 3] = [2.0, 3.0, 3.0];
    let cells = |axis: usize| -> Vec<(f64, f64)> {
        GRID.windows(2)
            .filter(|g| g[1] <= EXTENT[axis])
            .map(|g| ((g[0] + g[1]) / 2.0, g[1] - g[0]))
            .collect()
    };
    let band = pncad::geom_core::Band::linear(tol).expect("the run's tolerance forms a band");
    let (u, v) = ((w + 1) % 3, (w + 2) % 3);
    let mut area = 0.0;
    for &(cu, du) in &cells(u) {
        for &(cv, dv) in &cells(v) {
            let lit = cells(w).iter().any(|&(cw, _)| {
                let mut q = [0.0; 3];
                q[u] = cu;
                q[v] = cv;
                q[w] = cw;
                pncad::topo::point_in_solid(body, p3(q[0], q[1], q[2]), band, tol)
                    .expect("a cell centre lies off every face")
                    == pncad::topo::SolidContainment::In
            });
            if lit {
                area += du * dv;
            }
        }
    }
    area
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let (two, three, c) = build::<f64>(tol);
    // The 3-way lies inside each letter's prism, so its shadow down that
    // prism's axis lies inside the letter; equal area makes it the WHOLE
    // letter. Letter areas: H 2·(1/2·3) + 1·1/2, T 1/2·5/2 + 3·1/2,
    // C 1/2·3 + 2·(3/2·1/2).
    for (axis, letter, area) in [(2, "H", 3.5), (0, "T", 2.75), (1, "C", 3.0)] {
        let shadow = shadow_area(&three.body, axis, tol);
        assert_eq!(shadow, area, "the 3-way's shadow is the whole {letter}");
    }
    // The other order, (H x T) x C, builds the same 3-way: intersection
    // is commutative, and the join no longer refuses it
    // (`work/join/declared-flush-intersect-refuses-in-one-operand-order.md`).
    expect_seamed(
        "declared (H x T) x C intersect",
        check(try_intersect_declared(&two.body, &c, tol), V_3WAY, tol),
        V_3WAY,
    );
    // The shadow PROOF renders (standalone, not montage panels): the
    // 3-way solid viewed straight down each axis — orthographic, so
    // each frame IS the shadow: an H (z), a T (x), a C (y).
    let shadow = |name: &'static str, caption: &str, elev: f64, azim: f64| Stop {
        name,
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
            name,
            [0.80, 0.44, 0.30],
            three.body.clone(),
            three.contacts.clone(),
        )],
    };
    let shadows = vec![
        shadow("silhouette3_shadow_z", "z-shadow: H", 90.0, -90.0),
        shadow("silhouette3_shadow_x", "x-shadow: T", 0.0, 0.0),
        shadow("silhouette3_shadow_y", "y-shadow: C", 0.0, -90.0),
    ];
    vec![
        Stop {
            name: "silhouette",
            caption: "silhouette (H x T)".to_string(),
            // Montage carries only the 3-way (#91 revision note 5);
            // this stop stays in the tour + standalone render.
            montage: false,
            story: "shadow-silhouette solid: its z-shadow is an H, its x-shadow is a T \
                    — the tour's first `intersect`",
            ops: "extrude H (xy sketch, +z) x extrude T (yz sketch, +x), flush contacts \
                  declared -> 1 intersect node",
            delta: 1e-2,
            note: Some(format!(
                "volume {V_2WAY} = 17/4 (gated 1e-9); the T's stem spans exactly the H's \
                 bar band and both letters fill one block, so the contacts are declared — \
                 undeclared, the same operands refuse UndeclaredCoincidence"
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
            story: "three letter shadows: a blocky C prism along +y intersected with the \
                    H x T solid — intersect-of-intersect, boolean-of-boolean",
            ops: "extrude C (zx sketch, +y) x silhouette result, flush contacts declared \
                  -> 1 more intersect node",
            delta: 1e-2,
            note: Some(format!(
                "volume {V_3WAY} = 11/4 (gated 1e-9); built C x (H x T) because \
                 (H x T) x C refuses JoinDesync on the same declarations (wall 1)"
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
