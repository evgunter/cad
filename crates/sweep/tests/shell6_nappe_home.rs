//! **The cone nappe has one home.** `geom_brep::offset_surface` slides a
//! cone's apex along the OPENING nappe's normal field, so a
//! mirror-nappe face's material moves `−d` along its own chart normal.
//! Which nappe a FACE lies on is a fact only the face has, and
//! `topo::face_nappe` / `topo::group_nappe` are where the offset lane
//! decides it: both doors, the per-chart door's apex-window gate and
//! `geom_brep::ConeOffset::displacement` turn by that one answer.
//!
//! These rows pin the answer, the enforcement of its premise, the two
//! doors' agreement over it at an operand where BOTH build, the typed
//! refusals where a face or a chart has no nappe, and the composition
//! the displacement is documented to have.
//!
//! Rows marked (R1) and (R2) are the review lanes', adopted here from
//! `shell6_r1_probes.rs` / `shell6_r2_probes.rs` with their fixtures;
//! the probe files keep the rows that make a claim these do not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{ConeOffset, Nappe};
use geom_core::Tol;
use topo::{FaceKey, ReplaceFaceError};

use crate::common::cone_nappe::{
    H, R_NARROW, R_WIDE, T, band, chart_moves, cone_faces, corners, mirror_frustum,
    opening_frustum, reanchor_cone, revolved, stations, surface_of,
};

fn cone_of(body: &topo::Body<f64>, face: FaceKey) -> Surface<f64> {
    surface_of(body, face)
}

/// **The home answers both nappes, from the face's own corners.**
#[test]
fn face_nappe_reads_the_frustums_own_wall() {
    for (what, body, want) in [
        ("narrowing upward", mirror_frustum(), Nappe::Mirror),
        ("widening upward", opening_frustum(), Nappe::Opening),
    ] {
        let face = cone_faces(&body)[0];
        let st = stations(&body, face);
        let (lo, hi) = (
            st.iter().cloned().fold(f64::INFINITY, f64::min),
            st.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        );
        let got = topo::face_nappe(&body, face, band()).expect("the wall has a nappe");
        assert_eq!(got, want, "{what}: stations [{lo}, {hi}]");
        // The answer is the side BOTH extremes stand on, not a sum's.
        assert_eq!(
            (lo < 0.0, hi < 0.0),
            (want == Nappe::Mirror, want == Nappe::Mirror),
            "{what}: both extremes must stand on the answered side ([{lo}, {hi}])"
        );
        // And the whole chart agrees, which is what the doors ask for.
        assert_eq!(
            topo::group_nappe(&body, &cone_faces(&body), band()).expect("the chart agrees"),
            want,
            "{what}: the chart's bands share their wall's nappe"
        );
    }
}

/// **Both doors mint the turned offset, each at an operand where it
/// builds** (R2's `r2p8` for the per-chart half).
///
/// The doors are pinned against ONE expression —
/// `offset_surface(cone, group_nappe(..).turn(d))` — bit for bit, on
/// both nappes. They are pinned at different `d` because their reaches
/// differ and neither reach is about the nappe: the per-chart door's
/// `ReanchorOffCarrier` compares the rim's displacement `|d|·sin α`
/// against ε, so it builds only BELOW `ε/sin α`; the axial door meters
/// the request itself (`offset_axial_request`) and escalates on a `d`
/// that small. So the per-chart door is read at half that threshold —
/// stated in the RUN's own ε, because a fixed number is a different
/// question at each of the three eps rows — and the axial one at the
/// wall thickness.
///
/// The turn stays observable there: the two nappes' apexes stand
/// `2d/sin α = ε/sin²α ≈ 17ε` apart, so a door that took the wrong one
/// would land a visibly different cone. The last assertion of each pass
/// measures that separation rather than assuming it.
#[test]
fn both_doors_mint_the_turned_offset_on_both_nappes() {
    let tol = Tol::witness();
    let alpha = ((R_WIDE - R_NARROW) / H).atan();
    // Half the rim gate's own threshold: the moved rim's gap is then
    // `ε/2`, inside the band's zero, so the caps hold and the door
    // builds — at every eps row rather than at one.
    let small = 0.5 * tol.eps() / alpha.sin();
    for (what, body) in [
        ("narrowing upward (mirror)", mirror_frustum()),
        ("widening upward (opening)", opening_frustum()),
    ] {
        let faces = cone_faces(&body);
        let old = cone_of(&body, faces[0]);
        let nappe = topo::group_nappe(&body, &faces, band()).expect("the chart has a nappe");
        let v0 = topo::mass_properties(&body, tol).expect("props").volume;
        for (door, d) in [("per-chart", small), ("axial", T)] {
            let want = geom_brep::offset_surface(&old, nappe.turn(-d), band()).expect("the mint");
            let Surface::Cone {
                apex: b,
                half_angle: hb,
                ..
            } = &want
            else {
                panic!("a cone's offset is a cone");
            };
            let mut work = body.clone();
            let moves = chart_moves(&work, -d);
            let outcome = if door == "per-chart" {
                topo::replace_faces_offset(&mut work, &faces, -d, band(), tol)
            } else {
                topo::offset_charts_together(&mut work, &moves, band(), tol)
            };
            outcome.unwrap_or_else(|e| panic!("{what}: the {door} door refused {e}"));
            let Surface::Cone {
                apex: a,
                half_angle: ha,
                ..
            } = cone_of(&work, cone_faces(&work)[0])
            else {
                panic!("a cone's offset is a cone");
            };
            assert_eq!(
                (a.x.to_bits(), a.y.to_bits(), a.z.to_bits()),
                (b.x.to_bits(), b.y.to_bits(), b.z.to_bits()),
                "{what}: the {door} door's minted apex must be the home's turn, bitwise"
            );
            assert_eq!(
                ha.to_bits(),
                hb.to_bits(),
                "{what}: {door}: half-angle carried"
            );
            assert_eq!(
                topo::validate_geometric(&work, tol),
                Ok(()),
                "{what}: the {door} door's own output must validate"
            );
            let v1 = topo::mass_properties(&work, tol).expect("props").volume;
            assert!(
                v1 < v0,
                "{what}: the {door} door's inward request must shrink the solid ({v1} vs {v0})"
            );

            // The turn is observable at this operand: the wrong nappe
            // lands the apex further than ε away, so the agreement
            // above is a claim about the nappe and not about a
            // difference too small to see.
            let wrong = geom_brep::offset_surface(&old, nappe.turn(d), band()).expect("the mint");
            let Surface::Cone { apex: w, .. } = wrong else {
                panic!("a cone's offset is a cone");
            };
            let separation = (*b - w).norm();
            assert!(
                separation > tol.eps(),
                "{what}: the {door} door's turn must be observable here \
                 ({separation:e} vs eps {:e})",
                tol.eps()
            );
        }
    }
}

/// **What the per-chart door does as `|d|` grows** (R2's `r2p3`, the
/// reachability sweep, asserted rather than reported).
///
/// Below `ε/sin α` the rims still land on their carriers and the door
/// builds; above it the caps refuse `ReanchorOffCarrier` at the gap
/// `|d|·sin α`. Both nappes, both signs, one threshold — so the door's
/// reachability is a statement about ε and the fixture, never about the
/// nappe.
#[test]
fn the_per_chart_doors_reach_is_a_threshold_in_the_rim_tolerance() {
    let tol = Tol::witness();
    let alpha = ((R_WIDE - R_NARROW) / H).atan();
    let threshold = tol.eps() / alpha.sin();
    for (what, body) in [
        ("narrowing upward (mirror)", mirror_frustum()),
        ("widening upward (opening)", opening_frustum()),
    ] {
        let faces = cone_faces(&body);
        // Stated in the threshold's own units, so the row asks the same
        // question at every eps row: one magnitude whose gap lands
        // under the band's zero, one whose gap clears its escalate end.
        for mag in [0.5 * threshold, 1e3 * threshold] {
            for signed in [-mag, mag] {
                let mut work = body.clone();
                let got = topo::replace_faces_offset(&mut work, &faces, signed, band(), tol);
                if mag > threshold {
                    let Err(ReplaceFaceError::ReanchorOffCarrier { gap, .. }) = got else {
                        panic!("{what} d={signed:e}: wanted the rim refusal, got {got:?}");
                    };
                    assert!(
                        (gap - mag * alpha.sin()).abs() <= 1e-15 * gap.max(1.0),
                        "{what} d={signed:e}: the gap is |d|·sin α, got {gap:e}"
                    );
                } else {
                    got.unwrap_or_else(|e| {
                        panic!("{what} d={signed:e}: below the rim tolerance the door builds: {e}")
                    });
                }
            }
        }
    }
}

/// **The per-chart door's apex-window gate reads the same nappe.** An
/// inward request larger than the wall's own slant to its apex must
/// refuse `ApexWindow` on BOTH nappes: the window's near end, shifted
/// by `d·cot α`, has crossed the apex. Below that distance the gate
/// passes and the door refuses at its neighbouring caps instead.
///
/// The threshold is the slant from the apex to the near rim times
/// `tan α`: `0.0322119…` m for both frustums here. It is the mirror
/// nappe's row that is load-bearing: with the turn dropped, the shifted
/// window on that nappe runs AWAY from the apex for every inward `d`
/// and the gate can never fire.
#[test]
fn the_apex_window_gate_fires_on_both_nappes_at_the_same_reach() {
    let over = 0.04;
    let under = 0.03;
    for (what, body) in [
        ("narrowing upward", mirror_frustum()),
        ("widening upward", opening_frustum()),
    ] {
        let faces = cone_faces(&body);
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
                    // The NEAR end of the window — the one the gate
                    // reads on this face's nappe — has crossed the apex
                    // under the shift. Stated as the gate states it, so
                    // no disjunct can be satisfied by the other nappe's
                    // arithmetic.
                    let near = if *v_min > 0.0 { *v_min } else { *v_max };
                    let sense = if *v_min > 0.0 { 1.0 } else { -1.0 };
                    assert!(
                        (near + shift) * sense <= 0.0,
                        "{what} d={d}: the shifted near end must reach the apex \
                         (v [{v_min}, {v_max}], shift {shift})"
                    );
                }
                (Err(ReplaceFaceError::ReanchorOffCarrier { .. }), false) => {}
                other => panic!(
                    "{what} d={d}: wanted {}, got {other:?}",
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

/// **A face with no nappe refuses typed, at both doors, on one
/// variant** — and the premise is ENFORCED rather than assumed (R1's
/// `r1_sum_positive_with_negative_corners`, R2's `r2p4`).
///
/// Two operands, neither of which a valid revolve builds — the meridian
/// would have to cross the axis — so both re-attach the wall's own cone
/// with a moved apex:
///
/// - apex at the wall's mid-height: the corner stations cancel, so even
///   a SUM would refuse;
/// - apex at a quarter height: the stations are `−H/4, −H/4, +3H/4,
///   +3H/4`, whose sum is POSITIVE. A sum answers `Opening` for it,
///   which is the reading this unit replaced: the face's own corners
///   stand on both nappes and it has no nappe at all.
#[test]
fn a_face_whose_corners_reach_its_apex_refuses_at_both_doors() {
    for (what, apex_y) in [
        ("stations that cancel", H / 2.0),
        ("stations whose SUM is positive", H / 4.0),
    ] {
        let mut body = mirror_frustum();
        let group = cone_faces(&body);
        reanchor_cone(&mut body, &group, apex_y);
        let st = stations(&body, group[0]);
        let sum: f64 = st.iter().sum();
        assert!(
            st.iter().any(|s| *s < 0.0) && st.iter().any(|s| *s > 0.0),
            "{what}: the fixture's corners must stand on both nappes ({st:?})"
        );
        if apex_y < H / 2.0 {
            assert!(sum > 0.0, "{what}: and its SUM must be positive ({sum})");
        }

        let Err(ReplaceFaceError::NappeStraddles {
            face: f,
            station_min,
            station_max,
            ..
        }) = topo::face_nappe(&body, group[0], band())
        else {
            panic!("{what}: the home must refuse a face with no nappe");
        };
        assert_eq!(f, group[0]);
        assert!(
            station_min < 0.0 && station_max > 0.0,
            "{what}: the payload echoes the two extremes ({station_min}, {station_max})"
        );

        for door in ["per-chart", "axial"] {
            let mut work = body.clone();
            let moves = chart_moves(&work, -T);
            let got = if door == "per-chart" {
                topo::replace_faces_offset(&mut work, &group, -T, band(), Tol::witness())
            } else {
                topo::offset_charts_together(&mut work, &moves, band(), Tol::witness())
            };
            assert!(
                matches!(got, Err(ReplaceFaceError::NappeStraddles { .. })),
                "{what}: the {door} door must refuse on the nappe's variant, got {got:?}"
            );
        }
    }
}

/// **A CHART whose faces do not share a nappe refuses too** (R1's
/// `r1_a_chart_on_both_nappes`). A bi-cone's four bands re-anchored to
/// one cone at the kink leaves two `Mirror` and two `Opening` faces
/// over ONE surface key. One offset distance cannot be turned for both,
/// and neither door may pick a member's answer to stand for the rest —
/// so `group_nappe` refuses, in either group order, on both doors.
#[test]
fn a_chart_whose_faces_disagree_refuses_at_both_doors() {
    // Two cone bands with a CYLINDER between them, so one apex can sit
    // in the gap and leave both bands strictly clear of it — a bi-cone
    // whose two charts meet at a kink cannot: re-anchoring to the kink
    // puts a corner AT the apex, which the enforced premise refuses one
    // step earlier (the row above).
    let mut body = revolved(&[
        (0.0, 0.0),
        (R_NARROW, 0.0),
        (R_WIDE, H),
        (R_WIDE, 2.0 * H),
        (R_NARROW, 3.0 * H),
        (0.0, 3.0 * H),
    ]);
    let group = cone_faces(&body);
    assert_eq!(group.len(), 4, "two cone charts, two bands each");
    reanchor_cone(&mut body, &group, 1.5 * H);
    let (mut lower, mut upper) = (Vec::new(), Vec::new());
    for &f in &group {
        match topo::face_nappe(&body, f, band()).expect("each band has a nappe") {
            Nappe::Mirror => lower.push(f),
            Nappe::Opening => upper.push(f),
        }
    }
    assert_eq!((lower.len(), upper.len()), (2, 2));

    for (order, faces) in [
        ("mirror first", [lower.clone(), upper.clone()].concat()),
        ("opening first", [upper.clone(), lower.clone()].concat()),
    ] {
        for d in [-T, T] {
            let mut work = body.clone();
            let got = topo::replace_faces_offset(&mut work, &faces, d, band(), Tol::witness());
            let Err(ReplaceFaceError::NappeStraddles { face, what, .. }) = got else {
                panic!("{order} d={d}: a chart on both nappes must refuse, got {got:?}");
            };
            assert!(
                faces.contains(&face) && what.contains("chart"),
                "{order} d={d}: the refusal names the disagreeing member and the chart \
                 reading ({face:?}, {what})"
            );
        }
    }
    for d in [-T, T] {
        let mut work = body.clone();
        let moves = chart_moves(&work, d);
        let got = topo::offset_charts_together(&mut work, &moves, band(), Tol::witness());
        assert!(
            matches!(got, Err(ReplaceFaceError::NappeStraddles { .. })),
            "axial d={d}: the axial door had no group gate at all before this; got {got:?}"
        );
    }
}

/// **The displacement moves the face along its OWN outward normal**
/// (R2's `r2p6`). The door composes two things — turn `d` by the
/// chart's nappe, then displace with the turned action — and the
/// property that composition is documented to have is that the result
/// is `d` along the face's own outward chart normal. This row states
/// that direction WITHOUT naming `Nappe`, from the point's own side of
/// the apex, so it cannot re-derive the turn it is checking.
#[test]
fn the_displacement_moves_the_face_along_its_own_outward_normal() {
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
        for d_face in [-T, T] {
            let action = ConeOffset::new(apex, axis, half_angle, nappe.turn(d_face));
            for p in corners(&body, face) {
                let w = p - apex;
                let s = if w.dot(axis) < 0.0 { -1.0 } else { 1.0 };
                let n_face = w.reject_from(axis).normalize() * cos_a - axis * (sin_a * s);
                let got = action.displacement(nappe, p);
                let want_delta = n_face * d_face;
                assert!(
                    (got - want_delta).norm() <= 1e-16,
                    "{what} d={d_face}: the displacement must be `d` along the FACE's own \
                     outward chart normal at {p:?}: {got:?} vs {want_delta:?}"
                );
            }
        }
    }
}
