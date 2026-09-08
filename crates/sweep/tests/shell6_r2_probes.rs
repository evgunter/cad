//! R2 review probes for SHELL-6 (PR #2178). Lane-private. Print-first;
//! assert where the answer is already known.
//!
//! These rows exist to FALSIFY, not to pass: several of them assert
//! that a shipped row's assertion is weaker than its prose claims.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::PI;

use geom::Surface;
use geom_brep::{ConeOffset, Nappe};
use geom_core::{Band, Point2, Point3, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, ReplaceFaceError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const T: f64 = 1.0 / 128.0;
const H: f64 = 8.0 / 64.0;
const R_WIDE: f64 = 4.0 / 64.0;
const R_NARROW: f64 = 2.0 / 64.0;

fn revolved(pts: &[(f64, f64)]) -> Body<f64> {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(
            pts.iter()
                .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
                .collect(),
        )],
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

fn frustum(r0: f64, r1: f64, h: f64) -> Body<f64> {
    revolved(&[(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)])
}

fn mirror_frustum() -> Body<f64> {
    frustum(R_WIDE, R_NARROW, H)
}

fn opening_frustum() -> Body<f64> {
    frustum(R_NARROW, R_WIDE, H)
}

/// `verbs_offd`'s fixture, re-derived here so the re-baselined row's
/// arithmetic can be checked without touching that file.
fn coned_tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.0), (0.8, 0.3), (0.4, 0.6)])
}

fn cone_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cone { .. })))
        .map(|(k, _)| k)
        .collect()
}

fn cone_of(body: &Body<f64>, face: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

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

// ---------------------------------------------------------------
// P1. `sf2b_r2_probes::r2_per_chart_door_on_a_mirror_nappe_cone`
//     claims to carry "the whole differential, in one number". Measure
//     the differential and the row's own tolerance.
// ---------------------------------------------------------------

/// **The gap row cannot see the turn it says it measures.** The shipped
/// row asserts `|gap − |d|·sin α| <= 1e-15` on all four (nappe × sign)
/// cases. The PR body reports the turn moved those gaps by ~1e-17
/// between the two signs on the mirror nappe. This row measures both
/// numbers and asserts the spread is far INSIDE the tolerance — i.e.
/// the assertion is invariant under `d ↦ −d` and under the nappe, so it
/// would pass unchanged with the turn deleted.
#[test]
fn r2p1_the_shipped_gap_row_is_blind_to_the_turn() {
    let tol = Tol::witness();
    let alpha = ((R_WIDE - R_NARROW) / H).atan();
    let mut gaps: Vec<(String, f64, f64)> = Vec::new();
    for (what, body) in [
        ("narrowing upward (mirror)", mirror_frustum()),
        ("widening upward (opening)", opening_frustum()),
    ] {
        let faces = cone_faces(&body);
        for signed in [-T, T] {
            let mut work = body.clone();
            match topo::replace_faces_offset(&mut work, &faces, signed, band(), tol) {
                Err(ReplaceFaceError::ReanchorOffCarrier { gap, .. }) => {
                    println!("[r2p1] {what} d={signed:+}: gap = {gap:.20}");
                    gaps.push((what.to_string(), signed, gap));
                }
                other => panic!("[r2p1] {what} d={signed}: unexpected {other:?}"),
            }
        }
    }
    let want = T * alpha.sin();
    println!("[r2p1] |d|·sin α = {want:.20}");
    let spread = gaps
        .iter()
        .map(|(_, _, g)| (g - want).abs())
        .fold(0.0f64, f64::max);
    println!("[r2p1] max |gap − |d|·sin α| over all four rows = {spread:.3e}");
    println!(
        "[r2p1] the shipped row's tolerance                = {:.3e}",
        1e-15
    );
    assert!(
        spread < 1e-15 / 10.0,
        "[r2p1] the four gaps sit {spread:.3e} from the closed form, so the shipped \
         row's 1e-15 tolerance cannot separate the nappes or the signs"
    );
    // And the two mirror-nappe signs really do differ — by an amount
    // the shipped tolerance swallows whole.
    let m: Vec<f64> = gaps
        .iter()
        .filter(|(w, _, _)| w.contains("mirror"))
        .map(|(_, _, g)| *g)
        .collect();
    println!(
        "[r2p1] mirror-nappe gaps: {:?}, difference {:.3e}",
        m,
        (m[0] - m[1]).abs()
    );
}

// ---------------------------------------------------------------
// P2. `shell6_nappe_home::the_apex_window_gate_fires_on_both_nappes…`
//     has an inner assertion whose second disjunct is true for every
//     non-straddling operand.
// ---------------------------------------------------------------

/// **The apex-window row's inner assertion is vacuous.** It asserts
/// `(v_min+shift)·(v_max+shift) <= 0 || v_min·v_max > 0`. Both frustums'
/// operand windows lie wholly on one nappe, so `v_min·v_max > 0` is
/// already true and the disjunction short-circuits before the shifted
/// window is looked at. This row measures the payload the shipped row
/// receives and shows the second disjunct carries it.
#[test]
fn r2p2_the_apex_window_rows_inner_assertion_short_circuits() {
    let tol = Tol::witness();
    for (what, body) in [
        ("narrowing upward (mirror)", mirror_frustum()),
        ("widening upward (opening)", opening_frustum()),
    ] {
        let faces = cone_faces(&body);
        let mut work = body.clone();
        match topo::replace_faces_offset(&mut work, &faces, -0.04, band(), tol) {
            Err(ReplaceFaceError::ApexWindow {
                v_min,
                v_max,
                shift,
                ..
            }) => {
                println!(
                    "[r2p2] {what}: v_min={v_min} v_max={v_max} shift={shift} \
                     v_min*v_max={} (v_min+shift)*(v_max+shift)={}",
                    v_min * v_max,
                    (v_min + shift) * (v_max + shift)
                );
                assert!(
                    v_min * v_max > 0.0,
                    "[r2p2] {what}: the operand window is single-nappe, so the shipped \
                     row's second disjunct is already true and its first is never read"
                );
            }
            other => panic!("[r2p2] {what}: wanted ApexWindow, got {other:?}"),
        }
    }
}

// ---------------------------------------------------------------
// P3. Is the per-chart door's cone build genuinely unreachable?
// ---------------------------------------------------------------

/// **Adversarial: sweep `d` down until the caps stop refusing.** The PR
/// reports §2.1 unreachable because a cone's offset moves its rim off
/// every unmoved neighbour by `d·sin α`. That gap is compared against
/// the run's ε, so the claim is really "unreachable for `|d| ≫ ε/sin α`".
/// This row sweeps `|d|` down through that scale on both nappes and
/// reports what the door actually does.
#[test]
fn r2p3_the_per_chart_cone_build_is_reachable_below_the_rim_tolerance() {
    let tol = Tol::witness();
    let alpha = ((R_WIDE - R_NARROW) / H).atan();
    println!(
        "[r2p3] eps = {:e}, eps/sin α = {:e}",
        tol.eps(),
        tol.eps() / alpha.sin()
    );
    let mut built: Vec<(String, f64)> = Vec::new();
    for (what, body) in [
        ("narrowing upward (mirror)", mirror_frustum()),
        ("widening upward (opening)", opening_frustum()),
    ] {
        let faces = cone_faces(&body);
        for mag in [1e-3, 1e-6, 1e-9, 1e-11, 1e-12, 1e-13, 1e-15] {
            for signed in [-mag, mag] {
                let mut work = body.clone();
                let got = topo::replace_faces_offset(&mut work, &faces, signed, band(), tol);
                let label = match &got {
                    Ok(()) => {
                        built.push((what.to_string(), signed));
                        "BUILT".to_string()
                    }
                    Err(ReplaceFaceError::ReanchorOffCarrier { gap, .. }) => {
                        format!("ReanchorOffCarrier gap={gap:e}")
                    }
                    Err(e) => format!("{e}"),
                };
                println!("[r2p3] {what} d={signed:+e}: {label}");
            }
        }
    }
    println!("[r2p3] builds: {built:?}");
}

// ---------------------------------------------------------------
// P4. Claim 3: the apex-window gate's equivalence to what it replaced.
// ---------------------------------------------------------------

/// **A face whose corner-station SUM is positive while some corners are
/// negative.** Re-anchor the wall's cone so its apex sits at `H/4`: the
/// four corners give stations `−H/4, −H/4, +3H/4, +3H/4`, summing
/// POSITIVE, so `face_nappe` answers `Opening` while the face's own
/// window straddles the apex. The gate must still refuse, and on the
/// window's variant, not the nappe's.
#[test]
fn r2p4_sum_positive_with_negative_corners_still_refuses_on_the_window() {
    let tol = Tol::witness();
    let mut body = mirror_frustum();
    let group = cone_faces(&body);
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
    let new_apex = Point3::new(0.0, H / 4.0, 0.0);
    let reanchored = Surface::Cone {
        apex: new_apex,
        axis,
        half_angle,
        u_ref,
    };
    let key = body
        .set_face_surface(face, topo::FaceSurface::New(reanchored))
        .expect("the wall takes a re-anchored cone");
    for &other in &group[1..] {
        body.set_face_surface(other, topo::FaceSurface::Shared(key))
            .expect("the wall's other band shares it");
    }
    let stations: Vec<f64> = corners(&body, face)
        .iter()
        .map(|p| (*p - new_apex).dot(axis))
        .collect();
    let sum: f64 = stations.iter().sum();
    println!("[r2p4] corner stations {stations:?} sum {sum}");
    assert!(sum > 0.0, "[r2p4] the fixture's SUM must be positive");
    assert!(
        stations.iter().any(|s| *s < 0.0),
        "[r2p4] and some corners must be on the other nappe"
    );
    let nappe = topo::face_nappe(&body, face, band()).expect("the sum decides");
    println!("[r2p4] face_nappe says {nappe:?}");
    assert_eq!(nappe, Nappe::Opening);

    let mut work = body.clone();
    let got = topo::replace_faces_offset(&mut work, &group, -T, band(), tol);
    println!("[r2p4] per-chart door: {got:?}");
    assert!(
        matches!(got, Err(ReplaceFaceError::ApexWindow { .. })),
        "[r2p4] a face whose window straddles its apex must refuse on the WINDOW's \
         variant, whatever its corner sum says: got {got:?}"
    );

    // The axial door has no window gate at all: it takes the sum's
    // answer and mints. Report what it does with a straddling face
    // whose sum is definite.
    let mut work = body.clone();
    let moves = chart_moves(&work, -T);
    let got = topo::offset_charts_together(&mut work, &moves, band(), tol);
    println!("[r2p4] axial door on the same straddling face: {got:?}");
}

// ---------------------------------------------------------------
// P5. Claim 4: re-derive the re-baselined row's arithmetic.
// ---------------------------------------------------------------

/// **The re-baselined `verbs_offd` row, re-derived from the fixture.**
/// `coned_tube`'s outer wall: apex at `y = 0.9`, `tan α = 4/3`,
/// `cos α = 0.6`, window `v ∈ [−1.0, −0.5]` (mirror nappe). The door
/// turns `d`, so `shift = −d·cot α` and the near end's realized margin
/// is `0.5 + 0.75·d`. It goes negative at `d < −2/3`, so `d = −1.5`
/// crosses the apex and `d = +1.5` runs away from it. Both asserted
/// against the kernel, and the geometry named: on this wall the
/// face-outward direction points AWAY from the apex, so the crossing
/// offset is the INWARD one — which is the question the old `d = +1.5`
/// row meant to ask and could not.
#[test]
fn r2p5_the_rebaselined_row_asks_the_question_the_old_one_meant() {
    let tol = Tol::witness();
    let body = coned_tube();
    let face = cone_faces(&body)[0];
    let Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = cone_of(&body, face)
    else {
        panic!("the tube's outer wall is a cone");
    };
    let cos_a = half_angle.cos();
    let stations: Vec<f64> = corners(&body, face)
        .iter()
        .map(|p| (*p - apex).dot(axis))
        .collect();
    let vs: Vec<f64> = stations.iter().map(|h| h / cos_a).collect();
    println!(
        "[r2p5] apex={apex:?} axis={axis:?} cot α={} cos α={cos_a} v at corners {vs:?}",
        1.0 / half_angle.tan()
    );
    let nappe = topo::face_nappe(&body, face, band()).expect("the wall has a nappe");
    assert_eq!(
        nappe,
        Nappe::Mirror,
        "[r2p5] the outer wall is below its apex"
    );

    let cot = 1.0 / half_angle.tan();
    for d in [-1.5, 1.5] {
        let shift = nappe.turn(d) * cot;
        let v_near = vs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let realized = (v_near + shift) * -1.0;
        let mut work = body.clone();
        let got = topo::replace_face_offset(&mut work, face, d, band(), tol);
        println!("[r2p5] d={d:+}: shift={shift} v_near={v_near} realized={realized} -> {got:?}");
        if d < 0.0 {
            assert!(
                realized < 0.0,
                "[r2p5] d=−1.5's near end has crossed the apex"
            );
            assert!(
                matches!(got, Err(ReplaceFaceError::ApexWindow { .. })),
                "[r2p5] and the door must say so: {got:?}"
            );
        } else {
            assert!(realized > 0.0, "[r2p5] d=+1.5 runs away from the apex");
            assert!(
                matches!(got, Err(ReplaceFaceError::ReanchorOffCarrier { .. })),
                "[r2p5] so the old row's d now meets the rim gate instead: {got:?}"
            );
        }
    }
}

// ---------------------------------------------------------------
// P6. Claim 1: which face's nappe reaches `displacement`.
// ---------------------------------------------------------------

/// **The corner shared with an unmoved neighbour still gets the CONE's
/// nappe.** `transport_curve` is handed the group's nappe and the turned
/// `d`; the point it displaces lies on the cone. This row checks the
/// composed value against the direction a consumer would name — the
/// face's OWN outward chart normal — on both nappes, which is the
/// property the door's `d` is documented to have.
#[test]
fn r2p6_the_displacement_moves_the_face_along_its_own_outward_normal() {
    for (what, body, want) in [
        ("narrowing upward", mirror_frustum(), Nappe::Mirror),
        ("widening upward", opening_frustum(), Nappe::Opening),
    ] {
        let face = cone_faces(&body)[0];
        let Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } = cone_of(&body, face)
        else {
            panic!("a frustum's wall is a cone");
        };
        let nappe = topo::face_nappe(&body, face, band()).expect("nappe");
        assert_eq!(nappe, want);
        let (sin_a, cos_a) = half_angle.sin_cos();
        // The door's own composition: turn `d`, then displace.
        let d_face = T;
        let action = ConeOffset::new(apex, axis, half_angle, nappe.turn(d_face));
        for p in corners(&body, face) {
            let radial_geom = (p - apex).reject_from(axis).normalize();
            // The face's OWN outward chart normal at `p`, written
            // without naming `Nappe`: the geometric radial times cos α,
            // less the axial term signed by the point's own side of the
            // apex. On the opening nappe that is `n₊`; on the mirror one
            // it is `−n₊`, which is the whole content of the turn.
            let h = (p - apex).dot(axis);
            let s = if h < 0.0 { -1.0 } else { 1.0 };
            let n_face = radial_geom * cos_a - axis * (sin_a * s);
            let got = action.displacement(nappe, p);
            let want_delta = n_face * d_face;
            println!("[r2p6] {what} at {p:?}: got {got:?} want {want_delta:?}");
            assert!(
                (got - want_delta).norm() <= 1e-16,
                "[r2p6] {what}: the displacement must be `d` along the FACE's own \
                 outward chart normal at {p:?}: {got:?} vs {want_delta:?}"
            );
        }
    }
}

// ---------------------------------------------------------------
// P7. The end-to-end exercise, from a consumer's seat.
// ---------------------------------------------------------------

fn frustum_volume(r0: f64, r1: f64, h: f64) -> f64 {
    PI * h * (r0 * r0 + r0 * r1 + r1 * r1) / 3.0
}

fn frustum_wall_closed_form(r0: f64, r1: f64, h: f64) -> f64 {
    let alpha = ((r0 - r1).abs() / h).atan();
    let at = |y: f64| r0 + (r1 - r0) * y / h;
    let c0 = at(T) - T / alpha.cos();
    let c1 = at(h - T) - T / alpha.cos();
    frustum_volume(r0, r1, h) - frustum_volume(c0, c1, h - 2.0 * T)
}

/// **A user hollows a frustum on each nappe, then tries the per-chart
/// door on the same faces.** The public seat only: `topo::shell`,
/// `topo::mass_properties`, `topo::validate_geometric`,
/// `topo::replace_faces_offset`. Every refusal is a hard failure here —
/// this row reports what a user actually gets, and cannot pass by
/// printing.
#[test]
fn r2p7_end_to_end_a_user_hollows_both_nappes_then_tries_the_per_chart_door() {
    let tol = Tol::witness();
    for (what, r0, r1) in [
        ("frustum BELOW its apex (mirror)", R_WIDE, R_NARROW),
        ("frustum ABOVE its apex (opening)", R_NARROW, R_WIDE),
    ] {
        let body = frustum(r0, r1, H);
        let v_solid = topo::mass_properties(&body, tol).expect("props").volume;
        let closed = frustum_volume(r0, r1, H);
        println!("[r2p7] {what}: solid {v_solid}, closed form {closed}");
        assert!(
            (v_solid - closed).abs() <= 1e-14,
            "[r2p7] {what}: the operand itself"
        );

        let hollow = match topo::shell(&body, T, tol) {
            Ok(topo::Shelled { body: h, .. }) => h,
            Err(e) => panic!("[r2p7] {what}: `shell` REFUSED — a user gets nothing: {e}"),
        };
        assert_eq!(
            topo::validate_geometric(&hollow, tol),
            Ok(()),
            "[r2p7] {what}: the hollowed body must validate"
        );
        let v_wall = topo::mass_properties(&hollow, tol).expect("props").volume;
        let want = frustum_wall_closed_form(r0, r1, H);
        println!(
            "[r2p7] {what}: wall {v_wall} want {want} delta {:e}, cavity {}",
            v_wall - want,
            v_solid - v_wall
        );
        assert!(
            (v_wall - want).abs() <= 1e-12,
            "[r2p7] {what}: the wall's closed form is {want}, got {v_wall}"
        );
        assert!(
            v_wall < v_solid,
            "[r2p7] {what}: a wall cannot exceed the solid it was cut from"
        );

        // The same faces, through the per-chart door a user would reach
        // for to move ONE chart.
        let faces = cone_faces(&body);
        let mut work = body.clone();
        let got = topo::replace_faces_offset(&mut work, &faces, -T, band(), tol);
        println!("[r2p7] {what}: per-chart door on the same faces -> {got:?}");
    }
}
