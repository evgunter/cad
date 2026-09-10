//! **The five-wall sleeve** — every analytic surface kind the kernel
//! holds, in one meridian, hollowed by ONE `shell` call.
//!
//! # What the scene claims
//!
//! `shell` moves a body's whole boundary inward by `t` and inserts the
//! result as a cavity. The interesting question is not whether the
//! cavity exists but **what its walls are**: an offset that reached for
//! a general fitted surface would answer a cone with a NURBS patch and
//! the wall would stop being exact. It does not. Every analytic kind
//! offsets to its own kind, with one number moved:
//!
//! | wall | outer | cavity |
//! | --- | --- | --- |
//! | bottom, mouth | `Plane` | the same plane, origin moved `t` along its normal |
//! | rim, bore | `Cylinder` | radius `∓ t` — the bore's GROWS, the rim's shrinks |
//! | shoulder | `Cone` | the same half-angle, apex slid along the axis |
//! | crown | `Sphere` | radius `− t`, centre fixed |
//! | waist | `Torus` | minor radius `− t`, major radius fixed |
//!
//! The scene reads all five out of the stored surfaces and asserts
//! them, so a lane that taught one wall to fit rather than to offset
//! reddens here rather than in a picture nobody measures.
//!
//! # Why the meridian has this shape
//!
//! One profile has to mint all five kinds, and the two arcs are what
//! make it possible to state the difference between them: **an arc
//! about a centre ON the axis revolves to a sphere; the same arc about
//! a centre OFF it revolves to a torus.** Both are authored through
//! `Center { c, .. }`, so the centre is written down rather than
//! implied by a bulge, and the claim is legible in the source.
//!
//! The meridian is ANNULAR — it never touches the axis — for three
//! reasons that all matter here. It gives the bore its own cylinder
//! (the second cylinder, and the one whose offset runs the OTHER way);
//! it makes every latitude rim ONE closed edge rather than two
//! half-arcs meeting at a chart seam; and it makes the body a bored
//! SLEEVE rather than a cup, which is why the sealed solid is genus 2
//! — an annular solid is a torus with shape, and hollowing it gives
//! two shells of genus 1 apiece.
//!
//! Every joint is deliberately NON-tangent. The first draft of this
//! meridian met the waist's arc tangentially where it leaves the rim
//! cylinder, and `Profile::validate` refused it `UndeclaredTangency`
//! naming the joint and its three recourses — the coincidence ladder
//! doing its job on a shape a modeller draws by accident.
//!
//! # Stations
//!
//! Every station is a dyadic rational in metres (a whole number of
//! 1/64 m), so the profile's arithmetic is exact and the offset
//! numbers the assertions compare against are exact too. The two arcs
//! are 3-4-5 triangles on their own radii, which is what keeps their
//! junction stations dyadic as well.

use std::collections::BTreeMap;

use pncad::authoring::p2;
use pncad::geom::Surface;
use pncad::geom_core::{Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, FaceKey};

use crate::{SceneBody, Stop, View};
use pncad::authoring::validated;

/// One 64th of a metre — the grid every station stands on.
const U: f64 = 1.0 / 64.0;

/// The bore's radius: the cylinder the cavity's own bore offsets
/// OUTWARD from.
const R_BORE: f64 = 5.0 * U;
/// The rim cylinder's radius, and the vessel's widest station.
const R_RIM: f64 = 12.0 * U;
/// Where the rim cylinder ends and the waist's torus band begins.
const Y_RIM: f64 = 11.0 * U;

/// The waist band's major radius — its centre circle's own radius,
/// which is what makes it a torus rather than a sphere.
const R_MAJOR: f64 = 9.0 * U;
/// The waist band's minor radius: the meridian arc's own radius.
const R_MINOR: f64 = 5.0 * U;
/// The height of the waist band's centre circle.
const Y_TORUS: f64 = 7.0 * U;

/// Where the waist ends and the shoulder cone starts.
const R_WAIST: f64 = 9.0 * U;
/// …and its height. `R_WAIST` is `R_MAJOR` and this is `Y_TORUS +
/// R_MINOR`: the arc ends at the top of its own tube.
const Y_WAIST: f64 = 12.0 * U;

/// The crown sphere's radius. Its arc starts at its own equator, so
/// this is also the shoulder's upper radius.
const R_SPHERE: f64 = 10.0 * U;
/// The crown sphere's centre height, on the axis. Set so the
/// shoulder cone is as tall as the crown is, which is what makes all
/// five kinds separable in the render.
const Y_SPHERE: f64 = 18.0 * U;

/// The mouth's radius — the crown arc's far end, a 3-4-5 point of the
/// sphere (`0.8 R`, `0.6 R` above the centre).
const R_MOUTH: f64 = 8.0 * U;
/// The mouth's plane.
const Y_MOUTH: f64 = 24.0 * U;

/// The wall. Thick enough to read at montage scale, and well inside
/// the narrowest material run (4/64 m, between the bore and the
/// waist's own station).
const WALL: f64 = 1.0 * U;

/// The scene's chordal deviation.
const DELTA: f64 = 1e-3;

/// **The meridian**, authored through the PATHS lattice the way a user
/// would: stations named, arcs given their centres.
fn meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(p2(R_BORE, 0.0))
        .line_to(p2(R_RIM, 0.0), tol)
        .expect("the base annulus")
        .line_to(p2(R_RIM, Y_RIM), tol)
        .expect("the rim cylinder")
        .arc_to(
            Center {
                c: p2(R_MAJOR, Y_TORUS),
                winding: ArcSweep::Ccw,
                p: p2(R_WAIST, Y_WAIST),
            },
            tol,
        )
        .expect("an arc about a centre OFF the axis is a torus")
        .line_to(p2(R_SPHERE, Y_SPHERE), tol)
        .expect("the shoulder cone")
        .arc_to(
            Center {
                c: p2(0.0, Y_SPHERE),
                winding: ArcSweep::Ccw,
                p: p2(R_MOUTH, Y_MOUTH),
            },
            tol,
        )
        .expect("an arc about a centre ON the axis is a sphere")
        .line_to(p2(R_BORE, Y_MOUTH), tol)
        .expect("the mouth annulus")
        .line_to(Start, tol)
        .expect("the bore closes the meridian")
        .into()
}

/// The meridian revolved whole about `+y`.
fn sleeve(tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![meridian(tol)], tol).expect("the meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian revolves")
    .body
}

/// How many faces of each surface kind a body carries.
fn kinds(body: &Body<f64>) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for (_, face) in body.faces() {
        let name = match body.get_surface(face.surface) {
            Some(Surface::Plane { .. }) => "Plane",
            Some(Surface::Cylinder { .. }) => "Cylinder",
            Some(Surface::Cone { .. }) => "Cone",
            Some(Surface::Sphere { .. }) => "Sphere",
            Some(Surface::Torus { .. }) => "Torus",
            Some(_) => "other",
            None => panic!("a face's surface key does not resolve"),
        };
        *out.entry(name).or_insert(0) += 1;
    }
    out
}

/// Every distinct value `read` returns over the body's faces, sorted by
/// bits — the multiset an offset assertion compares against.
fn stored<F: Fn(&Surface<f64>) -> Option<f64>>(body: &Body<f64>, read: F) -> Vec<u64> {
    let mut out: Vec<u64> = body
        .faces()
        .filter_map(|(_, f)| body.get_surface(f.surface).and_then(&read))
        .map(f64::to_bits)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// `stored` for a set of expected values, in the same shape.
fn bits(values: &[f64]) -> Vec<u64> {
    let mut out: Vec<u64> = values.iter().copied().map(f64::to_bits).collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// The distinct values `read` returns, ascending — `stored`'s sibling
/// for the assertions that compare a GAP rather than a bit pattern.
fn radii<F: Fn(&Surface<f64>) -> Option<f64>>(body: &Body<f64>, read: F) -> Vec<f64> {
    let mut out: Vec<f64> = stored(body, read).into_iter().map(f64::from_bits).collect();
    out.sort_by(|a, b| a.partial_cmp(b).expect("stored radii are finite"));
    out
}

/// The scene: the vessel hollowed, and the same vessel opened at its
/// mouth, in one cell.
pub fn stops(tol: Tol) -> Vec<Stop> {
    let solid = sleeve(tol);

    // The operand: five kinds, seven faces, and one of them is the
    // bore — the wall whose offset runs the other way.
    let want_solid: BTreeMap<&str, usize> = [
        ("Plane", 2),
        ("Cylinder", 2),
        ("Cone", 1),
        ("Sphere", 1),
        ("Torus", 1),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        kinds(&solid),
        want_solid,
        "the meridian mints every analytic kind once, the plane and the cylinder twice"
    );
    assert_eq!(
        pncad::topo::validate_geometric(&solid, tol),
        Ok(()),
        "the operand: tier 3"
    );

    // ---- the hollow ----
    let sealed = pncad::topo::shell(&solid, WALL, tol)
        .expect("a five-kind vessel hollows in ONE call")
        .body;
    assert_eq!(
        pncad::topo::validate_geometric(&sealed, tol),
        Ok(()),
        "the sealed vessel: tier 3"
    );
    assert_eq!(sealed.shells().count(), 2, "outer boundary + cavity");

    // **The scene's claim, read off the stored surfaces.** Every kind
    // appears exactly twice: the wall's own face and its offset twin.
    // A fitted offset would answer one of these with a NURBS patch and
    // the census would say so.
    let want_sealed: BTreeMap<&str, usize> = want_solid.iter().map(|(k, n)| (*k, n * 2)).collect();
    assert_eq!(
        kinds(&sealed),
        want_sealed,
        "each wall's offset is a face of its own kind — not a fitted patch"
    );

    // …and each twin differs in exactly one number, bitwise.
    assert_eq!(
        stored(&sealed, |s| match s {
            Surface::Cylinder { radius, .. } => Some(*radius),
            _ => None,
        }),
        bits(&[R_RIM, R_RIM - WALL, R_BORE, R_BORE + WALL]),
        "the rim's cavity cylinder SHRINKS and the bore's GROWS — inward is per face, \
         never per kind"
    );
    // **The curved radii are compared as a DIFFERENCE, and that is a
    // finding rather than a convenience.** A cylinder's radius is the
    // station the meridian names, so it survives bitwise (above). An
    // ARC's is reconstructed from its endpoints against its centre, and
    // that route rounds: this sphere's authored radius is the exact
    // dyadic 10/64, and what the carrier stores is one ulp above it
    // (`0.15625000000000003`). Nothing promised otherwise — the scene
    // says so rather than authoring around it. What the offset owes is
    // the GAP, and the gap is exact: two radii on one chart share an
    // exponent, so subtracting the same wall from each is lossless.
    let sphere = radii(&sealed, |s| match s {
        Surface::Sphere { radius, .. } => Some(*radius),
        _ => None,
    });
    assert_eq!(sphere.len(), 2, "one sphere wall, one sphere cavity");
    assert_eq!(
        sphere[1] - sphere[0],
        WALL,
        "the crown's cavity is a sphere of radius r − t, exactly: {sphere:?}"
    );
    assert!(
        (sphere[1] - R_SPHERE).abs() <= f64::EPSILON * R_SPHERE,
        "…about the authored radius, up to the arc's own reconstruction: stored \
         {} against the authored {R_SPHERE}",
        sphere[1]
    );

    let minor = radii(&sealed, |s| match s {
        Surface::Torus { minor_radius, .. } => Some(*minor_radius),
        _ => None,
    });
    assert_eq!(minor.len(), 2, "one torus wall, one torus cavity");
    assert_eq!(
        minor[1] - minor[0],
        WALL,
        "the waist's cavity moves the TUBE radius by exactly t: {minor:?}"
    );
    assert!(
        (minor[1] - R_MINOR).abs() <= f64::EPSILON * R_MINOR,
        "…about the authored tube radius, up to the same reconstruction: stored {} \
         against the authored {R_MINOR}",
        minor[1]
    );
    let major = radii(&sealed, |s| match s {
        Surface::Torus { major_radius, .. } => Some(*major_radius),
        _ => None,
    });
    assert_eq!(
        major.len(),
        1,
        "…and leaves the centre circle alone: ONE major radius across both faces, {major:?}"
    );
    assert!(
        (major[0] - R_MAJOR).abs() <= f64::EPSILON * R_MAJOR,
        "…which is the authored centre-circle radius: stored {} against {R_MAJOR}",
        major[0]
    );

    assert_eq!(
        radii(&sealed, |s| match s {
            Surface::Cone { half_angle, .. } => Some(*half_angle),
            _ => None,
        })
        .len(),
        1,
        "the shoulder's cavity is a cone of the SAME half-angle — the apex slides, the \
         opening does not"
    );
    let planes = radii(&sealed, |s| match s {
        Surface::Plane { origin, .. } => Some(origin.y),
        _ => None,
    });
    assert_eq!(
        planes,
        vec![0.0, WALL, Y_MOUTH - WALL, Y_MOUTH],
        "each plane's twin stands t along its own normal"
    );

    // ---- the same vessel, opened ----
    let mouth: Vec<FaceKey> = solid
        .faces()
        .filter(|(_, f)| {
            matches!(
                solid.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if origin.y == Y_MOUTH
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(mouth.len(), 1, "the mouth is ONE annular face");
    let cup = pncad::topo::shell_open(&solid, WALL, &mouth, tol)
        .expect("the same vessel opens at its mouth")
        .body;
    assert_eq!(
        pncad::topo::validate_geometric(&cup, tol),
        Ok(()),
        "the opened cup: tier 3"
    );
    assert_eq!(
        cup.shells().count(),
        1,
        "one opening turns the cavity inside out: a cup is ONE shell"
    );
    assert_eq!(
        kinds(&cup),
        want_sealed,
        "opening moves no wall — the cup carries the sealed body's own face census"
    );

    // Placed beside the sealed vessel so one camera and one scale carry
    // both, which is what makes the rim readable against the wall it
    // came from.
    let apart = pncad::topo::transform_rigid(
        &cup,
        &pncad::geom_core::Affine3::translation(pncad::geom_core::Vec3::new(R_RIM * 3.0, 0.0, 0.0)),
        tol,
    )
    .expect("a translation is rigid");

    vec![Stop {
        name: "fivewall",
        caption: "FIVE WALL KINDS, ONE `shell` CALL (a bored sleeve, sealed and opened)"
            .to_string(),
        montage: true,
        story: "one annular meridian mints every analytic surface the kernel holds — \
                two planes, two cylinders, a cone, a sphere and a torus — and ONE \
                `shell` call hollows all of them. The claim is not that a cavity \
                appears but WHAT it is made of: each wall's offset is a face of its \
                own kind with one number moved, read out of the stored surfaces and \
                asserted. The arcs are what separate the last two kinds: a centre ON \
                the axis revolves to a sphere, the same arc about a centre OFF it to a \
                torus. The meridian never touches the axis, so the body is a bored \
                SLEEVE: genus 1 solid, genus 2 hollowed, and the bore's own cavity \
                cylinder is the one that GROWS. Drawn see-through, and beside it the \
                same sleeve OPENED at its mouth — one annular rim, one shell",
        ops: "PATHS meridian (2 lines, arc about an OFF-axis centre, line, arc about an \
              ON-axis centre, 2 lines) -> revolve(+y, Full) -> shell(t = 15.625 mm) \
              and shell_open(the mouth's face) -> transform_rigid to set the two side \
              by side",
        delta: DELTA,
        note: None,
        view: View {
            elev: 14.0,
            azim: -62.0,
            up: 'y',
        },
        bodies: vec![
            SceneBody::plain("fivewall", [0.55, 0.63, 0.78], sealed)
                .transparent(55)
                .step_at_frontier(
                    |e| {
                        matches!(
                            e,
                            pncad::step_export::StepExportError::CurvedShellClassification { .. }
                        )
                    },
                    "the writer's outward/void classifier has grown a curved arm. This \
                     is the FIFTH probe of that one gate — retire it with klein's WALL \
                     6, the `ring` scene's, `tubewall`'s `hollowtorus` and \
                     `torusvessel`'s, which are all the same gate",
                ),
            SceneBody::plain("fivewallcup", [0.78, 0.66, 0.50], apart),
        ],
    }]
}
