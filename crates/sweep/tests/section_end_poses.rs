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
use topo::{AtRestBody, Body, BooleanError, SectionEndReading, SectionPinMiss, SplitJoinError};

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
            match cut(t, d) {
                Ok(_) => {}
                Err(e) => wrong.push(format!("{tag} moved to t{t:e} d{d:e}: {e:?}")),
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

/// Planes `θ` = 1e-6 … 1e-5 off the axis, spelled as [`AXIAL`]: at
/// ε = 1e-12 each misses a chord's end past the band, by less than the
/// carrier resolves it to.
const AXIAL_PAST: [(f64, u8, f64, f64); 8] = [
    (0.3, 0, 3e-6, 0.5),
    (0.3, 3, 1e-6, 0.5),
    (0.3, 7, 3e-6, 0.5),
    (0.7, 1, 1e-6, 0.3),
    (0.7, 4, 3e-6, 0.3),
    (0.7, 7, 1e-5, 0.3),
    (0.95, 1, 1e-6, 0.3),
    (0.95, 4, 1e-6, 0.3),
];

/// The cause an in-band end pin on an unresolved carrier states, then
/// D4 ¶1 (i)'s last resort, and no lever.
const UNRESOLVED: &str = "The section curve's own rounding over this edge is too coarse to read this miss as the model's.";

/// The cause a definite end miss within the carrier's resolution states.
const WITHIN: &str = "The section curve's own rounding over this edge can account for this miss.";

/// **The section's own rounding is never offered as the input's, nor
/// called a defect.** At every ε row, no plane nearly along the axis
/// refuses as a reading of the input. At ε = 1e-12 each [`AXIAL`] pose
/// refuses with an in-band end pin on a carrier that cannot resolve it,
/// and each [`AXIAL_PAST`] pose with a definite end miss the carrier's
/// rounding covers: each a kernel limitation, in exactly its words.
#[test]
fn a_section_ends_own_rounding_is_a_kernel_limitation() {
    let tol = Tol::witness();
    let cyl = cylinder(tol);
    let mut wrong = Vec::new();
    let limit = geom_core::KERNEL_LIMIT_RECOURSE;
    for pose in AXIAL.into_iter().chain(AXIAL_PAST) {
        let got = axial_split(&cyl, pose, tol);
        if (tol.eps() - 1e-12).abs() > 1e-15 {
            if let Err(SplitError::Join(e @ SplitJoinError::SectionEndEscalated { .. })) = got {
                wrong.push(format!("{pose:?}: read as the input's: {e:?}"));
            }
            continue;
        }
        let past = AXIAL_PAST.contains(&pose);
        let want = match &got {
            Err(SplitError::Join(SplitJoinError::SectionUnresolved {
                miss:
                    SectionPinMiss::Undecided {
                        check: CertCheck::EndpointStart | CertCheck::EndpointEnd,
                        diag,
                        ..
                    },
                ..
            })) if !past => Some(
                diag.undecided(
                    "whether a section curve's end meets its vertex",
                    format!("{UNRESOLVED} {limit}"),
                )
                .to_string(),
            ),
            Err(SplitError::Join(SplitJoinError::SectionUnresolved {
                miss:
                    SectionPinMiss::Past {
                        check: check @ (CertCheck::EndpointStart | CertCheck::EndpointEnd),
                        sample,
                        margin,
                    },
                ..
            })) if past => Some(format!(
                "a section curve's end misses its vertex: {check} at sample {sample} is \
                 {margin:e}, definitely past the tolerance. {WITHIN} {limit}"
            )),
            _ => None,
        };
        match (want, &got) {
            (Some(want), Err(SplitError::Join(e))) if e.to_string() == want => {}
            (_, got) => wrong.push(format!("{pose:?}: {:?}", got.as_ref().err())),
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The unit cylinder about `z`, its axis at `(x0, 0)` and its base at
/// `z0`, cut by the plane through `(x0 + 1, 0, z)` with normal `(t, 0, 1)`
/// at `z` and at the lever's moves of `z`.
fn moved_cut(
    (x0, z0): (f64, f64),
    t: f64,
    tol: Tol,
) -> impl Fn(f64) -> Result<topo::SplitResult<f64>, SplitError> {
    let cyl = finished(
        sweep::test_support::prism_at(
            vec![
                (Point2::new(x0 + 1.0, 0.0), 1.0),
                (Point2::new(x0 - 1.0, 0.0), 1.0),
            ],
            z0,
            1.0,
            tol,
        ),
        tol,
    );
    move |z: f64| {
        let normal = Vec3::new(t, 0.0, 1.0);
        let plane = topo::test_support::split_plane(
            Point3::new(x0 + 1.0, 0.0, z),
            normal / normal.norm(),
            tol,
        );
        split(&cyl, &plane, tol)
    }
}

/// `t` at each ε row of the translation pins, and whether the row runs.
fn translation_tilt(eps: f64) -> Option<f64> {
    if (eps - 1e-12).abs() <= 1e-15 {
        Some(1e-2)
    } else if (eps - 1e-9).abs() <= 1e-12 {
        Some(1e-3)
    } else {
        println!("SKIPPED at this ε: the rows are ε = 1e-12 and 1e-9");
        None
    }
}

/// **Where the part sits across the miss does not move the reading.**
/// The seam vertex `ε` off a plane tilted `t` off square (`1e-2` at
/// ε = 1e-12, `1e-3` at 1e-9), on the unit cylinder moved `x0` across
/// the miss (0, 30 m and 1 km at 1e-12, 30 km at 1e-9): each refuses as
/// the vertex on the plane, with its lever, and the lever builds.
#[test]
fn a_section_end_reads_the_same_wherever_the_part_sits() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let Some(t) = translation_tilt(eps) else {
        return;
    };
    let at: &[f64] = if t == 1e-2 { &[0.0, 30.0, 1e3] } else { &[3e4] };
    let mut wrong = Vec::new();
    for &x0 in at {
        let cut = moved_cut((x0, 0.0), t, tol);
        match cut(eps) {
            Err(SplitError::Join(
                e @ SplitJoinError::SectionEndEscalated {
                    reading: SectionEndReading::OnPlane,
                    ..
                },
            )) if e.to_string().ends_with(ON_PLANE_LEVER) => {}
            other => wrong.push(format!("x0 {x0}: {:?}", other.err())),
        }
        for d in [0.0, CLEARLY_OFF * eps, -CLEARLY_OFF * eps] {
            if let Err(e) = cut(d) {
                wrong.push(format!("x0 {x0} moved to d{d:e}: {e:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **Along the miss, the vertex and the section end read one offset.**
/// The same pose with the cylinder's base at `z0` (30 m and 1 km at
/// ε = 1e-12, 1 km at 1e-9), so the miss runs along the large
/// coordinate. The plane passes the vertex by the two offsets `z0`'s
/// grid holds either side of ε. The vertex's reading off the plane and
/// the section end's miss read that one offset, to within far less than
/// the grid's spacing (`3.6e-15` at 30 m), so no grid point lands
/// between them: the offset at or under ε builds, and the one over it
/// refuses as the input's at the vertex, with its lever — never as the
/// kernel's, and never past the join's pin at a rounding of the part's
/// extent. The lever's moves build.
#[test]
fn along_the_miss_the_vertex_and_its_section_end_read_one_offset() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let Some(t) = translation_tilt(eps) else {
        return;
    };
    let at: &[f64] = if t == 1e-2 { &[30.0, 1e3] } else { &[1e3] };
    let mut wrong = Vec::new();
    for &z0 in at {
        let cut = moved_cut((0.0, z0), t, tol);
        let near = z0 + eps;
        let next = f64::from_bits(near.to_bits() + 1);
        let prev = f64::from_bits(near.to_bits() - 1);
        let (under, over) = if near - z0 <= eps {
            (near, next)
        } else {
            (prev, near)
        };
        if let Err(e) = cut(under) {
            wrong.push(format!("z0 {z0}, {:e} off: {e:?}", under - z0));
        }
        match cut(over) {
            Err(e @ SplitError::Reduce(topo::SplitReduceError::SliverVertex { .. })) => {
                let text = e.to_string();
                if !text.contains("Recourse: move") {
                    wrong.push(format!("z0 {z0}, {:e} off: no lever: {text}", over - z0));
                }
            }
            other => wrong.push(format!("z0 {z0}, {:e} off: {:?}", over - z0, other.err())),
        }
        for d in [0.0, CLEARLY_OFF * eps, -CLEARLY_OFF * eps] {
            if let Err(e) = cut(z0 + d) {
                wrong.push(format!("z0 {z0} moved to d{d:e}: {e:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// `(ε, h)`: the ring torus `R = 2`, `r = 0.5` about `y` against a block
/// whose face `y = h` is turned `ε/(R + r)` about `z`, read square to the
/// axis, refuses at its row.
const TORUS_SQUARE: [(f64, f64); 5] = [
    (1e-9, 0.0),
    (1e-9, 0.2),
    (1e-6, 0.0),
    (1e-6, 0.2),
    (1e-12, 0.2),
];

/// **A torus cut square to its axis is read square.** At each ε row the
/// concentric circles the table stands in for the face refuse in every
/// op and order as [`SectionEndReading::SquareToAxis`], with its
/// question and lever. The face moved square builds; turned clearly off
/// square it cuts the spiric quartic, which the kernel does not build
/// yet (`GermSectionSpiric`).
#[test]
fn a_torus_section_read_square_refuses_as_the_inputs_and_its_lever_builds() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let torus = finished(
        sweep::test_support::revolved_about_y(
            vec![(Point2::new(2.0, -0.5), 1.0), (Point2::new(2.0, 0.5), 1.0)],
            sweep::Revolution::Full,
            tol,
        ),
        tol,
    );
    let block = |h: f64, t: f64| {
        let rect = [(-4.0, -4.0), (4.0, -4.0), (4.0, h), (-4.0, h)]
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .to_vec();
        let upright = sweep::test_support::prism_at(rect, -4.0, 8.0, tol);
        let turned = topo::transform_rigid(
            &upright,
            &geom_core::test_support::rigid(
                &Affine3::rotation_about_axis(Point3::new(0.0, h, 0.0), Vec3::unit_z(), t),
                tol,
            ),
            tol,
        )
        .unwrap();
        finished(turned, tol)
    };
    let mut wrong = Vec::new();
    for (_, h) in rows(&TORUS_SQUARE, |r| r.0, eps) {
        let t = eps / 2.5;
        let other = block(h, t);
        for (order, a, b) in [("ab", &torus, &other), ("ba", &other, &torus)] {
            for (op, f) in OPS {
                let tag = format!("h{h} {order} {op}");
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
            let other = block(h, t);
            for (order, a, b) in [("ab", &torus, &other), ("ba", &other, &torus)] {
                for (op, f) in OPS {
                    match (t == 0.0, f(a, b, tol)) {
                        (true, Ok(_)) | (false, Err(BooleanError::GermSectionSpiric { .. })) => {}
                        (_, got) => wrong.push(format!(
                            "h{h} moved to t{t:e} {order} {op}: {:?}",
                            got.err()
                        )),
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
