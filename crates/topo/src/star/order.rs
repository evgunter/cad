//! **The germs' order round a vertex, the one door** (`star`'s module
//! docs, "The germs' order"): unit directions in one plane, ordered
//! clockwise about its normal, every reading levered as a distance, and
//! every tie the caller's.

use geom_core::{Band, Decide, Indeterminate, Lever, Margin, NonzeroSign, Sign, Vec3};

use super::Leaned;

/// K name: a germ's side of the direction an order is read from,
/// `n̂·(ĝ × ê)` levered at the shorter reach a reading of it uses and the
/// smaller lean (metres): its distance there from `ê`'s line.
pub(crate) const GERM_SIDE: &str = "germ_side";

/// K name: two germs in one half-turn, `n̂·(ĝ₀ × ĝ₁)` levered at their
/// arm and the smaller lean (metres). A decided zero is the caller's tie.
pub(crate) const GERM_ORDER: &str = "germ_order";

/// K name: two directions a side reads on one line, their cosine levered
/// at the arm (metres): along each other or opposite.
pub(crate) const GERM_SENSE: &str = "germ_sense";

/// Why the order refuses ([`before`], [`germ_order`]); each verb maps it
/// into its own error.
#[derive(Clone, Copy, Debug)]
pub(crate) enum OrderFault {
    /// A side or an order reads in the band, or reads zero where nothing
    /// orders the tie: the germs coincide to the tolerance.
    Coincident(Indeterminate),
    /// Two directions on one line whose sense reads in the band.
    Sense(Indeterminate),
    /// Two germs each along the first point opposite ways: a kernel
    /// defect.
    Opposite,
}

/// **The lever a reading of two directions' sides carries, given their
/// leans**: `lever · min(lean₀, lean₁)` ([`Leaned`]). A unit face line
/// turns by up to `δ / sin φ` under a face tilt `δ`; levered so, the
/// noise of a side read at `lever` is at most the two directions' own
/// tolerances, whatever the faces' angle.
pub(crate) fn leaned_lever<T: geom_core::Real>(lever: Lever<T>, (l0, l1): (T, T)) -> Lever<T> {
    // Unsourced lever: an arm scaled by a sine (the directions' lean),
    // a length no source reads.
    Lever::unsourced(lever.length() * l0.min(l1))
}

/// Whether two directions read on one line point the same way (`true`)
/// or opposite ways, their cosine levered at `arm` ([`GERM_SENSE`]). A
/// decided zero refuses with its margin, as the in-band arm does.
fn same_sense<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<bool, OrderFault> {
    crate::validate::decide_nonzero(GERM_SENSE, Margin::levered(u.dot(v), arm), band)
        .map(|sign| sign == NonzeroSign::Positive)
        .map_err(OrderFault::Sense)
}

/// Two germs' turn: off each other's line (the sign of `(g₀ × g₁)·n`) or
/// along one direction, decided so ([`turn`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Turn {
    /// `Negative` where `g₁` follows `g₀` clockwise about `n`.
    Off(NonzeroSign),
    /// Along one line, decided so by the reading.
    Along(Indeterminate),
}

/// **How `g₁` turns from `g₀` about `normal`**, the order's one sine
/// ([`GERM_ORDER`]): `(g₀ × g₁)·n` levered at `arm` and the smaller lean
/// ([`leaned_lever`]). Two germs a reading puts on one line are the
/// caller's tie ([`Turn::Along`]).
///
/// # Errors
///
/// The reading in the band.
pub(crate) fn turn<T: Decide>(
    (g0, g1): (Leaned<T>, Leaned<T>),
    normal: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<Turn, Indeterminate> {
    let lever = leaned_lever(arm, (g0.lean, g1.lean));
    let order = Margin::levered(g0.dir.cross(g1.dir).dot(normal), lever);
    match crate::validate::decide_nonzero(GERM_ORDER, order, band) {
        Ok(sign) => Ok(Turn::Off(sign)),
        Err(diag) if diag.margin.rejected_sign() == Some(Sign::Zero) => Ok(Turn::Along(diag)),
        Err(diag) => Err(diag),
    }
}

/// **Whether `germs.0` comes before `germs.1` clockwise about `normal`
/// from `e`**, the order's one comparison.
///
/// Each germ is first placed in its half-turn by its side of `e`,
/// `(g × e)·n` ([`GERM_SIDE`], levered at `e`'s `reach`): the near
/// half-turn `[0, π)` reads positive, the far one `[π, 2π)` negative, and
/// a germ on `e`'s line is placed by its sense ([`GERM_SENSE`]: along
/// `e`, angle 0, near; against it, π, far). A germ in the near half-turn
/// comes first. Within one half-turn the earlier germ is the one the
/// other follows clockwise, `(g₀ × g₁)·n < 0` ([`GERM_ORDER`], at `arm`).
/// Every reading is a sine levered as a distance and by the smaller lean
/// of the directions it reads ([`leaned_lever`]): a sine is linear in the
/// spacing at every angle, where a cosine is flat beside 0 and π. Two
/// germs along one direction are the caller's `tie`: the sign of
/// `(g₀ × g₁)·n` one order down (`geom_brep::tied_germ_turn`), or `None`
/// where nothing orders them, which refuses with the first-order reading.
///
/// # Errors
///
/// [`OrderFault`], and the tie's own.
pub(crate) fn before<T: Decide, E>(
    (e, reach): (Leaned<T>, Lever<T>),
    normal: Vec3<T>,
    germs: (Leaned<T>, Leaned<T>),
    arm: Lever<T>,
    tie: impl FnOnce() -> Result<Option<Sign>, E>,
    fault: impl Fn(OrderFault) -> E,
    band: Band,
) -> Result<bool, E> {
    let far = |g: Leaned<T>| -> Result<bool, OrderFault> {
        let lever = leaned_lever(reach, (e.lean, g.lean));
        let side = Margin::levered(g.dir.cross(e.dir).dot(normal), lever);
        match crate::validate::decide(GERM_SIDE, side, band).map_err(OrderFault::Coincident)? {
            Sign::Positive => Ok(false),
            Sign::Negative => Ok(true),
            Sign::Zero => Ok(!same_sense(g.dir, e.dir, reach, band)?),
        }
    };
    let (far0, far1) = (far(germs.0).map_err(&fault)?, far(germs.1).map_err(&fault)?);
    if far0 != far1 {
        return Ok(far1);
    }
    match turn(germs, normal, arm, band).map_err(|diag| fault(OrderFault::Coincident(diag)))? {
        Turn::Off(sign) => Ok(sign == NonzeroSign::Negative),
        Turn::Along(diag) => match tie()? {
            Some(Sign::Negative) => Ok(true),
            Some(Sign::Positive) => Ok(false),
            Some(Sign::Zero) | None => Err(fault(OrderFault::Coincident(diag))),
        },
    }
}

/// Where germ `j` lies from germ `i` about the normal ([`GERM_SIDE`]).
enum GermSide {
    /// Off `i`'s line: the sign of `(gᵢ × gⱼ)·n`.
    Off(NonzeroSign),
    /// Along `i`'s direction, decided so by this reading.
    Along(Indeterminate),
    /// Along the ray opposite `i`.
    Opposite,
}

/// The caller's two tie rules ([`germ_order`]).
pub(crate) struct Ties<A, P> {
    /// Of germs `i` and `j` read along one direction (the reading's
    /// zero), whether `i` comes after `j` clockwise.
    pub along: A,
    /// Two germs off the first's line tied in one half-turn: the sign of
    /// `(gᵢ × gⱼ)·n` one order down at the arm, or `None`
    /// ([`before`]'s `tie`).
    pub pair: P,
}

/// **The germs' cyclic order**: indices into `germs` (each a direction
/// and its arm), clockwise about `normal` from `germs[0]`. Every
/// comparison is [`before`]'s from `germs[0]`, levered at the shortest of
/// its three germs' arms. A germ along `germs[0]` has no place before or
/// after it at first order, so each germ's side of `germs[0]` is read
/// first at every arm a reading of it uses: in band with its sense along,
/// it refuses as the germs' coincidence rather than stand at `germs[0]`,
/// unless the reading decides it along; then the caller's `along` tie
/// places it just after `germs[0]` or, a full turn round, just before.
///
/// # Errors
///
/// [`OrderFault`], and the ties' own.
pub(crate) fn germ_order<T: Decide, E, A, P>(
    germs: &[(Leaned<T>, Lever<T>)],
    normal: Vec3<T>,
    ties: Ties<A, P>,
    fault: impl Fn(OrderFault) -> E,
    band: Band,
) -> Result<Vec<usize>, E>
where
    A: FnMut(usize, usize, Indeterminate) -> Result<bool, E>,
    P: Fn(usize, usize, Lever<T>) -> Result<Option<Sign>, E>,
{
    let Ties {
        along: mut after,
        pair,
    } = ties;
    let Some(&(from, from_arm)) = germs.first() else {
        return Ok(Vec::new());
    };
    let side = |i: usize, j: usize, arm: Lever<T>| -> Result<GermSide, OrderFault> {
        let (g, h) = (germs[i].0, germs[j].0);
        let side = Margin::levered(
            g.dir.cross(h.dir).dot(normal),
            leaned_lever(arm, (g.lean, h.lean)),
        );
        match crate::validate::decide_nonzero(GERM_SIDE, side, band) {
            Ok(sign) => Ok(GermSide::Off(sign)),
            Err(diag) if diag.margin.rejected_sign() == Some(Sign::Zero) => {
                Ok(if same_sense(g.dir, h.dir, arm, band)? {
                    GermSide::Along(diag)
                } else {
                    GermSide::Opposite
                })
            }
            Err(diag) => Err(OrderFault::Coincident(diag)),
        }
    };
    let along = |i: usize, j: usize, arm: Lever<T>| -> Result<Option<Indeterminate>, OrderFault> {
        Ok(match side(i, j, arm)? {
            GermSide::Along(diag) => Some(diag),
            GermSide::Off(_) | GermSide::Opposite => None,
        })
    };
    let apart = |g: Leaned<T>, arm: Lever<T>| -> Result<(), OrderFault> {
        let lever = leaned_lever(arm, (g.lean, from.lean));
        let side = Margin::levered(g.dir.cross(from.dir).dot(normal), lever);
        match crate::validate::decide_nonzero(GERM_SIDE, side, band) {
            Err(diag) if same_sense(g.dir, from.dir, arm, band)? => {
                Err(OrderFault::Coincident(diag))
            }
            _ => Ok(()),
        }
    };
    // `order` holds the germs strictly off `germs[0]`'s direction; a
    // germ along it stands just after it (`head`) or, a full turn
    // round, just before it (`tail`), each in its own order.
    let (mut order, mut head, mut tail): (Vec<usize>, Vec<usize>, Vec<usize>) =
        (vec![0], Vec::new(), Vec::new());
    for (i, &(g, g_arm)) in germs.iter().enumerate().skip(1) {
        if let Some(diag) = along(i, 0, from_arm.min(g_arm)).map_err(&fault)? {
            let ray = if after(0, i, diag)? {
                &mut tail
            } else {
                &mut head
            };
            let mut at = ray.len();
            for (slot, &j) in ray.iter().enumerate() {
                // Two germs each along `germs[0]` may still read apart.
                let later = match side(i, j, g_arm.min(germs[j].1)).map_err(&fault)? {
                    GermSide::Along(diag) => after(i, j, diag)?,
                    GermSide::Off(sign) => sign == NonzeroSign::Positive,
                    GermSide::Opposite => return Err(fault(OrderFault::Opposite)),
                };
                if !later {
                    at = slot;
                    break;
                }
            }
            ray.insert(at, i);
            continue;
        }
        apart(g, from_arm.min(g_arm)).map_err(&fault)?;
        let mut at = order.len();
        for (slot, &j) in order.iter().enumerate().skip(1) {
            let (h, h_arm) = germs[j];
            let arm = from_arm.min(g_arm).min(h_arm);
            if let Some(diag) = along(i, j, g_arm.min(h_arm)).map_err(&fault)? {
                if !after(i, j, diag)? {
                    at = slot;
                    break;
                }
                continue;
            }
            apart(g, arm).map_err(&fault)?;
            apart(h, arm).map_err(&fault)?;
            if before(
                (from, arm),
                normal,
                (g, h),
                arm,
                || pair(i, j, arm),
                &fault,
                band,
            )? {
                at = slot;
                break;
            }
        }
        order.insert(at, i);
    }
    Ok([&order[..1], &head, &order[1..], &tail].concat())
}
