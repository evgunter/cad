//! **The route a curved face takes, from the outside**
//! (`tessellate::route_of`): a face whose stored images are no iso lines
//! goes to the trimmed lane without asking a door, so a door in band
//! cannot turn away a body the trimmed lane meshes; a face the doors
//! cannot decide, or whose stored data contradicts its surface, refuses
//! with their cause; and a body that meshes records the same decisions,
//! and meshes to the same bytes, as it did when the route read carrier
//! kinds alone.
//!
//! **Where the constants come from.** Each `MAIN` digest below was read on
//! the merge base this route replaced (`origin/main` at `a56edec9d6`),
//! by this file run there, and is equal on this tree: the bytes and the
//! K stream of every body here that meshed then are unmoved.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::k_stats::Bracket;
use geom_core::{Affine3, Point3, Tol, Vec3};
use mesh::TessellateError;
use sweep::test_support::{ball_poled_z, brick, finished};
use topo::Body;
use topo::construction::Construct;

use crate::common;

/// FNV-1a over `bytes`, continuing `h`.
fn fnv(h: &mut u64, bytes: &[u8]) {
    for &b in bytes {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
}

/// The mesh's bytes (positions, then each patch's face and triangles), or
/// the refusal's `Debug`.
fn mesh_digest(body: &Body<f64>, delta: f64) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325;
    match mesh::tessellate(body, delta, Tol::witness()) {
        Ok(m) => {
            for p in &m.positions {
                for x in [p.x, p.y, p.z] {
                    fnv(&mut h, &x.to_bits().to_le_bytes());
                }
            }
            for q in &m.patches {
                fnv(&mut h, format!("{:?}", q.face).as_bytes());
                for t in &q.triangles {
                    for i in t {
                        fnv(&mut h, &i.to_le_bytes());
                    }
                }
            }
        }
        Err(e) => fnv(&mut h, format!("{e:?}").as_bytes()),
    }
    h
}

/// The K-funnel's recording of `run`, verdicts and escalations in order.
fn k_digest(run: impl FnOnce()) -> u64 {
    let bracket = Bracket::open();
    run();
    let recorded = bracket.finish();
    let mut h = 0xcbf2_9ce4_8422_2325;
    for v in &recorded.verdicts {
        fnv(&mut h, format!("{}@{:?};", v.predicate, v.sign).as_bytes());
    }
    for e in &recorded.escalations {
        fnv(&mut h, format!("{}!", e.predicate()).as_bytes());
    }
    h
}

/// The `z`-poled unit ball less `brick((x0, 3), (−3, 3), (−3, 3))` turned
/// about `y` through the origin by `theta`: the section is a circle
/// whose plane is tilted `theta` off the axis, which the mint stores as a
/// projected image.
fn ball_less_a_tilted_slab(x0: f64, theta: f64) -> Body<f64> {
    let tol = Tol::witness();
    let ball = finished(
        "the ball",
        ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol),
        tol,
    );
    let slab = topo::transform_rigid(
        &brick::<f64>((x0, 3.0), (-3.0, 3.0), (-3.0, 3.0), tol),
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 1.0, 0.0), theta),
            tol,
        ),
    )
    .unwrap();
    let slab = finished("the slab", slab, tol);
    let cut = topo::subtract(&ball, &slab, tol).expect("the cut builds");
    Body::clone(&cut.body().expect("a body").body)
}

/// The unit pipe `z ∈ [−2, 2]` less a box past `z = 1` turned 0.3 rad
/// about `x`: the wall's top is two ellipse arcs, whose stored images are
/// harmonic, so the route reads the wall at its doors.
fn pipe_cut_on_a_tilt() -> Body<f64> {
    let tol = Tol::witness();
    let pipe = finished("the pipe", crate::reach_probes::rod(1.0, -2.0, 4.0), tol);
    let cap = topo::transform_rigid(
        &brick::<f64>((-3.0, 3.0), (-3.0, 3.0), (1.0, 5.0), tol),
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), 0.3),
            tol,
        ),
    )
    .unwrap();
    let cap = finished("the cap", cap, tol);
    let cut = topo::subtract(&pipe, &cap, tol).expect("the cut builds");
    Body::clone(&cut.body().expect("a body").body)
}

/// The corpus the K stream is read over: bodies that mesh on the curved
/// lane, on the trimmed lane by a stored image of a non-iso form, and the reviewer's
/// tilted cells, each at `δ = 5e-2`.
fn k_corpus() -> Vec<(String, Body<f64>)> {
    let mut out = vec![
        ("rounded prism".to_string(), common::rounded_prism()),
        ("washer".to_string(), common::washer()),
        ("ball".to_string(), common::ball()),
        ("cone".to_string(), common::cone()),
        ("donut".to_string(), common::donut()),
        (
            "ball less a slab turned 0.3".to_string(),
            ball_less_a_tilted_slab(0.3, 0.3),
        ),
        ("pipe cut on a tilt".to_string(), pipe_cut_on_a_tilt()),
    ];
    for (x0, theta) in [(0.3, 2e-9), (0.6, 8e-9)] {
        out.push((
            format!("ball less a slab at {x0} turned {theta:e}"),
            ball_less_a_tilted_slab(x0, theta),
        ));
    }
    out
}

/// The washer with its first ringless curved face put on `surface`, its
/// edges and images as they were: the face the run then routes.
fn poisoned_washer(surface: impl Fn(&Surface<f64>) -> Surface<f64>) -> (Body<f64>, topo::FaceKey) {
    let mut body = common::washer();
    let (fk, sense, was) = body
        .faces()
        .find(|(fk, f)| {
            f.rings.is_empty() && matches!(body.face_surface(*fk), Some(Surface::Cylinder { .. }))
        })
        .map(|(fk, f)| (fk, f.sense, body.face_surface(fk).unwrap().clone()))
        .expect("a ringless wall");
    // Lifts RechartStrandsDescriptions: the poisoned surface is the mesher's input, edges as they were.
    body.set_face_surface_unvouched_for_tests(
        fk,
        topo::construction::FaceSurface::New {
            surface: surface(&was),
            sense,
        },
    )
    .expect("the surface swap is accepted");
    (body, fk)
}

/// Whether no stored image on `fk`'s outer loop has a form no iso line
/// takes (`Pcurve::is_non_iso_form`), the fact the route reads first.
fn no_non_iso_form(body: &Body<f64>, fk: topo::FaceKey) -> bool {
    let topo::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(fk).unwrap().outer)
        .unwrap()
        .boundary
    else {
        return true;
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .all(|h| body.pcurve(h).is_none_or(|c| !c.pcurve().is_non_iso_form()))
}

/// **A section tilted within the band meshes to main's bytes.** The
/// ball less a slab turned a few nanoradians: the circle's axis class
/// lands in the band at the shape door (margin ≈ θ), but the mint stored
/// each sphere face's section as a projected image, which is no iso
/// line, so the route sends the face to the trimmed lane without asking
/// the door. Each cell meshes, passes `check_mesh`, and digests to the
/// bytes it meshed to on the merge base.
#[test]
fn a_section_tilted_within_the_band_meshes_to_mains_bytes() {
    const MAIN: [(f64, f64, u64); 8] = [
        (0.3, 2e-9, 0x6111_7153_4849_f20f),
        (0.3, 3e-9, 0x74de_ad4e_f00a_af5b),
        (0.3, 5e-9, 0x8ecd_def2_bd65_7168),
        (0.3, 8e-9, 0xbfa0_f2c5_2a19_7a2a),
        (0.6, 2e-9, 0xe9f2_7fcd_c00d_4eca),
        (0.6, 3e-9, 0x8e5c_03c2_c520_6252),
        (0.6, 5e-9, 0x0127_71cf_258d_a68a),
        (0.6, 8e-9, 0xf8ce_2b87_7809_4824),
    ];
    for (x0, theta, want) in MAIN {
        let body = ball_less_a_tilted_slab(x0, theta);
        let label = format!("x0 = {x0}, θ = {theta:e}");
        assert!(
            body.faces().any(|(fk, _)| {
                matches!(body.face_surface(fk), Some(Surface::Sphere { .. }))
                    && !no_non_iso_form(&body, fk)
            }),
            "{label}: a sphere face stores a projected image"
        );
        let mesh = mesh::tessellate(&body, 5e-2, Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: meshes, got {e:?}"));
        mesh::validate::check_mesh(&mesh).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        assert_eq!(mesh_digest(&body, 5e-2), want, "{label}: main's bytes");
    }
}

/// **A body that meshes records main's K stream, on a run and on a memo
/// hit.** Over bodies on the curved lane (the doors decide inside the
/// route, at the face's turn, where they decided inside the lane) and on
/// the trimmed lane by a stored image of a non-iso form (the route asks no door):
/// the bracketed recording of `tessellate`, and of a second
/// `tessellate_with` against a warm memo (every face a hit, which records
/// no decision for its face), each digest to the merge base's, as does
/// the mesh.
#[test]
fn a_body_that_meshes_records_mains_k_stream() {
    const MAIN: [(&str, u64, u64, u64); 9] = [
        (
            "rounded prism",
            0x79cc_2cd7_7222_d8cd,
            0xde76_e44f_a8f7_6685,
            0x6760_9721_f7b4_9e2f,
        ),
        (
            "washer",
            0xc0e4_7e37_d369_66fd,
            0x2949_ec2c_247d_ebf5,
            0x83a3_9809_1983_f94c,
        ),
        (
            "ball",
            0x5172_8b12_4ed0_a3c1,
            0x6958_59e5_b31e_fdcd,
            0x5812_1400_df62_e576,
        ),
        (
            "cone",
            0x2c3b_a53e_fa5c_9ac9,
            0x2212_a44a_a3dd_7fcd,
            0xee15_493c_99b8_730d,
        ),
        (
            "donut",
            0x9ff4_f3d7_e4f0_9c13,
            0x9f64_3853_64cd_16f5,
            0x10e7_f0a2_46fd_3c81,
        ),
        (
            "ball less a slab turned 0.3",
            0x5dae_4d58_bce4_166f,
            0xcbf2_9ce4_8422_2325,
            0x7fbe_fdb6_9442_bb87,
        ),
        (
            "pipe cut on a tilt",
            0xcbf2_9ce4_8422_2325,
            0xcbf2_9ce4_8422_2325,
            0x4299_0881_b1b2_3fb5,
        ),
        (
            "ball less a slab at 0.3 turned 2e-9",
            0x5dae_4d58_bce4_166f,
            0xcbf2_9ce4_8422_2325,
            0x6111_7153_4849_f20f,
        ),
        (
            "ball less a slab at 0.6 turned 8e-9",
            0x5dae_4d58_bce4_166f,
            0xcbf2_9ce4_8422_2325,
            0xf8ce_2b87_7809_4824,
        ),
    ];
    let corpus = k_corpus();
    assert_eq!(corpus.len(), MAIN.len(), "one row per body");
    for ((name, body), (want_name, run, hit, bytes)) in corpus.iter().zip(MAIN) {
        assert_eq!(name, want_name, "the corpus and the table agree on order");
        let got = k_digest(|| {
            let _ = mesh::tessellate(body, 5e-2, Tol::witness());
        });
        assert_eq!(got, run, "{name}: the run's K stream is main's");
        let mut memo = mesh::PatchMemo::new();
        let _ = mesh::tessellate_with(body, 5e-2, Tol::witness(), &mut memo);
        let got = k_digest(|| {
            let _ = mesh::tessellate_with(body, 5e-2, Tol::witness(), &mut memo);
        });
        assert_eq!(got, hit, "{name}: a memo hit's K stream is main's");
        assert_eq!(mesh_digest(body, 5e-2), bytes, "{name}: main's bytes");
    }
}

/// **A door that cannot decide refuses with its cause.** A washer wall
/// put on a cylinder 3e-9 wider than its rims and rulings: no stored
/// image on it has a non-iso form, so the route asks the doors, and
/// the shape door's incidence of the rims lands in the band
/// (`props_rim_fit`). No lane is taken on a door that could not answer:
/// the route refuses `UnsupportedCurvedShape` with the escalation, as the
/// curved lane's door did when it ran inside the lane.
///
/// The swap drops the wall's images, and none can be minted back: the
/// mint certifies each image's incidence at the same band and escalates
/// there (`pcurve_cylinder_chart_incident`, margin 3e-9), as it does on a
/// rim tilted inside the band (`pcurve_map_residual`). So a face whose
/// stored images are all iso lines and whose door lands in the band is
/// not one the mint leaves behind; a face with no image is the one that
/// reaches the doors in band.
#[test]
fn a_door_in_band_refuses_with_its_cause() {
    let (body, face) = poisoned_washer(|s| match *s {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin,
            axis,
            radius: radius + 3e-9,
            u_ref,
        },
        _ => unreachable!("the poisoned face is a cylinder wall"),
    });
    assert!(
        no_non_iso_form(&body, face),
        "no stored image on the wall has a non-iso form"
    );
    let got = mesh::tessellate(&body, 5e-2, Tol::witness()).map(|_| ());
    assert!(
        matches!(
            &got,
            Err(TessellateError::UnsupportedCurvedShape {
                face: f,
                source: geom_brep::props::PropsError::Escalated { cause, .. },
            }) if *f == face && cause.predicate == Some("props_rim_fit")
        ),
        "{got:?}"
    );
}

/// **An edge off its face's surface refuses with that cause.** The same
/// wall put on a cone whose axis is across the washer's: its rims lie on
/// no circle of the cone, which the shape door decides at the circle's
/// axis class (`OffSurface`), and the route refuses with it.
#[test]
fn an_edge_off_its_surface_refuses_with_that_cause() {
    let (body, face) = poisoned_washer(|_| Surface::Cone {
        apex: Point3::new(0.0, 0.0, 4.0),
        axis: Vec3::new(0.0, 0.0, -1.0),
        half_angle: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    });
    assert_eq!(
        mesh::tessellate(&body, 5e-2, Tol::witness()).map(|_| ()),
        Err(TessellateError::UnsupportedCurvedShape {
            face,
            source: geom_brep::props::PropsError::OffSurface {
                what: "props_circle_axis_class",
            },
        })
    );
}

/// The pipe cut on a tilt with its wall put on a cylinder 3e-9 wider
/// than its rims, rulings and ellipse arcs: the swap drops the wall's
/// images, so the route reads it at its doors.
fn poisoned_ellipse_wall() -> (Body<f64>, topo::FaceKey) {
    let mut body = pipe_cut_on_a_tilt();
    let (fk, sense, was) = body
        .faces()
        .find(|(fk, f)| {
            f.rings.is_empty() && matches!(body.face_surface(*fk), Some(Surface::Cylinder { .. }))
        })
        .map(|(fk, f)| (fk, f.sense, body.face_surface(fk).unwrap().clone()))
        .expect("the wall");
    let Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref,
    } = was
    else {
        unreachable!("the wall is a cylinder")
    };
    // Lifts RechartStrandsDescriptions: the poisoned surface is the mesher's input, edges as they were.
    body.set_face_surface_unvouched_for_tests(
        fk,
        topo::construction::FaceSurface::New {
            surface: Surface::Cylinder {
                origin,
                axis,
                radius: radius + 3e-9,
                u_ref,
            },
            sense,
        },
    )
    .unwrap();
    (body, fk)
}

/// **A carrier of no iso kind is read before anything in band.** The
/// ellipse-cut wall, its stored data 3e-9 off its surface: the shape
/// door's first pass reads the ellipse's kind and decides the wall is no
/// rectangle with no decision taken, so no rim's or ruling's incidence
/// can escalate ahead of it, wherever the loop starts. The trimmed lane
/// then finds no stored image to read, and refuses that — as it did when
/// the route read carrier kinds, where this wall went to it unasked.
#[test]
fn a_carrier_of_no_iso_kind_is_read_before_anything_in_band() {
    let (body, face) = poisoned_ellipse_wall();
    let got = mesh::tessellate(&body, 5e-2, Tol::witness()).map(|_| ());
    assert!(
        matches!(
            &got,
            Err(TessellateError::DomainUnread { face: f, cause })
                if *f == face && matches!(**cause, topo::pcurves::RingPlacementError::Gap(_))
        ),
        "{got:?}"
    );
}
