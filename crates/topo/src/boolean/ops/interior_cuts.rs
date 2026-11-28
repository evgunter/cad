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
//! tube ([`super::CutSection::Loop`]). A ball's pair with a cone takes
//! no cut-in, and its pair's refusal stands. A pair with no
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
        geom::Surface::Torus { .. } | geom::Surface::Cylinder { .. } => {
            loop_cut_in((operand, face), (center, radius), x, y, sections, band)?
        }
        // A cone's pair with a ball reaches here, and no cut-in is built
        // for it: the pair's refusal stands
        // (work/lathe/a-cone-and-a-one-band-ball-meet-in-a-loop-inside-both-faces-and-refuse-its-cut-in.md,
        // `a_cone_with_a_ball_or_a_cone_in_every_order`).
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
        meridian: None,
    }))
}

/// The meridian planes a crossings-path circle cut-in may run in, as
/// the tangent of their turn about the sphere's axis from the plane
/// through the circle's centre direction, tried in this order
/// ([`closing_meridians`]). Each is a plane through the axis, so each
/// meets the circle in a chord where it meets it at all.
fn meridian_turns<T: Real>() -> [T; 9] {
    [
        T::from_f64(0.0),
        T::from_f64(0.125),
        T::from_f64(-0.125),
        T::from_f64(0.25),
        T::from_f64(-0.25),
        T::from_f64(0.375),
        T::from_f64(-0.375),
        T::from_f64(0.5),
        T::from_f64(-0.5),
    ]
}

/// **The cut-ins with the meridians the output stage's closing holds
/// on** ([`super::super::slit::close_cuts`]), chosen before the op
/// re-enters on them, or `None` where none is held: the op then does not
/// cut in, and the guard's refusal stands. The closing reads no join
/// chord along a cut ([`super::cuts_in_result`]), so each cut's meridian
/// plane is chosen off every first-pass section on its face:
///
/// - **a circle's cut** runs in the first plane of [`meridian_turns`]
///   that meets the circle decidedly twice, leaves both crossings
///   decidedly on the cut's half-plane ([`meridian_chord`]), and is
///   decidedly off every first-pass section's plane on the face. A
///   first-pass section is read where the crossing layer met the cut face
///   (`events`): a partner plane's own plane, or a partner sphere's
///   radical plane. Any other partner on a cut face (a cylinder, a cone, a
///   torus) is not read, and the op does not cut in. Where no turn clears
///   and one was read in band, the op escalates as that turn's reading;
///   only where every turn is decidedly blocked does it go on uncut;
/// - **a loop's cut** runs in its own meridian (through its net's sum,
///   [`loop_cut_in`]), off every first-pass section's plane on the face
///   as above, or the op does not cut in.
///
/// Another cut-in's circle on the face is no section to turn off: a
/// meridian plane holds the axis, so a cut-in circle in one is a great
/// circle through both poles, and a candidate meets the cut's own circle
/// twice on the sphere, so on that great circle. Two cut-in circles on one
/// face are disjoint sections of the partner's faces, so no candidate
/// lies in another's plane.
///
/// Where a cut's piece parts a face bounded by section curves alone, the
/// closing decides which part keeps the outline itself
/// ([`super::super::slit::close_cuts`]): where it cannot, the re-entry
/// answers [`BooleanError::CutOutlineUndecided`] and the op goes on
/// uncut ([`super::boolean_pipeline`]'s crossings path).
///
/// The meridians read the operands and the first pass, the same in
/// either operand order.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where a circle's own meridian is read in
/// band, where no turn clears and one is read in band, and where a loop's
/// meridian, or a section plane against it, is read in band: a loop takes
/// no other meridian.
pub(super) fn closing_meridians<T: Decide + AtRestPolicy>(
    (a, b): (&Body<T>, &Body<T>),
    cut_ins: Vec<SphereCutIn<T>>,
    events: &BTreeSet<(FaceKey, FaceKey)>,
    band: Band,
) -> Result<Option<Vec<SphereCutIn<T>>>, BooleanError> {
    let esc = |diag| BooleanError::Escalated {
        decision: BooleanDecision::Sphere(SphereQuestion::CutIn),
        diag,
    };
    let sign = |name: &'static str, m: T| decide(name, Margin::of(m), band);
    let mut out = Vec::with_capacity(cut_ins.len());
    for mut cut in cut_ins {
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
        let (center, axis) = (frame.origin(), frame.axis().normalize());
        // Each first-pass section's plane on the face, by a point of it
        // and its unit normal.
        let mut planes: Vec<(Point3<T>, Vec3<T>)> = Vec::new();
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
                _ => return Ok(None),
            };
            planes.push((at, normal.normalize()));
        }
        // Whether the meridian plane on the unit normal `k` is decidedly
        // off every section plane: crossing it (`|n × k|` read apart from
        // zero), or parallel to it and decidedly off it.
        let off_sections = |k: Vec3<T>| -> Result<bool, Indeterminate> {
            for &(at, n) in &planes {
                if sign("bool_cut_meridian_section_parallel", n.cross(k).norm())? == Sign::Zero
                    && sign("bool_cut_meridian_section_offset", n.dot(center - at))? == Sign::Zero
                {
                    return Ok(false);
                }
            }
            Ok(true)
        };
        match cut.section {
            CutSection::Circle {
                foot,
                rho,
                normal,
                u_ref,
            } => {
                let Some(k0) =
                    circle_meridian((center, axis), (foot, rho, u_ref), sign).map_err(esc)?
                else {
                    return Ok(None);
                };
                // The first turn that clears decidedly. Where none does, a
                // turn read in band escalates as itself: the cut is not
                // decided blocked there. Only where every turn is decidedly
                // blocked does the op go on uncut, to the guard's refusal.
                let mut in_band = None;
                let mut chosen = None;
                for t in meridian_turns::<T>() {
                    let k = k0 + axis.cross(k0) * t;
                    match meridian_reads((center, axis), (foot, rho, normal), k, &off_sections, sign)
                    {
                        Ok(Some(k)) => {
                            chosen = Some(k);
                            break;
                        }
                        Ok(None) => {}
                        Err(diag) => {
                            in_band.get_or_insert(diag);
                        }
                    }
                }
                match (chosen, in_band) {
                    (Some(k), _) => cut.meridian = Some(k),
                    (None, Some(diag)) => return Err(esc(diag)),
                    (None, None) => return Ok(None),
                }
            }
            CutSection::Loop { toward, .. } => {
                let k = axis.cross(toward);
                if sign("bool_sphere_cut_meridian", k.norm()).map_err(esc)? != Sign::Positive {
                    return Ok(None);
                }
                if !off_sections(k.normalize()).map_err(esc)? {
                    return Ok(None);
                }
            }
        }
        out.push(cut);
    }
    Ok(Some(out))
}

/// **The meridian plane on the normal `k` for a circle's cut**, `k`
/// itself where it is decidedly a direction, its chord of the circle
/// (`foot`, `rho`, `normal`) is read ([`meridian_chord`]), and it is
/// decidedly off every section plane (`off_sections`); `None` where one
/// of them is decidedly not so, and the reading's refusal where one is in
/// band.
fn meridian_reads<T: Decide>(
    (center, axis): (Point3<T>, Vec3<T>),
    circle: (Point3<T>, T, Vec3<T>),
    k: Vec3<T>,
    off_sections: &impl Fn(Vec3<T>) -> Result<bool, Indeterminate>,
    sign: impl Fn(&'static str, T) -> Result<Sign, Indeterminate>,
) -> Result<Option<Vec3<T>>, Indeterminate> {
    if sign("bool_sphere_cut_meridian", k.norm())? != Sign::Positive {
        return Ok(None);
    }
    let unit = k.normalize();
    if meridian_chord((center, axis), circle, unit, false, &sign)?.is_err() {
        return Ok(None);
    }
    Ok(off_sections(unit)?.then_some(k))
}

/// **A circle cut's own meridian normal**: `axis × (foot − center)`, the
/// plane through the circle's centre direction, or, where the foot is
/// decidedly on the polar axis, the plane through the circle's point
/// `foot + u_ref·ρ`. `None` where that too is decidedly no direction. The
/// one reading of it, by [`closing_meridians`]' turns and
/// [`super::apply_cut_ins`].
pub(super) fn circle_meridian<T: Decide>(
    (center, axis): (Point3<T>, Vec3<T>),
    (foot, rho, u_ref): (Point3<T>, T, Vec3<T>),
    sign: impl Fn(&'static str, T) -> Result<Sign, Indeterminate>,
) -> Result<Option<Vec3<T>>, Indeterminate> {
    let k = axis.cross(foot - center);
    if sign("bool_sphere_cut_meridian", k.norm())? != Sign::Zero {
        return Ok(Some(k));
    }
    let k = axis.cross(foot + u_ref * rho - center);
    Ok((sign("bool_sphere_cut_meridian", k.norm())? != Sign::Zero).then_some(k))
}

/// Why a meridian plane's chord of a cut circle is not read
/// ([`meridian_chord`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ChordMiss {
    /// The circle lies in the meridian's plane, or decidedly may.
    InItsPlane,
    /// The plane is not read meeting the circle twice.
    NotTwice,
    /// A crossing is not read on the cut's half-plane `k × axis`: the
    /// circle holds a pole of the chart.
    OffHalf,
}

/// **The two points where the meridian plane on the unit normal `k`
/// crosses a cut circle** (`foot`, `rho`, `normal`) on the sphere about
/// `center`, each decidedly on the cut's half-plane `k × axis`: the one
/// reading of the cut's chord, by [`closing_meridians`]' turns and by
/// [`super::apply_cut_ins`], which builds the cut on it.
///
/// The chord lies across the circle along `e = (normal × k)/|normal × k|`,
/// at the foot's offset from the plane, `d`, taken back along the plane
/// normal's part in the circle's plane (`w`): its midpoint is
/// `foot − w·(d/|w|²)` and its half-length `√(ρ² − d²/|w|²)`.
/// `through_foot`: the plane is the circle's own meridian (`d = 0`), and
/// the chord is the diameter `foot ± ρ·e`, built as such.
pub(super) fn meridian_chord<T: Decide>(
    (center, axis): (Point3<T>, Vec3<T>),
    (foot, rho, normal): (Point3<T>, T, Vec3<T>),
    k: Vec3<T>,
    through_foot: bool,
    sign: impl Fn(&'static str, T) -> Result<Sign, Indeterminate>,
) -> Result<Result<[Point3<T>; 2], ChordMiss>, Indeterminate> {
    let across = normal.cross(k);
    if sign("bool_sphere_cut_section_meridian", across.norm())? == Sign::Zero {
        return Ok(Err(ChordMiss::InItsPlane));
    }
    let e = across.normalize();
    let crossings = if through_foot {
        [foot - e * rho, foot + e * rho]
    } else {
        let d = k.dot(foot - center);
        let n = normal.normalize();
        let w = k - n * n.dot(k);
        let w2 = w.dot(w);
        let half2 = rho.powi(2) - d.powi(2) / w2;
        if sign("bool_sphere_cut_meridian_crosses", half2 / rho)? != Sign::Positive {
            return Ok(Err(ChordMiss::NotTwice));
        }
        let (mid, half) = (foot - w * (d / w2), half2.sqrt());
        [mid - e * half, mid + e * half]
    };
    let h = k.cross(axis);
    for p in crossings {
        if sign("bool_sphere_cut_half", h.dot(p - center))? != Sign::Positive {
            return Ok(Err(ChordMiss::OffHalf));
        }
    }
    Ok(Ok(crossings))
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
