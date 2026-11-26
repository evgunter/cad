//! **An interior loop, cut in.**
//!
//! The section certificate refuses a pair whose section has a component
//! inside both faces with no event on it (R-loop): the join cuts only
//! along what the crossing layer saw, so the loop would go unbuilt. On
//! the crossings path the interior-loop guard ([`super::guard_refusal`])
//! raises it, and on the no-crossings path the section pass
//! ([`super::section_extent_pass`]). Where a face of the pair is on a
//! sphere, the loop is the no-crossings path's trimmed escape in another
//! place: the sphere face is cut along a meridian of its own chart the
//! loop crosses ([`super::SphereCutIn`], [`super::apply_cut_ins`]), and
//! the pipeline re-enters once. The re-entered crossing layer meets the
//! loop where it crosses the cut ([`super::Pass::CutIn`]), and the
//! output stage closes the cut ([`super::super::slit`], closing a cut).
//!
//! Against a plane or a sphere the loop is the carriers' one circle.
//! Against a torus or a cylinder it is the march's one loop, and the cut
//! runs along the meridian through the sum of its carrier's net's points
//! about the sphere's centre, where the cut-in reads the loop off its
//! tube ([`super::CutSection::Loop`]). A ball's pair with a cone is
//! refused at the operand gate and never reaches here. A pair with no
//! sphere face (two tori, a torus and a cylinder) takes no cut: a torus,
//! cylinder or cone face is not simply connected, so a cut of it need not
//! glue back, and the refusal stands.
//!
//! A refusing pair of any other kind, or of any other refusal, keeps its
//! refusal for the whole op, and so does a cut-in that cannot be made:
//! the pipeline goes on, and the refusal is raised as it would have
//! been. A decision of the cut-in's own that its band cannot read
//! escalates as itself.

use std::collections::BTreeSet;

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use super::super::section_cert::Refusal as SectionRefusal;
use super::super::section_table::SectionTable;
use super::super::{BooleanDecision, SphereQuestion};
use super::{CutSection, PairVerdict, SphereCutIn};
use crate::body::Body;
use crate::entity::FaceKey;
use crate::props::AtRestPolicy;
use crate::validate::decide;
use crate::{BooleanError, Operand};

/// **The cut-ins the refusing pairs ask for**, or `None` where one of
/// them is not an R-loop with a sphere face, or its loop is not read:
/// the refusal then stands. Two spheres cut both faces, so the circle
/// lands as chords on both, as the extent scan cuts a crossing pair.
///
/// # Errors
///
/// None of its own; a sphere pair's circle refuses as the extent scan's
/// does ([`super::sphere_pair_cut`]).
pub(super) fn cut_ins<T: Decide + AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    pairs: &[PairVerdict],
    sections: &SectionTable<T>,
    band: Band,
) -> Result<Option<Vec<SphereCutIn<T>>>, BooleanError> {
    let mut out = Vec::new();
    for p in pairs {
        match p.verdict {
            Ok(_) => continue,
            Err(SectionRefusal::Loop) => {}
            Err(_) => return Ok(None),
        }
        let sa = a.proven_face(p.a_face).expect_described();
        let sb = b.proven_face(p.b_face).expect_described();
        let mut any = false;
        for (operand, face, x, y) in [
            (Operand::A, p.a_face, sa, sb),
            (Operand::B, p.b_face, sb, sa),
        ] {
            if !matches!(x, geom::Surface::Sphere { .. }) {
                continue;
            }
            let Some(cut) = cut_in((operand, face), x, y, sections, band)? else {
                return Ok(None);
            };
            out.push(cut);
            any = true;
        }
        if !any {
            return Ok(None);
        }
    }
    Ok((!out.is_empty()).then_some(out))
}

/// The cut-in of `operand`'s `face` on the sphere `x` across its section
/// with `y`; `None` for a plane the ball is not read crossing, or a loop
/// [`loop_cut_in`] does not read.
fn cut_in<T: Decide + AtRestPolicy>(
    (operand, face): (Operand, FaceKey),
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    sections: &SectionTable<T>,
    band: Band,
) -> Result<Option<SphereCutIn<T>>, BooleanError> {
    let &geom::Surface::Sphere { frame, radius } = x else {
        return Ok(None);
    };
    let center = frame.origin();
    Ok(match y {
        &geom::Surface::Plane { frame: plane } => {
            super::plane_cut_of_sphere((operand, face), center, radius, plane, band)?
        }
        geom::Surface::Sphere { .. } => Some(super::sphere_pair_cut((operand, face), x, y, band)?),
        // A cone's pair with a ball never reaches here: the operand gate
        // refuses it (`GermFrameUnsupported`), so its loop is not read.
        geom::Surface::Torus { .. } | geom::Surface::Cylinder { .. } => {
            loop_cut_in((operand, face), (center, radius), x, y, sections, band)?
        }
        _ => None,
    })
}

/// **The cut-in across the march's one loop** of the sphere `x` (centre
/// and radius) with the curved `y`: its meridian through the sum of the
/// loop's carrier's net's points about the centre, and the tube
/// [`super::apply_cut_ins`] reads the loop by. `None` where the section
/// is not one marched loop with a net and a tube, or the section or its
/// march refuses it: the pair's refusal then stands.
///
/// # Errors
///
/// A reading of the section or its march in band escalates as the
/// cut-in's own ([`SphereQuestion::CutIn`]); a section asked in the wrong
/// lane or with no region, and a march that desyncs, are kernel faults.
fn loop_cut_in<T: Decide + AtRestPolicy>(
    (operand, face): (Operand, FaceKey),
    (center, radius): (geom_core::Point3<T>, T),
    x: &geom::Surface<T>,
    y: &geom::Surface<T>,
    sections: &SectionTable<T>,
    band: Band,
) -> Result<Option<SphereCutIn<T>>, BooleanError> {
    let esc = |diag| BooleanError::Escalated {
        decision: BooleanDecision::Sphere(SphereQuestion::CutIn),
        diag,
    };
    let section = match geom_brep::pair_section(
        x,
        y,
        sections.region_for(x, y),
        None::<geom_core::NoLane>,
        band,
    ) {
        Ok(geom_brep::SectionRead::Built(section)) => section,
        Err(
            geom_brep::SectionError::Escalated(diag)
            | geom_brep::SectionError::RadiusEscalated { diag, .. },
        ) => return Err(esc(diag)),
        // The cut-in asks the pair it was handed, over the operation's
        // region: a lane or region it was not handed is its own defect.
        Err(geom_brep::SectionError::WrongLane { .. }) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "cut-in: the loop's section was asked in a lane not the pair's",
            });
        }
        Err(geom_brep::SectionError::NoRegion { .. }) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "cut-in: the loop's section was asked with no region to read it over",
            });
        }
        Ok(geom_brep::SectionRead::Elsewhere(_)) | Err(_) => return Ok(None),
    };
    let carried = match section.carry(T::ssi_lane(), band) {
        Ok(carried) => carried,
        Err(geom_brep::CarryError::Escalated(diag)) => return Err(esc(diag)),
        Err(geom_brep::CarryError::Desync(what)) => {
            return Err(BooleanError::ClassificationInvariant { what });
        }
        Err(geom_brep::CarryError::Refused(_)) => return Ok(None),
    };
    let [component] = carried.components() else {
        return Ok(None);
    };
    let Some(net) = component
        .carrier
        .whole_net()
        .map(geom::NurbsCurve3::control)
    else {
        return Ok(None);
    };
    if component.runs.is_empty() || net.is_empty() || !component.closed() {
        return Ok(None);
    }
    let toward = net
        .iter()
        .fold(geom_core::Vec3::zero(), |acc, &q| acc + (q - center));
    Ok(Some(SphereCutIn {
        operand,
        face,
        center,
        radius,
        section: CutSection::Loop {
            toward,
            tube: component.runs.iter().map(|r| (r.lo, r.hi)).collect(),
        },
    }))
}

/// **Whether the output stage closes every cut these crossings-path
/// cut-ins make** ([`super::super::slit::close_cuts`]), decided before
/// the op re-enters on them. The closing rests on what it reads of a cut
/// face, and each premise is checked here:
/// - **no join chord runs along a cut** ([`super::cuts_in_result`]): no
///   first-pass section on a cut face lies in the cut's meridian plane.
///   A section that crosses the cut does so at points, which the
///   re-entered op's join reads. A first-pass section is read where the
///   crossing layer met the cut face (`events`). Its partner's plane, or
///   the radical plane of a partner sphere, is read against the meridian
///   plane. A partner of any other kind is not read, so the premise is
///   not held;
/// - **the chart's pole lies beyond a circle** where a cut's piece parts
///   a face bounded by section curves alone (`slit::part_cut`,
///   `slit::pole_beyond`, which reads circles): every section circle on
///   a cut face (the cut-ins' and the first-pass partners') leaves both
///   of the sphere's chart poles decidedly on one side of its plane, and
///   a face with two or more cut-ins is cut across circles alone.
///
/// The premises read the operands and the first pass, the same in
/// either operand order. `false` where one fails: the op does not cut
/// in, and the guard's refusal stands.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a reading of the premises is in
/// band: the cut's meridian (`bool_sphere_cut_meridian`, as the cut-in
/// asks it), a section plane against it, or a pole's side of a circle's
/// plane (`bool_cut_close_pole_side`, as the closing asks it).
pub(super) fn closing_holds<T: Decide + AtRestPolicy>(
    (a, b): (&Body<T>, &Body<T>),
    cut_ins: &[SphereCutIn<T>],
    events: &BTreeSet<(FaceKey, FaceKey)>,
    band: Band,
) -> Result<bool, BooleanError> {
    let esc = |diag| BooleanError::Escalated {
        decision: BooleanDecision::Sphere(SphereQuestion::CutIn),
        diag,
    };
    let sign = |name: &'static str, m: T| decide(name, Margin::of(m), band).map_err(esc);
    for cut in cut_ins {
        let (holder, partner) = match cut.operand {
            Operand::A => (a, b),
            Operand::B => (b, a),
        };
        let &geom::Surface::Sphere { frame, radius } =
            holder.proven_face(cut.face).expect_described()
        else {
            return Err(BooleanError::ClassificationInvariant {
                what: "cut-in: the cut face is not on a sphere",
            });
        };
        let (center, axis) = (frame.origin(), frame.axis());
        let on_face: Vec<&SphereCutIn<T>> = cut_ins
            .iter()
            .filter(|c| c.operand == cut.operand && c.face == cut.face)
            .collect();
        // The cut's meridian plane, on its normal `k` (as `apply_cut_ins`
        // chooses it); a foot on the axis is not read here.
        let k = match cut.section {
            CutSection::Circle { foot, .. } => axis.cross(foot - center),
            CutSection::Loop { toward, .. } => axis.cross(toward),
        };
        if sign("bool_sphere_cut_meridian", k.norm())? != Sign::Positive {
            return Ok(false);
        }
        let k = k.normalize();
        // Every section circle on the face, by a point of its plane and
        // its normal: the cut-ins' own, then each first-pass partner's.
        let mut circles: Vec<(Point3<T>, Vec3<T>)> = Vec::new();
        for c in &on_face {
            match c.section {
                CutSection::Circle { foot, normal, .. } => circles.push((foot, normal)),
                CutSection::Loop { .. } if on_face.len() > 1 => return Ok(false),
                CutSection::Loop { .. } => {}
            }
        }
        for &(fa, fb) in events {
            let other = match cut.operand {
                Operand::A if fa == cut.face => fb,
                Operand::B if fb == cut.face => fa,
                _ => continue,
            };
            let (at, normal) = match *partner.proven_face(other).expect_described() {
                geom::Surface::Plane { frame: plane } => (plane.origin(), plane.axis()),
                geom::Surface::Sphere {
                    frame: partner_frame,
                    radius: partner_radius,
                } => {
                    // The radical plane, square to the centre line.
                    let line = partner_frame.origin() - center;
                    let d2 = line.dot(line);
                    let t = (d2 + radius.powi(2) - partner_radius.powi(2)) / (d2 + d2);
                    (center + line * t, line)
                }
                _ => return Ok(false),
            };
            let n = normal.normalize();
            let along = sign("bool_cut_meridian_section_parallel", n.cross(k).norm())?
                == Sign::Zero
                && sign("bool_cut_meridian_section_offset", n.dot(center - at))? == Sign::Zero;
            if along {
                return Ok(false);
            }
            circles.push((at, n));
        }
        for (at, normal) in circles {
            for pole in [center + axis * radius, center - axis * radius] {
                if sign("bool_cut_close_pole_side", normal.dot(pole - at))? == Sign::Zero {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}

/// Each piece of a loop's tube ([`CutSection::Loop`]) by its eight
/// corners: bounds the march computed, entered as such.
pub(super) fn tube_corners<T: Real>(tube: &[([f64; 3], [f64; 3])]) -> Vec<[Point3<T>; 8]> {
    tube.iter()
        .map(|&(lo, hi)| {
            core::array::from_fn(|corner| {
                let pick =
                    |i: usize| T::from_computed(if (corner >> i) & 1 == 0 { lo[i] } else { hi[i] });
                Point3::new(pick(0), pick(1), pick(2))
            })
        })
        .collect()
}

/// **The side every point of `points` takes**, read by `read`: `Zero`
/// where two decide different sides, or where `points` is empty. Every
/// point is needed, so the first reading that refuses is the answer's.
fn common_side<T: Real, E>(
    points: &[Point3<T>],
    mut read: impl FnMut(Point3<T>) -> Result<Sign, E>,
) -> Result<Sign, E> {
    let Some((&first, rest)) = points.split_first() else {
        return Ok(Sign::Zero);
    };
    let first = read(first)?;
    for &q in rest {
        if read(q)? != first {
            return Ok(Sign::Zero);
        }
    }
    Ok(first)
}

/// Why a loop's tube does not place a cut-in ([`read_loop_tube`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LoopTubeRefusal {
    /// A piece of the tube is not read wholly inside the cut's half-plane.
    OffHalf,
    /// No piece is read wholly on one side of the meridian's plane.
    NoCrossing,
}

/// **Whether a loop's tube places a cut-in** along the half-meridian of
/// the sphere about `centre` on `h`'s side of the meridian plane normal to
/// `k`: every piece (`pieces`, each by its corners) in that half-plane
/// holds the loop there, off the poles, and a piece wholly on each side
/// of the plane puts a point of the loop on that side, so the loop
/// crosses the half-meridian.
///
/// # Errors
///
/// A corner of a piece in the half-plane read in band; the crossing
/// search's kept reading where no piece answers ([`some_piece_wholly`]).
pub(super) fn read_loop_tube<T: Decide>(
    pieces: &[[Point3<T>; 8]],
    centre: Point3<T>,
    (h, k): (Vec3<T>, Vec3<T>),
    band: Band,
) -> Result<Result<(), LoopTubeRefusal>, Indeterminate> {
    let along =
        |by: Vec3<T>, name| move |q: Point3<T>| decide(name, Margin::of(by.dot(q - centre)), band);
    let half = along(h, "bool_sphere_cut_loop_half");
    for q in pieces {
        if common_side(q, half)? != Sign::Positive {
            return Ok(Err(LoopTubeRefusal::OffHalf));
        }
    }
    let side = along(k, "bool_sphere_cut_loop_side");
    Ok(
        if some_piece_wholly(pieces, Sign::Positive, side)?
            && some_piece_wholly(pieces, Sign::Negative, side)?
        {
            Ok(())
        } else {
            Err(LoopTubeRefusal::NoCrossing)
        },
    )
}

/// **A height on the half-meridian against a loop's tube**, by every
/// corner of it (`corners`) along `axis` from `centre`: `Negative` below
/// the whole tube, `Positive` above it, `Zero` within its heights.
///
/// # Errors
///
/// A corner read in band.
pub(super) fn height_against_tube<T: Decide>(
    corners: &[Point3<T>],
    (centre, axis): (Point3<T>, Vec3<T>),
    height: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    common_side(corners, |q| {
        decide(
            "bool_sphere_cut_loop_height",
            Margin::of(height - axis.dot(q - centre)),
            band,
        )
    })
}

/// **Whether some piece of a tube lies wholly on `side`**, each corner
/// read by `read`: a search over the pieces through the ladder
/// ([`geom_core::ladder::climb`]), a piece whose corner `read` does not
/// decide kept, and refused only where no piece answers.
///
/// # Errors
///
/// The kept reading, where no piece answers and one was undecided.
fn some_piece_wholly<T: Real>(
    pieces: &[[Point3<T>; 8]],
    side: Sign,
    read: impl Fn(Point3<T>) -> Result<Sign, Indeterminate>,
) -> Result<bool, Indeterminate> {
    use geom_core::ladder::{Answering, Rung, climb};
    let piece = |corners: &[Point3<T>; 8]| match common_side(corners, &read) {
        Ok(s) if s == side => Rung::Answers(()),
        Ok(_) => Rung::Passes,
        Err(diag) => Rung::InBand(diag),
    };
    climb(Answering::First, pieces, piece)
        .into_reading()
        .map(|answer| answer.is_some())
}

#[cfg(test)]
mod loop_tube_rows {
    #![allow(clippy::unwrap_used)]
    use super::{LoopTubeRefusal, height_against_tube, read_loop_tube, tube_corners};
    use geom_core::{Band, Point3, Sign, Tol, Vec3};

    /// A tube by its pieces' corners.
    type Tube = [([f64; 3], [f64; 3])];

    /// The reading of the cut on the half-meridian through `+x` of the
    /// sphere about the origin poled on `z`: the half-plane `x > 0`, the
    /// meridian plane `y = 0`.
    fn read(tube: &Tube) -> Result<(), LoopTubeRefusal> {
        let band = Band::linear(Tol::witness()).unwrap();
        let (h, k) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        read_loop_tube(&tube_corners(tube), Point3::origin(), (h, k), band).unwrap()
    }

    /// `height` along `z` against `tube`.
    fn place(tube: &Tube, height: f64) -> Sign {
        let band = Band::linear(Tol::witness()).unwrap();
        let corners: Vec<Point3<f64>> = tube_corners(tube).into_iter().flatten().collect();
        let along = (Point3::origin(), Vec3::new(0.0, 0.0, 1.0));
        height_against_tube(&corners, along, height, band).unwrap()
    }

    /// **Each of the loop's three readings refuses the tube it should**:
    /// a piece off the cut's half-plane, no piece wholly on one side of
    /// the meridian's plane (a straddling piece is no such piece), and a
    /// boundary hit within the tube's heights. A tube that passes the
    /// first two places hits below and above it.
    #[test]
    fn each_loop_tube_reading_refuses_its_own_tube() {
        let below = ([0.4, -0.1, 0.1], [0.6, -0.02, 0.3]);
        let above = ([0.4, 0.02, 0.1], [0.6, 0.1, 0.3]);
        let crosses = [below, above];
        assert_eq!(
            read(&crosses),
            Ok(()),
            "a tube in the half-plane, across the plane"
        );
        assert_eq!(
            read(&[below, ([-0.1, 0.02, 0.1], [0.6, 0.1, 0.3])]),
            Err(LoopTubeRefusal::OffHalf),
            "a piece across the axis's plane"
        );
        assert_eq!(
            read(&[above, ([0.4, 0.2, 0.1], [0.6, 0.3, 0.3])]),
            Err(LoopTubeRefusal::NoCrossing),
            "every piece on the positive side"
        );
        assert_eq!(
            read(&[below, ([0.4, -0.3, 0.1], [0.6, -0.2, 0.3])]),
            Err(LoopTubeRefusal::NoCrossing),
            "every piece on the negative side"
        );
        assert_eq!(
            read(&[above, ([0.4, -0.1, 0.1], [0.6, 0.1, 0.3])]),
            Err(LoopTubeRefusal::NoCrossing),
            "a piece straddling the plane is on neither side"
        );
        assert_eq!(place(&crosses, 0.0), Sign::Negative, "a hit below the tube");
        assert_eq!(place(&crosses, 0.5), Sign::Positive, "a hit above the tube");
        assert_eq!(
            place(&crosses, 0.2),
            Sign::Zero,
            "a hit within the tube's heights"
        );
    }
}
