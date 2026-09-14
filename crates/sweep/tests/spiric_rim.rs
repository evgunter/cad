//! **The exact spiric rim carrier** — the partial-revolve torus rim
//! under `shell`, minted as `Curve3::Spiric` by the offset-axial door.
//!
//! The rows here are the carrier's own: the closed forms against both
//! implicit forms, the speed and extent bounds, the box, the kind's
//! census refusals through public doors, and the re-pose parity of the
//! minted rim. The elbow rows that MOVE doors stay in their own suites
//! (`torax_axial`, `verbs_shell`, `torax_interval`, `shell7_seam_corner`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::{Curve3, Surface};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::path::{Open, Start};
use profile::{ArcSweep, Center, Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, ShellError, transform_rigid};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

/// The klein elbow of `torax_axial`: a disc of radius `r` centred
/// `R = 1.2` off the axis, revolved a quarter turn.
fn klein_elbow(r: f64) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(-r, 0.0), 1.0),
            ProfileVertex::new(p2(r, 0.0), 1.0),
        ])],
    )
    .validate(tol())
    .expect("the elbow's cross-section validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(1.2, 0.0),
            dir: Vec2::new(0.0, -1.0),
        },
        Revolution::Partial(-core::f64::consts::FRAC_PI_2),
        tol(),
    )
    .expect("the elbow revolves")
    .body
}

/// The tour's torus-walled vessel meridian (`demos/tour/src/torusvessel.rs`,
/// the BELLIED centre), spelled from the same stations so the sectioned
/// vessel's door is measured here on the scene's own body.
fn vessel_quarter() -> Body<f64> {
    let (r_foot, r_band, r_neck) = (5.0 / 64.0, 9.0 / 64.0, 7.0 / 64.0);
    let (y_foot, y_shoulder, y_mouth) = (4.0 / 64.0, 12.0 / 64.0, 24.0 / 64.0);
    let (h_tube, r_bellied) = (8.0 / 64.0, 6.0 / 64.0);
    let lp: ProfileLoop<f64> = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(r_foot, 0.0), tol())
        .expect("the base disc")
        .line_to(p2(r_foot, y_foot), tol())
        .expect("the foot")
        .line_to(p2(r_band, y_foot), tol())
        .expect("the lower shoulder")
        .arc_to(
            Center {
                c: p2(r_bellied, h_tube),
                winding: ArcSweep::Ccw,
                p: p2(r_band, y_shoulder),
            },
            tol(),
        )
        .expect("the band")
        .line_to(p2(r_neck, y_shoulder), tol())
        .expect("the upper shoulder")
        .line_to(p2(r_neck, y_mouth), tol())
        .expect("the neck")
        .line_to(p2(0.0, y_mouth), tol())
        .expect("the mouth disc")
        .line_to(Start, tol())
        .expect("the axis closes the meridian")
        .into();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

/// The chart moves `shell` would make for an inward wall `t`, one per
/// surface (`torax_axial::hollow_moves`, at any deciding scalar).
fn hollow_moves<T: geom_core::Real>(body: &Body<T>, t: T) -> Vec<topo::ChartMove<T>> {
    let mut charts: Vec<(topo::SurfaceKey, Vec<topo::FaceKey>)> = Vec::new();
    for (k, f) in body.faces() {
        match charts.iter_mut().find(|(s, _)| *s == f.surface) {
            Some((_, v)) => v.push(k),
            None => charts.push((f.surface, vec![k])),
        }
    }
    charts
        .into_iter()
        .map(|(_, faces)| {
            let sense = body.get_face(faces[0]).expect("face").sense;
            topo::ChartMove {
                faces,
                distance: if sense { -t } else { t },
            }
        })
        .collect()
}

/// The elbow's cavity through the axial door — the body `shell` builds
/// and stops on at tier 3, taken BEFORE tier 3 so its carriers can be
/// read.
fn elbow_cavity(r: f64, t: f64) -> (Body<f64>, Body<f64>) {
    let elbow = klein_elbow(r);
    let mut cavity = elbow.clone();
    let band = Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(&elbow, t), band, tol())
        .expect("the elbow's corners solve and its rims mint");
    (elbow, cavity)
}

/// Every spiric carrier of a body with its span.
fn spiric_edges(body: &Body<f64>) -> Vec<(topo::EdgeKey, Curve3<f64>, (f64, f64))> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve)?.certified()?;
            matches!(c.carrier(), Curve3::Spiric { .. })
                .then(|| (k, c.carrier().clone(), c.params()))
        })
        .collect()
}

/// The two moved surfaces a spiric rim separates: its torus and its
/// plane, read from the edge's own faces.
fn rim_surfaces(body: &Body<f64>, edge: topo::EdgeKey) -> (Surface<f64>, Surface<f64>) {
    let e = body.get_edge(edge).expect("edge");
    let face_of = |he| {
        body.get_loop(body.get_half_edge(he).expect("he").parent_loop)
            .expect("loop")
            .face
    };
    let surf = |f| {
        body.get_surface(body.get_face(f).expect("face").surface)
            .expect("surface")
            .clone()
    };
    let (a, b) = (surf(face_of(e.he_plus)), surf(face_of(e.he_minus)));
    match (&a, &b) {
        (Surface::Torus { .. }, Surface::Plane { .. }) => (a, b),
        (Surface::Plane { .. }, Surface::Torus { .. }) => (b, a),
        other => panic!("a spiric rim separates a torus and a plane, got {other:?}"),
    }
}

/// **Row 1 — residuals against BOTH implicit forms.** Each minted rim,
/// sampled at 10⁴ parameters over its whole period, lies on the moved
/// torus and in the moved cap plane at rounding — on the elbow's
/// numbers (`R = 1.2`, `r = 0.275`, `t = 0.05`) and the two-arc
/// torus's (`R = 2`, `r = 0.5`). The other oval passes this row (it is
/// the same section) and is killed by the endpoint meter, row 4; an
/// `offset` sign error in `eval` puts the plane residual at `2|d|`.
#[test]
fn the_minted_rim_lies_on_both_implicit_forms() {
    for (r, t) in [(0.275, 0.05), (0.5, 0.05)] {
        let body = if r == 0.5 {
            two_arc_torus(2.0, r)
        } else {
            klein_elbow(r)
        };
        let mut cavity = body.clone();
        let band = Band::linear(tol()).expect("band");
        topo::offset_charts_together(&mut cavity, &hollow_moves(&body, t), band, tol())
            .expect("the rims mint");
        let rims = spiric_edges(&cavity);
        assert_eq!(
            rims.len(),
            4,
            "a partial revolve of a disc has four rim edges"
        );
        for (edge, carrier, _) in &rims {
            let (torus, plane) = rim_surfaces(&cavity, *edge);
            let (mut worst_t, mut worst_p) = (0.0_f64, 0.0_f64);
            for i in 0..10_000 {
                let v = f64::from(i) * TAU / 10_000.0;
                let p = carrier.eval(v);
                worst_t = worst_t.max(geom_brep::implicit_residual(&torus, p).abs());
                worst_p = worst_p.max(geom_brep::implicit_residual(&plane, p).abs());
            }
            println!("[spiric] r = {r}: torus residual {worst_t:e}, plane residual {worst_p:e}");
            assert!(worst_t <= 4e-16, "torus residual {worst_t}");
            assert!(worst_p <= 4e-16, "plane residual {worst_p}");
        }
    }
}

/// `shell7_seam_corner`'s partial two-arc torus: the quarter turn of a
/// circle profile of radius `r` centred `R` off the axis, revolved
/// about `+y` through the origin.
fn two_arc_torus(big_r: f64, r: f64) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(big_r + r, 0.0), 1.0),
            ProfileVertex::new(p2(big_r - r, 0.0), 1.0),
        ])],
    )
    .validate(tol())
    .expect("the two-arc profile validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    )
    .expect("the torus revolves")
    .body
}

/// **Row 2 — `edge_extent` below the sampled point-set diameter on
/// every sub-span.** The certified lever `max(chord, r(1 − cos Δv/2))`
/// never exceeds the sampled diameter of the arc it describes; a
/// larger arm (the mutant `major_radius` in place of `minor_radius`)
/// exceeds it on the short spans.
#[test]
fn the_edge_extent_stays_below_the_sampled_diameter() {
    let (_, cavity) = elbow_cavity(0.275, 0.05);
    let (_, carrier, (t0, t1)) = spiric_edges(&cavity).remove(0);
    for n in [1_usize, 2, 4, 8, 16, 64] {
        let step = (t1 - t0) / n as f64;
        for k in 0..n {
            let (a, b) = (t0 + step * k as f64, t0 + step * (k + 1) as f64);
            let chord = carrier.eval(a).distance(carrier.eval(b));
            let extent = geom_brep::edge_extent(&carrier, a, b, chord);
            let samples: Vec<Point3<f64>> = (0..=200)
                .map(|i| carrier.eval(a + (b - a) * f64::from(i) / 200.0))
                .collect();
            let mut diameter = 0.0_f64;
            for p in &samples {
                for q in &samples {
                    diameter = diameter.max(p.distance(*q));
                }
            }
            assert!(
                extent <= diameter + 1e-15,
                "span [{a}, {b}]: extent {extent} exceeds the sampled diameter {diameter}"
            );
        }
    }
}

/// **Row 3 — the box.** 10⁴ sampled points of each minted rim lie
/// inside its certified box on every axis, through the boolean's own
/// reader (`edge_box_rule` → `spiric_arc_aabb`) as well as `geom`'s
/// door directly.
#[test]
fn the_box_contains_every_sample_of_the_minted_rim() {
    let (_, cavity) = elbow_cavity(0.275, 0.05);
    for (_, carrier, (t0, t1)) in spiric_edges(&cavity) {
        let b = geom::curves::boxes::conic_arc_aabb(
            &carrier,
            t0,
            t1,
            carrier.eval(t0),
            carrier.eval(t1),
        )
        .expect("a spiric boxes");
        for i in 0..10_000 {
            let v = f64::from(i) * TAU / 10_000.0;
            let p = carrier.eval(v);
            assert!(
                b.min_x <= p.x
                    && p.x <= b.max_x
                    && b.min_y <= p.y
                    && p.y <= b.max_y
                    && b.min_z <= p.z
                    && p.z <= b.max_z,
                "sample {p:?} escapes {b:?}"
            );
        }
    }
}

/// **Row 7 — rigid re-pose parity.** The cavity's minted spiric
/// carriers re-posed by `transform` equal, field by field within one
/// ulp, the carriers of the re-posed operand's own cavity — the
/// lune's re-pose row, on the kind-changing mint.
#[test]
fn the_minted_rim_survives_a_rigid_re_pose() {
    let (elbow, cavity) = elbow_cavity(0.275, 0.05);
    let map = Affine3::rotation_about_axis(
        Point3::new(0.25, -0.5, 0.125),
        Vec3::new(1.0, 0.0, 0.0),
        0.7,
    );
    assert_eq!(topo::validate_geometric_structural(&cavity, tol()), Ok(()));
    let posed_after = transform_rigid(&cavity, &map, tol()).expect("the cavity re-poses");
    let posed_first = transform_rigid(&elbow, &map, tol()).expect("the operand re-poses");
    let mut offset_after = posed_first.clone();
    let band = Band::linear(tol()).expect("band");
    topo::offset_charts_together(
        &mut offset_after,
        &hollow_moves(&posed_first, 0.05),
        band,
        tol(),
    )
    .expect("the posed elbow's rims mint");
    let mut want = spiric_edges(&posed_after);
    let got = spiric_edges(&offset_after);
    assert_eq!((want.len(), got.len()), (4, 4));
    let ulp = |x: f64| (x.abs() * f64::EPSILON).max(f64::MIN_POSITIVE);
    for (_, g, _) in got {
        let Curve3::Spiric {
            center: gc,
            axis: ga,
            u_ref: gn,
            major_radius: gr,
            minor_radius: gm,
            offset: gd,
        } = g
        else {
            unreachable!()
        };
        let i = want
            .iter()
            .position(|(_, w, _)| matches!(w, Curve3::Spiric { center, axis, u_ref, .. }
                if center.distance(gc) <= 1e-12 && (*axis - ga).norm() <= 1e-12 && (*u_ref - gn).norm() <= 1e-12))
            .unwrap_or_else(|| panic!("no re-posed twin for {g:?}"));
        let (_, w, _) = want.remove(i);
        let Curve3::Spiric {
            center: wc,
            axis: wa,
            u_ref: wn,
            major_radius: wr,
            minor_radius: wm,
            offset: wd,
        } = w
        else {
            unreachable!()
        };
        for (a, b) in [
            (gc.x, wc.x),
            (gc.y, wc.y),
            (gc.z, wc.z),
            (ga.x, wa.x),
            (ga.y, wa.y),
            (ga.z, wa.z),
            (gn.x, wn.x),
            (gn.y, wn.y),
            (gn.z, wn.z),
            (gr, wr),
            (gm, wm),
            (gd, wd),
        ] {
            assert!(
                (a - b).abs() <= ulp(a),
                "field {a} vs {b} differs by more than one ulp"
            );
        }
    }
}

/// **Row 11 — the census refusals reachable through public doors**,
/// on the elbow's cavity: the boolean operand gate, the mesh's trimmed
/// lane (its torus/plane roster is the MESH frontier,
/// `work/issues/trimmed-tessellation-lacks-torus-and-plane-arms.md`),
/// and the STEP writer (the export spline is the spiric unit's second
/// PR). The props doors are the closing measurement row below.
#[test]
fn the_census_refusals_through_public_doors() {
    let (_, cavity) = elbow_cavity(0.275, 0.05);
    let other = klein_elbow(0.1);
    let e = topo::union(&cavity, &other, tol()).expect_err("the boolean fence refuses the kind");
    assert!(
        matches!(e, topo::BooleanError::CurvedEdgeUnsupported { .. }),
        "the operand gate names the spiric edge, got {e:?}"
    );
    let e =
        mesh::tessellate(&cavity, 1e-3, tol()).expect_err("no trimmed lane for the torus chart");
    let mesh::TessellateError::UnsupportedCurve { note, .. } = &e else {
        panic!("the trimmed frontier names its roster, got {e:?}");
    };
    assert!(
        note.contains("trimmed-tessellation-lacks-torus-and-plane-arms"),
        "got {note}"
    );
    let e = step_export::step_string(&cavity, &step_export::StepOptions::default(), tol())
        .expect_err("no STEP entity for a spiric yet");
    assert!(
        matches!(e, step_export::StepExportError::UnsupportedCurve { kind, .. } if kind.starts_with("spiric")),
        "the writer names the spiric, got {e:?}"
    );
}

/// **The closing measurement — where the elbow stands after the
/// carrier.** `shell` walks the whole hollow — corners, the
/// kind-changing carrier mint, both endpoint meters, the midpoint
/// meter, `restate` with no re-author (the rims are
/// `Intersection`/`Derived`), certification at the minor radius,
/// insertion, pcurves (the torus wall uncached, typed) — and tier 3's
/// checks 1–6 pass; **check 7 refuses** at the props inventory: the
/// cavity's torus wall is visited before its caps in arena order, so
/// the payload is `torus_boundary`'s `NotIsoRectangle { "torus
/// boundary edge is not a circle" }`; a cap visited first would read
/// `loop_vector_area`'s `Unimplemented`. Same door as the lune's
/// (`NotIsoRectangle { "props_band_coplanar" }`), different premise.
#[test]
fn the_elbow_stops_at_the_props_door() {
    let elbow = klein_elbow(0.275);
    let e = topo::shell(&elbow, 0.05, tol())
        .expect_err("tier 3 needs the volume the props lane cannot give");
    println!("[spiric] the elbow's door: {e:?}");
    let ShellError::NotValid { errors } = e else {
        panic!("the hollow must reach tier 3 and stop at the props inventory, got {e:?}");
    };
    assert!(
        matches!(
            errors[..],
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    source: geom_brep::PropsError::NotIsoRectangle {
                        what: "torus boundary edge is not a circle"
                    },
                    ..
                },
            }]
        ),
        "check 7 at the torus wall's boundary parse, got {errors:?}"
    );
}

/// **The sectioned vessel stands at the same door** — the tour's
/// `torusvessel` wall 1 on the scene's own body: a quarter turn of the
/// bellied meridian hollows to tier 3 and stops at the torus wall's
/// boundary parse, exactly as the elbow does.
#[test]
fn the_sectioned_vessel_stops_at_the_same_door() {
    let quarter = vessel_quarter();
    assert_eq!(topo::validate_geometric(&quarter, tol()), Ok(()));
    let e = topo::shell(&quarter, 1.0 / 128.0, tol()).expect_err("the same props door");
    println!("[spiric] the sectioned vessel's door: {e:?}");
    let ShellError::NotValid { errors } = e else {
        panic!("the hollow must reach tier 3, got {e:?}");
    };
    assert!(
        matches!(
            errors[..],
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    source: geom_brep::PropsError::NotIsoRectangle {
                        what: "torus boundary edge is not a circle"
                    },
                    ..
                },
            }]
        ),
        "check 7 at the torus wall's boundary parse, got {errors:?}"
    );
}

#[cfg(feature = "interval")]
mod interval_rows {
    use geom_core::{Bounds, Interval};

    use super::*;

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    /// **Row 8 — the `Interval` lane.** The elbow's cavity at the
    /// certified scalar: every new `decide` site executes, the minted
    /// rims are spirics, and each encloses its f64 twin — `eval` at a
    /// bracketed `v` contains the f64 point. An escalation at a strict
    /// band is the certified scalar's honest answer and is pinned as
    /// such.
    #[test]
    fn interval_the_minted_rim_encloses_its_f64_twin() {
        let tol = Tol::witness();
        let r = 0.275_f64;
        let profile = Profile::new(
            SketchPlane::<Interval>::xy(),
            vec![ProfileLoop::new(vec![
                ProfileVertex::new(Point2::new(iv(-r), iv(0.0)), iv(1.0)),
                ProfileVertex::new(Point2::new(iv(r), iv(0.0)), iv(1.0)),
            ])],
        )
        .validate(tol)
        .expect("validates");
        let body = revolve(
            &profile,
            RevolveAxis {
                origin: Point2::new(iv(1.2), iv(0.0)),
                dir: Vec2::new(iv(0.0), iv(-1.0)),
            },
            Revolution::Partial(iv(-core::f64::consts::FRAC_PI_2)),
            tol,
        )
        .expect("revolves")
        .body;
        let moves: Vec<topo::ChartMove<Interval>> = super::hollow_moves(&body, iv(0.05));
        let mut cavity = body.clone();
        let band = Band::linear(tol).expect("band");
        match topo::offset_charts_together(&mut cavity, &moves, band, tol) {
            Ok(()) => {}
            Err(topo::ReplaceFaceError::Escalated { source })
                if tol.eps() < geom_core::tolerance::DEFAULT_EPS =>
            {
                test_utils::vacuity::stood_down(
                    &format!("the elbow's interval rim, eps = {:e}", tol.eps()),
                    &format!("the certified scalar escalated ({source:?}) before the rims minted"),
                );
                return;
            }
            Err(e) => panic!("the elbow's rims mint at the certified scalar: {e:?}"),
        }
        let (_, f64_cavity) = super::elbow_cavity(r, 0.05);
        let f64_rims = super::spiric_edges(&f64_cavity);
        let mut n = 0;
        for (_, e) in cavity.edges() {
            let c = cavity
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .expect("certified");
            let Curve3::Spiric { center, .. } = c.carrier() else {
                continue;
            };
            n += 1;
            let (t0, t1) = c.params();
            let twin = f64_rims
                .iter()
                .find(|(_, w, _)| {
                    matches!(w, Curve3::Spiric { center: wc, .. }
                    if center.x.lo() <= wc.x && wc.x <= center.x.hi())
                })
                .map(|(_, w, _)| w)
                .expect("an f64 twin");
            for k in 0..=8 {
                let v = t0 + (t1 - t0) * iv(f64::from(k) / 8.0);
                let p = c.carrier().eval(v);
                let q = twin.eval(v.lo() + (v.hi() - v.lo()) * 0.5);
                assert!(
                    p.x.lo() <= q.x && q.x <= p.x.hi(),
                    "x: {:?} vs {}",
                    p.x,
                    q.x
                );
                assert!(
                    p.y.lo() <= q.y && q.y <= p.y.hi(),
                    "y: {:?} vs {}",
                    p.y,
                    q.y
                );
                assert!(
                    p.z.lo() <= q.z && q.z <= p.z.hi(),
                    "z: {:?} vs {}",
                    p.z,
                    q.z
                );
            }
        }
        assert_eq!(n, 4, "four spiric rims at the certified scalar");
    }
}
