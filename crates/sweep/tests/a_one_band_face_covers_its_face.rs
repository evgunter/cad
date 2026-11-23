//! **A full revolve's one-band curved wall covers its face, not its
//! carrier.** The wall is one face whose seam is its wrap edge, so its
//! loop walks a rim, the seam out to a pole tip and back, or both rims
//! with the seam between them. `topo::cover::face_cover` reads that loop's
//! side with the tips it holds (`geom_brep::props::boundary_material_sign_holding`),
//! as tier 3 does, and its window, which the box cull reads: a cap's box
//! is floored at its rim, a zone's and a torus band's lie between their
//! rims. A cover that read the rim alone would keep the whole ball or
//! tube, and every row below would see the carrier's far side.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use topo::Body;

/// The y-range of the cover of `body`'s one face on a carrier `kind`
/// names.
fn cover_y(label: &str, body: &Body<f64>, kind: fn(&Surface<f64>) -> bool) -> (f64, f64) {
    let band = Band::linear(Tol::witness()).unwrap();
    let faces: Vec<_> = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| kind(body.face_surface(k).unwrap()))
        .collect();
    let [face] = faces.as_slice() else {
        panic!("{label}: {} faces of the kind, one expected", faces.len())
    };
    let cover = body
        .face_cover(*face, band)
        .unwrap_or_else(|e| panic!("{label}: no cover: {e}"));
    let b = cover.aabb(0.0);
    (b.min_y, b.max_y)
}

fn sphere(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Sphere { .. })
}

fn torus(s: &Surface<f64>) -> bool {
    matches!(s, Surface::Torus { .. })
}

/// The slack a cover's box may take past the face's own extent: far
/// below every gap the rows measure against the carrier.
const SLACK: f64 = 1e-6;

/// The bulge of a CCW unit-circle arc from latitude `a` to `b`.
fn bulge(a: f64, b: f64) -> f64 {
    ((b - a) / 4.0).tan()
}

/// The unit ball's cap `y ≥ h` (or `y ≤ h` below), a disc closing it.
fn cap(h: f64, north: bool) -> Body<f64> {
    let s = (1.0 - h * h).sqrt();
    let rim = h.asin();
    let verts = if north {
        vec![
            (Point2::new(0.0, h), 0.0),
            (Point2::new(s, h), bulge(rim, core::f64::consts::FRAC_PI_2)),
            (Point2::new(0.0, 1.0), 0.0),
        ]
    } else {
        vec![
            (
                Point2::new(0.0, -1.0),
                bulge(-core::f64::consts::FRAC_PI_2, rim),
            ),
            (Point2::new(s, h), 0.0),
            (Point2::new(0.0, h), 0.0),
        ]
    };
    sweep::test_support::revolved_about_y(verts, Revolution::Full, Tol::witness())
}

#[test]
fn a_one_band_cap_covers_from_its_rim_to_its_pole() {
    let h = 0.5;
    let (lo, hi) = cover_y("north cap", &cap(h, true), sphere);
    assert!(lo >= h - SLACK, "north cap's floor {lo}, its rim at {h}");
    assert!(hi <= 1.0 + SLACK, "north cap's top {hi}");
    let (lo, hi) = cover_y("south cap", &cap(-h, false), sphere);
    assert!(
        hi <= -h + SLACK,
        "south cap's ceiling {hi}, its rim at {}",
        -h
    );
    assert!(lo >= -1.0 - SLACK, "south cap's bottom {lo}");
}

/// The unit ball's zone `a ≤ y ≤ b`, two discs closing it.
#[test]
fn a_one_band_zone_covers_between_its_rims() {
    let (a, b) = (-0.3_f64, 0.6_f64);
    let (sa, sb) = ((1.0 - a * a).sqrt(), (1.0 - b * b).sqrt());
    let zone = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, a), 0.0),
            (Point2::new(sa, a), bulge(a.asin(), b.asin())),
            (Point2::new(sb, b), 0.0),
            (Point2::new(0.0, b), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let (lo, hi) = cover_y("zone", &zone, sphere);
    assert!(
        lo >= a - SLACK && hi <= b + SLACK,
        "zone's cover y ∈ [{lo}, {hi}], its rims at {a} and {b}"
    );
}

/// A torus band: the quarter of the circle of radius 0.5 about
/// `(2, 0.5)` from `(2, 0)` out to `(2.5, 0.5)`, closed by a radial and an
/// axial segment, revolved about y. The band lies at `y ∈ [0, 0.5]`, its
/// tube reaches `y = 1`.
#[test]
fn a_one_band_torus_wall_covers_its_band() {
    let band = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(2.0, 0.0), (core::f64::consts::PI / 8.0).tan()),
            (Point2::new(2.5, 0.5), 0.0),
            (Point2::new(2.0, 0.5), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let (lo, hi) = cover_y("torus band", &band, torus);
    assert!(
        lo >= -SLACK && hi <= 0.5 + SLACK,
        "torus band's cover y ∈ [{lo}, {hi}], the band's own [0, 0.5]"
    );
}

/// A frustum: the segment from `(2, 0)` to `(1, 1)` revolved about y,
/// its apex at `y = 2`. Its loop winds the axis with the seam between
/// its rims and closes in the chart clear of the apex, so its cover
/// holds no apex.
#[test]
fn a_one_band_frustum_covers_short_of_its_apex() {
    let frustum = sweep::test_support::revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let (lo, hi) = cover_y("frustum", &frustum, |s| matches!(s, Surface::Cone { .. }));
    assert!(
        lo >= -SLACK && hi <= 1.0 + SLACK,
        "frustum's cover y ∈ [{lo}, {hi}], its rims at 0 and 1, its apex at 2"
    );
}
