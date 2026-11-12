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
    /// orders the tie, or the zeros it read do not agree (two germs each
    /// along the first and opposite each other): the germs coincide to the
    /// tolerance.
    Coincident(Indeterminate),
    /// Two directions on one line whose sense reads in the band.
    Sense(Indeterminate),
}

/// **The lever a reading of two directions' sides carries, given their
/// leans**: `lever · min(lean₀, lean₁)` ([`Leaned`]).
///
/// A unit face line is steeper than its faces' own tilt: a tilt `δ` of
/// either face, a displacement `δ·L ≤ zero` at the reach `L` it governs,
/// turns their line in the reading's plane by up to `δ / sin φ`. A side
/// read at `lever` therefore carries noise up to
/// `lever · (δ₀ / s₀ + δ₁ / s₁)`, `sᵢ` each direction's lean (an edge's
/// `δ` its own departure, its lean 1). Levered at `lever · min(s₀, s₁)`
/// that noise is at most `lever · (δ₀ + δ₁)`: the two directions' own
/// tolerances, whatever the faces' angle. One direction tilted within
/// the zero therefore decides as the upright pose; two tilted the same
/// way can add past it, as they do between two edges. Against an edge
/// lying in the reading's plane, the reading is the edge's far end's
/// offset from the face line's other face; an edge off that plane adds
/// its own departure, foreshortened by `cos φ` (`boolean::sectors::rays_within`).
pub(crate) fn leaned_lever<T: geom_core::Real>(lever: Lever<T>, (l0, l1): (T, T)) -> Lever<T> {
    // Unsourced lever: an arm scaled by a sine (the directions' lean),
    // a length no source reads.
    Lever::unsourced(lever.length() * l0.min(l1))
}

/// Whether two directions read parallel point the same way (`true`) or
/// opposite ways, their cosine levered at `arm` ([`GERM_SENSE`]). A
/// decided zero is returned with its margin, as the in-band reading is:
/// both say the arm is too short to tell.
///
/// # Errors
///
/// The reading's zero or band.
pub(crate) fn same_sense<T: Decide>(
    u: Vec3<T>,
    v: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    crate::validate::decide_nonzero(GERM_SENSE, Margin::levered(u.dot(v), arm), band)
        .map(|sign| sign == NonzeroSign::Positive)
}

/// **The order's one sine**: `(g × h)·n` under `name`, levered at `arm`
/// and the two directions' smaller lean ([`leaned_lever`]). Its sign where
/// definite; `Err(diag)` inside it where it decides the two on one line.
///
/// # Errors
///
/// [`OrderFault::Coincident`]: the reading in the band.
fn sine<T: Decide>(
    name: &'static str,
    (g, h): (Leaned<T>, Leaned<T>),
    normal: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<Result<NonzeroSign, Indeterminate>, OrderFault> {
    let leaned = Margin::levered(
        g.dir.cross(h.dir).dot(normal),
        leaned_lever(arm, (g.lean, h.lean)),
    );
    match crate::validate::decide_nonzero(name, leaned, band) {
        Ok(sign) => Ok(Ok(sign)),
        Err(diag) if diag.margin.rejected_sign() == Some(Sign::Zero) => Ok(Err(diag)),
        Err(diag) => Err(OrderFault::Coincident(diag)),
    }
}

/// Where germ `h` lies from germ `g` about the normal ([`GERM_SIDE`]).
enum Place {
    /// Off `g`'s line: the sign of `(g × h)·n`.
    Off(NonzeroSign),
    /// Along `g`'s direction, decided so.
    Along(Indeterminate),
    /// Along the ray opposite `g`.
    Opposite,
}

/// [`sine`] under [`GERM_SIDE`], a zero placed by its sense.
fn place<T: Decide>(
    gh: (Leaned<T>, Leaned<T>),
    normal: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<Place, OrderFault> {
    match sine(GERM_SIDE, gh, normal, arm, band)? {
        Ok(sign) => Ok(Place::Off(sign)),
        Err(diag) => Ok(
            if same_sense(gh.0.dir, gh.1.dir, arm, band).map_err(OrderFault::Sense)? {
                Place::Along(diag)
            } else {
                Place::Opposite
            },
        ),
    }
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

/// **How `g₁` turns from `g₀` about `normal`** ([`sine`] under
/// [`GERM_ORDER`]). Two germs it puts on one line are the caller's tie
/// ([`Turn::Along`]).
///
/// # Errors
///
/// [`sine`]'s.
pub(crate) fn turn<T: Decide>(
    germs: (Leaned<T>, Leaned<T>),
    normal: Vec3<T>,
    arm: Lever<T>,
    band: Band,
) -> Result<Turn, OrderFault> {
    Ok(match sine(GERM_ORDER, germs, normal, arm, band)? {
        Ok(sign) => Turn::Off(sign),
        Err(diag) => Turn::Along(diag),
    })
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
/// other follows clockwise, `(g₀ × g₁)·n < 0` ([`turn`], at `arm`).
/// Every reading is [`sine`]: levered as a distance and by the smaller
/// lean of the directions it reads; a sine is linear in the spacing at
/// every angle, where a cosine is flat beside 0 and π. Two germs along
/// one direction are the caller's `tie`: the sign of `(g₀ × g₁)·n` one
/// order down (`geom_brep::tied_germ_turn`), or `None` where nothing
/// orders them, which refuses with the first-order reading.
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
        Ok(match place((g, e), normal, reach, band)? {
            Place::Off(sign) => sign == NonzeroSign::Negative,
            Place::Along(_) => false,
            Place::Opposite => true,
        })
    };
    let (far0, far1) = (far(germs.0).map_err(&fault)?, far(germs.1).map_err(&fault)?);
    if far0 != far1 {
        return Ok(far1);
    }
    match turn(germs, normal, arm, band).map_err(&fault)? {
        Turn::Off(sign) => Ok(sign == NonzeroSign::Negative),
        Turn::Along(diag) => match tie()? {
            Some(Sign::Negative) => Ok(true),
            Some(Sign::Positive) => Ok(false),
            Some(Sign::Zero) | None => Err(fault(OrderFault::Coincident(diag))),
        },
    }
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
/// Two germs each decided along `germs[0]` that read opposite each other
/// coincide to the tolerance, and refuse so.
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
    let side =
        |i: usize, j: usize, arm: Lever<T>| place((germs[i].0, germs[j].0), normal, arm, band);
    let along = |i: usize, j: usize, arm: Lever<T>| -> Result<Option<Indeterminate>, OrderFault> {
        Ok(match side(i, j, arm)? {
            Place::Along(diag) => Some(diag),
            Place::Off(_) | Place::Opposite => None,
        })
    };
    // A germ off `germs[0]`'s line at one arm is off it at every arm a
    // comparison of it uses.
    let apart = |g: Leaned<T>, arm: Lever<T>| -> Result<(), OrderFault> {
        match sine(GERM_SIDE, (g, from), normal, arm, band)? {
            Err(diag) if same_sense(g.dir, from.dir, arm, band).map_err(OrderFault::Sense)? => {
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
                    Place::Along(diag) => after(i, j, diag)?,
                    Place::Off(sign) => sign == NonzeroSign::Positive,
                    Place::Opposite => return Err(fault(OrderFault::Coincident(diag))),
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **Germs whose leaned zeros disagree refuse typed, never panic** (the
    /// FULL review's probe on PR 6871): germs at 0°, ±60° and 180°, each a
    /// face line leaning `1e-7` or `1e-6`, at arm `1e-3`. Levered by their
    /// lean the sines between them read zero, so the ±60° germs read along
    /// `germs[0]`, as the lean doctrine reads a face line within its
    /// faces' tolerance, and opposite each other: the zeros do not agree,
    /// and the order refuses as the germs' coincidence.
    #[test]
    fn germs_far_apart_on_steep_face_lines_refuse_as_coincident() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let n = Vec3::new(0.0, 0.0, 1.0_f64);
        let at = |deg: f64, lean: f64| {
            let t = deg.to_radians();
            (
                Leaned {
                    dir: Vec3::new(t.cos(), t.sin(), 0.0),
                    lean,
                },
                // Unsourced lever: the probe's hand-stated arm.
                Lever::unsourced(1e-3),
            )
        };
        for lean in [1e-7, 1e-6] {
            let germs = [
                at(0.0, lean),
                at(60.0, lean),
                at(-60.0, lean),
                at(180.0, lean),
            ];
            let got = germ_order(
                &germs,
                n,
                Ties {
                    along: |_, _, _| Ok::<_, OrderFault>(false),
                    pair: |_, _, _| Ok(None),
                },
                |f| f,
                band,
            );
            assert!(
                matches!(got, Err(OrderFault::Coincident(_))),
                "lean {lean}: the far-apart germs refuse as coincident, got {got:?}"
            );
        }
        // Upright, the same germs order: 0°, then clockwise −60°, 180°, 60°.
        let germs = [at(0.0, 1.0), at(60.0, 1.0), at(-60.0, 1.0), at(180.0, 1.0)];
        let got = germ_order(
            &germs,
            n,
            Ties {
                along: |_, _, _| Ok::<_, OrderFault>(false),
                pair: |_, _, _| Ok(None),
            },
            |f| f,
            band,
        )
        .unwrap();
        assert_eq!(got, vec![0, 2, 3, 1], "upright germs order clockwise");
    }
}
