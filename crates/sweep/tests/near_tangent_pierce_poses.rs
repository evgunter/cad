//! **Near-tangent pierce poses build their closed form or refuse typed**:
//! a prism's corner on the near face of a cube of side 4, the face's
//! plane `d` rad off one of the corner's edges ([`NtPrism::near_normal`],
//! JOIN's census poses bit for bit), every op in both orders against the
//! clipping oracle. The poses are where the near-tangent census
//! (`examples/near_tangent_census_probe.rs`) measured each of three
//! classification sites refusing `ClassificationInvariant` on valid
//! input, at ε = 1e-9, 1e-6 and 1e-12: a static fixture, so each row's
//! own poses are in the list whichever row runs it.
//!
//! - **An unglued corner pair read one carrier at the corner and apart
//!   at the origin** (`boolean/recl.rs` `require_same`,
//!   `boolean/vtxfac.rs`' coplanar lump). The pair is read about the
//!   corner, over the region its coplanarity was read on, so it refuses
//!   as the near-coincidence it is.
//! - **A pierce germ re-read by sector membership** (`boolean/vtxfac.rs`
//!   `pierce_germ_dir`): the germ along a bound read On lies a definite
//!   angle outside the sector at its arm, so neither ray read within.
//!   The ray is the transition's own, and [`GERM_WITNESSES`] build.
//! - **A section edge read in band of a face it bounds**
//!   (`boolean/ops.rs` `describe_edges`): the cube's near face lies
//!   `d` rad off the prism's vertical edge, so at `|d| = ε` the edge's
//!   far vertex lies on the zero band's edge. A run whose section chord
//!   starts at that vertex certifies it as the chord's own origin, in
//!   the zero band, and builds; a run whose chord ends there certifies
//!   `carrier(t₁)`, one rounding off the vertex and just in band, and
//!   refuses typed as the input's ([`EDGE_ON_FACES`]), never a desync.
//!
//! [`NtPrism::near_normal`]: crate::common::differential::NtPrism::near_normal

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{BooleanDecision, BooleanError, BooleanResult, ValidationError};

use crate::common::differential::{nt_cube, nt_prisms};

/// `(prism, edge, sixteenths, tilt)`: the census's `"{prism} nt e{edge}
/// a{sixteenths} d{tilt}"`, each beside the site it refused at and the
/// ε row: `germ` the pierce germ, `coplanar` the coplanar lump, `recl`
/// the ON sector pair, `describe` the minted edges' description.
const POSES: [(&str, usize, u32, f64); 32] = [
    ("Lbot", 0, 12, -1e-9),    // germ, ε = 1e-9
    ("Lmirror", 0, 12, -1e-9), // germ, ε = 1e-9
    ("Ltop", 0, 12, -1e-9),    // germ, ε = 1e-9
    ("convex", 0, 3, 1e-5),    // coplanar, ε = 1e-9
    ("w345", 1, 4, 1e-5),      // coplanar, ε = 1e-9
    ("w60", 1, 4, 1e-5),       // coplanar, ε = 1e-9
    ("convex", 0, 11, -1e-6),  // recl, ε = 1e-9
    ("w345", 1, 11, -1e-5),    // recl, ε = 1e-9
    ("w60", 1, 11, -1e-5),     // recl, ε = 1e-9
    ("Lbot", 0, 12, -1e-6),    // germ, ε = 1e-6
    ("Lmirror", 1, 0, -1e-6),  // germ, ε = 1e-6
    ("Ltop", 0, 12, -1e-6),    // germ, ε = 1e-6
    ("convex", 0, 3, 1e-4),    // coplanar, ε = 1e-6
    ("w60", 1, 4, 1e-4),       // coplanar, ε = 1e-6
    ("convex", 0, 11, -1e-4),  // recl, ε = 1e-6
    ("w345", 1, 11, 1e-4),     // recl, ε = 1e-6
    ("w60", 1, 11, -1e-4),     // recl, ε = 1e-6
    ("asym", 2, 1, 1e-7),      // germ, ε = 1e-12
    ("convex", 0, 10, -1e-7),  // germ, ε = 1e-12
    ("notch307", 2, 1, 1e-7),  // germ, ε = 1e-12
    ("convex", 0, 3, 1e-7),    // coplanar, ε = 1e-12
    ("w345", 1, 4, 1e-7),      // coplanar, ε = 1e-12
    ("w60", 1, 4, 1e-7),       // coplanar, ε = 1e-12
    ("convex", 0, 11, -1e-7),  // recl, ε = 1e-12
    ("w345", 1, 11, 1e-7),     // recl, ε = 1e-12
    ("w60", 1, 11, -1e-7),     // recl, ε = 1e-12
    ("convex", 0, 4, 1e-5),    // recl, ε = 1e-9
    ("Ltop", 0, 12, 1e-9),     // germ, ε = 1e-9
    ("Ltop", 2, 13, -1e-9),    // describe, ε = 1e-9
    ("Lbot", 2, 9, 1e-9),      // describe, ε = 1e-9
    ("Lmirror", 2, 9, -1e-9),  // describe, ε = 1e-9
    ("asym", 2, 13, -1e-9),    // describe, ε = 1e-9
];

/// `(prism, edge, sixteenths, tilt, run)` at ε = 1e-12: the runs that
/// build their closed form and ship a definite tier-3′ finding, each
/// pinned at what it ships (the findings' `Debug`, in order). They are
/// FUSE's class (`work/fuse/a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms.md`),
/// which the door does not see until it runs the census over its result
/// (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`):
/// either landing turns these red, to be re-pinned.
const SHIPS_A_FINDING: [Shipped; 4] = [
    (
        "w345",
        1,
        13,
        1e-8,
        "prism ∩ cube",
        &[
            "StaleContactDeclaration { declaration: VertexOnFace { vertex: VertexKey(2v1), face: FaceKey(19v7) } }",
            "StaleContactDeclaration { declaration: VertexOnFace { vertex: VertexKey(9v1), face: FaceKey(19v7) } }",
        ],
    ),
    (
        "w345",
        1,
        13,
        1e-9,
        "prism ∩ cube",
        &[
            "StaleContactDeclaration { declaration: VertexOnFace { vertex: VertexKey(2v1), face: FaceKey(19v9) } }",
            "StaleContactDeclaration { declaration: VertexOnFace { vertex: VertexKey(9v1), face: FaceKey(19v9) } }",
        ],
    ),
    (
        "w345",
        1,
        14,
        1e-8,
        "cube ∩ prism",
        &[
            "StaleContactDeclaration { declaration: VertexOnFace { vertex: VertexKey(23v3), face: FaceKey(6v1) } }",
            "StaleContactDeclaration { declaration: VertexOnFace { vertex: VertexKey(17v3), face: FaceKey(6v1) } }",
        ],
    ),
    (
        "w345",
        1,
        14,
        1e-8,
        "cube ∖ prism",
        &[
            r#"UndeclaredContact { contact: VertexOnFace { vertex: VertexKey(16v3), face: FaceKey(8v1) }, witness: "(2.0, 0.0, 1.0)" }"#,
            r#"UndeclaredContact { contact: VertexOnFace { vertex: VertexKey(22v3), face: FaceKey(8v1) }, witness: "(2.0, 0.0, 0.0)" }"#,
            r#"UndeclaredContact { contact: EdgeFaceOverlap { edge: EdgeKey(20v7), face: FaceKey(8v1) }, witness: "(2.0, 0.0, 0.5)" }"#,
        ],
    ),
];

/// The census's poses where only the pierce germ refused on main and
/// every run builds sound at all three ε rows: a germ signed against its
/// transition refuses them or ships them wrong.
const GERM_WITNESSES: [(&str, usize, u32, f64); 5] = [
    ("Lbot", 0, 12, -1e-9),
    ("Lmirror", 0, 12, -1e-9),
    ("Ltop", 0, 12, -1e-9),
    ("Ltop", 0, 12, 1e-9),
    ("Lbot", 0, 12, -1e-6),
];

/// The census's poses whose section edge across the prism's bottom or
/// top face meets the far vertex of its vertical edge, `|d|` m off the
/// cube's near face: on the zero band's edge at ε = 1e-9. Which runs
/// refuse depends on the chord's orientation: where it runs from the
/// pierce to the vertex, the end sample `carrier(t₁)` lands one
/// rounding off the vertex, ~5e-17 m past the band's edge, and refuses
/// in band; where it runs from the vertex, the vertex is its origin,
/// read in the zero band, and the run builds
/// (`work/cleave/a-chords-end-sample-reads-carrier-t1-not-its-vertex.md`).
/// `(prism, edge, sixteenths, tilt, runs refusing in band at ε = 1e-9)`.
const EDGE_ON_FACES: [(&str, usize, u32, f64, [&str; 3]); 4] = [
    (
        "Ltop",
        2,
        13,
        -1e-9,
        ["prism ∪ cube", "prism ∖ cube", "cube ∩ prism"],
    ),
    (
        "Lbot",
        2,
        9,
        1e-9,
        ["prism ∩ cube", "cube ∪ prism", "cube ∖ prism"],
    ),
    (
        "Lmirror",
        2,
        9,
        -1e-9,
        ["prism ∪ cube", "prism ∖ cube", "cube ∩ prism"],
    ),
    (
        "asym",
        2,
        13,
        -1e-9,
        ["prism ∪ cube", "prism ∖ cube", "cube ∩ prism"],
    ),
];

/// A pose, one of its runs, and the findings that run ships.
type Shipped = (
    &'static str,
    usize,
    u32,
    f64,
    &'static str,
    &'static [&'static str],
);

/// One run of a pose: its name, its result, and the closed-form volume.
type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>, f64);

/// A pose's six runs, every op in both orders, against the clipping
/// oracle's closed forms: what is wrong with them, and how many built.
/// A refusal is wrong where it is a kernel defect, as the executed-offer
/// harness reads one (`topo::test_support::offer_key`). A built body is
/// wrong off its closed form by more than ε over its area, or failing
/// tier 3′ on more than an in-band census escalation, unless it ships
/// what [`SHIPS_A_FINDING`] pins at ε = 1e-12. An empty result is wrong
/// where the closed form is more than a sliver ε thick over the
/// operands' faces.
fn check_pose((name, edge, a, d): (&str, usize, u32, f64), tol: Tol) -> (Vec<String>, usize) {
    let prisms = nt_prisms();
    let pr = prisms
        .iter()
        .find(|p| p.name == name)
        .unwrap_or_else(|| panic!("no near-tangent prism {name}"));
    let body = pr.body(tol);
    let m = pr.near_normal(edge, a, d);
    let cube = nt_cube(pr.v, m, tol);
    let props = topo::mass_properties(&body, tol).unwrap();
    let (va, vb, common) = (props.volume, 64.0, pr.overlap(m));
    // The cube's six faces of side 4.
    let faces = props.surface_area + 96.0;
    let at_1e12 = (tol.eps() - 1e-12).abs() < 1e-24;
    let runs: [Run; 6] = [
        (
            "prism ∪ cube",
            topo::union(&body, &cube, tol),
            va + vb - common,
        ),
        (
            "cube ∪ prism",
            topo::union(&cube, &body, tol),
            va + vb - common,
        ),
        ("prism ∩ cube", topo::intersect(&body, &cube, tol), common),
        ("cube ∩ prism", topo::intersect(&cube, &body, tol), common),
        (
            "prism ∖ cube",
            topo::subtract(&body, &cube, tol),
            va - common,
        ),
        (
            "cube ∖ prism",
            topo::subtract(&cube, &body, tol),
            vb - common,
        ),
    ];
    let (mut wrong, mut built) = (Vec::new(), 0);
    for (op, r, want) in runs {
        let tag = format!("{name} nt e{edge} a{a} d{d:e}, {op}");
        let pinned = SHIPS_A_FINDING
            .iter()
            .find(|&&(n, e, aa, dd, o, _)| (n, e, aa, dd, o) == (name, edge, a, d, op))
            .filter(|_| at_1e12)
            .map(|&(.., findings)| findings);
        let bb = match &r {
            Err(e) => {
                if topo::test_support::offer_key(e).1 {
                    wrong.push(format!("{tag}: a kernel defect: {e:?}"));
                }
                continue;
            }
            Ok(res) => res.body(),
        };
        let Some(bb) = bb else {
            if want > tol.eps() * faces {
                wrong.push(format!("{tag}: empty, the closed form {want:e}"));
            }
            continue;
        };
        built += 1;
        let props = topo::mass_properties(&bb.body, tol).unwrap();
        let slack = tol.eps() * props.surface_area;
        if (props.volume - want).abs() > slack {
            wrong.push(format!(
                "{tag}: volume {} against the closed form {want}, past ε·area {slack:e}",
                props.volume
            ));
        }
        let findings: Vec<String> = match topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
        {
            Ok(()) => Vec::new(),
            Err(errors) => errors
                .iter()
                .filter(|e| !matches!(e, ValidationError::CensusEscalated { .. }))
                .map(|e| format!("{e:?}"))
                .collect(),
        };
        match pinned {
            Some(want) if findings != want => {
                wrong.push(format!("{tag}: the pinned findings moved: {findings:#?}"));
            }
            None if !findings.is_empty() => {
                wrong.push(format!("{tag}: fails tier 3′ definitely: {findings:#?}"));
            }
            _ => {}
        }
    }
    (wrong, built)
}

#[test]
fn near_tangent_pierce_poses_build_their_closed_form_or_refuse_typed() {
    let tol = Tol::witness();
    let mut poses = POSES.to_vec();
    for &(n, e, a, d, ..) in &SHIPS_A_FINDING {
        if !poses.contains(&(n, e, a, d)) {
            poses.push((n, e, a, d));
        }
    }
    let wrong: Vec<String> = poses
        .into_iter()
        .flat_map(|pose| check_pose(pose, tol).0)
        .collect();
    assert!(
        wrong.is_empty(),
        "{} of the poses' runs are wrong:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

#[test]
fn the_pierce_germs_witnesses_build_every_run() {
    let tol = Tol::witness();
    for pose in GERM_WITNESSES {
        let (wrong, built) = check_pose(pose, tol);
        assert!(
            wrong.is_empty() && built == 6,
            "{pose:?}: {built} of 6 built:\n{}",
            wrong.join("\n")
        );
    }
}

/// **A section edge read in band of its face refuses as the input's**:
/// at ε = 1e-9 each [`EDGE_ON_FACES`] pose's named runs refuse
/// `Escalated` on [`BooleanDecision::EdgeOnFaces`] with the lever, and
/// its other runs do not refuse it; at ε = 1e-6 and 1e-12, where the
/// census measured none, the vertex reads clearly on or off the face
/// and no run refuses it.
#[test]
fn a_section_edge_read_in_band_of_its_face_refuses_typed() {
    let tol = Tol::witness();
    let at_1e9 = (tol.eps() - 1e-9).abs() < 1e-18;
    let prisms = nt_prisms();
    let mut wrong = Vec::new();
    for (name, edge, a, d, want) in EDGE_ON_FACES {
        let pr = prisms.iter().find(|p| p.name == name).unwrap();
        let body = pr.body(tol);
        let cube = nt_cube(pr.v, pr.near_normal(edge, a, d), tol);
        let runs = [
            ("prism ∪ cube", topo::union(&body, &cube, tol)),
            ("cube ∪ prism", topo::union(&cube, &body, tol)),
            ("prism ∩ cube", topo::intersect(&body, &cube, tol)),
            ("cube ∩ prism", topo::intersect(&cube, &body, tol)),
            ("prism ∖ cube", topo::subtract(&body, &cube, tol)),
            ("cube ∖ prism", topo::subtract(&cube, &body, tol)),
        ];
        for (op, r) in runs {
            let in_band = match &r {
                Err(
                    e @ BooleanError::Escalated {
                        decision: BooleanDecision::EdgeOnFaces,
                        ..
                    },
                ) => {
                    let text = e.to_string();
                    if !text.ends_with(
                        "Recourse: move the geometry so each edge and vertex that meets a face lies \
                         clearly on that face or clearly off it",
                    ) {
                        wrong.push(format!("{name} {op}: ends otherwise: {text}"));
                    }
                    true
                }
                _ => false,
            };
            if in_band != (at_1e9 && want.contains(&op)) {
                wrong.push(format!(
                    "{name} nt e{edge} a{a} d{d:e}, {op}: in band {in_band}: {:?}",
                    r.err()
                ));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
