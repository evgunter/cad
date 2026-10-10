//! **The ruled band ends at its plane caps** (FILLET-H7): the
//! cylinder–plane(∥) arm's band, carved between the plane caps it
//! ends at, perpendicular to its ruling or oblique.
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
//! D-profile extrude (one 254° cap arc) carves too; an oblique cap cuts
//! the band off in an ellipse, at the centroid closed form; the
//! kind-picker's arms and the lever it is metered at (the link's
//! extent, shown through the battery on one tilt at two lengths); a
//! curved end face refuses before metering; a mutant cut-off arc is
//! refused at the attachment gate.
//!
//! The Phase-1 measurements that framed the unit stay as rows: the
//! parallel-cylinder union (the `CylinderCylinderCylinder` consumer)
//! still refuses at the boolean's curved-pierce door, and a box's single
//! edge is cut off at its end faces by the plane–plane band's own
//! cut-off.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::outcomes::outcome;
use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec};
use geom_core::k_stats::Bracket;
use geom_core::{Band, Point2, Point3, Sign, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::ExtrudeSide;
use sweep::blend::battery::{
    BlendRequest, END_FACE_CURVED, EndSection, cap_transverse, run_battery,
};
use sweep::blend::{BlendDecision, BlendError, Blended, CornerConfig, RunOutPolicy, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, assert_naming_totality, cube, finished, revolved_about_y,
    rod_creases, rod_d_profile_at, rod_d_profile_of_length_at, rod_section_cut, rod_with_flat,
};
use sweep::{Extrusion, extrude};
use topo::query;
use topo::splitting::{SplitPart, split};
use topo::{Body, EdgeKey, FaceKey, VertexKey, mass_properties, validate_geometric};

const R: f64 = ROD_FILLET;

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

    let out = fillet_edges(
        &sweep::test_support::at_rest(source, tol()),
        &creases,
        R,
        tol(),
    )
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
        (4, 6, 4),
        "the boolean's rod: each cap one arc and one chord"
    );
    let bracket = Bracket::open();
    let _ = carve_and_check(&source, "rod ∖ box");
    let log = bracket.finish().verdicts;
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
    let source = sweep::test_support::finished("source", rod_with_flat(tol()), tol());
    let creases = rod_creases(&source);
    let vol0 = volume(&source);
    for &e in &creases {
        let out = fillet_edges(&source, &[e], R, tol()).expect("one crease carves");
        assert_eq!(census(&out.body), (6, 9, 5));
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

/// **The first moment `∫∫ (along·p) dA` of the region
/// [`rod_section_cut`] measures**, for a planar direction `along`, by
/// the same decomposition: the quad's polygon moment, less the fillet
/// sector's at `c`, plus the rod segment's (its sector at the origin
/// less the triangle `O, V, f_a`). A sector of radius `ρ` and sweep `σ`
/// has its centroid `4ρ·sin(σ/2)/(3σ)` out along its bisector. The
/// region is the one at the crease with `y > 0`; the other crease's is
/// its mirror in `y = 0`.
fn rod_section_moment(big_r: f64, flat: f64, r: f64, along: (f64, f64)) -> f64 {
    let dot = |p: (f64, f64)| along.0 * p.0 + along.1 * p.1;
    let h = ((big_r - r).powi(2) - (flat - r).powi(2)).sqrt();
    let c = (flat - r, h);
    let v = (flat, (big_r.powi(2) - flat.powi(2)).sqrt());
    let scale = big_r / (big_r - r);
    let f_a = (c.0 * scale, c.1 * scale);
    let quad = [c, (flat, h), v, f_a];
    let mut quad_moment = 0.0;
    for i in 0..4 {
        let (p, q) = (quad[i], quad[(i + 1) % 4]);
        quad_moment += dot((p.0 + q.0, p.1 + q.1)) * (p.0 * q.1 - q.0 * p.1) / 6.0;
    }
    let sector = |at: (f64, f64), rho: f64, from: f64, sweep: f64| {
        let reach = 4.0 * rho * (sweep / 2.0).sin() / (3.0 * sweep);
        let mid = from + sweep / 2.0;
        0.5 * rho * rho * sweep * dot((at.0 + reach * mid.cos(), at.1 + reach * mid.sin()))
    };
    let theta = ((flat - r) / (big_r - r)).acos();
    let beta = (flat / big_r).acos();
    let triangle = 0.5 * (v.0 * f_a.1 - f_a.0 * v.1) * dot((v.0 + f_a.0, v.1 + f_a.1)) / 3.0;
    quad_moment - sector(c, r, 0.0, theta) + sector((0.0, 0.0), big_r, beta, theta - beta)
        - triangle
}

/// **An oblique cap cuts the ruled band off in an ellipse.** The rod's
/// top is cut off by a plane through `(0, 0, 0.7)` with unit normal
/// `n`, tilted `φ` off the ruling's normal plane about `y` or about
/// `x`, `z = 0.7 − (n_x·x + n_y·y)/n_z`, so
/// each crease's upper end is a plane cap oblique to the ruling — and
/// its rim on the rod is itself an ellipse. The band is cut off in the
/// cap's section of it, an ellipse of minor semi-axis `r` and major
/// `r / n_z`; each crease removes its section over the length at the
/// section's centroid, `ΔV = A·0.7 − (n_x·∫∫x dA + n_y·∫∫y dA)/n_z`
/// ([`rod_section_moment`]), alone and both in one call. Each end arc
/// lies on the band and on the cap to `1e-12`.
#[test]
fn an_oblique_cap_cuts_the_ruled_band_off_in_an_ellipse() {
    let rod = rod_d_profile_at::<f64>(tol());
    let rod = sweep::test_support::finished("the rod", rod, tol());
    for (about, phi) in [("y", 0.3f64), ("y", -0.2), ("x", 0.3), ("x", -0.6)] {
        let n = match about {
            "y" => Vec3::new(phi.sin(), 0.0, phi.cos()),
            _ => Vec3::new(0.0, phi.sin(), phi.cos()),
        };
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, 0.7),
            n,
            geom_core::Tol::witness(),
        );
        let result = split(&rod, &plane, tol()).expect("the tilted cut splits");
        let SplitPart::Body(below) = &result.below else {
            panic!("the lower part carries material");
        };
        validate_geometric(below, tol()).expect("the cut rod is tier-3 valid");
        let creases = rod_creases(below);
        assert_eq!(creases.len(), 2, "the creases survive the cut");
        let area = rod_section_cut(ROD_R, ROD_FLAT, R);
        let (mx, my) = (
            rod_section_moment(ROD_R, ROD_FLAT, R, (1.0, 0.0)),
            rod_section_moment(ROD_R, ROD_FLAT, R, (0.0, 1.0)),
        );
        // A crease's section is the `y > 0` one or its mirror.
        let removes = |crease: EdgeKey| {
            let he = below.get_edge(crease).expect("a crease").he_plus;
            let start = below.get_half_edge(he).expect("its half").start;
            let y = below
                .get_point(below.get_vertex(start).expect("its vertex").point)
                .expect("its point")
                .y;
            area * 0.7 - (n.x * mx + n.y * my * y.signum()) / n.z
        };
        let (one, two) = (removes(creases[0]), removes(creases[1]));
        let what = format!("φ = {phi} about {about}");
        let enclosure = |body: &Body<f64>, which: &str| {
            let p = mass_properties(body, tol())
                .unwrap_or_else(|e| panic!("{what}: {which}'s certified props, got {e:?}"));
            (p.volume, p.volume_pad)
        };
        let (vol0, pad0) = enclosure(below, "the cut rod");
        for (request, removed) in [
            (vec![creases[0]], one),
            (vec![creases[1]], two),
            (creases.clone(), one + two),
        ] {
            let out = fillet_edges(
                &sweep::test_support::at_rest(below, tol()),
                &request,
                R,
                tol(),
            )
            .unwrap_or_else(|e| panic!("{what}: the oblique cap cuts off, got {e}"));
            validate_geometric(&out.body, tol())
                .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
            assert_naming_totality(below, &out, &request, &what);
            // The rod's wall is trimmed by an ellipse already, so both
            // volumes are the quadrature lane's certified enclosures.
            let (vol1, pad1) = enclosure(&out.body, "the filleted rod");
            let (dv, pad) = (vol0 - vol1, pad0 + pad1);
            assert!(
                pad < crate::band_planar_cut_off::pad_ceiling()
                    && (dv - removed).abs() < 1e-12 + pad,
                "{what}: ΔV {dv} ± {pad} vs the closed form {removed}"
            );
            assert!(
                (dv - removed).abs() < crate::band_planar_cut_off::midpoint_tol(),
                "{what}: ΔV's midpoint {dv} is off the closed form {removed}"
            );
            let (arcs, stray) = crate::band_planar_cut_off::arc_residual(&out);
            assert!(
                arcs > 0 && stray < 1e-12,
                "{what}: {arcs} end arcs, one straying {stray} from a face it joins"
            );
            let rec = out.naming.as_ref().expect("births");
            let mut ellipses = 0;
            for (arc, _, _) in &rec.arcs {
                let c = out
                    .body
                    .get_curve_geom(out.body.get_edge(*arc).unwrap().curve)
                    .and_then(|g| g.certified())
                    .expect("a certified end curve");
                match *c.carrier() {
                    Curve3::Circle { center, .. } => {
                        assert!(center.z.abs() < 1e-12, "{what}: the circle is the bottom's");
                    }
                    Curve3::Ellipse {
                        major, minor, axis, ..
                    } => {
                        ellipses += 1;
                        assert_eq!(minor, R, "{what}: minor = r");
                        assert!(
                            (major - R / n.z).abs() < 1e-14,
                            "{what}: major = r / cos φ, got {major}"
                        );
                        assert!(
                            axis.cross(n).norm() < 1e-14,
                            "{what}: the ellipse lies in the cap"
                        );
                        assert!(
                            matches!(c.description(), EdgeDescription::Intersection { .. }),
                            "{what}: the ellipse is the band × cap intersection"
                        );
                    }
                    ref other => panic!("{what}: an end curve, got {other:?}"),
                }
            }
            assert_eq!(
                ellipses,
                request.len(),
                "{what}: one ellipse per oblique end"
            );
        }
    }
}

/// **The kind-picker `fillet3_cap_transverse`** — each arm reachable
/// and distinct: a perpendicular cap is Zero, the circle; a definite
/// departure is the ellipse, `r / cos φ` along the tilt's trace; an
/// in-band departure escalates naming the predicate; and a departure
/// definite only at a long lever, whose ellipse's axes differ by the
/// tilt SQUARED, escalates as `CapEllipse` on the ellipse door's own
/// verdict — the near-perpendicular sliver band.
#[test]
fn cap_transverse_picks_the_circle_the_ellipse_or_escalates() {
    let band = Band::linear(tol()).expect("a band");
    let v = VertexKey::default();
    let tau = Vec3::new(0.0, 0.0, 1.0);
    let circle = cap_transverse(v, Vec3::new(0.0, 0.0, -1.0), tau, R, 1.0, band)
        .expect("a perpendicular cap is Zero");
    assert!(matches!(circle, EndSection::Circle), "{circle:?}");
    let phi = 0.3f64;
    let n = Vec3::new(phi.sin(), 0.0, phi.cos());
    let oblique = cap_transverse(v, n, tau, R, 1.0, band).expect("an oblique cap is the ellipse");
    let EndSection::Ellipse(Curve3::Ellipse {
        major,
        minor,
        u_ref: u_major,
        axis: normal,
        ..
    }) = oblique
    else {
        panic!("a definite departure picks the ellipse, got {oblique:?}");
    };
    assert_eq!(minor, R, "the minor semi-axis is the radius");
    assert!((major - R / phi.cos()).abs() < 1e-15, "major = r / cos φ");
    assert!(
        u_major.dot(n).abs() < 1e-15 && u_major.y.abs() < 1e-15,
        "the major axis is the tilt's trace in the cap, got {u_major:?}"
    );
    assert!(
        normal.cross(n).norm() < 1e-15,
        "the ellipse's axis is the cap's normal"
    );
    let t = 0.5 * (band.zero() + band.escalate());
    let escalated = cap_transverse(v, Vec3::new(t, 0.0, 1.0), tau, R, 1.0, band)
        .expect_err("an in-band cap escalates");
    let BlendError::Escalated { source, .. } = &escalated else {
        panic!("the in-band row must escalate, got {escalated:?}");
    };
    assert_eq!(source.predicate, Some("fillet3_cap_transverse"));
    let sliver = cap_transverse(v, Vec3::new(t, 0.0, 1.0), tau, R, 1e3, band)
        .expect_err("levered up, the departure is definite and the axes are not");
    let BlendError::Escalated {
        source, decision, ..
    } = &sliver
    else {
        panic!("the sliver band escalates, got {sliver:?}");
    };
    assert_eq!(source.predicate, Some("ellipse_axes_distinct"));
    assert_eq!(*decision, BlendDecision::CapEllipse);
}

/// **The vocabulary is the ratified one and the tag maps its policy.**
#[test]
fn the_transverse_cap_names_its_policy() {
    assert_eq!(
        CornerConfig::EndFace.policy(),
        Some(RunOutPolicy::CutOffAtEndFace)
    );
    let shown = format!(
        "{} / {}",
        CornerConfig::EndFace,
        RunOutPolicy::CutOffAtEndFace
    );
    assert!(
        shown.contains("an end face") && shown.contains("cut the band off"),
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
/// unioned — has a body: the rims' crossings of the walls are
/// certified, and the two pairs of cap discs, one plane each by margin,
/// glue whether or not they are declared, so the union is the declared
/// union bit for bit (D10) at its closed form, the fixture the concave
/// ruled band can be cut from. And a box's single edge, which is not a
/// ruled link, is cut off at its end faces by the plane–plane band's
/// own cut-off.
#[test]
fn the_parallel_cylinder_union_builds_and_a_box_edge_is_cut_off() {
    let cyl = |cx: f64| {
        let lp = profile::circle(Point2::new(cx, 0.0), 0.5, tol()).unwrap();
        let profile = Profile::new(SketchPlane::xy(), vec![lp.into()])
            .validate(tol())
            .unwrap();
        let body = extrude(
            &profile,
            Extrusion::Distance {
                depth: 1.0,
                side: ExtrudeSide::Along,
            },
            tol(),
        )
        .unwrap()
        .body;
        finished("the cylinder", body, tol())
    };
    let (a, b) = (cyl(0.0), cyl(0.6));
    let d = topo::flush::declare_all(&topo::flush::find_flush_candidates(&a, &b, tol()).unwrap());
    let declared = topo::union_with(&a, &b, &d, tol());
    let undeclared = topo::union(&a, &b, tol());
    assert_eq!(d.coincident_faces.len(), 2, "the two cap-disc pairs");
    let Ok(topo::BooleanResult::Body(bb)) = &declared else {
        panic!("the declared parallel pair builds: {declared:?}");
    };
    // Two r = 1/2 discs whose centres are 0.6 apart, one high.
    let lens = 0.5 * 0.6_f64.acos() - 0.3 * 0.8;
    let want = 2.0 * core::f64::consts::PI * 0.25 - lens;
    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
    assert!((v - want).abs() < 1e-9, "{v} vs the closed form {want}");
    assert_eq!(
        outcome(&undeclared),
        outcome(&declared),
        "undeclared is the declared union"
    );

    let body = cube(1.0, tol());
    let e = query::all_edges(&body)[0];
    fillet_edges(&sweep::test_support::at_rest(&body, tol()), &[e], R, tol())
        .expect("one box edge is cut off at its end faces");
}

/// **The lever `corner_at` hands `fillet3_cap_transverse` is the link's
/// own extent**, pinned through the battery's public entry on a cap
/// tilted 1e-2 rad at two lengths. The tilt is the smallest the split
/// door builds at EVERY eps row: its own near-parallel margin grows as
/// the tilt squared (1e-4 escalates at the join's carrier lane at
/// eps 1e-9, 1e-3 falls under the 1e-6 row's band, 1e-5 refuses
/// `CircularAxes`). The lever is the crease's extent, `0.6·L` (the cut
/// sits at six tenths of the rod), so the margins are `1.8e-3` and
/// `1.5e-2`. `run_battery` takes its band as an argument — explicit,
/// so the same at every eps row — and under `Band::new(1.2e-3,
/// 1.2e-2)` the same angle is IN BAND at `L = 0.3` (escalates naming
/// the predicate) and DEFINITE at `L = 2.5`, where the picker reaches
/// the ellipse and the ellipse's axes, `r·(1/cos φ − 1) = 5e-6` apart,
/// are one circle under that band: it escalates under the same
/// decision through `ellipse_axes_distinct`. A lever of `T::one()` in
/// place of the extent would put both at `1e-2` — in band — and red the
/// long rod's arm. At the fillet door's own band the departure is
/// definite at either length, and the door builds the ellipse wherever
/// its axes clear the band's escalate edge and escalates where they do
/// not.
#[test]
fn the_cap_lever_is_the_links_extent() {
    let phi = 1e-2_f64;
    // Pinned on BOTH edges, so the row is the same at every eps row.
    let band = Band::new(1.2e-3, 1.2e-2).expect("the row's own band, ten wide");
    let axes_apart = R * (1.0 / phi.cos() - 1.0);
    let door = Band::linear(tol()).expect("the door's band");
    for (len, in_band) in [(0.3, true), (2.5, false)] {
        let rod = rod_d_profile_of_length_at::<f64>(len, tol());
        let rod = sweep::test_support::finished("the rod", rod, tol());
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, 0.6 * len),
            Vec3::new(phi.sin(), 0.0, phi.cos()),
            geom_core::Tol::witness(),
        );
        let result = split(&rod, &plane, tol()).expect("a 1e-2 tilt splits");
        let SplitPart::Body(below) = &result.below else {
            panic!("the lower part carries material");
        };
        let creases = rod_creases(below);
        assert_eq!(creases.len(), 2, "L = {len}: two creases");
        for e in creases {
            match fillet_edges(
                &sweep::test_support::at_rest(below, tol()),
                &[e],
                ROD_FILLET,
                tol(),
            ) {
                Ok(out) if axes_apart > door.escalate() => {
                    validate_geometric(&out.body, tol()).expect("tier 3");
                }
                Err(err) if axes_apart <= door.escalate() => {
                    let BlendError::Escalated { source, .. } = err.error else {
                        panic!(
                            "L = {len}: the door's sliver escalates, got {:?}",
                            err.error
                        );
                    };
                    assert_eq!(source.predicate, Some("ellipse_axes_distinct"));
                }
                other => panic!(
                    "L = {len}: axes {axes_apart} apart against the door's escalate edge {}: \
                     got {:?}",
                    door.escalate(),
                    other.map(|_| ()).map_err(|e| e.error)
                ),
            }
            let verdict = run_battery(
                &BlendRequest {
                    body: below,
                    edges: vec![e],
                    size: ROD_FILLET,
                },
                band,
            )
            .expect_err("the tilt is in band, or its axes are");
            let BlendError::Escalated {
                source, decision, ..
            } = verdict
            else {
                panic!("L = {len}: the verdict escalates, got {verdict:?}");
            };
            let (want, predicate) = if in_band {
                (BlendDecision::CapTransverse, "fillet3_cap_transverse")
            } else {
                (BlendDecision::CapEllipse, "ellipse_axes_distinct")
            };
            assert_eq!(decision, want, "L = {len}");
            assert_eq!(source.predicate, Some(predicate), "L = {len}");
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
    let body = sweep::test_support::finished(
        "body",
        revolved_about_y(
            vec![
                (Point2::new(0.5, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.0, 1.0), 0.3),
                (Point2::new(0.5, 1.0), 0.0),
            ],
            sweep::Revolution::Partial(core::f64::consts::FRAC_PI_2),
            tol(),
        ),
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
        assert_eq!(detail, END_FACE_CURVED);
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
