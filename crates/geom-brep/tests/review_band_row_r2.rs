//! Review probes, lane band-row-r2 (PR 6514). Not for merge as-is.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::shared::tol::band;
use core::f64::consts::{FRAC_PI_2, PI};
use geom::{Curve3, Surface};
use geom_brep::pcurve_cache::Channel;
use geom_brep::{
    CertifyError, CurveSelector, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, RowContact,
    SharedRow, Structural, SurfaceKey, SurfacePair, analytic_shared_row,
};
use geom_core::{Interval, Point3, Real, Vec3};
use slotmap::SlotMap;

#[derive(Clone)]
struct Row<T: Real> {
    walls: [Surface<T>; 2],
    carrier: Curve3<T>,
    span: (T, T),
}

fn lift(r: &Row<f64>) -> Row<Interval> {
    let l = Interval::from_f64;
    Row {
        walls: r.walls.clone().map(|w| w.map_scalar(l)),
        carrier: r.carrier.map_scalar(l),
        span: (l(r.span.0), l(r.span.1)),
    }
}

fn arena<T: Real>(row: &Row<T>) -> (SlotMap<SurfaceKey, Surface<T>>, SurfacePair, [SurfaceKey; 2]) {
    let mut s = SlotMap::with_key();
    let keys = row.walls.clone().map(|w| s.insert(w));
    (s, SurfacePair::new(keys[0], keys[1]), keys)
}

fn stated<T: geom_core::Decide>(row: &Row<T>) -> Option<CurveSelector<T>> {
    let (s, pair, keys) = arena(row);
    analytic_shared_row(pair, keys.map(|k| (k, &s[k])), &row.carrier, row.span, band())
}

fn certify<T: geom_core::Decide + geom_core::CertifiedBounds>(
    row: &Row<T>,
    selector: CurveSelector<T>,
) -> Result<(), CertifyError> {
    let (s, pair, _) = arena(row);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection { pair, selector },
        carrier: row.carrier.clone(),
        param_start: row.span.0,
        param_end: row.span.1,
    };
    let (p0, p1) = (row.carrier.eval(row.span.0), row.carrier.eval(row.span.1));
    geom_brep::EdgeCurve::certify(
        spec,
        p0,
        p1,
        |k| s.get(k).cloned(),
        geom_core::test_support::fixture_lever(T::from_f64(1.0)),
        band(),
        geom_brep::NurbsLane::certified(),
    )
    .map(|_| ())
}

fn class<T: geom_core::Decide>(row: &Row<T>, sel: &CurveSelector<T>) -> Result<RowContact, CertifyError> {
    let CurveSelector::Structural(Structural::SharedRow(shared)) = sel else { panic!() };
    let (s, pair, _) = arena(row);
    let walls = pair.keys().map(|k| (&s[k], shared.image_on(pair, k).unwrap()));
    geom_brep::row_contact::analytic_row_contact(
        row.span,
        walls,
        geom_core::test_support::fixture_lever(T::from_f64(1.0)),
        band(),
    )
    .map(|(c, _)| c)
}

fn stadium() -> Row<f64> {
    Row {
        walls: [
            Surface::Cylinder {
                origin: Point3::new(0.0, -0.4, 0.9),
                axis: Vec3::new(1.0, 0.0, 0.0),
                radius: 0.1,
                u_ref: Vec3::new(0.0, -1.0, 0.0),
            },
            Surface::Torus {
                center: Point3::new(0.5, 0.0, 0.9),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major_radius: 0.4,
                minor_radius: 0.1,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
        ],
        carrier: Curve3::circle(
            Point3::new(0.5, -0.4, 0.9),
            Vec3::new(-1.0, 0.0, 0.0),
            0.1,
            Vec3::new(0.0, -1.0, 0.0),
        ),
        span: (0.0, FRAC_PI_2),
    }
}

fn images(row: &Row<f64>) -> (SurfacePair, [SurfaceKey; 2], [Pcurve<f64>; 2]) {
    let (_, pair, keys) = arena(row);
    let Some(CurveSelector::Structural(Structural::SharedRow(shared))) = stated(row) else {
        panic!("stadium states a row")
    };
    let im = keys.map(|k| shared.image_on(pair, k).unwrap().clone());
    (pair, keys, im)
}

fn with(row: &Row<f64>, which: usize, image: Pcurve<f64>) -> CurveSelector<f64> {
    let (pair, keys, mut im) = images(row);
    im[which] = image;
    let [a, b] = im;
    CurveSelector::Structural(Structural::SharedRow(
        SharedRow::new(pair, (keys[0], a), (keys[1], b)).expect("a row shape"),
    ))
}

fn nudge(c: Channel<f64>, fixed: bool, d: f64, flip_winding: bool) -> Channel<f64> {
    match c {
        Channel::Angle { phase, winding } if (winding == 0) == fixed => Channel::Angle {
            phase: phase + d,
            winding: if flip_winding { -winding } else { winding },
        },
        Channel::Free { c, a, b, l } if (l == 0.0) == fixed => Channel::Free {
            c: c + d,
            a,
            b,
            l: if flip_winding { -l } else { l },
        },
        other => other,
    }
}

#[test]
fn r2_stadium_end_to_end() {
    let row = stadium();
    let (_, _, im) = images(&row);
    eprintln!("stadium images: {im:?}");
    let sel = stated(&row).unwrap();
    eprintln!("f64 class {:?}, certify {:?}", class(&row, &sel), certify(&row, sel.clone()));
    let li = lift(&row);
    let sel = stated(&li).unwrap();
    eprintln!("Interval class {:?}, certify {:?}", class(&li, &sel), certify(&li, sel.clone()));
    // Reversed carrier direction / other quarter (cap to wall) via span (π/2, π)? not on torus side; skip.
}

#[test]
fn r2_misstated_rows_refuse() {
    let row = stadium();
    let (_, _, im) = images(&row);
    for which in 0..2 {
        let Pcurve::Harmonic { u, v } = im[which] else { panic!("{:?}", im[which]) };
        for (what, fixed, d, flip) in [
            ("fixed +1e-3", true, 1e-3, false),
            ("fixed +1e-6", true, 1e-6, false),
            ("fixed +1e-9", true, 1e-9, false),
            ("fixed +1e-13", true, 1e-13, false),
            ("fixed +pi", true, PI, false),
            ("moving phase +1e-3", false, 1e-3, false),
            ("moving phase +1e-7", false, 1e-7, false),
            ("moving winding flipped", false, 0.0, true),
            ("moving phase +pi winding flipped", false, PI, true),
        ] {
            let off = Pcurve::Harmonic {
                u: nudge(u, fixed, d, flip),
                v: nudge(v, fixed, d, flip),
            };
            let got = certify(&row, with(&row, which, off));
            eprintln!("wall {which} {what}: {:?}", got.as_ref().map_err(|e| format!("{e:?}").chars().take(160).collect::<String>()));
        }
    }
    // swapped: each wall stated with the other wall's image
    let (pair, keys, im2) = images(&row);
    let swapped = SharedRow::new(pair, (keys[0], im2[1].clone()), (keys[1], im2[0].clone()));
    if let Some(s) = swapped {
        let got = certify(&row, CurveSelector::Structural(Structural::SharedRow(s)));
        eprintln!("swapped images: {:?}", got.as_ref().map_err(|e| format!("{e:?}").chars().take(160).collect::<String>()));
    }
}

/// The cylinder's transverse circle at x = 0.3: on the cylinder, not the torus.
#[test]
fn r2_a_row_of_one_wall_only_refuses() {
    let mut row = stadium();
    let (pair, keys, im) = images(&row);
    row.carrier = Curve3::circle(
        Point3::new(0.3, -0.4, 0.9),
        Vec3::new(-1.0, 0.0, 0.0),
        0.1,
        Vec3::new(0.0, -1.0, 0.0),
    );
    eprintln!("constructor on off-torus circle: {:?}", stated(&row).is_some());
    let s = SharedRow::new(pair, (keys[0], im[0].clone()), (keys[1], im[1].clone())).unwrap();
    let got = certify(&row, CurveSelector::Structural(Structural::SharedRow(s)));
    eprintln!("junction images over off-torus carrier: {got:?}");
    assert!(got.is_err());
}

/// A unit cylinder about z and a sphere through its z=0 unit circle,
/// centred at (0,0,-h): they cross at a constant angle, sin = h/R.
fn shallow(h: f64, span: (f64, f64)) -> Row<f64> {
    Row {
        walls: [
            Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Sphere {
                center: Point3::new(0.0, 0.0, -h),
                radius: (1.0 + h * h).sqrt(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
        ],
        carrier: Curve3::circle(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        ),
        span,
    }
}

#[test]
fn r2_constant_shallow_crossings() {
    for span in [(0.0, FRAC_PI_2), (0.0, 2.0 * PI - 1e-3), (0.3, 0.31)] {
        for h in [0.5, 0.05, 0.02, 0.01, 0.005, 1e-3, 1e-5, 1e-7, 1e-9, 1e-11, 0.0] {
            let row = shallow(h, span);
            let Some(sel) = stated(&row) else {
                eprintln!("span {span:?} h {h}: constructor None");
                continue;
            };
            let c = class(&row, &sel);
            let li = lift(&row);
            let ci = stated(&li).map(|s| class(&li, &s));
            eprintln!(
                "span {span:?} h {h:e}: f64 {:?} | certify {:?} || Interval {:?}",
                c.as_ref().map_err(|e| format!("{e:?}").chars().take(90).collect::<String>()),
                certify(&row, sel).map_err(|e| format!("{e:?}").chars().take(200).collect::<String>()),
                ci.map(|r| r.map_err(|e| format!("{e:?}").chars().take(90).collect::<String>())),
            );
        }
    }
}

/// A cone and planes along a generator, carrier a Line from the apex.
fn cone_rows(tangent: bool, origin_at_apex: bool, span: (f64, f64)) -> Row<f64> {
    let a: f64 = 0.5;
    let dir = Vec3::new(a.sin(), 0.0, a.cos());
    let normal = if tangent { Vec3::new(a.cos(), 0.0, -a.sin()) } else { Vec3::new(0.0, 1.0, 0.0) };
    let u_ref = if tangent { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
    let o = if origin_at_apex { Point3::new(0.0, 0.0, 0.0) } else { Point3::new(0.0, 0.0, 0.0) + dir * 0.25 };
    Row {
        walls: [
            Surface::Cone {
                apex: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                half_angle: a,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Plane { origin: Point3::new(0.0, 0.0, 0.0), normal, u_ref },
        ],
        carrier: Curve3::line(o, dir),
        span,
    }
}

#[test]
fn r2_cone_generator_rows() {
    for tangent in [true, false] {
        for apex in [true, false] {
            for span in [(0.5, 1.0), (0.1, 2.0)] {
                let row = cone_rows(tangent, apex, span);
                let sel = stated(&row);
                eprintln!(
                    "cone tangent={tangent} carrier-at-apex={apex} span={span:?}: stated {:?}",
                    sel.as_ref().map(|s| match s {
                        CurveSelector::Structural(Structural::SharedRow(r)) => format!("{:?}", r.images()),
                        _ => String::new(),
                    })
                );
                if let Some(sel) = sel {
                    eprintln!("   class {:?} certify {:?}", class(&row, &sel), certify(&row, sel.clone()).map_err(|e| format!("{e:?}").chars().take(160).collect::<String>()));
                    let li = lift(&row);
                    if let Some(s) = stated(&li) {
                        eprintln!("   Interval class {:?}", class(&li, &s).map_err(|e| format!("{e:?}").chars().take(160).collect::<String>()));
                    }
                }
            }
        }
    }
}

fn ring(m: usize, turn: f64, profile: [(f64, f64); 2]) -> Surface<f64> {
    use geom_brep::edge_nurbs::sub_arc_joint;
    let h = turn / m as f64;
    let mut ku = vec![0.0; 3];
    for k in 1..m { ku.extend([sub_arc_joint(k, m); 2]); }
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=2 * m {
        let theta = j as f64 * h / 2.0;
        let (reach, w) = if j % 2 == 1 { (1.0 / (h / 2.0).cos(), (h / 2.0).cos()) } else { (1.0, 1.0) };
        for (r, z) in profile {
            let r = r * reach;
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    Surface::Nurbs(std::sync::Arc::new(geom::NurbsSurface::new(
        geom_core::spline::KnotVector::clamped(ku, 2).unwrap(),
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control, weights).unwrap()))
}

#[test]
fn r2_mixed_pair_refuses_unimplemented() {
    let carrier = Curve3::circle(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 1.0, Vec3::new(1.0, 0.0, 0.0));
    let span = (0.1, 1.0);
    let spline = ring(2, 1.2, [(1.0, 0.0), (1.0, 1.0)]);
    let spline_row = geom_brep::edge_nurbs::pick_arc_rim(&carrier, (span.0, span.1 - span.0), &spline, band()).unwrap().unwrap();
    // an analytic cylinder sharing the rim; its row image
    let cyl = Surface::Cylinder { origin: Point3::new(0.0, 0.0, 0.0), axis: Vec3::new(0.0, 0.0, 1.0), radius: 1.0, u_ref: Vec3::new(1.0, 0.0, 0.0) };
    let cyl_row = geom_brep::chart_pcurve_over(&carrier, span.0, span.1, &cyl, band()).unwrap();
    for order in [0, 1] {
        let walls = if order == 0 { [spline.clone(), cyl.clone()] } else { [cyl.clone(), spline.clone()] };
        let row = Row { walls, carrier: carrier.clone(), span };
        let (_, pair, keys) = arena(&row);
        let ims = if order == 0 { [spline_row.clone(), cyl_row.clone()] } else { [cyl_row.clone(), spline_row.clone()] };
        let [a, b] = ims;
        let s = SharedRow::new(pair, (keys[0], a), (keys[1], b)).unwrap();
        let got = certify(&row, CurveSelector::Structural(Structural::SharedRow(s)));
        eprintln!("mixed order {order}: {got:?}");
        assert!(matches!(got, Err(CertifyError::Unimplemented)));
        // misassigned: the harmonic image stated on the spline wall
        let [a, b] = if order == 0 { [cyl_row.clone(), spline_row.clone()] } else { [spline_row.clone(), cyl_row.clone()] };
        if let Some(s) = SharedRow::new(pair, (keys[0], a), (keys[1], b)) {
            eprintln!("mixed misassigned order {order}: {:?}", certify(&row, CurveSelector::Structural(Structural::SharedRow(s))));
        }
    }
}
