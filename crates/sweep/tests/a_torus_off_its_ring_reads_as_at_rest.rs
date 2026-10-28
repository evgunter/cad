//! **A torus off its ring convention ends in the same lever at the
//! offset door as at rest.** `topo::replace_face_offset` takes a body it
//! does not validate, so the plane×torus section its planning reads
//! (`geom_brep::route_pose`) is handed whatever torus the body carries:
//! here a half-torus whose wall is re-charted, through the public
//! `set_face_surfaces_describing`, from `R − r` just past the band to
//! just inside it. Both the door and tier 3 refuse the ring half
//! undecided, and both end in its one lever with the tolerance the
//! margin gives.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{TorusConvention, TorusRefusal, TorusVerdict};
use geom_core::{Band, Point2, Tol, Vec2};
use topo::construction::Construct;
use topo::{Body, FaceKey, ReplaceFaceError};

/// The upper half of the torus of radii `(big_r, r)` about the sketch
/// `y`-axis: its wall, and the plane annulus at `y = 0` it stands on.
fn half_torus(big_r: f64, r: f64, tol: Tol) -> (Body<f64>, FaceKey, FaceKey) {
    let profile = profile::Profile::new(
        profile::SketchPlane::<f64>::xy(),
        vec![profile::test_support::bulge_loop(vec![
            (Point2::new(big_r - r, 0.0), 1.0),
            (Point2::new(big_r + r, 0.0), 0.0),
        ])],
    )
    .validate(tol)
    .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = sweep::revolve(&profile, axis, sweep::Revolution::Full, tol)
        .unwrap()
        .body;
    let of = |torus: bool| {
        body.faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface).unwrap(),
                    geom::Surface::Torus { .. }
                ) == torus
            })
            .unwrap()
            .0
    };
    let (wall, cap) = (of(true), of(false));
    (body, wall, cap)
}

#[test]
fn a_ring_half_in_band_reads_as_at_rest() {
    let tol = Tol::witness();
    let e = Band::linear(tol).unwrap().escalate();
    let r = 0.5;
    // Built just past the band, then re-charted to just inside it with
    // the outer equator held, so every edge still certifies on the wall.
    let (mut body, wall, cap) = half_torus(r + 1.02 * e, r, tol);
    let outer = 2.0 * r + 1.02 * e;
    let gap = 0.98 * e;
    let geom::Surface::Torus {
        center,
        axis,
        u_ref,
        ..
    } = *body
        .get_surface(body.get_face(wall).unwrap().surface)
        .unwrap()
    else {
        panic!("the wall is a torus");
    };
    let sense = body.get_face(wall).unwrap().sense;
    let charts = vec![topo::construction::Rechart::new(
        geom::Surface::Torus {
            center,
            axis,
            u_ref,
            major_radius: (outer + gap) / 2.0,
            minor_radius: (outer - gap) / 2.0,
        },
        wall,
        sense,
    )];
    let carried = body.carried_redescriptions(&charts).unwrap();
    body.set_face_surfaces_describing(charts, &carried, tol)
        .expect("every edge certifies on the re-charted wall");

    let at_rest = topo::AtRestBody::validate(body.clone(), tol)
        .expect_err("tier 3 refuses the ring half in band")
        .iter()
        .find(|e| matches!(e, topo::ValidationError::DegenerateTorusEscalated { .. }))
        .expect("tier 3's ring-half escalation")
        .to_string();
    let err = topo::replace_face_offset(&mut body, cap, 0.1, tol)
        .expect_err("the door's section refuses the wall's ring half");
    let ReplaceFaceError::DegenerateTorus {
        refusal:
            TorusRefusal {
                convention: TorusConvention::Ring,
                verdict: TorusVerdict::Undecided(ref diag),
            },
        ..
    } = err
    else {
        panic!("the ring half undecided, typed: {err:?}");
    };
    assert_eq!(diag.predicate, Some("ring_torus_convention"));
    let text = err.to_string();
    let ending = |t: &str| t.split_once("Recourse: ").map(|(_, end)| end.to_owned());
    assert!(
        text.starts_with(
            "whether a torus's tube radius is smaller than its ring radius is undecided"
        ),
        "the half's subject: {text}"
    );
    assert_eq!(
        ending(&text),
        ending(&at_rest),
        "the door ends as tier 3 does on the same body: {text} | {at_rest}"
    );
    assert!(
        text.contains("make the tube radius clearly smaller than the ring radius"),
        "the ring half's lever: {text}"
    );
}
