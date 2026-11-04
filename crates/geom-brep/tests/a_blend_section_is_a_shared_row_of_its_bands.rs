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
    CertifyError, CurveSelector, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve,
    PcurveCache, RowContact, RowRefusal, SharedRow, Structural, SurfaceKey, SurfacePair,
    analytic_shared_row,
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

/// A unit cylinder about z and the sphere through the unit circle in
/// z = 0 whose normal there makes `angle` with the cylinder's: centred at
/// (0, 0, −tan `angle`), radius 1/cos `angle`. The two cross along that
/// circle at `angle`, over `span`.
fn cylinder_through_sphere(angle: f64, span: (f64, f64)) -> Row<f64> {
    Row {
        walls: [
            Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Sphere {
                center: Point3::new(0.0, 0.0, -angle.tan()),
                radius: 1.0 / angle.cos(),
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

/// A cone about z with its apex at the origin and a 30° half-angle, and
/// the plane through its axis that holds the generator along
/// (sin 30°, 0, cos 30°), which crosses the cone at 90° along it. The
/// carrier is that generator, its parameter's origin `before` metres
/// before the apex, over `span`.
fn cone_generator(before: f64, span: (f64, f64)) -> Row<f64> {
    let (s, c) = 30f64.to_radians().sin_cos();
    let along = Vec3::new(s, 0.0, c);
    let apex = Point3::new(0.0, 0.0, 0.0);
    Row {
        walls: [
            Surface::Cone {
                apex,
                axis: Vec3::new(0.0, 0.0, 1.0),
                half_angle: 30f64.to_radians(),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Plane {
                origin: apex,
                normal: Vec3::new(0.0, 1.0, 0.0),
                u_ref: along,
            },
        ],
        carrier: Curve3::line(apex - along * before, along),
        span,
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

/// `row` certified under its stated shared row, with the contact class
/// read by the certificate's own reading at the certificate's own lever
/// (`edge_extent_inf` over the edge), over each wall's certified image.
fn certified<T: geom_core::Decide + geom_core::CertifiedBounds>(
    row: &Row<T>,
) -> Result<RowContact, CertifyError> {
    let selector = stated(row).expect("both walls state the row and certify it");
    certify(row, selector.clone())?;
    let CurveSelector::Structural(Structural::SharedRow(shared)) = &selector else {
        panic!("the constructor states a shared row");
    };
    let (surfaces, pair, _) = arena(row);
    let (t0, t1) = row.span;
    let cache = |k: SurfaceKey| {
        PcurveCache::certify(
            shared.image_on(pair, k).unwrap().clone(),
            t0,
            t1,
            &row.carrier,
            &surfaces[k],
            band(),
        )
        .expect("each wall's image certified under the edge")
    };
    let [ka, kb] = pair.keys();
    let (ca, cb) = (cache(ka), cache(kb));
    let ends = (row.carrier.eval(t0), row.carrier.eval(t1));
    geom_brep::analytic_row_contact(
        [(ka, &surfaces[ka], &ca), (kb, &surfaces[kb], &cb)],
        geom_brep::edge_extent_inf(&row.carrier, t0, t1, ends),
        band(),
    )
}

#[test]
fn a_blend_section_certifies_as_a_touching_shared_row() {
    let row = stadium_junction();
    assert_eq!(
        certified(&row).unwrap(),
        RowContact::Touching,
        "f64: the section touches along its whole row"
    );
    assert_eq!(
        certified(&row.lifted()).unwrap(),
        RowContact::Touching,
        "Interval: the section touches along its whole row"
    );
}

/// The walls' angle along a shared circle is constant, and the reading
/// charges it no variation: 60° and 1° over a full turn, and a milliradian
/// over a quarter turn, all certify as crossing.
#[test]
fn a_crossing_analytic_shared_row_certifies_as_crossing() {
    let (quarter, turn) = (FRAC_PI_2, core::f64::consts::TAU);
    for (angle, span) in [
        (60f64.to_radians(), (0.0, quarter)),
        (1f64.to_radians(), (0.0, turn)),
        (1e-3, (0.0, quarter)),
    ] {
        let row = cylinder_through_sphere(angle, span);
        assert_eq!(
            certified(&row).unwrap(),
            RowContact::Crossing,
            "f64: {angle} rad over {span:?}"
        );
        assert_eq!(
            certified(&row.lifted()).unwrap(),
            RowContact::Crossing,
            "Interval: {angle} rad over {span:?}"
        );
    }
}

/// A cone's generator certifies on one nappe, whichever end of its
/// parameter the apex sits at: past the apex (the origin a metre before
/// it, the span `(1.5, 2.5)`), and from it (the origin at the apex, the
/// span `(0.5, 1)`). At `f64` only: the plane's image of the generator
/// holds `v` by a coefficient `sin α·cos α − cos α·sin α`, exactly zero at
/// `f64` and an enclosure about zero at `Interval`, where the constructor
/// therefore states no row.
#[test]
fn a_cone_generator_on_one_nappe_certifies_as_crossing() {
    for (before, span) in [(1.0, (1.5, 2.5)), (0.0, (0.5, 1.0))] {
        let row = cone_generator(before, span);
        assert_eq!(
            certified(&row).unwrap(),
            RowContact::Crossing,
            "the origin {before} m before the apex, over {span:?}"
        );
    }
}

/// A generator through the apex, or ending on it, refuses on the cone as
/// a row through its chart's singular point, before any image
/// certificate or contact reading.
#[test]
fn a_cone_generator_reaching_the_apex_refuses_on_the_cone() {
    for (before, span) in [(1.0, (0.5, 1.5)), (0.0, (0.0, 1.0))] {
        let row = cone_generator(before, span);
        let (_, pair, keys) = arena(&row);
        let image = |k: SurfaceKey| {
            geom_brep::pcurve_cache::chart_pcurve_over(
                &row.carrier,
                span.0,
                span.1,
                &row.walls[keys.iter().position(|x| *x == k).unwrap()],
                band(),
            )
            .unwrap()
        };
        let shared = SharedRow::new(pair, (keys[0], image(keys[0])), (keys[1], image(keys[1])))
            .expect("both images are rows");
        let got = certify(
            &row,
            CurveSelector::Structural(Structural::SharedRow(shared)),
        );
        assert!(
            matches!(
                got,
                Err(CertifyError::Row { surface, refusal: RowRefusal::Singular }) if surface == keys[0]
            ),
            "over {span:?} the row reaches the apex: {got:?}"
        );
    }
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

/// **The short-span limit at `Interval`, pinned where it stands today**:
/// the stadium junction over `1e-5` of its section escalates its touch
/// bound (`shared_row_touch`) at `Interval` while `f64` still reads
/// Touching. Red if the limit moves either way
/// (`work/select/the-shared-row-contact-class-is-levered-by-the-edges-length.md`).
///
/// Pinned at the default ε only. The `Interval` touch bound, levered, is
/// about `1e-13 / span` here, so each ε's band meets it at its own span:
/// at ε = 1e-12 every span up to `1e-2` escalates, and at ε = 1e-6 the
/// walls' own row certificates refuse a span this short before the
/// contact is read. Those rows say so in the log.
#[test]
fn a_stadium_section_over_ten_microradians_escalates_its_touch_at_interval() {
    let eps = crate::shared::tol::eps();
    if eps.to_bits() != geom_core::tolerance::DEFAULT_EPS.to_bits() {
        println!(
            "SKIPPED at ε {eps:e}: the short-span limit is pinned at the default ε {:e}, \
             where the Interval touch bound (about 1e-13 / span) crosses the band at a span \
             the walls' row certificates still admit",
            geom_core::tolerance::DEFAULT_EPS
        );
        return;
    }
    let mut row = stadium_junction();
    row.span = (0.0, 1e-5);
    assert_eq!(
        certified(&row).unwrap(),
        RowContact::Touching,
        "f64 still reads it"
    );
    let got = certified(&row.lifted());
    assert!(
        matches!(
            &got,
            Err(CertifyError::Escalated { check: geom_brep::CertCheck::Transversality, cause, .. })
                if cause.predicate == Some("shared_row_touch")
        ),
        "Interval escalates the touch bound over 1e-5: {got:?}"
    );
}
