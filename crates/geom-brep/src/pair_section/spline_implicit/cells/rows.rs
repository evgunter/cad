//! **The cell rows**: an arc's cells read and refined at a band the row
//! names, on the carry rows' wall against a rod and on a flat wall
//! against a hand-built arc. Each row names the mutant that turns it
//! red.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, Tol, Vec3};

use super::super::carry_rows::{band, pt, rod, rod_arcs, wall};
use super::super::{Bracketed, SplineImplicitPose};
use super::{Arc, Enclosure, REFINE_CELLS, apart};
use crate::pair_section::{CarriedComponent, CarryRefusal, ComponentEnds, ComponentShape};
use crate::ssi::residual::ImplicitForm;
use crate::ssi::{SsiBranch, SsiTube};

/// The rod of radius `0.09` across the carry rows' wall, read whole at
/// the run's band: the pose, the rod's form, and its two branches, the
/// lower arc first. The arcs stand `0.18` apart.
fn rod_read() -> (
    Surface<f64>,
    SplineImplicitPose<f64>,
    ImplicitForm,
    Vec<SsiBranch>,
) {
    let (w, p) = (wall(1.0), rod(1.0, 0.09));
    let pose = SplineImplicitPose::of(&w, &p).expect("a spline wall and a cylinder");
    let reading = crate::ImplicitSection::of(&p, pose.net(), pose.domain(), band())
        .expect("the pair is read");
    let form = reading.form().clone();
    let mut branches = reading.all().expect("the pair reads whole").branches;
    assert_eq!(branches.len(), 2, "the rod cuts the wall in two arcs");
    branches.sort_by(|a, b| a.witness.z.total_cmp(&b.witness.z));
    (p, pose, form, branches)
}

/// The band `(eps, K·eps)` at the run's `K`.
fn band_at(eps: f64) -> Band {
    Band::linear_at(Tol::witness(), eps).expect("the row's scale forms a band")
}

fn open() -> ComponentShape {
    ComponentShape::Open {
        ends: ComponentEnds::default(),
    }
}

/// The arc's component, carried.
fn carried(arc: &Arc<'_>, b: &SsiBranch) -> CarriedComponent<f64> {
    let Some(Ok(cells)) = arc else {
        panic!("the arc's cells read");
    };
    cells.carried(b, open()).expect("the arc carries")
}

fn cell_count(arc: &Arc<'_>) -> usize {
    match arc {
        Some(Ok(cells)) => cells.cells.len(),
        _ => panic!("the arc's cells read"),
    }
}

/// **Two arcs that stand within the band of each other carry neither,
/// at the stop**: at a band whose escalation is `0.1`, the rod's two
/// arcs, `0.18` apart, lie within twice it of each other, and no cell
/// between them is cleared (the rod's wall is within `0.09` of every
/// point between). Halving cannot part them, so the refinement stops,
/// having halved no more cells than its work bound, and both arcs refuse
/// `PadsMeet` naming a gap within that reach.
///
/// Red against: a refinement with no stop (it does not return); one
/// that reports arcs apart when it stops.
#[test]
fn two_arcs_within_the_band_of_each_other_carry_neither_at_the_stop() {
    let (p, pose, form, branches) = rod_read();
    let wide = band_at(0.1 / Tol::witness().k());
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let first = |b| Some(Enclosure::first(b, pose.net(), partner, [None, None], wide));
    let mut arcs: Vec<Arc<'_>> = branches.iter().map(first).collect();
    let at_first: usize = arcs.iter().map(cell_count).sum();
    let meets = apart(&mut arcs, wide);
    for meet in &meets {
        let Some(CarryRefusal::PadsMeet { gap, .. }) = meet else {
            panic!("arcs within the band of each other refuse by name: {meets:?}");
        };
        assert!(
            *gap <= 2.0 * wide.escalate(),
            "the arcs' cells stand {gap} apart, within twice the band's escalation"
        );
    }
    let at_stop: usize = arcs.iter().map(cell_count).sum();
    assert!(
        at_first < at_stop && at_stop <= at_first + REFINE_CELLS,
        "the refinement halved cells ({at_first} to {at_stop}), no more than its work bound"
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
/// reads each of its points too.
///
/// Red against: arcs held apart on their first reading alone (both
/// refuse); a refinement that drops a half holding a point of the arc
/// (the point reads on no arc).
#[test]
fn two_arcs_are_refined_until_they_read_apart_and_a_point_reads_as_it_does_alone() {
    let (p, pose, form, branches) = rod_read();
    let tight = band_at(0.04 / Tol::witness().k());
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let first = |b| {
        Some(Enclosure::first(
            b,
            pose.net(),
            partner,
            [None, None],
            tight,
        ))
    };
    let mut alone: Vec<Arc<'_>> = vec![first(&branches[0])];
    assert!(
        apart(&mut alone, tight).iter().all(Option::is_none),
        "an arc alone stands apart"
    );
    let mut both: Vec<Arc<'_>> = branches.iter().map(first).collect();
    assert_eq!(
        cell_count(&both[0]),
        cell_count(&alone[0]),
        "an arc's first reading is its own, whoever its neighbours are"
    );
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
    let w = wall(1.0);
    for (k, points) in rod_arcs(&w, 1.0, 0.09).into_iter().enumerate() {
        for q in points {
            let on = |c: &CarriedComponent<f64>| c.reads(q, tight).expect("the point reads");
            assert!(on(&both[k]).is_some(), "{q:?} reads on its own arc, {k}");
            assert!(on(&both[1 - k]).is_none(), "{q:?} reads on arc {k} alone");
            if k == 0 {
                assert!(
                    on(&alone).is_some(),
                    "{q:?} reads on its arc with no neighbour"
                );
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
/// reads on the arc.
///
/// Red against: a window's cells kept whether or not the partner's form
/// clears them (a pad of the window's width, some seventeen rungs).
#[test]
fn a_bending_one_span_arcs_pad_stands_within_two_rungs() {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let net = NurbsSurface::new(
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
    .unwrap();
    let (radius, rung) = (0.9f64, 0.02);
    let p = Surface::Cylinder {
        origin: pt(0.5, -0.5, 0.0),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
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
    let carrier = NurbsCurve3::new(
        arc(),
        ctrl.iter().map(|&(u, v)| pt(u, v, 0.0)).collect(),
        weights.clone(),
    )
    .unwrap();
    let pcurve = NurbsCurve2::new(
        arc(),
        ctrl.iter().map(|&(u, v)| Point2::new(u, v)).collect(),
        weights,
    )
    .unwrap();
    let branch = SsiBranch {
        witness: carrier.eval(0.5),
        carrier: geom::Curve3::Nurbs(std::sync::Arc::new(carrier)),
        params: (0.0, 1.0),
        end: crate::ssi::BranchEnd::Closed,
        certificate: crate::ssi::certify::SsiCertificate {
            samples: 0,
            on_locus_max: 0.0,
            hull_sup: 1e-12,
            tube: SsiTube::Chart {
                rung,
                pad_u: rung,
                pad_v: rung,
                nodes: [None, None],
            },
            tube_transversality: 1.0,
            tube_boxes: 1,
        },
        pcurve_a: None,
        pcurve_b: Some(pcurve),
        march_tol: 1e-9,
    };
    let form = ImplicitForm::of(&p).expect("a cylinder has a form");
    let partner = Bracketed {
        surface: &p,
        form: &form,
    };
    let mut arcs: Vec<Arc<'_>> = vec![Some(Enclosure::first(
        &branch,
        &net,
        partner,
        [None, None],
        band(),
    ))];
    assert!(apart(&mut arcs, band()).iter().all(Option::is_none));
    let comp = carried(&arcs[0], &branch);
    assert!(
        comp.pad <= 2.0 * rung,
        "the pad {} stands within two rungs ({rung}) of an arc that bends 0.15",
        comp.pad
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
