//! **A plane split through a holed body hands back halves that pass
//! the at-rest gate, with each holed section ONE face whose rings are
//! the holes' sections.** Every fixture is built through the public
//! doors (`topo::subtract`, `topo::union`, the sweep fixtures) and cut
//! by `split`.
//!
//! Two layers of the split meet here. The finish nests each section
//! polygon that is a hole into the section face around it, so a bored
//! body's section is an annulus rather than a face over the whole
//! outline plus a coplanar disc cancelling it. The join pairs a planar
//! face's crossings along that face's own line, so a ringed cap the
//! plane crosses is chorded beside its hole, not across it — and two
//! crossings on different faces, however close, never refuse.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::{bored_brick, halves_at_rest, section_faces, tilted, u_cut};
use crate::common::cavity::{brick, cut, prism, rod};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::bored_cylinder;
use topo::Body;
use topo::splitting::{SplitError, SplitPlane, split};

fn tol() -> Tol {
    Tol::witness()
}

fn at_x(x: f64) -> SplitPlane<f64> {
    SplitPlane {
        origin: Point3::new(x, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// Each section face of `half` as `(sense, ring count)`, in face order.
fn sections(half: &Body<f64>, plane: &SplitPlane<f64>) -> Vec<(bool, usize)> {
    section_faces(half, plane)
        .iter()
        .map(|&f| {
            let face = half.get_face(f).unwrap();
            (face.sense, face.rings.len())
        })
        .collect()
}

/// Every face's ring count, summed.
fn rings(half: &Body<f64>) -> usize {
    half.faces().map(|(_, f)| f.rings.len()).sum()
}

fn volume(half: &Body<f64>) -> f64 {
    topo::mass_properties(half, tol()).unwrap().volume
}

/// **A bore the plane crosses all round is a ring of the one section
/// face**, on each half, for a centred rod, an off-centre one and the
/// bored cylinder, flat and tilted (the tilted bore's section is an
/// ellipse, nested by the carrier separation). Where the plane is flat
/// the halves' volumes are the closed forms: half the brick less half
/// the rod, half the annular cylinder.
#[test]
fn a_split_through_a_bore_makes_one_annular_section_face_per_half() {
    use core::f64::consts::PI;
    let cases = [
        (
            "cavity cut flat",
            bored_brick(0.0, 0.0, 1.0),
            tilted(1.25, 0.0, false),
            Some(20.0 - 1.25 * PI),
        ),
        (
            "cavity cut at tilt 0.3",
            bored_brick(0.0, 0.0, 1.0),
            tilted(1.25, 0.3, false),
            None,
        ),
        (
            "off-centre cavity cut flat",
            bored_brick(0.8, 0.0, 1.0),
            tilted(1.25, 0.0, false),
            Some(20.0 - 1.25 * PI),
        ),
        (
            "bored cylinder cut flat",
            bored_cylinder(0.3, 0.2, 0.37, tol()),
            tilted(0.5, 0.0, false),
            Some(0.5 * PI * (1.0 - 0.09)),
        ),
        (
            "bored cylinder cut at tilt 0.3",
            bored_cylinder(0.3, 0.2, 0.37, tol()),
            tilted(0.5, 0.3, false),
            None,
        ),
    ];
    for (what, body, plane, half_volume) in cases {
        for (side, half) in ["below", "above"]
            .into_iter()
            .zip(halves_at_rest(what, &body, &plane))
        {
            assert_eq!(
                sections(&half, &plane),
                vec![(true, 1)],
                "{what} {side}: one section face, the bore its one ring"
            );
            if let Some(v) = half_volume {
                assert!(
                    (volume(&half) - v).abs() < 1e-9,
                    "{what} {side}: volume {} against {v}",
                    volume(&half)
                );
            }
        }
    }
}

/// **The U-cutter's pockets are two rings of one section face.** The
/// plane at `x = 3` (and at `x = 3.9`) crosses both prongs, so each
/// half's section is the block's square with the prongs' two
/// rectangles as rings; the volumes are the block's slab less the
/// prongs' part of it (`x ∈ [2, 4]` of each, `0.5 × 2` in section).
#[test]
fn a_split_through_the_u_cutters_pockets_makes_one_face_with_two_rings() {
    let body = u_cut();
    for (x, below, above) in [(3.0, 46.0, 14.0), (3.9, 58.6, 1.4)] {
        let what = format!("U-cutter split at x = {x}");
        let plane = at_x(x);
        for ((side, half), v) in ["below", "above"]
            .into_iter()
            .zip(halves_at_rest(&what, &body, &plane))
            .zip([below, above])
        {
            assert_eq!(
                sections(&half, &plane),
                vec![(true, 2)],
                "{what} {side}: one section face, each prong a ring"
            );
            assert!(
                (volume(&half) - v).abs() < 1e-9,
                "{what} {side}: volume {} against {v}",
                volume(&half)
            );
        }
    }
}

/// **A ringed cap the plane crosses is chorded beside its hole.** A
/// steep plane meets the `z = 0` cap of the brick (`z = 1` of the
/// bored cylinder) through the bore's rim, and the other cap beside
/// it: the crossed cap's fragments are each one loop with no ring,
/// the other cap keeps its ring whole on the side the rim lies on, and
/// each half's section is one face notched by the bore. Two more rod
/// positions put the plane through the bore on both caps, so the bore
/// parts each half's section in two.
#[test]
fn a_split_through_a_ringed_cap_chords_it_beside_its_hole() {
    let cases = [
        (
            "off-centre cavity cut at tilt 1.4, flipped",
            bored_brick(0.8, 0.0, 1.0),
            tilted(1.25, 1.4, true),
            [vec![(true, 0)], vec![(true, 0)]],
            [1, 0],
        ),
        (
            "bored cylinder cut at tilt -1, flipped",
            bored_cylinder(0.3, 0.2, 0.37, tol()),
            tilted(0.5, -1.0, true),
            [vec![(true, 0)], vec![(true, 0)]],
            [0, 1],
        ),
        (
            "cavity about (0.4, -0.3) cut at tilt 1.4, flipped",
            bored_brick(0.4, -0.3, 1.0),
            tilted(1.25, 1.4, true),
            [vec![(true, 0), (true, 0)], vec![(true, 0), (true, 0)]],
            [0, 0],
        ),
        (
            "cavity about (0.5, 0) cut at tilt 1.4, flipped",
            bored_brick(0.5, 0.0, 1.0),
            tilted(1.25, 1.4, true),
            [vec![(true, 0), (true, 0)], vec![(true, 0), (true, 0)]],
            [0, 0],
        ),
    ];
    for (what, body, plane, section, ringed) in cases {
        for (((side, half), section), ringed) in ["below", "above"]
            .into_iter()
            .zip(halves_at_rest(what, &body, &plane))
            .zip(section)
            .zip(ringed)
        {
            assert_eq!(
                sections(&half, &plane),
                section,
                "{what} {side}: the section faces"
            );
            assert_eq!(
                rings(&half),
                ringed,
                "{what} {side}: the rings any face carries"
            );
        }
    }
}

/// **A bored brick splits validly at every tilt and rod offset, every
/// section face counter-clockwise, and the halves' volumes add to the
/// whole.** The steep rows refused (`RingHomingAmbiguous`,
/// `TornComponent`) or came back invalid (`LoopRoleInverted`) on main
/// before the join paired a planar face's crossings along its line.
/// (Review R2's grid.)
#[test]
fn a_bored_brick_splits_at_every_tilt_and_offset() {
    let whole = 40.0 - 2.5 * core::f64::consts::PI;
    for (cx, cy) in [(0.0, 0.0), (0.4, -0.3), (0.5, 0.0), (0.95, 0.0)] {
        let body = bored_brick(cx, cy, 1.0);
        for t in [0.3, 0.9, 1.1, 1.4, 1.45] {
            for flip in [false, true] {
                let what = format!("rod ({cx}, {cy}) at tilt {t}, flipped {flip}");
                let plane = tilted(1.25, t, flip);
                let [below, above] = halves_at_rest(&what, &body, &plane);
                let sum = volume(&below) + volume(&above);
                assert!((sum - whole).abs() < 1e-6, "{what}: {sum} against {whole}");
                for (side, half) in [("below", &below), ("above", &above)] {
                    let s = sections(half, &plane);
                    assert!(
                        s.iter().all(|&(sense, _)| sense),
                        "{what} {side}: every section face counter-clockwise: {s:?}"
                    );
                }
            }
        }
    }
}

/// **A hole inside an island inside a hole goes to the island.** The
/// block less an annular groove (`R ∈ [1, 2]`, from `z = 1` up) whose
/// island is bored at `R = 0.5`: on a plane through the groove each
/// half's section is two faces, the block's square holed by the groove
/// and the island's disc holed by the bore. (Review R2's row; R1's
/// nested tube is the same claim.)
#[test]
fn a_hole_in_an_island_in_a_hole_goes_to_the_island() {
    let block = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 4.0));
    let grooved = cut("groove", &block, &rod(Point2::new(0.0, 0.0), 2.0, 1.0, 5.0));
    let island = rod(Point2::new(0.0, 0.0), 1.0, 0.5, 4.5);
    let islanded = match topo::union(&grooved, &island, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("the island unites: {:?}", other.err()),
    };
    let body = cut(
        "bore",
        &islanded,
        &rod(Point2::new(0.0, 0.0), 0.5, -1.0, 5.0),
    );
    for (t, flip) in [(0.0, false), (0.2, false), (0.4, true)] {
        let what = format!("plane through z = 2 at tilt {t}, flipped {flip}");
        let plane = tilted(2.0, t, flip);
        for (side, half) in ["below", "above"]
            .into_iter()
            .zip(halves_at_rest(&what, &body, &plane))
        {
            assert_eq!(
                sections(&half, &plane),
                vec![(true, 1), (true, 1)],
                "{what} {side}: the square holed by the groove, the island holed by the bore"
            );
        }
    }
}

/// **Two crossings on different faces never refuse, however close in
/// the sweep's order.** Two rods two apart in `y`, their facing seams
/// `g` apart in `x` with `g` across the band's ambiguity window
/// (`2e-9 … 9e-9` at the witness ε); the cut is flat, so the sweep's
/// `u` is `x` exactly. No face holds crossings of both rods, so nothing
/// the join pairs depends on their order. (Review R1/R2, finding M1.)
#[test]
fn a_flat_cut_answers_whatever_the_gap_between_crossings_on_different_faces() {
    let block = brick(Point3::new(-2.5, -2.5, 0.0), Point3::new(2.5, 2.5, 2.5));
    let first = cut(
        "first rod",
        &block,
        &rod(Point2::new(0.5, 1.0), 0.5, -0.5, 3.5),
    );
    for g in [2e-9, 5e-9, 9e-9] {
        let body = cut(
            "second rod",
            &first,
            &rod(Point2::new(1.5 + g, -1.0), 0.5, -0.5, 3.5),
        );
        let what = format!("seams {g:e} apart");
        let plane = tilted(1.25, 0.0, false);
        for (side, half) in ["below", "above"]
            .into_iter()
            .zip(halves_at_rest(&what, &body, &plane))
        {
            assert_eq!(sections(&half, &plane), vec![(true, 2)], "{what} {side}");
        }
    }
}

/// **A hairline slot crossed nearly along its length answers.** The
/// slot is `0.1 mm` wide and the plane runs `±1e-5` rad off its axis,
/// so the cap line's crossings of the slot's long walls sit a real
/// distance apart along the cap's line however close they are in the
/// sweep's `u`. (Review R2, finding M1.)
#[test]
fn a_hairline_slot_cut_nearly_along_its_axis_answers() {
    let t: f64 = 1.4;
    let w = 1e-4;
    let block = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 2.5));
    let outline = [
        (-0.8, -w / 2.0),
        (1.2, -w / 2.0),
        (1.2, w / 2.0),
        (-0.8, w / 2.0),
    ]
    .map(|(x, y)| Point2::new(x, y));
    let body = cut("slot", &block, &prism(&outline, -0.5, 3.0));
    for delta in [1e-5, -1e-5] {
        let plane = SplitPlane {
            origin: Point3::new(0.0, 0.0, 1.25),
            normal: Vec3::new(-t.sin(), -delta, -t.cos()).normalize(),
        };
        halves_at_rest(&format!("off-axis by {delta:e}"), &body, &plane);
    }
}

/// **A cap line a hair off the sweep's `v` axis never refuses at the
/// join.** The brick with a rod of radius `0.25` at `(x0, 1.72)`, `x0`
/// the steep plane's trace on the `z = 0` cap, so the cap's four
/// crossings sit at `y = −2, 1.47, 1.97, 2` (gaps 3.47, 0.5, 0.03), and
/// the body turned `δ` about `x` so the cap line leans off `v` by about
/// `δ / sin t`. Each face's crossings are ordered along that face's own
/// line, where these gaps are their real lengths, so no lean puts one
/// gap under the band and another over it: every lean splits into
/// halves at rest, or — where the lean puts a crossing vertex's sector
/// bisector in the band (`split_bisector_side`), 6 of the 16 poses
/// here — the REDUCTION refuses, before any order is read. (Review
/// R1's concern.)
#[test]
fn a_cap_line_a_hair_off_the_sweeps_v_axis_never_refuses_at_the_join() {
    let t: f64 = 1.4;
    let x0 = 1.25 * t.cos() / t.sin();
    let body = bored_brick(x0, 1.72, 0.25);
    let mut answered = 0;
    for d in [0.0, 1e-12, -1e-9, 2.6e-8, -2.6e-8, 3e-8, -1e-7, 1e-5] {
        let map = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), d);
        let posed = topo::transform_rigid(&body, &map, tol()).unwrap();
        for flip in [true, false] {
            let what = format!("lean {d:e}, flipped {flip}");
            let plane = tilted(1.25, t, flip);
            match split(&posed, &plane, tol()) {
                Err(SplitError::Reduce(_)) => continue,
                Err(e) => panic!("{what}: refused past the reduction: {e:?}"),
                Ok(_) => {}
            }
            for (side, half) in ["below", "above"]
                .into_iter()
                .zip(halves_at_rest(&what, &posed, &plane))
            {
                assert!(
                    sections(&half, &plane).iter().all(|&(sense, _)| sense),
                    "{what} {side}: every section face counter-clockwise"
                );
            }
            answered += 1;
        }
    }
    assert_eq!(answered, 10, "poses answered, of 16");
}

/// **Where nothing decides a clockwise polygon's place, it keeps its
/// own face.** The unit cylinder of height 2.5 with its seams turned to
/// `π/2 + 0.05`, cut through `(0, 0, 1.25)` at tilt 1.1 (both caps and
/// both seams crossed): the join chords the wall face holding both top
/// crossings across the arc outside it, and the section comes back as
/// the whole ellipse plus two clockwise 2-gons touching it, which the
/// nesting leaves standing. The halves pass tier 3 and read right by
/// cancellation.
///
/// This row pins that fallback firing. It goes red when
/// `work/cleave/split-pairs-curved-face-crossings-across-the-wrong-arc.md`
/// is fixed — each half's section then one counter-clockwise face —
/// and is rewritten to that, which retires the fallback's one known
/// customer.
#[test]
fn a_clockwise_section_nothing_places_keeps_its_face() {
    let turn = core::f64::consts::FRAC_PI_2 + 0.05;
    let cylinder: Body<f64> = sweep::test_support::prism(
        vec![
            (Point2::new(turn.cos(), turn.sin()), 1.0),
            (Point2::new(-turn.cos(), -turn.sin()), 1.0),
        ],
        2.5,
        tol(),
    );
    for flip in [false, true] {
        let plane = tilted(1.25, 1.1, flip);
        for (side, half) in ["below", "above"].into_iter().zip(halves_at_rest(
            "the turned cylinder",
            &cylinder,
            &plane,
        )) {
            let mut s = sections(&half, &plane);
            s.sort_unstable();
            assert_eq!(
                s,
                vec![(false, 0), (false, 0), (true, 0)],
                "flipped {flip} {side}: the ellipse and its two cancelling 2-gons"
            );
        }
    }
}
