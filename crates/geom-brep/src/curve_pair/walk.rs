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
//! at it. An endless span ([`Span::endless`]) has no ends ([`Span::ends`]):
//! none is read, and none anchors a witness.
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
//! stops is polished toward the meeting by Gauss–Newton held to the two
//! spans (`polish::pair_held`; an endless span holds nothing), and the
//! polished pair is read as that pair (`Found::read_pair`): its own gap
//! and each parameter's place on its span, with no foot, under the
//! window the pieces make: the pair's
//! gap bound and reach (`pose`'s module docs, "The gap's lower bound"),
//! every pair of the pieces within the pieces' reach and the polish's
//! run of it, and the curvature at most the carriers' bounds. An Off at
//! the polished pair prunes only where that bound says the pieces hold
//! no meeting of the spans. Where the two-sided premise is not proved
//! (`curve_pair_walk_basin`), a pair held at an end of each span reads
//! the one-sided window (`curve_pair_walk_basin_held`, [`window`]);
//! where neither is proved the window is undecided and the Off
//! escalates. Every piece the walk does not clear ends at such a pair,
//! so no meeting is left unread; an escalated clearing keeps its piece. Every point read is read under the reader's stretch rule
//! (the parent module's docs). A walk past
//! [`geom_core::bisect::PIECE_BUDGET`] refuses typed
//! ([`Unsettled::Exhausted`]).

use geom::{Curve3, PieceStep};
use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::{Pick, choose, decide};
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

    /// A bound on the curvature over every arc within `reach` metres of
    /// `u` along the carrier: over the parameters `reach` at the
    /// carrier's least speed covers ([`geom::carrier_speed::speed_range`],
    /// the whole carrier's, since `u` need not lie on the span), read by
    /// [`geom::carrier_speed::curvature_hi_over`], so a spline's crease
    /// costs only the windows that may hold it. A least speed not above
    /// zero covers every parameter, and reads the whole carrier's bound.
    fn curvature_near(&self, u: T, reach: T) -> T {
        let lo = geom::carrier_speed::speed_range(self.curve()).0.get();
        let width = reach / lo.max(T::zero());
        if !geom_core::is_finite_length(width) {
            return self.curvature;
        }
        geom::carrier_speed::curvature_hi_over(self.curve(), (u - width, u + width))
    }

    /// [`Self::curvature_near`] on one side of `u` only, the parameters
    /// above it (`above`) or below it: the arcs of a window held at a
    /// span end, which all lie on the span's side of the end. A crease
    /// exactly at `u` is clear of such a run
    /// ([`geom::NurbsCurve3::accel_upper_bound_over`]), so a window held
    /// at a creased end reads the curvature of its own side.
    fn curvature_into(&self, u: T, reach: T, above: bool) -> T {
        let lo = geom::carrier_speed::speed_range(self.curve()).0.get();
        let width = reach / lo.max(T::zero());
        if !geom_core::is_finite_length(width) {
            return self.curvature;
        }
        let run = if above {
            (u, u + width)
        } else {
            (u - width, u)
        };
        geom::carrier_speed::curvature_hi_over(self.curve(), run)
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
/// A heuristic's choice ([`geom_core::k_stats::choose`], ledger row F35),
/// not a `<`, because an enclosure that straddles it is neither taken nor
/// dropped by a `<`. **At a tie the step is dropped**: sound for the
/// reason above. A dropped step leaves the point unpolished, and the
/// window read at it (module docs) prunes only where its premise is proved,
/// so an unproved premise escalates the Off rather than losing a meeting.
fn step_taken<T: Decide>(moved: T, slack: T) -> bool {
    match choose(
        "curve_pair_walk_polish_step",
        Margin::of(slack * T::from_f64(16.0) - moved),
        "F35",
    ) {
        Pick::Positive => true,
        Pick::Negative | Pick::Tie => false,
    }
}

/// Whether a piece of reach `reach` is small enough to stop at, decided on
/// `row`: within the floor, or within the band where there is no finite
/// floor.
fn stops<T: Decide>(row: &'static str, reach: T, floor: Option<T>, band: Band) -> bool {
    let within_as = |margin: T| {
        matches!(
            decide(row, Margin::of(margin), band),
            Ok(Sign::Negative | Sign::Zero)
        )
    };
    match floor {
        Some(f) if geom_core::is_finite_length(f) => within_as(reach - f),
        Some(_) | None => within_as(reach),
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
                Err(refusal) => held = held.or(Some(refusal.refused())),
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
        let t = r.curve().foot(q, seed).seed();
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

/// **The window of a polished piece pair** (`pose`'s module docs, "The
/// gap's lower bound"): every pair of the pieces lies within `span`
/// metres, along its carrier, of the polished parameter on it (its run
/// to the piece's middle and the piece's own reach), and the carriers'
/// curvature is at most their bounds over the arcs within `span` of the
/// pair (`Read::curvature_near`), the premise (P2) states. Where the
/// sine at the pair, less the turn over `span` twice, is not proved
/// positive (`curve_pair_walk_basin`), or whose move is not proved, the
/// two-sided reading bounds no reach. There, a pair held at an end of
/// each span ([`held`]) reads the one-sided window (`pose`'s module
/// docs, "The one-sided window"): every piece lies inside its span, so
/// every pair of the window lies on the side of each end its span runs
/// to. Each end reads its own span's half-tangent, the one-sided
/// derivative from inside the span ([`geom::Curve3::ders1_from`]), so a
/// crease at the end turns no tangent, and the curvature over that side
/// alone (`Read::curvature_into`), whose run a crease at the end is clear
/// of (`curve_pair_walk_basin_held`). Otherwise the window is undecided.
fn window<T: Decide>(
    (ra, pa, s): (&Read<'_, '_, T>, Piece<T>, T),
    (rb, pb, t): (&Read<'_, '_, T>, Piece<T>, T),
    band: Band,
) -> super::WindowBound<T> {
    let run = |r: &Read<'_, '_, T>, p: Piece<T>, u: T| {
        Margin::metered_sup(u - p.mid(), r.hi()).value().abs() + p.reach(r.hi())
    };
    let span = run(ra, pa, s).max(run(rb, pb, t));
    let e = T::from_f64(band.escalate());
    let curvature = ra.curvature_near(s, span) + rb.curvature_near(t, span);
    let two = super::polish::bound(
        ("curve_pair_walk_basin", None),
        super::polish::Along::two_sided(ra.curve().ders1(s), rb.curve().ders1(t)),
        curvature,
        (e, span),
    );
    // A two-sided reading that bounds its reach stands; one whose basin
    // or move is not proved gives way to the one-sided reading where the
    // pair is held at an end of each span.
    if two.is_some_and(|b| geom_core::is_finite_length(b.reach)) {
        return two.unwrap_or_else(super::polish::undecided);
    }
    let one = || {
        let (above_a, above_b) = (held(ra.span, s)?, held(rb.span, t)?);
        // Each end's half-tangent is its own span's one-sided
        // derivative, turned to point into the span, and the curvature
        // is read over the span's side of the end alone.
        let into = |r: &Read<'_, '_, T>, u: T, above: bool| {
            let (p, d) = r.curve().ders1_from(u, !above);
            (p, if above { d } else { -d })
        };
        let curvature = ra.curvature_into(s, span, above_a) + rb.curvature_into(t, span, above_b);
        super::polish::bound(
            ("curve_pair_walk_basin_held", None),
            super::polish::Along::one_sided(into(ra, s, above_a), into(rb, t, above_b)),
            curvature,
            (e, span),
        )
    };
    one()
        .filter(|b| geom_core::is_finite_length(b.reach))
        .or(two)
        .unwrap_or_else(super::polish::undecided)
}

/// **A polished parameter at its span's end is that end**: where the
/// carrier's point at `u` is within the band of an end's point
/// ([`Span::place_param`], the place the pair read reads), the polish is
/// held there and `u` is the end's own parameter, so the pair names the
/// end the pair read finds and the window reads one-sided from it
/// ([`held`]). Otherwise, and where the place is not decided, `u` as
/// polished. A place to read, never a decision of its own.
fn at_end<T: Decide>(span: &Span<'_, T>, u: T, band: Band) -> T {
    let (t0, t1) = span.params();
    match span.place_param(u, band) {
        Ok(super::Place::End(super::SpanEnd::Start)) => t0,
        Ok(super::Place::End(super::SpanEnd::Stop)) => t1,
        _ => u,
    }
}

/// **Whether `u` is held at an end of `span`**: `u` is exactly one of
/// the span's end parameters (`curve_pair_walk_held_end`), and the span
/// has ends. Read as which side of `u` the span lies on, `true` for the
/// parameters above it, from the order of the span's two ends
/// (`curve_pair_walk_span_order`, asked only at an end); `None` at no
/// end, on an endless span, and where the order is not decided.
fn held<T: Decide>(span: &Span<'_, T>, u: T) -> Option<bool> {
    if !span.ended() {
        return None;
    }
    let at = |end: T| super::compare("curve_pair_walk_held_end", u, end) == Some(Sign::Zero);
    let (t0, t1) = span.params();
    let first = if at(t0) {
        true
    } else if at(t1) {
        false
    } else {
        return None;
    };
    let rising = match super::compare("curve_pair_walk_span_order", t0, t1)? {
        Sign::Negative => true,
        Sign::Positive => false,
        Sign::Zero => return None,
    };
    Some(first == rising)
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
        let n = n.normalize();
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
        let mut ends = Vec::new();
        for (_, q) in other.span.ends() {
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
    // Each span's ends against the other, first: none of an endless one.
    for (r, other, first) in [(&ra, &rb, true), (&rb, &ra, false)] {
        for (t, q) in r.span.ends() {
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
                let hold = |span: &Span<'_, T>| span.ended().then(|| span.params());
                let (s, t) = super::polish::pair_held(
                    |s| ra.curve().ders1(s),
                    |t| rb.curve().ders1(t),
                    (ma, mb),
                    |m| step_taken(m, slack),
                    (hold(a), hold(b)),
                );
                let (s, t) = (at_end(a, s, band), at_end(b, t, band));
                let candidate = super::Candidate {
                    point: ra.curve().eval(s),
                    ta: s,
                    tb: t,
                    bound: Some(window((&ra, pa, s), (&rb, pb, t), band)),
                };
                found.read_pair(a, b, candidate, band);
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

    use super::{Piece, Read, step_taken, window, witnesses};
    use crate::curve_pair::Span;

    /// **The one-sided window's reach, checked against its pairs**: the
    /// window of the pieces `pa` of `a` and `pb` of `b`, held at `s` and
    /// `t` (each an end of its span and of its piece), and every sampled
    /// pair of the pieces within the band's escalation of each other, by
    /// arc length from the held ends (cumulative chords over `n` samples
    /// of each piece). Every such pair lies within the reach where the
    /// window bounds one: the bound's consequence, not its verdict log.
    /// The reach is returned, `+∞` where unbounded.
    fn reach_covers_its_pairs(
        (a, pa, s): (&Span<'_, f64>, (f64, f64), f64),
        (b, pb, t): (&Span<'_, f64>, (f64, f64), f64),
        band: Band,
        n: usize,
    ) -> f64 {
        let (ra, rb) = (Read::of(a), Read::of(b));
        let piece = |(x, y): (f64, f64)| Piece {
            a: x,
            b: y,
            depth: 0,
        };
        let bound = window((&ra, piece(pa), s), (&rb, piece(pb), t), band);
        // Samples of a piece from its held end outward, each with its arc
        // length from that end.
        let walk = |c: &Curve3<f64>, (x, y): (f64, f64), from: f64| {
            let to = if from == x { y } else { x };
            let mut out = Vec::with_capacity(n + 1);
            let mut arc = 0.0;
            let mut last = c.eval(from);
            #[allow(clippy::cast_precision_loss)]
            for i in 0..=n {
                let p = c.eval(from + (to - from) * i as f64 / n as f64);
                arc += (p - last).norm();
                last = p;
                out.push((p, arc));
            }
            out
        };
        let (sa, sb) = (walk(a.carrier(), pa, s), walk(b.carrier(), pb, t));
        let e = band.escalate();
        let mut far = 0.0f64;
        for &(p, arc_a) in &sa {
            for &(q, arc_b) in &sb {
                if (p - q).norm() <= e {
                    far = far.max(arc_a.max(arc_b));
                }
            }
        }
        if bound.reach.is_finite() {
            assert!(
                far <= bound.reach,
                "a pair within the band lies {far:e} from the held ends, past the reach {:e}",
                bound.reach
            );
        }
        bound.reach
    }

    /// **A held window with a gap reads the gap as its slack.** Two arcs of
    /// the unit circle, the first ending at angle `0` and the second
    /// starting at `−δ`, `δ = 10⁻⁴`, before it: each held at its span's
    /// end, their half-tangents part by `180°`, and the arcs lie on each
    /// other over `δ`, so pairs within the band reach `δ` from the held
    /// ends while the pair's own gap is the chord of `δ`. The two-sided
    /// sine is `sin δ`, which proves no basin; the one-sided window reads
    /// `g = 1`, the slack `|q − p|`, and bounds a reach covering every
    /// such pair. Red where the slack drops the gap (the reach falls to
    /// about the band's escalation).
    #[test]
    fn a_held_window_with_a_gap_reaches_its_overlap() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let circle = super::super::meet_rows::circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
        let delta = 1e-4;
        let (a, b) = (
            Span::new(&circle, (-0.3, 0.0)),
            Span::new(&circle, (-delta, 0.3)),
        );
        let reach = reach_covers_its_pairs(
            (&a, (-0.01, 0.0), 0.0),
            (&b, (-delta, 0.01), -delta),
            band,
            4000,
        );
        assert!(reach.is_finite(), "the one-sided window bounds its reach");
    }

    /// **A held window long enough to come round again proves no basin.**
    /// Two arcs of the unit circle leaving angle `0` in opposite
    /// directions, each `3.3` rad long, so they lie on each other again
    /// past `π`, about `3` metres from the held ends. The half-tangents
    /// part by `180°` (`g = 1`), but the curvature over the window,
    /// `K = 2` over a span of `3.3`, takes it past the basin: no reach is
    /// bounded, and none is claimed short of the far pairs. Red where the
    /// basin drops its turn over the span (`K·span`).
    #[test]
    fn a_held_window_that_comes_round_again_bounds_no_reach() {
        // A band wide enough that the samples' spacing (about `10⁻³` rad)
        // finds the far pairs within it.
        let band = Band::new(1e-3, 1e-2).unwrap();
        let circle = super::super::meet_rows::circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
        let (a, b) = (
            Span::new(&circle, (-3.3, 0.0)),
            Span::new(&circle, (0.0, 3.3)),
        );
        reach_covers_its_pairs((&a, (-3.3, 0.0), 0.0), (&b, (0.0, 3.3), 0.0), band, 3000);
    }

    /// **A held window at a crease reads each span's own half-tangent.** A
    /// degree-2 spline creased at `0.5` (a double knot), its two legs
    /// leaving the crease `20°` apart and bending slightly, read as two
    /// spans meeting there. `ders1` at the crease is one side's
    /// derivative; read for both ends it puts the half-tangents `180°`
    /// apart (`g = 1`) where the legs part by `20°` (`g = sin 20°`), and
    /// the pairs within the band (`e = 10⁻³`), which reach about
    /// `e/sin 20°` along the legs, fall past the reach. Each end reads its
    /// own span's one-sided derivative and the curvature over its own side
    /// of the crease, and the reach covers them. Red where both ends read
    /// `ders1`; and where the curvature is read across the crease the
    /// bound is infinite, which the row refuses too.
    #[test]
    fn a_held_window_at_a_crease_reads_each_spans_own_half_tangent() {
        use geom_core::spline::{KnotVector, Knots};
        let band = Band::new(1e-4, 1e-3).unwrap();
        let (c, s) = (10f64.to_radians().cos(), 10f64.to_radians().sin());
        let crease = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(
                    KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
                ),
                vec![
                    Point3::new(-0.2 * c, 0.2 * s + 0.004, 0.0),
                    Point3::new(-0.1 * c, 0.1 * s, 0.0),
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(-0.1 * c, -0.1 * s, 0.0),
                    Point3::new(-0.2 * c, -0.2 * s - 0.004, 0.0),
                ],
                vec![1.0; 5],
            )
            .unwrap(),
        ));
        let (a, b) = (
            Span::new(&crease, (0.0, 0.5)),
            Span::new(&crease, (0.5, 1.0)),
        );
        let reach =
            reach_covers_its_pairs((&a, (0.45, 0.5), 0.5), (&b, (0.5, 0.55), 0.5), band, 4000);
        assert!(reach.is_finite(), "the one-sided window bounds its reach");
    }

    /// **F35's tie drops the step.** A step that moves its point exactly
    /// `16·slack` is a decided zero against its bound at `f64`, and an
    /// enclosure astride the bound is undecided: each is dropped, as a
    /// step past the bound is, while one short of it is taken. None of
    /// it reaches the verdict log. Red where the tie keeps the step.
    #[test]
    fn a_polish_step_at_its_bound_or_astride_it_is_dropped() {
        use geom_core::Interval;
        let bracket = geom_core::k_stats::Bracket::open();
        assert!(step_taken(15.0_f64, 1.0));
        assert!(!step_taken(16.0_f64, 1.0), "at the bound");
        assert!(!step_taken(17.0_f64, 1.0));
        let one = Interval::from_bounds(1.0, 1.0);
        assert!(step_taken(Interval::from_bounds(15.0, 15.5), one));
        assert!(
            !step_taken(Interval::from_bounds(15.5, 16.5), one),
            "astride the bound"
        );
        let recorded = bracket.finish();
        assert!(recorded.verdicts.is_empty() && recorded.escalations.is_empty());
    }

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
        let edge = Curve3::line(Point3::origin(), Vec3::new(0.0, 1.0, 0.0));
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
