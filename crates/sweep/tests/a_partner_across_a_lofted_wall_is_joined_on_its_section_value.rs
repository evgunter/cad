//! **A partner across a lofted wall is joined on the pair's section
//! value.** The loft prism's walls are splines; a ball or a brick across
//! one meets it in an arc from the wall's rim to its rim, or from side to
//! side, which the join reads as the carried component the section value
//! lists, its chords cut from the reading's own certified carrier on
//! both operands. Each run passes the join's lanes and the inside/outside
//! test, which crosses the spline walls (`geom_brep::curve_face::line_patch`),
//! and builds, or stops past both: at the result's own checks, or at the
//! at-rest certificate of the chord the join mints (a plane's arc across a
//! curved wall stops at the plane × spline lane's own image). Two arcs of
//! one pair a rung apart pass the carry and the join too. A brick wholly inside
//! a wall's face meets it in a closed loop, a ring of the wall the join
//! walls off as an island; a slab round the prism meets its walls in a
//! closed loop on each of its own big faces, a ring of a plane.
//!
//! Each run is at the witness tolerance. Red against main before
//! the section value carried the pair: each refused at the join, where a
//! germ on a spline wall has no frame (`GermFrameUnsupported`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_z, finished, loft_prism};
use topo::{BooleanError, EulerOpError, JoinError};

/// The run's refusal as the chord edge's certification, if it is one.
fn chord_certificate(e: &BooleanError) -> Option<&geom_brep::CertifyError> {
    let BooleanError::Join(join) = e else {
        return None;
    };
    match join.error() {
        JoinError::Euler(EulerOpError::DescriptionUncertified { error, .. }) => Some(error),
        _ => None,
    }
}

/// Whether the run passed the join's lanes and the inside/outside test
/// and stopped past both: at the result's checks, or, at `ε = 1e-12`, at
/// the chord's certificate, the lane's image of the arc missing its foot
/// path.
fn stops_past_the_join(refusal: &BooleanError, t: Tol) -> bool {
    match chord_certificate(refusal) {
        Some(
            geom_brep::CertifyError::PlaneNurbs(geom_brep::PlaneNurbsRefusal::Limb {
                limb: geom_brep::SsiLimb::ImageMiss,
                ..
            })
            | geom_brep::CertifyError::Escalated {
                check: geom_brep::CertCheck::PlaneNurbsHullValue,
                ..
            },
        ) => t.eps() == 1e-12,
        Some(_) => false,
        None => !matches!(
            refusal,
            BooleanError::Join(_) | BooleanError::Containment(_)
        ),
    }
}

/// **The run builds a valid body, or stops past the join and the
/// inside/outside test** ([`stops_past_the_join`]).
fn builds_or_stops_past_the_join(
    what: &str,
    run: Result<topo::BooleanResult<f64>, BooleanError>,
    t: Tol,
) {
    match run {
        Ok(built) => {
            let body = &built
                .body()
                .expect("a union or a difference here is never empty")
                .body;
            assert_eq!(topo::validate_geometric(body, t), Ok(()), "{what}: tier 3");
        }
        Err(refusal) => assert!(
            stops_past_the_join(&refusal, t),
            "{what}: the join and the inside/outside test pass, the run stopping past \
             both: {refusal:?}"
        ),
    }
}

/// **A ball across the prism's wall and its bottom cap**: the ball meets
/// the wall in an arc from the wall's bottom rim back to it, which the
/// join carries; the chord it mints on the wall and the ball is an
/// intersection of a sphere and a spline, which the at-rest certificate
/// holds through its curved-partner lane. The join completes, the
/// inside/outside test crosses the wall, and at the witness ε the union
/// stops at the result's edge-in-face check, which reads the chord on the
/// wall's carrier no further
/// (`work/sectarms/the-join-reads-a-spline-implicit-section.md`).
///
/// At `ε = 1e-12` the chord stops at its certificate instead: the lane's
/// image of the arc on the wall misses its foot path by more than that ε
/// (`work/sectarms/a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung.md`).
///
/// Red against: a certificate with no arm for a sphere beside a spline
/// (`CertifyError::Unimplemented`); the join reading a spline × sphere
/// germ through a frame (`GermFrameUnsupported`); a table that does not
/// hold the pair.
#[test]
fn a_ball_across_a_lofted_wall_is_joined_and_its_chord_certifies_at_rest() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    for (what, c) in [
        ("the flat wall", [0.0, 0.9, 0.1]),
        ("the flared wall", [0.2, -1.05, 0.1]),
    ] {
        let ball = finished(
            "the ball",
            ball_poled_z(0.25, Vec3::new(c[0], c[1], c[2]), t),
            t,
        );
        builds_or_stops_past_the_join(what, topo::union(&prism, &ball, t), t);
    }
}

/// **A ball across two close arcs of a lofted wall passes the carry and
/// the join**: on the prism whose sections stand `0.2` apart, the ball
/// of radius `0.21` about `(0.3, 0.95, 0.2)` cuts a wall in two rim arcs
/// about a rung apart. What the row holds is where each run stops, in
/// union, subtract and swapped union: past the carry, the join and the
/// inside/outside test, built, or stopped at the result's checks or at
/// `ε = 1e-12` at the chord's certificate (`builds_or_stops_past_the_join`). That each arc's
/// sites are read in its own cells is `geom-brep`'s to show
/// (`pair_section::spline_implicit::cells::rows`); a run that read a
/// site on both arcs would abort in the join, before either stop.
///
/// Red against: a carry that holds no two arcs apart, where a site on
/// one arc read on both and the run aborted `JoinDesync`; a pad read
/// off the windows' whole images, where the carry refused the two arcs
/// `PadsMeet` (`GermSectionUnbuilt`).
#[test]
fn a_ball_across_two_close_arcs_of_a_lofted_wall_passes_the_carry_and_the_join() {
    let t = Tol::witness();
    let prism = finished(
        "the shallow prism",
        sweep::test_support::loft_prism_at(&[0.0, 0.2, 0.4], t),
        t,
    );
    let ball = finished(
        "the ball",
        ball_poled_z(0.21, Vec3::new(0.3, 0.95, 0.2), t),
        t,
    );
    for (what, run) in [
        ("prism ∪ ball", topo::union(&prism, &ball, t)),
        ("prism ∖ ball", topo::subtract(&prism, &ball, t)),
        ("ball ∪ prism", topo::union(&ball, &prism, t)),
    ] {
        builds_or_stops_past_the_join(what, run, t);
    }
}

/// **Every operation of `a` and `b` builds a valid body, and their
/// volumes agree**: the union and the common part together are the two
/// operands, each difference its operand less the common part.
fn every_op_builds_and_its_volumes_agree(
    what: &str,
    a: &topo::AtRestBody<f64>,
    b: &topo::AtRestBody<f64>,
    t: Tol,
) {
    let volume = |body: &topo::AtRestBody<f64>| {
        topo::mass_properties(body, t, topo::test_support::pinned_resolution(t))
            .expect("the body measures")
            .met()
            .expect("the measure settles")
            .volume
    };
    let built = |run: Result<topo::BooleanResult<f64>, BooleanError>, op: &str| {
        let run = run.unwrap_or_else(|e| panic!("{what}: {op} builds: {e:?}"));
        let body = run
            .body()
            .unwrap_or_else(|| panic!("{what}: {op} is not empty"))
            .body
            .clone();
        assert_eq!(
            topo::validate_geometric(&body, t),
            Ok(()),
            "{what}: {op}: tier 3"
        );
        volume(&body)
    };
    let (va, vb) = (volume(a), volume(b));
    let joined = built(topo::union(a, b, t), "a ∪ b");
    let swapped = built(topo::union(b, a, t), "b ∪ a");
    let common = built(topo::intersect(a, b, t), "a ∩ b");
    let less = built(topo::subtract(a, b, t), "a ∖ b");
    let other = built(topo::subtract(b, a, t), "b ∖ a");
    let near = |x: f64, y: f64| (x - y).abs() < 1e-9;
    assert!(
        common > 0.0
            && near(joined, swapped)
            && near(joined + common, va + vb)
            && near(less, va - common)
            && near(other, vb - common),
        "{what}: ∪ {joined} and {swapped}, ∩ {common}, ∖ {less} and {other} against {va} and {vb}"
    );
}

/// **A brick across the prism's wall**: the brick's planes meet the wall
/// in arcs the join carries, each plane's chord an arc of the same
/// carrier as the wall's. Across the flat wall at its rim the guard reads
/// each of the wall's pairs off its listing and clears it, the join
/// completes, the inside/outside test crosses the wall, and every
/// operation builds a valid body whose volumes agree: the union and the
/// common part together are the two operands, the differences each
/// operand less the common part. Across the
/// flared wall the plane × spline certificate refuses the chord, its own
/// image missing it (at `ε = 1e-6`, its hull value escalating first).
///
/// Red against: a carried chord refused on a planar face; the join
/// reading a plane × spline germ through a frame.
#[test]
fn a_brick_across_a_lofted_wall_is_joined_on_its_carried_arcs() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let brick = |x, y, z| {
        finished(
            "the brick",
            topo::test_support::brick_with_fresh_keys::<f64>(x, y, z, t),
            t,
        )
    };
    let flat = brick((-0.25, 0.25), (0.75, 1.25), (-0.25, 0.25));
    let report = topo::test_support::section_report(topo::BooleanOp::Union, &prism, &flat, t)
        .expect("the reduction reads");
    let cleared: Vec<_> = report
        .iter()
        .filter(|(fa, _, verdict)| {
            matches!(prism.face_surface(*fa), Ok(geom::Surface::Nurbs(_)))
                && verdict.starts_with("Ok(")
        })
        .collect();
    assert!(
        !cleared.is_empty(),
        "the flat wall: the brick crosses it, and the guard clears its pairs: {report:?}"
    );
    every_op_builds_and_its_volumes_agree("the flat wall's brick", &prism, &flat, t);
    let flared = brick((-0.25, 0.35), (-1.3, -0.8), (0.75, 1.25));
    let refusal = topo::union(&prism, &flared, t).expect_err("the plane × spline lane refuses");
    // At `ε = 1e-6` the lane's hull value escalates before its image is
    // read; at every other row the image misses the chord.
    let read = match chord_certificate(&refusal) {
        Some(geom_brep::CertifyError::Escalated {
            check: geom_brep::CertCheck::PlaneNurbsHullValue,
            ..
        }) => t.eps() == 1e-6,
        Some(geom_brep::CertifyError::PlaneNurbs(_)) => t.eps() != 1e-6,
        _ => false,
    };
    assert!(
        read,
        "the flared wall: the join mints the carried chord, which the plane × spline lane \
         reads: {refusal:?}"
    );
}

/// **A brick wholly inside the flat wall's face closes a ring on the
/// wall**: its four planes meet the wall in a closed loop of four
/// carried arcs, and the chord that closes the loop walls an island off
/// the wall, wound by paths along the wall's own iso curves. Every
/// operation passes the join and the inside/outside test; at the witness
/// ε the result's volume reads no ring on a curved face
/// (`VolumeUncomputable { source: RingOnCurvedFace }`).
///
/// Red against: a ring lane with no winding on a spline wall, where each
/// run refused `CurvedBooleanUnsupported` naming the wall. Which way
/// round the island winds is not read here, since the stop comes before
/// any reader of the wall's two faces: topo's
/// `a_spline_wall_island_winds_as_its_corners_turn_about_the_outward_normal`
/// holds it.
#[test]
fn a_brick_inside_a_lofted_walls_face_closes_a_ring_on_the_wall() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let brick = finished(
        "the brick",
        topo::test_support::brick_with_fresh_keys::<f64>(
            (-0.25, 0.25),
            (0.75, 1.25),
            (0.75, 1.25),
            t,
        ),
        t,
    );
    for (what, run) in [
        ("prism ∪ brick", topo::union(&prism, &brick, t)),
        ("prism ∖ brick", topo::subtract(&prism, &brick, t)),
        ("prism ∩ brick", topo::intersect(&prism, &brick, t)),
        ("brick ∪ prism", topo::union(&brick, &prism, t)),
        ("brick ∖ prism", topo::subtract(&brick, &prism, t)),
    ] {
        builds_or_stops_past_the_join(what, run, t);
    }
}

/// **A fork with two prongs inside the flat wall's face closes two
/// rings on the wall**: when the first island is walled off, the other
/// prong's pierce rings are bystanders on the wall, each placed out of
/// the island by a path on the wall. The union passes the join and the
/// inside/outside test.
///
/// Red against: a re-homing with no reading on a spline wall, where the
/// first island's chord refused `RingIslandUnread` naming the wall.
#[test]
fn a_fork_inside_a_lofted_walls_face_closes_two_rings_on_the_wall() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let brick = |x, y| {
        finished(
            "a brick",
            topo::test_support::brick_with_fresh_keys::<f64>(x, y, (0.75, 1.25), t),
            t,
        )
    };
    let joined = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>| {
        topo::union(a, b, t)
            .expect("two bricks sharing a face unite")
            .body()
            .expect("a union is never empty")
            .body
            .clone()
    };
    let bridge = brick((-0.6, 0.6), (1.5, 1.8));
    let fork = joined(
        &joined(&brick((-0.6, -0.3), (0.75, 1.5)), &bridge),
        &brick((0.3, 0.6), (0.75, 1.5)),
    );
    builds_or_stops_past_the_join("prism ∪ fork", topo::union(&prism, &fork, t), t);
}

/// **A fork with one prong inside the flat wall's face and one across
/// its bottom rim refuses by name**: the rim prong's chords run from the
/// wall's rim back to it, so the face one of them walls off is bounded
/// by the wall's own rim edges beside the chords, and re-homing the
/// inside prong's rings against that face reads a rim edge, which is no
/// certified section of the wall. `RingIslandUnread` naming a spline
/// face, in `prism ∪ fork` and `prism ∖ fork`
/// (`work/sectarms/re-homing-a-ring-on-a-spline-wall-refuses-where-the-new-face-holds-a-rim-edge.md`).
#[test]
fn a_fork_inside_and_across_the_rim_of_a_lofted_wall_refuses_its_re_homing_by_name() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let brick = |x, y, z| {
        finished(
            "a brick",
            topo::test_support::brick_with_fresh_keys::<f64>(x, y, z, t),
            t,
        )
    };
    let joined = |a: &topo::AtRestBody<f64>, b: &topo::AtRestBody<f64>| {
        topo::union(a, b, t)
            .expect("two bricks sharing a face unite")
            .body()
            .expect("a union is never empty")
            .body
            .clone()
    };
    let z = (0.75, 1.25);
    let fork = joined(
        &joined(
            &brick((-0.6, -0.3), (0.75, 1.5), z),
            &brick((-0.6, 0.6), (1.5, 1.8), z),
        ),
        &brick((0.3, 0.6), (0.75, 1.5), (-0.25, 1.25)),
    );
    for (what, run) in [
        ("prism ∪ fork", topo::union(&prism, &fork, t)),
        ("prism ∖ fork", topo::subtract(&prism, &fork, t)),
    ] {
        let refusal = run.expect_err("the re-homing reads no rim edge");
        let BooleanError::Join(join) = &refusal else {
            panic!("{what}: the join refuses the re-homing: {refusal:?}");
        };
        assert!(
            matches!(
                join.error(),
                JoinError::RingIslandUnread {
                    kind: geom::SurfaceKind::Nurbs,
                    ..
                }
            ),
            "{what}: the re-homing names the wall: {refusal:?}"
        );
    }
}

/// **A slab round the prism closes a ring on each of its big faces**:
/// the slab `x, y ∈ [−3, 3]`, `z ∈ [0.75, 1.25]` meets the prism's four
/// walls in a closed loop of four carried arcs on its top and on its
/// bottom. Each loop is a ring of a planar face, whose island is wound
/// on the face's own plane by the arcs' own areas, the closing chord's
/// with them. Every operation, in either order, passes the join and the
/// inside/outside test, and builds: the volumes agree.
///
/// Red against: a ring lane that closes a carried chord on a curved face
/// alone, where `prism ∪ slab` and `slab ∖ prism` refused
/// `CurvedBooleanUnsupported` naming the slab's plane; a planar winding
/// with no term for a spline edge.
#[test]
fn a_slab_round_the_prism_closes_a_ring_on_each_of_its_big_faces() {
    let t = Tol::witness();
    let prism = finished("the loft prism", loft_prism(t), t);
    let slab = finished(
        "the slab",
        topo::test_support::brick_with_fresh_keys::<f64>((-3.0, 3.0), (-3.0, 3.0), (0.75, 1.25), t),
        t,
    );
    every_op_builds_and_its_volumes_agree("the slab", &prism, &slab, t);
}
