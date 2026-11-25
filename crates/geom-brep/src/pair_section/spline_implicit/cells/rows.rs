//! **The cell rows**: an arc's cells read and refined at a band the row
//! names, on the carry rows' wall against a rod and on a flat wall
//! against hand-built arcs. Each row names the mutant that turns it
//! red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point2, Point3, Tol, Vec3};

use super::super::carry_rows::{band, pt, rod, rod_arcs, wall};
use super::super::{Bracketed, SplineImplicitPose};
use super::{Enclosure, Listed, Stand, apart, stand};
use crate::pair_section::{
    CarriedComponent, CarryError, CarryRefusal, ComponentEnds, ComponentShape, PadBox, PadsStop,
    TubeRun,
};
use crate::ssi::enclose::Box3;
use crate::ssi::residual::ImplicitForm;
use crate::ssi::{SsiBranch, SsiTube};

/// What a whole reading hands the cells: the pose, the partner's form,
/// and the branches, lowest witness first.
type Read = (SplineImplicitPose<f64>, ImplicitForm, Vec<SsiBranch>);

/// The whole reading of the carry rows' wall against `p` at the run's
/// band, or the refusal it stops on before any carry.
fn try_read(p: &Surface<f64>) -> Result<Read, crate::ssi::SsiError> {
    let w = wall(1.0);
    let pose = SplineImplicitPose::of(&w, p).expect("a spline wall and a cylinder");
    let reading = crate::ImplicitSection::of(p, pose.net(), pose.domain(), band())?;
    let form = reading.form().clone();
    let mut branches = reading.all()?.branches;
    assert_eq!(branches.len(), 2, "the rod cuts the wall in two arcs");
    branches.sort_by(|a, b| a.witness.z.total_cmp(&b.witness.z));
    Ok((pose, form, branches))
}

/// [`try_read`], read.
fn read(p: &Surface<f64>) -> Read {
    try_read(p).expect("the pair reads whole")
}

/// The band whose escalation is `escalate`, at the run's `K`.
fn band_escalating(escalate: f64) -> Band {
    Band::linear_at(Tol::witness(), escalate / Tol::witness().k())
        .expect("the row's scale forms a band")
}

fn open() -> ComponentShape {
    ComponentShape::Open {
        ends: ComponentEnds::default(),
    }
}

fn cells<'e, 'a>(arc: &'e Listed<'a>) -> &'e Enclosure<'a> {
    arc.cells().expect("the arc's cells read")
}

/// Every cell's chart rectangle, arc by arc: what a refinement moves.
fn rects(arcs: &[Listed<'_>]) -> Vec<crate::ssi::exhaust::UvRect> {
    arcs.iter()
        .flat_map(|a| cells(a).cells.iter().map(|c| c.rect))
        .collect()
}

/// The arc's component, carried.
fn carried(arc: &Listed<'_>, b: &SsiBranch) -> CarriedComponent<f64> {
    cells(arc).carried(b, open()).expect("the arc carries")
}

/// The stop each arc's `PadsMeet` names; panics on an arc that stands
/// apart or refuses otherwise.
fn stops(meets: &[Option<CarryRefusal>]) -> Vec<PadsStop> {
    meets
        .iter()
        .map(|m| match m {
            Some(CarryRefusal::PadsMeet { stop, .. }) => *stop,
            other => panic!("the arcs refuse `PadsMeet`: {other:?}"),
        })
        .collect()
}

/// How many pairs of the two arcs' boxes share a point once each is
/// widened by `by`, as a site's read widens them.
fn widened_overlaps(a: &CarriedComponent<f64>, b: &CarriedComponent<f64>, by: f64) -> usize {
    a.boxes
        .iter()
        .flat_map(|x| b.boxes.iter().map(move |y| (x, y)))
        .filter(|(x, y)| (0..3).all(|k| x.lo[k].max(y.lo[k]) - by <= x.hi[k].min(y.hi[k]) + by))
        .count()
}

/// The flat wall `S(u, v) = (u, v, 0)`.
fn flat() -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(
        k.clone(),
        k,
        vec![
            pt(0.0, 0.0, 0.0),
            pt(0.0, 1.0, 0.0),
            pt(1.0, 0.0, 0.0),
            pt(1.0, 1.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap()
}

/// A hand-built branch on the flat wall: `carrier` with the pcurve
/// `pcurve` under it, certified by a chart tube of rung `rung` (the wall
/// has unit chart speed, so its pads are the rung) with the node boxes
/// `nodes`, its limb-2 bound `hull_sup`.
fn hand_built(
    carrier: NurbsCurve3<f64>,
    pcurve: NurbsCurve2<f64>,
    rung: f64,
    nodes: [Option<(f64, f64)>; 2],
    hull_sup: f64,
) -> SsiBranch {
    SsiBranch {
        witness: carrier.eval(0.5),
        carrier: geom::Curve3::Nurbs(std::sync::Arc::new(carrier)),
        params: (0.0, 1.0),
        end: crate::ssi::BranchEnd::Closed,
        certificate: crate::ssi::certify::SsiCertificate {
            samples: 0,
            on_locus_max: 0.0,
            hull_sup,
            tube: SsiTube::Chart {
                rung,
                pad_u: rung,
                pad_v: rung,
                nodes,
            },
            tube_transversality: 1.0,
            tube_boxes: 1,
        },
        pcurve_a: None,
        pcurve_b: Some(pcurve),
        march_tol: 1e-9,
    }
}

/// A straight hand-built branch on the flat wall from `(0, y)` to
/// `(1, y + slope)`, its degree-1 pcurve broken at the interior knots
/// `inner`.
fn line(y: f64, slope: f64, inner: &[f64], rung: f64, nodes: [Option<(f64, f64)>; 2]) -> SsiBranch {
    let mut k = vec![0.0, 0.0];
    k.extend_from_slice(inner);
    k.extend([1.0, 1.0]);
    let knots = || KnotVector::clamped(k.clone(), 1).unwrap();
    let us: Vec<f64> = [0.0]
        .into_iter()
        .chain(inner.iter().copied())
        .chain([1.0])
        .collect();
    let ones = vec![1.0; us.len()];
    let at = |u: f64| (u, y + slope * u);
    let carrier = NurbsCurve3::new(
        knots(),
        us.iter().map(|&u| pt(at(u).0, at(u).1, 0.0)).collect(),
        ones.clone(),
    )
    .unwrap();
    let pcurve = NurbsCurve2::new(
        knots(),
        us.iter().map(|&u| Point2::new(at(u).0, at(u).1)).collect(),
        ones,
    )
    .unwrap();
    hand_built(carrier, pcurve, rung, nodes, 1e-12)
}

/// The rod along `u` about the flat wall's line `v = ½`, of radius `r`:
/// it cuts the wall in the lines `v = ½ ± r`.
fn flat_rod(r: f64) -> Surface<f64> {
    Surface::Cylinder {
        frame: geom::Frame::new(pt(0.0, 0.5, 0.0), Vec3::unit_x(), Vec3::unit_z()),
        radius: r,
    }
}

fn boxed(lo: [f64; 3], hi: [f64; 3]) -> Box3 {
    Box3 {
        x: Interval::from_bounds(lo[0], hi[0]),
        y: Interval::from_bounds(lo[1], hi[1]),
        z: Interval::from_bounds(lo[2], hi[2]),
    }
}

/// **Cells stand apart exactly where no site is read in both**: a site
/// is read in a cell widened by the band's escalation `e` along each
/// axis, so two cells stand apart where, widened so, they share no
/// point. Two cells `1.5·e` apart along two axes are farther than `2·e`
/// apart as the crow flies, and both hold the point between them: they
/// meet, and the two components carrying them both read that point. Two
/// cells `2.5·e` apart along one axis stand apart, and no point is read
/// on both.
///
/// Red against: the gap read as a distance (the diagonal pair apart,
/// their point read on two arcs); cells widened by half the escalation
/// (the same).
#[test]
fn cells_stand_apart_exactly_where_no_site_is_read_in_both() {
    let b = band();
    let e = b.escalate();
    let h = 1e-3;
    let component = |lo: [f64; 3], hi: [f64; 3]| {
        let mid = |k: usize| 0.5 * (lo[k] + hi[k]);
        let carrier = geom::Curve3::line(pt(0.0, mid(1), mid(2)), geom_core::UnitVec3::exact_x());
        CarriedComponent {
            witness: carrier.eval(0.5),
            carrier,
            span: (0.0, 1.0),
            shape: open(),
            certificate: None,
            winds: [false, false],
            pad: 2.0 * h,
            reach: 0.0,
            boxes: vec![PadBox {
                lo,
                hi,
                pad: 2.0 * h,
                node: None,
            }],
            runs: vec![TubeRun {
                lo,
                hi,
                run: (0.0, 1.0),
            }],
        }
    };
    let (lo_a, hi_a) = ([0.0, -h, -h], [1.0, 0.0, 0.0]);
    let reads_both = |lo_b: [f64; 3], hi_b: [f64; 3]| {
        let p = pt(0.5, 0.5 * lo_b[1], 0.5 * lo_b[2]);
        let on = |c: CarriedComponent<f64>| c.reads(p, b).expect("the point reads").is_some();
        on(component(lo_a, hi_a)) && on(component(lo_b, hi_b))
    };
    let diagonal = ([0.0, 1.5 * e, 1.5 * e], [1.0, 1.5 * e + h, 1.5 * e + h]);
    assert!(
        matches!(
            stand(boxed(lo_a, hi_a), boxed(diagonal.0, diagonal.1), &[], e),
            Stand::Meet(gap) if gap > e && gap < 2.0 * e
        ),
        "cells 1.5 e apart along two axes meet"
    );
    assert!(
        reads_both(diagonal.0, diagonal.1),
        "and the point between them reads on both"
    );
    let along = ([0.0, 2.5 * e, -h], [1.0, 2.5 * e + h, 0.0]);
    assert_eq!(
        stand(boxed(lo_a, hi_a), boxed(along.0, along.1), &[], e),
        Stand::Apart,
        "cells 2.5 e apart along one axis stand apart"
    );
    assert!(
        !reads_both(along.0, along.1),
        "and the point between them reads on one at most"
    );
}

/// **The boxes of two carried arcs share no point a site could be read
/// in**: the rod of radius `0.09`, straight along the wall and turned
/// 30° in it, at a band whose escalation is `0.04` and `0.02`. Once the
/// arcs stand apart, no box of the one, widened by the escalation,
/// shares a point with a widened box of the other.
///
/// At `ε = 1e-6` the turned rod is not read: its whole reading refuses
/// the tube before any carry (asserted, with the ε it is allowed at).
///
/// Red against: the gap read as a distance, where the turned rod's arcs
/// run diagonally and ninety pairs of their boxes overlap once widened;
/// cells widened by half the escalation.
#[test]
fn the_boxes_of_two_carried_arcs_share_no_widened_point() {
    let turn = 30f64.to_radians();
    let turned = Surface::Cylinder {
        frame: geom::Frame::new(
            pt(0.0, 1.15, 1.0),
            Vec3::new(turn.cos(), 0.0, turn.sin()),
            Vec3::new(-turn.sin(), 0.0, turn.cos()),
        ),
        radius: 0.09,
    };
    for (what, p, escalate) in [("straight", rod(1.0, 0.09), 0.04), ("turned", turned, 0.02)] {
        let (pose, form, branches) = match try_read(&p) {
            Ok(read) => read,
            Err(unread) => {
                assert!(
                    what == "turned" && Tol::witness().eps() == 1e-6,
                    "{what}: the pair reads whole: {unread:?}"
                );
                println!("{what}: not read at this ε, before any carry: {unread:?}");
                continue;
            }
        };
        let tight = band_escalating(escalate);
        let partner = Bracketed {
            surface: &p,
            form: &form,
        };
        let first = |b| {
            Listed::arc(Enclosure::first(
                b,
                pose.net(),
                partner,
                [None, None],
                tight,
            ))
        };
        let mut both: Vec<Listed<'_>> = branches.iter().map(first).collect();
        let meets = apart(&mut both, tight);
        assert!(
            meets.iter().all(Option::is_none),
            "{what}: the rod's arcs stand apart: {meets:?}"
        );
        let c = [
            carried(&both[0], &branches[0]),
            carried(&both[1], &branches[1]),
        ];
        assert_eq!(
            widened_overlaps(&c[0], &c[1], tight.escalate()),
            0,
            "{what}: no two boxes of the arcs hold one point"
        );
    }
}

/// **Two arcs that stand within the band of each other stall, and carry
/// neither**: at a band whose escalation is `0.1`, the rod's two arcs,
/// `0.18` apart, lie within twice it of each other, and no cell between
/// them is cleared (the rod's wall is within `0.09` of every point
/// between). Every half is kept and still meets, so the share of halves
/// still meeting does not fall over two cycles: the pair has stopped
/// narrowing, and both arcs refuse `PadsMeet` naming the stall, before
/// the work bound.
///
/// Red against: a refinement with no stop (it does not return); one
/// with no stall (the refusal names the work bound); one that reports
/// arcs apart when it stops.
#[test]
fn two_arcs_within_the_band_of_each_other_stall_and_carry_neither() {
    let p = rod(1.0, 0.09);
    let (pose, form, branches) = read(&p);
    let wide = band_escalating(0.1);
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let first = |b| Listed::arc(Enclosure::first(b, pose.net(), partner, [None, None], wide));
    let mut arcs: Vec<Listed<'_>> = branches.iter().map(first).collect();
    let meets = apart(&mut arcs, wide);
    assert_eq!(
        stops(&meets),
        [PadsStop::Stall; 2],
        "arcs that do not narrow stall"
    );
    for meet in &meets {
        let Some(CarryRefusal::PadsMeet { gap, .. }) = meet else {
            unreachable!("read above")
        };
        assert!(
            *gap <= 2.0 * wide.escalate(),
            "the arcs' cells stand {gap} apart, within twice the band's escalation"
        );
    }
}

/// **Two arcs that keep narrowing and never part end at the work
/// bound**: at a band whose escalation is `0.06`, the rod's arcs,
/// `0.18` apart, are each cleared to within the band of the rod, which
/// leaves their cells nearer than twice the escalation for good. Every
/// cycle clears halves, so the stall does not speak, and the refinement
/// ends at its work bound, both arcs refusing `PadsMeet` naming it.
///
/// Red against: a refinement with no work bound (it does not return).
#[test]
fn two_arcs_that_narrow_and_never_part_end_at_the_work_bound() {
    let p = rod(1.0, 0.09);
    let (pose, form, branches) = read(&p);
    let wide = band_escalating(0.06);
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let first = |b| Listed::arc(Enclosure::first(b, pose.net(), partner, [None, None], wide));
    let mut arcs: Vec<Listed<'_>> = branches.iter().map(first).collect();
    assert_eq!(
        stops(&apart(&mut arcs, wide)),
        [PadsStop::Work; 2],
        "arcs that narrow without parting end at the work bound"
    );
}

/// **Two arcs the first reading does not part are refined until they
/// read apart, and a point reads as it does with no neighbour**: at a
/// band whose escalation is `0.04`, the rod's arcs, `0.18` apart, stand
/// more than four escalations apart, so cells within the band of each
/// arc stand apart; the first reading's cells, a rung (`0.089`) across,
/// do not. Refined, both arcs carry; every point of each arc (bisected
/// along the wall's iso-lines) reads on its own arc and not on the
/// other; and the lower arc read alone, which keeps its first reading,
/// reads each of its points at the same foot. The refined cells a site
/// reads carry their own pads, narrower than the arc's.
///
/// Red against: arcs held apart on their first reading alone (both
/// refuse); a refinement that drops a half holding a point of the arc
/// (the point reads on no arc); a cell's box carrying the arc's pad.
#[test]
fn two_arcs_are_refined_until_they_read_apart_and_a_point_reads_as_it_does_alone() {
    let p = rod(1.0, 0.09);
    let (pose, form, branches) = read(&p);
    let tight = band_escalating(0.04);
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let first = |b| {
        Listed::arc(Enclosure::first(
            b,
            pose.net(),
            partner,
            [None, None],
            tight,
        ))
    };
    let mut alone: Vec<Listed<'_>> = vec![first(&branches[0])];
    assert!(
        apart(&mut alone, tight).iter().all(Option::is_none),
        "an arc alone stands apart"
    );
    let mut both: Vec<Listed<'_>> = branches.iter().map(first).collect();
    let meets = apart(&mut both, tight);
    assert!(
        meets.iter().all(Option::is_none),
        "refined, the arcs stand apart: {meets:?}"
    );
    let alone = carried(&alone[0], &branches[0]);
    let both = [
        carried(&both[0], &branches[0]),
        carried(&both[1], &branches[1]),
    ];
    assert_ne!(
        both[0].boxes, alone.boxes,
        "the pair was refined past the first reading the arc keeps alone"
    );
    assert!(
        both[0].boxes.iter().any(|b| b.pad < 0.75 * both[0].pad),
        "a refined cell carries its own pad, under the arc's {}",
        both[0].pad
    );
    let w = wall(1.0);
    for (k, points) in rod_arcs(&w, 1.0, 0.09).into_iter().enumerate() {
        for q in points {
            let on = |c: &CarriedComponent<f64>| c.reads(q, tight).expect("the point reads");
            let own = on(&both[k]).unwrap_or_else(|| panic!("{q:?} reads on its own arc, {k}"));
            assert!(on(&both[1 - k]).is_none(), "{q:?} reads on arc {k} alone");
            if k == 0 {
                let lone = on(&alone).unwrap_or_else(|| panic!("{q:?} reads on its arc alone"));
                assert!(
                    (lone - own).abs() < 1e-9,
                    "{q:?} reads the same foot with a neighbour ({own}) and without ({lone})"
                );
            }
        }
    }
}

/// **The work bound grows with the section**: the rod at radius `0.09`
/// and at `0.02`, each at a band whose escalation is `0.236` of its
/// arcs' gap. The small rod's arcs are read in eight times the cells,
/// and both pairs are refined and carried.
///
/// Red against: a work bound of one number for every section (the small
/// rod refuses after one round).
#[test]
fn the_work_bound_grows_with_the_sections_first_reading() {
    for r in [0.09, 0.02] {
        let p = rod(1.0, r);
        let (pose, form, branches) = read(&p);
        let tight = band_escalating(0.236 * 2.0 * r);
        let partner = Bracketed {
            surface: &p,
            form: &form,
        };
        let first = |b| {
            Listed::arc(Enclosure::first(
                b,
                pose.net(),
                partner,
                [None, None],
                tight,
            ))
        };
        let mut arcs: Vec<Listed<'_>> = branches.iter().map(first).collect();
        let before = rects(&arcs);
        let meets = apart(&mut arcs, tight);
        assert!(
            meets.iter().all(Option::is_none),
            "r = {r}: the arcs are carried: {meets:?}"
        );
        assert_ne!(before, rects(&arcs), "r = {r}: the pair was refined");
    }
}

/// **A node's box another arc does not share is kept whole, and does
/// not stall its neighbour**: on the flat wall, the line `v = ½ − r`
/// ends in a node's box a tenth across, and the line `v = ½ + r`, which
/// ends at no node, has cells within reach of that box at a band whose
/// escalation is `0.025` or `0.026`. The box cannot be halved; the
/// other arc's cells can, and narrow until they part from it. Both
/// arcs carry, and the node's box is still one cell.
///
/// Red against: a stall that counts a cell it cannot halve (the pair
/// refuses while the other arc still narrows); a node's box halved in
/// refinement (more than one cell marked with the node).
#[test]
fn a_node_box_another_arc_does_not_share_does_not_stall_its_neighbour() {
    let (r, rung, h) = (0.09, 0.04, 0.1);
    let net = flat();
    let p = flat_rod(r);
    let form = ImplicitForm::of(&p).expect("a cylinder has a form");
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let a = line(0.5 - r, 0.0, &[0.5, 0.9], rung, [None, Some((h, h))]);
    let b = line(0.5 + r, 0.0, &[0.5, 0.9], rung, [None, None]);
    for escalate in [0.025, 0.026] {
        let bd = band_escalating(escalate);
        let mut arcs = vec![
            Listed::arc(Enclosure::first(&a, &net, partner, [None, Some(0)], bd)),
            Listed::arc(Enclosure::first(&b, &net, partner, [None, None], bd)),
        ];
        let before = rects(&arcs[1..]);
        let meets = apart(&mut arcs, bd);
        assert!(
            meets.iter().all(Option::is_none),
            "escalation {escalate}: the arcs part from the node's box: {meets:?}"
        );
        assert_ne!(
            before,
            rects(&arcs[1..]),
            "escalation {escalate}: the other arc was refined to part from it"
        );
        let node_cells = carried(&arcs[0], &a)
            .boxes
            .iter()
            .filter(|b| b.node == Some(0))
            .count();
        assert_eq!(
            node_cells, 1,
            "escalation {escalate}: the node's box is whole"
        );
    }
}

/// **Two arcs ending at different nodes whose boxes meet carry
/// neither**: on the flat wall, the lines `v = ½ ∓ r` each end in a
/// node's box, at two nodes, and the boxes stand within twice the
/// band's escalation of each other. The boxes are kept whole and never
/// part, and nothing else that still meets can be halved, so the pair
/// stalls and refuses, in two cases:
///
/// - at a rung of `0.06` and an escalation of `0.038`, the lines' own
///   cells meet at first and part once refined, the boxes (`0.116`
///   across, `0.064` apart) left meeting;
/// - at a rung of `0.08` and an escalation of `0.036`, only the boxes
///   (`0.11` across, `0.07` apart) meet, and the pair stalls at once
///   with no cell halved.
///
/// The same boxes at one node both arcs end at are where the arcs meet,
/// and both carry.
///
/// Red against: two arcs that each end at a node read as sharing one
/// (the boxes stand at the node, and both carry); a cell that cannot be
/// halved dropped from the next round's scan (the lines' cells part and
/// both carry); a node's box halved in refinement (its halves clear); a
/// cell that cannot be halved counted as one that can (the second case
/// runs a round a box to the work bound).
#[test]
fn two_arcs_ending_at_different_nodes_whose_boxes_meet_carry_neither() {
    let r = 0.09;
    let net = flat();
    let p = flat_rod(r);
    let form = ImplicitForm::of(&p).expect("a cylinder has a form");
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    for (rung, escalate, h, refines) in [(0.06, 0.038, 0.058, true), (0.08, 0.036, 0.055, false)] {
        let bd = band_escalating(escalate);
        let a = line(0.5 - r, 0.0, &[0.5, 0.9], rung, [None, Some((h, h))]);
        let b = line(0.5 + r, 0.0, &[0.5, 0.9], rung, [None, Some((h, h))]);
        let read = |ends: [usize; 2]| {
            let mut arcs = vec![
                Listed::arc(Enclosure::first(
                    &a,
                    &net,
                    partner,
                    [None, Some(ends[0])],
                    bd,
                )),
                Listed::arc(Enclosure::first(
                    &b,
                    &net,
                    partner,
                    [None, Some(ends[1])],
                    bd,
                )),
            ];
            let before = rects(&arcs);
            let meets = apart(&mut arcs, bd);
            (meets, before != rects(&arcs))
        };
        let (meets, refined) = read([0, 1]);
        assert_eq!(
            refined, refines,
            "rung {rung}: whether the lines' own cells met and were refined"
        );
        assert_eq!(
            stops(&meets),
            [PadsStop::Stall; 2],
            "rung {rung}: boxes of two nodes that meet cannot be halved apart"
        );
        let (meets, _) = read([0, 0]);
        assert!(
            meets.iter().all(Option::is_none),
            "rung {rung}: the same boxes at one node both arcs end at are where the arcs \
             meet: {meets:?}"
        );
    }
}

/// **A window that holds no cell is a desync**: a hand-built line on the
/// flat wall at `v = 0.2`, where the rod about `v = ½` of radius `0.09`
/// passes nowhere near. Every cell of its tube is cleared, though limb 3
/// puts a piece of the arc in each window, so the arc carries a desync.
///
/// Red against: an arc carried on no cells.
#[test]
fn a_window_that_holds_no_cell_is_a_desync() {
    let net = flat();
    let p = flat_rod(0.09);
    let form = ImplicitForm::of(&p).expect("a cylinder has a form");
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let b = line(0.2, 0.0, &[], 0.02, [None, None]);
    let arc = Enclosure::first(&b, &net, partner, [None, None], band()).expect("the tube reads");
    assert!(arc.cells.is_empty(), "the rod's form clears every cell");
    assert!(
        matches!(arc.carried(&b, open()), Err(CarryError::Desync(_))),
        "an arc with a window of no cells is a desync"
    );
}

/// **Two arcs 1.1, 1.3 and 1.6 rungs apart are carried, each point on
/// its own arc alone** (the item's done-when), at rungs `0.02` and
/// `0.005`, at the run's band: on the flat wall, a rod slanted `0.05` in
/// the wall, its axis over the wall so that it cuts it in two parallel
/// lines the stated gap apart, each a hand-built branch of spans half a
/// rung long. Forty-one points of each line read on their own arc and
/// not on the other.
///
/// Red against: arcs held apart by their pads about their carriers, a
/// rung and more each (every gap refuses); a site read on an arc by its
/// pad alone.
#[test]
fn two_arcs_a_rung_and_a_tenth_apart_are_carried_each_point_on_its_own() {
    let net = flat();
    let slope = 0.05f64;
    let across = slope.hypot(1.0);
    for rung in [0.02, 0.005] {
        for gap_rungs in [1.1, 1.3, 1.6] {
            // The lines stand `half` either side of `v = ½ + slope·u`
            // along `v`, so `half / across` square to themselves: the
            // rod's radius is that over 0.92, its axis as far over the
            // wall as leaves that chord.
            let half = 0.5 * gap_rungs * rung * across;
            let (chord, radius) = (half / across, half / across / 0.92);
            let p = Surface::Cylinder {
                frame: geom::Frame::new(
                    pt(0.0, 0.5, (radius * radius - chord * chord).sqrt()),
                    Vec3::new(1.0, slope, 0.0).normalize(),
                    Vec3::unit_z(),
                ),
                radius,
            };
            let form = ImplicitForm::of(&p).expect("a cylinder has a form");
            let partner = Bracketed {
                surface: &p,
                form: &form,
            };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let spans = (2.0 / rung).ceil() as u32;
            let inner: Vec<f64> = (1..spans)
                .map(|i| f64::from(i) / f64::from(spans))
                .collect();
            let ys = [0.5 - half, 0.5 + half];
            let lines = ys.map(|y| line(y, slope, &inner, rung, [None, None]));
            let mut arcs: Vec<Listed<'_>> = lines
                .iter()
                .map(|b| Listed::arc(Enclosure::first(b, &net, partner, [None, None], band())))
                .collect();
            let what = format!("rung {rung}, {gap_rungs} rungs apart");
            let meets = apart(&mut arcs, band());
            assert!(
                meets.iter().all(Option::is_none),
                "{what}: the arcs are carried: {meets:?}"
            );
            let comps = [carried(&arcs[0], &lines[0]), carried(&arcs[1], &lines[1])];
            for (k, y) in ys.into_iter().enumerate() {
                for i in 0..=40 {
                    let u = f64::from(i) / 40.0;
                    let q: Point3<f64> = pt(u, y + slope * u, 0.0);
                    let on = |c: &CarriedComponent<f64>| {
                        c.reads(q, band()).expect("the point reads").is_some()
                    };
                    assert!(on(&comps[k]), "{what}: {q:?} reads on its own arc");
                    assert!(
                        !on(&comps[1 - k]),
                        "{what}: {q:?} reads on its own arc alone"
                    );
                }
            }
        }
    }
}

/// **A bending one-span arc's pad stands within two rungs**: on the
/// flat wall `S(u, v) = (u, v, 0)`, the circle about `(½, −½)` of radius
/// `0.9` crosses from the side `u = 0` to the side `u = 1`, rising
/// `0.15` between. Its pcurve is one rational span, so its tube is one
/// window over the arc's whole bend, nineteen rungs across at a rung of
/// `0.02`. The cells the cylinder's form does not clear lie along the
/// arc, so the pad stands within two rungs, and each point of the circle
/// reads on the arc. The window overhangs the sides `u = 0` and `u = 1`
/// by its pad, and is cut to the knot rectangle first, so no cell is a
/// sliver of a side. Each cell's pad is no less than the carrier's
/// limb-2 bound, shown on the same arc with a bound of a metre.
///
/// Red against: a window's cells kept whether or not the partner's form
/// clears them (a pad of the window's width, some seventeen rungs); a
/// window halved as it stands past the rectangle (cells of no width on
/// the sides); a cell's pad left under the limb-2 bound.
#[test]
fn a_bending_one_span_arcs_pad_stands_within_two_rungs() {
    let net = flat();
    let (radius, rung) = (0.9f64, 0.02);
    let p = Surface::Cylinder {
        frame: geom::Frame::new(pt(0.5, -0.5, 0.0), Vec3::unit_z(), Vec3::unit_x()),
        radius,
    };
    // The arc as one rational quadratic span: its ends on the sides, its
    // middle control point where their tangents meet, weighted by the
    // cosine of the half angle.
    let rise = (radius * radius - 0.25).sqrt();
    let cos = rise / radius;
    let ctrl = [
        (0.0, rise - 0.5),
        (0.5, radius / cos - 0.5),
        (1.0, rise - 0.5),
    ];
    let arc = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let weights = vec![1.0, cos, 1.0];
    let branch = |hull_sup: f64| {
        hand_built(
            NurbsCurve3::new(
                arc(),
                ctrl.iter().map(|&(u, v)| pt(u, v, 0.0)).collect(),
                weights.clone(),
            )
            .unwrap(),
            NurbsCurve2::new(
                arc(),
                ctrl.iter().map(|&(u, v)| Point2::new(u, v)).collect(),
                weights.clone(),
            )
            .unwrap(),
            rung,
            [None, None],
            hull_sup,
        )
    };
    let form = ImplicitForm::of(&p).expect("a cylinder has a form");
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let read = |branch: &SsiBranch| {
        let first = Enclosure::first(branch, &net, partner, [None, None], band());
        let mut arcs = vec![Listed::arc(first)];
        assert!(apart(&mut arcs, band()).iter().all(Option::is_none));
        carried(&arcs[0], branch)
    };
    let comp = read(&branch(1e-12));
    assert!(
        comp.pad <= 2.0 * rung,
        "the pad {} stands within two rungs ({rung}) of an arc that bends 0.15",
        comp.pad
    );
    assert!(
        comp.boxes.iter().all(|b| b.hi[0] - b.lo[0] > 1e-3 * rung),
        "every cell has width on the wall: none is a sliver of a side"
    );
    let loose = read(&branch(1.0));
    assert!(
        loose.pad >= 1.0 && loose.boxes.iter().all(|b| b.pad >= 1.0),
        "a cell's pad holds the carrier's limb-2 bound"
    );
    for i in 0..=40 {
        let u = f64::from(i) / 40.0;
        let q: Point3<f64> = pt(u, (radius * radius - (u - 0.5).powi(2)).sqrt() - 0.5, 0.0);
        assert!(
            comp.reads(q, band()).expect("the point reads").is_some(),
            "the circle's point {q:?} reads on the arc"
        );
    }
}
