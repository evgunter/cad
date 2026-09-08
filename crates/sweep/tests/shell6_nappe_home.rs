//! **The cone nappe has one home.** `geom_brep::offset_surface` slides a
//! cone's apex along the OPENING nappe's normal field, so a
//! mirror-nappe face's material moves `−d` along its own chart normal.
//! Which nappe a FACE lies on is a fact only the face has, and
//! `topo::face_nappe` is where the tree decides it: both offset doors,
//! the per-chart door's apex-window gate and
//! `geom_brep::ConeOffset::displacement` turn by that one answer.
//!
//! These rows pin the answer itself, the two doors' agreement over it,
//! the typed refusal where a face has no nappe, and the pointwise
//! displacement against the per-point read it replaced.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{ConeOffset, Nappe};
use geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, ReplaceFaceError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The wall thickness the small-offset rows use.
const T: f64 = 1.0 / 128.0;

/// The narrowing frustum's wall sits BELOW its apex (the mirror nappe);
/// the widening one's sits above it (the opening nappe). One `h` and
/// one `|tan α|` for both, so their windows are mirror images.
const H: f64 = 8.0 / 64.0;
const R_WIDE: f64 = 4.0 / 64.0;
const R_NARROW: f64 = 2.0 / 64.0;

fn frustum(r0: f64, r1: f64) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r0, 0.0), 0.0),
            ProfileVertex::new(p2(r1, H), 0.0),
            ProfileVertex::new(p2(0.0, H), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// Narrowing upward: the wall is below its apex.
fn mirror_frustum() -> Body<f64> {
    frustum(R_WIDE, R_NARROW)
}

/// Widening upward: the wall is above its apex.
fn opening_frustum() -> Body<f64> {
    frustum(R_NARROW, R_WIDE)
}

/// The faces wearing the frustum's cone. A full revolve splits the wall
/// into two bands over ONE surface key, so the chart is a group.
fn cone_group(body: &Body<f64>) -> Vec<FaceKey> {
    let group: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cone { .. })))
        .map(|(k, _)| k)
        .collect();
    assert!(!group.is_empty(), "a frustum wears a cone chart");
    let key = body.get_face(group[0]).unwrap().surface;
    assert!(
        group
            .iter()
            .all(|&k| body.get_face(k).unwrap().surface == key),
        "the wall's bands share one surface key"
    );
    group
}

/// One [`topo::ChartMove`] per surface key: the axial door names every
/// face of the body, and a chart is moved once however many bands wear
/// it.
fn chart_moves(body: &Body<f64>, d: f64) -> Vec<topo::ChartMove<f64>> {
    let mut moves: Vec<topo::ChartMove<f64>> = Vec::new();
    for (k, f) in body.faces() {
        match moves
            .iter_mut()
            .find(|m| body.get_face(m.faces[0]).unwrap().surface == f.surface)
        {
            Some(m) => m.faces.push(k),
            None => moves.push(topo::ChartMove {
                faces: vec![k],
                distance: d,
            }),
        }
    }
    moves
}

fn cone_of(body: &Body<f64>, face: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

/// Every corner of `face`, in `next` order over every loop — the very
/// points [`topo::face_nappe`] sums.
fn corners(body: &Body<f64>, face: FaceKey) -> Vec<Point3<f64>> {
    let data = body.get_face(face).unwrap();
    let mut out = Vec::new();
    for lk in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        let topo::LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            let v = body.get_half_edge(he).unwrap().start;
            out.push(*body.get_point(body.get_vertex(v).unwrap().point).unwrap());
        }
    }
    out
}

/// **The home answers both nappes, from the face's own corners.**
#[test]
fn face_nappe_reads_the_frustums_own_wall() {
    for (what, body, want) in [
        ("narrowing upward", mirror_frustum(), Nappe::Mirror),
        ("widening upward", opening_frustum(), Nappe::Opening),
    ] {
        let face = cone_group(&body)[0];
        let Surface::Cone { apex, axis, .. } = cone_of(&body, face) else {
            panic!("a frustum's wall is a cone");
        };
        let station: f64 = corners(&body, face)
            .iter()
            .map(|p| (*p - apex).dot(axis))
            .sum();
        let got = topo::face_nappe(&body, face, band()).expect("the wall has a nappe");
        assert_eq!(got, want, "{what}: station sum {station}");
        assert_eq!(
            station < 0.0,
            want == Nappe::Mirror,
            "{what}: the answer is the summed station's sign ({station})"
        );
    }
}

/// **The two doors mint the same cone.** The axial door builds, so its
/// minted wall can be read off the body it returns; the per-chart door
/// refuses downstream at the caps on this fixture (the row below), so
/// what is pinned here is that the surface the axial door actually
/// stored is bit-for-bit the mint of the SHARED expression both doors
/// now compute — `face_nappe(..).turn(d)` handed to `offset_surface`.
#[test]
fn both_doors_mint_the_turned_offset_on_both_nappes() {
    for (what, body) in [
        ("narrowing upward", mirror_frustum()),
        ("widening upward", opening_frustum()),
    ] {
        let face = cone_group(&body)[0];
        let old = cone_of(&body, face);
        let nappe = topo::face_nappe(&body, face, band()).expect("the wall has a nappe");
        let want = geom_brep::offset_surface(&old, nappe.turn(-T), band()).expect("the mint");

        let mut work = body.clone();
        let moves = chart_moves(&work, -T);
        topo::offset_charts_together(&mut work, &moves, band(), Tol::witness())
            .expect("the axial door hollows the frustum's charts inward");
        let got = cone_of(&work, cone_group(&work)[0]);
        let (
            Surface::Cone {
                apex: a,
                half_angle: ha,
                ..
            },
            Surface::Cone {
                apex: b,
                half_angle: hb,
                ..
            },
        ) = (&got, &want)
        else {
            panic!("a cone's offset is a cone");
        };
        assert_eq!(
            (a.x.to_bits(), a.y.to_bits(), a.z.to_bits()),
            (b.x.to_bits(), b.y.to_bits(), b.z.to_bits()),
            "{what}: the axial door's minted apex is the home's turn, bitwise"
        );
        assert_eq!(ha.to_bits(), hb.to_bits(), "{what}: half-angle carried");

        // And the cavity is smaller than the operand it was cut from.
        let v0 = topo::mass_properties(&body, Tol::witness()).unwrap().volume;
        let v1 = topo::mass_properties(&work, Tol::witness()).unwrap().volume;
        assert!(
            v1 < v0,
            "{what}: an inward offset of every chart shrinks the body ({v1} vs {v0})"
        );
    }
}

/// **The per-chart door's apex-window gate reads the same nappe.** An
/// inward request larger than the wall's own slant to its apex must
/// refuse `ApexWindow` on BOTH nappes: the window's near end, shifted
/// by `d·cot α`, has crossed the apex. Below that distance the gate
/// passes and the door refuses at its neighbouring caps instead
/// (`ReanchorOffCarrier`) — the refusal #1199 measured, unchanged.
///
/// The threshold is `|v_near|/cot α` = the slant from the apex to the
/// near rim, times `tan α`: 0.0322119… m for both frustums here. It is
/// the mirror nappe's row that is load-bearing: with the turn dropped,
/// the shifted window on that nappe runs AWAY from the apex for every
/// inward `d` and the gate can never fire.
#[test]
fn the_apex_window_gate_fires_on_both_nappes_at_the_same_reach() {
    let over = 0.04;
    let under = 0.03;
    for (what, body) in [
        ("narrowing upward", mirror_frustum()),
        ("widening upward", opening_frustum()),
    ] {
        let faces = cone_group(&body);
        for (d, expect_window) in [(-over, true), (-under, false), (over, false)] {
            let mut work = body.clone();
            let got = topo::replace_faces_offset(&mut work, &faces, d, band(), Tol::witness());
            match (&got, expect_window) {
                (
                    Err(ReplaceFaceError::ApexWindow {
                        v_min,
                        v_max,
                        shift,
                        ..
                    }),
                    true,
                ) => {
                    assert!(
                        (v_min + shift) * (v_max + shift) <= 0.0 || v_min * v_max > 0.0,
                        "{what} d={d}: the shifted window is what refused ({v_min}, {v_max}, {shift})"
                    );
                }
                (Err(ReplaceFaceError::ReanchorOffCarrier { .. }), false) => {}
                other => panic!(
                    "{what} d={d}: wanted {} , got {other:?}",
                    if expect_window {
                        "ApexWindow"
                    } else {
                        "ReanchorOffCarrier"
                    }
                ),
            }
        }
    }
}

/// **A face with no nappe refuses typed, at both doors, with one
/// variant.** No valid revolve builds a cone face that straddles its
/// own apex — the meridian would have to cross the axis — so the
/// fixture is built by re-attaching the wall's own cone with its apex
/// at the wall's mid-height, which makes the corner stations sum to
/// exactly zero. That is the operand the decide has to refuse rather
/// than guess a sign for.
#[test]
fn a_straddling_face_refuses_at_both_doors() {
    let mut body = mirror_frustum();
    let group = cone_group(&body);
    let face = group[0];
    let Surface::Cone {
        axis,
        half_angle,
        u_ref,
        ..
    } = cone_of(&body, face)
    else {
        panic!("a frustum's wall is a cone");
    };
    let straddling = Surface::Cone {
        apex: Point3::new(0.0, H / 2.0, 0.0),
        axis,
        half_angle,
        u_ref,
    };
    let key = body
        .set_face_surface(face, topo::FaceSurface::New(straddling))
        .expect("the wall takes a re-anchored cone");
    for &other in &group[1..] {
        body.set_face_surface(other, topo::FaceSurface::Shared(key))
            .expect("the wall's other band shares it");
    }
    let station: f64 = corners(&body, face)
        .iter()
        .map(|p| (*p - Point3::new(0.0, H / 2.0, 0.0)).dot(axis))
        .sum();
    assert_eq!(station, 0.0, "the fixture's corner stations cancel exactly");

    match topo::face_nappe(&body, face, band()) {
        Err(ReplaceFaceError::NappeStraddles {
            face: f,
            station: s,
        }) => {
            assert_eq!(f, face);
            assert_eq!(s, 0.0);
        }
        other => panic!("the home must refuse a straddling face: {other:?}"),
    }
    let mut work = body.clone();
    match topo::replace_faces_offset(&mut work, &group, -T, band(), Tol::witness()) {
        Err(ReplaceFaceError::NappeStraddles { face: f, .. }) => assert_eq!(f, face),
        other => panic!("the per-chart door must refuse a straddling face: {other:?}"),
    }
    let mut work = body.clone();
    let moves = chart_moves(&work, -T);
    match topo::offset_charts_together(&mut work, &moves, band(), Tol::witness()) {
        Err(ReplaceFaceError::NappeStraddles { face: f, .. }) => assert_eq!(f, face),
        other => panic!("the axial door must refuse a straddling face: {other:?}"),
    }
}

/// **The per-face read and the per-point read never disagree on a face
/// that HAS a nappe.** `ConeOffset::displacement` used to recover the
/// chart radial by `copysign` on the point's own axial station; it
/// takes the face's nappe now. On every corner of both frustums the two
/// agree bitwise — which is what makes the per-point read a redundant
/// second authority rather than a different answer.
#[test]
fn the_displacement_agrees_with_the_read_it_replaced() {
    for (what, body) in [
        ("narrowing upward", mirror_frustum()),
        ("widening upward", opening_frustum()),
    ] {
        let face = cone_group(&body)[0];
        let Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } = cone_of(&body, face)
        else {
            panic!("a frustum's wall is a cone");
        };
        let nappe = topo::face_nappe(&body, face, band()).expect("the wall has a nappe");
        for d in [-T, T] {
            let action = ConeOffset::new(apex, axis, half_angle, nappe.turn(d));
            for p in corners(&body, face) {
                let (sin_a, cos_a) = half_angle.sin_cos();
                let w: Vec3<f64> = p - apex;
                let h = w.dot(axis);
                let radial = w.reject_from(axis).normalize();
                let old = (radial * cos_a.copysign(h) - axis * sin_a) * nappe.turn(d);
                let got = action.displacement(nappe, p);
                assert_eq!(
                    (got.x.to_bits(), got.y.to_bits(), got.z.to_bits()),
                    (old.x.to_bits(), old.y.to_bits(), old.z.to_bits()),
                    "{what} d={d} at {p:?}: the nappe read moved, the number did not"
                );
            }
        }
    }
}
