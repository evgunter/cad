//! **The certified walk**: the meetings of two spans whose pair has a
//! spiric or spline side, and the closed forms' differential test oracle
//! on the rest. It reads any two carriers through their rate bounds
//! alone ([`crate::carrier_speed`]).
//!
//! **Candidates first.** Each span's middle, and the points halfway
//! between one span's end and the other's ends, one of which a
//! positive-length overlap holds, are read on both spans at each of their
//! feet: a foot is the carrier's closed-form parameter where it inverts
//! (a line, a circle), and otherwise the polished middle of each piece of
//! the span the point may rest on ([`near_pieces`]).
//!
//! **The walk** is [`geom_core::bisect`]'s, over pairs of pieces. A piece
//! reaches at most `ℓ` metres from its middle point, `ℓ` its half-span
//! times the carrier's greatest speed. Two pieces are cleared when the
//! distance between their middle points less both reaches decides
//! positive (`curve_pair_walk_clear`), or, where both carriers bound
//! their acceleration, the distance's component along a unit `n` less
//! each piece's run along `n` and its bend (`|n·C′|·h + |C″|·h²/2`), read
//! along the gap and across each piece's tangent. Pieces stop at
//! [`geom_core::bisect::tangency_floor`] of `1/κ`, `κ` the two carriers'
//! summed curvature bound (`curve_pair_walk_piece`); a spline bounds no
//! curvature here, so its pieces stop at the band itself. A pair that
//! stops is polished toward the meeting by Gauss–Newton, each carrier's
//! foot of the other's point is polished, and the point is read on both
//! spans ([`Span::contact`]). Every piece the walk does not clear ends at
//! such a pair, so no meeting is left unread; an escalated clearing keeps
//! its piece. A walk past [`geom_core::bisect::PIECE_BUDGET`] refuses
//! typed ([`Unsettled::Exhausted`]).

use geom::Curve3;
use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, SupSpeed, Vec3};

use super::span::Span;
use super::{MeetFault, MeetRefusal, Meeting, Meetings};
use crate::roots::Unsettled;

/// A span as the walk reads it: with its bounds on `|C″|` and on the
/// curvature, `None` on a spline.
struct Read<'r, 's, T: Decide> {
    span: &'r Span<'s, T>,
    accel: Option<T>,
    curvature: Option<T>,
}

impl<'r, 's, T: Decide> Read<'r, 's, T> {
    fn of(span: &'r Span<'s, T>) -> Self {
        Self {
            span,
            accel: crate::carrier_speed::accel_hi(span.carrier()),
            curvature: crate::carrier_speed::curvature_hi(span.carrier()),
        }
    }

    fn curve(&self) -> &'s Curve3<T> {
        self.span.carrier()
    }

    fn hi(&self) -> SupSpeed<T> {
        self.span.speeds().1
    }
}

/// A run `(a, b)` of a span, cut `depth` times.
#[derive(Clone, Copy)]
struct Piece<T> {
    a: T,
    b: T,
    depth: u32,
}

impl<T: Decide> Piece<T> {
    fn whole((a, b): (T, T)) -> Self {
        Self { a, b, depth: 0 }
    }

    fn mid(self) -> T {
        geom::mid_param(self.a, self.b)
    }

    /// Half the piece's parameter run.
    fn half(self) -> T {
        (self.b - self.a).abs() * T::from_f64(0.5)
    }

    /// The most metres a point of the piece lies from its middle point.
    fn reach(self, hi: SupSpeed<T>) -> T {
        geom_core::bisect::reach(hi, (self.a, self.b))
    }

    fn halves(self) -> [Self; 2] {
        let (m, depth) = (self.mid(), self.depth + 1);
        [
            Self {
                a: self.a,
                b: m,
                depth,
            },
            Self {
                a: m,
                b: self.b,
                depth,
            },
        ]
    }
}

/// Whether a walk decision definitely came out `sign`; an escalation
/// keeps the piece open.
fn surely<T: Decide>(name: &'static str, margin: T, band: Band, sign: Sign) -> bool {
    decide(name, Margin::of(margin), band) == Ok(sign)
}

/// Whether a polish step that moves its point `moved` metres is taken:
/// while it moves less than `16·slack`, sixteen times the most the
/// polished point can lie from where it started, so a step that runs off
/// along the carrier is dropped. A polished parameter is read, never
/// trusted, so the bound can lose a meeting's foot but never invent one.
///
/// The comparison is a decision, not a `<`, because an enclosure that
/// straddles it is neither taken nor dropped by a `<`; the band is the
/// least pair of positive `f64`s, so it reads the sign alone.
fn step_taken<T: Decide>(moved: T, slack: T) -> bool {
    Band::new(f64::from_bits(1), f64::from_bits(2)).is_ok_and(|exact| {
        surely(
            "curve_pair_walk_polish_step",
            slack * T::from_f64(16.0) - moved,
            exact,
            Sign::Positive,
        )
    })
}

/// Gauss–Newton from `t` toward `r`'s foot of `q` (`step_taken`).
fn polish_foot<T: Decide>(r: &Read<'_, '_, T>, q: Point3<T>, t: T, slack: T) -> T {
    super::polish::foot(|t| r.curve().ders1(t), q, t, |m| step_taken(m, slack))
}

/// Whether a piece of reach `reach` is small enough to stop at, decided on
/// `row`: within the floor, or within the band where there is no finite
/// floor.
fn stops<T: Decide>(row: &'static str, reach: T, floor: Option<T>, band: Band) -> bool {
    let within = |margin: T| {
        matches!(
            decide(row, Margin::of(margin), band),
            Ok(Sign::Negative | Sign::Zero)
        )
    };
    match floor {
        Some(f) if geom_core::is_finite_length(f) => within(reach - f),
        Some(_) | None => within(reach),
    }
}

/// The pieces of `r`'s span that `q` may rest on: halved until each is
/// decided clear of `q` or its reach stops at the band. Every point of the
/// span within the band of `q` lies in a kept piece.
fn near_pieces<T: Decide>(
    r: &Read<'_, '_, T>,
    q: Point3<T>,
    band: Band,
) -> Result<Vec<Piece<T>>, geom_core::bisect::Exhausted> {
    let mut out = Vec::new();
    bisect(
        [Piece::whole(r.span.params())],
        |p| -> Step<Piece<T>, core::convert::Infallible> {
            let reach = p.reach(r.hi());
            let gap = (r.curve().eval(p.mid()) - q).norm();
            if surely(
                "curve_pair_walk_foot_clear",
                gap - reach,
                band,
                Sign::Positive,
            ) {
                return Step::Settled;
            }
            if stops("curve_pair_walk_foot_piece", reach, None, band) {
                out.push(p);
                return Step::Settled;
            }
            Step::Halve(p.halves())
        },
    )?;
    Ok(out)
}

/// The parameters of `r`'s carrier where `q` may lie on it within the band
/// (module docs).
fn feet<T: Decide>(
    r: &Read<'_, '_, T>,
    q: Point3<T>,
    band: Band,
) -> Result<Vec<T>, geom_core::bisect::Exhausted> {
    let (t0, t1) = r.span.params();
    if let Some(t) = r.curve().param_near(q, geom::mid_param(t0, t1)) {
        let slack = (r.curve().eval(t) - q).norm();
        return Ok(vec![polish_foot(r, q, t, slack)]);
    }
    Ok(near_pieces(r, q, band)?
        .into_iter()
        .map(|p| {
            let m = p.mid();
            let slack = (r.curve().eval(m) - q).norm() + p.reach(r.hi());
            polish_foot(r, q, m, slack)
        })
        .collect())
}

/// What a point read on both spans makes of it.
enum Read2<T: geom_core::Real> {
    /// A meeting.
    Met(Meeting<T>),
    /// Definitely not on one span or the other.
    Missed,
    /// Neither decided: the first escalation.
    Held(Indeterminate),
}

/// `q` read on `a` at each of `ta` and on `b` at each of `tb`: on a span
/// where one parameter puts it there; off it where every one decides it
/// off; held otherwise.
fn read_both<T: Decide>(
    (a, ta): (&Span<'_, T>, &[T]),
    (b, tb): (&Span<'_, T>, &[T]),
    q: Point3<T>,
    band: Band,
) -> Read2<T> {
    let side = |s: &Span<'_, T>, ts: &[T]| {
        let mut held = None;
        for &t in ts {
            match s.contact(q, t, band) {
                Ok(c) => {
                    if let Some(hit) = c.hit() {
                        return Ok(Some(hit));
                    }
                }
                Err(diag) => held = held.or(Some(diag)),
            }
        }
        held.map_or(Ok(None), Err)
    };
    match (side(a, ta), side(b, tb)) {
        (Ok(None), _) | (_, Ok(None)) => Read2::Missed,
        (Ok(Some(on_a)), Ok(Some(on_b))) => Read2::Met(Meeting {
            point: q,
            on_a,
            on_b,
        }),
        (Err(diag), _) | (_, Err(diag)) => Read2::Held(diag),
    }
}

/// Whether the pieces `pa` of `a` and `pb` of `b` are definitely apart
/// (module docs).
fn pieces_apart<T: Decide>(
    (a, pa): (&Read<'_, '_, T>, Piece<T>),
    (b, pb): (&Read<'_, '_, T>, Piece<T>),
    band: Band,
) -> bool {
    let ((qa, da), (qb, db)) = (a.curve().ders1(pa.mid()), b.curve().ders1(pb.mid()));
    let gap = qa - qb;
    let dist = gap.norm();
    let clear = |bound: T| surely("curve_pair_walk_clear", bound, band, Sign::Positive);
    if clear(dist - pa.reach(a.hi()) - pb.reach(b.hi())) {
        return true;
    }
    let (Some(acc_a), Some(acc_b)) = (a.accel, b.accel) else {
        return false;
    };
    let (ha, hb) = (pa.half(), pb.half());
    let bend = (acc_a * ha.powi(2) + acc_b * hb.powi(2)) * T::from_f64(0.5);
    // `|D| ≥ |n·D|` for every unit `n`: along the gap, and across each
    // piece's tangent, where two near-parallel pieces read their
    // separation rather than their offset along it.
    let across = |d: Vec3<T>| gap - d * (gap.dot(d) / d.dot(d));
    [gap, across(da), across(db)].into_iter().any(|n| {
        let n = n / n.norm();
        clear(n.dot(gap).abs() - n.dot(da).abs() * ha - n.dot(db).abs() * hb - bend)
    })
}

/// The tangency floor of `a` against `b` in metres, or `None` where a
/// carrier bounds no curvature.
fn floor<T: Decide>(a: &Read<'_, '_, T>, b: &Read<'_, '_, T>, band: Band) -> Option<T> {
    let kappa = a.curvature? + b.curvature?;
    Some(geom_core::bisect::tangency_floor(
        T::one() / kappa,
        band.escalate(),
    ))
}

/// **The meetings of `a` and `b` by the walk** (module docs).
///
/// # Errors
///
/// [`MeetRefusal`] carrying the meetings found: [`MeetFault::Escalated`]
/// where a point read stayed in the band, [`Unsettled::Exhausted`] past
/// the piece budget.
pub(super) fn meet<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Meetings<T>, MeetRefusal<T>> {
    let (ra, rb) = (Read::of(a), Read::of(b));
    let mut found = Vec::new();
    let mut held = None;
    let exhausted = |found| MeetRefusal {
        fault: MeetFault::Unsettled(Unsettled::Exhausted),
        found,
        shared: None,
    };
    let mut take = |read: Read2<T>, found: &mut Vec<Meeting<T>>| match read {
        Read2::Met(m) => found.push(m),
        Read2::Missed => {}
        Read2::Held(diag) => held = held.or(Some(diag)),
    };
    let mut points = Vec::new();
    for (r, other) in [(&ra, &rb), (&rb, &ra)] {
        let (o0, o1) = other.span.params();
        let mut ends = Vec::new();
        for q in [other.curve().eval(o0), other.curve().eval(o1)] {
            let Ok(ts) = feet(r, q, band) else {
                return Err(exhausted(found));
            };
            ends.extend(ts);
        }
        points.extend(
            r.span
                .witness_params(ends)
                .into_iter()
                .map(|t| r.curve().eval(t)),
        );
    }
    for q in points {
        let (Ok(ta), Ok(tb)) = (feet(&ra, q, band), feet(&rb, q, band)) else {
            return Err(exhausted(found));
        };
        take(read_both((a, &ta), (b, &tb), q, band), &mut found);
    }
    let floor = floor(&ra, &rb, band);
    let walk = bisect(
        [(Piece::whole(a.params()), Piece::whole(b.params()))],
        |(pa, pb)| -> Step<(Piece<T>, Piece<T>), core::convert::Infallible> {
            if pieces_apart((&ra, pa), (&rb, pb), band) {
                return Step::Settled;
            }
            let stop_a = stops("curve_pair_walk_piece", pa.reach(ra.hi()), floor, band);
            let stop_b = stops("curve_pair_walk_piece", pb.reach(rb.hi()), floor, band);
            if stop_a && stop_b {
                let (ma, mb) = (pa.mid(), pb.mid());
                let slack = (ra.curve().eval(ma) - rb.curve().eval(mb)).norm()
                    + pa.reach(ra.hi())
                    + pb.reach(rb.hi());
                let (s, t, _) = super::polish::pair(
                    |s| ra.curve().ders1(s),
                    |t| rb.curve().ders1(t),
                    (ma, mb),
                    |m| step_taken(m, slack),
                );
                let t = polish_foot(&rb, ra.curve().eval(s), t, slack);
                let s = polish_foot(&ra, rb.curve().eval(t), s, slack);
                let q = ra.curve().eval(s);
                take(read_both((a, &[s]), (b, &[t]), q, band), &mut found);
                return Step::Settled;
            }
            if !stop_a && (stop_b || pa.depth <= pb.depth) {
                Step::Halve(pa.halves().map(|h| (h, pb)))
            } else {
                Step::Halve(pb.halves().map(|h| (pa, h)))
            }
        },
    );
    if walk.is_err() {
        return Err(exhausted(found));
    }
    match held {
        None => Ok(Meetings {
            shared: None,
            points: found,
        }),
        Some(diag) => Err(MeetRefusal {
            fault: MeetFault::Escalated(diag),
            found,
            shared: None,
        }),
    }
}
