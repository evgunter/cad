//! **The certified walk**: the meetings of two spans whose pair has a
//! spiric or spline side, and the closed forms' differential test oracle
//! on the rest. It reads any two carriers through their rate bounds
//! alone ([`geom::carrier_speed`]).
//!
//! **Ends first, then candidates.** Each span's ends are read against
//! the other span at each of their feet there. Then each span's middle,
//! and the points halfway between one span's end and the feet of the
//! other's ends, one of which a positive-length overlap holds, are read
//! on both spans at each of their feet: a foot is the carrier's
//! closed-form parameter where it inverts (a line, a circle), and
//! otherwise the middle of each piece of the span the point may rest on
//! polished to the point's foot ([`Curve3::foot`]). A witness halfway to
//! an unpolished piece middle lies within the band's gap of a joint, not
//! at it.
//!
//! **The walk** is [`geom_core::bisect`]'s, over pairs of pieces. A piece
//! reaches at most `ℓ` metres from its middle point, `ℓ` its half-span
//! times the carrier's greatest speed. Two pieces are cleared when the
//! distance between their middle points less both reaches decides
//! positive (`curve_pair_walk_clear`), or the distance's component
//! along a unit `n` less each piece's run along `n` and its bend
//! (`|n·C′|·h + |C″|·h²/2`, [`geom::carrier_speed::accel_hi`]), read
//! along the gap and across each piece's tangent. The bend needs each
//! carrier `C¹` across its piece, and its bound is finite only where it
//! is: a spline with a crease clears on the first reading alone. Pieces
//! stop at [`geom_core::bisect::tangency_floor`] of `1/κ`, `κ` the two
//! carriers' summed curvature bound (`curve_pair_walk_piece`). Where
//! that bound is not finite (a spline with a crease, or whose speed
//! floor certifies nothing) the floor is zero, poison, or at `Interval`
//! an enclosure reaching zero, and a piece stops only once its reach is
//! within the band. A pair that
//! stops is polished toward the meeting by Gauss–Newton, each carrier's
//! foot of the other's point is polished, and the point is read on both
//! spans ([`Span::contact`]). Every piece the walk does not clear ends at
//! such a pair, so no meeting is left unread; an escalated clearing keeps
//! its piece. Every point read is read under the reader's stretch rule
//! (the parent module's docs). A walk past
//! [`geom_core::bisect::PIECE_BUDGET`] refuses typed
//! ([`Unsettled::Exhausted`]).

use geom::{Curve3, PieceStep};
use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, SupSpeed, Vec3};

use super::span::Span;
use super::{Both, Found, MeetFault, MeetRefusal, Meetings};
use crate::roots::Unsettled;

/// A span as the walk reads it: with its bounds on `|C″|` and on the
/// curvature.
struct Read<'r, 's, T: Decide> {
    span: &'r Span<'s, T>,
    accel: T,
    curvature: T,
}

impl<'r, 's, T: Decide> Read<'r, 's, T> {
    fn of(span: &'r Span<'s, T>) -> Self {
        Self {
            span,
            accel: geom::carrier_speed::accel_hi(span.carrier()),
            curvature: geom::carrier_speed::curvature_hi(span.carrier()),
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
    surely(
        "curve_pair_walk_polish_step",
        slack * T::from_f64(16.0) - moved,
        Band::least(),
        Sign::Positive,
    )
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

/// The middles of the pieces of `span` that `q` may rest on, on the
/// carrier's point walk ([`Curve3::point_walk`]): halved until each is
/// decided clear of `q` or its reach stops at the band. Every point of the
/// span within the band of `q` lies in a kept piece. It reads only the
/// carrier's greatest speed, so it never refuses for want of a least one.
fn near_pieces<T: Decide>(
    span: &Span<'_, T>,
    q: Point3<T>,
    band: Band,
) -> Result<Vec<T>, geom_core::bisect::Exhausted> {
    let mut out = Vec::new();
    span.carrier().point_walk(
        q,
        span.params(),
        span.speeds().1,
        |p| -> PieceStep<core::convert::Infallible> {
            if surely(
                "curve_pair_walk_foot_clear",
                p.gap - p.reach,
                band,
                Sign::Positive,
            ) {
                return PieceStep::Settled;
            }
            if stops("curve_pair_walk_foot_piece", p.reach, None, band) {
                out.push(p.mid);
                return PieceStep::Settled;
            }
            PieceStep::Halve
        },
    )?;
    Ok(out)
}

/// The seeds of `q`'s feet on `r`'s carrier where it may lie on it within
/// the band (module docs): [`Curve3::param_near`]'s parameter where the
/// kind inverts, else the middle of each piece of the span `q` may rest
/// on. Each is a seed, read at its foot ([`Curve3::foot`]).
fn feet<T: Decide>(
    r: &Read<'_, '_, T>,
    q: Point3<T>,
    band: Band,
) -> Result<Vec<T>, geom_core::bisect::Exhausted> {
    let (t0, t1) = r.span.params();
    if let Some(t) = r.curve().param_near(q, geom::mid_param(t0, t1)) {
        return Ok(vec![t]);
    }
    near_pieces(r.span, q, band)
}

/// `q` read on `a` at each of `ta` and on `b` at each of `tb`: on a span
/// where one parameter puts it there; off it where every one decides it
/// off; held otherwise.
fn read_both<T: Decide>(
    (a, ta): (&Span<'_, T>, &[T]),
    (b, tb): (&Span<'_, T>, &[T]),
    q: Point3<T>,
    band: Band,
) -> Both<T> {
    let side = |s: &Span<'_, T>, ts: &[T]| {
        let mut held = None;
        for &t in ts {
            match s.contact(q, t, band) {
                Ok(Some(hit)) => return Ok(Some(hit)),
                Ok(None) => {}
                Err(refusal) => held = held.or(Some(refusal)),
            }
        }
        held.map_or(Ok(None), Err)
    };
    super::both(side(a, ta), side(b, tb))
}

/// The feet of `q` on `r`'s carrier ([`feet`]'s seeds, each polished to
/// its foot by [`Curve3::foot`]), one each.
fn polished_feet<T: Decide>(
    r: &Read<'_, '_, T>,
    q: Point3<T>,
    band: Band,
) -> Result<Vec<T>, geom_core::bisect::Exhausted> {
    let mut out: Vec<T> = Vec::new();
    for seed in feet(r, q, band)? {
        let t = r.curve().foot(q, seed).t();
        if !out.iter().any(|&u| same(u, t)) {
            out.push(t);
        }
    }
    Ok(out)
}

/// Whether two parameters are one, read exactly.
fn same<T: Decide>(s: T, t: T) -> bool {
    super::compare("curve_pair_walk_same_foot", s, t) == Some(Sign::Zero)
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
    let (ha, hb) = (pa.half(), pb.half());
    let bend = (a.accel * ha.powi(2) + b.accel * hb.powi(2)) * T::from_f64(0.5);
    // `|D| ≥ |n·D|` for every unit `n`: along the gap, and across each
    // piece's tangent, where two near-parallel pieces read their
    // separation rather than their offset along it.
    let across = |d: Vec3<T>| gap - d * (gap.dot(d) / d.dot(d));
    [gap, across(da), across(db)].into_iter().any(|n| {
        let n = n / n.norm();
        clear(n.dot(gap).abs() - n.dot(da).abs() * ha - n.dot(db).abs() * hb - bend)
    })
}

/// **The witness points** (module docs): each span's middle, and the
/// points halfway between each of its ends and the feet on it of the
/// other span's ends.
fn witnesses<T: Decide>(
    ra: &Read<'_, '_, T>,
    rb: &Read<'_, '_, T>,
    band: Band,
) -> Result<Vec<Point3<T>>, geom_core::bisect::Exhausted> {
    let mut points = Vec::new();
    for (r, other) in [(ra, rb), (rb, ra)] {
        let (o0, o1) = other.span.params();
        let mut ends = Vec::new();
        for q in [other.curve().eval(o0), other.curve().eval(o1)] {
            ends.extend(polished_feet(r, q, band)?);
        }
        points.extend(
            r.span
                .witness_params(ends)
                .into_iter()
                .map(|t| r.curve().eval(t)),
        );
    }
    Ok(points)
}

/// The tangency floor of `a` against `b` in metres: not finite where a
/// carrier bounds no finite curvature.
fn floor<T: Decide>(a: &Read<'_, '_, T>, b: &Read<'_, '_, T>, band: Band) -> T {
    geom_core::bisect::tangency_floor(T::one() / (a.curvature + b.curvature), band.escalate())
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
    let mut found = Found::default();
    let exhausted = |found: Found<T>| MeetRefusal {
        fault: MeetFault::Unsettled(Unsettled::Exhausted),
        found: found.points,
        shared: None,
    };
    // Each span's ends against the other, first.
    for (r, other, first) in [(&ra, &rb, true), (&rb, &ra, false)] {
        let (t0, t1) = r.span.params();
        for t in [t0, t1] {
            let q = r.curve().eval(t);
            let Ok(feet) = polished_feet(other, q, band) else {
                return Err(exhausted(found));
            };
            found.offer(
                q,
                || {
                    if first {
                        read_both((a, &[t]), (b, &feet), q, band)
                    } else {
                        read_both((a, &feet), (b, &[t]), q, band)
                    }
                },
                band,
            );
        }
    }
    let Ok(points) = witnesses(&ra, &rb, band) else {
        return Err(exhausted(found));
    };
    for q in points {
        let (Ok(ta), Ok(tb)) = (feet(&ra, q, band), feet(&rb, q, band)) else {
            return Err(exhausted(found));
        };
        found.offer(q, || read_both((a, &ta), (b, &tb), q, band), band);
    }
    let floor = Some(floor(&ra, &rb, band));
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
                let t = rb.curve().foot(ra.curve().eval(s), t).t();
                let s = ra.curve().foot(rb.curve().eval(t), s).t();
                let q = ra.curve().eval(s);
                found.offer(q, || read_both((a, &[s]), (b, &[t]), q, band), band);
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
    found.finish(a, b, None, band)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use geom::Curve3;
    use geom_core::k_stats::decide;
    use geom_core::{Band, Margin, Point3, Sign, Vec3};

    use super::{Read, witnesses};
    use crate::curve_pair::Span;

    /// **The walk's witnesses stand on polished feet.** A quadratic spline
    /// ending at the origin, which no closed form inverts, and a segment
    /// leaving that end square to it: the joint of a lofted wall. The feet
    /// of the segment's start on the spline are seeded at the middles of
    /// the pieces near it, up to about the band's escalation from the
    /// joint; a witness halfway between such a seed and the spline's end
    /// lies a few nanometres off the joint, in the band's gap of both
    /// spans' ends. Polished, every foot is the joint, and every witness
    /// lies within the zero band of the joint or past the band from it.
    /// Red where the seeds are taken as feet.
    #[test]
    fn the_walks_witnesses_stand_on_polished_feet() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let spline = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2)
                    .unwrap(),
                vec![
                    Point3::new(-1.0, -1.0, 2.0),
                    Point3::new(-0.4, -1.0, 1.3),
                    Point3::new(0.0, 0.0, 0.0),
                ],
                vec![1.0; 3],
            )
            .unwrap(),
        ));
        let edge = Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::new(0.0, 1.0, 0.0),
        };
        let (a, b) = (Span::new(&spline, (0.0, 1.0)), Span::new(&edge, (0.0, 1.0)));
        let points = witnesses(&Read::of(&a), &Read::of(&b), band).unwrap();
        let off =
            |q: &Point3<f64>| decide("test_witness", Margin::norm3(*q - Point3::origin()), band);
        assert!(
            points.iter().any(|q| off(q) == Ok(Sign::Zero)),
            "a witness at the joint: {points:?}"
        );
        for q in &points {
            assert!(off(q).is_ok(), "{q:?} lies in the band's gap of the joint");
        }
    }
}
