//! PR 3 adversarial review — target 6 (join mirror sites, HARDER ring
//! fixture): a two-hole box whose divided top/bottom faces carry TWO
//! rings each, so `laringmv` must execute BOTH verdicts (keep one
//! ring In, move one ring Out) in a single re-homing sweep; plus a
//! split THROUGH one hole (the ring is divided, not re-homed —
//! two section polygons from one plane), and the genus-1 fixture on
//! the interval lane (which the acceptance interval test omits).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Tol;
use geom_core::{Point3, Real, Vec3};
use topo::{
    Body, FaceSurface, LoopBoundary, MefSite, MevSite, SplitPart, SplitPlane, mass_properties,
    split, validate_closed,
};

fn body_of<T: Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

/// A geometric box `[0,w]×[0,2]×[0,2]` with square 1×1 through-holes
/// (along z) centered at each `cx` — the acceptance fixture's
/// builder, generalized to N holes and generic over the scalar lane.
fn holed_box<T: geom_core::Decide>(w: f64, holes: &[f64]) -> Body<T> {
    let pt = |x: f64, y: f64, z: f64| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    let mut body = Body::<T>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let strut = |body: &mut Body<T>, at, x, y, z| {
        body.mev_line(
            MevSite::Fan { he1: at, he2: at },
            pt(x, y, z),
            Tol::witness(),
        )
        .unwrap()
    };
    let mef = |body: &mut Body<T>, he1, he2| {
        body.mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
            .unwrap()
    };
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(w, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let e_bc = strut(&mut body, e_ab.he_minus, w, 2.0, 0.0);
    let e_cd = strut(&mut body, e_bc.he_minus, 0.0, 2.0, 0.0);
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = mef(&mut body, he_dc, e_ab.he_plus);
    let e_aa = strut(&mut body, e_ab.he_plus, 0.0, 0.0, 2.0);
    let e_bb = strut(&mut body, e_bc.he_plus, w, 0.0, 2.0);
    let e_cc = strut(&mut body, e_cd.he_plus, w, 2.0, 2.0);
    let e_dd = strut(&mut body, f_bottom.he_plus, 0.0, 2.0, 2.0);
    let f_front = mef(&mut body, e_aa.he_minus, e_bb.he_minus);
    let _f_right = mef(&mut body, e_bb.he_minus, e_cc.he_minus);
    let _f_back = mef(&mut body, e_cc.he_minus, e_dd.he_minus);
    let _f_left = mef(&mut body, e_dd.he_minus, f_front.he_plus);
    for &cx in holes {
        let (x0, x1, y0, y1) = (cx - 0.5, cx + 0.5, 0.5, 1.5);
        let hole = strut(&mut body, f_front.he_plus, x0, y0, 2.0);
        let kill = body.kemr(hole.he_plus, hole.he_minus).unwrap();
        let s_pq = body
            .mev_line(
                MevSite::Lone { r#loop: kill.ring },
                pt(x1, y0, 2.0),
                Tol::witness(),
            )
            .unwrap();
        let s_qr = strut(&mut body, s_pq.he_minus, x1, y1, 2.0);
        let s_rs = strut(&mut body, s_qr.he_minus, x0, y1, 2.0);
        let mef_top = mef(&mut body, s_pq.he_plus, s_rs.he_minus);
        let e_pp = strut(&mut body, s_pq.he_plus, x0, y0, 0.0);
        let e_qq = strut(&mut body, s_qr.he_plus, x1, y0, 0.0);
        let e_rr = strut(&mut body, s_rs.he_plus, x1, y1, 0.0);
        let e_ss = strut(&mut body, mef_top.he_minus, x0, y1, 0.0);
        let w_front = mef(&mut body, e_pp.he_minus, e_qq.he_minus);
        let _w_right = mef(&mut body, e_qq.he_minus, e_rr.he_minus);
        let _w_back = mef(&mut body, e_rr.he_minus, e_ss.he_minus);
        let _w_left = mef(&mut body, e_ss.he_minus, w_front.he_plus);
        body.kfmrh(f_bottom.face, mef_top.face).unwrap();
    }
    // Plating pass: outward Newell plane per face outer loop.
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    for f in faces {
        let outer = body.get_face(f).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("outer loops are cycles");
        };
        let pts: Vec<Point3<T>> = body
            .loop_cycle(first)
            .unwrap()
            .iter()
            .map(|&he| {
                let v = body.get_half_edge(he).unwrap().start;
                *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
            })
            .collect();
        let plane = geom_brep::newell_plane(&pts, band).unwrap();
        body.set_face_surface(f, FaceSurface::New(plane)).unwrap();
    }
    body
}

fn plane_x<T: geom_core::Decide>(c: f64) -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(c), T::from_f64(0.0), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(1.0), T::from_f64(0.0), T::from_f64(0.0)),
    }
}

/// The multi-ring `laringmv` sweep: two holes, split between them —
/// the divided top/bottom faces each hold two rings at re-homing
/// time, one In (stays) and one Out (moves). Both verdicts execute
/// in one sweep; each side ends up an independent genus-1 body.
#[test]
fn two_hole_box_split_between_rehomes_both_ways() {
    let body = holed_box::<f64>(6.0, &[1.0, 5.0]);
    assert_eq!(validate_closed(&body), Ok(()));
    let rings = |b: &Body<f64>| b.faces().map(|(_, f)| f.rings.len()).sum::<usize>();
    assert_eq!(rings(&body), 4, "top and bottom carry two rings each");
    let r = split(&body, &plane_x(3.0), Tol::witness()).unwrap();
    let (above, below) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // One hole each: each side keeps exactly ONE ring pair (top +
    // bottom of its hole = 2 ring loops per side).
    assert_eq!(rings(below), 2, "below keeps its own hole's rings");
    assert_eq!(rings(above), 2, "above got the moved rings");
    assert_eq!(below.shells().count(), 1);
    assert_eq!(above.shells().count(), 1);
    // Volumes: total 6·2·2 − 2·(1·1·2) = 20, split 10/10.
    let (va, vb) = (
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
    );
    assert!((va - 10.0).abs() < 1e-12, "above {va}");
    assert!((vb - 10.0).abs() < 1e-12, "below {vb}");
}

/// Split THROUGH a hole (x = 1 bisects the first hole): the ring is
/// cut, not re-homed — the plane meets the body in TWO disjoint
/// section polygons (above and below the hole's channel), and each
/// side comes out genus-0-with-a-channel plus the other side's hole.
#[test]
fn split_through_hole_two_section_polygons() {
    let body = holed_box::<f64>(6.0, &[1.0, 5.0]);
    let s = topo::plane_section(&body, &plane_x::<f64>(1.0), Tol::witness()).unwrap();
    assert_eq!(s.polygons.len(), 2, "channel splits the section in two");
    let r = split(&body, &plane_x(1.0), Tol::witness()).unwrap();
    let (above, below) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // Below: [0,1] slab with an open channel notch — no rings left.
    // Above: [1,6] slab with the intact second hole (2 ring loops).
    let rings = |b: &Body<f64>| b.faces().map(|(_, f)| f.rings.len()).sum::<usize>();
    assert_eq!(rings(below), 0);
    assert_eq!(rings(above), 2);
    let (va, vb) = (
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
    );
    // Total 20; below = 1·2·2 − (0.5·1·2 channel half) = 3.
    assert!((vb - 3.0).abs() < 1e-12, "below {vb}");
    assert!((va - 17.0).abs() < 1e-12, "above {va}");
}

/// The genus-1 ring re-homing on the INTERVAL lane — the acceptance
/// interval test never exercises `laringmv`/`point_in_loop` there.
/// Either it works like f64 (lane agreement) or it must refuse typed
/// (the documented interval posture) — a silent wrong answer or a
/// panic is the only failure. Executed to find out which.
#[cfg(feature = "interval")]
#[test]
fn interval_lane_ring_rehoming() {
    use geom_core::Interval;
    let body = holed_box::<Interval>(6.0, &[1.0, 5.0]);
    assert_eq!(validate_closed(&body), Ok(()));
    match split(&body, &plane_x::<Interval>(3.0)) {
        Ok(r) => {
            let (above, below) = (body_of(&r.above), body_of(&r.below));
            assert_eq!(validate_closed(above), Ok(()));
            assert_eq!(validate_closed(below), Ok(()));
            let rings = |b: &Body<Interval>| b.faces().map(|(_, f)| f.rings.len()).sum::<usize>();
            assert_eq!((rings(below), rings(above)), (2, 2), "same as f64 lane");
        }
        Err(e) => {
            eprintln!("interval ring re-homing refused typed: {e}");
        }
    }
}
