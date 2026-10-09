//! **`shell_open` through a curved designated face.**
//!
//! A designated face on a periodic chart opens in its chart's own form:
//! a chart that wraps its period through a pole becomes a seamed band
//! (each of its faces kept, its seams cut short at the cavity's
//! corners), one that wraps between two boundaries becomes two (one at
//! each boundary), and a window that does not wrap becomes a ring, as
//! on a plane. What the readers cannot yet read about a ringed curved window
//! refuses where they read it — the mesh, or tier 3's check 7 — and not
//! in the shell op. Each built row is checked against its closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::census::{genus_of, rings_of};
use crate::common::shell_operands::{
    capped_vessel, cone_tipped_vessel, d_section, dome_sector, domed_vessel, funnel_vessel,
    hollow_capped_vessel, nearly_domed_vessel, vessel,
};
use core::f64::consts::PI;
use geom_core::Tol;
use sweep::test_support::finished;
use topo::{Body, FaceKey, ShellError};

/// Every face of `body` on a surface of `kind`.
fn faces_on(body: &Body<f64>, kind: geom::SurfaceKind) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| body.get_surface(f.surface).map(geom::Surface::kind) == Some(kind))
        .map(|(k, _)| k)
        .collect()
}

fn open(
    body: &Body<f64>,
    faces: &[FaceKey],
    t: f64,
) -> Result<topo::Shelled<f64>, ShellError<f64>> {
    let tol = Tol::witness();
    topo::shell_open(&finished("the operand", body.clone(), tol), t, faces, tol)
}

/// The volume of a spherical cap of height `h` on a sphere of radius `rho`.
fn cap_volume(rho: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * rho - h) / 3.0
}

/// **A spherical cap opens to a seamed band.** The revolve wears the cap
/// on two half-faces meeting along two seams at the pole; the cavity's
/// counterpart, lifted back onto the sphere, is a smaller cap over the
/// same pole. The rim is the band between the wall's junction and the
/// cavity's, and it keeps the operand's own faces and seams: both
/// half-faces survive ring-free, each seam cut short at the cavity's
/// corner, and the pole is gone. The body is tier-3 valid, meshes, and
/// its volume is the vessel minus a cavity that runs up to the sphere.
#[test]
fn a_spherical_cap_opens_to_a_seamed_band() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.6, 0.05);
    let (body, rho, centre) = capped_vessel(r, h, 60.0);
    let cap = faces_on(&body, geom::SurfaceKind::Sphere);
    assert_eq!(cap.len(), 2, "the revolve wears the cap on two half-faces");
    let shelled = open(&body, &cap, t).expect("the cap opens");
    let cup = &shelled.body;

    assert_eq!(topo::validate_geometric(cup, tol), Ok(()), "tier 3");
    assert_eq!(cup.shells().count(), 1, "the rim fuses the cavity in");
    assert_eq!(
        (rings_of(cup), genus_of(cup)),
        (0, 0),
        "a seamed band carries no ring, and a cup is genus 0"
    );
    let band = faces_on(cup, geom::SurfaceKind::Sphere);
    assert_eq!(
        band, cap,
        "both half-faces survive as the band, under their keys"
    );
    let rim = &shelled.naming.rims[0];
    assert_eq!(
        rim.rim, cap[0],
        "the record's rim face is the first designated"
    );
    assert!(
        shelled.naming.dead.loops.contains(&rim.ring),
        "the glue's ring is absorbed into the band's outer loop"
    );
    assert!(
        rim.holes.is_empty(),
        "a band through a pole is one band, with no second band's row"
    );
    // The pole is the one vertex both seams reached in the operand.
    let pole = body
        .vertex_points()
        .find(|(_, p)| p.x.abs() < 1e-12 && p.z.abs() < 1e-12 && p.y > h)
        .map(|(k, _)| k)
        .expect("the operand's pole");
    assert!(cup.get_vertex(pole).is_none(), "the pole dies");
    assert!(shelled.naming.dead.vertices.contains(&pole));

    let a = r - t;
    let outer = PI * r * r * h + cap_volume(rho, rho - (rho * rho - r * r).sqrt());
    let reach = (rho * rho - a * a).sqrt();
    let cavity = PI * a * a * (centre + reach - t) + cap_volume(rho, rho - reach);
    let props = topo::mass_properties(cup, tol).expect("the band's props");
    assert!(
        (props.volume - (outer - cavity)).abs() <= 1e-12 + props.volume_pad,
        "cup volume: got {} (pad {}), want {}",
        props.volume,
        props.volume_pad,
        outer - cavity
    );
    for delta in [1e-2, 1e-3] {
        mesh::tessellate(cup, delta, tol)
            .unwrap_or_else(|e| panic!("the band must triangulate at delta = {delta}, got {e:?}"));
    }
}

/// **A void's cap opens to a seamed band too, with the roles swapped.**
/// The hollow capped vessel's void is the vessel eroded by `0.2`, so it
/// has a pole-touching cap of two half-faces. Designated, the void's
/// own faces and seams die and its dilated twin survives as the band —
/// both twin half-faces live, ring-free. The outer wall keeps its
/// cavity; the void's wall becomes a cup opening into the gap. Its
/// volume is the outer wall's plus that cup, each a difference of two
/// vessels `w(a, b, ρ)`: a foot of radius `a` standing at `b` under a
/// cap of radius `ρ` about the operand's own centre.
#[test]
fn a_voids_cap_opens_to_a_seamed_band_of_its_twin() {
    let tol = Tol::witness();
    let (hollow, cap, rho) = hollow_capped_vessel();
    assert_eq!(cap.len(), 2, "the void's cap is two half-faces");
    let t = 0.05;
    let shelled = open(&hollow, &cap, t).expect("the void's cap opens");
    let body = &shelled.body;
    assert_eq!(topo::validate_geometric(body, tol), Ok(()), "tier 3");
    let rim = &shelled.naming.rims[0];
    assert_eq!(rim.side, topo::RimShell::Void);
    for &face in &cap {
        assert!(body.get_face(face).is_none(), "a designated void face dies");
        let twin = shelled.naming.inner_of(face).expect("its twin");
        let data = body.get_face(twin).expect("the twin survives as a branch");
        assert!(data.rings.is_empty(), "a seamed band carries no ring");
    }
    assert_eq!(rings_of(body), 0, "no ring anywhere");

    let (_, _, centre) = capped_vessel(0.5, 0.6, 60.0);
    let w = |a: f64, b: f64, sphere: f64| {
        let reach = (sphere * sphere - a * a).sqrt();
        PI * a * a * (centre + reach - b) + cap_volume(sphere, sphere - reach)
    };
    let outer_wall = w(0.5, 0.0, rho) - w(0.5 - t, t, rho - t);
    let cup = w(0.3 + t, 0.2 - t, rho - 0.2) - w(0.3, 0.2, rho - 0.2);
    let props = topo::mass_properties(body, tol).expect("props");
    assert!(
        (props.volume - (outer_wall + cup)).abs() <= 1e-12 + props.volume_pad,
        "volume: got {} (pad {}), want {}",
        props.volume,
        props.volume_pad,
        outer_wall + cup
    );
    mesh::tessellate(body, 1e-2, tol).expect("the void's band triangulates");
}

/// **A cap that bulges past its equator does not open**: the cavity's
/// narrower wall meets the sphere BELOW the junction with the vessel's
/// own wall, so the region between the two is not a band of the
/// designated face at all. Refused naming the shape, before any write.
#[test]
fn a_bulging_cap_whose_cavity_meets_the_sphere_below_the_wall_refuses_typed() {
    let (body, _, _) = capped_vessel(0.5, 0.6, 120.0);
    let cap = faces_on(&body, geom::SurfaceKind::Sphere);
    match open(&body, &cap, 0.05) {
        Err(ShellError::OpenFaceRimNotExpressible { face, .. }) => assert_eq!(face, cap[0]),
        other => panic!("expected the rim's shape refusal, got {other:?}"),
    }
}

/// **A window on a half-cylinder is a ring, and the mesh is what cannot
/// read it.** The D-section's curved face does not wrap, so the cavity's
/// counterpart is glued on as a ring, exactly as on a plane. Tier 3
/// passes — props reads a cylinder wall bounded by rims and rulings —
/// and the volume is the closed form; the tessellator refuses the
/// ringed wall (TESS's `a-notched-or-ringed-cylinder-wall-does-not-tessellate`).
#[test]
fn a_d_section_window_opens_to_a_ring_and_the_mesh_refuses_it() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.8, 0.05);
    let body = d_section(r, h);
    let wall = faces_on(&body, geom::SurfaceKind::Cylinder);
    assert_eq!(wall.len(), 1, "the half-cylinder is one face");
    let shelled = open(&body, &wall, t).expect("the window opens");
    let cut = &shelled.body;
    assert_eq!(topo::validate_geometric(cut, tol), Ok(()), "tier 3");
    assert_eq!(cut.shells().count(), 1, "the rim fuses the cavity in");
    let rim = cut.get_face(wall[0]).expect("the designated face survives");
    assert_eq!(rim.rings, vec![shelled.naming.rims[0].ring], "one ring");

    let segment = r * r * (t / r).acos() - t * (r * r - t * t).sqrt();
    let want = PI * r * r / 2.0 * h - segment * (h - 2.0 * t);
    let props = topo::mass_properties(cut, tol).expect("a ringed cylinder wall's props");
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "volume: got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
    assert!(
        mesh::tessellate(cut, 1e-2, tol).is_err(),
        "the tessellator reads no ringed cylinder wall yet"
    );
}

/// **A window on a sphere refuses at tier 3's check 7**, which has no
/// volume reading for a ringed sphere face. The shell op builds it and
/// discards it at its own closing validation, as the boolean does.
#[test]
fn a_sphere_window_refuses_at_check_7() {
    let body = dome_sector(1.0, 120.0);
    let zone = faces_on(&body, geom::SurfaceKind::Sphere);
    assert_eq!(zone.len(), 1, "the sector's sphere face is one window");
    match open(&body, &zone, 0.05) {
        Err(ShellError::NotValid { errors }) => assert!(
            matches!(
                errors[..],
                [topo::ValidationError::VolumeUncomputable {
                    source: topo::MassPropsError::RingOnCurvedFace { face },
                    ..
                }] if face == zone[0]
            ),
            "got {errors:?}"
        ),
        other => panic!("expected check 7's refusal, got {other:?}"),
    }
}

/// The vertex of `body` at `(rho, y)` in the meridian half-plane
/// `z = 0, x ≥ 0`, if any.
fn vertex_at(body: &Body<f64>, rho: f64, y: f64) -> Option<topo::VertexKey> {
    body.vertex_points()
        .find(|(_, p)| (p.x - rho).abs() < 1e-12 && (p.y - y).abs() < 1e-12 && p.z.abs() < 1e-12)
        .map(|(k, _)| k)
}

/// **A cone tip shells through its apex, on either nappe, sealed and
/// open.** The tip (`cone_tipped_vessel`, apex up) lies on the mirror
/// nappe and the funnel (`funnel_vessel`, apex down) on the opening
/// one; each cone face reaches its apex at a corner, and lies on the
/// nappe its other corners stand on. Sealed, the cavity's apex is the
/// moved apex, slid `t / sin α` along the axis into the material.
/// Opened through the cone, the cavity's counterpart cone is lifted
/// back onto the designated one by the nappe-turned distance
/// (`shell::lift_to`'s cone arm), and the cone keeps both half-faces as
/// a seamed band through its apex: the cavity wall's corner lands on
/// the operand's cone at radius `r − t`, and the apex dies. Opened
/// through the flat cap instead, the cavity runs out through it. Each
/// row is tier-3 valid and its volume is the closed form.
#[test]
fn a_cone_tip_shells_through_its_apex_on_either_nappe() {
    let tol = Tol::witness();
    let (r, h, k, t): (f64, f64, f64, f64) = (0.5, 0.6, 0.4, 0.05);
    let (a, slope) = (r - t, k / r);
    let slide = t * (r * r + k * k).sqrt() / r;
    let outer = PI * r * r * (h + k / 3.0);
    // `dir` points from the apex into the cone's material along `y`;
    // `cap` is the flat cap's station.
    for (what, body, nappe, apex, dir, cap) in [
        (
            "the tip",
            cone_tipped_vessel(r, h, k),
            topo::Nappe::Mirror,
            h + k,
            -1.0,
            0.0,
        ),
        (
            "the funnel",
            funnel_vessel(r, h, k),
            topo::Nappe::Opening,
            0.0,
            1.0,
            h + k,
        ),
    ] {
        let cone = faces_on(&body, geom::SurfaceKind::Cone);
        assert_eq!(cone.len(), 2, "{what}: the revolve wears two half-faces");
        for &f in &cone {
            assert_eq!(
                topo::face_nappe(&body, f, crate::common::approx::band()).expect("a nappe"),
                nappe,
                "{what}: a face reaching its apex lies on its other corners' nappe"
            );
        }
        let floor = cap - dir * t;
        let column = |top: f64| PI * a * a * (top - floor).abs();
        let corner = apex + dir * (slide + a * slope);
        let sealed_cavity = column(corner) + PI * a * a * a * slope / 3.0;
        let volume = |b: &Body<f64>| {
            let p = topo::mass_properties(b, tol).expect("props");
            (p.volume, p.volume_pad)
        };

        let sealed = topo::shell(&finished("the operand", body.clone(), tol), t, tol)
            .unwrap_or_else(|e| panic!("{what}: the sealed hollow builds, got {e:?}"))
            .body;
        assert_eq!(
            topo::validate_geometric(&sealed, tol),
            Ok(()),
            "{what}: tier 3"
        );
        assert!(
            vertex_at(&sealed, 0.0, apex + dir * slide).is_some(),
            "{what}: the cavity's apex is the moved apex"
        );
        let (got, pad) = volume(&sealed);
        let want = outer - sealed_cavity;
        assert!(
            (got - want).abs() <= 1e-12 + pad,
            "{what}: sealed volume {got} (pad {pad}), want {want}"
        );

        let opened =
            open(&body, &cone, t).unwrap_or_else(|e| panic!("{what}: the cone opens, got {e:?}"));
        let cup = &opened.body;
        assert_eq!(
            topo::validate_geometric(cup, tol),
            Ok(()),
            "{what}: open, tier 3"
        );
        assert_eq!(
            (cup.shells().count(), rings_of(cup), genus_of(cup)),
            (1, 0, 0),
            "{what}: one shell, a ring-free seamed band, genus 0"
        );
        assert_eq!(
            faces_on(cup, geom::SurfaceKind::Cone),
            cone,
            "{what}: both half-faces survive as the band, under their keys"
        );
        for &f in &cone {
            let surface = cup.get_surface(cup.get_face(f).unwrap().surface);
            assert!(
                matches!(surface, Some(geom::Surface::Cone { apex: p, .. })
                    if p.x == 0.0 && p.y == apex && p.z == 0.0),
                "{what}: the band wears the designated cone, got {surface:?}"
            );
        }
        let lifted = apex + dir * a * slope;
        assert!(
            vertex_at(cup, a, lifted).is_some(),
            "{what}: the cavity wall's corner is lifted onto the designated cone at r - t"
        );
        let tip = vertex_at(&body, 0.0, apex).expect("the operand's apex");
        assert!(cup.get_vertex(tip).is_none(), "{what}: the apex dies");
        let (got, pad) = volume(cup);
        let want = outer - column(lifted) - PI * a * a * a * slope / 3.0;
        assert!(
            (got - want).abs() <= 1e-12 + pad,
            "{what}: open volume {got} (pad {pad}), want {want}"
        );
        mesh::tessellate(cup, 1e-2, tol)
            .unwrap_or_else(|e| panic!("{what}: the band triangulates, got {e:?}"));

        let flat = faces_on(&body, geom::SurfaceKind::Plane);
        let through_cap = open(&body, &flat, t)
            .unwrap_or_else(|e| panic!("{what}: the cap opens, got {e:?}"))
            .body;
        assert_eq!(
            topo::validate_geometric(&through_cap, tol),
            Ok(()),
            "{what}: open through the cap, tier 3"
        );
        let (got, pad) = volume(&through_cap);
        let want = outer - sealed_cavity - PI * a * a * t;
        assert!(
            (got - want).abs() <= 1e-12 + pad,
            "{what}: open-through-cap volume {got} (pad {pad}), want {want}"
        );
    }
}

/// **A hemisphere tangent to its wall shells at the tangent circle.**
/// The cavity's sphere and cylinder, both at `r − t`, are tangent again
/// at the equator, and the moved corner is that circle: the meridian
/// solve's double root, whose two computed roots tie, standing on
/// neither side of the pair's foot, which is the corner. The same body
/// at the tangent bullet's scale, where the pair is too ill-conditioned
/// to solve at all, takes the same foot. Each is tier-3 valid, meshes,
/// and its volume is the closed form; opened through the floor, the
/// cavity runs out through it.
#[test]
fn a_tangent_dome_shells_at_its_tangent_circle() {
    let tol = Tol::witness();
    for (r, h, t) in [(0.5, 0.6, 0.05), (3.0 / 64.0, 8.0 / 64.0, 1.0 / 128.0)] {
        let body = domed_vessel(r, h);
        let a = r - t;
        let solid =
            |rad: f64, base: f64| PI * rad * rad * (h - base) + 2.0 * PI * rad.powi(3) / 3.0;
        let sealed_want = solid(r, 0.0) - solid(a, t);
        let hollow = topo::shell(&finished("the operand", body.clone(), tol), t, tol)
            .unwrap_or_else(|e| panic!("r = {r}: the dome shells, got {e:?}"))
            .body;
        assert_eq!(
            topo::validate_geometric(&hollow, tol),
            Ok(()),
            "r = {r}: tier 3"
        );
        assert!(
            vertex_at(&hollow, a, h).is_some(),
            "r = {r}: the cavity's corner is on the tangent circle"
        );
        let props = topo::mass_properties(&hollow, tol).expect("props");
        assert!(
            (props.volume - sealed_want).abs() <= 1e-12 * r.powi(3) + props.volume_pad,
            "r = {r}: volume {} (pad {}), want {sealed_want}",
            props.volume,
            props.volume_pad
        );
        mesh::tessellate(&hollow, r / 50.0, tol)
            .unwrap_or_else(|e| panic!("r = {r}: the hollow triangulates, got {e:?}"));

        let floor = faces_on(&body, geom::SurfaceKind::Plane);
        let cup = open(&body, &floor, t)
            .unwrap_or_else(|e| panic!("r = {r}: the floor opens, got {e:?}"))
            .body;
        assert_eq!(
            topo::validate_geometric(&cup, tol),
            Ok(()),
            "r = {r}: open, tier 3"
        );
        let props = topo::mass_properties(&cup, tol).expect("props");
        let want = sealed_want - PI * a * a * t;
        assert!(
            (props.volume - want).abs() <= 1e-12 * r.powi(3) + props.volume_pad,
            "r = {r}: open volume {} (pad {}), want {want}",
            props.volume,
            props.volume_pad
        );
    }
}

/// **A dome short of tangent by `gap` shells at its upper root, at
/// every gap.** The cap's sphere of radius `r + gap` crosses the wall at
/// a small angle, and so does the cavity's at `r − t + gap` against the
/// cavity's wall at `r − t`: two roots `±√((r − t + gap)² − (r − t)²)`
/// about the sphere's centre station, the upper one the corner. At a gap
/// inside the band that is still a crossing, not the foot between the
/// roots, where the moved surfaces are tangent and the edge's crossing
/// description would not certify. The gaps are the run's `ε` scaled: at
/// `ε/1000` the roots are too close for nearness to choose and the old
/// corner's side of the foot does, at `ε/2` nearness does, and at `20ε`
/// (a joint the profile no longer takes as tangent) the pair is plainly
/// transversal. Each is tier-3 valid with the cavity's corner on the
/// upper root, at the closed form.
#[test]
fn a_dome_short_of_tangent_shells_at_its_upper_root() {
    let tol = Tol::witness();
    let (r, h, t): (f64, f64, f64) = (0.5, 0.6, 0.05);
    let eps = tol.eps();
    for gap in [1e-3 * eps, 0.5 * eps, 20.0 * eps] {
        let (body, rho, centre) = nearly_domed_vessel(r, h, gap);
        let (a, inner) = (r - t, rho - t);
        let corner = centre + (inner * inner - a * a).sqrt();
        let outer = PI * r * r * h + cap_volume(rho, rho - (h - centre));
        let cavity = PI * a * a * (corner - t) + cap_volume(inner, inner - (corner - centre));
        let hollow = topo::shell(&finished("the operand", body, tol), t, tol)
            .unwrap_or_else(|e| panic!("gap = {gap:e}: the dome shells, got {e:?}"))
            .body;
        assert_eq!(
            topo::validate_geometric(&hollow, tol),
            Ok(()),
            "gap = {gap:e}: tier 3"
        );
        // The roots stand `2·(corner − centre)` apart; a tenth of the
        // upper root's distance from the foot tells it from the foot and
        // the lower root, with room for the cancellation in `inner² − a²`.
        let near = |p: &geom_core::Point3<f64>| (p.x - a).abs() < 1e-12 && p.z.abs() < 1e-12;
        let got: Vec<f64> = hollow
            .vertex_points()
            .filter(|(_, p)| near(p))
            .map(|(_, p)| p.y)
            .collect();
        assert!(
            got.iter()
                .any(|y| (y - corner).abs() < 0.1 * (corner - centre)),
            "gap = {gap:e}: the cavity's corner is on the upper root {corner}, got {got:?}"
        );
        let props = topo::mass_properties(&hollow, tol).expect("props");
        let want = outer - cavity;
        assert!(
            (props.volume - want).abs() <= 1e-12 + props.volume_pad,
            "gap = {gap:e}: volume {} (pad {}), want {want}",
            props.volume,
            props.volume_pad
        );
    }
}

/// **Two distinct roots the same distance from the corner still
/// refuse.** Opening the tangent dome lifts the cavity's sphere back to
/// radius `r`, which CROSSES the cavity's cylinder at `r − t` at two
/// stations `h ± √(r² − (r − t)²)`, symmetric about the old corner at
/// the equator: a transversal pair, past the tangency arm, whose two
/// answers `nearest` cannot choose between.
#[test]
fn opening_a_tangent_dome_refuses_two_equidistant_roots() {
    let body = domed_vessel(0.5, 0.6);
    let dome = faces_on(&body, geom::SurfaceKind::Sphere);
    match open(&body, &dome, 0.05) {
        Err(ShellError::Lift { error, .. }) => match *error {
            topo::ReplaceFaceError::TogetherAxialCorner { what, .. } => assert!(
                what.contains("same distance"),
                "the refusal names the tie, got {what}"
            ),
            other => panic!("expected the corner solve's tie, got {other:?}"),
        },
        other => panic!("expected the lift's refusal, got {other:?}"),
    }
}

/// **A curved designation meets the same connectivity gates as a
/// plane**: the vessel's whole side wall leaves its two caps apart.
#[test]
fn a_whole_side_wall_disconnects_the_caps() {
    let body = vessel(1.0, 2.0);
    let wall = faces_on(&body, geom::SurfaceKind::Cylinder);
    match open(&body, &wall, 0.2) {
        Err(ShellError::OpenFacesDisconnect { components: 2, .. }) => {}
        other => panic!("expected the disconnect gate, got {other:?}"),
    }
}

/// The D-section with its half-cylinder SPLIT along the ruling at
/// `u = 0` into two faces on one chart: a window of two faces that does
/// not wrap. Built through the public Euler doors, then minted and
/// finished.
fn split_d_section(r: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let mut body = d_section(r, h);
    let wall = faces_on(&body, geom::SurfaceKind::Cylinder)[0];
    let surface = body.get_face(wall).unwrap().surface;
    // The wall's two arcs, each split at its x = r point.
    let arcs: Vec<topo::EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
        })
        .map(|(k, _)| k)
        .collect();
    let mut mids = Vec::new();
    for arc in arcs {
        let key = body.get_edge(arc).unwrap().curve;
        let (t0, t1) = body
            .get_curve_geom(key)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        let made = body.split_edge(arc, 0.5 * (t0 + t1), tol).unwrap();
        mids.push(made.vertex);
    }
    let p = |b: &Body<f64>, v: topo::VertexKey| b.vertex_points().find(|(k, _)| *k == v).unwrap().1;
    let (bottom, top) = if p(&body, mids[0]).z < p(&body, mids[1]).z {
        (mids[0], mids[1])
    } else {
        (mids[1], mids[0])
    };
    let leaving = |b: &Body<f64>, v: topo::VertexKey| {
        let lk = b.get_face(wall).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = b.get_loop(lk).unwrap().boundary else {
            panic!("a cycle")
        };
        b.loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| b.get_half_edge(he).unwrap().start == v)
            .unwrap()
    };
    let (he1, he2) = (leaving(&body, bottom), leaving(&body, top));
    let (pb, pt) = (p(&body, bottom), p(&body, top));
    body.mef(
        topo::MefSite::Chords { he1, he2 },
        geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart(surface),
            carrier: geom::Curve3::Line {
                origin: pb,
                dir: (pt - pb) / (pt - pb).norm(),
            },
            param_start: 0.0,
            param_end: (pt - pb).norm(),
        },
        topo::FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// **A window of two faces that does not wrap is still a ring.** The
/// D-section's half-cylinder, split along its middle ruling into two
/// faces on one chart, carries an interior edge, but no cycle of its
/// boundary winds the period. So its two faces merge, as a plane
/// chart's do, and the window opens to the same ring and the same
/// closed form as the unsplit D-section.
#[test]
fn a_two_face_window_that_does_not_wrap_opens_to_a_ring() {
    let tol = Tol::witness();
    let (r, h, t) = (0.5, 0.8, 0.05);
    let body = split_d_section(r, h);
    let wall = faces_on(&body, geom::SurfaceKind::Cylinder);
    assert_eq!(wall.len(), 2, "the half-cylinder is split in two");
    let shelled = open(&body, &wall, t).expect("the split window opens");
    let cut = &shelled.body;
    assert_eq!(topo::validate_geometric(cut, tol), Ok(()), "tier 3");
    assert_eq!(rings_of(cut), 1, "one ring");
    let rim = faces_on(cut, geom::SurfaceKind::Cylinder);
    assert_eq!(rim.len(), 1, "the two faces merge into one rim");
    assert_eq!(
        cut.get_face(rim[0]).map(|f| f.rings.len()),
        Some(1),
        "and the rim carries the ring"
    );
    // The merge leaves each split arc's vertex between two edges of one
    // circle, on the ring and on its cavity twin; the shell ends with
    // the join, so the result holds the unsplit window's cells.
    assert_eq!(
        shelled.naming.edge_joins.len(),
        4,
        "each split arc joined back"
    );
    let whole = open(
        &d_section(r, h),
        &faces_on(&d_section(r, h), geom::SurfaceKind::Cylinder),
        t,
    )
    .expect("the unsplit window opens")
    .body;
    assert_eq!(
        (cut.vertices().count(), cut.edges().count()),
        (whole.vertices().count(), whole.edges().count()),
        "the unsplit window's cells"
    );
    let segment = r * r * (t / r).acos() - t * (r * r - t * t).sqrt();
    let want = PI * r * r / 2.0 * h - segment * (h - 2.0 * t);
    let props = topo::mass_properties(cut, tol).expect("props");
    assert!(
        (props.volume - want).abs() <= 1e-9 + props.volume_pad,
        "volume: got {} (pad {}), want {want}",
        props.volume,
        props.volume_pad
    );
}

/// The faces of `body` on a cylinder of radius `radius`, read off the
/// stored surface.
fn walls_of(body: &Body<f64>, radius: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius: r, .. }) if (*r - radius).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// The two boundary edges of a band face walking its seam twice: the
/// edges its outer loop walks once.
fn boundaries_of(body: &Body<f64>, face: FaceKey) -> Vec<topo::EdgeKey> {
    let lk = body.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
        panic!("a cycle")
    };
    let edges: Vec<_> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| body.get_half_edge(he).unwrap().edge)
        .collect();
    edges
        .iter()
        .copied()
        .filter(|e| edges.iter().filter(|x| *x == e).count() == 1)
        .collect()
}

/// The volume of the annulus `[r1, r2]` revolved, `h` tall.
fn annulus(r1: f64, r2: f64, h: f64) -> f64 {
    PI * (r2 * r2 - r1 * r1) * h
}

/// **A tube's wall wraps its period between two boundaries, and opens to
/// two seamed bands.** Each cylinder wall of the full-revolve tube is one
/// face walking its seam from one cap's circle to the other's; its cavity
/// counterpart, lifted back, lies across the middle of it. The rim is a
/// band at each end, between the wall's circle and its cavity twin: the
/// designated face keeps the band at its seam's start under its key, and
/// the band at the end is a new face on the same cylinder, recorded as the
/// rim's one hole row. Neither carries a ring, both ring loops the glue
/// made are retired, and each row's ring stands for one of the wall's two
/// circles. Opening the outer wall extends the cavity out to `ro`, the
/// inner wall in to `ri`; both are tier-3 valid, mesh, and match their
/// closed forms.
#[test]
fn a_tubes_wall_opens_to_two_seamed_bands() {
    let tol = Tol::witness();
    let (ri, ro, h, t) = (0.3, 0.5, 0.4, 0.05);
    let body = crate::common::shell_operands::tube(ri, ro, h);
    let outer = annulus(ri, ro, h);
    for (what, radius, cavity) in [
        ("outer wall", ro, annulus(ri + t, ro, h - 2.0 * t)),
        ("inner wall", ri, annulus(ri, ro - t, h - 2.0 * t)),
    ] {
        let wall = walls_of(&body, radius);
        assert_eq!(
            wall.len(),
            1,
            "{what}: the revolve wears the wall on one face"
        );
        let circles = boundaries_of(&body, wall[0]);
        assert_eq!(
            circles.len(),
            2,
            "{what}: the wall runs between two circles"
        );
        let shelled = open(&body, &wall, t).unwrap_or_else(|e| panic!("{what} opens: {e:?}"));
        let cut = &shelled.body;
        assert_eq!(topo::validate_geometric(cut, tol), Ok(()), "{what}: tier 3");
        assert_eq!(
            (cut.shells().count(), genus_of(cut)),
            (1, 1),
            "{what}: the rim fuses the cavity in, and the opened tube is genus 1"
        );
        let rim = &shelled.naming.rims[0];
        let [hole] = &rim.holes[..] else {
            panic!(
                "{what}: one hole row for the second band, got {:?}",
                rim.holes
            )
        };
        assert_eq!(
            rim.rim, wall[0],
            "{what}: the designated face is the first band"
        );
        let mut bands = walls_of(cut, radius);
        bands.sort();
        let mut want = vec![wall[0], hole.face];
        want.sort();
        assert_eq!(
            bands, want,
            "{what}: the cylinder carries exactly the two bands"
        );
        for &band in &bands {
            assert!(
                cut.get_face(band).unwrap().rings.is_empty(),
                "{what}: a seamed band carries no ring"
            );
        }
        assert!(
            shelled.naming.dead.loops.contains(&rim.ring)
                && shelled.naming.dead.loops.contains(&hole.ring),
            "{what}: both glue rings are absorbed into the bands' outer loops"
        );
        let stands_for = |rows: &[(topo::EdgeKey, topo::EdgeKey)]| -> Vec<topo::EdgeKey> {
            rows.iter().map(|&(_, source)| source).collect()
        };
        let (a, b) = (stands_for(&rim.ring_edges), stands_for(&hole.ring_edges));
        assert!(
            a.len() == 1 && b.len() == 1 && a[0] != b[0],
            "{what}: each row's ring stands for one circle, a different one each: {a:?}, {b:?}"
        );
        assert!(
            circles.contains(&a[0]) && circles.contains(&b[0]),
            "{what}: the rows' circles are the wall's own"
        );
        for (rows, band) in [(&rim.ring_edges, wall[0]), (&hole.ring_edges, hole.face)] {
            assert!(
                boundaries_of(cut, band).contains(&rows[0].0),
                "{what}: each ring's twin circle bounds its own band"
            );
        }

        let want = outer - cavity;
        let props = topo::mass_properties(cut, tol).expect("the bands' props");
        assert!(
            (props.volume - want).abs() <= 1e-12 + props.volume_pad,
            "{what}: volume: got {} (pad {}), want {want}",
            props.volume,
            props.volume_pad
        );
        for delta in [1e-2, 1e-3] {
            mesh::tessellate(cut, delta, tol).unwrap_or_else(|e| {
                panic!("{what}: the bands must triangulate at delta = {delta}, got {e:?}")
            });
        }
    }
}

/// **A void's band wall opens to two bands of its twin.** The tube shelled
/// sealed at `0.08` has a void `[0.38, 0.42] × [0.08, 0.32]` whose walls
/// are bands. Designated at `t = 0.02`, the void's face dies and its
/// dilated twin's wall, lifted back onto it, survives as the two bands;
/// the void's wall becomes a cup opening into the gap. Volume: the outer
/// thin wall plus that cup, the dilated void with its wall pulled back to
/// the designated radius, less the void.
#[test]
fn a_voids_band_wall_opens_to_two_bands_of_its_twin() {
    let tol = Tol::witness();
    let (ri, ro, h, s, t) = (0.3, 0.5, 0.4, 0.08, 0.02);
    let tube = crate::common::shell_operands::tube(ri, ro, h);
    let hollow = topo::shell(&finished("the tube", tube, tol), s, tol)
        .expect("the tube shells sealed")
        .body;
    let (vi, vo, vh) = (ri + s, ro - s, h - 2.0 * s);
    let outer_wall = annulus(ri, ro, h) - annulus(ri + t, ro - t, h - 2.0 * t);
    for (what, radius, dilated) in [
        (
            "the void's outer wall",
            vo,
            annulus(vi - t, vo, vh + 2.0 * t),
        ),
        (
            "the void's inner wall",
            vi,
            annulus(vi, vo + t, vh + 2.0 * t),
        ),
    ] {
        let wall = walls_of(&hollow, radius);
        assert_eq!(wall.len(), 1, "{what}: one face");
        let shelled = open(&hollow, &wall, t).unwrap_or_else(|e| panic!("{what} opens: {e:?}"));
        let body = &shelled.body;
        assert_eq!(
            topo::validate_geometric(body, tol),
            Ok(()),
            "{what}: tier 3"
        );
        let rim = &shelled.naming.rims[0];
        assert_eq!(rim.side, topo::RimShell::Void, "{what}: a void designation");
        assert!(
            body.get_face(wall[0]).is_none(),
            "{what}: the designated face dies"
        );
        let [hole] = &rim.holes[..] else {
            panic!("{what}: one hole row, got {:?}", rim.holes)
        };
        let mut bands = walls_of(body, radius);
        bands.sort();
        let mut want = vec![rim.rim, hole.face];
        want.sort();
        assert_eq!(
            bands, want,
            "{what}: the twin's two bands, on the designated radius"
        );
        let want = outer_wall + dilated - annulus(vi, vo, vh);
        let props = topo::mass_properties(body, tol).expect("props");
        assert!(
            (props.volume - want).abs() <= 1e-12 + props.volume_pad,
            "{what}: volume: got {} (pad {}), want {want}",
            props.volume,
            props.volume_pad
        );
        mesh::tessellate(body, 1e-2, tol)
            .unwrap_or_else(|e| panic!("{what}: the bands triangulate, got {e:?}"));
    }
}

/// The tube with its outer wall SPLIT along the ruling at `u = π` into
/// two faces on one chart: a band of two branches meeting along two
/// seams. Built through the public Euler doors, then minted.
fn split_tube(ri: f64, ro: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let mut body = crate::common::shell_operands::tube(ri, ro, h);
    let wall = walls_of(&body, ro)[0];
    let surface = body.get_face(wall).unwrap().surface;
    let seam = {
        let lk = body.get_face(wall).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            panic!("a cycle")
        };
        let edges: Vec<_> = body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().edge)
            .collect();
        let boundaries = boundaries_of(&body, wall);
        *edges
            .iter()
            .find(|e| !boundaries.contains(e))
            .expect("the wall's seam")
    };
    let mut mids = Vec::new();
    for arc in boundaries_of(&body, wall) {
        let key = body.get_edge(arc).unwrap().curve;
        let (t0, t1) = body
            .get_curve_geom(key)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        mids.push(body.split_edge(arc, 0.5 * (t0 + t1), tol).unwrap().vertex);
    }
    let p = |b: &Body<f64>, v: topo::VertexKey| b.vertex_points().find(|(k, _)| *k == v).unwrap().1;
    let leaving = |b: &Body<f64>, v: topo::VertexKey| {
        let lk = b.get_face(wall).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = b.get_loop(lk).unwrap().boundary else {
            panic!("a cycle")
        };
        b.loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| b.get_half_edge(he).unwrap().start == v)
            .unwrap()
    };
    let (he1, he2) = (leaving(&body, mids[0]), leaving(&body, mids[1]));
    let (p1, p2) = (p(&body, mids[0]), p(&body, mids[1]));
    body.mef(
        topo::MefSite::Chords { he1, he2 },
        geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart(surface),
            carrier: geom::Curve3::Line {
                origin: p1,
                dir: (p2 - p1) / (p2 - p1).norm(),
            },
            param_start: 0.0,
            param_end: (p2 - p1).norm(),
        },
        topo::FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    // The seam now parts two faces, so it is no longer a wrap edge.
    let ends = {
        let e = body.get_edge(seam).unwrap();
        let start = |he| body.get_half_edge(he).unwrap().start;
        (p(&body, start(e.he_plus)), p(&body, start(e.he_minus)))
    };
    body.set_edge_curve(
        seam,
        geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart(surface),
            carrier: geom::Curve3::Line {
                origin: ends.0,
                dir: (ends.1 - ends.0) / (ends.1 - ends.0).norm(),
            },
            param_start: 0.0,
            param_end: (ends.1 - ends.0).norm(),
        },
        tol,
    )
    .unwrap();
    topo::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// **A band of two branches does not open yet**: the tube's outer wall
/// split along a second ruling wraps between its two circles as two faces
/// meeting along two seams, and the two-band surgery cuts one seam of one
/// face. Refused naming the shape, on the first designated face.
#[test]
fn a_band_of_two_branches_refuses_typed() {
    let body = split_tube(0.3, 0.5, 0.4);
    let wall = walls_of(&body, 0.5);
    assert_eq!(wall.len(), 2, "the outer wall is split in two");
    match open(&body, &wall, 0.05) {
        Err(ShellError::OpenFaceRimNotExpressible { face, what }) => {
            assert_eq!(face, wall[0], "the refusal names the first designated face");
            assert!(what.contains("one face walking one seam"), "got {what:?}");
        }
        other => panic!("expected the band's shape refusal, got {other:?}"),
    }
}
