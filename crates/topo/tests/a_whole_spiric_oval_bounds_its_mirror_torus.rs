//! **A whole spiric oval bounds a face on its torus's mirror**, through
//! public Euler doors alone. Reflecting a torus through a plane
//! parallel to its axis gives the one other torus that holds the
//! plane's section: `mvfs` seeds a lone vertex on the oval,
//! `set_face_surface` puts the face on the mirror torus, and `mef` at
//! the lone loop closes one edge on the whole oval, described in that
//! chart. The mint then stores both halves' wall rows tagged
//! [`WallTorus::Mirror`], and tier 3 reads the body clean, with the
//! chart axis either way and at `f64` and `Interval` (every number in
//! the fixture is dyadic, so the lift is exact).
//!
//! The same body is the quadrature lane's mirror-wall fixture: its wall
//! terms read the carrier's own torus, so `mass_properties` refuses the
//! face typed rather than integrating it against the wrong torus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, SpiricImage, WallTorus};
use geom_core::{Band, Bounds, Point3, Tol, Vec3};
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite};
use topo::{AtRestPolicy, Body, HalfEdgeKey};

/// The mirror-torus body for a chart axis of `sigma·a`, and the oval
/// edge's two half-edges.
fn mirror_oval<T: AtRestPolicy>(sigma: f64) -> (Body<T>, [HalfEdgeKey; 2]) {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let f = T::from_f64;
    let (big, r, d) = (f(0.09375), f(0.0703125), f(0.0078125));
    let (n, a) = (Vec3::<T>::unit_x(), Vec3::<T>::unit_y());
    let carrier = geom::Curve3::spiric(Point3::origin(), a, n, big, r, d, band).unwrap();
    let mut body = Body::<T>::new();
    let seed = body.mvfs(carrier.eval(T::zero()), true).unwrap();
    let mirror = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Torus {
                    center: WallTorus::Mirror.center(Point3::origin(), n, d),
                    axis: a * f(sigma),
                    major_radius: big,
                    minor_radius: r,
                    u_ref: n,
                },
                sense: true,
            },
        )
        .unwrap();
    let made = body
        .mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart(mirror),
                carrier,
                param_start: T::zero(),
                param_end: T::tau(),
            },
            FaceSurface::Inherit,
            tol,
        )
        .unwrap_or_else(|e| panic!("axis {sigma}: the closed oval edge: {e}"));
    (body, [made.he_plus, made.he_minus])
}

fn a_whole_spiric_oval_closes_one_edge_on_its_mirror_torus<T: AtRestPolicy + Bounds>() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for sigma in [1.0, -1.0] {
        let (mut body, hes) = mirror_oval::<T>(sigma);
        topo::mint_pcurves(&mut body, tol)
            .unwrap_or_else(|e| panic!("axis {sigma}: the mint refuses: {e}"));
        for he in hes {
            let row = body
                .pcurve(he)
                .unwrap_or_else(|| panic!("axis {sigma}: {he:?} has no row"));
            let Pcurve::Spiric {
                image: SpiricImage::Wall { sense, torus, .. },
                ..
            } = *row.pcurve()
            else {
                panic!("axis {sigma}: a wall row: {:?}", row.pcurve());
            };
            assert_eq!(torus, WallTorus::Mirror, "axis {sigma}: the mirror named");
            assert!(
                sense.lo() == sigma && sense.hi() == sigma,
                "axis {sigma}: the sense is the chart axis's, exactly: {sense:?}"
            );
        }
        let findings = topo::pcurves::validate_pcurves(&body, band);
        assert!(
            findings.is_empty(),
            "axis {sigma}: tier 3 reads the body clean: {findings:?}"
        );
    }
}

/// The quadrature lane reads the wall terms of the carrier's OWN torus,
/// so it refuses the mirror wall typed — the `QuadratureUnsupported`
/// arm naming the mirror — rather than integrate the face against a
/// torus it is not on. Pinned at `f64`; the arm reads no scalar.
#[test]
fn the_quadrature_lane_refuses_a_mirror_wall_typed() {
    let tol = Tol::witness();
    for sigma in [1.0, -1.0] {
        let (mut body, _) = mirror_oval::<f64>(sigma);
        topo::mint_pcurves(&mut body, tol).unwrap();
        // Raw door: the measured body is a body tier 3 cannot measure (VolumeUncomputable), whose typed refusal is the subject.
        let shown = match topo::test_support::measure_unvalidated(
            &body,
            tol,
            topo::test_support::pinned_resolution(tol),
        ) {
            Ok(m) => panic!(
                "axis {sigma}: integrated a mirror wall: {:?}",
                m.enclosure.enclosure()
            ),
            Err(e) => format!("{e:?}"),
        };
        assert!(
            shown.contains("QuadratureUnsupported") && shown.contains("mirror torus"),
            "axis {sigma}: the typed mirror-wall refusal: {shown}"
        );
    }
}

/// **The oval on its mirror torus mirrors.** Across each coordinate
/// plane, at each chart axis, the image's wall images are restated for
/// the mapped carrier (whose stand-off negates) and tier 3 reads the
/// image as it reads the source: the same structural verdict, and every
/// stored row clean.
fn the_oval_on_its_mirror_torus_mirrors<T: AtRestPolicy + Bounds>() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for sigma in [1.0, -1.0] {
        let (mut body, _) = mirror_oval::<T>(sigma);
        topo::mint_pcurves(&mut body, tol).unwrap();
        for (plane, normal) in [
            ("x", Vec3::<T>::unit_x()),
            ("y", Vec3::<T>::unit_y()),
            ("z", Vec3::<T>::unit_z()),
        ] {
            let at = format!("axis {sigma}, across {plane} = 0");
            let reflection = topo::Reflection::across(Point3::origin(), normal, tol).unwrap();
            let image = topo::mirror_body(&body, &reflection, tol)
                .unwrap_or_else(|e| panic!("{at}: the mirror refuses: {e}"));
            assert_eq!(
                topo::validate(&image),
                topo::validate(&body),
                "{at}: the image's structural verdict is the source's"
            );
            assert!(
                image.pcurves().next().is_some(),
                "{at}: the image stores its wall rows"
            );
            let findings = topo::pcurves::validate_pcurves(&image, band);
            assert!(
                findings.is_empty(),
                "{at}: tier 3 reads the image clean: {findings:?}"
            );
        }
    }
}

/// One row per scalar.
macro_rules! rows_at {
    ($module:ident, $scalar:ty) => {
        mod $module {
            #[test]
            fn a_whole_spiric_oval_closes_one_edge_on_its_mirror_torus() {
                super::a_whole_spiric_oval_closes_one_edge_on_its_mirror_torus::<$scalar>();
            }

            #[test]
            fn the_oval_on_its_mirror_torus_mirrors() {
                super::the_oval_on_its_mirror_torus_mirrors::<$scalar>();
            }
        }
    };
}

rows_at!(at_f64, f64);
rows_at!(at_interval, geom_core::Interval);
