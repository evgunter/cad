//! **A shared row on analytic charts certifies at any contact order.** A
//! fillet of a rounded outline meets its next link's band G1 along the
//! rolling ball's section: the cylinder band's transverse circle, which is
//! also the torus band's meridian. That curve is a row of both charts and is
//! known in advance, so it takes D2's structural selector,
//! `Structural::SharedRow`. Its certificate reads the two charts' row images
//! and the row's contact class (touching here). It reads no witness margin,
//! which could never pass: `κ_rel` is exactly zero where the section meets
//! the cap.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use core::f64::consts::FRAC_PI_2;
use geom::{Curve3, Surface};
use geom_brep::pcurve_cache::Channel;
use geom_brep::{
    CertifyError, CurveSelector, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, RowContact,
    RowRefusal, SharedRow, Structural, SurfaceKey, SurfacePair, analytic_shared_row,
};
use geom_core::{Interval, Point3, Real, Vec3};
use slotmap::SlotMap;

/// Two analytic walls and the edge's carrier and interval.
struct Row<T: Real> {
    walls: [Surface<T>; 2],
    carrier: Curve3<T>,
    span: (T, T),
}

impl Row<f64> {
    fn lifted(&self) -> Row<Interval> {
        let lift = Interval::from_f64;
        Row {
            walls: self.walls.clone().map(|w| w.map_scalar(lift)),
            carrier: self.carrier.map_scalar(lift),
            span: (lift(self.span.0), lift(self.span.1)),
        }
    }
}

/// The stadium's junction (a 0.5 half-length, 0.5 end-radius slot,
/// filleted at 0.1): the line link's cylinder band, the arc link's torus
/// band, and the quarter section from the wall at (0.5, −0.5, 0.9) to the
/// cap at (0.5, −0.4, 1.0).
fn stadium_junction() -> Row<f64> {
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

/// A unit cylinder about z and the sphere of radius 2 about
/// (0, 0, −√3), which cross along the unit circle in z = 0 at 60°.
fn cylinder_through_sphere() -> Row<f64> {
    Row {
        walls: [
            Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Sphere {
                center: Point3::new(0.0, 0.0, -(3.0f64.sqrt())),
                radius: 2.0,
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
        span: (0.0, FRAC_PI_2),
    }
}

/// The walls in an arena, keyed in the pair's order.
fn arena<T: Real>(
    row: &Row<T>,
) -> (
    SlotMap<SurfaceKey, Surface<T>>,
    SurfacePair,
    [SurfaceKey; 2],
) {
    let mut surfaces = SlotMap::with_key();
    let keys = row.walls.clone().map(|w| surfaces.insert(w));
    (surfaces, SurfacePair::new(keys[0], keys[1]), keys)
}

/// The shared row the constructor states for `row`.
fn stated<T: geom_core::Decide>(row: &Row<T>) -> Option<CurveSelector<T>> {
    let (surfaces, pair, keys) = arena(row);
    let walls = keys.map(|k| (k, &surfaces[k]));
    analytic_shared_row(pair, walls, &row.carrier, row.span, band())
}

/// `row`'s edge certified under `selector`.
fn certify<T: geom_core::Decide + geom_core::CertifiedBounds>(
    row: &Row<T>,
    selector: CurveSelector<T>,
) -> Result<EdgeCurve<T>, CertifyError> {
    let (surfaces, pair, _) = arena(row);
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection { pair, selector },
        carrier: row.carrier.clone(),
        param_start: row.span.0,
        param_end: row.span.1,
    };
    let (p0, p1) = (row.carrier.eval(row.span.0), row.carrier.eval(row.span.1));
    EdgeCurve::certify(
        spec,
        p0,
        p1,
        |k| surfaces.get(k).cloned(),
        geom_core::test_support::fixture_lever(T::from_f64(1.0)),
        band(),
        geom_brep::NurbsLane::certified(),
    )
}

/// `row` certified under its stated shared row.
fn certified<T: geom_core::Decide + geom_core::CertifiedBounds>(
    row: &Row<T>,
) -> Result<EdgeCurve<T>, CertifyError> {
    certify(
        row,
        stated(row).expect("both walls state the row and certify it"),
    )
}

#[test]
fn a_blend_section_certifies_as_a_touching_shared_row() {
    let row = stadium_junction();
    assert_eq!(
        certified(&row).unwrap().certificate().contact,
        Some(RowContact::Touching),
        "f64: the section touches along its whole row"
    );
    assert_eq!(
        certified(&row.lifted()).unwrap().certificate().contact,
        Some(RowContact::Touching),
        "Interval: the section touches along its whole row"
    );
}

#[test]
fn a_crossing_analytic_shared_row_certifies_as_crossing() {
    let row = cylinder_through_sphere();
    assert_eq!(
        certified(&row).unwrap().certificate().contact,
        Some(RowContact::Crossing),
        "f64: the walls cross at 60° along the whole row"
    );
    assert_eq!(
        certified(&row.lifted()).unwrap().certificate().contact,
        Some(RowContact::Crossing),
        "Interval: the walls cross at 60° along the whole row"
    );
}

/// The torus's row image is its meridian, a row with the azimuth fixed;
/// stated a milliradian off, it is not the edge on the torus, and the
/// torus's own row certificate says so.
#[test]
fn a_meridian_stated_off_by_a_milliradian_refuses_on_the_torus() {
    let row = stadium_junction();
    let (_, pair, keys) = arena(&row);
    let Some(CurveSelector::Structural(Structural::SharedRow(shared))) = stated(&row) else {
        panic!("the junction states a shared row");
    };
    let torus = keys[1];
    let image = |k: SurfaceKey| shared.image_on(pair, k).unwrap().clone();
    let Pcurve::Harmonic { u, v } = image(torus) else {
        panic!("a torus row image is closed form: {:?}", image(torus));
    };
    let nudged = |c: Channel<f64>| match c {
        Channel::Angle { phase, winding: 0 } => Channel::fixed_angle(phase + 1e-3),
        other => other,
    };
    let off = Pcurve::Harmonic {
        u: nudged(u),
        v: nudged(v),
    };
    let misstated = SharedRow::new(pair, (keys[0], image(keys[0])), (torus, off)).unwrap();
    let got = certify(
        &row,
        CurveSelector::Structural(Structural::SharedRow(misstated)),
    );
    assert!(
        matches!(
            got,
            Err(CertifyError::Row { surface, refusal: RowRefusal::Image(_) }) if surface == torus
        ),
        "the off-by-δ meridian refuses on the torus's row certificate: {got:?}"
    );
}
