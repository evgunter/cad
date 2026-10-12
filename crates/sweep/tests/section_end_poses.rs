//! **A section chord's end read in band of the vertex it meets is the
//! input's reading where the vertex's offset off the plane is the miss,
//! and the kernel's where it is the section conic's own**, at the split
//! and at the Boolean.
//!
//! The join mints each curved section chord on its conic, and the
//! attachment gate pins the carrier's ends to the chord's vertices. Two
//! readings of the input put an end one rounding past the band:
//!
//! - a vertex read on the plane at the zero band's edge: a cylinder or a
//!   cone frustum cut through its rim vertex `d` off it, the plane tilted
//!   `t` off square, and a box whose top face passes `d` from the
//!   cylinder's seam vertex (the Boolean's `EdgeOnFaces`);
//! - a plane read square to the wall's axis in its zero band, `t·r = ε`:
//!   the table stands the axis-normal circle in for the section, and a
//!   vertex on the plane misses that circle's plane by the tilt.
//!
//! Each refuses typed with its lever, and the lever builds: the vertex
//! moved onto the plane or clearly off it; the plane square or clearly
//! off square. The planes nearly along the axis cut the split's own
//! elongated ellipses, whose rounding grows past the band with no vertex
//! near it: those ends stay the gate's refusal, a kernel defect.
//!
//! Every pose was measured at its row
//! (`examples/section_end_census_probe.rs`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, split};
use topo::{
    AtRestBody, Body, BooleanDecision, BooleanError, EulerOpError, SectionEndReading,
    SplitJoinError,
};

fn finished(body: Body<f64>, tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol).expect("a valid operand")
}

/// The unit cylinder about `z`, `0 ≤ z ≤ 1`, its seam vertices at
/// `(±1, 0, 0)` and `(±1, 0, 1)`.
fn cylinder(tol: Tol) -> AtRestBody<f64> {
    finished(
        sweep::test_support::prism(
            vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)],
            1.0,
            tol,
        ),
        tol,
    )
}

/// A frustum about `y`: radius 1 at `y = 0` to 0.5 at `y = 1`, its rim
/// vertex at `(1, 0, 0)`.
fn frustum(tol: Tol) -> AtRestBody<f64> {
    finished(
        sweep::test_support::revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(0.5, 1.0), 0.0),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            sweep::Revolution::Full,
            tol,
        ),
        tol,
    )
}

/// Which body a split pose cuts, and how its plane stands.
#[derive(Clone, Copy, Debug)]
enum Cut {
    /// The cylinder by the plane through `(1, 0, d)` with normal
    /// `(t, 0, 1)`.
    Cylinder,
    /// The frustum by the plane through `(1, d, 0)` with normal
    /// `(t, 1, 0)`.
    Frustum,
}

/// `(ε, cut, t, d, reading)`: each refuses at its row.
const SPLITS: [(f64, Cut, f64, f64, SectionEndReading); 20] = {
    use Cut::{Cylinder as C, Frustum as F};
    use SectionEndReading::{OnPlane as V, SquareToAxis as S};
    [
        (1e-9, C, 1e-3, 1e-9, V),
        (1e-9, C, 1e-3, -1e-9, V),
        (1e-9, F, 1e-3, 1e-9, V),
        (1e-9, F, 1e-1, -1e-9, V),
        (1e-9, C, 1e-9, 5e-8, S),
        (1e-9, C, 1e-9, 1e-6, S),
        (1e-9, C, 1e-9, 1e-4, S),
        (1e-6, C, 1e-1, 1e-6, V),
        (1e-6, C, 1e-1, -1e-6, V),
        (1e-6, F, 1e-1, 1e-6, V),
        (1e-6, C, 1e-6, 5e-5, S),
        (1e-6, C, 1e-6, 1e-1, S),
        (1e-12, C, 1e-3, 1e-12, V),
        (1e-12, C, 1e-2, -1e-12, V),
        (1e-12, C, 1e-1, -1e-12, V),
        (1e-12, F, 1e-3, -1e-12, V),
        (1e-12, F, 1e-1, 1e-12, V),
        (1e-12, C, 1e-12, 5e-11, S),
        (1e-12, C, 1e-12, 1e-9, S),
        (1e-12, C, 1e-12, 1e-7, S),
    ]
};

const ON_PLANE: &str = "whether an edge along the cut lies on both faces it bounds is undecided";

const ON_PLANE_LEVER: &str = "Recourse: move the split plane or the geometry so each edge and \
                              vertex that meets the plane lies clearly on it or clearly off it, \
                              and an edge that ends on it leaves it at a clear angle";

const SQUARE: &str =
    "whether a section read square to a curved face's axis ends at its vertex is undecided";

const SQUARE_LEVER: &str = "Recourse: move the geometry so each plane that cuts a curved face \
                            runs clearly square to its axis or clearly off square, and each \
                            vertex that meets it lies clearly on it or clearly off it";

/// Clearly off the plane: past the band, `Kε = 10ε`.
const CLEARLY_OFF: f64 = 50.0;

/// Clearly off square at every row: a centiradian reads the tilt, and the
/// ellipse it makes, definite (a milliradian leaves the ellipse's axes in
/// band of equal at ε = 1e-6).
const CLEARLY_TILTED: f64 = 1e-2;

fn rows<R: Copy>(all: &[R], eps_of: impl Fn(&R) -> f64, eps: f64) -> Vec<R> {
    let rows: Vec<R> = all
        .iter()
        .filter(|r| (eps_of(r) - eps).abs() <= 1e-3 * eps)
        .copied()
        .collect();
    assert!(
        !rows.is_empty(),
        "no poses are pinned at ε = {eps:e}: the rows are ε = 1e-9, 1e-6 and 1e-12"
    );
    rows
}

/// At each ε row, each split pose refuses
/// [`SplitJoinError::SectionEndEscalated`] with its reading, its question
/// and its lever; and the lever builds: the vertex onto the plane or
/// clearly off it, or the plane square or clearly off square.
#[test]
fn a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_split_and_its_lever_builds() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let (cyl, cone) = (cylinder(tol), frustum(tol));
    let cut = |at: Cut, t: f64, d: f64| {
        let (body, origin, normal) = match at {
            Cut::Cylinder => (&cyl, Point3::new(1.0, 0.0, d), Vec3::new(t, 0.0, 1.0)),
            Cut::Frustum => (&cone, Point3::new(1.0, d, 0.0), Vec3::new(t, 1.0, 0.0)),
        };
        let plane = topo::test_support::split_plane(origin, normal / normal.norm(), tol);
        split(body, &plane, tol)
    };
    let mut wrong = Vec::new();
    for (_, at, t, d, want) in rows(&SPLITS, |r| r.0, eps) {
        let tag = format!("{at:?} t{t:e} d{d:e}");
        match cut(at, t, d) {
            Err(SplitError::Join(e))
                if matches!(
                    e,
                    SplitJoinError::SectionEndEscalated { reading, .. } if reading == want
                ) =>
            {
                let (question, lever) = match want {
                    SectionEndReading::OnPlane => (ON_PLANE, ON_PLANE_LEVER),
                    SectionEndReading::SquareToAxis => (SQUARE, SQUARE_LEVER),
                };
                let text = e.to_string();
                let pin = matches!(
                    e,
                    SplitJoinError::SectionEndEscalated { ref diag, .. }
                        if matches!(
                            diag.predicate,
                            Some("carrier_endpoint_start" | "carrier_endpoint_end")
                        )
                );
                if !text.starts_with(question) || !text.ends_with(lever) || !pin {
                    wrong.push(format!("{tag}: reads otherwise: {e:?}: {text}"));
                }
            }
            other => wrong.push(format!("{tag}: want {want:?}, got {:?}", other.err())),
        }
        let moves: Vec<(f64, f64)> = match want {
            SectionEndReading::OnPlane => [0.0, CLEARLY_OFF * eps, -CLEARLY_OFF * eps]
                .map(|moved| (t, moved))
                .to_vec(),
            SectionEndReading::SquareToAxis => [0.0, CLEARLY_TILTED, -CLEARLY_TILTED]
                .map(|moved| (moved, d))
                .to_vec(),
        };
        for (t, d) in moves {
            if let Err(e) = cut(at, t, d) {
                wrong.push(format!("{tag} moved to t{t:e} d{d:e}: {e:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Which Boolean pose, against the cylinder.
#[derive(Clone, Copy, Debug)]
enum Against {
    /// A box below the plane through `(1, 0, d)`, its top face turned
    /// `t` about `y` there: the seam vertex `(1, 0, 0)` `d` off it.
    SeamVertex,
    /// A box below `z = 0.5`, its top face turned `t` about `y` through
    /// the axis.
    SquareBox,
}

/// `(ε, pose, t, d)`: each refuses at its row, the seam vertex's as
/// `EdgeOnFaces`, the box's as the plane read square.
const BOOLEANS: [(f64, Against, f64, f64); 5] = [
    (1e-9, Against::SeamVertex, 1e-3, 1e-9),
    (1e-9, Against::SeamVertex, 1e-2, -1e-9),
    (1e-9, Against::SquareBox, 1e-9, 0.0),
    (1e-6, Against::SeamVertex, 1e-2, -1e-6),
    (1e-6, Against::SquareBox, 1e-6, 0.0),
];

type Op =
    fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<topo::BooleanResult<f64>, BooleanError>;

const OPS: [(&str, Op); 3] = [
    ("U", topo::union),
    ("I", topo::intersect),
    ("S", topo::subtract),
];

/// At each ε row, each Boolean pose refuses in every op and order: the
/// seam vertex's end as [`BooleanDecision::EdgeOnFaces`], the square
/// box's as [`SectionEndReading::SquareToAxis`], each with its lever; and
/// the lever builds.
#[test]
fn a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds() {
    let tol = Tol::witness();
    let eps = tol.eps();
    if (eps - 1e-12).abs() <= 1e-15 {
        println!(
            "SKIPPED at this ε: the seam vertex at the band's edge reads in band of the plane \
             first (`Coincidence`), and the square box certifies its chords and refuses \
             downstream (work/cleave/a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12.md)"
        );
        return;
    }
    let cyl = cylinder(tol);
    let rect = |x0: f64, y0: f64, x1: f64, y1: f64| {
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .to_vec()
    };
    let block = |pose: Against, t: f64, d: f64| {
        let (z0, h, about) = match pose {
            Against::SeamVertex => (d - 2.0, 2.0, Point3::new(1.0, 0.0, d)),
            Against::SquareBox => (-1.0, 1.5, Point3::new(0.0, 0.0, 0.5)),
        };
        let upright = sweep::test_support::prism_at(rect(-3.0, -3.0, 3.0, 3.0), z0, h, tol);
        let turned = topo::transform_rigid(
            &finished(upright, tol),
            &Affine3::rotation_about_axis(about, Vec3::new(0.0, 1.0, 0.0), t),
            tol,
        )
        .unwrap();
        finished(turned, tol)
    };
    let mut wrong = Vec::new();
    for (_, pose, t, d) in rows(&BOOLEANS, |r| r.0, eps) {
        let other = block(pose, t, d);
        for (order, a, b) in [("ab", &cyl, &other), ("ba", &other, &cyl)] {
            for (op, f) in OPS {
                let tag = format!("{pose:?} t{t:e} d{d:e} {order} {op}");
                match (pose, f(a, b, tol)) {
                    (
                        Against::SeamVertex,
                        Err(
                            e @ BooleanError::Escalated {
                                decision: BooleanDecision::EdgeOnFaces,
                                ..
                            },
                        ),
                    ) => {
                        let text = e.to_string();
                        if !text.contains(
                            "move the geometry so each edge and vertex that meets a face lies \
                             clearly on it or clearly off it",
                        ) {
                            wrong.push(format!("{tag}: reads otherwise: {text}"));
                        }
                    }
                    (
                        Against::SquareBox,
                        Err(
                            e @ BooleanError::Join(SplitJoinError::SectionEndEscalated {
                                reading: SectionEndReading::SquareToAxis,
                                ..
                            }),
                        ),
                    ) => {
                        let text = e.to_string();
                        if !text.contains(SQUARE) || !text.ends_with(SQUARE_LEVER) {
                            wrong.push(format!("{tag}: reads otherwise: {text}"));
                        }
                    }
                    (_, got) => wrong.push(format!("{tag}: got {:?}", got.err())),
                }
            }
        }
        let moves: Vec<(f64, f64)> = match pose {
            Against::SeamVertex => [0.0, CLEARLY_OFF * eps, -CLEARLY_OFF * eps]
                .map(|moved| (t, moved))
                .to_vec(),
            Against::SquareBox => [0.0, CLEARLY_TILTED, -CLEARLY_TILTED]
                .map(|moved| (moved, d))
                .to_vec(),
        };
        for (t, d) in moves {
            let other = block(pose, t, d);
            for (order, a, b) in [("ab", &cyl, &other), ("ba", &other, &cyl)] {
                for (op, f) in OPS {
                    if let Err(e) = f(a, b, tol) {
                        wrong.push(format!(
                            "{pose:?} moved to t{t:e} d{d:e} {order} {op}: {e:?}"
                        ));
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Planes nearly along the cylinder's axis, `θ` off it, through
/// `(w·cos φ, w·sin φ, z)` with normal `(cos φ, sin φ, θ)`: no vertex lies
/// near any of them.
const AXIAL: [(f64, f64, f64, f64); 8] = [
    (0.0, 0.4, 3e-4, 0.5),
    (0.0, 1.1, 1e-5, 0.3),
    (0.0, 2.5, 1e-5, 0.5),
    (0.0, 5.3, 3e-5, 0.5),
    (0.3, 3.2, 3e-5, 0.3),
    (0.7, 1.8, 3e-5, 0.5),
    (0.7, 4.6, 3e-4, 0.3),
    (0.95, 1.8, 1e-4, 0.5),
];

fn axial_split(
    cyl: &AtRestBody<f64>,
    (w, phi, th, z): (f64, f64, f64, f64),
    tol: Tol,
) -> Result<topo::SplitResult<f64>, SplitError> {
    let (c, s) = (phi.cos(), phi.sin());
    let normal = Vec3::new(c, s, th);
    let plane =
        topo::test_support::split_plane(Point3::new(c * w, s * w, z), normal / normal.norm(), tol);
    split(cyl, &plane, tol)
}

/// **The section's own rounding is never read as the input's.** At
/// every ε row, no plane nearly along the axis refuses as a reading of
/// the input at a chord's end; at ε = 1e-12 each refuses with the gate's
/// in-band endpoint pin, a kernel defect: the elongated ellipse's
/// rounding grown past the band.
#[test]
fn a_section_ends_own_rounding_stays_the_kernels() {
    let tol = Tol::witness();
    let cyl = cylinder(tol);
    let mut wrong = Vec::new();
    for pose in AXIAL {
        let got = axial_split(&cyl, pose, tol);
        if (tol.eps() - 1e-12).abs() > 1e-15 {
            if let Err(SplitError::Join(e @ SplitJoinError::SectionEndEscalated { .. })) = got {
                wrong.push(format!("{pose:?}: read as the input's: {e:?}"));
            }
            continue;
        }
        match got {
            Err(SplitError::Join(
                e @ SplitJoinError::Euler(EulerOpError::Certification {
                    error:
                        geom_brep::CertifyError::Escalated {
                            check:
                                geom_brep::CertCheck::EndpointStart | geom_brep::CertCheck::EndpointEnd,
                            ..
                        },
                }),
            )) if e.to_string().ends_with(geom_core::KERNEL_DEFECT_ENDING) => {}
            other => wrong.push(format!("{pose:?}: {:?}", other.err())),
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
