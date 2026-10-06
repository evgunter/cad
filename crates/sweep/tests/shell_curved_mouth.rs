//! **`shell_open` through a curved designated face.**
//!
//! A designated face on a periodic chart opens in its chart's own form:
//! a chart that wraps its period through a pole becomes a seamed band
//! (each of its faces kept, its seams cut short at the cavity's
//! corners), and a window that does not wrap becomes a ring, as on a
//! plane. What the readers cannot yet read about a ringed curved window
//! refuses where they read it — the mesh, or tier 3's check 7 — and not
//! in the shell op. Each built row is checked against its closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::census::{genus_of, rings_of};
use crate::common::shell_operands::{
    capped_vessel, cone_tipped_vessel, d_section, dome_sector, domed_vessel, hollow_capped_vessel,
    vessel,
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

/// **A cone tip and a tangent dome refuse before the rim stage**: the
/// sealed hollow of each refuses at the cavity's own offset door — a
/// cone face that reaches its apex has no nappe to turn a distance by,
/// and a hemisphere tangent to its wall meets the cavity's wall at a
/// double corner — and opening either refuses identically, so neither
/// refusal is the mouth's.
#[test]
fn a_cone_tip_and_a_tangent_dome_refuse_in_the_sealed_arm() {
    let tol = Tol::witness();
    for (what, body, kind) in [
        (
            "the cone tip",
            cone_tipped_vessel(0.5, 0.6, 0.4),
            geom::SurfaceKind::Cone,
        ),
        (
            "the tangent dome",
            domed_vessel(0.5, 0.6),
            geom::SurfaceKind::Sphere,
        ),
    ] {
        let sealed = topo::shell(&finished("the operand", body.clone(), tol), 0.05, tol)
            .expect_err("the sealed hollow refuses");
        let opened = open(&body, &faces_on(&body, kind), 0.05).expect_err("so does the open one");
        let door = |e: &ShellError<f64>| match e {
            ShellError::Face { error, .. } => format!("{error:?}"),
            other => panic!("{what}: expected the cavity door's refusal, got {other:?}"),
        };
        assert_eq!(
            door(&sealed),
            door(&opened),
            "{what}: one refusal, sealed or open"
        );
        assert!(
            matches!(
                (&sealed, what),
                (ShellError::Face { error, .. }, "the cone tip")
                    if matches!(**error, topo::ReplaceFaceError::NappeStraddles { .. })
            ) || matches!(
                (&sealed, what),
                (ShellError::Face { error, .. }, "the tangent dome")
                    if matches!(**error, topo::ReplaceFaceError::TogetherAxialCorner { .. })
            ),
            "{what}: got {sealed:?}"
        );
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
