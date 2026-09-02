//! CERT-N2 R2 reviewer probes: drive the S99 masquerade through every
//! consumer class's PUBLIC door from one workspace test crate. Probe
//! file — not for merge. Every row prints what LEAVES the door.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use topo::{Body, FaceKey, FaceSurface};

fn square() -> Vec<ProfileLoop<f64>> {
    let v = |x: f64, y: f64| ProfileVertex::new(Point2::new(x, y), 0.0);
    vec![ProfileLoop::new(vec![
        v(-1.0, -1.0),
        v(1.0, -1.0),
        v(1.0, 1.0),
        v(-1.0, 1.0),
    ])]
}

/// A bowed loft with described NURBS walls (copied from
/// `sweep/tests/m9_2_chart_region_loft.rs`).
fn loft() -> Body<f64> {
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body
}

fn nurbs_wall(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Nurbs(p)) if !p.is_placeholder()))
        .map(|(k, _)| k)
        .expect("a loft has described NURBS walls")
}

/// The wall's own structure with every control point re-mapped.
fn corrupt(
    wall: &NurbsSurface<f64>,
    f: impl Fn(usize, Point3<f64>) -> Point3<f64>,
) -> NurbsSurface<f64> {
    let control = wall
        .control()
        .iter()
        .enumerate()
        .map(|(i, p)| f(i, *p))
        .collect();
    NurbsSurface::new(
        wall.knots_u().clone(),
        wall.knots_v().clone(),
        control,
        wall.weights().to_vec(),
    )
    .unwrap()
}

fn poison_x(_: usize, p: Point3<f64>) -> Point3<f64> {
    Point3::new(f64::NAN, p.y, p.z)
}
fn poison_y(_: usize, p: Point3<f64>) -> Point3<f64> {
    Point3::new(p.x, f64::NAN, p.z)
}
fn poison_one_point(i: usize, p: Point3<f64>) -> Point3<f64> {
    if i == 0 {
        Point3::new(f64::NAN, f64::NAN, f64::NAN)
    } else {
        p
    }
}

/// A loft body whose one NURBS wall is swapped for the masquerade.
fn masqueraded(
    f: impl Fn(usize, Point3<f64>) -> Point3<f64>,
) -> (Body<f64>, FaceKey, NurbsSurface<f64>) {
    let mut body = loft();
    let wall = nurbs_wall(&body);
    let Some(Surface::Nurbs(payload)) = body
        .get_surface(body.get_face(wall).unwrap().surface)
        .cloned()
    else {
        unreachable!()
    };
    let masq = corrupt(&payload, f);
    assert!(
        !masq.is_placeholder(),
        "the masquerade reads described since PR 1558"
    );
    body.set_face_surface(
        wall,
        FaceSurface::New(Surface::Nurbs(Arc::new(masq.clone()))),
    )
    .unwrap();
    (body, wall, masq)
}

fn tol() -> Tol {
    Tol::witness()
}
fn band() -> Band {
    Band::linear(tol()).unwrap()
}

#[test]
fn n2r2_class3_chart_stretch_sup_inf_f64() {
    let (_, _, masq) = masqueraded(poison_x);
    let s = Surface::Nurbs(Arc::new(masq));
    let sup = geom_brep::chart_stretch_sup(&s);
    let inf = geom_brep::chart_stretch_inf(&s);
    eprintln!("[class 3 f64 x-poison] chart_stretch_sup = {sup:?}");
    eprintln!("[class 3 f64 x-poison] chart_stretch_inf = {inf:?}");
    let (_, _, my) = masqueraded(poison_y);
    let sy = Surface::Nurbs(Arc::new(my));
    eprintln!(
        "[class 3 f64 y-poison] sup = {:?} inf = {:?}",
        geom_brep::chart_stretch_sup(&sy),
        geom_brep::chart_stretch_inf(&sy)
    );
    let (_, _, m1) = masqueraded(poison_one_point);
    let s1 = Surface::Nurbs(Arc::new(m1));
    eprintln!(
        "[class 3 f64 one-point] sup = {:?} inf = {:?}",
        geom_brep::chart_stretch_sup(&s1),
        geom_brep::chart_stretch_inf(&s1)
    );
}

#[cfg(feature = "interval")]
#[test]
fn n2r2_class3_chart_stretch_sup_inf_interval() {
    use geom_core::Interval;
    let (_, _, masq) = masqueraded(poison_x);
    let mi: NurbsSurface<Interval> = masq.map_scalar(Interval::from_f64);
    assert!(!mi.is_placeholder());
    let s = Surface::Nurbs(Arc::new(mi));
    let sup = geom_brep::chart_stretch_sup(&s);
    let inf = geom_brep::chart_stretch_inf(&s);
    let show = |x: Interval| {
        format!(
            "[{}, {}] poison={}",
            geom_core::Bounds::lo(x),
            geom_core::Bounds::hi(x),
            x.is_poison()
        )
    };
    eprintln!(
        "[class 3 Interval x-poison] sup_u={} sup_v={}",
        show(sup.0),
        show(sup.1)
    );
    eprintln!(
        "[class 3 Interval x-poison] inf_u={} inf_v={} sup_u={} sup_v={} area_inf={}",
        show(inf.inf_u),
        show(inf.inf_v),
        show(inf.sup_u),
        show(inf.sup_v),
        show(inf.area_inf)
    );
    // Placeholder at Interval for comparison.
    let ph = Surface::<Interval>::nurbs_placeholder();
    let sp = geom_brep::chart_stretch_sup(&ph);
    eprintln!(
        "[class 3 Interval placeholder] sup_u={} sup_v={}",
        show(sp.0),
        show(sp.1)
    );
}

#[test]
fn n2r2_class9_validate_geometric_and_pseudomanifold() {
    for (name, f) in [
        (
            "x-poison",
            poison_x as fn(usize, Point3<f64>) -> Point3<f64>,
        ),
        ("y-poison", poison_y),
        ("one-point", poison_one_point),
    ] {
        let (body, _, _) = masqueraded(f);
        let g = topo::validate_geometric(&body, tol());
        eprintln!("[class 9 {name}] validate_geometric -> {g:?}");
        let pm =
            topo::validate_pseudomanifold(&body, &topo::boolean::ContactRecords::default(), tol());
        eprintln!("[class 8/7 {name}] validate_pseudomanifold -> {pm:?}");
    }
}

#[test]
fn n2r2_props_mass_properties() {
    let (body, _, _) = masqueraded(poison_x);
    let mp = topo::mass_properties(&body, tol());
    eprintln!("[props.rs:1349 x-poison] mass_properties -> {mp:?}");
    let (body, _, _) = masqueraded(poison_y);
    let mp = topo::mass_properties(&body, tol());
    eprintln!("[props.rs:1349 y-poison] mass_properties -> {mp:?}");
}

#[test]
fn n2r2_class2_tessellate() {
    let (body, _, _) = masqueraded(poison_x);
    let m = mesh::tessellate(&body, 1e-2, tol());
    match &m {
        Ok(mesh) => eprintln!("[class 2 x-poison] tessellate -> Ok: {mesh:?}"),
        Err(e) => eprintln!("[class 2 x-poison] tessellate -> Err: {e}"),
    }
}

#[test]
fn n2r2_class1_step_export() {
    let (body, _, _) = masqueraded(poison_x);
    let doc = step_export::step_string(&body, &step_export::StepOptions::default(), tol());
    match &doc {
        Ok(d) => eprintln!(
            "[class 1 x-poison] step_string -> Ok ({} bytes) contains NaN: {}",
            d.len(),
            d.contains("NaN")
        ),
        Err(e) => eprintln!("[class 1 x-poison] step_string -> Err: {e}"),
    }
}

#[test]
fn n2r2_class11_class4_mint_pcurves() {
    for (name, f) in [
        (
            "x-poison",
            poison_x as fn(usize, Point3<f64>) -> Point3<f64>,
        ),
        ("y-poison", poison_y),
        ("one-point", poison_one_point),
    ] {
        let (mut body, _, _) = masqueraded(f);
        let r = topo::mint_pcurves(&mut body, tol());
        eprintln!("[class 11/4 {name}] mint_pcurves -> {r:?}");
        let v = topo::pcurves::validate_pcurves(&body, band());
        eprintln!(
            "[class 11/4 {name}] validate_pcurves -> {} errors: {v:?}",
            v.len()
        );
    }
}

#[test]
fn n2r2_class10_replace_face_offset() {
    let (mut body, wall, _) = masqueraded(poison_x);
    let r = topo::replace_face_offset(&mut body, wall, 0.1, 1e-6, band(), tol());
    eprintln!("[class 10 x-poison] replace_face_offset -> {r:?}");
}

/// Class 12: is a non-finite real file-reachable? Part 21 cannot spell
/// NaN, but `1.E999` parses to +inf in Rust. Export a loft, corrupt one
/// CARTESIAN_POINT coordinate, re-import.
#[test]
fn n2r2_class12_step_import_overflow_real() {
    let body = loft();
    let doc = step_export::step_string(&body, &step_export::StepOptions::default(), tol()).unwrap();
    // Find a CARTESIAN_POINT line and replace its first coordinate:
    // `#n = CARTESIAN_POINT('', (x, y, z));`
    let mut out = String::new();
    let mut done = false;
    for line in doc.lines() {
        if !done && line.contains("CARTESIAN_POINT(") {
            let after_name = line.find("CARTESIAN_POINT(").unwrap() + "CARTESIAN_POINT(".len();
            if let Some(rel) = line[after_name..].find('(') {
                let open = after_name + rel + 1;
                if let Some(comma) = line[open..].find(',') {
                    out.push_str(&line[..open]);
                    out.push_str("1.E999");
                    out.push_str(&line[open + comma..]);
                    out.push('\n');
                    done = true;
                    continue;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(done, "a CARTESIAN_POINT was corrupted");
    let r = step_import::import_step(&out, &step_import::ImportOptions::default(), tol());
    match r {
        Ok(_) => eprintln!(
            "[class 12] import of a 1.E999 coordinate -> Ok (a non-finite real entered a body)"
        ),
        Err(e) => eprintln!("[class 12] import of a 1.E999 coordinate -> Err: {e}"),
    }
}
