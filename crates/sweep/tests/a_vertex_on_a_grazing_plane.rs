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

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitJoinError, split};
use topo::{
    AtRestBody, Body, BooleanDecision, BooleanError, Coincide, EulerOpError, SplitReduceError,
    mass_properties,
};

fn tol() -> Tol {
    Tol::witness()
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn moved(body: &AtRestBody<f64>, by: Affine3<f64>) -> AtRestBody<f64> {
    finished(topo::transform_rigid(body, &by, tol()).unwrap())
}

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, tol()).unwrap().volume
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
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let cyl = finished(sweep::test_support::prism(circle, 1.0, tol()));
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
                let r = cut(d).unwrap_or_else(|err| panic!("{label}: refused: {err}"));
                let vol = |b: Option<&Body<f64>>| b.map_or(0.0, volume);
                (vol(r.above.body()), vol(r.below.body()))
            };
            let base = parts(0.0, &format!("φ {phi} ψ {name} on the plane"));
            assert!(
                (base.0 + base.1 - core::f64::consts::PI).abs() <= 1e-6,
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
                            (got.0 - base.0).abs() <= 1e-6 && (got.1 - base.1).abs() <= 1e-6,
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
                                .to_string()
                                .contains("move the split plane"),
                            "{label}: the refusal names its lever"
                        );
                    }
                    Reading::Off => {
                        // Moved off the cap, the plane cuts the wall clear
                        // of the vertex. Moved into it, the rim grazes the
                        // plane over a stretch the root lane reads as a
                        // touch (filed on the CLEAVE slate).
                        let r = cut(d);
                        if d > 0.0 {
                            let got = parts(d, &label);
                            assert!(
                                (got.0 - base.0).abs() <= 1e-6 && (got.1 - base.1).abs() <= 1e-6,
                                "{label}: {got:?} against {base:?} on the plane"
                            );
                        }
                        assert!(
                            !matches!(
                                r,
                                Err(SplitError::Join(SplitJoinError::Euler(
                                    EulerOpError::Certification { .. }
                                )))
                            ),
                            "{label}: the miss is decided before the join's certificate: {:?}",
                            r.err()
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn the_boolean_reads_a_grazing_vertex_against_its_section() {
    let e = tol().eps();
    let phi = 2.2;
    for (name, psi) in ladder().into_iter().skip(1) {
        let (cyl, v, n) = pose(phi, psi);
        // The slab x ≥ 1 about the seam vertex of the unturned cylinder,
        // its face's outward normal −x turned onto −n, moved δ along n.
        let slab = |d: f64| {
            let slab = sweep::test_support::prism_at(
                [(1.0, -3.0), (4.0, -3.0), (4.0, 3.0), (1.0, 3.0)]
                    .iter()
                    .map(|&(x, y)| (Point2::new(x, y), 0.0))
                    .collect(),
                -3.0,
                6.0,
                tol(),
            );
            let slab = finished(slab);
            let slab = moved(
                &slab,
                Affine3::rotation_about_axis(
                    Point3::new(1.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    -psi,
                ),
            );
            let slab = moved(
                &slab,
                Affine3::rotation_about_axis(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    phi,
                ),
            );
            moved(&slab, Affine3::translation(n * d))
        };
        let cut = |d: f64| topo::subtract(&cyl, &slab(d), tol());
        let kept = |d: f64, label: &str| {
            let r = cut(d).unwrap_or_else(|err| panic!("{label}: refused: {err}"));
            r.body().map_or(0.0, |b| volume(&b.body))
        };
        let below = {
            let plane = topo::test_support::split_plane(v, n, tol());
            let halves = split(&cyl, &plane, tol()).unwrap();
            volume(halves.below.body().unwrap())
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
                // Where the offset alone is in the band, an edge on the
                // plane is read first; that is not this reading.
                Reading::Band {
                    offset_in_band: true,
                } => {}
                Reading::Band {
                    offset_in_band: false,
                } => {
                    let Err(BooleanError::Escalated {
                        decision: BooleanDecision::Coincidence(Coincide::VertexOnFace, _),
                        diag,
                    }) = cut(d)
                    else {
                        panic!("{label}: the vertex refuses typed on the face");
                    };
                    let (predicate, got) = reported(&diag);
                    assert_eq!(predicate, "vertex_section_side", "{label}");
                    assert_reads(&label, got, d / psi.sin(), psi.sin());
                }
                Reading::Off => {
                    let r = cut(d);
                    assert!(
                        !matches!(
                            r,
                            Err(BooleanError::Join(SplitJoinError::Euler(
                                EulerOpError::Certification { .. }
                            )))
                        ),
                        "{label}: the miss is decided before the join's certificate: {:?}",
                        r.err()
                    );
                }
            }
        }
    }
}
