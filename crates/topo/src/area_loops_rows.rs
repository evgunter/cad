//! **What a loop that is not a cycle means to a question about a face's
//! region** ([`crate::live::FaceView::area_loops`]): a lone ring is a
//! point of the boundary that bounds no area, and a lone outer loop is a
//! face with no outline, which every area reader refuses.

use geom_core::{Band, Point3, Tol, Vec3};

use crate::contain::{ContainError, FaceContainment, contfp};

use crate::Body;
use crate::test_support_fixtures::box_with_a_lone_ring;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("a band")
}

fn up() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

/// The facet reads the ring as a boundary point, and the square as the
/// region's one cycle.
#[test]
fn a_lone_ring_is_a_boundary_point_that_bounds_no_area() {
    let (body, top, lone) = box_with_a_lone_ring(Tol::witness());
    let area = body.proven_face(top).area_loops().expect("an outline");
    assert_eq!(area.cycles().count(), 1, "the square alone bounds area");
    let rings = area.lone_rings();
    assert_eq!(rings.len(), 1, "one lone ring");
    assert_eq!(rings[0].vertex, lone, "the ring's vertex");
    let p = rings[0].point;
    assert_eq!([p.x, p.y, p.z], [0.5, 0.5, 1.0], "its point");
}

/// Red where the boundary pre-pass refused the whole face as
/// `EmptyLoop`: the ring bounds no area, and its vertex is a vertex of
/// the face's boundary.
#[test]
fn contfp_reads_a_lone_ring_vertex_as_a_vertex_of_the_face() {
    let (body, top, lone) = box_with_a_lone_ring(Tol::witness());
    assert_eq!(
        contfp(&body, top, up(), Point3::new(0.5, 0.5, 1.0), band()),
        Ok(FaceContainment::OnVertex(lone)),
        "a point at the ring's vertex"
    );
    assert_eq!(
        contfp(&body, top, up(), Point3::new(0.25, 0.75, 1.0), band()),
        Ok(FaceContainment::In),
        "a point clear of the ring"
    );
}

/// Red where the ray walk skipped the ring: a hit on its vertex is a hit
/// on the boundary, a graze.
#[test]
fn a_ray_through_a_lone_ring_vertex_grazes() {
    let (body, top, _) = box_with_a_lone_ring(Tol::witness());
    let hit = |p| crate::solid_contain::point_in_face(&body, top, up(), p, band());
    assert_eq!(hit(Point3::new(0.5, 0.5, 1.0)), Ok(None), "at the ring");
    assert_eq!(
        hit(Point3::new(0.25, 0.75, 1.0)),
        Ok(Some(true)),
        "clear of it"
    );
}

/// Red where the tips read refused the face as an empty loop "at rest":
/// the ring's vertex is a point the face holds and its image drops.
#[test]
fn a_lone_rings_vertex_is_one_of_the_faces_tips() {
    let (body, top, _) = box_with_a_lone_ring(Tol::witness());
    let tips = crate::props::face_tips(body.proven_face(top)).expect("the face's tips");
    assert!(
        tips.iter().any(|p| [p.x, p.y, p.z] == [0.5, 0.5, 1.0]),
        "the ring's point among the tips: {tips:?}"
    );
}

/// A face whose outer loop is a lone vertex has no outline: every area
/// reader refuses it as the lone loop it is. Red where the ray walk read
/// it as a miss (`Some(false)`) and the region reading as `CorruptFace`.
#[test]
fn a_face_with_no_outline_is_refused_by_every_area_reader() {
    let mut body = Body::<f64>::new();
    let seed = body
        .mvfs(Point3::new(0.0, 0.0, 0.0), true)
        .expect("a skeleton");
    let no = body
        .proven_face(seed.face)
        .area_loops()
        .err()
        .expect("no outline");
    assert_eq!(
        (no.face, no.r#loop, no.vertex),
        (seed.face, seed.r#loop, seed.vertex),
        "the refusal names the face, its loop and the vertex"
    );
    // The one refusal of a face with no outline, which names its loop.
    let lone = crate::solid_contain::no_outline(no);
    assert!(
        format!("{lone:?}").contains(&format!("{:?}", seed.r#loop)),
        "the refusal names the loop: {lone:?}"
    );
    let at = Point3::new(0.0, 0.0, 0.0);
    assert_eq!(
        crate::solid_contain::point_in_face(&body, seed.face, up(), at, band()),
        Err(lone.clone()),
        "the ray walk"
    );
    assert!(
        matches!(
            crate::region::boundary(&body, seed.face, &[]),
            Err(e) if e == lone
        ),
        "the region reading"
    );
    assert_eq!(
        contfp(&body, seed.face, up(), at, band()),
        Err(ContainError::EmptyLoop(seed.r#loop)),
        "the face door"
    );
    assert!(
        matches!(
            crate::props::face_tips(body.proven_face(seed.face)),
            Err(crate::MassPropsError::NoOutline(n)) if n == no
        ),
        "the measure"
    );
}

// ---- reviewer probes (PR 7474) ----

fn raw(body: &Body<f64>) -> std::thread::Result<String> {
    let tol = Tol::witness();
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        format!(
            "{:?}",
            crate::test_support::measure_unvalidated(
                body,
                tol,
                crate::test_support::pinned_resolution(tol)
            )
            .map(|m| m.enclosure.volume)
        )
    }))
}

/// A half-edge of the top face whose parent_loop names another loop:
/// next is intact, so `loop_cycle` closes; the claims walk does not.
#[test]
fn probe_raw_door_on_a_parent_loop_tear() {
    let (mut body, top, _) = box_with_a_lone_ring(Tol::witness());
    let he = body.proven_face(top).half_edges()[1];
    let other = body
        .faces
        .iter()
        .find(|(k, _)| *k != top)
        .map(|(_, f)| f.outer)
        .unwrap();
    body.half_edges.get_mut(he).unwrap().parent_loop = other;
    let r = raw(&body);
    eprintln!(
        "PROBE parent_loop tear: {:?}",
        r.as_ref().map_err(|_| "PANIC")
    );
    assert!(r.is_ok(), "the raw door panics on a parent_loop tear");
}

/// The lone ring's vertex removed: a dead key behind a lone loop.
#[test]
fn probe_raw_door_on_a_dead_lone_ring_vertex() {
    let (mut body, _, lone) = box_with_a_lone_ring(Tol::witness());
    body.vertices.remove(lone);
    let r = raw(&body);
    eprintln!(
        "PROBE dead lone vertex: {:?}",
        r.as_ref().map_err(|_| "PANIC")
    );
    assert!(r.is_ok(), "the raw door panics on a dead lone-ring vertex");
}

/// The intact lone-ring box through the raw door.
#[test]
fn probe_raw_door_on_the_lone_ring_box() {
    let (body, _, _) = box_with_a_lone_ring(Tol::witness());
    let r = raw(&body);
    eprintln!("PROBE intact: {:?}", r.as_ref().map_err(|_| "PANIC"));
}

/// A plain brick, one top half-edge's parent_loop re-pointed.
#[test]
fn probe_raw_door_on_a_brick_parent_loop_tear() {
    let mut body = crate::test_support_fixtures::brick::<f64>(
        (0.0, 1.0),
        (0.0, 1.0),
        (0.0, 1.0),
        Tol::witness(),
    );
    let fk = body.faces.iter().next().map(|(k, _)| k).unwrap();
    let he = body.proven_face(fk).half_edges()[1];
    let other = body
        .faces
        .iter()
        .find(|(k, _)| *k != fk)
        .map(|(_, f)| f.outer)
        .unwrap();
    body.half_edges.get_mut(he).unwrap().parent_loop = other;
    let r = raw(&body);
    eprintln!(
        "PROBE brick parent_loop tear: {:?}",
        r.as_ref().map_err(|_| "PANIC")
    );
    assert!(r.is_ok(), "the raw door panics on a parent_loop tear");
}
