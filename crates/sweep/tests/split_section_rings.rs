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
use topo::splitting::{SplitError, SplitJoinError, SplitPlane, split};

fn tol() -> Tol {
    Tol::witness()
}

fn at_x(x: f64) -> SplitPlane<f64> {
    topo::test_support::split_plane(Point3::new(x, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))
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
    // The volumes' agreement is read at the run's resolution.
    let close = (1e3 * tol().eps()).max(1e-6);
    let mut measured = 0;
    for (cx, cy) in [(0.0, 0.0), (0.4, -0.3), (0.5, 0.0), (0.95, 0.0)] {
        let body = bored_brick(cx, cy, 1.0);
        for t in [0.3, 0.9, 1.1, 1.4, 1.45] {
            for flip in [false, true] {
                let what = format!("rod ({cx}, {cy}) at tilt {t}, flipped {flip}");
                let plane = tilted(1.25, t, flip);
                let [below, above] = halves_at_rest(&what, &body, &plane);
                // A tilted ellipse wall's quadrature may escalate at a
                // tight ε (`props_quad_converged`); that pose's volume is
                // then not read, and the count below says how many were.
                let volumes = [&below, &above].map(|h| topo::mass_properties(h, tol()));
                let quadrature = |r: &Result<topo::MassProperties<f64>, topo::MassPropsError>| {
                    matches!(
                        r,
                        Err(topo::MassPropsError::Face {
                            source: geom_brep::PropsError::Escalated { cause },
                            ..
                        }) if cause.predicate == Some("props_quad_converged")
                    )
                };
                match &volumes {
                    [Ok(b), Ok(a)] => {
                        let sum = b.volume + a.volume;
                        assert!((sum - whole).abs() < close, "{what}: {sum} against {whole}");
                        measured += 1;
                    }
                    [b, a] if [b, a].iter().all(|r| r.is_ok() || quadrature(r)) => {}
                    other => panic!("{what}: volume refused: {other:?}"),
                }
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
    // 40 at ε = 1e-9 and 1e-6, 36 at 1e-12: the fewest over the rows.
    assert!(
        measured >= 36,
        "volumes read on only {measured} of 40 poses"
    );
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
/// `g` apart in `x` with `g ∈ {2, 5, 9}·ε`, across the band's ambiguity
/// window `(ε, K·ε)` at every ε row; the cut is flat, so the sweep's
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
    for g in [2.0, 5.0, 9.0].map(|k| k * tol().eps()) {
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
/// slot is `0.1 mm` wide (`w`) and the steep plane (tilt `t = 1.4`)
/// leans `±δ` off its axis, so on the `z = 0` cap the plane's line
/// crosses the slot's two long walls a real `w / δ` apart along the
/// line, while in the sweep's frame (`u` the plane's projection of
/// `x`) the two crossings differ in `u` by `δ·w·|cos²t − sin²t| /
/// (sin t·cos t)`. `δ` is chosen to put that at `5·ε`, inside the
/// band's ambiguity window `(ε, K·ε)` at every ε row: a pairing read
/// off the sweep's order with the band would refuse here. (Review R2,
/// finding M1.)
#[test]
fn a_hairline_slot_cut_nearly_along_its_axis_answers() {
    let t: f64 = 1.4;
    let w = 1e-4;
    let (s, c) = t.sin_cos();
    let lean = 5.0 * tol().eps() * s * c / (w * (c * c - s * s).abs());
    let block = brick(Point3::new(-3.0, -3.0, 0.0), Point3::new(3.0, 3.0, 2.5));
    let outline = [
        (-0.8, -w / 2.0),
        (1.2, -w / 2.0),
        (1.2, w / 2.0),
        (-0.8, w / 2.0),
    ]
    .map(|(x, y)| Point2::new(x, y));
    let body = cut("slot", &block, &prism(&outline, -0.5, 3.0));
    for delta in [lean, -lean] {
        let plane = topo::test_support::split_plane(
            Point3::new(0.0, 0.0, 1.25),
            Vec3::new(-s, -delta, -c).normalize(),
        );
        halves_at_rest(&format!("off-axis by {delta:e}"), &body, &plane);
    }
}

/// **A cap line a hair off the sweep's `v` axis never refuses at the
/// join.** The brick with a rod of radius `0.25` at `(x0, 1.72)`, `x0`
/// the steep plane's trace on the `z = 0` cap, so the cap's four
/// crossings sit at `y = −2, 1.47, 1.97, 2` (gaps 3.47, 0.5, 0.03), and
/// the body turned `δ` about `x` so the cap line leans off `v` by about
/// `δ / sin t`. The leans `δ / sin t ∈ {±26, ±30}·ε` put the 0.03 gap's
/// `u` difference under `ε` and the 0.5 gap's over `K·ε` at every ε
/// row — a column order would read one line as two columns. Each face's crossings are ordered along that face's own
/// line, where these gaps are their real lengths, so no lean puts one
/// gap under the band and another over it: no lean refuses at the
/// join, the unleaned pose answers, and every pose that answers is at
/// rest. A lean that puts a crossing vertex's sector bisector in the
/// band refuses at the reduction (`split_bisector_side`), and one the
/// pcurve mint cannot certify refuses there (`pcurve_trim_containment`)
/// — stages before and after the join's order, not this row's subject.
/// (Review R1's concern.)
#[test]
fn a_cap_line_a_hair_off_the_sweeps_v_axis_never_refuses_at_the_join() {
    let t: f64 = 1.4;
    let x0 = 1.25 * t.cos() / t.sin();
    let body = bored_brick(x0, 1.72, 0.25);
    let eps = tol().eps();
    for d in [0.0, 26.0, -26.0, 30.0, -30.0].map(|k| k * eps * t.sin()) {
        let map = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), d);
        let posed = topo::transform_rigid(&body, &map, tol()).unwrap();
        for flip in [true, false] {
            let what = format!("lean {d:e}, flipped {flip}");
            let plane = tilted(1.25, t, flip);
            match split(&posed, &plane, tol()) {
                Err(e @ SplitError::Join(_)) => panic!("{what}: refused at the join: {e:?}"),
                Err(e) if d == 0.0 => panic!("{what}: the unleaned pose refused: {e:?}"),
                Err(_) => continue,
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
        }
    }
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

/// **A thin tube cut at a tilt is one annular face per half.** The unit
/// cylinder bored concentrically at radius `a` up to `0.9`, cut at
/// tilts 0.1 and 0.3: the bore's elliptic section nests in the
/// cylinder's because the separation reads the hole's true reach (the
/// largest singular value), not the `√2`-loose sum of its semi-axes,
/// which left `a ≥ 0.75` un-nested. (Review R1/R2.)
#[test]
fn a_thin_tube_cut_at_a_tilt_is_one_annular_face_per_half() {
    for a in [0.75, 0.8, 0.9] {
        let body = bored_cylinder(a, 0.0, 0.37, tol());
        for t in [0.1, 0.3] {
            let what = format!("tube bored at {a}, tilt {t}");
            let plane = tilted(0.5, t, false);
            for (side, half) in ["below", "above"]
                .into_iter()
                .zip(halves_at_rest(&what, &body, &plane))
            {
                assert_eq!(sections(&half, &plane), vec![(true, 1)], "{what} {side}");
            }
        }
    }
}

/// **A plane through a notch's tip line refuses at the join, typed, as
/// main does.** The block `[0, 4]² × [0, 2]` less a V-notch whose tip
/// line is `x = 2, y = 2`, alone and with a second notch beside it, cut
/// by planes through that tip line. The fixed partners of the line's
/// crossings meet across a face an earlier chord divided, so they are
/// returned to the book's rule, which leaves two ends unpaired:
/// `Join(UnpairedLooseEnds { count: 2 })`, the refusal main gives. (It
/// reached the Euler layer as `NotSameFace` before the partners were
/// re-checked at use.) This row pins the refusal's stage and kind, not
/// that the pose should refuse.
#[test]
fn a_plane_through_a_notch_tip_refuses_at_the_join() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 2.0));
    let notch = [(1.0, 5.0), (2.0, 2.0), (3.0, 5.0)].map(|(x, y)| Point2::new(x, y));
    let v = cut("notch", &block, &prism(&notch, -1.0, 3.0));
    let second = [(0.5, 4.5), (1.0, 2.0), (1.5, 4.5)].map(|(x, y)| Point2::new(x, y));
    let two = cut("second notch", &v, &prism(&second, -1.0, 3.0));
    for (name, body) in [("one notch", &v), ("two notches", &two)] {
        for (o, n) in [
            (Point3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 1.0, 0.3)),
            (Point3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 1.0)),
        ] {
            let plane = topo::test_support::split_plane(o, n.normalize());
            assert!(
                matches!(
                    split(body, &plane, tol()),
                    Err(SplitError::Join(SplitJoinError::UnpairedLooseEnds {
                        count: 2
                    }))
                ),
                "{name}, plane through {o:?}: refuses with two ends unpaired"
            );
        }
    }
}

/// Twice the signed area of a polygon's corners in `(u, v)`.
fn twice_area(polygon: &topo::SectionPolygon<f64>) -> f64 {
    let uv = &polygon.uv;
    (0..uv.len())
        .map(|i| {
            let (a, b) = (uv[i], uv[(i + 1) % uv.len()]);
            a.x * b.y - b.x * a.y
        })
        .sum()
}

/// **`plane_section` reports the U-cutter's section as one region, the
/// prongs its two holes**: the outline the block's `4 × 4` square
/// counter-clockwise, each hole a prong's `0.5 × 2` rectangle
/// clockwise.
#[test]
fn plane_section_of_the_u_cutter_is_one_region_with_two_holes() {
    let body = u_cut();
    for x in [3.0, 3.9] {
        let s = topo::plane_section(&body, &at_x(x), tol()).unwrap();
        assert_eq!(s.regions.len(), 1, "x = {x}: one region");
        let region = &s.regions[0];
        assert_eq!(twice_area(&region.outline), 32.0, "x = {x}: the outline");
        let holes: Vec<f64> = region.holes.iter().map(twice_area).collect();
        assert_eq!(holes, [-2.0, -2.0], "x = {x}: the prongs");
        for hole in &region.holes {
            assert!(
                hole.points
                    .iter()
                    .all(|p| (p.y - 1.0).abs() <= 2.0 && (1.0..=3.0).contains(&p.z)),
                "x = {x}: a hole's corners are a prong's: {:?}",
                hole.points
            );
        }
    }
}

/// **`plane_section` reports a bored body's section as one region, the
/// bore its hole**, for the flat and tilted cuts the split nests: every
/// corner of the hole lies on the bore, every corner of the outline
/// off it. The bore's circle has two corners, so its winding is read
/// on its arcs, not its corners' shoelace.
#[test]
fn plane_section_of_a_bored_body_is_one_region_with_the_bore_its_hole() {
    let cases = [
        (
            "cavity cut flat",
            bored_brick(0.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),
            tilted(1.25, 0.0, false),
        ),
        (
            "cavity cut at tilt 0.3",
            bored_brick(0.0, 0.0, 1.0),
            (0.0, 0.0, 1.0),
            tilted(1.25, 0.3, false),
        ),
        (
            "off-centre cavity cut flat",
            bored_brick(0.8, 0.0, 1.0),
            (0.8, 0.0, 1.0),
            tilted(1.25, 0.0, false),
        ),
        (
            "bored cylinder cut at tilt 0.3",
            bored_cylinder(0.3, 0.2, 0.37, tol()),
            (0.2, 0.0, 0.3),
            tilted(0.5, 0.3, false),
        ),
    ];
    for (what, body, (cx, cy, r), plane) in cases {
        let s = topo::plane_section(&body, &plane, tol()).unwrap();
        assert_eq!(s.regions.len(), 1, "{what}: one region");
        let region = &s.regions[0];
        assert_eq!(region.holes.len(), 1, "{what}: the bore is the one hole");
        let on_bore = |p: &Point3<f64>| ((p.x - cx).hypot(p.y - cy) - r).abs() < 1e-9;
        assert!(
            region.holes[0].points.iter().all(on_bore),
            "{what}: the hole's corners lie on the bore: {:?}",
            region.holes[0].points
        );
        assert!(
            !region.outline.points.iter().any(on_bore),
            "{what}: the outline's corners lie off the bore: {:?}",
            region.outline.points
        );
    }
}

/// **An island inside a hole is a region of its own**, holding the hole
/// inside it: the grooved block of
/// `a_hole_in_an_island_in_a_hole_goes_to_the_island`, sliced flat at
/// `z = 2`, is the square holed by the groove and the island's disc
/// holed by the bore.
#[test]
fn plane_section_puts_a_hole_in_an_island_in_the_islands_region() {
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
    let s = topo::plane_section(&body, &tilted(2.0, 0.0, false), tol()).unwrap();
    let radius = |p: &Point3<f64>| p.x.hypot(p.y);
    let mut regions: Vec<(usize, Vec<f64>)> = s
        .regions
        .iter()
        .map(|r| {
            let holes = r.holes.iter().flat_map(|h| h.points.iter().map(radius));
            (r.outline.points.len(), holes.collect())
        })
        .collect();
    regions.sort_by_key(|r| r.0);
    assert_eq!(regions.len(), 2, "the square and the island: {regions:?}");
    let island_holes = &regions[0].1;
    let square_holes = &regions[1].1;
    assert!(
        island_holes.iter().all(|&r| (r - 0.5).abs() < 1e-9) && !island_holes.is_empty(),
        "the island holds the bore: {regions:?}"
    );
    assert!(
        square_holes.iter().all(|&r| (r - 2.0).abs() < 1e-9) && !square_holes.is_empty(),
        "the square holds the groove: {regions:?}"
    );
}

/// **A hole nothing places refuses**: the turned cylinder of
/// `a_clockwise_section_nothing_places_keeps_its_face`, whose join mints
/// two clockwise polygons touching the ellipse around them. `split`
/// keeps them as faces cancelling the ellipse's; a section's regions
/// cannot state them, so `plane_section` refuses. This row goes red with
/// that one when
/// `work/cleave/split-pairs-curved-face-crossings-across-the-wrong-arc.md`
/// is fixed.
#[test]
fn plane_section_refuses_a_hole_nothing_places() {
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
        let r = topo::plane_section(&cylinder, &tilted(1.25, 1.1, flip), tol());
        assert!(
            matches!(r, Err(topo::SectionError::UnplacedHole { .. })),
            "flipped {flip}: {:?}",
            r.map(|s| s.regions.len())
        );
    }
}
