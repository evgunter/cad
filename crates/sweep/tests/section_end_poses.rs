//! **A section chord's pin read in band is the input's where its
//! carrier resolves the edge, and a kernel limitation where the carrier's
//! own rounding can account for it**, at the split and at the Boolean.
//!
//! The join mints each curved section chord on its conic, and the
//! attachment gate pins the carrier's ends to the chord's vertices. Two
//! readings of the input put an end one rounding past the band:
//!
//! - a vertex read on the plane at the zero band's edge: a cylinder cut
//!   through its seam vertex `d` off it, the plane tilted `t` off square;
//! - a plane read square to the wall's axis in its zero band, `t·r = ε`:
//!   the table stands the axis-normal circle in for the section, and a
//!   vertex on the plane misses that circle's plane by the tilt; at the
//!   Boolean, a block's face across the cylinder turned `t`.
//!
//! (A frustum's rim vertex and a box face passing the seam vertex at the
//! band's edge are read upstream now, against the section they end:
//! `vertex_section_side`.)
//!
//! Each refuses typed with its lever, and the lever builds: the vertex
//! moved onto the plane or clearly off it; the plane square or clearly
//! off square. The planes nearly along the axis cut elongated ellipses
//! whose centres lie about `r/θ` off the face, so the carrier resolves
//! its arc only to `u` times that distance: their in-band ends are a
//! kernel limitation, with no lever.
//!
//! Every pose was measured at its row
//! (`examples/section_end_census_probe.rs`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::CertCheck;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, split};
use topo::{AtRestBody, Body, BooleanError, SectionEndReading, SplitJoinError};

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

/// `(ε, t, d, reading)`: the cylinder by the plane through `(1, 0, d)`
/// with normal `(t, 0, 1)` refuses at its row: its seam vertex at the
/// zero band's edge (`OnPlane`), or the plane read square, `t·r = ε`,
/// far from any vertex (`SquareToAxis`).
const SPLITS: [(f64, f64, f64, SectionEndReading); 14] = {
    use SectionEndReading::{OnPlane as V, SquareToAxis as S};
    [
        (1e-9, 1e-3, 1e-9, V),
        (1e-9, 1e-3, -1e-9, V),
        (1e-9, 1e-9, 5e-8, S),
        (1e-9, 1e-9, 1e-6, S),
        (1e-9, 1e-9, 1e-4, S),
        (1e-6, 1e-1, 1e-6, V),
        (1e-6, 1e-1, -1e-6, V),
        (1e-6, 1e-6, 5e-5, S),
        (1e-6, 1e-6, 1e-1, S),
        (1e-12, 1e-2, 1e-12, V),
        (1e-12, 1e-2, -1e-12, V),
        (1e-12, 1e-1, -1e-12, V),
        (1e-12, 1e-12, 5e-11, S),
        (1e-12, 1e-12, 1e-9, S),
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
/// and its lever, on an end pin; and the lever builds: the vertex onto
/// the plane or clearly off it, or the plane square or clearly off
/// square.
#[test]
fn a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_split_and_its_lever_builds() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let cyl = cylinder(tol);
    let cut = |t: f64, d: f64| {
        let normal = Vec3::new(t, 0.0, 1.0);
        let plane =
            topo::test_support::split_plane(Point3::new(1.0, 0.0, d), normal / normal.norm(), tol);
        split(&cyl, &plane, tol)
    };
    let mut wrong = Vec::new();
    for (_, t, d, want) in rows(&SPLITS, |r| r.0, eps) {
        let tag = format!("t{t:e} d{d:e}");
        match cut(t, d) {
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
            if let Err(e) = cut(t, d) {
                wrong.push(format!("{tag} moved to t{t:e} d{d:e}: {e:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Which Boolean pose, against the cylinder: each a block whose one face
/// across the cylinder is turned `t` about `y`, read square to the axis
/// at `t·r = ε`.
#[derive(Clone, Copy, Debug)]
enum Against {
    /// The box `[−3, 3]² × [−1, 0.5]`, turned about `(0, 0, 0.5)`.
    SquareBox,
    /// The slab `[1 + dd, 3] × [−2, 2] × [0.25, 0.75]`, its face at
    /// `x = 1 + dd` just inside the wall, turned about `(1 + dd, 0, 0.5)`.
    WallTilt { dd: f64 },
}

/// `(ε, pose, t)`: each refuses at its row in every op and order. At
/// ε = 1e-12 none reaches the join in band: the square box builds at
/// `t = ε`, and the slab refuses downstream
/// (`work/cleave/a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12.md`).
const BOOLEANS: [(f64, Against, f64); 6] = [
    (1e-9, Against::SquareBox, 1e-9),
    (1e-9, Against::WallTilt { dd: -5e-8 }, 1e-9),
    (1e-9, Against::WallTilt { dd: -1e-6 }, 1e-9),
    (1e-6, Against::SquareBox, 1e-6),
    (1e-6, Against::WallTilt { dd: -5e-5 }, 1e-6),
    (1e-6, Against::WallTilt { dd: -1e-3 }, 1e-6),
];

type Op =
    fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<topo::BooleanResult<f64>, BooleanError>;

const OPS: [(&str, Op); 3] = [
    ("U", topo::union),
    ("I", topo::intersect),
    ("S", topo::subtract),
];

/// At each ε row, each Boolean pose refuses in every op and order as
/// [`SectionEndReading::SquareToAxis`] with its question and lever; and
/// the lever builds: the face square, or clearly off square.
#[test]
fn a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds() {
    let tol = Tol::witness();
    let eps = tol.eps();
    if (eps - 1e-12).abs() <= 1e-15 {
        println!(
            "SKIPPED at this ε: no Boolean pose reaches the join in band at 1e-12; the square \
             box builds and the slab refuses downstream \
             (work/cleave/a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12.md)"
        );
        return;
    }
    let cyl = cylinder(tol);
    let rect = |x0: f64, y0: f64, x1: f64, y1: f64| {
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .to_vec()
    };
    let block = |pose: Against, t: f64| {
        let (upright, about) = match pose {
            Against::SquareBox => (
                sweep::test_support::prism_at(rect(-3.0, -3.0, 3.0, 3.0), -1.0, 1.5, tol),
                Point3::new(0.0, 0.0, 0.5),
            ),
            Against::WallTilt { dd } => (
                sweep::test_support::prism_at(rect(1.0 + dd, -2.0, 3.0, 2.0), 0.25, 0.5, tol),
                Point3::new(1.0 + dd, 0.0, 0.5),
            ),
        };
        let turned = topo::transform_rigid(
            &upright,
            &geom_core::test_support::rigid(
                &Affine3::rotation_about_axis(about, Vec3::new(0.0, 1.0, 0.0), t),
                tol,
            ),
            tol,
        )
        .unwrap();
        finished(turned, tol)
    };
    let mut wrong = Vec::new();
    for (_, pose, t) in rows(&BOOLEANS, |r| r.0, eps) {
        let other = block(pose, t);
        for (order, a, b) in [("ab", &cyl, &other), ("ba", &other, &cyl)] {
            for (op, f) in OPS {
                let tag = format!("{pose:?} t{t:e} {order} {op}");
                match f(a, b, tol) {
                    Err(e @ BooleanError::Join(_))
                        if matches!(
                            &e,
                            BooleanError::Join(j) if matches!(j.error(),
                            SplitJoinError::SectionEndEscalated {
                                reading: SectionEndReading::SquareToAxis,
                                ..
                            })
                        ) =>
                    {
                        let text = e.to_string();
                        if !text.contains(SQUARE) || !text.ends_with(SQUARE_LEVER) {
                            wrong.push(format!("{tag}: reads otherwise: {text}"));
                        }
                    }
                    got => wrong.push(format!("{tag}: got {:?}", got.err())),
                }
            }
        }
        for t in [0.0, CLEARLY_TILTED, -CLEARLY_TILTED] {
            let other = block(pose, t);
            for (order, a, b) in [("ab", &cyl, &other), ("ba", &other, &cyl)] {
                for (op, f) in OPS {
                    if let Err(e) = f(a, b, tol) {
                        wrong.push(format!("{pose:?} moved to t{t:e} {order} {op}: {e:?}"));
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// Planes nearly along the cylinder's axis, `θ` off it, through
/// `(w·cos φ, w·sin φ, z)` with normal `(cos φ, sin φ, θ)`, `φ = 0.4 +
/// 0.7·k` as the census spells it: no vertex lies near any of them, and
/// at ε = 1e-12 each escalates a chord's end pin on its own rounding.
const AXIAL: [(f64, u8, f64, f64); 8] = [
    (0.3, 0, 1e-5, 0.3),
    (0.3, 4, 3e-5, 0.3),
    (0.7, 2, 3e-5, 0.5),
    (0.95, 1, 3e-5, 0.3),
    (0.7, 7, 1e-4, 0.5),
    (0.95, 2, 1e-4, 0.5),
    (0.7, 3, 3e-4, 0.3),
    (0.95, 3, 3e-4, 0.5),
];

fn axial_split(
    cyl: &AtRestBody<f64>,
    (w, k, th, z): (f64, u8, f64, f64),
    tol: Tol,
) -> Result<topo::SplitResult<f64>, SplitError> {
    let phi = 0.4 + f64::from(k) * 0.7;
    let (c, s) = (phi.cos(), phi.sin());
    let normal = Vec3::new(c, s, th);
    let plane =
        topo::test_support::split_plane(Point3::new(c * w, s * w, z), normal / normal.norm(), tol);
    split(cyl, &plane, tol)
}

/// The words a section's unresolved end pin ends in: its cause, then
/// D4 ¶1 (i)'s last resort, and no lever.
const UNRESOLVED: &str =
    "The section curve's own rounding over this edge can account for this miss.";

/// **The section's own rounding is never offered as the input's.** At
/// every ε row, no plane nearly along the axis refuses as a reading of
/// the input; at ε = 1e-12 each refuses with an end pin on a carrier
/// that cannot resolve it, a kernel limitation in exactly its words: no
/// lever and no defect.
#[test]
fn a_section_ends_own_rounding_is_a_kernel_limitation() {
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
        match &got {
            Err(SplitError::Join(
                e @ SplitJoinError::SectionUnresolved {
                    check: CertCheck::EndpointStart | CertCheck::EndpointEnd,
                    diag,
                    ..
                },
            )) => {
                let want = diag
                    .undecided(
                        "whether a section curve's end meets its vertex",
                        format!("{UNRESOLVED} {}", geom_core::KERNEL_LIMIT_RECOURSE),
                    )
                    .to_string();
                if e.to_string() != want {
                    wrong.push(format!("{pose:?}: reads otherwise: {e}"));
                }
            }
            other => wrong.push(format!("{pose:?}: {:?}", other.as_ref().err())),
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
