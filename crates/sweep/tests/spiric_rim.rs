//! **The exact spiric rim carrier** — the partial-revolve torus rim
//! under `shell`, minted as `Curve3::Spiric` by the offset-axial door.
//!
//! The rows here are the carrier's own: the closed forms against both
//! implicit forms, the speed and extent bounds, the box, the kind's
//! census refusals through public doors, and the re-pose parity of the
//! minted rim — on the SECTIONED VESSEL (the tour's `torusvessel` wall
//! 1), the partial revolve whose rims mint through a public door and
//! reach tier 3. The klein ELBOW's rims mint too, but its hollow stops
//! one door earlier, at its equator seams' re-author
//! (`the_elbow_stops_at_its_seam_reauthor` below), so its cavity is not
//! readable through any public door at this head. The elbow rows that
//! MOVE doors stay in their own suites (`torax_axial`, `verbs_shell`,
//! `torax_interval`, `shell7_seam_corner`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::{Curve3, Surface};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::path::{Open, Start};
use profile::{ArcSweep, Center, Profile, ProfileLoop, SketchPlane, test_support::bulge_loop};
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
        vec![bulge_loop(vec![(p2(-r, 0.0), 1.0), (p2(r, 0.0), 1.0)])],
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

/// The sectioned vessel's cavity through the axial door — the body
/// `shell` builds and stops on at tier 3, taken BEFORE tier 3 so its
/// carriers can be read.
pub(crate) fn vessel_cavity(t: f64) -> (Body<f64>, Body<f64>) {
    let quarter = vessel_quarter();
    let mut cavity = quarter.clone();
    let band = Band::linear(tol()).expect("band");
    topo::offset_charts_together(&mut cavity, &hollow_moves(&quarter, t), band, tol())
        .expect("the vessel's corners solve and its rims mint");
    (quarter, cavity)
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
/// torus and in the moved cap plane at rounding — the sectioned vessel
/// (`R = 6/64`, `r = 5/64`) at two wall thicknesses. The elbow's and
/// the two-arc torus's numbers are not readable through a public door
/// at this head (their hollows refuse at the equator seams' re-author
/// before tier 3, below). The other oval passes this row (it is the
/// same section) and is killed by the endpoint meter, row 4; an
/// `offset` sign error in `eval` puts the plane residual at `2|d|`.
#[test]
fn the_minted_rim_lies_on_both_implicit_forms() {
    for t in [1.0 / 128.0, 1.0 / 256.0] {
        let (_, cavity) = vessel_cavity(t);
        let rims = spiric_edges(&cavity);
        assert_eq!(rims.len(), 2, "the band's two rims, one per moved cap");
        for (edge, carrier, _) in &rims {
            let (torus, plane) = rim_surfaces(&cavity, *edge);
            let (mut worst_t, mut worst_p) = (0.0_f64, 0.0_f64);
            for i in 0..10_000 {
                let v = f64::from(i) * TAU / 10_000.0;
                let p = carrier.eval(v);
                worst_t = worst_t.max(geom_brep::implicit_residual(&torus, p).abs());
                worst_p = worst_p.max(geom_brep::implicit_residual(&plane, p).abs());
            }
            println!("[spiric] t = {t}: torus residual {worst_t:e}, plane residual {worst_p:e}");
            assert!(worst_t <= 2e-15, "torus residual {worst_t}");
            assert!(worst_p <= 2e-15, "plane residual {worst_p}");
        }
    }
}

/// **Row 2 — `edge_extent` below the sampled point-set diameter on
/// every sub-span**, on the vessel's minted rim. The certified lever
/// `max(chord, r′(1 − cos Δv/2))` never exceeds the sampled diameter
/// of the arc it describes. This fixture CANNOT see the wrong-arm
/// mutant (`major_radius` in place of `minor_radius`): the fold only
/// overtakes the chord on the LONG spans, where
/// `R(1 − cos Δv/2) > 2r′·sin(Δv/2)` needs `R ≳ 2r′`, and the vessel's
/// `R/r′ = 1.33`. The row below, at the elbow's ratio `R/r = 5.3`, is
/// the one that reds under it.
#[test]
fn the_edge_extent_stays_below_the_sampled_diameter() {
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
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

/// **Row 2′ — the extent arm at a ratio that can see it.** A spiric
/// on the elbow's numbers (`R = 1.2`, `r = 0.225`, `d = 0.05`, so
/// `R/r = 5.3`), minted through the kind's door: over the whole
/// oval and its halves the certified lever stays below the sampled
/// diameter, and the mutant that reads `major_radius` as the fold's
/// arm exceeds it by `≈ 0.75` m on the full period (`R(1 − cos π)
/// = 2.4` against a diameter of `≈ 1.65`). The lever is what
/// transversality's dihedral pass classifies against, so a larger arm
/// would let a sliver classify transverse — this is the row that pins
/// it below the diameter.
#[test]
fn the_edge_extent_arm_is_the_minor_radius_at_the_elbows_ratio() {
    let band = Band::linear(tol()).expect("band");
    let carrier = Curve3::spiric(
        Point3::new(0.3, -0.2, 0.7),
        Vec3::unit_z(),
        Vec3::unit_x(),
        1.2,
        0.225,
        0.05,
        band,
    )
    .expect("the elbow's numbers are in regime");
    for (a, b) in [
        (0.0, TAU),
        (0.0, core::f64::consts::PI),
        (0.7, 2.9),
        (2.0, 5.5),
    ] {
        let chord = carrier.eval(a).distance(carrier.eval(b));
        let extent = geom_brep::edge_extent(&carrier, a, b, chord);
        let samples: Vec<Point3<f64>> = (0..=400)
            .map(|i| carrier.eval(a + (b - a) * f64::from(i) / 400.0))
            .collect();
        let mut diameter = 0.0_f64;
        for p in &samples {
            for q in &samples {
                diameter = diameter.max(p.distance(*q));
            }
        }
        println!("[spiric] span [{a}, {b}]: extent {extent}, sampled diameter {diameter}");
        assert!(
            extent <= diameter + 1e-15,
            "span [{a}, {b}]: extent {extent} exceeds the sampled diameter {diameter}"
        );
    }
}

/// **Row 3 — the box.** 10⁴ sampled points of each minted rim lie
/// inside its certified box on every axis, through `geom`'s door
/// (`conic_arc_aabb` → `spiric_arc_aabb`). The boolean's own reader
/// (`edge_box_rule` → `edge_box`) is crate-private and no public door
/// reaches its spiric arm at this head; `topo`'s in-src row
/// `the_spiric_edge_box_and_reach_contain_a_dense_sample` executes it
/// on a hand-built sector.
#[test]
fn the_box_contains_every_sample_of_the_minted_rim() {
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
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
    let (quarter, cavity) = vessel_cavity(1.0 / 128.0);
    let map = Affine3::rotation_about_axis(
        Point3::new(0.25, -0.5, 0.125),
        Vec3::new(1.0, 0.0, 0.0),
        0.7,
    );
    assert_eq!(topo::validate_geometric_structural(&cavity, tol()), Ok(()));
    let posed_after = transform_rigid(&cavity, &map, tol()).expect("the cavity re-poses");
    let posed_first = transform_rigid(&quarter, &map, tol()).expect("the operand re-poses");
    let mut offset_after = posed_first.clone();
    let band = Band::linear(tol()).expect("band");
    topo::offset_charts_together(
        &mut offset_after,
        &hollow_moves(&posed_first, 1.0 / 128.0),
        band,
        tol(),
    )
    .expect("the posed vessel's rims mint");
    let mut want = spiric_edges(&posed_after);
    let got = spiric_edges(&offset_after);
    assert_eq!((want.len(), got.len()), (2, 2));
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
/// on the vessel's cavity: the boolean operand gate, the mesh's trimmed
/// lane (its torus/plane roster is the MESH frontier,
/// `work/issues/trimmed-tessellation-lacks-torus-and-plane-arms.md`),
/// and the STEP writer (the export spline is the spiric unit's second
/// PR). The props doors are the closing measurement row below.
#[test]
fn the_census_refusals_through_public_doors() {
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
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
/// carrier.** §4 of the spec predicted tier 3's check 7. Measured: the
/// elbow's rims MINT (both endpoint meters and the midpoint meter pass
/// — the mutants that flip the oval or the reach guard's sign refuse
/// on the rim edge, `torax_axial`), and the hollow then refuses one
/// door EARLIER than predicted, on the EQUATOR SEAMS: a disc revolved
/// a quarter turn carries its two profile vertices as
/// `RevolvedPoint`-declared chart seams between the two torus faces,
/// and `restate` re-authors a declaration in its own sketch plane —
/// whose start corner the moved cap has displaced OFF that plane by
/// `t`, so `offset_axial_reauthor_plane` refuses typed. The spec's §0
/// authority measurement covered the rims (`Intersection`/`Derived`,
/// no re-author) and not the seams (`Chart`/`Declared`). This is the
/// pre-registered STOP-1 shape (a re-author refusal off §4's chain);
/// the row pins the measured door, and what to do about a declared
/// seam whose corner leaves its sketch plane is the orchestrator's
/// question, not this unit's. The sectioned vessel next door has no
/// such seam and reaches check 7.
#[test]
fn the_elbow_stops_at_its_seam_reauthor() {
    let elbow = klein_elbow(0.275);
    let e = topo::shell(&elbow, 0.05, tol()).expect_err("the equator seams' re-author");
    println!("[spiric] the elbow's door: {e:?}");
    let ShellError::Face { error, .. } = e else {
        panic!("the offset door's refusal, got {e:?}");
    };
    let topo::ReplaceFaceError::TogetherAxialEdge { what, .. } = *error else {
        panic!("the re-author's out-of-plane refusal, got {error:?}");
    };
    assert_eq!(
        what,
        "a revolved point's moved corner stands out of the family's own sketch plane, so the same rotation does not pass through it"
    );
}

/// **The sectioned vessel reaches the props door** — the tour's
/// `torusvessel` wall 1 on the scene's own body: a quarter turn of the
/// bellied meridian hollows through the kind-changing mint, both
/// endpoint meters, certification at the minor radius, insertion and
/// pcurves (the torus wall uncached, typed) to tier 3, whose checks
/// 1–6 pass; **check 7 refuses** `VolumeUncomputable` — at a CAP,
/// visited before the torus wall in arena order, so the payload is
/// `loop_vector_area`'s `Unimplemented` (the oval's area is an
/// elliptic integral). The torus wall behind it is NOT
/// `torus_boundary`'s parse: `topo`'s face flux routes a loop carrying
/// a spiric to the quadrature lane, whose chart gate refuses
/// `QuadratureUnsupported { "conic trim on a cone/sphere/torus chart
/// …" }` (`props.rs`, `cut_face_rounds`) — read from the source, since
/// no public door visits the wall before the cap; the closed-form
/// parse's own answer for the wall is the row below. The spec named
/// the wall's parse first; the run shows the cap. Same door as the
/// lune's (`NotIsoRectangle { "props_meridian_great" }`), different
/// premise.
#[test]
fn the_sectioned_vessel_stops_at_the_props_door() {
    let quarter = vessel_quarter();
    assert_eq!(topo::validate_geometric(&quarter, tol()), Ok(()));
    let e = topo::shell(&quarter, 1.0 / 128.0, tol()).expect_err("tier 3's volume");
    println!("[spiric] the sectioned vessel's door: {e:?}");
    let ShellError::NotValid { errors } = e else {
        panic!("the hollow must reach tier 3, got {e:?}");
    };
    assert!(
        matches!(
            errors[..],
            [topo::ValidationError::VolumeUncomputable {
                source: topo::MassPropsError::Face {
                    source: geom_brep::PropsError::Unimplemented,
                    ..
                },
                ..
            }]
        ),
        "check 7 at a cap's loop area, got {errors:?}"
    );
}

/// **The torus wall's closed-form parse names its spiric rim** — the
/// one public door that reaches `torus_boundary`'s named `Spiric` arm:
/// the vessel cavity's torus face's loop, read through
/// `topo::props::loop_edges`, handed to `geom_brep::curved_face` (the
/// closed-form flux door `topo`'s face flux takes for an UNtrimmed
/// loop). `NotIsoRectangle { "torus boundary edge is not a circle" }`,
/// the arm the props door would raise if the quadrature lane did not
/// take the face first.
#[test]
fn the_torus_walls_closed_form_parse_names_its_spiric_rim() {
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    let (edge, _, _) = spiric_edges(&cavity).remove(0);
    let e = cavity.get_edge(edge).expect("edge");
    let face_of = |he| {
        cavity
            .get_loop(cavity.get_half_edge(he).expect("he").parent_loop)
            .expect("loop")
            .face
    };
    let torus_face = [face_of(e.he_plus), face_of(e.he_minus)]
        .into_iter()
        .find(|f| {
            matches!(
                cavity.get_surface(cavity.get_face(*f).expect("face").surface),
                Some(Surface::Torus { .. })
            )
        })
        .expect("a spiric rim bounds a torus face");
    let face = cavity.get_face(torus_face).expect("face");
    let surface = cavity.get_surface(face.surface).expect("surface");
    let (edges, _) = topo::props::loop_edges(&cavity, face.outer).expect("the loop reads");
    let band = Band::linear(tol()).expect("band");
    let e = geom_brep::curved_face(surface, &edges, face.sense, band)
        .expect_err("the rim-or-meridian parse has no arm for a spiric");
    assert_eq!(
        e,
        geom_brep::PropsError::NotIsoRectangle {
            what: "torus boundary edge is not a circle"
        }
    );
}

/// **`classify_kind` opens no iso side for a spiric rim**: the vessel
/// cavity's torus chart, asked about each of its spiric rims through
/// the public `topo::classify_kind`, answers `None` — the coherence
/// walk's `NonIsoCarrier` — where a rim circle would answer `Rim` and a
/// meridian `Meridian`.
#[test]
fn a_spiric_rim_is_no_iso_traversal_of_its_torus_chart() {
    let (_, cavity) = vessel_cavity(1.0 / 128.0);
    let rims = spiric_edges(&cavity);
    assert_eq!(rims.len(), 2);
    for (edge, _, _) in rims {
        let e = cavity.get_edge(edge).expect("edge");
        let curve = cavity
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .expect("certified");
        let face_of = |he| {
            cavity
                .get_loop(cavity.get_half_edge(he).expect("he").parent_loop)
                .expect("loop")
                .face
        };
        let chart_surface = [face_of(e.he_plus), face_of(e.he_minus)]
            .into_iter()
            .map(|f| {
                cavity
                    .get_surface(cavity.get_face(f).expect("face").surface)
                    .expect("surface")
            })
            .find(|s| matches!(s, Surface::Torus { .. }))
            .expect("a spiric rim bounds a torus face");
        let chart = topo::Chart::of(chart_surface).expect("the torus charts");
        assert!(
            topo::classify_kind(&chart, curve).is_none(),
            "a spiric opens no iso side"
        );
    }
}

/// **`split_edge` meters a spiric's interiority at its minor radius**:
/// the vessel cavity's rim split at its mid-parameter yields two
/// certified spiric pieces whose spans partition the original, on the
/// same carrier. The only door that reads the kind's `split_edge`
/// meter arm.
#[test]
fn a_spiric_rim_splits_at_its_mid_parameter() {
    let (_, mut cavity) = vessel_cavity(1.0 / 128.0);
    let (edge, carrier, (t0, t1)) = spiric_edges(&cavity).remove(0);
    let mid = (t0 + t1) * 0.5;
    let made = cavity
        .split_edge(edge, mid, tol())
        .expect("a spiric rim splits at an interior parameter");
    let piece = |k: topo::EdgeKey| {
        let e = cavity.get_edge(k).expect("edge");
        let c = cavity
            .get_curve_geom(e.curve)
            .and_then(|g| g.certified())
            .expect("certified");
        assert!(
            matches!(c.carrier(), Curve3::Spiric { .. }),
            "the piece keeps the kind"
        );
        assert_eq!(
            format!("{:?}", c.carrier()),
            format!("{carrier:?}"),
            "the piece keeps the carrier"
        );
        c.params()
    };
    let (a0, a1) = piece(edge);
    let (b0, b1) = piece(made.new_edge);
    let (lo, hi) = if a0 < b0 {
        ((a0, a1), (b0, b1))
    } else {
        ((b0, b1), (a0, a1))
    };
    assert!(
        (lo.0 - t0).abs() <= 1e-15 && (hi.1 - t1).abs() <= 1e-15,
        "the spans cover the original"
    );
    assert!(
        (lo.1 - mid).abs() <= 1e-15 && (hi.0 - mid).abs() <= 1e-15,
        "the cut is at the midpoint"
    );
    let split_point = *cavity
        .get_point(cavity.get_vertex(made.vertex).expect("vertex").point)
        .expect("point");
    assert!(
        split_point.distance(carrier.eval(mid)) <= 1e-15,
        "the new vertex is on the carrier"
    );
}

mod interval_rows {
    use geom_core::{Bounds, Interval, Real};

    use super::*;

    fn iv(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    /// The vessel's meridian as a raw loop at any deciding scalar — the
    /// band arc as its bulge (`tan(θ/4) = 1/2`, the 3-4-5 arc), so the
    /// interval and f64 twins are built by one spelling.
    fn vessel_loop<T: geom_core::Real>(iv: &impl Fn(f64) -> T) -> ProfileLoop<T> {
        let p = |x: f64, y: f64| Point2::new(iv(x), iv(y));
        bulge_loop(vec![
            (p(0.0, 0.0), iv(0.0)),
            (p(5.0 / 64.0, 0.0), iv(0.0)),
            (p(5.0 / 64.0, 4.0 / 64.0), iv(0.0)),
            (p(9.0 / 64.0, 4.0 / 64.0), iv(0.5)),
            (p(9.0 / 64.0, 12.0 / 64.0), iv(0.0)),
            (p(7.0 / 64.0, 12.0 / 64.0), iv(0.0)),
            (p(7.0 / 64.0, 24.0 / 64.0), iv(0.0)),
            (p(0.0, 24.0 / 64.0), iv(0.0)),
        ])
    }

    fn vessel_at<T: geom_core::Decide + geom_brep::PcurveFittedLane>(
        iv: &impl Fn(f64) -> T,
    ) -> Body<T> {
        let tol = Tol::witness();
        let profile = Profile::new(SketchPlane::<T>::xy(), vec![vessel_loop(iv)])
            .validate(tol)
            .expect("the vessel meridian validates");
        revolve(
            &profile,
            RevolveAxis {
                origin: Point2::new(iv(0.0), iv(0.0)),
                dir: Vec2::new(iv(0.0), iv(1.0)),
            },
            Revolution::Partial(iv(core::f64::consts::FRAC_PI_2)),
            tol,
        )
        .expect("the vessel revolves")
        .body
    }

    /// **Row 8 — the `Interval` lane.** The sectioned vessel's cavity at
    /// the certified scalar: every new `decide` site executes, the
    /// minted rims are spirics, and each encloses its f64 twin — `eval`
    /// at a bracketed `v` contains the f64 point. An escalation at a
    /// strict band is the certified scalar's honest answer and is pinned
    /// as such.
    #[test]
    fn interval_the_minted_rim_encloses_its_f64_twin() {
        let tol = Tol::witness();
        let body = vessel_at(&iv);
        let moves: Vec<topo::ChartMove<Interval>> = super::hollow_moves(&body, iv(1.0 / 128.0));
        let mut cavity = body.clone();
        let band = Band::linear(tol).expect("band");
        match topo::offset_charts_together(&mut cavity, &moves, band, tol) {
            Ok(()) => {}
            Err(topo::ReplaceFaceError::Escalated { source })
                if tol.eps() < geom_core::tolerance::DEFAULT_EPS =>
            {
                test_utils::vacuity::stood_down(
                    &format!("the vessel's interval rim, eps = {:e}", tol.eps()),
                    &format!("the certified scalar escalated ({source:?}) before the rims minted"),
                );
                return;
            }
            Err(e) => panic!("the vessel's rims mint at the certified scalar: {e:?}"),
        }
        let f64_body = vessel_at(&|x| x);
        let mut f64_cavity = f64_body.clone();
        topo::offset_charts_together(
            &mut f64_cavity,
            &super::hollow_moves(&f64_body, 1.0 / 128.0),
            Band::linear(tol).expect("band"),
            tol,
        )
        .expect("the f64 twin's rims mint");
        let f64_rims = super::spiric_edges(&f64_cavity);
        let mut n = 0;
        for (_, e) in cavity.edges() {
            let c = cavity
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .expect("certified");
            let Curve3::Spiric { offset, .. } = c.carrier() else {
                continue;
            };
            n += 1;
            let (t0, t1) = c.params();
            let twin = f64_rims
                .iter()
                .find(|(_, w, _)| {
                    matches!(w, Curve3::Spiric { offset: wd, .. } if offset.lo() <= *wd && *wd <= offset.hi())
                })
                .map(|(_, w, _)| w)
                .expect("an f64 twin with the same stand-off");
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
        assert_eq!(n, 2, "two spiric rims at the certified scalar");
    }
}
