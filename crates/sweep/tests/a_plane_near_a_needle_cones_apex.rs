//! **A plane near a needle cone's apex, nearly along its axis.** The
//! cone: profile `(0, 0), (tan α, 0), (0, 1)` revolved whole about `y`,
//! apex `(0, 1, 0)`, at α = 1° and 3°. The plane passes through the point
//! `kε` down the seam ruling from the apex, its normal tilted `tilt` off
//! the axis toward azimuth `spin`; at the tilts here it is a few degrees
//! off the axis and cuts the tip off along a long thin ellipse.
//!
//! **Its closed form.** The tip is the cone over that ellipse, so with
//! `h` the apex's distance from the plane, `c = cos tilt` and
//! `D = c² − tan²α·(1 − c²)`, the ellipse's area is `π tan²α h² / D^{3/2}`
//! and the tip's volume `π tan²α h³ / (3 D^{3/2})`; the other side is the
//! rest of the cone, `π tan²α / 3` whole.
//!
//! **In the band.** At `k = 1000` the cone's radius where the plane meets
//! the seam is `1000ε·sin α`, 17ε at 1° and 52ε at 3°, and a wall normal
//! read there moves by `ε/ρ`, 3.3° and 1.1° — as much as the 3.1° and
//! 1.1° the plane makes with the wall at `tilt = 1.5`. Every margin of
//! the pose is a fixed multiple of ε: the refusals pinned here were
//! measured the same at ε 1e-6, 1e-9 and 1e-12, each typed with its
//! lever, none naming a broken invariant. At `k = 10⁵` the same planes
//! land their closed forms.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::PI;

use crate::revolve_common::needle;
use geom_brep::{CertCheck, CertifyError};
use geom_core::{Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitJoinError, split};
use topo::{CensusUnsupportedCause, ContactRecords, ContactRefusal, EulerOpError, ValidationError};

/// The pose's plane, and the tip's volume above it (the apex's side).
fn pose(deg: f64, k: f64, tilt: f64, spin: f64, tol: Tol) -> (topo::SplitPlane<f64>, f64) {
    let a = deg.to_radians();
    let apex = Point3::new(0.0, 1.0, 0.0);
    let down_seam = Vec3::new(a.sin(), -a.cos(), 0.0);
    let n = Vec3::new(tilt.sin() * spin.cos(), tilt.cos(), tilt.sin() * spin.sin());
    let h = -n.dot(down_seam) * k * tol.eps();
    assert!(h > 0.0, "the apex lies above the plane");
    let t2 = a.tan().powi(2);
    let c2 = tilt.cos().powi(2);
    let d = c2 - t2 * (1.0 - c2);
    assert!(d > 0.0, "the section is an ellipse");
    let tip = PI * t2 * h.powi(3) / (3.0 * d.powf(1.5));
    let plane = topo::test_support::split_plane(apex + down_seam * (k * tol.eps()), n, tol);
    (plane, tip)
}

/// **In the band of the apex, each pose refuses typed, with its lever.**
/// At `tilt = 1.5`, `spin = 0` the chord's start reads the plane within
/// the zero band of tangent to the wall, its datum along the section; at
/// `tilt = 1.2`, `spin = 2` on the 1° cone the chord's certificate reads
/// its crossing angle inside the band. Neither is a broken invariant.
#[test]
fn a_needle_cone_cut_in_band_of_its_apex_refuses_with_its_lever() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for deg in [1.0, 3.0] {
        let what = format!("α = {deg}°, tilt 1.5, spin 0");
        let (plane, _) = pose(deg, 1e3, 1.5, 0.0, tol);
        let err = split(&needle(deg, None, tol), &plane, tol).expect_err(&what);
        assert!(
            matches!(
                err,
                SplitError::Join(SplitJoinError::SectionDepartureGrazes { .. })
            ),
            "{what}: {err:?}"
        );
        let text = err.to_string();
        for (part, words) in [
            ("the decision that ran", "'chord_arc_leave_section'"),
            ("the lever", "farther from its apex"),
        ] {
            assert!(text.contains(words), "{what}: names {part}: {text}");
        }
        assert!(!text.contains("paired"), "{what}: pairs nothing: {text}");
    }
    let what = "α = 1°, tilt 1.2, spin 2";
    let (plane, _) = pose(1.0, 1e3, 1.2, 2.0, tol);
    let err = split(&needle(1.0, None, tol), &plane, tol).expect_err(what);
    let SplitError::Join(SplitJoinError::Euler(EulerOpError::Certification {
        error:
            CertifyError::Escalated {
                check: CertCheck::Transversality,
                cause,
                ..
            },
    })) = &err
    else {
        panic!("{what}: {err:?}");
    };
    let margin = cause
        .margin
        .diagnostic_f64_for_error_text()
        .value()
        .expect("a point margin")
        .abs();
    assert!(
        (eps..10.0 * eps).contains(&margin),
        "{what}: the crossing angle's margin {margin:e} is in the band"
    );
    assert!(
        err.to_string().contains("tighten the tolerance"),
        "{what}: {err}"
    );
}

/// **Off the apex the same planes land their closed forms**, both sides,
/// each passing tiers 3 and 3′ — the refusals' lever (move the geometry)
/// taken: the plane meets the seam a hundred times farther down. Where
/// the ellipse's vertices sit on the seams (`spin = 0`) at `tilt = 1.5`,
/// tier 3′ refuses as measured, a census residue filed on its own row.
#[test]
fn a_needle_cone_cut_off_its_apex_lands_its_closed_forms() {
    let tol = Tol::witness();
    for deg in [1.0f64, 3.0] {
        let body = needle(deg, None, tol);
        let whole = PI * deg.to_radians().tan().powi(2) / 3.0;
        for (tilt, spin) in [(1.2, 0.0), (1.2, 2.0), (1.5, 0.0), (1.5, 2.0)] {
            let what = format!("α = {deg}°, tilt {tilt}, spin {spin}");
            let (plane, tip) = pose(deg, 1e5, tilt, spin, tol);
            let r = split(&body, &plane, tol).unwrap_or_else(|e| panic!("{what}: {e:?}"));
            for (side, part, want) in [("above", &r.above, tip), ("below", &r.below, whole - tip)] {
                let what = format!("{what}, {side}");
                let b = part
                    .body()
                    .unwrap_or_else(|| panic!("{what}: empty where {want:e} is"));
                let got = topo::test_support::pinned_volume(&what, b, tol).volume;
                assert!(
                    (got - want).abs() <= 1e-6 * want,
                    "{what}: volume {got:e}, closed form {want:e}"
                );
                assert_eq!(topo::validate_geometric(b, tol), Ok(()), "{what}: tier 3");
                let t3p = topo::validate_pseudomanifold(b, &ContactRecords::default(), tol);
                if spin == 0.0 && tilt == 1.5 {
                    // The ellipse's vertices on the seams: the census
                    // cannot settle a ruling against the half ending on
                    // it, filed as
                    // work/splitdoor/tier-3-prime-cannot-settle-a-ruling-against-an-ellipse-it-ends-at-on-a-needle-cone.md
                    let errs = t3p.expect_err(&what);
                    assert!(
                        errs.iter().all(|e| matches!(
                            e,
                            ValidationError::CensusUnsupported {
                                cause: CensusUnsupportedCause::ContactLane(
                                    ContactRefusal::NotCertifiable { what }
                                ),
                                ..
                            } if what.contains("could not settle a count")
                        )),
                        "{what}: tier 3′ as measured: {errs:?}"
                    );
                } else {
                    assert_eq!(t3p, Ok(()), "{what}: tier 3′");
                }
            }
        }
    }
}
