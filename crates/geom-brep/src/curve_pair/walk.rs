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
//! no meeting of the spans. That Off reads a foot: the window's reach
//! is placed on each span through `Span::past_by`, which reads
//! `Span::place` and so [`Curve3::foot`]. Where the two-sided reading
//! bounds no reach (its basin, `curve_pair_walk_basin`, or its move not
//! proved), a pair
//! held at an end of each span reads the one-sided window
//! (`curve_pair_walk_basin_held`, [`window`]); where neither bounds a
//! reach the window is undecided and the Off escalates. Every piece the
//! walk does not clear ends at such a pair, so no meeting is left
//! unread; an escalated clearing keeps its piece. Every point read is
//! read under the reader's stretch rule (the parent module's docs). A
//! walk past [`geom_core::bisect::PIECE_BUDGET`] refuses typed
//! ([`Unsettled::Exhausted`]).

use geom::{Curve3, ParamSide, PieceStep};
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
    /// `u` along the carrier, on both sides of it ([`Arcs::Around`]) or on
    /// one ([`Arcs::Toward`]): over the
    /// parameters `reach` at the carrier's least speed covers
    /// ([`geom::carrier_speed::speed_range`], the whole carrier's, since
    /// `u` need not lie on the span), read by
    /// [`geom::carrier_speed::curvature_hi_over`], so a spline's crease
    /// costs only the windows that may hold it. One side is the arcs of a
    /// window held at a span end, which all lie on the span's side of the
    /// end; a crease exactly at `u` is clear of that run
    /// ([`geom::NurbsCurve3::accel_upper_bound_over`]), so a window held at
    /// a creased end reads the curvature of its own side. A least speed
    /// not above zero covers every parameter, and reads the whole
    /// carrier's bound.
    fn curvature_near(&self, u: T, reach: T, arcs: Arcs) -> T {
        let lo = geom::carrier_speed::speed_range(self.curve()).0.get();
        let width = reach / lo.max(T::zero());
        if !geom_core::is_finite_length(width) {
            return self.curvature;
        }
        let run = match arcs {
            Arcs::Around => (u - width, u + width),
            Arcs::Toward(ParamSide::Above) => (u, u + width),
            Arcs::Toward(ParamSide::Below) => (u - width, u),
        };
        geom::carrier_speed::curvature_hi_over(self.curve(), run)
    }
}

/// **Which arcs about a parameter a curvature bound reads**
/// (`Read::curvature_near`).
#[derive(Clone, Copy)]
enum Arcs {
    /// Both sides of it: a two-sided window.
    Around,
    /// One side only: a window held at a span's end, whose arcs all lie
    /// on its span's side.
    Toward(ParamSide),
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
/// alone (`Read::curvature_near` on that side), whose run a crease at the end is clear
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
    let curvature =
        ra.curvature_near(s, span, Arcs::Around) + rb.curvature_near(t, span, Arcs::Around);
    let two = super::polish::bound(
        ("curve_pair_walk_basin", None),
        super::polish::Along::two_sided(ra.curve().ders1(s), rb.curve().ders1(t)),
        curvature,
        (e, span),
    );
    // A two-sided reading that bounds its reach stands; one whose basin
    // or move is not proved gives way to the one-sided reading where the
    // pair is held at an end of each span.
    if let Some(bound) = two.filter(|b| geom_core::is_finite_length(b.reach)) {
        return bound;
    }
    let one = || {
        let (side_a, side_b) = (held(ra.span, s)?, held(rb.span, t)?);
        // Each end reads its span's side: its half-tangent is the
        // one-sided derivative from that side, turned to point into the
        // span, and the curvature is read over that side alone.
        let into = |r: &Read<'_, '_, T>, u: T, side: ParamSide| {
            let (p, d) = r.curve().ders1_from(u, side);
            (p, if side == ParamSide::Above { d } else { -d })
        };
        let curvature = ra.curvature_near(s, span, Arcs::Toward(side_a))
            + rb.curvature_near(t, span, Arcs::Toward(side_b));
        super::polish::bound(
            ("curve_pair_walk_basin_held", None),
            super::polish::Along::one_sided(into(ra, s, side_a), into(rb, t, side_b)),
            curvature,
            (e, span),
        )
    };
    // Where the one-sided reading bounds no reach either, the two-sided
    // result stands as it is: a basin proved but its move not, whose
    // bound is undecided (`polish::undecided`, the reach `+∞`), or none.
    one()
        .filter(|b| geom_core::is_finite_length(b.reach))
        .or(two)
        .unwrap_or_else(super::polish::undecided)
}

/// **A polished parameter at its span's end is that end**: where the
/// carrier's point at `u` is within the band of an end's point
/// ([`Span::place_param`]), `u` is that end's own parameter, so the pair
/// names the end the pair read finds and the window reads one-sided from
/// it ([`held`]). Of the two ends it takes the one whose parameter is
/// nearer `u` (`curve_pair_walk_nearer_end`, ledger row F46): on a span
/// closed on itself the place reads either end, and the far one would put
/// the pair a whole span from its pieces. The snapped parameter's place
/// is that end's ([`super::Place::End`] of the end taken), so the
/// parameter and the end the pair read reports are one: the place read at
/// `u` names the first end whose chord it found zero, which on a carrier
/// that comes back through an end's point mid-span need not be the end
/// taken. Elsewhere, and where the place is not decided, `u` as polished.
/// The place is read once, held apart with its record
/// ([`super::PlaceRead`]), for the pair read to use.
fn at_end<T: Decide>(span: &Span<'_, T>, u: T, band: Band) -> (T, super::PlaceRead) {
    let (place, record) = geom_core::k_stats::detached(|| span.place_param(u, band));
    let at_an_end = matches!(place, Ok(super::Place::End(_)));
    let read = super::PlaceRead { place, record };
    if !at_an_end {
        return (u, read);
    }
    let (t0, t1) = span.params();
    // A choice between two sound places, the ends' parameters: either
    // anchors a window the pair's own readings decide, and the ring and
    // the window refuse what a far anchor leaves undecided.
    let nearer_first = choose(
        "curve_pair_walk_nearer_end",
        Margin::of((u - t1).abs() - (u - t0).abs()),
        "F46",
    );
    let (end, which) = match nearer_first {
        Pick::Positive | Pick::Tie => (t0, super::SpanEnd::Start),
        Pick::Negative => (t1, super::SpanEnd::Stop),
    };
    // The snapped parameter and the place name one end: the place read
    // at `u` may name the other, whose chord it tested first.
    (
        end,
        super::PlaceRead {
            place: Ok(super::Place::End(which)),
            record: read.record,
        },
    )
}

/// **The stop's reading of its polished pair `(s, t)`** of the pieces
/// `pa` and `pb` (module docs): each parameter at its span's end snapped
/// to that end with its place ([`at_end`]), the window about the pair
/// ([`window`]), and the pair read as a pair ([`Found::read_pair`]).
fn read_polished<T: Decide>(
    found: &mut Found<T>,
    (ra, pa): (&Read<'_, '_, T>, Piece<T>),
    (rb, pb): (&Read<'_, '_, T>, Piece<T>),
    (s, t): (T, T),
    band: Band,
) {
    let ((s, on_a), (t, on_b)) = (at_end(ra.span, s, band), at_end(rb.span, t, band));
    let candidate = super::Candidate {
        point: ra.curve().eval(s),
        ta: s,
        tb: t,
        bound: Some(window((ra, pa, s), (rb, pb, t), band)),
    };
    found.read_pair(ra.span, rb.span, candidate, (on_a, on_b), band);
}

/// **Whether `u` is held at an end of `span`**: `u` is exactly one of
/// the span's end parameters (`curve_pair_walk_held_end`), and the span
/// has ends. Read as which side of `u` the span lies on, from the order
/// of the span's two ends
/// (`curve_pair_walk_span_order`, asked only at an end); `None` at no
/// end, on an endless span, and where the order is not decided.
pub(super) fn held<T: Decide>(span: &Span<'_, T>, u: T) -> Option<ParamSide> {
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
    Some(if first == rising {
        ParamSide::Above
    } else {
        ParamSide::Below
    })
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
                let polished = super::polish::pair_held(
                    |s| ra.curve().ders1(s),
                    |t| rb.curve().ders1(t),
                    (ma, mb),
                    |m| step_taken(m, slack),
                    (hold(a), hold(b)),
                );
                read_polished(&mut found, (&ra, pa), (&rb, pb), polished, band);
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

    use super::{Piece, Read, at_end, read_polished, step_taken, window, witnesses};
    use crate::curve_pair::Span;

    /// **The one-sided window's reach, checked against its pairs**: the
    /// window of the pieces `pa` of `a` and `pb` of `b`, held at `s` and
    /// `t` (each an end of its span and of its piece), and every sampled
    /// pair of the pieces within the band's escalation of each other, by
    /// its arc length from the held ends (summed chords over `n` samples
    /// of each piece, widened past a chord's shortfall). Every such pair
    /// lies within the reach where the window bounds one: the bound's
    /// consequence, not its verdict log.
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
        // length from that end: the summed chords, widened by `10⁻⁶`. A
        // chord under-reads its arc by a relative `(κh)²/24` at most, `κ`
        // the curvature over the step and `h` its arc, so the widened sum
        // is never less than the arc wherever `κh ≤ 4.8·10⁻³`; each piece
        // asserts that premise, from its curvature bound
        // (`curvature_hi_over`) and its longest step.
        let walk = |c: &Curve3<f64>, (x, y): (f64, f64), from: f64| {
            let to = if from == x { y } else { x };
            let mut out = Vec::with_capacity(n + 1);
            let (mut arc, mut step) = (0.0, 0.0f64);
            let mut last = c.eval(from);
            #[allow(clippy::cast_precision_loss)]
            for i in 0..=n {
                let p = c.eval(from + (to - from) * i as f64 / n as f64);
                let chord = (p - last).norm();
                arc += chord;
                step = step.max(chord * (1.0 + 1e-6));
                last = p;
                out.push((p, arc * (1.0 + 1e-6)));
            }
            let kappa = geom::carrier_speed::curvature_hi_over(c, (x.min(y), x.max(y)));
            assert!(
                kappa * step <= 4.8e-3,
                "the chords' margin holds only for κh ≤ 4.8e-3: κ {kappa:e}, h {step:e}"
            );
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

    /// **A polished parameter within the band of its span's end is that
    /// end.** On the segment `x ∈ [0, 1]` of the `x` axis, read at the
    /// band `(10⁻⁹, 10⁻⁸)`: a parameter `10⁻¹²` short of an end is
    /// that end's own parameter, bit for bit, the `End` the pair read
    /// reads; one `10⁻³` short, or in the middle, is itself. Red where
    /// the polish's place is kept short of the end.
    #[test]
    fn a_polished_parameter_at_its_end_is_the_end() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let x = Curve3::line(Point3::origin(), Vec3::new(1.0, 0.0, 0.0));
        let span = Span::new(&x, (0.0, 1.0));
        assert_eq!(
            at_end(&span, 1.0 - 1e-12, band).0.to_bits(),
            1.0f64.to_bits()
        );
        assert_eq!(at_end(&span, 1e-12, band).0.to_bits(), 0.0f64.to_bits());
        assert_eq!(at_end(&span, 1.0 - 1e-3, band).0, 1.0 - 1e-3);
        assert_eq!(at_end(&span, 0.5, band).0, 0.5);
    }

    /// **On a span closed on itself, a polished parameter is the end on its
    /// own side.** The unit circle's whole turn, `(0, 2π)`, whose two ends
    /// are one point: a parameter `10⁻¹²` short of `2π` is `2π`, and one
    /// `10⁻¹²` past `0` is `0`. Red where the place's first end is taken,
    /// which puts `2π − 10⁻¹²` at `0`, a whole turn from the pieces it was
    /// polished in.
    #[test]
    fn a_polished_parameter_on_a_closed_span_is_the_end_on_its_side() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let circle = super::super::meet_rows::circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
        let turn = std::f64::consts::TAU;
        let span = Span::new(&circle, (0.0, turn));
        assert_eq!(
            at_end(&span, turn - 1e-12, band).0.to_bits(),
            turn.to_bits()
        );
        assert_eq!(at_end(&span, 1e-12, band).0.to_bits(), 0.0f64.to_bits());
    }

    /// A degree-2 spline creased at `0.5` whose own leg below the crease
    /// curves strongly (`y = 5x²` from the crease at the origin, leaving
    /// along `+x`) and whose other leg runs straight down `−y`, square to
    /// it; and a line from the crease at `20°` above `+x`. Read held at
    /// the spline's last end (its span `(0, 0.5)` falls into the crease)
    /// and the line's first (it rises), the two ends' sides differ, the
    /// two sides of the crease differ in direction and in curvature, and
    /// only the span's own side is the window's.
    fn crease_and_line() -> (Curve3<f64>, Curve3<f64>) {
        use geom_core::spline::{KnotVector, Knots};
        let crease = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(
                    KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
                ),
                vec![
                    Point3::new(0.2, 0.2, 0.0),
                    Point3::new(0.1, 0.0, 0.0),
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(0.0, -0.1, 0.0),
                    Point3::new(0.0, -0.2, 0.0),
                ],
                vec![1.0; 5],
            )
            .unwrap(),
        ));
        let (s, c) = 20f64.to_radians().sin_cos();
        (crease, Curve3::line(Point3::origin(), Vec3::new(c, s, 0.0)))
    }

    /// **A held window reads each end on its own span's side.** The
    /// spline of [`crease_and_line`], held falling at its crease, against
    /// the line held rising at the same point, over short pieces of each
    /// (about `4·10⁻³` m), at `e = 10⁻⁴`. The legs part from the line by
    /// `20°` on the span's own side and `110°` on the other, so the
    /// half-tangents read from the wrong side, or turned the wrong way,
    /// give `g ≈ 0.94` where the window's is `sin 20°`, and its reach
    /// falls short of the pairs within the band, which reach about
    /// `e/sin 20°` and further as the own leg bends toward the line. The
    /// window bounds a reach covering them. Red where `ders1_from` reads
    /// the other side, where `held` names the other side, and where the
    /// curvature is read over the other side (straight, so `K = 0`).
    #[test]
    fn a_held_window_reads_each_end_on_its_own_spans_side() {
        let band = Band::new(1e-5, 1e-4).unwrap();
        let (crease, line) = crease_and_line();
        let (a, b) = (Span::new(&crease, (0.0, 0.5)), Span::new(&line, (0.0, 1.0)));
        let reach =
            reach_covers_its_pairs((&a, (0.49, 0.5), 0.5), (&b, (0.0, 4e-3), 0.0), band, 4000);
        assert!(reach.is_finite(), "the one-sided window bounds its reach");
    }

    /// **A held window reads the curvature of its own side.** The pair of
    /// [`a_held_window_reads_each_end_on_its_own_spans_side`] over the
    /// spline's whole own leg and `0.4` m of the line: the own leg bends
    /// back across the line at `x ≈ 0.073`, about `0.08` m from the held
    /// ends. Its curvature (about `10` at the crease) over the window
    /// takes the basin past proving, so no reach is bounded, and none is
    /// claimed short of the far crossing. Read over the other, straight
    /// side, `K` is zero, the basin proves, and the reach claimed is
    /// about `e/sin 20°`, short of the crossing: the row is red there.
    #[test]
    fn a_held_window_reads_the_curvature_of_its_own_side() {
        let band = Band::new(1e-5, 1e-4).unwrap();
        let (crease, line) = crease_and_line();
        let (a, b) = (Span::new(&crease, (0.0, 0.5)), Span::new(&line, (0.0, 1.0)));
        let reach =
            reach_covers_its_pairs((&a, (0.0, 0.5), 0.5), (&b, (0.0, 0.4), 0.0), band, 4000);
        assert!(!reach.is_finite(), "no reach is bounded: {reach:e}");
    }

    /// **A two-sided basin whose move fails gives way to the one-sided
    /// window.** Two unit circles in one plane, the first's arc ending at
    /// its angle `0` and the second's starting `0.1` m along `+x` from that
    /// end at its angle `30°`: their half-tangents into the arcs part by
    /// `150°`, so the two-sided sine is `½` and the one-sided `g` is `1`.
    /// Over arcs `0.05` m long (`K·span ≈ 0.1`), the two-sided basin
    /// proves (`½ − 2·0.1 > 0`) but its move does not
    /// (`0.4² < 2·2·0.1`); the one-sided move does (`0.9² > 0.4`), and
    /// the window bounds a reach. Red where a proved two-sided basin
    /// stands whatever its move.
    #[test]
    fn a_two_sided_basin_whose_move_fails_gives_way_to_the_one_sided_window() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let first = super::super::meet_rows::circle([-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
        let (s30, c30) = 30f64.to_radians().sin_cos();
        let second = super::super::meet_rows::circle([0.1 - c30, -s30, 0.0], [0.0, 0.0, 1.0], 1.0);
        let start = 30f64.to_radians();
        let (a, b) = (
            Span::new(&first, (-0.05, 0.0)),
            Span::new(&second, (start, start + 0.05)),
        );
        let reach = reach_covers_its_pairs(
            (&a, (-0.05, 0.0), 0.0),
            (&b, (start, start + 0.05), start),
            band,
            1000,
        );
        assert!(
            reach.is_finite(),
            "the one-sided window bounds its reach: {reach:e}"
        );
    }

    /// **A mid-span parameter whose point is an end's is snapped by
    /// parameter, and its place is the end it snaps to.** A degree-1
    /// polyline through `(0, 0)`, `(1, 0)`, `(1, 1)`, back through `(0, 0)`
    /// and on to `(0, −1)`, over the uniform knots, so its parameter
    /// `0.75` is the start's point mid-span. The place reads `0.75` at an
    /// end (its chord to the start is zero), and the snap takes the end
    /// whose parameter is nearer, `1`, whose point is `(0, −1)`. Read
    /// through the stop's own composition ([`read_polished`]) against a
    /// line through `(0, −1)`: the meeting is at `(0, −1)` and names the
    /// polyline's stop end. Red where the snapped parameter keeps the
    /// place read before the snap, which names the start, `(0, 0)`: a
    /// wrong definite answer.
    #[test]
    fn a_mid_span_parameter_at_an_ends_point_snaps_by_parameter() {
        use geom_core::spline::{KnotVector, Knots};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let loop_back = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(
                    KnotVector::clamped(vec![0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0], 1).unwrap(),
                ),
                vec![
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(1.0, 0.0, 0.0),
                    Point3::new(1.0, 1.0, 0.0),
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(0.0, -1.0, 0.0),
                ],
                vec![1.0; 5],
            )
            .unwrap(),
        ));
        let span = Span::new(&loop_back, (0.0, 1.0));
        assert_eq!(at_end(&span, 0.75, band).0.to_bits(), 1.0f64.to_bits());
        let across = Curve3::line(Point3::new(0.0, -1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let other = Span::new(&across, (-1.0, 1.0));
        let (ra, rb) = (Read::of(&span), Read::of(&other));
        let mut found = crate::curve_pair::Found::default();
        read_polished(
            &mut found,
            (&ra, Piece::whole(span.params())),
            (&rb, Piece::whole(other.params())),
            (0.75, 0.0),
            band,
        );
        assert!(
            matches!(
                found.points.as_slice(),
                [m] if m.on_a == crate::curve_pair::Hit::End(crate::curve_pair::SpanEnd::Stop)
                    && (m.point - Point3::new(0.0, -1.0, 0.0)).norm() < 1e-12
            ),
            "the meeting is at the stop end it snapped to: {:?}",
            found.points
        );
    }

    /// **A tie between the two ends takes the start** (ledger row F46): a
    /// degree-1 polyline that comes back through its start's point at its
    /// span's middle, `0.5`, where the two ends' parameters are equally
    /// far. The place reads `0.5` at the start, the nearer-end choice ties,
    /// and the tie takes `t₀`, the start, its place the start's. Red where
    /// the tie takes the stop end.
    #[test]
    fn a_tie_between_the_ends_takes_the_start() {
        use geom_core::spline::{KnotVector, Knots};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let back = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(
                    KnotVector::clamped(vec![0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0], 1).unwrap(),
                ),
                vec![
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(1.0, 0.0, 0.0),
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(0.0, 1.0, 0.0),
                    Point3::new(0.0, -1.0, 0.0),
                ],
                vec![1.0; 5],
            )
            .unwrap(),
        ));
        let span = Span::new(&back, (0.0, 1.0));
        let (t, read) = at_end(&span, 0.5, band);
        assert_eq!(t.to_bits(), 0.0f64.to_bits(), "the tie takes the start");
        assert_eq!(
            read.place,
            Ok(crate::curve_pair::Place::End(
                crate::curve_pair::SpanEnd::Start
            ))
        );
    }

    /// The escalations a reading left on its frame, by predicate.
    fn escalated<R>(read: impl FnOnce() -> R) -> Vec<&'static str> {
        let (_, record) = geom_core::k_stats::detached(read);
        record
            .recorded()
            .escalations
            .iter()
            .map(geom_core::k_stats::Escalation::predicate)
            .collect()
    }

    /// The x axis over `(0, 1)`, and a line along `y` through the axis's
    /// point at `x`, lifted `z` off it, over `(−1, 1)`.
    fn axis_and_cross(x: f64, z: f64) -> (Curve3<f64>, Curve3<f64>) {
        (
            Curve3::line(Point3::origin(), Vec3::new(1.0, 0.0, 0.0)),
            Curve3::line(Point3::new(x, 0.0, z), Vec3::new(0.0, 1.0, 0.0)),
        )
    }

    /// **A pair read whose outcome rests on a place carries the place's
    /// escalation, and one whose outcome does not leaves none**
    /// ([`super::super::pair_both`]): the x axis's parameter `1 − 5·10⁻⁹`,
    /// its chord to the stop end in the band's gap, so its place refuses.
    /// Against a line crossing the axis there the gap is zero, the outcome
    /// is the place's refusal, and its escalation is on the frame. Against
    /// the same line lifted `10⁻⁶` off the axis the gap is positive, no
    /// outcome rests on the place, and its escalation is superseded. Red
    /// where a zero gap supersedes or drops the records, which leaves a
    /// refusal with no escalation behind it, and where a positive gap
    /// splices them.
    #[test]
    fn a_pair_read_carries_the_escalation_its_outcome_rests_on() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let ta = 1.0 - 5e-9;
        let chord = "curve_pair_span_end_chord";
        for (lift, carried) in [(0.0, true), (1e-6, false)] {
            let (x, cross) = axis_and_cross(ta, lift);
            let (a, b) = (Span::new(&x, (0.0, 1.0)), Span::new(&cross, (-1.0, 1.0)));
            let candidate = crate::curve_pair::Candidate {
                point: x.eval(ta),
                ta,
                tb: 0.0,
                bound: None,
            };
            let mut read = None;
            let left = escalated(|| {
                let places = (at_end(&a, ta, band).1, at_end(&b, 0.0, band).1);
                read = Some(super::super::pair_both(&a, &b, candidate, places, band).0);
            });
            assert_eq!(
                left.contains(&chord),
                carried,
                "lift {lift}: escalations {left:?}, read {:?}",
                read
            );
            if carried {
                assert!(
                    read.is_some_and(|r| r.is_err()),
                    "the place's refusal is the outcome"
                );
            }
        }
    }

    /// **The run arm reads only where the polished pair is in the run**
    /// (`pose`'s module docs, "The run"; [`super::super::window_off`]):
    /// the x axis over `(0, 1)` and a line square to it, read at a pair on
    /// the axis at `x = 1.5`, past the span, with a window of reach 10
    /// and no bend, so both of the axis span's ends are within reach.
    /// Where the line crosses the axis at `1.5` the pair's gap is zero, the
    /// pair is in the run, and each end's clearance from the line (0.5 and
    /// 1.5) clears it: no meeting, and nothing held. Where the line crosses
    /// at `0.5` instead, the pair at `1.5` is `1` off it, not in the run,
    /// and the reach alone clears no end: the window holds the crossing at
    /// `0.5`, and the Off escalates. Red where the run arm never reads (the
    /// first escalates), and where any decided gap counts as in the run (the
    /// second clears the crossing).
    #[test]
    fn the_run_arm_reads_only_where_the_pair_is_in_the_run() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let bound = crate::curve_pair::WindowBound {
            floor: f64::NEG_INFINITY,
            reach: 10.0,
            bend: 0.0,
        };
        for (crossing, cleared) in [(1.5, true), (0.5, false)] {
            let (x, cross) = axis_and_cross(crossing, 0.0);
            let (a, b) = (Span::new(&x, (0.0, 1.0)), Span::new(&cross, (-1.0, 1.0)));
            let tb = cross.param_near(x.eval(1.5), 0.0).unwrap();
            let candidate = crate::curve_pair::Candidate {
                point: x.eval(1.5),
                ta: 1.5,
                tb,
                bound: Some(bound),
            };
            let places = (at_end(&a, 1.5, band).1, at_end(&b, tb, band).1);
            let mut found = crate::curve_pair::Found::default();
            found.read_pair(&a, &b, candidate, places, band);
            assert!(
                found.points.is_empty(),
                "crossing {crossing}: {:?}",
                found.points
            );
            assert_eq!(
                found.held.is_empty(),
                cleared,
                "crossing {crossing}: held {:?}",
                found.held
            );
        }
    }

    /// **A pair read at a positive gap is no meeting.** Two parallel lines
    /// `10⁻⁶` apart, their pair read at the middle of each with no window
    /// bounded (floor `−∞`): the gap is decided positive, so the pair read
    /// finds no meeting, and with no floor to clear it the window's Off
    /// escalates. Red where a positive gap still reads each parameter's
    /// place as a side (the pair read would invent a meeting at a near
    /// miss).
    #[test]
    fn a_pair_read_at_a_positive_gap_is_no_meeting() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let x = Curve3::line(Point3::origin(), Vec3::new(1.0, 0.0, 0.0));
        let near = Curve3::line(Point3::new(0.0, 1e-6, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let (a, b) = (Span::new(&x, (0.0, 1.0)), Span::new(&near, (0.0, 1.0)));
        let candidate = crate::curve_pair::Candidate {
            point: x.eval(0.5),
            ta: 0.5,
            tb: 0.5,
            bound: Some(crate::curve_pair::polish::undecided()),
        };
        let places = (at_end(&a, 0.5, band).1, at_end(&b, 0.5, band).1);
        let mut found = crate::curve_pair::Found::default();
        found.read_pair(&a, &b, candidate, places, band);
        assert!(
            found.points.is_empty(),
            "no meeting at a near miss: {:?}",
            found.points
        );
        assert!(!found.held.is_empty(), "the undecided window escalates");
    }

    /// **A pair read past a span is off it.** The `x` axis's segment
    /// `x ∈ [0, 1]` and a line square to it through `(1.5, 0, 0)`: their
    /// pair at `(1.5, 0)` has a zero gap, but its first parameter lies past
    /// the first span, so the pair read finds no meeting. At the walk's
    /// stop the hold keeps both parameters on their spans and this arm is
    /// not reached; the pair read is a reading of any pair, and this pins
    /// it. Red where a parameter past its span reads as an end.
    #[test]
    fn a_pair_read_past_a_span_is_off_it() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let x = Curve3::line(Point3::origin(), Vec3::new(1.0, 0.0, 0.0));
        let up = Curve3::line(Point3::new(1.5, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let (a, b) = (Span::new(&x, (0.0, 1.0)), Span::new(&up, (-1.0, 1.0)));
        let candidate = crate::curve_pair::Candidate {
            point: x.eval(1.5),
            ta: 1.5,
            tb: 0.0,
            bound: None,
        };
        let places = (at_end(&a, 1.5, band).1, at_end(&b, 0.0, band).1);
        let (read, _) = crate::curve_pair::pair_both(&a, &b, candidate, places, band);
        assert!(matches!(read, Ok(None)), "off the first span: {read:?}");
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
    /// basin drops both of its turns over the span (`sine = g`,
    /// `basin = sine`). It does not pin either turn alone: dropping one
    /// leaves `g − K·span`, and two arcs that leave a held pair `180°`
    /// apart and come within the band again turn by more than `g` over
    /// the window, which that reading refuses too.
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
        let reach =
            reach_covers_its_pairs((&a, (-3.3, 0.0), 0.0), (&b, (0.0, 3.3), 0.0), band, 3000);
        assert!(!reach.is_finite(), "no reach is bounded: {reach:e}");
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
