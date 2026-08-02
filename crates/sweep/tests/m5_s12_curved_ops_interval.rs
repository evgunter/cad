//! M5 S12 interval lane: curved `revert` and the newly-live curved
//! ∖/∩ at the CERTIFIED scalar (feature `interval`).
//!
//! Everything S12 adds is exact structure — a `bool` negation, a
//! surface-KEY equality, an arena scan of surface kinds — so none of it
//! can widen and none of it can escalate. That is precisely what makes
//! this lane worth running: it pins that the flip and the inheritance
//! are STILL bitwise at a scalar that widens everything metric around
//! them, and that the ops they unblock decide DEFINITELY from honest
//! enclosures rather than landing on an escalation.
//!
//! Rows: the involution and determinism rows bitwise on a curved
//! `Body<Interval>`; the blind hole and its ∩ twin with certified
//! volume enclosures containing the closed forms; the mixed-sense split
//! with the inherited bit read back; and the per-class door refusing
//! structurally (no band quoted — it never compares anything).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Bounds, Interval, Point2, Real, Tolerance, Vec2, Vec3};
use geom_surfaces::Surface;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, BooleanError, mass_properties};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tolerance::get())
        .unwrap()
}

const R: f64 = 0.35;

fn plate() -> Body<Interval> {
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    extrude(&validated(vec![lp]), Extrusion::Distance(iv(0.8)))
        .unwrap()
        .body
}

/// The three-arc cylindrical boss at (1.2, 1.7), sketched at `z0`.
fn boss(z0: f64, len: f64) -> Body<Interval> {
    let theta = 2.0 * PI / 3.0;
    let bulge = iv((theta / 4.0).tan());
    let at = |i: usize| {
        let th = theta * i as f64;
        p2(1.2 + R * th.cos(), 1.7 + R * th.sin())
    };
    let lp = ProfileLoop::builder(at(0))
        .arc_to(at(1), bulge)
        .arc_to(at(2), bulge)
        .close_with_bulge(bulge);
    let plane = SketchPlane::from_frame(
        geom_core::Point3::new(iv(0.0), iv(0.0), iv(z0)),
        Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
        Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&vp, Extrusion::Distance(iv(len))).unwrap().body
}

/// A 3 × 3 × 1 plate with a concave semicircular notch on its `x = 3`
/// wall — S11's `sense: false` arc wall at the certified scalar.
fn notched() -> Body<Interval> {
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .arc_to(p2(3.0, 2.0), iv(-1.0))
        .line_to(p2(3.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    extrude(&validated(vec![lp]), Extrusion::Distance(iv(1.0)))
        .unwrap()
        .body
}

fn encloses(vol: Interval, analytic: f64, what: &str) {
    assert!(
        vol.lo() <= analytic && analytic <= vol.hi(),
        "{what}: enclosure [{}, {}] must contain {analytic}",
        vol.lo(),
        vol.hi()
    );
    assert!(
        vol.hi() - vol.lo() <= 1e-9,
        "{what}: enclosure stays tight, got width {}",
        vol.hi() - vol.lo()
    );
}

/// The flip is bitwise at the certified scalar: nothing widens, the
/// involution is exact, and the chart is not perturbed (a widened
/// sphere or cylinder chart would show up immediately here).
#[test]
fn interval_curved_revert_is_bitwise() {
    for body in [boss(0.0, 1.0), notched()] {
        let original = format!("{body:?}");
        let rev = body.revert().unwrap();
        assert_eq!(
            format!("{:?}", rev.revert().unwrap()),
            original,
            "involution"
        );
        assert_eq!(
            format!("{:?}", body.revert().unwrap()),
            format!("{rev:?}"),
            "determinism"
        );
        for (k, f) in body.faces() {
            let curved = !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. }));
            let now = rev.get_face(k).unwrap().sense;
            assert_eq!(now, if curved { !f.sense } else { f.sense });
            if curved {
                assert_eq!(
                    format!("{:?}", rev.get_surface(f.surface).unwrap()),
                    format!("{:?}", body.get_surface(f.surface).unwrap()),
                    "a chart must not widen under revert"
                );
            }
        }
    }
}

/// The blind hole and its ∩ twin decide DEFINITELY at the certified
/// scalar, with volume enclosures containing the closed forms.
#[test]
fn interval_curved_subtract_and_intersect_decide_definitely() {
    let a = plate();
    let b = boss(0.3, 1.0);
    let cut = topo::subtract(&a, &b).expect("curved subtract decides at Interval");
    let cut = &cut.body().expect("a body").body;
    assert_eq!(topo::validate_geometric(cut), Ok(()));
    encloses(
        mass_properties(cut).unwrap().volume,
        3.0 * 3.0 * 0.8 - PI * R * R * 0.5,
        "blind hole",
    );

    let met = topo::intersect(&a, &b).expect("curved intersect decides at Interval");
    let met = &met.body().expect("a body").body;
    assert_eq!(topo::validate_geometric(met), Ok(()));
    encloses(
        mass_properties(met).unwrap().volume,
        PI * R * R * 0.5,
        "the plug",
    );
}

/// The mixed-sense split at the certified scalar: the fragments of the
/// reversed wall still come back `sense: false`, and the volume
/// enclosure still contains the exact closed form.
#[test]
fn interval_split_of_a_reversed_wall_inherits_the_bit() {
    let a = notched();
    let lp = ProfileLoop::builder(p2(2.0, 0.5))
        .line_to(p2(4.0, 0.5))
        .line_to(p2(4.0, 2.5))
        .line_to(p2(2.0, 2.5))
        .close();
    let plane = SketchPlane::from_frame(
        geom_core::Point3::new(iv(0.0), iv(0.0), iv(0.3)),
        Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
        Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let b = extrude(&vp, Extrusion::Distance(iv(0.4))).unwrap().body;

    let notch = PI * 0.25 / 2.0;
    let out = topo::intersect(&a, &b).expect("the split decides at Interval");
    let out = &out.body().expect("a body").body;
    assert_eq!(topo::validate_geometric(out), Ok(()));
    encloses(
        mass_properties(out).unwrap().volume,
        (2.0 - notch) * 0.4,
        "the meet across the reversed wall",
    );
    let reversed = out
        .faces()
        .filter(|(_, f)| !matches!(out.get_surface(f.surface), Some(Surface::Plane { .. })))
        .filter(|(_, f)| !f.sense)
        .count();
    assert!(
        reversed > 0,
        "the mef re-mint did not inherit the parent bit at Interval"
    );
}

/// The per-class door is STRUCTURAL, and this lane proves it: at a
/// scalar where every metric comparison could widen into an
/// escalation, the sphere class still refuses the same typed variant,
/// and the message quotes no band because it compares nothing.
#[test]
fn interval_per_class_door_is_structural() {
    let lp = ProfileLoop::builder(p2(0.0, -1.0))
        .arc_to(p2(0.0, 1.0), iv(1.0))
        .close();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(iv(0.0), iv(1.0)),
    };
    let ball = revolve(&validated(vec![lp]), axis, Revolution::Full)
        .unwrap()
        .body;
    let ball = topo::transform_rigid(
        &ball,
        &Affine3::translation(Vec3::new(iv(1.5), iv(1.5), iv(0.5))),
    )
    .unwrap();

    let err = topo::subtract(&plate(), &ball).expect_err("the sphere class has no join lane");
    let BooleanError::CurvedOpUnsupported { .. } = err else {
        panic!("expected the per-class door, got {err:?}");
    };
    let msg = err.to_string();
    assert!(msg.contains("no seam lane"), "{msg}");
    assert!(
        !msg.contains("escalate"),
        "a structural door quotes no band: {msg}"
    );
    // And the live class still decides at this scalar (the door is per
    // class, not a blanket restored).
    assert!(topo::subtract(&plate(), &boss(0.3, 1.0)).is_ok());
}
