//! **Near-tangent pierce poses build their closed form or refuse typed**:
//! a prism's corner on the near face of a cube of side 4, the face's
//! plane `d` rad off one of the corner's edges ([`NtPrism::near_normal`],
//! JOIN's census poses bit for bit), every op in both orders against the
//! clipping oracle. The poses are where the near-tangent census
//! (`examples/near_tangent_census_probe.rs`), on its sixteenths and off
//! them, measured a classification site refusing
//! `ClassificationInvariant` on valid input, at ε = 1e-9, 1e-6 and
//! 1e-12: a static fixture, so each row's own poses are in the list
//! whichever row runs it.
//!
//! - **A crossing along an edge folded onto a flanker the search never
//!   paired** (`boolean/recl.rs` `resolve_edge_sector`): the wedge's face
//!   holds the cube's edge, and the cube face beside it lies `d` rad off
//!   that face. The fold rule puts the crossing on that nearly coplanar
//!   flanker, whose pair with the wedge's face `pair_search` reads off the
//!   two planes' common line, which swings off the edge by the edge's
//!   in-band offset over `d`. The record is that pair as its codes read it.
//! - **A crossing recorded on the reference's other twin**
//!   (`resolve_bisector_graze`, `resolve_edge_sector`): the ray lies
//!   along a bisector of each solid, and the record names the twin past
//!   the bisector from the start-holder.
//! - **A reference read after an earlier germ rewrote it**
//!   (`find_ref_sector`): one record carries both solids' edges On, on
//!   rays a definite angle apart, and the first event's germ rewrites
//!   both On codes. The reference is read off the codes as first read.
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
//!   far vertex lies on the zero band's edge. Certification reads the
//!   section chord's carrier, its ends included: a run whose chord starts
//!   at that vertex reads it as the chord's own origin, in the zero band,
//!   and builds; a run whose chord ends there reads `carrier(t₁)`, one
//!   rounding off the vertex and just in band, and refuses typed as the
//!   input's ([`EDGE_ON_FACES`]), never a desync.
//!
//! [`NtPrism::near_normal`]: crate::common::differential::NtPrism::near_normal

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::{BooleanDecision, BooleanError, BooleanResult, ValidationError};

use crate::common::differential::{nt_cube, nt_prisms};

/// `(prism, edge, sixteenths, tilt)`: the census's `"{prism} nt e{edge}
/// a{sixteenths} d{tilt}"`, each beside the site it refused at and the
/// ε row: `germ` the pierce germ, `coplanar` the coplanar lump, `recl`
/// the ON sector pair, `describe` the minted edges' description, `edge`
/// the edge-sector crossing, `twin` the reference's other twin,
/// `reference` the reference read after a germ, `shipped` contact
/// records the result's geometry no longer confirmed.
const POSES: [(&str, usize, f64, f64); 55] = [
    ("Lbot", 0, 12.0, -1e-9),    // germ, ε = 1e-9
    ("Lmirror", 0, 12.0, -1e-9), // germ, ε = 1e-9
    ("Ltop", 0, 12.0, -1e-9),    // germ, ε = 1e-9
    ("convex", 0, 3.0, 1e-5),    // coplanar, ε = 1e-9
    ("w345", 1, 4.0, 1e-5),      // coplanar, ε = 1e-9
    ("w60", 1, 4.0, 1e-5),       // coplanar, ε = 1e-9
    ("convex", 0, 11.0, -1e-6),  // recl, ε = 1e-9
    ("w345", 1, 11.0, -1e-5),    // recl, ε = 1e-9
    ("w60", 1, 11.0, -1e-5),     // recl, ε = 1e-9
    ("Lbot", 0, 12.0, -1e-6),    // germ, ε = 1e-6
    ("Lmirror", 1, 0.0, -1e-6),  // germ, ε = 1e-6
    ("Ltop", 0, 12.0, -1e-6),    // germ, ε = 1e-6
    ("convex", 0, 3.0, 1e-4),    // coplanar, ε = 1e-6
    ("w60", 1, 4.0, 1e-4),       // coplanar, ε = 1e-6
    ("convex", 0, 11.0, -1e-4),  // recl, ε = 1e-6
    ("w345", 1, 11.0, 1e-4),     // recl, ε = 1e-6
    ("w60", 1, 11.0, -1e-4),     // recl, ε = 1e-6
    ("asym", 2, 1.0, 1e-7),      // germ, ε = 1e-12
    ("convex", 0, 10.0, -1e-7),  // germ, ε = 1e-12
    ("notch307", 2, 1.0, 1e-7),  // germ, ε = 1e-12
    ("convex", 0, 3.0, 1e-7),    // coplanar, ε = 1e-12
    ("w345", 1, 4.0, 1e-7),      // coplanar, ε = 1e-12
    ("w60", 1, 4.0, 1e-7),       // coplanar, ε = 1e-12
    ("convex", 0, 11.0, -1e-7),  // recl, ε = 1e-12
    ("w345", 1, 11.0, 1e-7),     // recl, ε = 1e-12
    ("w60", 1, 11.0, -1e-7),     // recl, ε = 1e-12
    ("convex", 0, 4.0, 1e-5),    // recl, ε = 1e-9
    ("Ltop", 0, 12.0, 1e-9),     // germ, ε = 1e-9
    ("Ltop", 2, 13.0, -1e-9),    // describe, ε = 1e-9
    ("Lbot", 2, 9.0, 1e-9),      // describe, ε = 1e-9
    ("Lmirror", 2, 9.0, -1e-9),  // describe, ε = 1e-9
    ("asym", 2, 13.0, -1e-9),    // describe, ε = 1e-9
    ("w345", 1, 12.0, 1e-7),     // edge, ε = 1e-12
    ("w345", 1, 12.0, 1e-8),     // edge, ε = 1e-12
    ("w345", 1, 12.0, 1e-9),     // edge, ε = 1e-12
    ("w345", 1, 3.0, -1e-7),     // edge, ε = 1e-12
    ("w345", 1, 3.0, -1e-8),     // edge, ε = 1e-12
    ("w345", 1, 3.0, -1e-9),     // edge, ε = 1e-12
    ("w345", 1, 11.0, -1e-7),    // edge, ε = 1e-12
    ("w345", 1, 11.0, -1e-8),    // edge, ε = 1e-12
    ("w345", 1, 11.0, -1e-9),    // edge, ε = 1e-12
    ("w345", 1, 3.7, 1e-7),      // twin, ε = 1e-6
    ("w345", 1, 3.7, -1e-7),     // twin, ε = 1e-6
    ("w345", 1, 3.8, 1e-7),      // twin, ε = 1e-6
    ("w345", 1, 3.8, -1e-7),     // twin, ε = 1e-6
    ("w345", 1, 11.7, 1e-7),     // edge, ε = 1e-6
    ("w345", 1, 11.7, -1e-7),    // twin, ε = 1e-6
    ("w345", 1, 11.8, 1e-7),     // twin, ε = 1e-6
    ("w345", 1, 11.8, -1e-7),    // twin, ε = 1e-6
    ("vee300", 0, 11.75, -1e-7), // reference, ε = 1e-9 and 1e-12
    ("vee300", 0, 11.75, -2e-8), // reference, ε = 1e-12
    ("vee300", 0, 11.75, -2e-9), // reference, ε = 1e-12
    ("w345", 1, 13.0, 1e-8),     // shipped, ε = 1e-12
    ("w345", 1, 13.0, 1e-9),     // shipped, ε = 1e-12
    ("w345", 1, 14.0, 1e-8),     // shipped, ε = 1e-12
];

/// `(ε, prism, edge, sixteenths, tilt, run)`: the runs that build their
/// closed form at that ε row and ship a definite tier-3′ finding, each
/// pinned at what it ships ([`Pinned`], in order). They are
/// FUSE's class (`a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms` (a closed FUSE row; `docs/doc-ledger/fuse-leaves-the-tracker.md`)),
/// which the door does not see until it runs the census over its result
/// (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`):
/// either landing turns these red, to be re-pinned.
const SHIPS_A_FINDING: [Shipped; 1] = [
    // The far end of the prism's edge `e1`, 2 m out, lies within ε of the
    // cube's face, where the cube's edge crosses the prism's top edge.
    (
        1e-6,
        "w345",
        1,
        3.8,
        1e-7,
        "prism ∪ cube",
        &[(
            "EdgeEdgeCross { a: EdgeKey(16v1), b: EdgeKey(31v3), side: None }",
            [2.0, 1.0186570888e-5, 1.0],
        )],
    ),
];

/// The census's poses where only the pierce germ refused on main and
/// every run builds sound at all three ε rows: a germ signed against its
/// transition refuses them or ships them wrong.
const GERM_WITNESSES: [(&str, usize, f64, f64); 5] = [
    ("Lbot", 0, 12.0, -1e-9),
    ("Lmirror", 0, 12.0, -1e-9),
    ("Ltop", 0, 12.0, -1e-9),
    ("Ltop", 0, 12.0, 1e-9),
    ("Lbot", 0, 12.0, -1e-6),
];

/// `(ε, prism, edge, sixteenths, tilt, built)`: poses whose crossing is
/// recorded on a flanker or twin `pair_search` never paired, or on the
/// reference's other twin, with how many of the six runs build at that
/// ε row. A germ left off its record refuses them, and one recorded
/// with codes read off another pair ships them wrong.
const EDGE_WITNESSES: [(f64, &str, usize, f64, f64, usize); 4] = [
    (1e-12, "w345", 1, 12.0, 1e-7, 6),
    (1e-12, "w345", 1, 12.0, 1e-8, 6),
    // The rest refuse typed: `pc ∪` `RechartUnvouched` at both, and the
    // `cp` runs at `d = 1e-7` an escalated carrier pair.
    (1e-6, "w345", 1, 11.7, 1e-7, 2),
    (1e-6, "w345", 1, 11.7, -1e-7, 5),
];

/// The census's poses whose section edge across the prism's bottom or
/// top face meets the far vertex of its vertical edge, `|d|` m off the
/// cube's near face: on the zero band's edge at ε = 1e-9. Which runs
/// refuse depends on the chord's orientation, because certification
/// bounds the chord's carrier and not its vertices: where it runs from
/// the pierce to the vertex, the end sample `carrier(t₁)` lands one
/// rounding off the vertex, ~5e-17 m past the band's edge, and refuses
/// in band; where it runs from the vertex, the vertex is its origin,
/// read in the zero band, and the run builds.
/// `(prism, edge, sixteenths, tilt, runs refusing in band at ε = 1e-9)`.
const EDGE_ON_FACES: [(&str, usize, f64, f64, [&str; 3]); 4] = [
    (
        "Ltop",
        2,
        13.0,
        -1e-9,
        ["prism ∪ cube", "prism ∖ cube", "cube ∩ prism"],
    ),
    (
        "Lbot",
        2,
        9.0,
        1e-9,
        ["prism ∩ cube", "cube ∪ prism", "cube ∖ prism"],
    ),
    (
        "Lmirror",
        2,
        9.0,
        -1e-9,
        ["prism ∪ cube", "prism ∖ cube", "cube ∩ prism"],
    ),
    (
        "asym",
        2,
        13.0,
        -1e-9,
        ["prism ∪ cube", "prism ∖ cube", "cube ∩ prism"],
    ),
];

/// A pose, one of its runs, and the findings that run ships.
type Shipped = (
    f64,
    &'static str,
    usize,
    f64,
    f64,
    &'static str,
    &'static [Pinned],
);

/// A shipped `UnrecordedContact`: its contact's `Debug`, and the point
/// its witness names to within ε. Its margin is the Zero reading that
/// found the contact, so at most ε; the numbers past that are the
/// census's rounding charge, and pinned they would only report it.
type Pinned = (&'static str, [f64; 3]);

/// Why `finding` is not the pinned one, if it is not.
fn pinned_mismatch(finding: &ValidationError, (contact, at): Pinned, eps: f64) -> Option<String> {
    let ValidationError::UnrecordedContact {
        contact: got,
        witness,
        margin,
    } = finding
    else {
        return Some("not an UnrecordedContact".into());
    };
    if format!("{got:?}") != contact {
        return Some(format!("contact {got:?}, pinned {contact}"));
    }
    let point: Vec<f64> = witness
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(", ")
        .map(|c| c.parse().unwrap_or(f64::NAN))
        .collect();
    let off = (point.len() == 3).then(|| {
        (0..3)
            .map(|i| (point[i] - at[i]).powi(2))
            .sum::<f64>()
            .sqrt()
    });
    if !off.is_some_and(|d| d <= eps) {
        return Some(format!("witness {witness}, pinned {at:?} within ε"));
    }
    match margin.map(geom_core::MarginDiag::diagnostic_f64_for_error_text) {
        Some(geom_core::ErrorTextReading::Value(m)) if (0.0..=eps).contains(&m) => None,
        other => Some(format!("margin {other:?}, not a Zero reading in [0, ε]")),
    }
}

/// One run of a pose: its name, its result, and the closed-form volume.
type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>, f64);

/// A pose's six runs, every op in both orders, against the clipping
/// oracle's closed forms: what is wrong with them, and how many built.
/// A refusal is wrong where it is a kernel defect, as the executed-offer
/// harness reads one (`topo::test_support::offer_key`). A built body is
/// wrong off its closed form by more than ε over its area, or failing
/// tier 3′ on more than an in-band census escalation, unless it ships
/// what [`SHIPS_A_FINDING`] pins at its ε row. An empty result is wrong
/// where the closed form is more than a sliver ε thick over the
/// operands' faces.
fn check_pose((name, edge, a, d): (&str, usize, f64, f64), tol: Tol) -> (Vec<String>, usize) {
    let (wrong, built, _) = check_pose_refusals((name, edge, a, d), tol);
    (wrong, built)
}

/// [`check_pose`], with each run that refused: its op and the refusal's
/// `Debug`.
fn check_pose_refusals(
    (name, edge, a, d): (&str, usize, f64, f64),
    tol: Tol,
) -> (Vec<String>, usize, Vec<(&'static str, String)>) {
    let prisms = nt_prisms();
    let pr = prisms
        .iter()
        .find(|p| p.name == name)
        .unwrap_or_else(|| panic!("no near-tangent prism {name}"));
    let body = pr.body(tol);
    let m = pr.near_normal(edge, a, d);
    let cube = nt_cube(pr.v, m, tol);
    let props =
        topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol)).unwrap();
    let (va, vb, common) = (props.enclosure.volume, 64.0, pr.overlap(m));
    // The cube's six faces of side 4.
    let faces = props.enclosure.surface_area + 96.0;
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
    let (mut wrong, mut built, mut refused) = (Vec::new(), 0, Vec::new());
    for (op, r, want) in runs {
        let tag = format!("{name} nt e{edge} a{a} d{d:e}, {op}");
        let pinned = SHIPS_A_FINDING
            .iter()
            .find(|&&(eps, n, e, aa, dd, o, _)| {
                (n, e, aa, dd, o) == (name, edge, a, d, op) && (tol.eps() - eps).abs() <= eps * 1e-9
            })
            .map(|&(.., findings)| findings);
        let bb = match &r {
            Err(e) => {
                if topo::test_support::offer_key(e).1 {
                    wrong.push(format!("{tag}: a kernel defect: {e:?}"));
                }
                refused.push((op, format!("{e:?}")));
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
        let props =
            topo::mass_properties(&bb.body, tol, topo::test_support::pinned_resolution(tol))
                .unwrap();
        let slack = tol.eps() * props.enclosure.surface_area;
        if (props.enclosure.volume - want).abs() > slack {
            wrong.push(format!(
                "{tag}: volume {} against the closed form {want}, past ε·area {slack:e}",
                props.enclosure.volume
            ));
        }
        let errors = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol)
            .err()
            .unwrap_or_default();
        let unread = |e: &ValidationError| {
            matches!(
                e,
                ValidationError::FaceThroughEdgeWedgeUndecided {
                    why: topo::TouchUnread::InBand | topo::TouchUnread::PieceInBand,
                    ..
                } | ValidationError::BoundaryCrossesItselfUndecided {
                    why: topo::TouchUnread::InBand | topo::TouchUnread::PieceInBand,
                    ..
                }
            )
        };
        let escalated = |e: &ValidationError| matches!(e, ValidationError::CensusEscalated { .. });
        // A self-touch's side in band with no escalation beside it
        // ([`UNREAD`]).
        if errors.iter().any(unread) && !errors.iter().any(escalated) {
            wrong.push(format!("{UNREAD}{tag}"));
        }
        // In band: an escalation, or a self-touch whose side is
        // undecided.
        let findings: Vec<ValidationError> = errors
            .into_iter()
            .filter(|e| !escalated(e) && !unread(e))
            .collect();
        match pinned {
            Some(want) => {
                let moved: Vec<String> = if findings.len() == want.len() {
                    findings
                        .iter()
                        .zip(want)
                        .filter_map(|(f, &w)| pinned_mismatch(f, w, tol.eps()))
                        .collect()
                } else {
                    vec![format!(
                        "{} findings, pinned {}",
                        findings.len(),
                        want.len()
                    )]
                };
                if !moved.is_empty() {
                    wrong.push(format!(
                        "{tag}: the pinned findings moved ({}): {findings:#?}",
                        moved.join("; ")
                    ));
                }
            }
            None if !findings.is_empty() => {
                wrong.push(format!("{tag}: fails tier 3′ definitely: {findings:#?}"));
            }
            _ => {}
        }
    }
    (wrong, built, refused)
}

#[test]
fn near_tangent_pierce_poses_build_their_closed_form_or_refuse_typed() {
    let tol = Tol::witness();
    let mut poses = POSES.to_vec();
    for &(_, n, e, a, d, ..) in &SHIPS_A_FINDING {
        if !poses.contains(&(n, e, a, d)) {
            poses.push((n, e, a, d));
        }
    }
    let lines: Vec<String> = poses
        .into_iter()
        .flat_map(|pose| check_pose(pose, tol).0)
        .collect();
    let wrong: Vec<&str> = lines
        .iter()
        .map(String::as_str)
        .filter(|l| !l.starts_with(UNREAD))
        .collect();
    assert!(
        wrong.is_empty(),
        "{} of the poses' runs are wrong:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
    let got: Vec<&str> = lines
        .iter()
        .map(String::as_str)
        .filter(|l| l.starts_with(UNREAD))
        .collect();
    let want = UNREADS
        .iter()
        .find(|(eps, _)| (tol.eps() - eps).abs() <= eps * 1e-9)
        .map_or(0, |&(_, n)| n);
    assert_eq!(
        got.len(),
        want,
        "the unescalated side-in-band runs at ε = {:e}:\n{}",
        tol.eps(),
        got.join("\n")
    );
}

/// A run line [`check_pose`] reports for a body whose tier 3′ refusal
/// holds `FaceThroughEdgeWedgeUndecided` or
/// `BoundaryCrossesItselfUndecided` in band with no `CensusEscalated`
/// beside it: the side of a solid's touch with itself, along an edge
/// lying in its face or at a point, is the one reading in band. It is no pass: the poses'
/// test counts them ([`UNREADS`]), and the witness tests accept them.
const UNREAD: &str = "UNREAD ";

/// How many runs of the poses' test read [`UNREAD`], by ε; none at any
/// ε not listed.
const UNREADS: &[(f64, usize)] = &[(1e-9, 2), (1e-6, 2), (1e-12, 3)];

/// A pose: `(prism, edge, sixteenths, tilt)`.
type Pose = (&'static str, usize, f64, f64);

/// The poses' runs pinned to refuse on limb 2's measured in-band
/// escalation, by ε: on the zero band's edge (`|d| = ε`), the
/// [`EDGE_ON_FACES`] class — the section edge from the pierce to the far
/// vertex measures its end one rounding past the band (about `ε + 2e-16`
/// m at ε = 1e-9), in certification arithmetic, so limb 2 escalates on
/// that measured value (`Escalated` on `HullValue`, which the boolean
/// refuses as `Escalated` on `EdgeOnFaces`), the input ambiguous
/// at this ε. No other run of these poses refuses on it.
const BAND_EDGE_RUNS: &[(f64, Pose, &[&str])] = &[
    (
        1e-9,
        ("Lmirror", 0, 12.0, -1e-9),
        &["prism ∪ cube", "cube ∩ prism", "prism ∖ cube"],
    ),
    (
        1e-9,
        ("Ltop", 0, 12.0, -1e-9),
        &["prism ∪ cube", "cube ∩ prism", "prism ∖ cube"],
    ),
    (
        1e-9,
        ("Ltop", 0, 12.0, 1e-9),
        &["prism ∪ cube", "cube ∩ prism", "prism ∖ cube"],
    ),
    (
        1e-6,
        ("Lmirror", 1, 0.0, -1e-6),
        &[
            "prism ∪ cube",
            "prism ∩ cube",
            "cube ∩ prism",
            "prism ∖ cube",
            "cube ∖ prism",
        ],
    ),
    (
        1e-6,
        ("Ltop", 0, 12.0, -1e-6),
        &[
            "prism ∪ cube",
            "cube ∪ prism",
            "cube ∩ prism",
            "prism ∖ cube",
        ],
    ),
];

/// Whether a run's refusal is limb 2's measured in-band escalation as
/// the boolean reads it: `Escalated` on [`BooleanDecision::EdgeOnFaces`],
/// decided by limb 2's value predicate (`ssi_hull_value`) — the section
/// edge's end measured in the ambiguity band or straddling its edge.
fn measured_in_band(refusal: &str) -> bool {
    refusal.contains("Escalated { decision: EdgeOnFaces")
        && refusal.contains("predicate: Some(\"ssi_hull_value\")")
}

/// **The band-edge runs escalate on the measured value**
/// ([`BAND_EDGE_RUNS`]): at their ε each pinned pose's runs that refuse
/// on limb 2's measured value ([`measured_in_band`]) are exactly the pinned ones,
/// and none of them reads as the kernel's limit.
#[test]
fn the_band_edge_runs_escalate_on_the_measured_value() {
    let tol = Tol::witness();
    for (eps, pose, ops) in BAND_EDGE_RUNS {
        if (tol.eps() - eps).abs() > eps * 1e-9 {
            continue;
        }
        let (_, _, refused) = check_pose_refusals(*pose, tol);
        let in_band: Vec<&str> = refused
            .iter()
            .filter(|(_, e)| measured_in_band(e))
            .map(|(op, _)| *op)
            .collect();
        assert_eq!(&in_band, ops, "{pose:?} at ε = {eps:e}: {refused:#?}");
        for (op, e) in &refused {
            assert!(
                !test_utils::kernel_limit::is_kernel_limit(e),
                "{pose:?}, {op}: a band-edge run read as the kernel's limit: {e}"
            );
        }
    }
}

/// **The germ witnesses build every run**, but for the band-edge runs
/// [`BAND_EDGE_RUNS`] pins, which refuse as the input's measured in-band
/// escalation and as nothing else.
#[test]
fn the_pierce_germs_witnesses_build_every_run() {
    let tol = Tol::witness();
    for pose in GERM_WITNESSES {
        let (wrong, built, refused) = check_pose_refusals(pose, tol);
        let wrong: Vec<_> = wrong
            .into_iter()
            .filter(|l| !l.starts_with(UNREAD))
            .collect();
        let pinned: &[&str] = BAND_EDGE_RUNS
            .iter()
            .find(|(eps, p, _)| *p == pose && (tol.eps() - eps).abs() <= eps * 1e-9)
            .map_or(&[], |(_, _, ops)| ops);
        let ops: Vec<&str> = refused.iter().map(|(op, _)| *op).collect();
        assert!(
            wrong.is_empty() && built == 6 - pinned.len() && ops == pinned,
            "{pose:?}: {built} of 6 built, refused {ops:?}, pinned {pinned:?}:\n{}",
            wrong.join("\n")
        );
        for (op, e) in &refused {
            assert!(
                measured_in_band(e),
                "{pose:?}, {op}: pinned to the measured in-band escalation, got {e}"
            );
        }
    }
}

#[test]
fn the_edge_and_twin_witnesses_build() {
    let tol = Tol::witness();
    let row: Vec<_> = EDGE_WITNESSES
        .iter()
        .filter(|w| (tol.eps() - w.0).abs() <= w.0 * 1e-9)
        .collect();
    if row.is_empty() {
        println!("SKIPPED at this ε: the edge and twin witnesses are poses of ε = 1e-6 and 1e-12");
        return;
    }
    for &&(_, name, edge, a, d, want) in &row {
        let (wrong, built) = check_pose((name, edge, a, d), tol);
        let wrong: Vec<_> = wrong
            .into_iter()
            .filter(|l| !l.starts_with(UNREAD))
            .collect();
        assert!(
            wrong.is_empty() && built >= want,
            "{name} nt e{edge} a{a} d{d:e}: {built} of 6 built, {want} wanted:\n{}",
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
                         clearly on it or clearly off it, and an edge that ends on it leaves it at \
                         a clear angle",
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
