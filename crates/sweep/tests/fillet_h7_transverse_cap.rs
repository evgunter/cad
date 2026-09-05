//! **The ruled band ends at its transverse cap** (FILLET-H7): the
//! cylinder–plane(∥) arm's band, carved between the plane caps
//! perpendicular to its ruling.
//!
//! The consumer is a rod with a flat milled along it — `cylinder ∖ box`
//! through the public boolean door — whose two creases are straight
//! cylinder–plane edges ending in the rod's two caps. The rows here
//! pin: both creases carve in one call and each alone; the result is
//! tier-3 valid on a closed-form inventory; the volume moves by the
//! PRISM closed form `ΔV = A_section · L` (`test_support::rod_section_cut`
//! derives `A_section`); each band is the arm's exact cylinder and its
//! cut-off arcs are exact circles of the band's radius about the
//! spine's crossing of the cap, described as the band×cap intersection;
//! the trimlines are described as the band's tangent contact with the
//! support they lie in; naming is total; the same shape spelled as a
//! D-profile extrude (one 254° cap arc) carves too; an oblique cap
//! refuses typed naming the reserved run-out; the predicate's trio and
//! the lever it is metered at (the link's extent, shown through the
//! battery on one tilt at two lengths); a curved end face refuses before
//! metering; a mutant cut-off arc is refused at the attachment gate.
//!
//! The Phase-1 measurements that framed the unit stay as rows: the
//! parallel-cylinder union (the `CylinderCylinderCylinder` consumer)
//! still refuses at the boolean's curved-pierce door, and a box's single
//! edge still refuses as a run-out — the cut-off is not widened to
//! plane–plane straight edges.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec};
use geom_core::k_stats::{start_verdict_log, take_verdict_log};
use geom_core::{Band, Point2, Point3, Sign, Tol, Vec3};
use profile::{Profile, ProfileVertex, SketchPlane};
use sweep::blend::battery::{BlendRequest, RULED_END_NOT_TRANSVERSE, cap_transverse, run_battery};
use sweep::blend::{BlendError, Blended, CornerConfig, RunOutPolicy, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, assert_naming_totality, cube, revolved_about_y,
    rod_creases, rod_d_profile_at, rod_d_profile_of_length_at, rod_section_cut, rod_with_flat,
};
use sweep::{Extrusion, extrude};
use topo::query;
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, EdgeKey, FaceKey, VertexKey, mass_properties, validate_geometric};

const R: f64 = ROD_FILLET;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("closed-form props");
    assert_eq!(p.volume_pad, 0.0, "the inventory is closed-form");
    p.volume
}

/// The faces an edge separates, by surface key.
fn edge_surfaces(body: &Body<f64>, e: EdgeKey) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let ed = body.get_edge(e).unwrap();
    let surface = |he| {
        let h = body.get_half_edge(he).unwrap();
        let f = body.get_loop(h.parent_loop).unwrap().face;
        body.get_face(f).unwrap().surface
    };
    (surface(ed.he_plus), surface(ed.he_minus))
}

/// **Carve every crease of `source` and check the claims that hold for
/// any rod-with-a-flat**: census, tier 3, the prism closed form, the
/// arm's exact band, the exact cut-off arcs and their descriptions, the
/// trimlines' descriptions, naming totality.
fn carve_and_check(source: &Body<f64>, what: &str) -> Blended<f64> {
    let creases = rod_creases(source);
    assert_eq!(creases.len(), 2, "{what}: two ruling creases");
    let (v0, e0, f0) = census(source);
    let vol0 = volume(source);

    let out = fillet_edges(source, &creases, R, tol())
        .unwrap_or_else(|e| panic!("{what}: both creases carve, got {e}"));
    assert_eq!(out.blend_faces.len(), 2, "{what}: one band per crease");
    assert!(
        out.corner_faces.is_empty() && out.band_faces.is_empty(),
        "{what}: a transverse cap mints no corner patch and no closed band"
    );
    // Per crease: +4 feet −2 old vertices; +4 rim pieces +2 arcs +2
    // trimlines −1 crease −4 near pieces; +2 slivers +2 strips −1 −2.
    assert_eq!(
        census(&out.body),
        (v0 + 4, e0 + 6, f0 + 2),
        "{what}: the census delta of two cut-off bands"
    );
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));

    // The prism closed form: the band is a cylinder along the whole
    // rod, so the volume it removes is the section's cut times the
    // length, once per crease.
    let cut = 2.0 * rod_section_cut(ROD_R, ROD_FLAT, R) * ROD_L;
    let vol1 = volume(&out.body);
    assert!(
        (vol0 - vol1 - cut).abs() < 1e-12,
        "{what}: ΔV = 2·A_section·L: measured {} vs closed form {cut}",
        vol0 - vol1
    );

    let rec = out.naming.as_ref().expect("birth records");
    for (band, crease) in &rec.blends {
        // The band is the arm's exact cylinder: radius R about a spine
        // along z at the sheet crossing (x = flat − R, |y| = h).
        let s = out
            .body
            .get_surface(out.body.get_face(*band).unwrap().surface)
            .unwrap();
        let geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } = *s
        else {
            panic!("{what}: the band is a cylinder, got {s:?}");
        };
        assert_eq!(radius, R, "{what}: the band's radius is the ball's");
        assert!(
            axis.cross(Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-15,
            "{what}: spine along z"
        );
        let h = ((ROD_R - R).powi(2) - (ROD_FLAT - R).powi(2)).sqrt();
        assert!(
            (origin.x - (ROD_FLAT - R)).abs() < 1e-12 && (origin.y.abs() - h).abs() < 1e-12,
            "{what}: the spine is the sheet crossing, got {origin:?}"
        );
        let band_surface = out.body.get_face(*band).unwrap().surface;
        // Its two cut-off arcs: exact circles of radius R about the
        // spine's crossing of each cap, described as band × cap.
        let arcs: Vec<EdgeKey> = rec
            .arcs
            .iter()
            .filter(|(_, _, e)| e == crease)
            .map(|(a, _, _)| *a)
            .collect();
        assert_eq!(arcs.len(), 2, "{what}: one cut-off arc per cap");
        for a in arcs {
            let c = out
                .body
                .get_curve_geom(out.body.get_edge(a).unwrap().curve)
                .and_then(|g| g.certified())
                .expect("a certified arc");
            let Curve3::Circle {
                center,
                radius: rr,
                axis: ax,
                ..
            } = *c.carrier()
            else {
                panic!("{what}: a cut-off arc is a circle, got {:?}", c.carrier());
            };
            assert_eq!(rr, R, "{what}: the section circle has the band's radius");
            assert!(
                (center.x - origin.x).abs() < 1e-12
                    && (center.y - origin.y).abs() < 1e-12
                    && (center.z.min(ROD_L - center.z)).abs() < 1e-12,
                "{what}: the section is centred on the spine in a cap plane, got {center:?}"
            );
            assert!(
                ax.cross(axis).norm() < 1e-12,
                "{what}: the section's axis is the spine's"
            );
            let (t0, t1) = c.params();
            assert!(t1 - t0 < core::f64::consts::PI, "{what}: the arc is short");
            let EdgeDescription::Intersection { s1, s2, .. } = c.description() else {
                panic!(
                    "{what}: the arc is a transverse intersection, got {:?}",
                    c.description()
                );
            };
            let (fa, fb) = edge_surfaces(&out.body, a);
            assert!(
                (*s1 == fa && *s2 == fb) || (*s1 == fb && *s2 == fa),
                "{what}: the arc's description names its two faces' surfaces"
            );
            assert!(
                *s1 == band_surface || *s2 == band_surface,
                "{what}: the arc's description cites the band"
            );
        }
        // Its two trimlines: lines along the ruling, described as the
        // band's TANGENT contact with the support each lies in.
        let trims: Vec<(EdgeKey, FaceKey)> = rec
            .trims
            .iter()
            .filter(|(_, e, _)| e == crease)
            .map(|(t, _, f)| (*t, *f))
            .collect();
        assert_eq!(trims.len(), 2, "{what}: one trimline per support");
        for (t, support) in trims {
            let c = out
                .body
                .get_curve_geom(out.body.get_edge(t).unwrap().curve)
                .and_then(|g| g.certified())
                .expect("a certified trimline");
            let Curve3::Line { dir, .. } = *c.carrier() else {
                panic!("{what}: a trimline is a line");
            };
            assert!(
                dir.cross(axis).norm() < 1e-15,
                "{what}: the trimline runs along the ruling"
            );
            let EdgeDescription::TangentIntersection { s1, s2, .. } = c.description() else {
                panic!(
                    "{what}: a trimline is a tangent contact, got {:?}",
                    c.description()
                );
            };
            let support_surface = out.body.get_face(support).unwrap().surface;
            assert!(
                (*s1 == band_surface && *s2 == support_surface)
                    || (*s1 == support_surface && *s2 == band_surface),
                "{what}: the trimline's description cites the band and its support"
            );
        }
    }
    assert_naming_totality(source, &out, &creases, what);
    // The rows' SOURCE columns, read against the source body: a foot
    // names the crease end it retracted from and the support it lies
    // in; a cut-off arc names the crease end it cuts off and the crease
    // whose band it bounds; a fragment names a cap rim of the source.
    let ends_of = |e: EdgeKey| -> [VertexKey; 2] {
        let ed = source.get_edge(e).expect("a source edge");
        [
            source.get_half_edge(ed.he_plus).unwrap().start,
            source.half_edge_end(ed.he_plus).unwrap(),
        ]
    };
    let faces_of = |e: EdgeKey| -> [FaceKey; 2] {
        let ed = source.get_edge(e).expect("a source edge");
        let face = |he| {
            let l = source.get_half_edge(he).unwrap().parent_loop;
            source.get_loop(l).unwrap().face
        };
        [face(ed.he_plus), face(ed.he_minus)]
    };
    let crease_ends: Vec<VertexKey> = creases.iter().flat_map(|c| ends_of(*c)).collect();
    assert_eq!(rec.feet.len(), 8, "{what}: two feet per crease end");
    for (foot, v, f) in &rec.feet {
        assert!(
            crease_ends.contains(v),
            "{what}: a foot's source vertex is a crease end"
        );
        assert_ne!(foot, v, "{what}: a foot is not the vertex it retracts from");
        assert!(
            creases
                .iter()
                .any(|c| ends_of(*c).contains(v) && faces_of(*c).contains(f)),
            "{what}: a foot's support is a support of a crease ending at its vertex"
        );
    }
    assert_eq!(rec.arcs.len(), 4, "{what}: one cut-off arc per crease end");
    for (_, v, e) in &rec.arcs {
        assert!(creases.contains(e), "{what}: an arc names its crease");
        assert!(
            ends_of(*e).contains(v),
            "{what}: an arc names the end of its crease it cuts off"
        );
    }
    for (piece, src) in &rec.meridian_remnants {
        assert!(
            source.get_edge(*src).is_some(),
            "{what}: a fragment names a source rim"
        );
        assert!(
            out.body.get_edge(*piece).is_some(),
            "{what}: a fragment survives"
        );
        let rim_faces = faces_of(*src);
        assert!(
            creases.iter().any(|c| {
                let cf = faces_of(*c);
                rim_faces.iter().any(|f| cf.contains(f))
                    && ends_of(*src).iter().any(|v| ends_of(*c).contains(v))
            }),
            "{what}: a split rim shares a support and an end with a crease"
        );
    }
    out
}

/// **The rod with a flat fillets**: both creases in one call, at the
/// prism closed form, with the K funnel reached BY NAME by the new
/// predicate (the cap is decided, not assumed) at a margin of exactly
/// zero — the cap normal IS the ruling.
#[test]
fn the_rod_with_a_flat_fillets_both_creases_at_the_prism_closed_form() {
    let source = rod_with_flat(tol());
    assert_eq!(
        census(&source),
        (6, 8, 4),
        "the boolean's rod: seam-split cap arcs"
    );
    start_verdict_log();
    let _ = carve_and_check(&source, "rod ∖ box");
    let log = take_verdict_log();
    let caps: Vec<_> = log
        .iter()
        .filter(|v| v.predicate == "fillet3_cap_transverse")
        .collect();
    assert_eq!(caps.len(), 4, "four cap ends decided, one per crease end");
    assert!(
        caps.iter().all(|v| v.sign == Sign::Zero),
        "every cap is transverse: {caps:?}"
    );
}

/// **Each crease alone carves too**, at half the prism, and the other
/// crease survives untouched.
#[test]
fn one_crease_alone_carves_at_half_the_prism() {
    let source = rod_with_flat(tol());
    let creases = rod_creases(&source);
    let vol0 = volume(&source);
    for &e in &creases {
        let out = fillet_edges(&source, &[e], R, tol()).expect("one crease carves");
        assert_eq!(census(&out.body), (8, 11, 5));
        validate_geometric(&out.body, tol()).expect("tier 3");
        let cut = rod_section_cut(ROD_R, ROD_FLAT, R) * ROD_L;
        assert!(
            (vol0 - volume(&out.body) - cut).abs() < 1e-12,
            "ΔV = A_section · L"
        );
        let other = creases.iter().find(|k| **k != e).unwrap();
        assert!(
            out.body.get_edge(*other).is_some(),
            "the other crease survives"
        );
        assert_naming_totality(&source, &out, &[e], "one crease");
    }
}

/// **The same shape spelled through the extrude door** — a D-profile
/// whose cap arc sweeps past π, so the far foot's split parameter lies
/// a turn off the carrier's principal branch and the one-period window
/// picks it.
#[test]
fn the_d_profile_rod_carves_through_a_cap_arc_past_pi() {
    let source = rod_d_profile_at::<f64>(tol());
    assert_eq!(census(&source), (4, 6, 4), "one cap arc per cap, past π");
    let _ = carve_and_check(&source, "D-profile rod");
}

/// **The oblique cap refuses typed and names the reserved run-out.**
/// The rod's top is cut off by a plane tilted 0.3 rad off the ruling's
/// normal plane, so each crease's upper end is trivalent with its two
/// other edges in one plane face — the cap shape — but that face is
/// not perpendicular to the ruling: `fillet3_cap_transverse` reads a
/// definite departure and the request refuses as a run-out at that
/// vertex, with the corner recourse's "general run-outs" clause.
#[test]
fn an_oblique_cap_refuses_typed_as_the_reserved_run_out() {
    let rod = rod_d_profile_at::<f64>(tol());
    let phi = 0.3f64;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.7),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&rod, &plane, tol()).expect("the tilted cut splits");
    let SplitPart::Body(below) = &result.below else {
        panic!("the lower part carries material");
    };
    validate_geometric(below, tol()).expect("the cut rod is tier-3 valid");
    let creases = rod_creases(below);
    assert_eq!(creases.len(), 2, "the creases survive the cut");
    for e in creases {
        let err = fillet_edges(below, &[e], R, tol()).expect_err("an oblique cap refuses");
        let BlendError::UnsupportedRunOut { at, detail } = err.error else {
            panic!("the oblique cap is a run-out, got {:?}", err.error);
        };
        assert_eq!(detail, RULED_END_NOT_TRANSVERSE);
        let topo::EntityId::Vertex(v) = at else {
            panic!("the refusal names the vertex, got {at:?}");
        };
        let p = below
            .get_vertex(v)
            .and_then(|x| below.get_point(x.point))
            .unwrap();
        assert!(p.z > 0.5, "the refusing end is the oblique one, at {p:?}");
        let shown = err.to_string();
        assert!(
            shown.contains("general run-outs") && shown.contains("oblique"),
            "the sentence names the reserved run-out: {shown}"
        );
    }
}

/// **The two-tolerance trio for `fillet3_cap_transverse`** — each arm
/// reachable and distinct: a perpendicular cap is Zero, a definite
/// departure refuses as a run-out carrying the corner recourse, an
/// in-band one escalates naming the predicate with the same recourse.
#[test]
fn cap_transverse_trio_definite_pass_definite_refuse_in_band_escalate() {
    let band = Band::linear(tol()).expect("a band");
    let v = VertexKey::default();
    let tau = Vec3::new(0.0, 0.0, 1.0);
    cap_transverse(v, Vec3::new(0.0, 0.0, -1.0), tau, 1.0, band)
        .expect("a perpendicular cap is Zero");
    let phi = 0.3f64;
    let oblique = cap_transverse(v, Vec3::new(phi.sin(), 0.0, phi.cos()), tau, 1.0, band)
        .expect_err("an oblique cap refuses");
    assert!(
        matches!(oblique, BlendError::UnsupportedRunOut { .. }),
        "the oblique cap is a run-out, got {oblique:?}"
    );
    let t = 0.5 * (band.zero() + band.escalate());
    let escalated = cap_transverse(v, Vec3::new(t, 0.0, 1.0), tau, 1.0, band)
        .expect_err("an in-band cap escalates");
    let BlendError::Escalated { source, .. } = &escalated else {
        panic!("the in-band row must escalate, got {escalated:?}");
    };
    assert_eq!(source.predicate, Some("fillet3_cap_transverse"));
    // The lever matters: the same angle at a longer link is a larger
    // departure in meters, so an in-band reading at lever 1 is a
    // definite refusal at lever 1e3.
    let levered = cap_transverse(v, Vec3::new(t, 0.0, 1.0), tau, 1e3, band)
        .expect_err("levered up, the departure is definite");
    assert!(matches!(levered, BlendError::UnsupportedRunOut { .. }));
    // Both refusing arms carry one recourse.
    let (d, e) = (oblique.to_string(), escalated.to_string());
    assert!(d.contains("general run-outs"), "{d}");
    assert!(e.contains("general run-outs"), "{e}");
}

/// **The vocabulary is the ratified one and the tag maps its policy.**
#[test]
fn the_transverse_cap_names_its_policy() {
    assert_eq!(
        CornerConfig::TransverseCap.policy(),
        Some(RunOutPolicy::CutOffAtTransverseCap)
    );
    let shown = format!(
        "{} / {}",
        CornerConfig::TransverseCap,
        RunOutPolicy::CutOffAtTransverseCap
    );
    assert!(
        shown.contains("transverse cap") && shown.contains("cut the band off"),
        "{shown}"
    );
}

/// **A mutant cut-off arc is refused at the ATTACHMENT gate.**
/// Re-describing a carved arc at the wrong radius, or about the wrong
/// centre, is refused by `set_edge_curve`'s certification — the arc
/// must lie on both the band and the cap, and neither mutant does — so
/// the mutant never reaches tier 3; the untouched body stays tier-3
/// clean beside it. The spec anticipated the red at tier 3; it lands
/// one gate earlier.
#[test]
fn a_cut_off_arc_at_the_wrong_radius_or_centre_is_refused_at_the_attachment_gate() {
    let source = rod_with_flat(tol());
    let out = carve_and_check(&source, "mutant base");
    let rec = out.naming.as_ref().unwrap();
    let (arc, _, _) = rec.arcs[0];
    let c = out
        .body
        .get_curve_geom(out.body.get_edge(arc).unwrap().curve)
        .and_then(|g| g.certified())
        .unwrap()
        .clone();
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = *c.carrier()
    else {
        panic!("an arc");
    };
    let EdgeDescription::Intersection { s1, s2, witness } = c.description() else {
        panic!("a transverse intersection");
    };
    let (s1, s2, witness) = (*s1, *s2, *witness);
    let (t0, t1) = c.params();
    let mutants = [
        (
            "wrong radius",
            Curve3::Circle {
                center,
                axis,
                radius: radius * 1.05,
                u_ref,
            },
        ),
        (
            "wrong centre",
            Curve3::Circle {
                center: center + Vec3::new(0.01, 0.0, 0.0),
                axis,
                radius,
                u_ref,
            },
        ),
    ];
    for (label, carrier) in mutants {
        let mut body = out.body.clone();
        let attached = body.set_edge_curve(
            arc,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
                carrier,
                param_start: t0,
                param_end: t1,
            },
            tol(),
        );
        let refused = attached.expect_err(&format!("{label}: the attachment gate refuses"));
        assert!(
            matches!(refused, topo::EulerOpError::Certification { .. }),
            "{label}: the refusal is the certification's, got {refused:?}"
        );
        validate_geometric(&body, tol())
            .unwrap_or_else(|e| panic!("{label}: the untouched body stays tier-3 clean: {e:?}"));
    }
}

/// **Phase-1 ground, kept as pins.** The `CylinderCylinderCylinder`
/// consumer — two parallel cylinders of one height, overlapping,
/// unioned — has no body: the union refuses at the boolean's
/// curved-pierce door, so the concave ruled band has no fixture. And a
/// box's single edge is NOT a ruled link, so it still refuses as the
/// run-out it always was: the cut-off is not widened to plane–plane.
#[test]
fn the_parallel_cylinder_union_still_refuses_and_a_box_edge_is_still_a_run_out() {
    let cyl = |cx: f64| {
        let lp = profile::circle(p2(cx, 0.0), 0.5, tol()).unwrap();
        let profile = Profile::new(SketchPlane::xy(), vec![lp.into()])
            .validate(tol())
            .unwrap();
        extrude(&profile, Extrusion::Distance(1.0), tol())
            .unwrap()
            .body
    };
    let err = topo::union(&cyl(0.0), &cyl(0.6), tol()).expect_err("the parallel pair refuses");
    assert!(
        matches!(err, topo::BooleanError::CurvedPierceUnsupported { .. }),
        "the boolean's curved-pierce door, got {err:?}"
    );

    let body = cube(1.0, tol());
    let e = query::all_edges(&body)[0];
    let err = fillet_edges(&body, &[e], R, tol()).expect_err("one box edge refuses");
    assert!(
        matches!(err.error, BlendError::UnsupportedRunOut { .. }),
        "a partly requested corner is a run-out, got {:?}",
        err.error
    );
}

/// **The lever `corner_at` hands `fillet3_cap_transverse` is the link's
/// own extent**, pinned through the battery's public entry on a cap
/// tilted 1e-3 rad — the smallest tilt the split door builds (1e-4
/// escalates at the join's carrier lane, 1e-5 refuses `CircularAxes`)
/// — at two lengths. The lever is the crease's extent, `0.6·L` (the
/// cut sits at six tenths of the rod), so the margins are `1.8e-4` and
/// `1.5e-3`. At the fillet door's own band both are DEFINITE
/// departures (every buildable tilt is, at every buildable length), so
/// both rods refuse there identically; `run_battery` takes its band as
/// an argument, and under `Band::new(1.2e-4, 1.2e-3)` the same angle is
/// IN BAND at `L = 0.3` (escalates naming the predicate) and DEFINITE
/// at `L = 2.5` (refuses as the run-out). A lever of `T::one()` in
/// place of the extent would put both at `1e-3` — in band — and red the
/// long rod's arm; a band one decade lower (`2e-4, 2e-3`) admitted the
/// short rod's tilted cap as transverse, which is the lever seen from
/// the other side.
#[test]
fn the_cap_lever_is_the_links_extent() {
    let phi = 1e-3_f64;
    let band = Band::new(1.2e-4, 1.2e-3).expect("a band ten wide, like the door's");
    for (len, in_band) in [(0.3, true), (2.5, false)] {
        let rod = rod_d_profile_of_length_at::<f64>(len, tol());
        let plane = SplitPlane {
            origin: Point3::new(0.0, 0.0, 0.6 * len),
            normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
        };
        let result = split(&rod, &plane, tol()).expect("a 1e-3 tilt splits");
        let SplitPart::Body(below) = &result.below else {
            panic!("the lower part carries material");
        };
        let creases = rod_creases(below);
        assert_eq!(creases.len(), 2, "L = {len}: two creases");
        for e in creases {
            // The door's band: definite at either length.
            let err = fillet_edges(below, &[e], ROD_FILLET, tol()).expect_err("oblique");
            assert!(
                matches!(err.error, BlendError::UnsupportedRunOut { .. }),
                "L = {len}: the door refuses definitely, got {:?}",
                err.error
            );
            // A band the tilt lands inside of at one length only.
            let verdict = run_battery(
                &BlendRequest {
                    body: below,
                    edges: vec![e],
                    size: ROD_FILLET,
                },
                band,
            )
            .expect_err("the oblique end is never admitted");
            match (in_band, verdict) {
                (true, BlendError::Escalated { source, .. }) => {
                    assert_eq!(source.predicate, Some("fillet3_cap_transverse"));
                }
                (false, BlendError::UnsupportedRunOut { detail, .. }) => {
                    assert_eq!(detail, RULED_END_NOT_TRANSVERSE);
                }
                (_, other) => panic!(
                    "L = {len}: the verdict must follow the lever (in band: {in_band}), got \
                     {other:?}"
                ),
            }
        }
    }
}

/// **A curved end face refuses typed before any metering** — the
/// run-out the mid-curve taxonomy reserves. A quarter revolve of a
/// bored rectangle whose top is an arc: its wedge walls are planes
/// containing the axis, so each cylinder × wedge-wall crease is a
/// ruling that ends on the flat bottom (a transverse cap) and against
/// the torus top (a curved face) — `corner_at`'s curved-end branch,
/// which refuses with the same detail as the oblique cap and names the
/// upper end.
#[test]
fn a_curved_end_face_refuses_typed_before_metering() {
    let body = revolved_about_y(
        vec![
            ProfileVertex::new(p2(0.5, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 0.0), 0.0),
            ProfileVertex::new(p2(1.0, 1.0), 0.3),
            ProfileVertex::new(p2(0.5, 1.0), 0.0),
        ],
        sweep::Revolution::Partial(core::f64::consts::FRAC_PI_2),
        tol(),
    );
    validate_geometric(&body, tol()).expect("the wedge is tier-3 valid");
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 4, "two walls × two wedge planes");
    for e in creases {
        let err = fillet_edges(&body, &[e], ROD_FILLET, tol()).expect_err("a curved end");
        let BlendError::UnsupportedRunOut { at, detail } = err.error else {
            panic!("the curved end is a run-out, got {:?}", err.error);
        };
        assert_eq!(detail, RULED_END_NOT_TRANSVERSE);
        let topo::EntityId::Vertex(v) = at else {
            panic!("the refusal names the vertex, got {at:?}");
        };
        let p = body
            .get_vertex(v)
            .and_then(|x| body.get_point(x.point))
            .unwrap();
        assert!(p.y > 0.5, "the refusing end is the curved one, at {p:?}");
    }
}
