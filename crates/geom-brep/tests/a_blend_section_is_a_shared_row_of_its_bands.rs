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
        self.widened(0.0)
    }

    /// Every input as the interval of half-width `w` about it.
    fn widened(&self, w: f64) -> Row<Interval> {
        let lift = |x: f64| Interval::from_bounds(x - w, x + w);
        Row {
            walls: self.walls.clone().map(|s| s.map_scalar(lift)),
            carrier: self.carrier.map_scalar(lift),
            span: (lift(self.span.0), lift(self.span.1)),
        }
    }
}

/// **A rounded outline's line-to-arc junction**, filleted at `r` with the
/// cap at `z + r`: the arc link's torus band about `centre` (axis z, spine
/// radius `big`), the line link's cylinder band along the heading `d` at
/// `heading` radians, and the quarter section from the wall to the cap.
/// The junction's spine point is `centre + big·n` with `n = (d_y, −d_x, 0)`
/// the outline's outward normal there, and the section is the circle of
/// radius `r` about it, square to `d`.
fn junction(centre: Point3<f64>, big: f64, r: f64, heading: f64) -> Row<f64> {
    let (s, c) = heading.sin_cos();
    let d = Vec3::new(c, s, 0.0);
    let n = Vec3::new(d.y, -d.x, 0.0);
    let spine = centre + n * big;
    Row {
        walls: [
            Surface::Cylinder {
                frame: geom::Frame::new(spine, d, n),
                radius: r,
            },
            Surface::Torus {
                frame: geom::Frame::new(centre, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
                major_radius: big,
                minor_radius: r,
            },
        ],
        carrier: Curve3::circle(spine, -d, r, n),
        span: (0.0, FRAC_PI_2),
    }
}

/// The stadium's junction (a 0.5 half-length, 0.5 end-radius slot,
/// filleted at 0.1): the line link's cylinder band along x, the arc
/// link's torus band, and the quarter section from the wall at
/// (0.5, −0.5, 0.9) to the cap at (0.5, −0.4, 1.0).
fn stadium_junction() -> Row<f64> {
    junction(Point3::new(0.5, 0.0, 0.9), 0.4, 0.1, 0.0)
}

/// A unit cylinder about z and the sphere through the unit circle in
/// z = 0 whose normal there makes `angle` with the cylinder's: centred at
/// (0, 0, −tan `angle`), radius 1/cos `angle`. The two cross along that
/// circle at `angle`, over `span`.
fn cylinder_through_sphere(angle: f64, span: (f64, f64)) -> Row<f64> {
    Row {
        walls: [
            Surface::Cylinder {
                frame: geom::Frame::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                radius: 1.0,
            },
            Surface::Sphere {
                frame: geom::Frame::new(
                    Point3::new(0.0, 0.0, -angle.tan()),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                radius: 1.0 / angle.cos(),
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
                frame: geom::Frame::new(apex, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
                half_angle: 30f64.to_radians(),
            },
            Surface::Plane {
                frame: geom::Frame::new(apex, Vec3::new(0.0, 1.0, 0.0), along),
            },
        ],
        carrier: Curve3::line(apex - along * before, along),
        span,
    }
}

/// **A rounded outline's line-to-arc junction, chamfered** at setback `s`
/// on a top at height `z` of `centre`: the arc link's frustum (the cone
/// through the plane's trim circle `R − s` and the wall's `s` down, its
/// apex on the arc's axis), the line link's strip along the heading `d`,
/// and the section between the two feet. The strip holds the generator
/// and the arc's tangent `d` there, which is the cone's tangent plane
/// along that generator, so the two touch along it.
fn chamfer_junction(centre: Point3<f64>, big: f64, s: f64, heading: f64) -> Row<f64> {
    let (sn, c) = heading.sin_cos();
    let d = Vec3::new(c, sn, 0.0);
    let n = Vec3::new(d.y, -d.x, 0.0);
    let up = Vec3::new(0.0, 0.0, 1.0);
    let on_plane = centre + n * (big - s);
    let on_wall = centre + n * big - up * s;
    let g = on_wall - on_plane;
    let len = g.norm();
    Row {
        walls: [
            Surface::Plane {
                frame: geom::Frame::new(on_plane, (n + up).normalize(), d),
            },
            Surface::Cone {
                frame: geom::Frame::new(centre + up * (big - s), -up, n),
                half_angle: core::f64::consts::FRAC_PI_4,
            },
        ],
        carrier: Curve3::line(on_plane, g / len),
        span: (0.0, len),
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
fn certify<T: geom_core::Certifies + geom_core::CertifiedBounds>(
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
fn certified<T: geom_core::Certifies + geom_core::CertifiedBounds>(
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
    geom_brep::analytic_row_contact(
        [(ka, &surfaces[ka], &ca), (kb, &surfaces[kb], &cb)],
        geom_brep::edge_extent_inf(&row.carrier, t0, t1),
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

/// **A section on a band that is not axis-aligned is a shared row too.**
/// The stadium rotated about z by 7°, 30°, 45° and 90°, and a rounded
/// pentagon's junction (its arc about (1.2, −0.7), spine radius 0.3,
/// filleted at 0.08, entered along a heading of 144°): the cylinder's
/// image of the section holds its axial coordinate by coefficients the
/// derivation computes, a residual of rounding at `f64`, which the row's
/// reading decides as zero against the band.
#[test]
fn a_section_on_a_rotated_band_certifies_as_a_touching_shared_row() {
    let rotated = |deg: f64| {
        let (s, c) = deg.to_radians().sin_cos();
        let centre = Point3::new(0.5 * c, 0.5 * s, 0.9);
        (
            format!("the stadium turned {deg}°"),
            junction(centre, 0.4, 0.1, deg.to_radians()),
        )
    };
    let mut rows: Vec<(String, Row<f64>)> = [7.0, 30.0, 45.0, 90.0].map(rotated).into();
    rows.push((
        "a rounded pentagon".to_owned(),
        junction(Point3::new(1.2, -0.7, 0.9), 0.3, 0.08, 144f64.to_radians()),
    ));
    for (what, row) in &rows {
        assert_eq!(certified(row).unwrap(), RowContact::Touching, "f64: {what}");
        assert_eq!(
            certified(&row.lifted()).unwrap(),
            RowContact::Touching,
            "Interval: {what}"
        );
    }
}

/// **A chamfer's section is a shared row too**: the strip and the
/// frustum touch along the generator between their feet, the plane's
/// normal constant and the cone's constant along a generator on one
/// nappe. The rounded plate's junction (arc radius 0.3, setback 0.05),
/// turned and moved off the origin, and a setback just under the arc's
/// radius. Read at `f64` and at its point intervals: what this row says
/// of the certified scalar is that the exact geometry certifies, not
/// that a carve's computed geometry does (the next row).
#[test]
fn a_chamfer_section_certifies_as_a_touching_plane_cone_row() {
    let rows = [
        (
            "the plate's junction",
            chamfer_junction(Point3::new(0.7, -0.3, 0.5), 0.3, 0.05, 0.0),
        ),
        (
            "turned 30°, off the origin",
            chamfer_junction(Point3::new(1.2, -0.7, 0.9), 0.3, 0.05, 30f64.to_radians()),
        ),
        (
            "turned 144°",
            chamfer_junction(
                Point3::new(-2.0, 3.1, -0.4),
                0.25,
                0.08,
                144f64.to_radians(),
            ),
        ),
        (
            "a setback just under the radius",
            chamfer_junction(Point3::new(0.0, 0.0, 0.5), 0.3, 0.29, 7f64.to_radians()),
        ),
    ];
    for (what, row) in &rows {
        assert_eq!(certified(row).unwrap(), RowContact::Touching, "f64: {what}");
        assert_eq!(
            certified(&row.lifted()).unwrap(),
            RowContact::Touching,
            "Interval: {what}"
        );
    }
}

/// **How wide a chamfer section's inputs may be and still admit**, at
/// `Interval`: the plate's junction with every input widened to an
/// interval of half-width `w`, which is what a carve's computed feet and
/// surfaces hand the admission. It certifies at `w = ε/100` and refuses
/// at `w = 10ε`, at every ε row (measured at ε = 1e-12: to `1e-14`, not
/// from `1e-13`; at ε = 1e-9: to `1e-11`), so a carve whose computed
/// geometry encloses wider than a tenth of the band reaches the
/// refusal its rows pin at the tightest ε
/// (`work/band/every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12.md`).
#[test]
fn a_chamfer_section_admits_while_its_inputs_are_narrow_against_the_band() {
    let eps = crate::shared::tol::eps();
    let row = chamfer_junction(Point3::new(0.7, -0.3, 0.5), 0.3, 0.05, 30f64.to_radians());
    let narrow = row.widened(eps / 100.0);
    assert_eq!(
        stated(&narrow)
            .map(|_| ())
            .ok_or(())
            .and_then(|()| certified(&narrow).map_err(|_| ())),
        Ok(RowContact::Touching),
        "ε = {eps}: inputs {} wide admit and touch",
        eps / 100.0
    );
    let wide = row.widened(10.0 * eps);
    let admitted = stated(&wide).is_some() && certified(&wide).is_ok();
    assert!(
        !admitted,
        "ε = {eps}: inputs {} wide admit, past the band",
        10.0 * eps
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
/// span `(0.5, 1)`). The plane's image of the generator holds `v` by the
/// slope `sin α·cos α − cos α·sin α`, exactly zero at `f64` and an
/// enclosure about zero at `Interval`; the row's reading decides it zero
/// against the band at both.
#[test]
fn a_cone_generator_on_one_nappe_certifies_as_crossing() {
    for (before, span) in [(1.0, (1.5, 2.5)), (0.0, (0.5, 1.0))] {
        let row = cone_generator(before, span);
        assert_eq!(
            certified(&row).unwrap(),
            RowContact::Crossing,
            "f64: the origin {before} m before the apex, over {span:?}"
        );
        assert_eq!(
            certified(&row.lifted()).unwrap(),
            RowContact::Crossing,
            "Interval: the origin {before} m before the apex, over {span:?}"
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
