//! **The football revolves.** An arc of a spindle-torus sheet's circle
//! from one of its axis points to the other, closed by its chord on the
//! axis and revolved about that chord: the lemon (`R < 0`), whose two
//! apexes are vertices, and the apple (`0 < R < r`), whose two dimple
//! apexes are. The horn (`R = r`) is reached at its one cusp from one
//! side as a needle tip; from both sides it pinches.
//!
//! Each body is measured against a closed form of revolution written
//! independently of the kernel's: Pappus on the circular segment the
//! profile bounds, and on its arc, for the football; the integral of
//! `πρ(z)²` along the axis for the needle.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::validated;
use core::f64::consts::PI;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use sweep::{Revolution, RevolveAxis, RevolveError, revolve};

fn y_axis() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// The football of the sheet `(R, r)` about the sketch `y` axis: the
/// arc from the lower apex to the upper through the circle's widest
/// point, closed down the axis.
fn football(big: f64, r: f64) -> profile::ProfileLoop<f64> {
    let h = ((r - big) * (r + big)).sqrt();
    let v_star = (-big / r).acos();
    bulge_loop(vec![
        (Point2::new(0.0, -h), (v_star / 2.0).tan()),
        (Point2::new(0.0, h), 0.0),
    ])
}

/// The football's volume and area by Pappus. The axis cuts the circle
/// at `d = |R|` from its centre, parting it into a segment of half
/// angle `α = acos(d/r)` across the axis from the centre and the rest:
/// the lemon is that segment, the apple the rest. The segment's area is
/// `r²α − d√(r² − d²)`, its centroid `2(r² − d²)^{3/2}/(3A)` from the
/// centre; an arc of half angle `θ` has its centroid `r·sin θ/θ` from it.
fn pappus(big: f64, r: f64) -> (f64, f64) {
    let d = big.abs();
    let alpha = (d / r).acos();
    let segment = r * r * alpha - d * (r * r - d * d).sqrt();
    let centroid = 2.0 * (r * r - d * d).powf(1.5) / (3.0 * segment);
    if big < 0.0 {
        let arc = 2.0 * r * alpha;
        (
            2.0 * PI * segment * (centroid - d),
            2.0 * PI * arc * (r * alpha.sin() / alpha - d),
        )
    } else {
        // The disc's moment about the axis less the segment's, whose
        // centroid is across the axis.
        let moment = PI * r * r * big - segment * (big - centroid);
        let rest = PI - alpha;
        let arc = 2.0 * r * rest;
        (
            2.0 * PI * moment,
            2.0 * PI * arc * (big + r * alpha.sin() / rest),
        )
    }
}

/// The body revolved, its datum read, meshed closed, through tier 3, and
/// measured against `(volume, area)`.
fn holds(what: &str, lp: profile::ProfileLoop<f64>, sheet: f64, expected: (f64, f64)) {
    holds_by(what, lp, Revolution::Full, (2, sheet), expected);
}

/// [`holds`] revolved `by`, with `faces` sheet faces of major radius
/// `sheet`.
fn holds_by(
    what: &str,
    lp: profile::ProfileLoop<f64>,
    by: Revolution<f64>,
    (faces, sheet): (usize, f64),
    (volume, area): (f64, f64),
) {
    let tol = Tol::witness();
    let built =
        revolve(&validated(vec![lp]), y_axis(), by, tol).unwrap_or_else(|e| panic!("{what}: {e}"));
    let sheets: Vec<f64> = built
        .body
        .faces()
        .filter_map(|(_, f)| match built.body.get_surface(f.surface) {
            Some(geom::Surface::Spindle { major_radius, .. }) => Some(*major_radius),
            _ => None,
        })
        .collect();
    assert!(
        sheets.len() == faces && sheets.iter().all(|&m| (m - sheet).abs() <= 1e-15),
        "{what}: the sheet faces' major radii {sheets:?}, not {faces} of {sheet}"
    );
    let delta = 1e-2;
    let mesh = mesh::tessellate(&built.body, delta, tol)
        .unwrap_or_else(|e| panic!("{what}: tessellate: {e:?}"));
    mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}: the mesh: {e:?}"));
    crate::common::differential::assert_meshed_geometry(what, &built.body, &mesh, delta);
    let body = topo::AtRestBody::validate(built.body, tol)
        .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
    let props = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{what}: {e:?}"));
    let got = props.enclosure;
    assert!(
        (got.volume - volume).abs() <= 1e-10 * volume + got.volume_pad,
        "{what}: volume {} against {volume} (pad {})",
        got.volume,
        got.volume_pad
    );
    assert!(
        (got.surface_area - area).abs() <= 1e-10 * area + got.area_pad,
        "{what}: area {} against {area} (pad {})",
        got.surface_area,
        got.area_pad
    );
}

/// **The football and the apple build through the revolve door** with
/// their apexes as vertices: two sheet faces, each a π-band between the
/// two meridians that end at both apexes (the pole form), meshed closed
/// with a fan at each apex, through tier 3, and measuring their exact
/// volume and area.
#[test]
fn the_football_and_the_apple_revolve() {
    for (what, big, r) in [
        ("the lemon", -0.5, 1.0),
        ("a slim lemon", -0.9, 1.0),
        ("the apple", 0.4, 1.0),
        ("a small apple", 0.3, 0.7),
    ] {
        holds(what, football(big, r), big, pappus(big, r));
    }
}

/// **A horn's cusp reached from one side is a needle tip**: the horn of
/// radius 1 (`R = r`, its centre `(1, 0)` and its cusp at the origin),
/// its arc from the cusp to `(1, 1)`, closed by a line to the axis and
/// down it. The face is the sheet's band `π/2 ≤ v ≤ π` between the cusp
/// and a disc. The arc's centre is read a hair from `r`, the revolve
/// decides it on the axis, and the sheet is stored a horn exactly, its
/// one cusp at the arc's end.
///
/// `ρ(z) = 1 − √(1 − z²)` on `0 ≤ z ≤ 1`, so `π∫ρ² dz = π(5/3 − π/2)`;
/// the area is the band's `2π(π/2 − 1)` and the disc's `π`.
#[test]
fn a_horn_reached_at_its_cusp_from_one_side_is_a_needle_tip() {
    let needle = bulge_loop(vec![
        (Point2::new(0.0, 0.0), -(PI / 8.0).tan()),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    holds(
        "the needle",
        needle,
        1.0,
        (
            PI * (5.0 / 3.0 - PI / 2.0),
            2.0 * PI * (PI / 2.0 - 1.0) + PI,
        ),
    );
}

/// **A horn's cusp reached from both sides pinches**: the horn's whole
/// circle, its vertex at the cusp, would revolve to a solid that meets
/// itself at one point, which the full revolve refuses at the vertex.
#[test]
fn a_horn_reached_at_its_cusp_from_both_sides_pinches() {
    let quarter = -(PI / 8.0).tan();
    let pinch = bulge_loop(vec![
        (Point2::new(0.0, 0.0), quarter),
        (Point2::new(1.0, 1.0), quarter),
        (Point2::new(2.0, 0.0), quarter),
        (Point2::new(1.0, -1.0), quarter),
    ]);
    let got = revolve(
        &validated(vec![pinch]),
        y_axis(),
        Revolution::Full,
        Tol::witness(),
    );
    assert!(
        matches!(
            got,
            Err(RevolveError::NonManifoldAxisContact { loop_index: 0, .. })
        ),
        "{:?}",
        got.map(|_| ())
    );
}

/// **A football re-stored on the other sheet is refused at its edges.**
/// Both sheets of a spindle torus share their apexes, so a body whose
/// sheet faces are re-stored on the other sheet (`R → −R`, the same axis,
/// apexes and tube) has every vertex where it was and every apex at a
/// vertex: no apex reading can tell it from the football. What tier 3
/// refuses is its edges, off the sheet the faces now name. The apexes'
/// own reading is `topo`'s `chart::tests` row on a sheet's poles.
#[test]
fn a_football_restored_on_the_other_sheet_is_refused_at_its_edges() {
    let tol = Tol::witness();
    for (what, big, r) in [("the lemon", -0.5, 1.0), ("the apple", 0.4, 1.0)] {
        let built = revolve(
            &validated(vec![football(big, r)]),
            y_axis(),
            Revolution::Full,
            tol,
        )
        .unwrap();
        let mut body = built.body;
        let keys: Vec<_> = body
            .surfaces()
            .filter(|(_, s)| matches!(s, geom::Surface::Spindle { .. }))
            .map(|(k, _)| k)
            .collect();
        assert!(!keys.is_empty(), "{what}: no sheet");
        for key in keys {
            let Some(geom::Surface::Spindle {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }) = body.get_surface(key).cloned()
            else {
                unreachable!("the key was read off a sheet")
            };
            body = body
                .with_surface_moved_for_tests(
                    key,
                    geom::Surface::Spindle {
                        center,
                        axis,
                        major_radius: -major_radius,
                        minor_radius,
                        u_ref,
                    },
                )
                .unwrap();
        }
        let errors = topo::AtRestBody::validate(body, tol).expect_err(what);
        let off_sheet = |e: &topo::ValidationError| {
            matches!(
                e,
                topo::ValidationError::EdgeCertification {
                    error: geom_brep::CertifyError::CarrierOffChart {
                        chart: geom::SurfaceKind::Spindle,
                        ..
                    },
                    ..
                }
            )
        };
        assert!(
            errors.iter().any(off_sheet)
                && errors.iter().all(|e| off_sheet(e)
                    || matches!(
                        e,
                        topo::ValidationError::Pcurve { .. }
                            | topo::ValidationError::JoinUndecidedAtRest { .. }
                    )),
            "{what}: {errors:?}"
        );
    }
}

/// **A football revolved part way is its lune**: one sheet face between
/// two meridians that end at both apexes, each shared with a plane cap,
/// meshed closed, through tier 3, and measuring `θ/2π` of the
/// football's volume and of its sheet's area, and two caps of the
/// profile's area: the circular segment on a lemon, the rest of the
/// disc on an apple. Each cap meridian is certified on the sheet up to
/// its apexes, where the sheet's two sheets meet.
#[test]
fn a_football_revolved_part_way_is_its_lune() {
    for (what, big, r, theta) in [
        ("the lemon", -0.5, 1.0, 2.0_f64),
        ("the apple", 0.4, 1.0, 2.0),
        ("a slim lemon", -0.9, 1.0, 1.0),
        ("a lemon past a half turn", -0.5, 1.0, 4.5),
        ("a small apple", 0.3, 0.7, -2.5),
    ] {
        let (volume, area) = pappus(big, r);
        let d = big.abs();
        let segment = r * r * (d / r).acos() - d * (r * r - d * d).sqrt();
        let cap = if big < 0.0 {
            segment
        } else {
            PI * r * r - segment
        };
        let share = theta.abs() / (2.0 * PI);
        holds_by(
            what,
            football(big, r),
            Revolution::Partial(theta),
            (1, big),
            (share * volume, share * area + 2.0 * cap),
        );
    }
}

/// The sheet's arc band `v₀ ≤ v ≤ v₁` of the circle `(R, r)`:
/// `∫ πρ² dz` with `ρ = R + r cos v`, `z = r sin v`, and `∫ 2πρ ds`,
/// each in closed form. Signed with the arc's direction; a body's volume
/// is the absolute sum round its closed profile, where a line square to
/// the axis and the axis itself add none.
fn arc_band(big: f64, r: f64, v0: f64, v1: f64) -> (f64, f64) {
    let vol = |v: f64| {
        let (s, c) = v.sin_cos();
        PI * r * (big * big * s + big * r * (v + s * c) + r * r * (s - s.powi(3) / 3.0))
    };
    let area = |v: f64| 2.0 * PI * r * (big * v + r * v.sin());
    (vol(v1) - vol(v0), area(v1) - area(v0))
}

fn apex_angle(big: f64, r: f64) -> f64 {
    (-big / r).acos()
}

/// The football's arc drawn as two halves meeting at its widest point.
fn football_halves(big: f64, r: f64) -> profile::ProfileLoop<f64> {
    let (h, b) = (
        ((r - big) * (r + big)).sqrt(),
        (apex_angle(big, r) / 4.0).tan(),
    );
    bulge_loop(vec![
        (Point2::new(0.0, -h), b),
        (Point2::new(big + r, 0.0), b),
        (Point2::new(0.0, h), 0.0),
    ])
}

/// Built, through tier 3, and measured, with no mesh.
fn measures(what: &str, lp: profile::ProfileLoop<f64>, ax: RevolveAxis<f64>, (v, a): (f64, f64)) {
    let tol = Tol::witness();
    let built = revolve(&validated(vec![lp]), ax, Revolution::Full, tol)
        .unwrap_or_else(|e| panic!("{what}: {e}"));
    let body = topo::AtRestBody::validate(built.body, tol)
        .unwrap_or_else(|e| panic!("{what}: tier 3: {e:?}"));
    let g = topo::mass_properties(&body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{what}: {e:?}"))
        .enclosure;
    assert!(
        (g.volume - v).abs() <= 1e-10 * v + g.volume_pad,
        "{what}: volume {} against {v} (pad {})",
        g.volume,
        g.volume_pad
    );
    assert!(
        (g.surface_area - a).abs() <= 1e-10 * a + g.area_pad,
        "{what}: area {} against {a} (pad {})",
        g.surface_area,
        g.area_pad
    );
}

/// Meshed closed as well ([`measures`]).
fn meshes_and_measures(
    what: &str,
    lp: profile::ProfileLoop<f64>,
    ax: RevolveAxis<f64>,
    exact: (f64, f64),
) {
    let tol = Tol::witness();
    let built = revolve(&validated(vec![lp.clone()]), ax, Revolution::Full, tol)
        .unwrap_or_else(|e| panic!("{what}: {e}"));
    let delta = 1e-2;
    let mesh = mesh::tessellate(&built.body, delta, tol)
        .unwrap_or_else(|e| panic!("{what}: tessellate: {e:?}"));
    mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{what}: the mesh: {e:?}"));
    crate::common::differential::assert_meshed_geometry(what, &built.body, &mesh, delta);
    measures(what, lp, ax, exact);
}

/// **More bodies with an apex as a vertex**, each meshed closed, through
/// tier 3 and measuring its closed form of revolution ([`arc_band`]):
/// - half a football (a bullet), its arc from an apex to the widest
///   point and a disc there, either way up;
/// - the football's arc drawn as two halves;
/// - half a football under a hemisphere of its widest radius, a sheet
///   and a sphere meeting on the rim;
/// - the football about a tilted axis off the origin.
#[test]
fn bodies_with_an_apex_as_a_vertex_revolve() {
    for (big, r) in [(-0.5_f64, 1.0_f64), (-0.9, 1.0), (0.4, 1.0), (0.3, 0.7)] {
        let vs = apex_angle(big, r);
        let (h, w, b) = (r * vs.sin(), big + r, (vs / 4.0).tan());
        for upper in [false, true] {
            let (lp, (v, a)) = if upper {
                (
                    bulge_loop(vec![
                        (Point2::new(0.0, 0.0), 0.0),
                        (Point2::new(w, 0.0), b),
                        (Point2::new(0.0, h), 0.0),
                    ]),
                    arc_band(big, r, 0.0, vs),
                )
            } else {
                (
                    bulge_loop(vec![
                        (Point2::new(0.0, -h), b),
                        (Point2::new(w, 0.0), 0.0),
                        (Point2::new(0.0, 0.0), 0.0),
                    ]),
                    arc_band(big, r, -vs, 0.0),
                )
            };
            meshes_and_measures(
                &format!("a bullet R {big} r {r} upper {upper}"),
                lp,
                y_axis(),
                (v.abs(), a + PI * w * w),
            );
        }
    }
    for big in [-0.5_f64, 0.4] {
        let vs = apex_angle(big, 1.0);
        let (h, w, b) = (vs.sin(), big + 1.0, (vs / 4.0).tan());
        meshes_and_measures(
            &format!("the football in halves R {big}"),
            football_halves(big, 1.0),
            y_axis(),
            pappus(big, 1.0),
        );
        let (v, a) = arc_band(big, 1.0, -vs, 0.0);
        meshes_and_measures(
            &format!("half a football under a hemisphere R {big}"),
            bulge_loop(vec![
                (Point2::new(0.0, -h), b),
                (Point2::new(w, 0.0), (PI / 8.0).tan()),
                (Point2::new(0.0, w), 0.0),
            ]),
            y_axis(),
            (v.abs() + 2.0 / 3.0 * PI * w.powi(3), a + 2.0 * PI * w * w),
        );
        let (o, d) = ((0.3, -0.2), (0.6, 0.8));
        let at =
            |rho: f64, z: f64| Point2::new(o.0 + rho * d.1 + z * d.0, o.1 - rho * d.0 + z * d.1);
        meshes_and_measures(
            &format!("the football about a tilted axis R {big}"),
            bulge_loop(vec![(at(0.0, -h), (vs / 2.0).tan()), (at(0.0, h), 0.0)]),
            RevolveAxis {
                origin: Point2::new(o.0, o.1),
                dir: Vec2::new(d.0, d.1),
            },
            pappus(big, 1.0),
        );
    }
}

/// **A football cavity**: a cylinder of radius 1.5 and height 3 with the
/// football hollowed out on its axis. The sheet faces are π-bands with
/// the material outside them, the bit clear, whose boundary encodes no
/// side: a side read off one reading of them, as if they encoded one,
/// is the inverted face this refuses.
#[test]
fn a_football_cavity_revolves() {
    for big in [-0.5_f64, 0.4] {
        let vs = apex_angle(big, 1.0);
        let h = vs.sin();
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -1.5), 0.0),
            (Point2::new(1.5, -1.5), 0.0),
            (Point2::new(1.5, 1.5), 0.0),
            (Point2::new(0.0, 1.5), 0.0),
            (Point2::new(0.0, h), -(vs / 2.0).tan()),
            (Point2::new(0.0, -h), 0.0),
        ]);
        let (v, a) = pappus(big, 1.0);
        let (cyl_v, cyl_a) = (PI * 2.25 * 3.0, 2.0 * PI * 2.25 + 2.0 * PI * 1.5 * 3.0);
        meshes_and_measures(
            &format!("the football cavity R {big}"),
            lp,
            y_axis(),
            (cyl_v - v, cyl_a + a),
        );
    }
}

/// **What a sheet face's own boundary encodes.** The football's π-bands
/// part the closed sheet in two, so their boundary encodes no side and
/// the bit is the face's one reading; a bullet's band and the needle's,
/// closed by a rim, encode the side the bit states.
#[test]
fn a_sheet_faces_boundary_encodes_its_side_where_a_rim_closes_it() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let needle = bulge_loop(vec![
        (Point2::new(0.0, 0.0), -(PI / 8.0).tan()),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let bullet = |big: f64| {
        let vs = apex_angle(big, 1.0);
        bulge_loop(vec![
            (Point2::new(0.0, -vs.sin()), (vs / 4.0).tan()),
            (Point2::new(big + 1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ])
    };
    for (what, lp, encodes) in [
        ("the lemon", football(-0.5, 1.0), false),
        ("the apple", football(0.4, 1.0), false),
        ("a lemon bullet", bullet(-0.5), true),
        ("an apple bullet", bullet(0.4), true),
        ("the needle", needle, true),
    ] {
        let body = revolve(&validated(vec![lp]), y_axis(), Revolution::Full, tol)
            .unwrap()
            .body;
        let mut sheets = 0;
        for (_, f) in body.faces() {
            let s = body.get_surface(f.surface).unwrap();
            if !matches!(s, geom::Surface::Spindle { .. }) {
                continue;
            }
            sheets += 1;
            let (edges, _) = topo::props::loop_edges(&body, f.outer).unwrap();
            let got = geom_brep::props::boundary_material_sign_loops(s, &[&edges], band);
            let want = if encodes {
                geom_brep::props::MaterialSign::Encoded(if f.sense {
                    geom_core::Sign::Positive
                } else {
                    geom_core::Sign::Negative
                })
            } else {
                geom_brep::props::MaterialSign::Unencoded
            };
            assert_eq!(got, Ok(want), "{what}: sense {}", f.sense);
        }
        assert_eq!(sheets, 2, "{what}");
    }
}

/// **A football face with its bit flipped is refused**, one face or
/// both: the π-bands encode no side and no seam region reads a sheet
/// (`work/spiricarm/a-sheet-face-whose-boundary-encodes-no-side-has-no-seam-region-reading.md`),
/// so what refuses is the body: the two faces' wedge at each meridian
/// where one is flipped, a negative volume where both are.
#[test]
fn a_football_with_a_sheet_faces_bit_flipped_is_refused() {
    let tol = Tol::witness();
    for big in [-0.5_f64, 0.4] {
        let built = revolve(
            &validated(vec![football(big, 1.0)]),
            y_axis(),
            Revolution::Full,
            tol,
        )
        .unwrap();
        let sheet: Vec<_> = built.body.faces().map(|(k, _)| k).collect();
        for keys in [vec![sheet[0]], sheet.clone()] {
            let mut body = built.body.clone();
            for &k in &keys {
                body = body.flipped_face_sense_for_tests(k).unwrap();
            }
            let errors = topo::AtRestBody::validate(body, tol).expect_err("a flipped bit");
            let refused = if keys.len() == 1 {
                errors
                    .iter()
                    .all(|e| matches!(e, topo::ValidationError::LaminaWedge { .. }))
            } else {
                matches!(
                    errors.as_slice(),
                    [topo::ValidationError::NegativeVolume { .. }]
                )
            };
            assert!(refused, "R {big}, {} flipped: {errors:?}", keys.len());
        }
    }
}

/// **The wide apples and the lower needle measure**, where the mesh
/// refuses them (`work/spiricarm/a-wide-apples-tube-angle-passes-the-latitude-windows-cut.md`):
/// tier 3 and the closed form alone.
#[test]
fn the_wide_apples_and_the_lower_needle_measure() {
    measures(
        "the lower needle",
        bulge_loop(vec![
            (Point2::new(0.0, -1.0), 0.0),
            (Point2::new(1.0, -1.0), -(PI / 8.0).tan()),
            (Point2::new(0.0, 0.0), 0.0),
        ]),
        y_axis(),
        (
            PI * (5.0 / 3.0 - PI / 2.0),
            2.0 * PI * (PI / 2.0 - 1.0) + PI,
        ),
    );
    let z = geom_core::Band::linear(Tol::witness()).unwrap().zero();
    for big in [0.6_f64, 0.8, 0.99, 1.0 - (1e3 * z).max(1e-6)] {
        measures(
            &format!("a wide apple R {big}"),
            football(big, 1.0),
            y_axis(),
            pappus(big, 1.0),
        );
    }
}

/// **Near the horn, the drawn ends say which sheet**, the reach decided
/// zero either way. The band reads the arc's ends on the axis against
/// each reading's apexes, which part by `√(2r·|rc − r|)`:
/// - an apple a hair inside the horn drawn apex to apex (two half
///   arcs, `R = 1 − z/10` and `1 − z/100` for the band's zero `z`, its
///   apexes `√(z/5)` and `√(z/50)` along the axis from the centre's
///   foot, far past the band) is its own sheet, `R = rc`, through tier 3
///   and measuring the apple's closed form. Where the stored centre's
///   rounding moves those apexes past the band (at a fine ε), it refuses
///   `ApexEndOffSheet` instead;
/// - the needle, drawn to the centre's foot, is the horn (its own row);
/// - an arc `z/4` outside tangency ending on the axis `√z` from the
///   foot (within `3z/4` of the axis there), at no reading's apex,
///   refuses `ApexEndOffSheet`.
#[test]
fn near_the_horn_the_drawn_ends_say_which_sheet() {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let z = band.zero();
    for d in [z / 10.0, z / 100.0] {
        let big = 1.0 - d;
        // The stored circle's centre carries a rounding `u` of its own,
        // which moves its apexes by `u/h` at height `h = √(2d)`: past the
        // band (a fine ε), no reading holds the drawn ends.
        if f64::EPSILON / (2.0 * d).sqrt() > z / 10.0 {
            let got = revolve(
                &validated(vec![football_halves(big, 1.0)]),
                y_axis(),
                Revolution::Full,
                tol,
            );
            assert!(
                matches!(got, Err(RevolveError::ApexEndOffSheet { .. })),
                "R = 1 − {d:e}: {:?}",
                got.map(|_| ())
            );
            continue;
        }
        let built = revolve(
            &validated(vec![football_halves(big, 1.0)]),
            y_axis(),
            Revolution::Full,
            tol,
        )
        .unwrap_or_else(|e| panic!("R = 1 − {d:e}: {e}"));
        for (_, f) in built.body.faces() {
            if let Some(geom::Surface::Spindle { major_radius, .. }) =
                built.body.get_surface(f.surface)
            {
                assert!(
                    (major_radius - big).abs() <= 1e-15,
                    "R = 1 − {d:e}: stored {major_radius}"
                );
            }
        }
        measures(
            &format!("an apple at R = 1 − {d:e}"),
            football_halves(big, 1.0),
            y_axis(),
            pappus(big, 1.0),
        );
    }
    let (cx, y0) = (1.0 + 0.25 * z, z.sqrt());
    let start = Point2::new(cx - (1.0 - y0 * y0).sqrt(), y0);
    let span = PI - y0.asin() - PI / 2.0;
    let got = revolve(
        &validated(vec![bulge_loop(vec![
            (start, -(span / 4.0).tan()),
            (Point2::new(cx, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ])]),
        y_axis(),
        Revolution::Full,
        tol,
    );
    assert!(
        matches!(
            got,
            Err(RevolveError::ApexEndOffSheet {
                loop_index: 0,
                segment_index: 0
            })
        ),
        "{:?}",
        got.map(|_| ())
    );
}

/// **A horn's cusp revolved part way refuses at its cap meridians**: the
/// needle tip, and the horn's circle as four quarter arcs with a face
/// reaching the cusp from each side. A cap meets the horn along a
/// meridian that ends at the cusp, where the horn's two sheets are
/// tangent, so the sheet's distance bound reads no better than the
/// distance from the cusp over a stretch near it, and limb 2 stops past
/// the band
/// (`work/spiricarm/a-horns-cusp-revolved-part-way-refuses-at-its-cap-meridians-certificate.md`).
#[test]
fn a_horns_cusp_revolved_part_way_refuses_at_its_cap_meridians() {
    let quarter = -(PI / 8.0).tan();
    for (what, lp) in [
        (
            "the needle",
            bulge_loop(vec![
                (Point2::new(0.0, 0.0), quarter),
                (Point2::new(1.0, 1.0), 0.0),
                (Point2::new(0.0, 1.0), 0.0),
            ]),
        ),
        (
            "the horn's circle in quarters",
            bulge_loop(vec![
                (Point2::new(0.0, 0.0), quarter),
                (Point2::new(1.0, 1.0), quarter),
                (Point2::new(2.0, 0.0), quarter),
                (Point2::new(1.0, -1.0), quarter),
            ]),
        ),
    ] {
        let got = revolve(
            &validated(vec![lp]),
            y_axis(),
            Revolution::Partial(2.0),
            Tol::witness(),
        );
        assert!(
            matches!(
                got,
                Err(RevolveError::Op {
                    source: topo::EulerOpError::Certification {
                        error: geom_brep::CertifyError::AnalyticRung3(
                            geom_brep::AnalyticRung3Refusal::KernelLimit {
                                operand: geom::SurfaceKind::Spindle,
                                ..
                            }
                        )
                    }
                })
            ),
            "{what}: {:?}",
            got.map(|_| ())
        );
    }
}

/// REVIEW PROBE (PR 6521): what a part-way football costs, and whether
/// it builds, across near-horn lemons and apples, scales and turns.
#[test]
fn review_6521_probe_lune_cost() {
    let mut rows = Vec::new();
    for (big_over_r, r) in [
        (-0.5, 1.0),
        (0.4, 1.0),
        (-0.99, 1.0),
        (-0.9999, 1.0),
        (-0.999_999, 1.0),
        (0.99, 1.0),
        (0.9999, 1.0),
        (0.999_999, 1.0),
        (-0.01, 1.0),
        (0.01, 1.0),
        (-0.5, 1e3),
        (-0.5, 1e-3),
        (0.4, 1e3),
        (0.4, 1e-3),
    ] {
        for theta in [1e-3, 2.0, 6.28, -6.28] {
            let big = big_over_r * r;
            let t0 = std::time::Instant::now();
            let got = revolve(
                &validated(vec![football(big, r)]),
                y_axis(),
                Revolution::Partial(theta),
                Tol::witness(),
            );
            let dt = t0.elapsed().as_secs_f64();
            let what = match &got {
                Ok(_) => "built".to_string(),
                Err(e) => format!("{e:?}").chars().take(160).collect(),
            };
            rows.push(format!("R/r {big_over_r} r {r} θ {theta}: {dt:.3} s {what}"));
        }
    }
    for row in &rows {
        eprintln!("review probe: {row}");
    }
}

/// REVIEW PROBE (PR 6521): a near-sphere football, full and part way.
#[test]
fn review_6521_probe_near_sphere() {
    for big in [-0.2, -0.1, -0.05, -0.03, -0.02, -0.01, 0.01, 0.02, 0.05, 0.1, 0.2] {
        for by in [Revolution::Full, Revolution::Partial(2.0)] {
            let got = revolve(
                &validated(vec![football(big, 1.0)]),
                y_axis(),
                by,
                Tol::witness(),
            );
            let what = match &got {
                Ok(_) => "built".to_string(),
                Err(e) => format!("{e}").chars().take(220).collect(),
            };
            eprintln!("review probe: R {big} {by:?}: {what}");
        }
    }
}
