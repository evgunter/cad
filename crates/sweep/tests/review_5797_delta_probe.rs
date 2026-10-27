//! Review probe (PR 5797 delta): a declared and a derived circular arc
//! off a sphere face, read at the build door and at rest, at f64 and
//! Interval.


use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, MappedSource, PcurveCertifyError};
use geom_core::{Affine3, Band, Bounds, Decide, Interval, Point2, Point3, Real, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{Revolution, RevolveAxis};
use topo::{AtRestPolicy, Body, CurveGeom, EulerOpError, FaceKey, MevSite, PcurveMintError};

fn hemisphere<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    let quarter = bulge_loop(vec![
        (Point2::new(f(0.0), f(0.0)), f(0.0)),
        (Point2::new(f(1.0), f(0.0)), f((core::f64::consts::PI / 8.0).tan())),
        (Point2::new(f(0.0), f(1.0)), f(0.0)),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![quarter]).validate(Tol::witness()).unwrap();
    let axis = RevolveAxis { origin: Point2::new(f(0.0), f(0.0)), dir: Vec2::new(f(0.0), f(1.0)) };
    sweep::revolve(&profile, axis, Revolution::Full, Tol::witness()).unwrap().body
}

fn face_on<T: Decide>(body: &Body<T>, sphere: bool) -> (FaceKey, geom_brep::SurfaceKey) {
    body.faces()
        .find(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Sphere { .. }) => sphere,
            Some(Surface::Plane { .. }) => !sphere,
            _ => false,
        })
        .map(|(k, f)| (k, f.surface))
        .unwrap()
}

fn dummy<T: Real>() -> MappedCurve<T> {
    MappedCurve::whole(MappedSource::ExtrudedPoint {
        point: Point2::new(T::zero(), T::zero()),
        place: Affine3::identity(),
        vec: Vec3::new(T::zero(), T::zero(), T::one()),
    })
}

fn lever_ok(door: &str, text: &str) {
    println!("DECLARED {door}: {text}");
    assert!(text.contains("Recourse: move the declared curve"), "{door}: {text}");
    for w in ["defect", "repair the body", "loosen", "tighten"] {
        assert!(!text.contains(w), "{door}: {w}: {text}");
    }
}
fn kernel_ok(door: &str, text: &str) {
    println!("DERIVED {door}: {text}");
    assert!(!text.contains("declared curve") && text.contains("defect"), "{door}: {text}");
}

/// (B) a strut arc on the sphere face from its equator corner, lying in
/// the disc cap's plane y = 0: declared (scaffold) or derived (chart of
/// the cap, on which it lies).
fn strut_case<T: Decide + AtRestPolicy + Bounds + geom_core::SpanLocate>(declared: bool) {
    let f = T::from_f64;
    let mut body = hemisphere::<T>();
    let (wall, _) = face_on(&body, true);
    let (_, cap) = face_on(&body, false);
    let wf = body.get_face(wall).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(wf.outer).unwrap().boundary else { panic!() };
    let cycle = body.loop_cycle(first).unwrap();
    let he = *cycle
        .iter()
        .find(|&&he| {
            let v = body.get_half_edge(he).unwrap().start;
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            (p.x.hi() - 1.0).abs() < 1e-9 && p.y.hi().abs() < 1e-9
        })
        .expect("equator corner");
    let r = f(0.3);
    let carrier = Curve3::Circle {
        center: Point3::new(f(0.7), f(0.0), f(0.0)),
        axis: Vec3::new(f(0.0), f(-1.0), f(0.0)),
        radius: r,
        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
    };
    let span = f(0.5);
    let spec = if declared {
        EdgeCurveSpec::arc_of_circle(carrier.clone(), T::zero(), span).unwrap()
    } else {
        EdgeCurveSpec { description: EdgeDescriptionSpec::chart(cap), carrier: carrier.clone(), param_start: T::zero(), param_end: span }
    };
    body.mev(MevSite::Fan { he1: he, he2: he }, carrier.eval(span), spec, Tol::witness()).unwrap();
    let findings = topo::pcurves::validate_pcurves(&body, Band::linear(Tol::witness()).unwrap());
    let [finding] = findings.as_slice() else { panic!("{}: {findings:?}", T::NAME) };
    let PcurveMintError::Certify { error, authority, .. } = finding else { panic!("{finding:?}") };
    assert!(matches!(error, PcurveCertifyError::CarrierOffChart { .. } | PcurveCertifyError::Escalated { .. }), "{error:?}");
    assert_eq!(authority.is_declared(), declared);
    let minted = topo::mint_pcurves_of(&mut body, &[wall], Tol::witness()).unwrap_err();
    let rest = topo::ValidationError::Pcurve { finding: minted.clone() }.to_string();
    let ext = sweep::ExtrudeError::Pcurve(minted.clone()).to_string();
    let rev = sweep::RevolveError::Pcurve(minted.clone()).to_string();
    for (door, text) in [("mint", minted.to_string()), ("extrude-wrap", ext), ("revolve-wrap", rev), ("rest", rest)] {
        let door = format!("{} strut {door}", T::NAME);
        if declared { lever_ok(&door, &text) } else { kernel_ok(&door, &text) }
    }
}

/// (A) set_edge_curve: the meridian seam re-described as a wider circle
/// arc through its two ends, inside the sphere.
fn seam_case<T: Decide + AtRestPolicy + Bounds>(declared: bool) {
    let f = T::from_f64;
    let mut body = hemisphere::<T>();
    let (_, sphere) = face_on(&body, true);
    let (edge, curve) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve).and_then(CurveGeom::certified)?;
            matches!(c.carrier(), Curve3::Circle { axis, .. } if axis.y.hi().abs() < 0.5).then(|| (k, c.clone()))
        })
        .unwrap();
    let (t0, t1) = curve.params();
    let (s, e) = (curve.carrier().eval(t0), curve.carrier().eval(t1));
    let c = Point3::new(f(-0.5), f(-0.5), f(0.0));
    let (a, b) = (s - c, e - c);
    let r = a.dot(a).sqrt();
    let z = a.x * b.y - a.y * b.x;
    let axis = if z.hi() > 0.0 { Vec3::unit_z() } else { Vec3::new(f(0.0), f(0.0), f(-1.0)) };
    let span = (a.dot(b) / (r * r)).acos();
    let carrier = Curve3::Circle { center: c, axis, radius: r, u_ref: a / r };
    let d = EdgeDescriptionSpec::chart(sphere);
    let spec = EdgeCurveSpec { description: if declared { d.declared_by(dummy()) } else { d }, carrier, param_start: T::zero(), param_end: span };
    let err = body.set_edge_curve(edge, spec, Tol::witness()).unwrap_err();
    let door = format!("{} seam set_edge_curve", T::NAME);
    let text = err.to_string();
    if let EulerOpError::Certification { error } = &err { println!("{door} {declared}: {error:?}"); }
    if declared { lever_ok(&door, &text) } else {
        println!("DERIVED {door}: {text}");
        assert!(!text.contains("declared curve"), "{text}");
    }
}

#[test] fn strut_f64() { strut_case::<f64>(true); strut_case::<f64>(false); }
#[test] fn strut_interval() { strut_case::<Interval>(true); strut_case::<Interval>(false); }
#[test] fn seam_f64() { seam_case::<f64>(true); seam_case::<f64>(false); }
#[test] fn seam_interval() { seam_case::<Interval>(true); seam_case::<Interval>(false); }
