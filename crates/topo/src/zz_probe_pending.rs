//! Review probe: a closed prism whose six faces are all pending, run
//! through public doors; each door must refuse typed, never panic.
#![allow(clippy::all, clippy::pedantic, clippy::restriction)]
use std::panic::{AssertUnwindSafe, catch_unwind};

use geom_core::{Band, Point3, RigidMotion, Tol, Vec3};

fn run<R: core::fmt::Debug>(name: &str, f: impl FnOnce() -> R) -> bool {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(r) => {
            let s = format!("{r:?}");
            eprintln!("PROBE {name}: ok -> {}", &s[..s.len().min(300)]);
            true
        }
        Err(e) => {
            let msg = e
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            eprintln!("PROBE {name}: PANIC -> {}", &msg[..msg.len().min(400)]);
            false
        }
    }
}

#[test]
fn zz_probe_pending_prism_doors() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let body = crate::fixtures::raw_prism(4, tol).body;
    let mut ok = true;
    ok &= run("validate", || crate::validate::validate(&body));
    ok &= run("validate_closed", || crate::validate::validate_closed(&body));
    ok &= run("validate_geometric", || crate::validate::validate_geometric(&body, tol));
    ok &= run("transform_rigid", || {
        crate::transform::transform_rigid(
            &body,
            &RigidMotion::translation(Vec3::new(1.0, 2.0, 3.0)).unwrap(),
        )
        .map(|b| b.surface_slots().filter(|(_, s)| s.is_pending()).count())
    });
    ok &= run("mirror_body", || {
        let r = crate::mirror::Reflection::across(
            Point3::new(0.1, -0.2, 0.3),
            Vec3::new(1.0, 2.0, 3.0),
            tol,
        )
        .unwrap();
        crate::mirror::mirror_body(&body, &r, tol)
            .map(|b| b.surface_slots().filter(|(_, s)| s.is_pending()).count())
    });
    ok &= run("reverted", || body.reverted().surface_slots().count());
    ok &= run("point_in_solid", || {
        crate::solid_contain::point_in_solid(&body, Point3::new(0.5, 0.5, 0.5), band, tol)
    });
    ok &= run("merge_coplanar_faces", || {
        let mut b = body.clone();
        b.merge_coplanar_faces(tol).map(|_| ())
    });
    ok &= run("closed_form", || {
        crate::props::mass_properties_closed_form(&body, band, tol).map(|m| m.volume)
    });
    for f in body.faces().map(|(k, _)| k).collect::<Vec<_>>() {
        ok &= run("face_carrier_kind", || crate::readback::face_carrier_kind(&body, f));
        ok &= run("face_pose", || crate::readback::face_pose(&body, f).map(|_| ()));
    }
    assert!(ok, "a door panicked on a pending face");
}

#[test]
fn zz_probe_recharting_a_finished_face_to_pending() {
    use crate::construction::{Construct as _, FaceSurface};
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let brick = crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let face = brick.faces().map(|(k, _)| k).next().unwrap();
    let mut b = brick.clone();
    let sense = b.get_face(face).unwrap().sense;
    let r = b.set_face_surface(face, FaceSurface::Pending { sense });
    eprintln!("PROBE set_face_surface(Pending) on a brick face: {r:?}");
    if r.is_ok() {
        let mut ok = true;
        ok &= run("validate_geometric", || crate::validate::validate_geometric(&b, tol));
        ok &= run("point_in_solid", || {
            crate::solid_contain::point_in_solid(&b, Point3::new(0.5, 0.5, 0.5), band, tol)
        });
        ok &= run("transform_rigid", || {
            crate::transform::transform_rigid(
                &b,
                &RigidMotion::translation(Vec3::new(1.0, 2.0, 3.0)).unwrap(),
            )
            .map(|_| ())
        });
        ok &= run("merge_coplanar_faces", || {
            let mut c = b.clone();
            c.merge_coplanar_faces(tol).map(|_| ())
        });
        ok &= run("closed_form", || {
            crate::props::mass_properties_closed_form(&b, band, tol).map(|m| m.volume)
        });
        assert!(ok, "a door panicked on a re-charted pending face");
    }
}

/// The ruling separates "no surface yet" from a corrupt net. Does the
/// cover door's refusal still tell them apart?
#[test]
fn zz_probe_cover_tells_pending_from_poisoned() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let pending = crate::fixtures::raw_prism(4, tol).body;
    let pf = pending.faces().map(|(k, _)| k).next().unwrap();
    let got_pending = format!("{:?}", crate::cover::face_cover(&pending, pf, band));

    let mut poisoned =
        crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let qf = poisoned.faces().map(|(k, _)| k).next().unwrap();
    let key = poisoned.get_face(qf).unwrap().surface;
    *poisoned.surface_mut(key).unwrap() = crate::fixtures::poisoned_net(|_| true);
    let got_poisoned = format!("{:?}", crate::cover::face_cover(&poisoned, qf, band));
    let pis_pending = format!(
        "{:?}",
        crate::solid_contain::point_in_solid(&pending, Point3::new(0.5, 0.5, 0.5), band, tol)
    );
    let pis_poisoned = format!(
        "{:?}",
        crate::solid_contain::point_in_solid(&poisoned, Point3::new(0.5, 0.5, 0.5), band, tol)
    );
    eprintln!("PROBE cover pending : {got_pending}");
    eprintln!("PROBE cover poisoned: {got_poisoned}");
    eprintln!("PROBE pis pending : {pis_pending}");
    eprintln!("PROBE pis poisoned: {pis_poisoned}");
    let strip = |s: &str| s.replace(&format!("{pf:?}"), "F").replace(&format!("{qf:?}"), "F");
    assert_ne!(
        strip(&got_pending),
        strip(&got_poisoned),
        "face_cover answers a pending face exactly as it answers a poisoned net"
    );
}
