//! **A spline wall against a cylinder, sphere or torus: stages 1 to 3**
//! (module docs of [`super`]), through the march door, at `f64`.
//!
//! - **The hull.** `φ = f∘S` is the composite of the partner's canonical
//!   form ([`Curved`]) with the wall's rational chart, so on each Bézier
//!   cell of the wall it is `N/D`: `N` the form written in the wall's
//!   homogeneous channels `(w·x, w·y, w·z, w)` in the partner's frame, a
//!   tensor-product polynomial, and `D` the power of the weight function
//!   of `N`'s degree, whose coefficients are positive. Their Bernstein
//!   coefficients ([`PatchSpans`], formed in certification arithmetic)
//!   bound them over the cell, so `φ` over the cell lies in `hull(N) /
//!   hull(D)`. A net whose every cell's floor in metres is decided of one
//!   sign is apart; one whose largest `|φ|` reads a ceiling in metres
//!   within the zero band lies on the partner
//!   ([`SectionError::CoincidentSurfaces`]); a ceiling in band
//!   escalates. Both metres are read over the net's hull in the
//!   partner's frame, which holds every point of the wall.
//! - **The door.** A side of the rectangle collapsed to a point
//!   ([`crate::chart_singularity::spline_collapsed_row`]) is that point,
//!   the row's control point: `f` there is decided clear of the band, of
//!   one sign, or the pose refuses, naming the side.
//! - **The pass and the sweep**: the wall's boundary pass against the
//!   form ([`ImplicitWall::pass`]) and the regularity sweep
//!   ([`super::sweep`]), as on the plane. A declined sweep is the
//!   general rung's: the whole reading over the rectangle that a plane's
//!   pose falls to is not built for a curved partner.

use geom::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::k_stats::{Magnitude, decide, decide_magnitude};
use geom_core::spline::TensorCoeffs;
use geom_core::spline::compose::CanonicalSurface;
use geom_core::spline::compose::patch::PatchSpans;
use geom_core::{Band, Bounds, Interval, Margin, Sign, Vec3};

use super::super::{ChartReach, Components, PairSection, Reached};
use super::sweep::{self, Phi, Sweep};
use super::{Hull, SplineImplicitPose, contact_reach};
use crate::intersect::SectionError;
use crate::ssi::residual::{Curved, ImplicitWall};

/// Why a declined sweep's pose refuses.
const UNBUILT: &str = "the regularity sweep does not decide the section loop-free, and the \
                       whole reading over the wall's knot rectangle that would list its \
                       components is not built for a curved partner";

/// **The pair's classification, the march door's arm for a curved
/// partner** (module docs).
///
/// # Errors
///
/// [`SectionError::DegenerateOperand`] for a partner the form does not
/// read ([`Curved::of`]) or a net whose composite does not certify; the
/// hull's [`SectionError::CoincidentSurfaces`] and
/// [`SectionError::Escalated`]; the door's
/// [`SectionError::Unaccounted`]; the pass's refusals, mapped
/// ([`super::super::whole_reading_refused`]); and
/// [`SectionError::RoutesToGeneralRung`] where the sweep declines.
pub(super) fn section(
    pose: SplineImplicitPose<f64>,
    band: Band,
) -> Result<PairSection<f64>, SectionError> {
    let curved =
        Curved::of(pose.partner()).map_err(|what| SectionError::DegenerateOperand { what })?;
    if hull(&curved, pose.net(), band)? == Hull::Apart {
        return Ok(pose.section(Components::Counted(Vec::new())));
    }
    door(&curved, pose.net(), band)?;
    let domain = pose.domain();
    let wall =
        ImplicitWall::of(pose.net(), domain, band).map_err(super::super::whole_reading_refused)?;
    let pass = wall
        .pass(&curved)
        .map_err(super::super::whole_reading_refused)?;
    let reach = pass
        .crossings
        .iter()
        .map(|c| ChartReach::side(c.at.side))
        .chain(pass.contacts.iter().map(contact_reach))
        .fold(ChartReach::default(), ChartReach::or);
    match sweep::sweep(
        wall.boxes(),
        Phi::Curved(&curved),
        wall.account_floor(),
        band,
        domain.extent,
    ) {
        Sweep::Declined(_) => Err(SectionError::RoutesToGeneralRung {
            pair: "a NURBS wall and a cylinder, sphere or torus",
            why: UNBUILT,
        }),
        Sweep::LoopFree => Ok(
            match Reached::new(pose.order(ChartReach::default(), reach)) {
                Some(reached) => pose.section(Components::Reaching(reached)),
                None => pose.section(Components::Counted(Vec::new())),
            },
        ),
    }
}

/// The net's control points in the partner's frame, each certified, and
/// their hull, which holds every point of the wall.
fn framed(
    curved: &Curved,
    net: &NurbsSurface<f64>,
) -> Result<(Vec<Vec3<Interval>>, Vec3<Interval>), SectionError> {
    let framed: Vec<Vec3<Interval>> = net
        .control()
        .iter()
        .map(|&p| curved.framed_point(p))
        .collect();
    let certified =
        |v: &Vec3<Interval>| v.x.is_certified() && v.y.is_certified() && v.z.is_certified();
    let Some(&first) = framed.first() else {
        return Err(SectionError::DegenerateOperand {
            what: "the spline wall's net has no control point",
        });
    };
    if !framed.iter().all(certified) {
        return Err(SectionError::DegenerateOperand {
            what: "a control point of the spline wall does not certify in the partner's frame",
        });
    }
    let hull = framed.iter().skip(1).fold(first, |h, v| {
        Vec3::new(
            Interval::hull(h.x, v.x),
            Interval::hull(h.y, v.y),
            Interval::hull(h.z, v.z),
        )
    });
    Ok((framed, hull))
}

/// **Stage 1, the hull** (module docs).
///
/// # Errors
///
/// [`SectionError::DegenerateOperand`] where the composite does not
/// certify; [`SectionError::CoincidentSurfaces`] for a net within the
/// zero band of the partner everywhere (D10); [`SectionError::Escalated`]
/// where its ceiling is in the band.
pub(super) fn hull(
    curved: &Curved,
    net: &NurbsSurface<f64>,
    band: Band,
) -> Result<Hull, SectionError> {
    let (framed, box_) = framed(curved, net)?;
    let (ku, kv) = (net.knots_u_f64_interim(), net.knots_v_f64_interim());
    let nv = kv.control_count();
    let weights = net.weights_f64_interim();
    let coord = |v: &Vec3<Interval>, c: usize| match c {
        0 => v.x,
        1 => v.y,
        _ => v.z,
    };
    let channel = |c: usize| {
        PatchSpans::decompose(
            &TensorCoeffs::from_fn(ku, kv, |i, j| {
                let k = i * nv + j;
                Interval::point(weights[k]) * coord(&framed[k], c)
            }),
            &[],
            &[],
        )
    };
    let (gx, gy, gz) = (channel(0), channel(1), channel(2));
    let gw = PatchSpans::decompose(
        &TensorCoeffs::from_fn(ku, kv, |i, j| Interval::point(weights[i * nv + j])),
        &[],
        &[],
    );
    let w2 = gw.mul(&gw);
    let radial = gx.mul(&gx).add(&gy.mul(&gy));
    let (num, den) = match curved.form() {
        CanonicalSurface::Cylinder { radius } => (radial.sub(&w2.scale(radius.sqr())), w2),
        CanonicalSurface::Sphere { radius } => {
            (radial.add(&gz.mul(&gz)).sub(&w2.scale(radius.sqr())), w2)
        }
        CanonicalSurface::Torus { major, minor } => {
            let shifted = radial
                .add(&gz.mul(&gz))
                .add(&w2.scale(major.sqr() - minor.sqr()));
            let four = Interval::point(4.0) * major.sqr();
            (
                shifted.mul(&shifted).sub(&radial.mul(&w2).scale(four)),
                w2.mul(&w2),
            )
        }
        CanonicalSurface::Plane | CanonicalSurface::Cone { .. } => {
            unreachable!("a curved partner is a cylinder, a sphere or a torus")
        }
    };
    let (nu, nvc) = num.cell_counts();
    let mut values = Vec::with_capacity(nu * nvc);
    for su in 0..nu {
        for sv in 0..nvc {
            let d = den.cell_hull(su, sv);
            let f = num.cell_hull(su, sv) / d;
            if !(d.is_certified() && d.lo() > 0.0 && f.is_certified()) {
                return Err(SectionError::DegenerateOperand {
                    what: "the composite of the partner's form and the spline wall does not \
                           certify over a cell of the wall",
                });
            }
            values.push(f);
        }
    }
    let sides: Vec<_> = values
        .iter()
        .map(|&f| {
            decide(
                "section_nurbs_implicit_hull",
                Margin::of(curved.floor_metres(f, box_)),
                band,
            )
            .ok()
        })
        .collect();
    let all = |s| !sides.is_empty() && sides.iter().all(|&t| t == Some(s));
    if all(Sign::Positive) || all(Sign::Negative) {
        return Ok(Hull::Apart);
    }
    let reach = values
        .iter()
        .fold(0.0_f64, |m, f| m.max(f.lo().abs()).max(f.hi().abs()));
    let ceiling = curved.ceiling_metres(reach, box_);
    match decide_magnitude(
        "section_nurbs_implicit_coincident",
        Margin::of(ceiling),
        band,
    ) {
        Ok(Magnitude::Zero) => Err(SectionError::CoincidentSurfaces),
        Ok(Magnitude::Positive) => Ok(Hull::Cuts),
        Err(diag) => Err(SectionError::Escalated(diag)),
    }
}

/// **The door's collapsed sides** (module docs).
///
/// # Errors
///
/// [`SectionError::Unaccounted`], naming the side.
pub(super) fn door(
    curved: &Curved,
    net: &NurbsSurface<f64>,
    band: Band,
) -> Result<(), SectionError> {
    let control = net.control();
    for side in crate::ssi::boundary::SIDES {
        let Some(row) = crate::chart_singularity::spline_collapsed_row(net, side, band) else {
            continue;
        };
        let signs: Vec<_> = row
            .iter()
            .filter_map(|&i| control.get(i))
            .map(|&p| {
                let q = curved.framed_point(p);
                decide(
                    "section_nurbs_implicit_pole",
                    Margin::of(curved.floor_metres(curved.value(q), q)),
                    band,
                )
                .ok()
            })
            .collect();
        let one = |s| !signs.is_empty() && signs.iter().all(|&t| t == Some(s));
        if !(one(Sign::Positive) || one(Sign::Negative)) {
            return Err(SectionError::Unaccounted {
                cause: format!(
                    "{side} collapses to a point not decided clear of the {}",
                    curved.name()
                ),
            });
        }
    }
    Ok(())
}
