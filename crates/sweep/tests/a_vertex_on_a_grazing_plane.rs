//! A plane through a seam vertex of the unit cylinder, grazing its wall:
//! the vertex is read against the section the plane cuts in the wall,
//! `δ / sin ψ` from it, not against its offset `δ` from the plane.
//!
//! The cylinder (`0 ≤ z ≤ 1`) is turned `φ` about its axis, so a seam
//! vertex sits at `v = (cos φ, sin φ, 0)`, with the wall's normal `w`
//! there. The plane runs through `v + δ·n`, `n = cos ψ·w + sin ψ·ẑ`, so
//! `ψ` is the dihedral between the plane and the wall at `v`. On the
//! ladder `ψ ∈ {90°, 30°, atan 0.1, 3°}` each pose is set by the reading
//! it should get, `m = |δ| / sin ψ`: within the zero band it builds, in
//! the band it refuses typed at the vertex, and past it the vertex is
//! read off the plane before the join is reached. The same poses run
//! through the Boolean, as the cylinder less a slab whose face is the
//! plane.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::splitting::{SplitError, SplitJoinError, split};
use topo::{AtRestBody, Body, BooleanDecision, BooleanError, Coincide, SplitReduceError};

fn tol() -> Tol {
    Tol::witness()
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn moved(body: &AtRestBody<f64>, by: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        topo::transform_rigid(body, &geom_core::test_support::rigid(&by, tol()), tol()).unwrap(),
    )
}

/// The most the two halves' certified volume pads may sum to: a half
/// whose wall pieces merged across the seam is integrated by
/// quadrature, and its pad is the bound the volume checks allow.
/// Measured (PR 5334): at most 1.9e-9 at ε = 1e-12, 1.25e-6 at 1e-9 and
/// 4.8e-5 at 1e-6.
fn pad_ceiling() -> f64 {
    (4000.0 * tol().eps()).min(1e-4)
}

fn volume(body: &Body<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
    )
    .volume
}

/// The ladder, as `(label, ψ)`.
fn ladder() -> [(&'static str, f64); 4] {
    [
        ("90°", core::f64::consts::FRAC_PI_2),
        ("30°", core::f64::consts::FRAC_PI_6),
        ("atan 0.1", 0.1f64.atan()),
        ("3°", 3f64.to_radians()),
    ]
}

/// The cylinder turned `φ`, and the plane's unit normal at its seam
/// vertex for `ψ`.
fn pose(phi: f64, psi: f64) -> (AtRestBody<f64>, Point3<f64>, Vec3<f64>) {
    pose_of_height(phi, psi, 1.0)
}

/// [`pose`] for the cylinder `0 ≤ z ≤ h`.
fn pose_of_height(phi: f64, psi: f64, h: f64) -> (AtRestBody<f64>, Point3<f64>, Vec3<f64>) {
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let cyl = finished(sweep::test_support::prism(circle, h, tol()));
    let z = Vec3::new(0.0, 0.0, 1.0);
    let cyl = moved(
        &cyl,
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), z, phi),
    );
    let (c, s) = (phi.cos(), phi.sin());
    let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin());
    (cyl, Point3::new(c, s, 0.0), n)
}

/// The offsets a pose is read at, as multiples of ε: each reading `m`
/// in units of ε turned into `δ = m·sin ψ`, both signs, and `δ = 0`.
fn offsets(psi: f64) -> Vec<f64> {
    let mut out = vec![0.0];
    for m in [0.4, 1.8, 6.0] {
        out.extend([m * psi.sin(), -m * psi.sin()]);
    }
    out
}

/// What a pose at offset `d` (metres) reads, with `m = |d| / sin ψ`.
#[derive(Debug, PartialEq)]
enum Reading {
    /// `m` within the zero band.
    On,
    /// `m` in the band; whether `δ` alone already is.
    Band { offset_in_band: bool },
    /// `m` past the band.
    Off,
}

fn reading(d: f64, psi: f64) -> Reading {
    let (e, k) = (tol().eps(), tol().k());
    let m = d.abs() / psi.sin();
    let clear = |x: f64, lo: f64, hi: f64| x * 1.5 <= hi && x >= lo * 1.5;
    if m <= 0.5 * e {
        Reading::On
    } else if clear(m, e, k * e) {
        assert!(
            d.abs() <= 0.95 * e || d.abs() >= 1.5 * e,
            "δ = {d:e} sits on the band's edge"
        );
        Reading::Band {
            offset_in_band: d.abs() > e,
        }
    } else if m >= 1.5 * k * e {
        Reading::Off
    } else {
        panic!("m = {m:e} sits on an edge of the band ({e:e}, {:e})", k * e)
    }
}

/// The reading an in-band refusal reports: its predicate and its
/// number.
fn reported(diag: &geom_core::Indeterminate) -> (&'static str, f64) {
    (
        diag.predicate.expect("a named decision"),
        diag.margin
            .diagnostic_f64_for_error_text()
            .value()
            .expect("a point reading"),
    )
}

/// `got` is `want` to the rounding of the vertex's offset (a few ulps
/// of a unit coordinate) over the sine it is divided by.
fn assert_reads(label: &str, got: f64, want: f64, sine: f64) {
    let slack = 8.0 * f64::EPSILON / sine + 1e-9 * want.abs();
    assert!(
        (got - want).abs() <= slack,
        "{label}: read {got:e}, the closed form is {want:e}"
    );
}

#[test]
fn the_split_reads_a_grazing_vertex_against_its_section() {
    let (e, k) = (tol().eps(), tol().k());
    assert!(
        0.9 / 3f64.to_radians().sin() >= 1.5 * k,
        "the definite rows need K ≤ {}",
        0.9 / 3f64.to_radians().sin() / 1.5
    );
    for phi in [0.9, 2.2] {
        for (name, psi) in ladder() {
            let (cyl, v, n) = pose(phi, psi);
            let cut = |d: f64| {
                let plane = topo::test_support::split_plane(v + n * d, n, tol());
                split(&cyl, &plane, tol())
            };
            let parts = |d: f64, label: &str| {
                let r = cut(d).unwrap_or_else(|err| {
                    panic!(
                        "{label}: refused: {err_at}",
                        err_at = err.render(geom_brep::recourse::Reading::Build)
                    )
                });
                // Each half's volume, and the pad its certificate gives:
                // a half whose wall pieces merged across the seam is
                // integrated by quadrature.
                let vol = |b: Option<&Body<f64>>| {
                    b.map_or((0.0, 0.0), |b| {
                        let m = topo::mass_properties(
                            &topo::test_support::finished(
                                "the measured body",
                                topo::Body::clone(b),
                                tol(),
                            ),
                            tol(),
                            topo::test_support::pinned_resolution(tol()),
                        )
                        .unwrap();
                        (m.enclosure.volume, m.enclosure.volume_pad)
                    })
                };
                let ((a, pa), (b, pb)) = (
                    vol(r.above.body().map(|b| &**b)),
                    vol(r.below.body().map(|b| &**b)),
                );
                eprintln!("{label}: volume pads {pa:e} above, {pb:e} below");
                assert!(
                    pa + pb <= pad_ceiling(),
                    "{label}: the quadrature's pads {pa:e} + {pb:e} pass {:e}",
                    pad_ceiling()
                );
                (a, b, pa + pb)
            };
            let base = parts(0.0, &format!("φ {phi} ψ {name} on the plane"));
            assert!(
                (base.0 + base.1 - core::f64::consts::PI).abs() <= 1e-6 + base.2,
                "φ {phi} ψ {name}: the halves make the cylinder: {base:?}"
            );
            let mut rows: Vec<f64> = offsets(psi).into_iter().map(|m| m * e).collect();
            if name == "3°" {
                rows.extend([0.9 * e, -0.9 * e]);
            }
            for d in rows {
                let label = format!("φ {phi} ψ {name} δ {:+.3}ε", d / e);
                match reading(d, psi) {
                    Reading::On => {
                        let got = parts(d, &label);
                        assert!(
                            (got.0 - base.0).abs() <= 1e-6 + got.2 + base.2
                                && (got.1 - base.1).abs() <= 1e-6 + got.2 + base.2,
                            "{label}: {got:?} against {base:?} on the plane"
                        );
                    }
                    Reading::Band { offset_in_band } => {
                        let Err(SplitError::Reduce(SplitReduceError::SliverVertex {
                            diag, ..
                        })) = cut(d)
                        else {
                            panic!("{label}: the vertex refuses typed at the split");
                        };
                        let (predicate, got) = reported(&diag);
                        if offset_in_band {
                            assert_eq!(predicate, "split_vertex_side", "{label}");
                            assert_reads(&label, got, -d, 1.0);
                        } else {
                            assert_eq!(predicate, "vertex_section_side", "{label}");
                            assert_reads(&label, got, -d / psi.sin(), psi.sin());
                        }
                        assert!(
                            cut(d)
                                .unwrap_err()
                                .render(geom_brep::recourse::Reading::Build)
                                .contains("move the split plane"),
                            "{label}: the refusal names its lever"
                        );
                    }
                    Reading::Off => {
                        // Moved off the cap, the plane cuts the wall clear
                        // of the vertex. Moved into it, the rim lies in the
                        // band of the plane over a stretch that the root
                        // lane reads as a touch while the vertex reads off
                        // by its section, and two ends go unpaired (filed:
                        // `a-rim-grazing-the-plane-at-a-vertex-read-off-by-
                        // its-section-leaves-two-ends-unpaired`).
                        if d > 0.0 {
                            let got = parts(d, &label);
                            assert!(
                                (got.0 - base.0).abs() <= 1e-6 + got.2 + base.2
                                    && (got.1 - base.1).abs() <= 1e-6 + got.2 + base.2,
                                "{label}: {got:?} against {base:?} on the plane"
                            );
                        } else {
                            let r = cut(d);
                            assert!(
                                matches!(
                                    r,
                                    Err(SplitError::Join(SplitJoinError::UnpairedLooseEnds {
                                        count: 2
                                    }))
                                ),
                                "{label}: {:?}",
                                r.err()
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The slab `x ≥ 1` about the seam vertex of the unturned cylinder of
/// height `h`, reaching `back` and `ahead` along the rim's tangent there
/// and past both caps, its face's outward normal `−x` turned onto `−n`
/// for the pose `(φ, ψ)`, then moved by `by`.
fn seam_slab(
    (phi, psi): (f64, f64),
    (back, ahead): (f64, f64),
    h: f64,
    by: Vec3<f64>,
) -> AtRestBody<f64> {
    let slab = finished(sweep::test_support::prism_at(
        [(1.0, -back), (4.0, -back), (4.0, ahead), (1.0, ahead)]
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
        -3.0,
        h + 5.0,
        tol(),
    ));
    let slab = moved(
        &slab,
        Affine3::rotation_about_axis(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), -psi),
    );
    let slab = moved(
        &slab,
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), phi),
    );
    moved(&slab, Affine3::translation(by))
}

#[test]
fn the_boolean_reads_a_grazing_vertex_against_its_section() {
    let e = tol().eps();
    let phi = 2.2;
    for (name, psi) in ladder() {
        let (cyl, v, n) = pose(phi, psi);
        let slab = |d: f64| seam_slab((phi, psi), (3.0, 3.0), 1.0, n * d);
        let cut = |d: f64| topo::subtract(&cyl, &slab(d), tol());
        let kept = |d: f64, label: &str| {
            let r = cut(d).unwrap_or_else(|err| {
                panic!(
                    "{label}: refused: {err_at}",
                    err_at = err.render(geom_brep::recourse::Reading::Build)
                )
            });
            r.body().map_or(0.0, |b| volume(&b.body))
        };
        let below = {
            let plane = topo::test_support::split_plane(v, n, tol());
            let halves = split(&cyl, &plane, tol()).unwrap();
            halves.below.body().map_or(0.0, |b| volume(b))
        };
        let base = kept(0.0, &format!("ψ {name} on the plane"));
        assert!(
            (base - below).abs() <= 1e-6,
            "ψ {name}: the difference keeps the split's lower half: {base} against {below}"
        );
        let mut rows: Vec<f64> = offsets(psi).into_iter().map(|m| m * e).collect();
        if name == "3°" {
            rows.extend([0.9 * e, -0.9 * e]);
        }
        for d in rows {
            let label = format!("ψ {name} δ {:+.3}ε", d / e);
            match reading(d, psi) {
                Reading::On => {
                    let got = kept(d, &label);
                    assert!((got - base).abs() <= 1e-6, "{label}: {got} against {base}");
                }
                // Where the offset alone is in the band, an edge along the
                // plane is read first (the cap's rim at 90°, an edge on
                // the plane); that is not this reading.
                Reading::Band {
                    offset_in_band: true,
                } => {}
                Reading::Band {
                    offset_in_band: false,
                } => {
                    let Err(BooleanError::Escalated {
                        decision: BooleanDecision::Coincidence(Coincide::VertexOnSection, _),
                        diag,
                    }) = cut(d)
                    else {
                        panic!("{label}: the vertex refuses typed on its section");
                    };
                    let (predicate, got) = reported(&diag);
                    assert_eq!(predicate, "vertex_section_side", "{label}");
                    assert_reads(&label, got, d / psi.sin(), psi.sin());
                }
                Reading::Off => {
                    // Off the cap, the slab's face meets the wall in a
                    // pairing the curved arm has no lane for; into it, the
                    // rim's touch leaves two ends unpaired, as at the split.
                    let r = cut(d);
                    let want = if d > 0.0 {
                        matches!(r, Err(BooleanError::CurvedBooleanUnsupported { .. }))
                    } else {
                        matches!(
                            &r,
                            Err(BooleanError::Join(join))
                                if matches!(join.error(), SplitJoinError::UnpairedLooseEnds { count: 2 })
                        )
                    };
                    assert!(want, "{label}: {:?}", r.err());
                }
            }
        }
    }
}

/// The plane exactly through the seam vertex of the cylinder turned
/// `φ = 2.2`, at dihedral `ψ`, its stored origin `l` along the rim's
/// tangent, in the plane: the vertex's offset is the evaluation's
/// rounding alone, a few ulps of `l`.
fn through_the_seam(psi: f64, l: f64) -> Result<topo::splitting::SplitResult<f64>, SplitError> {
    let phi = 2.2;
    let (cyl, v, n) = pose(phi, psi);
    let along = Vec3::new(-phi.sin(), phi.cos(), 0.0);
    let plane = topo::test_support::split_plane(v + along * l, n, tol());
    split(&cyl, &plane, tol())
}

/// Whether `r` refuses on the section reading, at either door and in
/// whatever shape the refusal takes: the one ending this family must not
/// reach, since its reading is the rounding's and not the input's.
fn offers_the_section_lever<B, E: core::fmt::Debug>(r: &Result<B, E>) -> bool {
    r.as_ref()
        .err()
        .is_some_and(|e| format!("{e:?}").contains("\"vertex_section_side\""))
}

/// **A reading within the kernel's own resolution keeps the offset's
/// verdict.** On the plane through the seam vertex, a few kilometres of
/// stored origin put the offset's rounding, over `sin ψ`, past `ε/K`:
/// the section reading is not admitted, and the vertex reads ON by its
/// offset as it always did. No row refuses at the vertex; every body the
/// split returns is valid; and at the default band the plane at
/// `ψ = 3·10⁻⁴` builds at every origin.
#[test]
fn an_unresolved_section_reading_keeps_the_offsets_verdict() {
    let family: [(f64, &[f64]); 3] = [
        (
            3e-4,
            &[1100.0, 1350.0, 1600.0, 1850.0, 2100.0, 2350.0, 2600.0],
        ),
        (
            1e-4,
            &[1100.0, 1350.0, 1600.0, 1850.0, 2100.0, 2350.0, 2600.0],
        ),
        (
            1e-5,
            &[100.0, 125.0, 150.0, 175.0, 200.0, 225.0, 250.0, 270.0],
        ),
    ];
    let default_band = tol().eps() == geom_core::tolerance::DEFAULT_EPS;
    for (psi, lengths) in family {
        for &l in lengths {
            let label = format!("ψ {psi:e} L {l}");
            let r = through_the_seam(psi, l);
            assert!(!offers_the_section_lever(&r), "{label}: {:?}", r.err());
            match &r {
                Ok(halves) => {
                    for part in [halves.above.body(), halves.below.body()]
                        .into_iter()
                        .flatten()
                    {
                        assert!(
                            topo::validate_geometric(part, tol()).is_ok(),
                            "{label}: an invalid body"
                        );
                    }
                }
                Err(err) => assert!(
                    !(default_band && psi == 3e-4),
                    "{label}: the default band builds here: {err:?}"
                ),
            }
        }
    }
}

/// **A reading through a vanishing sine offers no lever.** At
/// `ψ = 0.1·K·u/ε` the rounding of an offset a few ulps of a metre
/// lands, over `sin ψ`, about the band by the rounding's choice: no row
/// refuses at the vertex on it, and every side a row returns passes
/// tier 3 (the split refuses the rest at its at-rest gate,
/// `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body`).
#[test]
fn a_reading_through_a_vanishing_sine_offers_no_lever() {
    let (e, k) = (tol().eps(), tol().k());
    let psi = (0.1 * k * geom_core::UNIT_ROUNDOFF / e).asin();
    for i in 1..=25 {
        let l = 0.1 * f64::from(i);
        let r = through_the_seam(psi, l);
        assert!(
            !offers_the_section_lever(&r),
            "ψ {psi:e} L {l}: the kernel's rounding offered as the user's lever: {:?}",
            r.as_ref().err()
        );
        if let Ok(halves) = &r {
            for part in [halves.above.body(), halves.below.body()]
                .into_iter()
                .flatten()
            {
                assert!(
                    topo::validate_geometric(part, tol()).is_ok(),
                    "ψ {psi:e} L {l}: a side returned that tier 3 refuses"
                );
            }
        }
    }
}

/// The unit cylinder's wall (`0 ≤ y ≤ 1`) under a cone wall rising to
/// `(0.5, 1.5)`, revolved about `y`: its rim vertex `(1, 1, 0)`.
fn capped_cylinder() -> AtRestBody<f64> {
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.5, 1.5),
        Point2::new(0.0, 1.5),
    ]);
    let body = revolve(
        &validated(vec![meridian]),
        axis_y(),
        Revolution::Full,
        tol(),
    )
    .expect("the meridian revolves")
    .body;
    finished(body)
}

/// **A plane touching the solid only at a vertex reads no section
/// there.** Through the rim vertex, its normal `(cos θ, sin θ, 0)` leans
/// above every face at the vertex for `θ` under 45°, so every edge leaves
/// the vertex below it and no section enters a face: the vertex reads its
/// offset alone, and the split lands the body whole below, at every
/// offset within the zero band.
#[test]
fn a_supporting_plane_through_a_rim_vertex_builds() {
    let e = tol().eps();
    let body = capped_cylinder();
    let v = Point3::new(1.0, 1.0, 0.0);
    for theta in [5f64.to_radians(), 30f64.to_radians()] {
        let n = Vec3::new(theta.cos(), theta.sin(), 0.0);
        let below = |d: f64| {
            let label = format!("θ {:.0}° δ {:+.1}ε", theta.to_degrees(), d / e);
            let plane = topo::test_support::split_plane(v + n * d, n, tol());
            let r = split(&body, &plane, tol())
                .unwrap_or_else(|err| panic!("{label}: refused: {err:?}"));
            assert!(r.above.body().is_none(), "{label}: nothing lands above");
            volume(r.below.body().expect("the body lands below"))
        };
        let base = below(0.0);
        for c in [0.3, 0.5, 0.9] {
            for d in [c * e, -c * e] {
                let got = below(d);
                assert!(
                    (got - base).abs() <= 1e-9 * base,
                    "θ {theta}: δ {d:e}: {got} against {base}"
                );
            }
        }
    }
}

/// **The shallowest face a section enters decides.** A cone wall 2° off
/// the axis below the rim vertex `(1, 1, 0)` and one 5° off above it,
/// cut by a plane through the vertex tilted `β = 2°` off the meridian
/// plane, so the trace enters both faces: at `8°` the lower face meets
/// it at a dihedral of about 10°, the upper at about 3.6°. At `δ =
/// 0.12ε` the steeper reading is within the zero band and the shallower
/// one is in the band; the vertex refuses on the shallower.
#[test]
fn the_shallowest_entered_face_decides() {
    let e = tol().eps();
    let (lo, hi) = (2f64.to_radians(), 5f64.to_radians());
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0 - lo.tan(), 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0 - hi.tan(), 2.0),
        Point2::new(0.0, 2.0),
    ]);
    let body = finished(
        revolve(
            &validated(vec![meridian]),
            axis_y(),
            Revolution::Full,
            tol(),
        )
        .expect("the meridian revolves")
        .body,
    );
    let (tilt, beta) = (8f64.to_radians(), 2f64.to_radians());
    let n = Vec3::new(tilt.cos() * beta.cos(), tilt.sin() * beta.cos(), beta.sin());
    let sine = |wall: Vec3<f64>| n.cross(wall).norm();
    let lower = sine(Vec3::new(lo.cos(), -lo.sin(), 0.0));
    let upper = sine(Vec3::new(hi.cos(), hi.sin(), 0.0));
    let shallow = lower.min(upper);
    let d = 0.12 * e;
    let (steep, shallowest) = (d / lower.max(upper), d / shallow);
    assert!(
        steep <= 0.9 * e && shallowest >= 1.5 * e && shallowest * 1.5 <= tol().k() * e,
        "the pose parts the two readings: {steep:e} and {shallowest:e}"
    );
    let v = Point3::new(1.0, 1.0, 0.0);
    let plane = topo::test_support::split_plane(v + n * d, n, tol());
    let Err(SplitError::Reduce(SplitReduceError::SliverVertex { diag, .. })) =
        split(&body, &plane, tol())
    else {
        panic!("the vertex refuses on the shallower face");
    };
    let (predicate, got) = reported(&diag);
    assert_eq!(predicate, "vertex_section_side");
    assert_reads("the shallower face", got, -d / shallow, shallow);
}

/// **The Boolean's twin of the supporting plane**: the capped cylinder
/// less a slab whose face is the supporting plane through the rim
/// vertex. The slab meets the solid only at the vertex, so the
/// difference keeps the whole solid at every offset within the zero
/// band.
#[test]
fn a_supporting_slab_at_a_rim_vertex_keeps_the_solid() {
    let e = tol().eps();
    let body = capped_cylinder();
    let whole = volume(&body);
    let v = Point3::new(1.0, 1.0, 0.0);
    for theta in [5f64.to_radians(), 30f64.to_radians()] {
        let n = Vec3::new(theta.cos(), theta.sin(), 0.0);
        // The slab `x ≥ 1` about the vertex, its face's outward normal
        // `−x` turned onto `−n` about the axis through the vertex.
        let slab = |d: f64| {
            let slab = finished(sweep::test_support::prism_at(
                [(1.0, -3.0), (4.0, -3.0), (4.0, 3.0), (1.0, 3.0)]
                    .iter()
                    .map(|&(x, y)| (Point2::new(x, y), 0.0))
                    .collect(),
                -3.0,
                6.0,
                tol(),
            ));
            let slab = moved(
                &slab,
                Affine3::rotation_about_axis(v, Vec3::new(0.0, 0.0, 1.0), theta),
            );
            moved(&slab, Affine3::translation(n * d))
        };
        for c in [0.0, 0.3, -0.3, 0.5, -0.5, 0.9, -0.9] {
            let label = format!("θ {:.0}° δ {c:+.1}ε", theta.to_degrees());
            let r = topo::subtract(&body, &slab(c * e), tol())
                .unwrap_or_else(|err| panic!("{label}: refused: {err:?}"));
            let got = r.body().map_or(0.0, |b| volume(&b.body));
            assert!(
                (got - whole).abs() <= 1e-9 * whole,
                "{label}: {got} against {whole}"
            );
        }
    }
}

/// `work`'s result, and whether it read a vertex's section reading its
/// rounding does not admit (decided `vertex_section_clear`).
fn reading_unadmitted<R>(work: impl FnOnce() -> R) -> (R, bool) {
    let bracket = geom_core::k_stats::Bracket::open();
    let r = work();
    let recorded = bracket.finish();
    let clear = "vertex_section_clear";
    let read = recorded.verdicts.iter().any(|v| v.predicate == clear)
        || recorded.escalations.iter().any(|e| e.predicate() == clear);
    (r, read)
}

/// **A reading its own rounding does not admit offers no lever, at
/// every band and at both doors.** The plane meets the seam vertex at
/// dihedral `ψ = 4uL/ε`, its stored origin `L` along the rim's tangent:
/// the offset's rounding, about `1.9uL`, lands over `sin ψ` near `ε/2`,
/// past the `ε/K` that admits the reading. The plane is set `√K·ε·sin ψ`
/// off the vertex either way, so the reading itself lies mid-band, where
/// an admitted one would refuse with the section's lever. The seam's
/// ruling must leave the plane's band over the cylinder's height,
/// `ψ·h ≥ 6Kε`, which at a coarse band takes a tall cylinder and a far
/// origin (`h = L`). No row refuses on the section reading; what the
/// join then does with the vertex the offset puts on the plane is its
/// own gate's. Each row first reaches that reading: it decides
/// `vertex_section_clear`, which only an unadmitted reading is read
/// under, so an earlier refusal cannot pass it.
#[test]
fn an_unadmitted_section_reading_offers_no_lever_at_either_door() {
    let (e, k) = (tol().eps(), tol().k());
    let u = geom_core::UNIT_ROUNDOFF;
    let phi = 2.2;
    assert!(k >= 4.0, "the rows need the charge near ε/2 past ε/K");
    let reach = (6.0 * k * e * e / (4.0 * u)).sqrt().max(1.0);
    assert!(
        reach <= 500.0,
        "the fixture stays in the session box: {reach}"
    );
    let psi = (4.0 * u * reach / e).asin();
    let (cyl, v, n) = pose_of_height(phi, psi, reach);
    let along = Vec3::new(-phi.sin(), phi.cos(), 0.0);
    for side in [1.0, -1.0] {
        let d = side * k.sqrt() * e * psi.sin();
        let label = format!("ψ {psi:e} L {reach} δ {d:+e}");
        let plane = topo::test_support::split_plane(v + along * reach + n * d, n, tol());
        let (r, unadmitted) = reading_unadmitted(|| split(&cyl, &plane, tol()));
        assert!(
            unadmitted,
            "{label}: the split reads the section unadmitted"
        );
        assert!(
            !offers_the_section_lever(&r),
            "{label}: the split: {:?}",
            r.err()
        );
        let slab = seam_slab((phi, psi), (3.0, reach), reach, n * d);
        let (r, unadmitted) = reading_unadmitted(|| topo::subtract(&cyl, &slab, tol()));
        assert!(
            unadmitted,
            "{label}: the Boolean reads the section unadmitted"
        );
        assert!(
            !offers_the_section_lever(&r),
            "{label}: the Boolean: {:?}",
            r.err()
        );
    }
}

/// The torus of radii `(2, 1)` about `y`: its seam vertex on the inner
/// equator, `(1, 0, 0)`, is a saddle of the wall.
fn torus() -> AtRestBody<f64> {
    let profile = profile::Profile::new(
        profile::SketchPlane::<f64>::xy(),
        vec![profile::test_support::bulge_loop(vec![
            (Point2::new(1.0, 0.0), 1.0),
            (Point2::new(3.0, 0.0), 1.0),
        ])],
    )
    .validate(tol())
    .unwrap();
    finished(
        revolve(&profile, axis_y(), Revolution::Full, tol())
            .unwrap()
            .body,
    )
}

/// **A sine within its own rounding of zero reads no side.** The slab
/// `x ≥ 1` is tangent to the torus at its saddle vertex `(1, 0, 0)`,
/// where its plane cuts the wall in two branches crossing at the vertex
/// and the section's sine reads `0 ± 8.9e-16`. Turned `θ` about the
/// vertex and moved `δ` off it, both far below the band, the sine reads
/// `θ` inside that bound, and a quotient by it has no bound to give:
/// the vertex keeps its offset's verdict, and the Boolean ends where the
/// untilted slab does, at the torus's frontier, not on a side the
/// division made up (it read `±13 m` off).
#[test]
fn a_slab_tangent_at_a_torus_saddle_reads_no_side_off_its_sine() {
    let torus = torus();
    let v = Point3::new(1.0, 0.0, 0.0);
    for (theta, d) in [(0.0, 0.0), (2e-16, 1e-15), (4e-16, -2e-15)] {
        let slab = finished(sweep::test_support::prism_at(
            [(1.0, -3.0), (4.0, -3.0), (4.0, 3.0), (1.0, 3.0)]
                .iter()
                .map(|&(x, y)| (Point2::new(x, y), 0.0))
                .collect(),
            -3.0,
            6.0,
            tol(),
        ));
        let slab = moved(
            &slab,
            Affine3::rotation_about_axis(v, Vec3::new(0.0, 0.0, 1.0), theta),
        );
        let slab = moved(&slab, Affine3::translation(Vec3::new(d, 0.0, 0.0)));
        let r = topo::subtract(&torus, &slab, tol());
        assert!(
            matches!(
                r,
                Err(BooleanError::CurvedBooleanUnsupported {
                    kind: geom::SurfaceKind::Torus,
                    ..
                })
            ),
            "θ {theta:e} δ {d:e}: {:?}",
            r.err()
        );
    }
}
