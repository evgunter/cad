//! **The pose of two carrier spans**, decided once: the one
//! classification the meeting outcome takes as input ([`Pose`]). It is
//! read for the pairs the table gives a closed form ([`super::Method`]):
//! lines, circles and ellipses. A pair with a spiric or spline side has
//! no closed pose and refuses typed ([`PoseRefusal::Walked`]); its
//! meetings are the walk's.
//!
//! # The arms
//!
//! Every length below is metres, and every closed form solves in one
//! carrier's own parameter: a line's, or a conic's angle.
//!
//! - **The router** (two lines, and a line against a conic's plane):
//!   where the pair can meet is the overlap of the two spans'
//!   projections on the first line (for a conic, the line's span with
//!   the conic's centre ± its larger semi-axis), length `L`, and the
//!   escalation `e` either side. The pair's offset there is the second
//!   line's distance from the overlap's middle point on the first, or the
//!   line's height above the conic's plane at that point. **The parallel
//!   argument runs first, whatever the tilt reads**: where the offset less
//!   its most variation over the overlap and `e` either side,
//!   `rate·(L/2 + e)`, decides positive (`curve_pair_pose_parallel`), the
//!   pair is [`Pose::Parallel`], `rate` being `s/c` for two lines (`s`
//!   the sine between them, `c = |û₁·û₂|`) and `|d̂·n̂|` for a line and a
//!   plane. **Why that proves no meeting anywhere on either span:** a
//!   meeting is two points, one per span, within the band's zero `z` of
//!   each other, so their projections on the first line lie within `z`
//!   of each other, each inside its own span's projection (for a conic,
//!   inside its extent's): the meeting projects within `z < e` of the
//!   overlap, even where the overlap is empty (the projections then part
//!   by less than `z`) or shorter than the band. Over that interval the
//!   offset moves at most `rate` per metre from its value at the
//!   overlap's middle, so it stays past the band's escalation and no
//!   point of one lies within `z` of the other. The parts of either span
//!   outside that interval lie farther than `z` from the other by their
//!   projections alone. **Split invariance follows by construction**: a
//!   piece of a span projects inside the whole's projection, so a
//!   piece's overlap is a sub-interval of the whole's, and its offset at
//!   its own middle less its own variation is at least the whole's
//!   margin; a pair read Parallel whole is read Parallel in every split.
//!   **The gap between the projections reads next**
//!   (`curve_pair_pose_gap`), before the tilt or the offset is read: by
//!   the same projection argument, projections apart by a gap that
//!   decides positive hold no meeting, whatever the lines' distance
//!   reads, and the pair is [`Pose::Apart`]. A piece's gap is at least
//!   the whole's, so a pair read Apart whole is read Parallel or Apart in
//!   every split. A gap read zero or in the escalation gap says nothing,
//!   and the reading goes on. The projections carry the rounding of the
//!   spans' ends as evaluated ([`OnOneLine`]), so a coordinate large by
//!   its parameter is charged as one large by its origin; the offset is a
//!   local difference, read as it is.
//!   Only where neither decides does the tilt route, levered at the
//!   spans' reach from the overlap's middle, the point the offset is read
//!   at (`curve_pair_pose_tilt`): read zero at the nearer span's reach
//!   from above, which holds every point a meeting can use, the pair is
//!   level where it can meet; definite at the farther span's from below
//!   is a crossing. Level, the offset at the
//!   overlap's middle (`curve_pair_pose_offset`) decides: zero is level,
//!   definite refuses [`Unsettled::Meter`], the parallel margin having
//!   not decided, and in the gap what the offset measures decides
//!   (`Offset`): a second line's distance escalates, and a line's height
//!   above a conic's plane, only a lower bound on its distance from the
//!   conic, is level, read by the arm in the plane with its candidates
//!   read on both spans. A tilt in its band escalates.
//! - **Two lines.** Crossing, the candidate is the first line's point
//!   nearest the second. Level, the lines are [`Pose::Coincident`] only
//!   where each span's two ends lie within the zero band of the other's
//!   carrier (`curve_pair_pose_offset`): a line's distance from another is
//!   affine along it, so its ends bound it over the whole span, and
//!   `Coincident` claims one curve over both spans, not only where they
//!   can meet. Two lines level at a shallow corner, their overlap one
//!   point and an end apart past the band, are read as the crossing the
//!   farther span's tilt decides, and refuse [`Unsettled::Meter`] where it
//!   reads zero.
//! - **A line and a conic.** Crossing, the line crosses the plane once,
//!   and the candidate is the closest pair polished from there (below).
//!   Level, the line lies in the plane where it can meet, and there the
//!   conic's signed distance from the line, `(C(θ) − o)·m̂` with `m̂` the
//!   line's normal in the plane, is a first harmonic in the conic's angle
//!   (below).
//! - **Two conics.** Each conic's height above the other's plane is a
//!   first harmonic `c₀ + A·cos(θ − φ)` in its own angle, `A` its swing.
//!   **The parallel argument runs first, whatever the swing reads**, as
//!   the router's does: where the height read with its swing as its
//!   spread, `c₀ ∓ A`, decides one-signed for
//!   either conic (`curve_pair_pose_parallel`), that conic
//!   lies wholly on one side of the other's plane, past the band, and the
//!   pair is [`Pose::Apart`]. Only then are the two swings read
//!   (`curve_pair_pose_swing`), both before either is acted on. One
//!   read zero lies in a plane parallel to the other's, its height
//!   constant to within that swing, and the screen has not cleared it:
//!   the conics lie in one plane to within the slant `|c₀|` plus the
//!   swing (the larger slant where both read zero), below. A slant in the
//!   gap is no meeting of its own: a height only bounds the distance from
//!   below, so the arms in one plane read the pair and every candidate
//!   they place is read on both spans, the slant included. Only where
//!   neither
//!   swing reads zero does one in the gap escalate. Both definite: the
//!   height of one conic above the other's plane, a first harmonic whose
//!   roots are where it crosses that plane, places the candidates, each
//!   the closest pair polished from one (below). **The conic read is the
//!   one that crosses the other's plane the less obliquely**: the larger
//!   `(A² − c₀²)/r²`, `r` its larger semi-axis, which is the squared
//!   sine of its crossing for a circle. That is a choice between two
//!   sound readings, not a reading: it is taken off the log
//!   (`curve_pair_pose_across`, `k_stats::choose`, ledger row
//!   F33, at [`Band::least`]), and a tie or an undecided difference
//!   reads the first. **What is order-free**: the route (apart, level,
//!   across, escalation) depends only on the pair, and across planes
//!   the harmonic read is the same in either order wherever the choice
//!   decides. In one plane the arms below parametrize the first conic
//!   of their kinds' order (`closed`), so a coplanar pair of like kinds
//!   is read from the first span. In one plane, **one reading decides
//!   whether two conics are one curve**, whatever their kinds
//!   (`curve_pair_pose_one_curve`, `one_curve_gap`): each conic is
//!   `c + L·x` over the unit circle, `L = [a·û, b·v̂]`, and the centres'
//!   gap plus `‖L₁ − L₂Q‖` for the best rotation `Q`, its rounding
//!   charged, bounds how far a point of either lies from the other, a
//!   tilt out of the plane included. Read zero is [`Pose::Coincident`]. Past it:
//!   - **two circles**: their centres' distance `d`
//!     (`curve_pair_pose_centres`). Zero is one centre, and the radii's gap
//!     with `d` and the slant as its spread (`curve_pair_pose_radii`)
//!     decides [`Pose::Concentric`]; read zero all round, it contradicts
//!     the one-curve reading, which is at most `d + |Δr|` plus the slant
//!     ([`MeetFault::Contradiction`]). Definite, the first circle's distance from
//!     the second's centre runs between `|r₁ − d|` and `r₁ + d`, so its
//!     distance from the second circle has those extremes less `r₂`,
//!     exactly; and its square is a first harmonic in the first circle's
//!     angle, whose roots are read from the extremes (below);
//!   - **two ellipses, or an ellipse and a circle**: the first's level on
//!     the second, of degree two in the first's angle, goes to the
//!     certified subdivision ([`crate::half_angle::certified_subdivision`])
//!     in the metres of the lower bound on a point's distance from an
//!     ellipse ([`geom::ellipse_level_clearance`], [`Curve3::clearance`]'s),
//!     with a ceiling on the level per metre of it (`coplanar_ellipses`).
//!     A circle is the ellipse of its equal semi-axes, its level
//!     `(|q − c|² − r²)/r²`; the ellipse is the one parametrized.
//!
//! # A first harmonic's roots, and a double root
//!
//! A residual `R(θ)` whose range is `[lo, hi]` is decided on its extremes
//! (`curve_pair_pose_extreme`): definitely one-signed is
//! [`Pose::Apart`], definitely straddling is two crossings, an end of a
//! distance in the escalation gap escalates (a height's, below), and an
//! extreme in the zero band is a touch there (below). Two definite
//! crossings are read from the extreme nearer zero
//! ([`crate::roots::arc_past`]).
//!
//! **A conic's height above the other's plane is not its distance from
//! the other**, only a lower bound on it, so an extreme of the height in
//! the gap, the other extreme definite past it, says nothing the band
//! answers: it places a touch there, read through its window and
//! polished as any plane root's ("A crossing of the other's plane",
//! below). Where the polish's basin is not proved there, the extreme's
//! escalation is the refusal: what is open is the band's question, which
//! a finer band decides, not the polish's premise.
//!
//! **An extreme in the zero band is a touch at the extreme**, whatever lies
//! between its roots, by C5 (`crates/geom-brep/README.md`: a meeting is a
//! longest interval of one span everywhere definitely on the other):
//!
//! - **a distance** (a conic's signed distance from a line in its plane,
//!   a circle's from a circle in its plane): between the roots the
//!   residual runs from zero to the extreme and back, so the whole arc
//!   between them lies within the zero band of the other carrier. It is
//!   one meeting, however far apart its roots lie, and its touch is the
//!   extreme's point;
//! - **a height**: the touch is a locator, not a meeting. Its window and
//!   the polish read what the arc about it holds ("A crossing of the
//!   other's plane", below), as they do for an extreme in the gap. They
//!   refuse differently: where the polish's basin is not proved, a touch
//!   an extreme in the gap placed escalates on that extreme (the band's
//!   question is open), and one an extreme in the zero band placed
//!   refuses on the basin ([`Unsettled::Meter`]): the height is in the
//!   zero band there, so the band has nothing open and the polish's
//!   premise is what failed.
//!
//! **Two definite crossings are both read, however near their roots**
//! (`curve_pair_pose_root_gap`, read for its conditioning: undecided,
//! the roots are placed past what the band reads, and it escalates).
//! Where the band cannot split their points they ring as one meeting,
//! the double root the band cannot split. They are not read as one touch
//! at the extreme between them: both extremes are decided past the band,
//! and where a conic curves sharply enough to leave the band between two
//! roots that close (a thin ellipse's vertex, its radius of curvature
//! `b²/a`), that point lies the extreme's value off the other carrier.
//!
//! **In one plane, a candidate admitted under a `Zero` reading is placed
//! by the exactly degenerate formula**: a touch is the extreme's own
//! point, `C(θ_hi)` or `C(θ_hi + π)`, on the first carrier exactly and
//! within the extreme's value of the second. An extreme read zero whose
//! value has not crossed (no real root) is that touch too. Between two
//! circles in one plane the residual there is the distance to the other
//! carrier itself, so the reading of the candidate decides it. A line the
//! tilt read level lies over the conic's plane by a slant, and is read as
//! below.
//!
//! # A slanted pair in one plane
//!
//! **A line and a conic the router reads level are read with their
//! slant.** With `m` the line's normal in the plane and `v = û × m` its
//! other normal, `{û, m, v}` is orthonormal, so a conic point's distance
//! from the line is `√(R² + H²)`: `R(θ) = (C(θ) − o)·m`, the residual in
//! the plane, and `H(θ) = (C(θ) − o)·v`, both first harmonics in `θ`.
//! In the plane `H` is zero, and a line the tilt read level holds a slant
//! in it.
//!
//! - **The stretch.** A meeting needs `|R| ≤ √(R² + H²) ≤ e`, the band's
//!   escalation: the arcs of `|R| ≤ e`, `φ₁ ≤ |θ − θ_hi| ≤ φ₂` with
//!   `cos φ₁ = (e − c₀)/A` and `cos φ₂ = (−e − c₀)/A`. Off them the pair
//!   is past the escalation. Each root, and each touch, lies on one of
//!   them, its stretch, read alone. An arc of half a turn or more is not
//!   read (`curve_pair_pose_level_arc`, a proof's inequality).
//! - **The floor.** On the stretch the distance is at least `|H|`, and
//!   `H` is a first harmonic on an arc short of half a turn: its least
//!   `|H|` there is at an end, or at the one turn of `H` within it (where
//!   `H′` changes sign), or zero where `H` changes sign. That least `|H|`
//!   bounds every pair of the stretch from below, so it is the
//!   candidate's window floor: past the band, the stretch holds no
//!   meeting.
//! - **The candidates.** Each root is read at itself. A touch whose
//!   extreme is proved within the band's zero (`curve_pair_pose_touch_zero`)
//!   is read at the extreme: in the plane the residual between the
//!   roots is no larger than at the extreme, so the run is one meeting,
//!   and the touch keeps its angle's enclosure. Otherwise both roots are
//!   read, `θ_hi ± φ₀` with `cos φ₀ = −c₀/A`: the zero set may part into
//!   two, one about each root, with the extreme between them past the
//!   zero, and a slanted touch's roots are where `R` is zero and the
//!   distance is the slant alone. Two roots read on one stretch are two
//!   candidates, and `stretches` reads the point between them: two
//!   meetings where it is off the band, an escalation where it is in it
//!   (C5). The floor reads every candidate's Off.
//! - **The read.** The candidate is read as the walk's pair is
//!   (`Found::read_pair`, its window not held to the spans,
//!   [`WindowSpans`]): a gap in the zero band is a meeting; an
//!   Off prunes where the floor clears, or where the stretch's reach and
//!   run lie past a span ([`Span::past_by`]), and otherwise escalates. Its
//!   pairs lie on the stretch, within the stretch's run at the conic's
//!   greatest speed of the candidate's point, and within `e` of that on
//!   the line: where the ball of that reach about the candidate is clear
//!   of either span, by the span's walk ([`Span::clear_by`]), the stretch
//!   holds no pair on the spans and no candidate is read.
//!
//! A touch read at its extreme stands for its stretch, so a run within
//! the zero is not two meetings. Two candidates ring as one meeting where
//! the band cannot split them, and otherwise are two meetings, which
//! escalate where the line between them is not decided off the conic
//! (`curve_pair_meet_stretch`).
//! Where the candidate's gap reads off and the floor does not clear, the
//! least distance lies where neither the candidate nor the floor places
//! it, and the pair refuses: the safe direction.
//!
//! # One meeting to a part
//!
//! **Each part of the zero set must hold a candidate**, or a meeting goes
//! unread. Write `z` and `e` for the band's zero and escalation.
//!
//! - **In one plane**, `H` constant: the zero set on a stretch is
//!   `|R| ≤ √(z² − H²)`, a level set of `|R|`. `R` is a first harmonic on
//!   an arc short of half a turn, monotone between a root and an
//!   extreme, so each part of `|R| ≤ c` holds a root of `R` or its
//!   extreme. Each is a candidate: the roots always, and the extreme
//!   where it is proved within the zero, otherwise the roots on each
//!   side of it (above, "The candidates"). The pose reads a pair as in
//!   the plane where `H`'s lean, its swing `√(h_c² + h_s²)`, is proved below the zero
//!   (`curve_pair_pose_level_flat`). What a lean below the zero leaves
//!   of `H`'s turn over the arc is
//!   `work/onehome/a-level-root-past-a-span-on-a-shared-stretch-escalates-the-census.md`'s.
//! - **Slanted, where `e/z` is at least `2.33`**: a stretch to one side
//!   of an extreme holds at most one part (below, "No second part on a
//!   side arc"), and the pair is read. The first tilt branch and the
//!   stretches about an extreme are read without that argument, and
//!   `work/onehome/a-level-root-past-a-span-on-a-shared-stretch-escalates-the-census.md`
//!   holds them.
//! - **Slanted, where `e/z` is below `2.33`**: no argument holds, and
//!   the pair is not read (`curve_pair_pose_slant_tight`, refusing).
//!
//! ## No second part on a side arc
//!
//! In exact arithmetic, with `z` and `e` the band's zero and escalation
//! and `k = e/z`. The conic is `C(θ) = c + a·cos θ·U + b·sin θ·V`,
//! `a ≥ b > 0`, with `n = U × V`; the line has origin `o` and unit `û`,
//! with `den = û·n`, `m` and `v` as above, and `{û, m, v}` orthonormal.
//!
//! **The projected conic.** `P(θ) = (R, H) = P₀ + M·(cos θ, sin θ)` with
//! `M = [[a U·m, b V·m], [a U·v, b V·v]]`, so
//! `det M = ab·(U × V)·(m × v) = ab·den`, since `m × v = û`. `R`'s swing
//! is `A = √((a U·m)² + (b V·m)²)`; `m` lies in the plane, so
//! `(U·m)² + (V·m)² = 1` and `b ≤ A ≤ a`.
//!
//! **Two premises.**
//! - **(S) definite crossings**: `hi = c₀ + A > e` and `lo = c₀ − A < −e`,
//!   as `harmonic_roots` reads `(lo, hi)` decided past the band. So
//!   `A > e`.
//! - **(T) the tilt's second branch**: the router reads level where
//!   `|den|` levered by `max(reach, speed_hi)` is not definite, which is
//!   `|den|·a ≤ e`, the ellipse's greatest speed being `a`.
//!
//! **The arc is a graph.** On a side arc `R` runs from `e` to `−e`, and
//! `R′ = −A sin(θ − θ_hi) ≠ 0`, so `H = g(R)` over `R ∈ [−e, e]`. Since
//! `P′ = M·(−sin θ, cos θ)` and `P″ = −M·(cos θ, sin θ)`,
//! `R′H″ − H′R″ = det[P′ | P″] = det M`, so
//! `g″ = ab·den / R′³`: one-signed, so `g` is convex or concave
//! throughout. And `R′² = A² − (R − c₀)² = (hi − R)(R − lo)`.
//!
//! **Two parts need a sharp bend.** Let `g` be convex (for concave, take
//! `−H`), `W(R) = √(z² − R²)` and `L = −W`. The zero set over `[−z, z]` is
//! `{g ≤ W} ∩ {g ≥ L}`. `g − W` is convex, so the first is an interval.
//! Two parts then need `R₁ < R_m < R₂` with `g(R_m) < L(R_m)` and
//! `g ≥ L` at both ends, so `g − L` is not concave between them, and
//! `g″(ξ) > L″(ξ) = z²/(z² − ξ²)^{3/2} ≥ 1/z` at some `|ξ| < z`.
//!
//! **`R′` is not small there.** At `ξ`, `R′² = pq` with `p = hi − ξ`,
//! `q = ξ − lo` and `p + q = 2A`. By (S), `p` and `q` are each past
//! `δ = e − z`, so `pq ≥ δ(2A − δ) > δ·A`, as `A > e > δ`.
//!
//! **The chain.** `ab·|den| ≤ b·e ≤ A·e` by (T) and `b ≤ A`. So
//! `(δA)^{3/2} < |R′(ξ)|³ < z·ab·|den| ≤ zeA`, so `A < z²e²/(e − z)³`. With
//! `A > e`, two parts need `(e − z)³ < z²e`, that is `(k − 1)³ < k`. That
//! fails for `k` at or past the real root of `(k − 1)³ = k`, `2.3247…`:
//! there a side arc holds at most one part. A part holding the root is
//! read there; a part not holding it leaves the root's Off unread by the
//! floor, and the pair escalates.
//!
//! The argument reads `d` from the carrier line. Where the segment's end
//! cuts or moves a part, the span ends are read first
//! ([`end_candidates`]). (S) and (T) are their decisions taken exactly,
//! and the rounding a decision carries moves `k` a little, by an amount
//! not measured here.
//!
//! # A touch in the subdivision
//!
//! **The coplanar subdivisions place a touch by its in-band stretch**
//! ([`crate::half_angle::certified_subdivision_touching`]). Two conics
//! that touch stay within the band of each other over an arc much wider
//! than the band, so no cut there reads a definite sign. Before that arc
//! is cut, the piece holding it reads convex toward its ends' common
//! side (`curve_pair_coplanar_sub_convex`): `F` has one extremum there
//! and at most two roots, both about it. The extremum is the candidate.
//! Where the carrier there reads in the zero band, the most residual over
//! the arc between the piece's roots is read
//! (`curve_pair_coplanar_sub_touch`): the most `|F|` convexity leaves
//! there, over the frame's floor on `|F|` per metre. Neither door charges
//! a jet's rounding: `F` is read locally and exactly ([`Level`]).
//! - **Zero** is one touch. The arc lies everywhere within the zero band
//!   of the other carrier, so by C5 (`crates/geom-brep/README.md`: a
//!   meeting is a longest interval of one span everywhere definitely on
//!   the other) it is one meeting, however far apart its roots lie; a
//!   touch is that meeting's kind, two carriers whose double root the
//!   band cannot split, not a shared run of one curve.
//! - **In the gap** it escalates.
//! - **Definite**, the walk goes on, and roots within one in-band arc
//!   end in its refusal.
//!
//! The touch is the extremum's own point, on the first carrier exactly,
//! and is read on both spans like any candidate.
//!
//! # A touch's spread
//!
//! **A coplanar touch is reported at one point, and that point is held
//! to within the band of every other reading of it**
//! (`curve_pair_pose_touch_spread`; `work/onecopy/touch-point-spread.md`).
//! Its readers match it by distance against the band (census pass 5′'s
//! `Declared::confine`, the boolean's curved pierce `held_on`), so two
//! readings of one touch (two spans, two operand orders, two callers)
//! must place it within the band of each other. Each **coplanar** path
//! bounds how far two readings of its point on one carrier can lie
//! apart, and that is read in the zero band: zero keeps the touch; in the
//! gap it escalates ([`MeetFault::Escalated`]); decided past the band it
//! refuses on that row's meter ([`Unsettled::Meter`]).
//! - **The subdivision**: `θ*` is a bisection midpoint on `F′`, and
//!   `G″ ≥ c` on the piece, read, so it lies within `(|G′(θ*)| + e₁)/c`
//!   of the extremum; at the carrier's top speed that is its spread
//!   ([`crate::half_angle::Touch`]), and two readings of one `F` lie
//!   within twice that.
//! - **The closed forms**: the touch is `C(θ_hi)`, `θ_hi` the angle of the
//!   first harmonic's coefficients, a well-conditioned reading of them.
//!   At a point scalar it is exact by `[simple.conditioned]`, so two
//!   readings are one; at `Interval` the angle's own enclosure is the
//!   spread, read as `r·width(θ_hi)` (`angle_spread`).
//!
//! **Two operand orders read two carriers.** Swapped, the touch is
//! placed on the other carrier, whose point lies from the first's by up
//! to twice the spread and the gap between the carriers there (at most
//! the zero band, the touch read in it). The spread held is the
//! reading's own; the gap is the touch's, and C5 admits it.
//!
//! **The across-planes touch and the walk's are not held here**:
//! `work/onecopy/a-touch-point-read-across-planes-or-by-the-walk-carries-an-unheld-spread.md`.
//!
//! Whether one point may stand for the whole in-band arc, where a
//! reader's question straddles the arc, is a separate question, open on
//! PR 6570.
//!
//! # Across planes: the closest pair
//!
//! **Where a curve crosses the other's plane is not where the two meet**
//! when they pass within the band without touching: the meeting lies
//! `h/σ` along the curve, `h` their distance and `σ` the sine of the
//! curve's angle to the plane. So a crossing (or a touch) of the other's
//! plane only starts the candidate: it is polished by Gauss–Newton
//! ([`super::polish`], an `f64` structure search) toward the two spans'
//! closest pair, and that pair is read with what it states of its window.
//! Every point of the curve within the band's escalation `e` of the other
//! carrier is within `e` of the other's plane, so it lies on the arc
//! about a crossing or a touch where the height is within `e`: `ℓ`, read
//! off the harmonic (or `e/σ` on a line). The *window* is the pairs of
//! the two spans within `e` of each other whose first point lies within
//! `ℓ` of the crossing along the first carrier: so both points lie in the
//! ball of `ℓ + e` about the crossing, in metres. Every pair of the spans
//! within `e` of each other near the crossing is in it, and the gap's
//! lower bound reads it as arcs from the polished pair ((P1) below).
//! Nothing reads whether the polish converged: the pair is a place, and
//! what it says of the window is a bound.
//!
//! **Held to the spans.** The polish is held to the two spans
//! ([`super::polish::pair_held`]; an endless span holds nothing), as the
//! walk's is, from the crossing and its foot each put on the turn about
//! its span's middle ([`Span::branch`]), so a hold clamps a parameter
//! into its own span and not a turn away from it. The window is the
//! region about the crossing on the spans. A pair of it within the
//! band's escalation `e` has its first point within `ℓ` of the crossing
//! along the first carrier, so within `ℓ` of it in metres, and its second
//! within `e` of the first: both lie in the ball of `ℓ + e` about the
//! crossing. The crossing is the start's point on the first carrier, a
//! seed computed in `f64` (`line_conic`'s `o + d̂·t`, `transverse`'s
//! plane root), and `ℓ` is read from it as it landed: `line_conic`'s is
//! `(e + |h|)/|d̂·n̂|`, `h` the seed's own height above the plane, so the
//! seed's distance from the true crossing is read, not charged. Where
//! that ball is clear of either span, by the span's own
//! piece walk in metres ([`Span::clear_by`],
//! `curve_carrier_piece_clear_by`), the window holds no such pair and the
//! pose reads none. A parameter's distance along a carrier is no such
//! bound, nor is a ball about the start's foot on the second carrier: the
//! foot need not be the crossing's nearest point, and a conic can come
//! round within the ball from far along its arc.
//! The polish is not read there: held at an end of a span far from the
//! crossing, its travel is no part of the window, and would only weaken
//! the basin. The polished pair names a parameter on each span and is
//! read as that pair
//! (`Found::read_posed`, C5's pair rule): its own gap, and each
//! parameter's place, with no foot. A pair a span stops is held exactly
//! at that span's end, and reads the window there one-sided ("The
//! one-sided window", "A held end's floor"), where the window's arcs on
//! that span all lie on its side of the end: on a conic, where the
//! span's parameter run and the window's at the least speed fall short of
//! a turn together (`curve_pair_pose_held_one_piece`). A span that comes
//! round to within the window of its other end holds the window in two
//! pieces, one each side of the gap, and reads it two-sided. No pair of
//! the window lies past either span, so an Off at the pair prunes only
//! by the floor, and otherwise escalates. The reach and the run are read
//! past a span for the walk's pair (`walk`'s module docs, "The reach and
//! the run, past a span"), never for the pose's.
//!
//! # The gap's lower bound
//!
//! At the polished pair, `p` on the first carrier and `q` on the second,
//! with unit tangents `u` and `v`, write `w = q − p`, `n̂ = u×v/|u×v|`, and
//! the *slack* `σ_w = |w − n̂(n̂·w)|`, the part of the gap a move along the
//! carriers could still take up. Two premises, stated where they are
//! read:
//!
//! - **(P1) the span.** `span = max(span₁, arc₂)`, an arc bound on each
//!   carrier from the polished pair:
//!   - on the first, `span₁ = region + travel`, `travel` the polish's
//!     parameter run at the carrier's greatest speed: every pair of the
//!     window within `e` has its first point within `ℓ` of the crossing
//!     along the carrier, and `p` is within its travel of the start;
//!   - on the second, the pair's second point lies within `ℓ + e` of the
//!     crossing in metres, a chord, so within `R = ℓ + e + |q − c|` of
//!     `q`, `c` the crossing. On a line that is the arc. On a conic the
//!     arc is read from `q` (`Side::arc_from`): the angles
//!     `J = [t − ρ, t + ρ]` about `q`'s, `ρ = (π/2)·R/|C′(t)|`, hold every
//!     such point where the span's parameters outside `J` and its copies
//!     a turn either side (a span near a turn meets the carrier on both
//!     sides of a gap within the ball) are clear of the ball of `ℓ + e`
//!     about the crossing, by the span's own walk
//!     ([`Span::clear_by`]; each piece's place against `J` proved,
//!     `curve_pair_pose_window_piece`). The pieces end at `J`'s own
//!     bounds `t ∓ ρ` (and their copies), the same values, so at `f64` they
//!     abut `J` exactly. At an interval scalar each walked piece's ball
//!     reaches its ends' enclosures (its reach is the enclosed width at
//!     the greatest speed), as `J`'s arc does, so no sliver between them
//!     goes unwalked. Then `arc₂ = ρ·max |C′|` over
//!     `J`, the arc from `q` within `J`. Otherwise, or where `ρ` reaches
//!     half a turn (`curve_pair_pose_window_half`), `arc₂` is half the turn at the conic's greatest
//!     speed, a point's arc from `q` taken the shorter way round. The
//!     choice of `ρ` is a heuristic; the walk is the proof, so a foot
//!     landed on a thin ellipse's other branch, or a carrier coming round
//!     within the ball from far along it, reads the half turn.
//!
//!   The walk's window is its pieces, a parameter run on each carrier,
//!   and is an arc bound on both.
//! - **(P2) the curvature.** `K = κ₁ + κ₂` bounds the carriers'
//!   curvatures over the arcs (P1) covers: the first's within `span₁` of
//!   the start, the second's over `J`, or the whole carrier where `arc₂`
//!   is its half turn (`Side::curvature_near`).
//!
//! By arc length from the pair, a point of each carrier is `A = p + uα +
//! r_A` and `B = q + vβ + r_B`, with `|r_A| ≤ κ₁α²/2` and `|r_B| ≤ κ₂β²/2`
//! (a curve's departure from its tangent line). With `m = max(|α|, |β|)`,
//! `A − B = −w + uα − vβ + r` and `|r| ≤ K·m²/2`:
//!
//! - **across**, `u` and `v` are square to `n̂`, so
//!   `|A − B| ≥ |(A − B)·n̂| ≥ |w·n̂| − K·m²/2`;
//! - **along**, the least of `|uα − vβ|` over `β` is `|α|·sin φ`, and over
//!   `α` it is `|β|·sin φ`, so `|uα − vβ| ≥ sin φ·m` and
//!   `|A − B| ≥ s·m − σ_w − K·m²/2` for any `s ≤ sin φ`.
//!
//! **The move.** A pair within `e` therefore has `f(m) < 0`, where
//! `f(m) = s·m − K·m²/2 − (σ_w + e)`. Where `D = s² − 2K(σ_w + e)` is
//! positive (`curve_pair_pose_move`, a proof's inequality), `f` is
//! positive between its roots `m∓ = (s ∓ √D)/K`, so `m < m₋` or
//! `m > m₊`. **The basin** (`curve_pair_pose_basin`) is the premise that
//! puts the window below `m₊`: `s − K·span > 0`, since then
//! `span < s/K ≤ m₊`, and by (P1) `m ≤ span`. Every pair of the window
//! within `e` lies within `m₋ = 2(σ_w + e)/(s + √D)` of `(p, q)` along
//! both carriers (written so a pair of lines, `K = 0`, reads
//! `(σ_w + e)/s`). `s` is the sine's lower bound over the window,
//! `sin φ − K·span`, so the basin reads `sin φ − 2K·span > 0`, the
//! tangents keeping at least half their angle; the travel the polish
//! made is folded into `span` rather than metered on its own.
//!
//! **The floor and the reach.** Such a pair is at least
//! `floor = |w·n̂| − K·m₋²/2` apart, the gap across less the *bend* over
//! the move. So:
//!
//! - **a floor past the band** (`curve_pair_pose_closest`) contradicts any
//!   pair within `e`: the window holds no meeting;
//! - **the reach** `m₋` places every such pair: on the first carrier
//!   within `m₋` of `p`, on the second within `m₋` of `q`. The walk reads
//!   it past a span (`walk`'s module docs, "The reach and the run, past a
//!   span"); the pose's pair is held to the spans, and reads no reach past
//!   one.
//!
//! **The one-sided window.** Where `p` is exactly an end of the first
//! span, `q` exactly an end of the second, and every pair of the window
//! lies on the spans (the walk's pieces do), let `u` and `v` be each
//! span's own half-tangent at its end: the one-sided derivative from
//! inside the span, pointing into it. At a crease (a `C⁰` knot) the two
//! sides' derivatives differ, and the span's own side is the one its
//! arcs leave along; `K` is likewise read over that side of the end
//! alone, the arcs the window holds. Every pair of the window is then
//! at arc `α, β ≥ 0`.
//! With `c = u·v`, `|uα − vβ|² = α² + β² − 2αβc`:
//!
//! - where `c ≤ 0` this is at least `α² + β² ≥ m²`;
//! - where `c > 0`, with the larger of `α, β` at `m`, it is least at the
//!   other `= c·m`, which lies in `[0, m]`, and is `m²(1 − c²)` there.
//!
//! So `|uα − vβ| ≥ g·m` with `g = √(1 − max(c, 0)²)`: `sin φ` while the
//! half-tangents part by less than a right angle, and `1` past it, where
//! the two-sided `sin φ` falls to zero at a joint (`φ = 180°`). The along
//! bound is `|A − B| ≥ s·m − |w| − K·m²/2`, `s ≤ g`, the whole gap `|w|`
//! standing for the slack since no across direction is read. The move,
//! the basin (`curve_pair_walk_basin_held`) and the reach are the
//! two-sided ones with `g` for `sin φ` and `|w|` for `σ_w`. There is no
//! floor: `|uα − vβ|` is not bounded above by anything the slack
//! outweighs, so the window states its reach and bend and a floor of
//! `−∞`. The walk and the pose read it where the two-sided reading bounds
//! no reach: its basin not proved, or its move. The pose's polish starts
//! where a curve crosses or touches the other's plane, so its window is
//! that crossing's, and its pair is held at both ends only where the
//! crossing is at both: a line leaving an arc's end nearly along its
//! tangent reads its meeting there one-sided, where the two-sided basin
//! fails on the tangents' small angle.
//!
//! **A held end's floor.** Where only `q` is held at an end of its span,
//! with `v` its half-tangent into the span, every pair of the window has
//! `β ≥ 0`, and `α` free. With `P` the projection square to `u`,
//! `|−w + uα − vβ| ≥ |P(w + vβ)|`, and over `β ≥ 0` that is least at
//! `β = 0` where `Pw·Pv ≥ 0` (the arc into the span runs away from the
//! other carrier), at `|Pw|`, and otherwise at the foot of `Pw` on the
//! half-line `−Pv·β`, `√(|Pw|² − (Pw·Pv)²/|Pv|²)`. Either is a height
//! `h` the pair's own across `|w·n̂|` can stand beside, and the two-sided
//! reach and bend hold over the held window, a part of the two-sided one,
//! so the floor is the greater of `|w·n̂|` and `h`, less the bend
//! (`polish::Along::held_across`). `p` held alike, with `u` and `v`
//! exchanged. It is what clears a crossing just past an end, where the
//! gap at the held pair runs along the held span's tangent and `|w·n̂|`
//! reads nothing (the keel's wall past a bore's rim).
//!
//! Where `D` is not proved positive the bound is undecided: no floor, and
//! no reach short of the carriers. **The composition is the reader's**
//! (`Found::read_posed` for the pose's pair, `Found::read_pair` for the
//! walk's), on the candidate's [`WindowBound`]: a floor past the band is
//! no meeting; otherwise the pair is read on both spans, and a gap in the
//! band there is a meeting, with no premise on the polish. The walk's Off
//! prunes where the reach or the run arm clears a span; any other Off is
//! a straddle of
//! the floor against the gap at the pair (`curve_pair_meet_window`), or
//! the reach's own (`curve_pair_span_past_reach_straddle`), and
//! escalates. A location along the carriers enters only to prune, and is
//! read in metres against the band; where it is ill-conditioned (a small
//! `s`, a wide enclosure) the reach is long and the Off escalates, which
//! is the safe direction. A candidate whose point speaks for its whole
//! window, an end, a witness or an exact root, states no window, and its
//! read is its point's alone, as before windows were stated.
//!
//! The basin and the move are proofs' inequalities, read exactly
//! ([`geom_core::k_stats::prove`]); a basin not proved refuses
//! [`Unsettled::Meter`] on its row, but at a touch a height's extreme in
//! the gap placed, which escalates ("A first harmonic's roots"). The floor, the reach and the gap are
//! metres of gap, read against the band. All are logged.
//!
//! # The spans' ends, and deflation
//!
//! A touch or a crossing is one point, but two curves near it stay
//! within the band over a stretch about it, long where they cross at a
//! shallow angle or touch with little relative curvature. Where that
//! point lies past a span and the stretch reaches into both, the spans
//! meet in the band at an end of one of them, which no isolated
//! candidate reads. So each span's ends are read against the other
//! first ([`end_candidates`]): a span end within the band of the other
//! span is a meeting, and one in the band's gap escalates. An endless
//! span ([`Span::endless`]) has no end: no point lies past it, and its
//! seam is a point of its interior like any other.
//!
//! **A meeting so found is a root the pose already holds where the
//! arm's equation is the meeting's own**, and there the arm divides it
//! out rather than solving for it again: re-derived near a tangent, a
//! root comes from a square root whose rounding is about `√ulp` of the
//! radius, and its gap from its twin escalates where the found meeting
//! decides it exactly. The arms whose equation is the meeting's are the
//! in-plane ones: a circle's or an ellipse's signed distance from a line
//! in its plane, and a circle's distance from a circle in its plane.
//! Their first harmonics, `c₀ + A·cos(θ − θ_hi)`, have their two roots
//! symmetric about `θ_hi`, so the other root is `2θ_hi − θ_J`, linear in
//! the found one's angle `θ_J`: the line's second meeting with a conic in
//! its plane (Vieta), and the reflection of a meeting of two circles in
//! one plane across their line of centres. Two found meetings leave a
//! first harmonic none.
//!
//! **A found meeting need not be a root.** A meeting the ends find is a
//! span's end within the zero band of the other span, which need not lie
//! at a root of the residual. On a thin ellipse near its vertex, an end
//! on one branch is in the zero band; its reflection lands on the other
//! branch beside it, while the second root lies farther along. So a line
//! and a conic in its plane offer the reflection of one found meeting
//! only where it is not shown to be no root ([`reflected`]): where it
//! rings onto the found meeting's point in metres while its angle from
//! `θ_J` is decided past that meeting's ring in angle, the reflection is
//! the other branch's, and the arm reads both roots, `θ_hi ± φ₀`. A root
//! within the band of the found meeting's point then rings onto it
//! (`curve_pair_meet_same`). Two that land apart, neither ringing, are
//! read as two candidates, and `stretches` reads the point between them.
//!
//! Two lines' crossing is solved linearly, with no square root to
//! condition, so it lands inside a found joint's ring and is not read
//! again: that form divides nothing out.
//!
//! **A crossing of the other's plane is not a meeting**, so the arms
//! across a plane divide nothing out. A conic's or a line's root on the
//! other's plane marks a window, about `e/|d̂·n̂|` of arc either side,
//! where it lies within the band's escalation of that plane, and that
//! window can hold more than one meeting, each in a stretch of its own.
//! A conic's plane root is read through its window first (`window`),
//! and where the window certainly holds no meeting but the found ones,
//! none among them included, the root is not polished. What the window
//! certifies is two claims, one per kind of arc:
//!
//! - **beside a found meeting**, an arc past its ring, read outward,
//!   holds no point within the band's zero of the other carrier: every
//!   piece reads off past the zero, or receding, and so no second
//!   meeting adjoins the ring. A point there within the band's gap of the
//!   other carrier is the found meeting's own stretch (C5), which the
//!   found meeting already answers;
//! - **with no ring**, an arc holds no point within the band's
//!   escalation of the other span: every piece reads off past the
//!   escalation, of the other carrier or, where that carrier runs on past
//!   its span beside the arc, of the span itself. Here a near-miss in the
//!   gap answers to nothing found, so reading it off past the zero would
//!   decide a pair the band leaves undecided. Past the escalation, it is
//!   decided apart.
//!
//! Every other plane root is polished and bounded as it would be with no
//! meeting found; a candidate it gives nearer a found meeting than the
//! escalation is that meeting again (C5's ring rule), and a basin that is
//! not proved refuses. The coplanar subdivisions divide nothing out: their roots
//! nearer a found meeting than the escalation are not read again.
//!
//! # The band
//!
//! A decision the reading rests on escalates in the band's gap
//! ([`MeetFault::Escalated`]). A decision read as a screen, for the one
//! verdict that lets the reading stop or take a shorter road, passes any
//! other reading on, an escalation included, to the readings after it,
//! which decide the pair or escalate on their own: the router's parallel
//! margin (`curve_pair_pose_parallel`), its gap (`curve_pair_pose_gap`)
//! and its tilt over the overlap (`curve_pair_pose_tilt`), the polish's
//! clearance (`curve_pair_pose_clear`), and the like in the window's
//! readings and the branch compares. A basin that is not proved refuses
//! [`Unsettled::Meter`], but at a height's extreme in the gap, which
//! escalates. No escalation prunes a candidate.

mod window;

use geom::{Azimuth, Curve3, CurveKind};
use geom_core::UnitVec3;
use geom_core::k_stats::{Pick, choose, decide, straddle};
use geom_core::one_line::{OnOneLine, on_one_line};
use geom_core::{Band, Decide, Lever, Margin, Point3, Sign, Vec3};

use super::span::Span;
use super::{Arm, MeetFault, Method};
use crate::half_angle::{
    Local, SubdivisionFrame, SubdivisionRows, Terms, TouchRows, certified_subdivision_touching,
};
use crate::ladder::read_identity;
use crate::roots::{Roots, Unsettled, arc_past};
use geom::Conic;

/// How two carriers read as one curve run: their parameters increase the
/// same way along it, or opposite ways.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sense {
    /// The same way.
    Same,
    /// Opposite ways.
    Opposite,
}

/// A candidate meeting: a point, and a parameter on each carrier for
/// reading it there.
///
/// `bound` is what the generator states of its window (module docs,
/// "The gap's lower bound"). `None` where an Off at the point speaks for
/// the whole window (an end, a witness, an exact root), read as the point
/// alone (`Found::read`): on the carrier it was placed on the parameter
/// is the point's own, and on the other it is its foot's
/// [`geom::Foot::seed`], which certifies nothing, so the point is read
/// through each carrier's certificate. Otherwise it is the closest pair,
/// polished held to the spans, its parameters the pair's own on both,
/// read as that pair under its window's floor and reach
/// (`Found::read_posed`).
#[derive(Clone, Copy, Debug)]
pub struct Candidate<T: geom_core::Real> {
    /// The point.
    pub point: Point3<T>,
    /// Its seed on the first carrier.
    pub ta: T,
    /// Its seed on the second carrier.
    pub tb: T,
    /// The window it stands for, `None` for a point that is its own.
    pub bound: Option<WindowBound<T>>,
}

/// **What a polished pair states of its window** (module docs, "The
/// gap's lower bound"), for the reader to compose with its point's
/// reading (`Found::read`).
#[derive(Clone, Copy, Debug)]
pub struct WindowBound<T> {
    /// A lower bound in metres on the gap between any two points of the
    /// carriers in the window, `|w·n̂| − bend`; `−∞` where undecided.
    pub floor: T,
    /// The most metres along either carrier, from the candidate's own
    /// parameters, at which two points of the window lie within the
    /// band's escalation of each other, `m₋`; `+∞` where undecided.
    pub reach: T,
    /// The most metres either carrier departs from its tangent line at
    /// the candidate over `reach`, together, `K·m₋²/2`; `+∞` where
    /// undecided.
    pub bend: T,
    /// Where the window's pairs lie against the two spans.
    pub spans: WindowSpans,
}

/// **Where a window's pairs lie against the two spans**
/// ([`WindowBound::spans`]), which says how its pair is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowSpans {
    /// Every pair lies on the two spans: the pose's closest pair, polished
    /// held to them (module docs, "Held to the spans"), read as that pair
    /// with nothing read past a span (`Found::read_posed`).
    Held,
    /// The window may reach past a span: the walk's, and a slanted pair's
    /// stretch in one plane, read as the walk's pair is, its reach and run
    /// read past each span (`Found::read_pair`).
    MayPass,
}

/// The pose of two spans' carriers (module docs).
#[derive(Clone, Debug)]
pub enum Pose<T: geom_core::Real> {
    /// One curve within the band over the spans. `witnesses` are points of
    /// the carrier that a positive-length overlap of the spans holds one
    /// of ([`Span::witness_params`]).
    Coincident {
        /// How the two parameters run.
        sense: Sense,
        /// The overlap's candidate points.
        witnesses: Vec<Candidate<T>>,
    },
    /// Two circles in one plane on one centre, their radii definitely
    /// apart: no point meets.
    Concentric,
    /// Parallel lines, or a line parallel to a conic's plane, definitely
    /// apart: no point meets.
    Parallel,
    /// Definitely apart otherwise: the spans' projections on a line
    /// apart past the band, a conic wholly to one side of the other's
    /// plane, or the residual's extremes one-signed.
    Apart,
    /// One touch: a double root the band cannot split, of the residual in
    /// one plane or of the height above the other's plane.
    Tangent(Candidate<T>),
    /// Isolated meetings, at most four, each a candidate to be read.
    Generic(Vec<Candidate<T>>),
}

/// Why [`pose`] has no pose to give.
#[derive(Clone, Copy, Debug)]
pub enum PoseRefusal {
    /// The pair has a spiric or spline side: its meetings are the walk's.
    Walked {
        /// The first carrier's kind.
        a: CurveKind,
        /// The second carrier's kind.
        b: CurveKind,
    },
    /// A closed-form arm could not read the pose ([`MeetFault`]).
    Closed(MeetFault),
}

/// The predicate rows the pose reads under.
mod row {
    pub(super) const TILT: &str = "curve_pair_pose_tilt";
    pub(super) const OFFSET: &str = "curve_pair_pose_offset";
    pub(super) const PARALLEL: &str = "curve_pair_pose_parallel";
    pub(super) const GAP: &str = "curve_pair_pose_gap";
    pub(super) const SENSE: &str = "curve_pair_pose_sense";
    pub(super) const SWING: &str = "curve_pair_pose_swing";
    pub(super) const EXTREME: &str = "curve_pair_pose_extreme";
    pub(super) const ROOT_GAP: &str = "curve_pair_pose_root_gap";
    pub(super) const CLEAR: &str = "curve_pair_pose_clear";
    pub(super) const BASIN: &str = "curve_pair_pose_basin";
    pub(super) const BASIN_HELD: &str = "curve_pair_pose_basin_held";
    pub(super) const LEVEL_ARC: &str = "curve_pair_pose_level_arc";
    pub(super) const TOUCH_ZERO: &str = "curve_pair_pose_touch_zero";
    pub(super) const LEVEL_FLAT: &str = "curve_pair_pose_level_flat";
    pub(super) const SLANT_TIGHT: &str = "curve_pair_pose_slant_tight";
    pub(super) const HELD_ONE_PIECE: &str = "curve_pair_pose_held_one_piece";
    pub(super) const WINDOW_PIECE: &str = "curve_pair_pose_window_piece";
    pub(super) const WINDOW_HALF: &str = "curve_pair_pose_window_half";
    pub(super) const REFLECTION_FAR: &str = "curve_pair_pose_reflection_far";
    pub(super) const CENTRES: &str = "curve_pair_pose_centres";
    pub(super) const RADII: &str = "curve_pair_pose_radii";
    pub(super) const ONE_CURVE: &str = "curve_pair_pose_one_curve";
    pub(super) const BRANCH: &str = "curve_pair_pose_branch";
    pub(super) const ACROSS: &str = "curve_pair_pose_across";
    pub(super) const TOUCH_SPREAD: &str = "curve_pair_pose_touch_spread";
}

/// The coplanar conic subdivision's rows.
const COPLANAR_ROWS: SubdivisionRows = crate::rows::COPLANAR;

/// The coplanar subdivision's touch rows (module docs, "A touch in the
/// subdivision").
const COPLANAR_TOUCH: TouchRows = crate::rows::COPLANAR_TOUCH;

/// **The pose of `a`'s and `b`'s carriers** (module docs), dispatched
/// through the table ([`super::method`]).
///
/// # Errors
///
/// [`PoseRefusal`], as its variants say.
pub fn pose<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let (ka, kb) = (a.carrier().kind(), b.carrier().kind());
    match super::method(ka, kb) {
        Method::Closed(arm) => closed(arm, a, b, &[], band).map_err(PoseRefusal::Closed),
        Method::Walk => Err(PoseRefusal::Walked { a: ka, b: kb }),
    }
}

/// [`pose`] for a closed-form arm.
pub(super) fn closed<T: Decide>(
    arm: Arm,
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    // Where one side is a circle and the other an ellipse, the ellipse
    // comes first, so in one plane the reading against the circle is at
    // its exact foot. Across planes the two-conic arm picks the conic it
    // reads by its crossing, whatever the kinds (`conic_conic`).
    let flipped: Vec<_> = known.iter().copied().map(flip).collect();
    match arm {
        Arm::LineLine => line_line(a, b, band),
        Arm::LineConic => line_conic(a, b, known, band),
        Arm::ConicLine => Ok(swapped(line_conic(b, a, &flipped, band)?)),
        Arm::CircleCircle => conic_conic(a, b, Coplanar::Circles, known, band),
        Arm::EllipseCircle => conic_conic(a, b, Coplanar::Ellipses, known, band),
        Arm::CircleEllipse => Ok(swapped(conic_conic(
            b,
            a,
            Coplanar::Ellipses,
            &flipped,
            band,
        )?)),
        Arm::EllipseEllipse => conic_conic(a, b, Coplanar::Ellipses, known, band),
    }
}

/// A candidate with its two carriers' roles swapped.
fn flip<T: geom_core::Real>(c: Candidate<T>) -> Candidate<T> {
    Candidate {
        point: c.point,
        ta: c.tb,
        tb: c.ta,
        bound: c.bound,
    }
}

/// A pose read with the spans' roles swapped, given back in their order.
fn swapped<T: geom_core::Real>(pose: Pose<T>) -> Pose<T> {
    match pose {
        Pose::Coincident { sense, witnesses } => Pose::Coincident {
            sense,
            witnesses: witnesses.into_iter().map(flip).collect(),
        },
        Pose::Tangent(c) => Pose::Tangent(flip(c)),
        Pose::Generic(cs) => Pose::Generic(cs.into_iter().map(flip).collect()),
        Pose::Concentric => Pose::Concentric,
        Pose::Parallel => Pose::Parallel,
        Pose::Apart => Pose::Apart,
    }
}

/// A line carrier's line.
fn line_of<T: Decide>(c: &Curve3<T>) -> Result<geom::Line<T>, MeetFault> {
    match *c {
        Curve3::Line(line) => Ok(line),
        Curve3::Circle(..)
        | Curve3::Ellipse(..)
        | Curve3::Spiric { .. }
        | Curve3::Nurbs(_)
        | Curve3::Periodic(_) => Err(MeetFault::Desync),
    }
}

/// A line span's two end points, start first.
type Ends<T> = (Point3<T>, Point3<T>);

/// **A line span's two end points**, start first ([`Span::ends`]). A
/// line is open, so a span of one always has ends: an endless one is a
/// span read as the kind it is not, as a carrier of the wrong kind is.
fn line_ends<T: Decide>(s: &Span<'_, T>) -> Result<Ends<T>, MeetFault> {
    let mut ends = s.ends().map(|(_, q)| q);
    match (ends.next(), ends.next()) {
        (Some(p0), Some(p1)) => Ok((p0, p1)),
        _ => Err(MeetFault::Desync),
    }
}

/// A conic carrier's frame.
fn conic_of<T: Decide>(c: &Curve3<T>) -> Result<&dyn Conic<T>, MeetFault> {
    c.conic().ok_or(MeetFault::Desync)
}

/// **How far the pair's two spans reach from `at`**, the point the
/// router reads its offset at: the farther span's reach from below
/// ([`crate::edge_reach_bounds`]), the lever of a tilt whose definite
/// sign is a crossing, and the nearer's from above (its cover's far
/// distance), which bounds where the two can meet, since a meeting is a
/// point of both, and levers a tilt whose Zero is level there.
fn span_reaches<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    at: Point3<T>,
) -> (Lever<T>, Lever<T>) {
    let read = |s: &Span<'_, T>| {
        let (t0, t1) = s.params();
        let cover = s
            .carrier()
            .cover_over(geom::cover::ParamWindow::new(t0, t1))
            .ok();
        let above = crate::ExtentBall::of_cover_from(cover.as_ref(), at).lever();
        crate::edge_reach_bounds(s.carrier(), (t0, t1), at, above)
    };
    let (ra, rb) = (read(a), read(b));
    (ra.below().max(rb.below()), ra.above().min(rb.above()))
}

/// A lower bound on `q`'s distance from the carrier
/// ([`Curve3::clearance`], the one bound), zero where it has none in
/// closed form.
fn clearance<T: Decide>(c: &Curve3<T>, q: Point3<T>) -> T {
    c.clearance(q).unwrap_or_else(T::zero)
}

/// `q`'s foot on `s`'s carrier ([`Curve3::foot`]) seeded at the span's
/// middle, as a seed ([`geom::Foot::seed`]): a candidate the pose reads
/// on, not a certified parameter.
fn foot<T: Decide>(s: &Span<'_, T>, q: Point3<T>) -> T {
    let (t0, t1) = s.params();
    s.carrier().foot(q, geom::mid_param(t0, t1)).seed()
}

/// **Each span's ends, read against the other** (module docs, "The
/// spans' ends"): each end's point on its own carrier, with its foot on
/// the other's.
pub(super) fn end_candidates<T: Decide>(a: &Span<'_, T>, b: &Span<'_, T>) -> Vec<Candidate<T>> {
    let mut out = Vec::with_capacity(4);
    for (ta, point) in a.ends() {
        out.push(Candidate {
            point,
            ta,
            tb: foot(b, point),
            bound: None,
        });
    }
    for (tb, point) in b.ends() {
        out.push(Candidate {
            point,
            ta: foot(a, point),
            tb,
            bound: None,
        });
    }
    out
}

/// How two carriers read as one curve run, from the cosine between
/// their directions or oriented normals levered by `lever`: definitely
/// signed wherever they are one curve.
///
/// # Errors
///
/// The band's escalation, and [`MeetFault::Unsupported`] where the cosine
/// reads zero: carriers the pose read as one curve whose runs read square
/// to each other, which only a degenerate frame gives.
fn sense<T: Decide>(cosine: T, lever: Lever<T>, band: Band) -> Result<Sense, MeetFault> {
    match decide(row::SENSE, Margin::levered(cosine, lever), band)? {
        Sign::Positive => Ok(Sense::Same),
        Sign::Negative => Ok(Sense::Opposite),
        Sign::Zero => Err(MeetFault::Unsupported {
            pose: "one curve whose two runs read square to each other",
        }),
    }
}

/// Whether `margin` is definitely nonzero (`row`).
fn definite_as<T: Decide>(
    row: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<bool, MeetFault> {
    Ok(match decide(row, margin, band)? {
        Sign::Zero => false,
        Sign::Positive | Sign::Negative => true,
    })
}

/// The witnesses of a coincident pair ([`Span::witness_params`] on each
/// span), each placed on the carrier of the span it came from and given
/// its foot on the other by `foot_a`/`foot_b`.
fn witnesses<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    foot_a: &impl Fn(Point3<T>) -> T,
    foot_b: &impl Fn(Point3<T>) -> T,
) -> Vec<Candidate<T>> {
    let ends = |s: &Span<'_, T>, foot: &dyn Fn(Point3<T>) -> T| {
        s.ends().map(|(_, q)| foot(q)).collect::<Vec<_>>()
    };
    let mut out = Vec::new();
    for ta in a.witness_params(ends(b, foot_a)) {
        let point = a.carrier().eval(ta);
        out.push(Candidate {
            point,
            ta,
            tb: foot_b(point),
            bound: None,
        });
    }
    for tb in b.witness_params(ends(a, foot_b)) {
        let point = b.carrier().eval(tb);
        out.push(Candidate {
            point,
            ta: foot_a(point),
            tb,
            bound: None,
        });
    }
    out
}

/// A carrier the polish reads: a line, or a conic's frame.
#[derive(Clone, Copy)]
enum Side<'k, T: geom_core::Real> {
    Line(geom::Line<T>),
    Conic(&'k dyn Conic<T>),
}

impl<T: Decide> Side<'_, T> {
    /// The point and tangent at `t`: the carrier's own
    /// [`Curve3::ders1`].
    fn ders(self, t: T) -> (Point3<T>, Vec3<T>) {
        match self {
            Self::Line(line) => Curve3::Line(line).ders1(t),
            Self::Conic(k) => k.jet_at(t),
        }
    }

    /// **Whether `u` is held at an end of `span` with the window on one
    /// side of it** (module docs, "Held to the spans"): held at an end
    /// ([`super::walk::held`]), and, on a conic, the span's parameter run
    /// and the window's, `reach` metres at the least speed, short of a turn
    /// together (`curve_pair_pose_held_one_piece`), so the window's arcs on
    /// the span all lie on its side of the end and none comes round to the
    /// other. A line never comes round.
    fn held(self, span: &Span<'_, T>, u: T, reach: T) -> Option<geom::ParamSide> {
        let side = super::walk::held(span, u)?;
        let Self::Conic(k) = self else {
            return Some(side);
        };
        let (t0, t1) = span.params();
        let short = T::tau() - (t1 - t0).abs() - reach / k.speed_lo();
        (geom_core::k_stats::prove(row::HELD_ONE_PIECE, Margin::of(short)) == Ok(Sign::Positive))
            .then_some(side)
    }

    /// The most metres the carrier runs between the parameters `t0` and
    /// `t1`: their difference at its greatest speed.
    fn run(self, t0: T, t1: T) -> T {
        let speed = match self {
            Self::Line(line) => line.dir.get().norm(),
            Self::Conic(k) => k.speed_hi(),
        };
        (t1 - t0).abs() * speed
    }

    /// **The second carrier's window, as an arc from `q`** (module docs,
    /// (P1)): every pair of the window within the band's escalation has
    /// its second point within `ball` metres of `crossing`, so within
    /// `radius` of `q = C(t)`. On a line that chord is the arc. On a conic
    /// the parameters `J = [t − ρ, t + ρ]`, `ρ` a quarter turn's worth of
    /// `radius` at the speed at `t`, hold every such point where the span
    /// outside `J` and its copies a turn either side is clear of the ball
    /// by the span's own walk
    /// ([`Span::clear_by`]), each piece's place against `J` proved
    /// (`curve_pair_pose_window_piece`). The arc from `q` is then at most
    /// `J`'s run at the greatest speed on it. Where that is not read, or
    /// `J` is half a turn or more, the arc is at most half the carrier's
    /// turn at its greatest speed, taken the shorter way round.
    ///
    /// Returns the arc and the angle about `t` it covers, `ρ` or the half
    /// turn, for [`Self::curvature_near`].
    fn arc_from(
        self,
        span: &Span<'_, T>,
        t: T,
        radius: T,
        (crossing, ball): (Point3<T>, T),
        band: Band,
    ) -> (T, T) {
        let Self::Conic(k) = self else {
            return (radius, T::zero());
        };
        let whole = (T::pi() * k.speed_hi(), T::pi());
        let half = T::pi() / T::from_f64(2.0);
        let rho = half * radius / k.jet_at(t).1.norm();
        if geom_core::k_stats::param_order(row::WINDOW_HALF, T::pi(), rho) != Ok(Sign::Positive) {
            return whole;
        }
        let (from, to) = (t - rho, t + rho);
        // The span's parameters outside `J` and its copies a turn either
        // side: the carrier outside `J` is one turn, `(t + ρ, t − ρ + τ)`,
        // and a span of at most a turn about `t` meets it, and the copy a
        // turn back, in one piece each. A piece is read where its ends'
        // order is proved, and one proved empty is none.
        let tau = T::tau();
        let mut outside = Vec::with_capacity(2);
        if span.ended() {
            let (t0, t1) = span.params();
            let (lo, hi) = (t0.min(t1), t0.max(t1));
            for (x, y) in [
                (lo.max(to - tau), hi.min(from)),
                (lo.max(to), hi.min(from + tau)),
            ] {
                match geom_core::k_stats::param_order(row::WINDOW_PIECE, y, x) {
                    Ok(Sign::Positive) => outside.push((x, y)),
                    Ok(Sign::Negative | Sign::Zero) => {}
                    Err(_) => return whole,
                }
            }
        } else {
            outside.push((to, from + tau));
        }
        let clear = outside
            .into_iter()
            .all(|piece| Span::new(span.carrier(), piece).clear_by(crossing, ball, band));
        if !clear {
            return whole;
        }
        (rho * speeds_within(k, t, rho).1, rho)
    }

    /// The angle at the least speed that `reach` metres of the carrier
    /// cover, the most a run of that length turns through: on a line,
    /// none.
    fn angle_of(self, reach: T) -> T {
        match self {
            Self::Line(_) => T::zero(),
            Self::Conic(k) => reach / k.speed_lo(),
        }
    }

    /// A bound on the curvature over the angles within `rho` of `t`: zero
    /// on a line; on a conic `ab/|C′|³` at its least speed there
    /// ([`speeds_within`]).
    fn curvature_near(self, t: T, rho: T) -> T {
        let Self::Conic(k) = self else {
            return T::zero();
        };
        let (a, b) = k.semi_axes();
        a * b / speeds_within(k, t, rho).0.powi(3)
    }
}

/// **A conic's least and greatest speed over the angles within `rho` of
/// `t`**: `|C′|² = b² + (a² − b²)·sin²θ`, least where the angles come
/// nearest `θ ≡ 0 (mod π)` and greatest nearest `θ ≡ π/2` (for `a ≥ b`;
/// the other way round otherwise), each its extreme overall where the
/// angles reach it.
fn speeds_within<T: Decide>(k: &dyn Conic<T>, t: T, rho: T) -> (T, T) {
    let (a, b) = k.semi_axes();
    let half = T::pi() / T::from_f64(2.0);
    let (slow, fast) = (
        (a - b).select_le_zero(t - half, t),
        (a - b).select_le_zero(t, t - half),
    );
    let off = |x: T| {
        (x.reduce_periodic_centred(T::pi()).abs() - rho)
            .max(T::zero())
            .min(half)
    };
    let (long, short) = (k.speed_hi(), k.speed_lo());
    let speed = |s2: T| (short.powi(2) + (long.powi(2) - short.powi(2)) * s2).sqrt();
    (
        speed(off(slow).sin().powi(2)),
        speed(off(fast).cos().powi(2)),
    )
}

/// A crossing or touch of the other's plane, polished to the carriers'
/// closest pair (module docs, "Across planes").
struct Polish<'k, T: geom_core::Real> {
    /// The first span, the polish held to it.
    span_a: &'k Span<'k, T>,
    /// The second.
    span_b: &'k Span<'k, T>,
    /// The first carrier.
    a: Side<'k, T>,
    /// The second.
    b: Side<'k, T>,
    /// Where the polish starts: the crossing on the first, its foot on
    /// the second.
    start: (T, T),
    /// `ℓ`: the metres along the first carrier, from `start`, where it is
    /// within the band's escalation of the second's plane.
    reach: T,
    /// A lower bound on the start point's distance from the second
    /// carrier ([`Curve3::clearance`]).
    clear: T,
    /// What the basin's sine is levered by.
    lever: Lever<T>,
}

impl<T: Decide> Polish<'_, T> {
    /// The closest pair's candidate, or `None` where no point of the
    /// window is within the band of the second carrier (module docs,
    /// "Across planes" and "The gap's lower bound").
    ///
    /// # Errors
    ///
    /// [`Unsettled::Meter`] on `curve_pair_pose_basin` where the basin's
    /// inequality is not proved.
    fn closest(&self, band: Band) -> Result<Option<Candidate<T>>, MeetFault> {
        let (s0, t0) = self.start;
        if decide(row::CLEAR, Margin::of(self.clear - self.reach), band) == Ok(Sign::Positive) {
            return Ok(None);
        }
        let two = T::from_f64(2.0);
        let e = T::from_f64(band.escalate());
        let region = (self.reach + e) * two;
        let hold = |span: &Span<'_, T>| span.ended().then(|| span.params());
        // The start on the turn about each span's middle, so a hold
        // clamps a parameter to its own span, not a turn away from it.
        let (s0, t0) = (self.span_a.branch(s0), self.span_b.branch(t0));
        // Every pair of the window within the band's escalation has its
        // first point within `ℓ` of the crossing along the first carrier,
        // and its second within the escalation of that: both within
        // `ℓ + e` of the crossing, in metres. Where the ball of that
        // radius about the crossing is clear of either span, by the span's
        // own walk, the window on the spans holds no such pair.
        let crossing = self.a.ders(s0).0;
        let ball = self.reach + e;
        if self.span_a.clear_by(crossing, ball, band) || self.span_b.clear_by(crossing, ball, band)
        {
            return Ok(None);
        }
        let (s, t) = super::polish::pair_held(
            |s| self.a.ders(s),
            |t| self.b.ders(t),
            (s0, t0),
            |_| true,
            (hold(self.span_a), hold(self.span_b)),
        );
        let ((p, da), (q, db)) = (self.a.ders(s), self.b.ders(t));
        // Every pair of the window within the escalation has its first
        // point within `region` of the start along the first carrier, and
        // the polished pair is within its travel of it; its second point
        // is within the ball about the crossing, so within the ball and
        // `q`'s own distance from the crossing of `q`, read as an arc
        // from `q` (module docs, (P1)).
        let along_a = region + self.a.run(s0, s);
        let (arc_b, near_b) = self.b.arc_from(
            self.span_b,
            t,
            ball + (q - crossing).norm(),
            (crossing, ball),
            band,
        );
        let span = along_a.max(arc_b);
        let curvature =
            self.a.curvature_near(s0, self.a.angle_of(along_a)) + self.b.curvature_near(t, near_b);
        // A side held at an end of its span reads its window's arcs
        // into the span alone, so the gap is read across the other
        // side's tangent with the held arc a half-line (module docs, "A
        // held end's floor").
        let (held_a, held_b) = (
            self.a.held(self.span_a, s, span),
            self.b.held(self.span_b, t, span),
        );
        let into = super::polish::into_span;
        let mut along = super::polish::Along::two_sided((p, da), (q, db));
        if let Some(side) = held_b {
            along = along.held_across(q - p, da, into(db, side));
        }
        if let Some(side) = held_a {
            along = along.held_across(p - q, db, into(da, side));
        }
        let two_sided =
            super::polish::bound((row::BASIN, Some(self.lever)), along, curvature, (e, span));
        // The walk's fallback (`polish::two_or_one_sided`), differing from
        // the walk's window in three ways: the carriers' own derivatives,
        // a conic's and a line's, which have no crease, so the
        // half-tangent is the tangent turned into the span; the conic's
        // curvature bound, over both sides of the end; and the two-sided
        // reading carries a held side's floor. Where neither reading
        // bounds a reach, the pose refuses on its basin.
        let one = || {
            let (side_a, side_b) = (held_a?, held_b?);
            super::polish::bound(
                (row::BASIN_HELD, None),
                super::polish::Along::one_sided((p, into(da, side_a)), (q, into(db, side_b))),
                curvature,
                (e, span),
            )
        };
        let Some(bound) = super::polish::two_or_one_sided(two_sided, one) else {
            return Err(MeetFault::Unsettled(Unsettled::Meter { row: row::BASIN }));
        };
        let bound = WindowBound {
            spans: WindowSpans::Held,
            ..bound
        };
        Ok(Some(Candidate {
            point: p,
            ta: s,
            tb: t,
            bound: Some(bound),
        }))
    }
}

/// The metres a line span's two ends ([`line_ends`]) lie from the
/// origin, as evaluated.
fn ends<T: Decide>((p0, p1): Ends<T>) -> T {
    (p0 - Point3::origin()).norm() + (p1 - Point3::origin()).norm()
}

/// The projection of a line span's two ends on `line`: their distances
/// along it from its origin. Its one caller, `line_conic`, hands it the
/// line with its direction already normalised (`unit`), so the dot is
/// the metres and is not divided again as [`geom::Line::height`] would.
fn on_line<T: Decide>(
    geom::Line { origin: o, dir: u }: geom::Line<T>,
    (p0, p1): Ends<T>,
) -> (T, T) {
    ((p0 - o).dot(u.get()), (p1 - o).dot(u.get()))
}

/// Where the router sends a line's pair (module docs, "The router").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Route {
    /// The projections definitely apart: no place to meet.
    Apart,
    /// Definitely apart over every place they can meet.
    Parallel,
    /// Crossing: the tilt definite over the farther span's reach.
    Crossing,
    /// Level over where they can meet, the offset there read zero.
    Level,
}

/// **The router** of two lines, or of a line and a conic's plane (module
/// docs, "The router"): `tilt` the sine of the pair's angle, `rate` the
/// most metres the offset moves per metre along the first line, `offset`
/// the second's distance (or the plane's height) at the overlap's middle,
/// `at` the overlap ([`OnOneLine`], carrying its own rounding), `lever`
/// the farther span's reach from the overlap's middle from below and
/// `over` the nearer's from above ([`span_reaches`]).
///
/// The parallel argument runs first: where the offset less its most
/// variation over the overlap and the escalation either side decides
/// positive (`curve_pair_pose_parallel`), no point meets, whatever the
/// tilt reads. Then the gap: where the projections are disjoint and
/// their gap decides positive (`curve_pair_pose_gap`), no point meets.
/// Only otherwise does the tilt route: zero over the nearer span's
/// reach, level where the offset reads zero, and [`Unsettled::Meter`]
/// where it does not, the parallel margin having not decided; otherwise
/// definite over the farther span's reach, a crossing.
///
/// A tilt not Zero over `over` is never Zero over `lever`, for two
/// reasons. The first span is a line for every caller, whose reach is
/// exact, so `lever` is never shorter than `over`:
/// `max(r, b) ≥ r ≥ min(r, a)`. And a conic whose cover cannot be read
/// (non-finite data) makes `over` poison, which reads no Zero; the same
/// data poisons the samples `lever` is read from below by, so the
/// crossing read escalates.
///
/// # Errors
///
/// The band's escalation of the tilt or of the
/// offset, and the Meter refusal above.
fn route<T: Decide>(
    tilt: T,
    rate: T,
    (offset, offset_is): (T, Offset),
    at: OnOneLine<T>,
    (lever, over): (Lever<T>, Lever<T>),
    band: Band,
) -> Result<Route, MeetFault> {
    let length = at.extent().max(T::zero());
    let e = T::from_f64(band.escalate());
    let half = length * T::from_f64(0.5) + e;
    let parallel = Margin::of(offset - rate * half);
    if decide(row::PARALLEL, parallel, band) == Ok(Sign::Positive) {
        return Ok(Route::Parallel);
    }
    if decide(row::GAP, Margin::of(at.apart()), band) == Ok(Sign::Positive) {
        return Ok(Route::Apart);
    }
    if decide(row::TILT, Margin::levered(tilt, over), band) != Ok(Sign::Zero) {
        return if definite_as(row::TILT, Margin::levered(tilt, lever), band)? {
            Ok(Route::Crossing)
        } else {
            // Why no input reaches here: the doc's last paragraph.
            unreachable!(
                "a tilt not Zero over the nearer span's reach read Zero over the farther's"
            )
        };
    }
    let offset = decide(row::OFFSET, Margin::of(offset), band);
    match (offset, offset_is) {
        (Ok(Sign::Positive | Sign::Negative), _) => {
            Err(MeetFault::Unsettled(Unsettled::Meter { row: row::OFFSET }))
        }
        (Ok(Sign::Zero), _) | (Err(_), Offset::Height) => Ok(Route::Level),
        (Err(gap), Offset::Distance) => Err(MeetFault::Escalated(gap)),
    }
}

/// **What the router's offset measures**, which fixes what a level offset
/// in the band's gap reads (module docs, "The router").
#[derive(Clone, Copy)]
enum Offset {
    /// The second line's distance from the first: in the gap, the band's
    /// escalation.
    Distance,
    /// The line's height above the conic's plane, a lower bound on its
    /// distance from the conic: in the gap the line is level, and the arm
    /// in the plane reads it, its candidates read on both spans.
    Height,
}

/// Whether two lines are one curve within the band over both whole spans
/// (`curve_pair_pose_offset`): each span's ends lie within the zero band
/// of the other's carrier. A line's distance from another is affine in
/// its parameter, so its ends bound it over the span.
fn one_line<T: Decide>(
    (a, la): (Ends<T>, geom::Line<T>),
    (b, lb): (Ends<T>, geom::Line<T>),
    band: Band,
) -> Result<bool, MeetFault> {
    // One line is an identity: any end decided off the other's line
    // answers "not one line", whatever an earlier end read
    // ([`read_identity`]).
    let offsets = [(a, lb), (b, la)]
        .into_iter()
        .flat_map(|((p0, p1), other)| {
            [p0, p1].map(|q| (row::OFFSET, Margin::of(other.distance(q))))
        });
    Ok(read_identity(offsets, band).holds()?)
}

/// Two lines (module docs).
fn line_line<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (la, lb) = (line_of(a.carrier())?, line_of(b.carrier())?);
    let (
        geom::Line {
            origin: o1,
            dir: d1,
        },
        geom::Line {
            origin: o2,
            dir: d2,
        },
    ) = (la, lb);
    let (u1, u2) = (UnitVec3::squared(d1.get()), UnitVec3::squared(d2.get()));
    let n = u1.get().cross(u2.get());
    let s = n.norm();
    // Read from `a`'s span start along its carrier, never from either
    // carrier's origin, and charged for how each end was evaluated:
    // `origin + dir·t` rounds at `|origin| + |dir|·|t|`, which a far
    // origin makes far larger than the end's own distance
    // (`geom_core::one_line`, "The charge").
    let (ends_a, ends_b) = (line_ends(a)?, line_ends(b)?);
    let produced = |geom::Line { origin: o, dir: d }: geom::Line<T>, s: &Span<'_, T>| {
        let (t0, t1) = s.params();
        (o - Point3::origin()).norm() * T::from_f64(2.0) + d.get().norm() * (t0.abs() + t1.abs())
    };
    let (a0, _) = ends_a;
    let at = on_one_line(u1.get(), ends_a, ends_b, produced(la, a) + produced(lb, b));
    let (unit1, unit2) = (
        geom::Line {
            origin: o1,
            dir: u1,
        },
        geom::Line {
            origin: o2,
            dir: u2,
        },
    );
    let mid = a0 + u1.get() * at.mid();
    let off = unit2.distance(mid);
    let (arm, over) = span_reaches(a, b, mid);
    let crossing = || {
        let along = (o2 - o1).cross(u2.get()).dot(n) / s.powi(2);
        let point = o1 + u1.get() * along;
        Pose::Generic(vec![Candidate {
            point,
            ta: la.param(point),
            tb: lb.param(point),
            bound: None,
        }])
    };
    match route(
        s,
        s / u1.get().dot(u2.get()).abs(),
        (off, Offset::Distance),
        at,
        (arm, over),
        band,
    )? {
        Route::Apart => Ok(Pose::Apart),
        Route::Parallel => Ok(Pose::Parallel),
        Route::Crossing => Ok(crossing()),
        Route::Level if one_line((ends_a, unit1), (ends_b, unit2), band)? => Ok(Pose::Coincident {
            sense: sense(u1.get().dot(u2.get()), arm, band)?,
            witnesses: witnesses(a, b, &|q| la.param(q), &|q| lb.param(q)),
        }),
        // Level where they can meet but apart past the band along a span:
        // a shallow corner, read as the crossing the farther span's tilt
        // decides.
        Route::Level if definite_as(row::TILT, Margin::levered(s, arm), band)? => Ok(crossing()),
        Route::Level => Err(MeetFault::Unsettled(Unsettled::Meter { row: row::TILT })),
    }
}

/// A line `a` and a conic `b` (module docs).
fn line_conic<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let line = line_of(a.carrier())?;
    let k = conic_of(b.carrier())?;
    let geom::Line { origin: o, dir: d } = line;
    let u = UnitVec3::squared(d.get());
    let unit = geom::Line { origin: o, dir: u };
    let e = T::from_f64(band.escalate());
    let n = k.frame().axis();
    let den = u.get().dot(n);
    // Where they can meet (module docs): the line's span and the conic's
    // extent, both projected on the line.
    let c = geom::plane_offset(o, u.get(), k.frame().origin());
    let ends_a = line_ends(a)?;
    // A conic's extent is no second span: its projection is read from the
    // line's origin, charged as the router's screens are. That anchor
    // charges the carrier origin's metres once, where a read from the
    // span's start would charge both ends' evaluation, twice as many
    // (`work/cpair/the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin`).
    let at = OnOneLine::of(
        on_line(unit, ends_a),
        (c - k.speed_hi(), c + k.speed_hi()),
        geom_core::rounding_charge(ends(ends_a)),
    );
    let mid = o + u.get() * at.mid();
    let height = geom::plane_offset(k.frame().origin(), n, mid);
    let (arm, over) = span_reaches(a, b, mid);
    match route(
        den,
        den.abs(),
        (height.abs(), Offset::Height),
        at,
        (arm, over),
        band,
    )? {
        Route::Apart => return Ok(Pose::Apart),
        Route::Parallel => return Ok(Pose::Parallel),
        Route::Crossing => {
            // The line's height above the plane runs `|d̂·n̂|` per metre,
            // so a point of it within the band's escalation `e` of the
            // plane is within `(e + |h|)/|d̂·n̂|` of the crossing's seed,
            // `h` the seed's own height above the plane: the seed, `o +
            // d̂·t` with `t = (c − o)·n̂ / d̂·n̂` in `f64`, is read where it
            // landed, a distance, rather than charged for its rounding.
            let point = o + u.get() * (geom::plane_offset(o, n, k.frame().origin()) / den);
            let height = geom::plane_offset(k.frame().origin(), n, point).abs();
            let polish = Polish {
                span_a: a,
                span_b: b,
                a: Side::Line(line),
                b: Side::Conic(&k),
                start: (line.param(point), foot(b, point)),
                reach: (e + height) / den.abs(),
                clear: clearance(b.carrier(), point),
                lever: arm,
            };
            return Ok(Pose::Generic(polish.closest(band)?.into_iter().collect()));
        }
        Route::Level => {}
    }
    // In the plane: the conic's signed distance from the line, along the
    // line's normal in the plane, a first harmonic in the conic's angle.
    let m = n.cross(u.get()).normalize();
    let c0 = geom::plane_offset(o, m, k.frame().origin());
    let (cp, sp) = k.along(m);
    let swing = (cp.powi(2) + sp.powi(2)).sqrt();
    if !definite_as(row::SWING, Margin::of(swing), band)? {
        return Err(MeetFault::Unsupported {
            pose: "a conic the band reads as thin as the line it lies beside",
        });
    }
    let range = Range::first_harmonic(
        c0,
        swing,
        k.belly_along(o, m),
        Azimuth::of(sp, cp).principal_as(row::BRANCH, Band::least()),
    );
    // The pair's distance is `√(R² + H²)`, `R` the residual above and `H`
    // the conic's offset along `v = û × m`, the line's other normal: a
    // line the tilt read level lies over the plane by a slant, which `H`
    // holds (module docs, "A slanted pair in one plane").
    let v = u.get().cross(m);
    let (hc, hs) = k.along(v);
    let level = Slant {
        r: (c0, cp, sp),
        h: (geom::plane_offset(o, v, k.frame().origin()), hc, hs),
        at: range.at_hi,
        swing,
        e,
        zero: T::from_f64(band.zero()),
    };
    // A slanted pair at a band too tight for the slant argument is not
    // read (module docs, "One meeting to a part"): flat where `H`'s
    // swing is proved below the zero.
    if !level.flat() && band.escalate() < SLANT_RATIO * band.zero() {
        return Err(MeetFault::Unsettled(Unsettled::Meter {
            row: row::SLANT_TIGHT,
        }));
    }
    // A window pair's conic point lies on the stretch, within its run at
    // the greatest speed of the candidate's, and its line point within
    // `e` of that: where the ball of that reach about the candidate's
    // point is clear of either span, by the span's own walk, the stretch
    // holds no pair on the spans.
    let place = |theta: T| -> Result<Option<Candidate<T>>, MeetFault> {
        let (floor, run) = level.read(theta)?;
        let point = k.point_at(theta);
        let reach = k.speed_hi() * run + level.e;
        if a.clear_by(point, reach, band) || b.clear_by(point, reach, band) {
            return Ok(None);
        }
        Ok(Some(Candidate {
            point,
            ta: line.param(point),
            tb: theta,
            bound: Some(WindowBound {
                floor,
                reach,
                bend: T::zero(),
                spans: WindowSpans::MayPass,
            }),
        }))
    };
    // One found meeting is deflated, its reflection offered, unless the
    // reflection is shown no root (module docs, "The spans' ends, and
    // deflation"): then both roots are read (`Slant::roots`, which
    // decides no gap between them), and ringing drops the one within the
    // band of the found meeting's point.
    let harmonic = match known {
        [j] if !reflected(&k, &range, j, e, band) => {
            let (lo, hi) = level.roots();
            Harmonic::Cross(vec![lo, hi])
        }
        _ => owed(
            &k,
            &range,
            &known.iter().map(|c| c.tb).collect::<Vec<_>>(),
            band,
        )?,
    };
    Ok(match harmonic {
        Harmonic::Apart => Pose::Apart,
        // A touch whose extreme is within the zero is one run, read at the
        // extreme; otherwise the zero set may part about the two roots,
        // and both are read (module docs, "A slanted pair in one plane").
        Harmonic::Touch(theta) if level.within_zero(theta) => match place(theta)? {
            Some(c) => {
                spread_held(angle_spread(k.speed_hi(), c.tb), band)?;
                Pose::Tangent(c)
            }
            None => Pose::Apart,
        },
        Harmonic::Touch(_) => {
            let (lo, hi) = level.roots();
            let mut out = Vec::with_capacity(2);
            for theta in [lo, hi] {
                out.extend(place(theta)?);
            }
            Pose::Generic(out)
        }
        Harmonic::Cross(thetas) => {
            let mut out = Vec::with_capacity(2);
            for theta in thetas {
                out.extend(place(theta)?);
            }
            Pose::Generic(out)
        }
    })
}

/// **A line and a conic in one plane, the line over it by a slant**
/// (module docs, "A slanted pair in one plane"): the residual
/// `R(θ) = r₀ + r_c·cos θ + r_s·sin θ` in the plane, `H(θ)` likewise
/// along the line's other normal, so the distance is `√(R² + H²)`; `e`
/// is the band's escalation.
struct Slant<T> {
    r: (T, T, T),
    h: (T, T, T),
    /// `R`'s greatest's angle, `θ_hi`, and its swing `A` (`H`'s is its
    /// lean, [`Self::lean`]).
    at: T,
    swing: T,
    e: T,
    zero: T,
}

/// The least ratio `k = e/z` of the band's escalation to its zero at
/// which a slanted pair's side arc holds at most one part of its zero set:
/// past the real root of `(k − 1)³ = k`, `2.3247…` (one plus the plastic
/// number), with a margin for the rounding its premises' decisions carry
/// (module docs, "No second part on a side arc").
const SLANT_RATIO: f64 = 2.33;

impl<T: Decide> Slant<T> {
    /// Whether `H`'s lean ([`Self::lean`]) is proved below the band's zero
    /// (`curve_pair_pose_level_flat`): `H` within the zero of a constant,
    /// the pair read as in one plane.
    fn flat(&self) -> bool {
        geom_core::k_stats::prove(row::LEVEL_FLAT, Margin::of(self.zero - self.lean()))
            == Ok(Sign::Positive)
    }

    /// `H`'s lean, its swing `√(h_c² + h_s²)` about `h₀` (`R`'s is
    /// [`Self::swing`]).
    fn lean(&self) -> T {
        let (_, hc, hs) = self.h;
        (hc.powi(2) + hs.powi(2)).sqrt()
    }

    /// Whether the distance at `theta` is proved within the band's zero
    /// (`curve_pair_pose_touch_zero`, a bound's inequality against the
    /// zero level).
    fn within_zero(&self, theta: T) -> bool {
        let gap = (Self::at(self.r, theta).powi(2) + Self::at(self.h, theta).powi(2)).sqrt();
        geom_core::k_stats::prove(row::TOUCH_ZERO, Margin::of(self.zero - gap))
            == Ok(Sign::Positive)
    }

    /// The residual's two roots, `θ_hi ± φ₀` with `cos φ₀ = −c₀/A`, about
    /// either extreme: where `R` does not reach zero they are the extreme
    /// nearer it.
    fn roots(&self) -> (T, T) {
        let (c0, _, _) = self.r;
        let zero = (-c0 / self.swing).max(-T::one()).min(T::one()).acos();
        (self.at - zero, self.at + zero)
    }

    fn at((c0, c, s): (T, T, T), theta: T) -> T {
        c0 + c * theta.cos() + s * theta.sin()
    }

    fn slope((_, c, s): (T, T, T), theta: T) -> T {
        s * theta.cos() - c * theta.sin()
    }

    /// **A candidate's stretch, its floor and its run**: the arc of
    /// `|R| ≤ e` holding `theta`, the stretch outside which the pair is
    /// past the band's escalation; on it the least `|H|`, a lower bound
    /// on the distance (`|H| ≤ √(R² + H²)`); and the most angle from
    /// `theta` to the arc's ends. An arc of half a turn or more is not
    /// read (`curve_pair_pose_level_arc`).
    fn read(&self, theta: T) -> Result<(T, T), MeetFault> {
        let (c0, _, _) = self.r;
        let (swing, at) = (self.swing, self.at);
        let acos = |x: T| x.max(-T::one()).min(T::one()).acos();
        // `R = c₀ + A cos φ`, `φ = θ − at`: within `e` where
        // `φ₁ ≤ |φ| ≤ φ₂`.
        let (arg_near, arg_far) = ((self.e - c0) / swing, (-self.e - c0) / swing);
        let (near, far) = (acos(arg_near), acos(arg_far));
        let phi = (theta - at).reduce_periodic_centred(T::tau());
        // The arc holding `θ`: one about `at` where `R ≤ e` all round it
        // (`φ₁ = 0`), one about `at + π` where `R ≥ −e` all round that
        // (`φ₂ = π`), and otherwise the one on `θ`'s side of `at`. The
        // first two are read on the residual's own values, clear of the
        // angles' cut at `±π`.
        let pick =
            |x: T, (a, b): (T, T), (c, d): (T, T)| (x.select_le_zero(a, c), x.select_le_zero(b, d));
        let side = pick(phi, (at - far, at - near), (at + near, at + far));
        let below = pick(T::one() + arg_far, (at + near, at + T::tau() - near), side);
        let (from, to) = pick(T::one() - arg_near, (at - far, at + far), below);
        let half = T::pi();
        if geom_core::k_stats::prove(row::LEVEL_ARC, Margin::of(half - (to - from)))
            != Ok(Sign::Positive)
        {
            return Err(MeetFault::Unsettled(Unsettled::Meter {
                row: row::LEVEL_ARC,
            }));
        }
        // The least `|H|` over the arc: at an end, at the one turn of `H`
        // within it (the arc is short of half a turn), or zero where `H`
        // changes sign.
        let (h0, _, _) = self.h;
        let lean = self.lean();
        let (h_from, h_to) = (Self::at(self.h, from), Self::at(self.h, to));
        let (d_from, d_to) = (Self::slope(self.h, from), Self::slope(self.h, to));
        let turn = d_from.select_le_zero(h0 - lean, h0 + lean);
        let pair = |x: T, y: T| (x * y).select_le_zero(T::zero(), x.abs().min(y.abs()));
        let floor = (d_from * d_to)
            .select_le_zero(pair(h_from, turn).min(pair(turn, h_to)), pair(h_from, h_to));
        // The run read on the arc's own branch; the candidate keeps its
        // angle as found.
        let mid = (from + to) / T::from_f64(2.0);
        let on_arc = mid + (theta - mid).reduce_periodic_centred(T::tau());
        let run = (on_arc - from).abs().max((to - on_arc).abs());
        Ok((floor, run))
    }
}

/// What two conics in one plane are, `a` the one parametrized: the arm's
/// kinds, fixed by the table.
#[derive(Clone, Copy)]
enum Coplanar {
    /// Two circles.
    Circles,
    /// Two conics, not both circles: an ellipse and a circle are read as
    /// two ellipses, the circle's level that of its equal semi-axes.
    Ellipses,
}

/// The conic `k`'s height above the plane through `on` with unit normal
/// `n`: its first harmonic's constant, cosine and sine parts, in metres.
/// The plane door's sinusoid is this one
/// ([`crate::curve_face::plane_crossing`]).
pub(crate) fn height<T: Decide>(k: &dyn Conic<T>, on: Point3<T>, n: Vec3<T>) -> (T, T, T) {
    let (cp, sp) = k.along(n);
    (geom::plane_offset(on, n, k.frame().origin()), cp, sp)
}

/// Two conics, `a` the one parametrized in one plane (module docs).
fn conic_conic<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    coplanar: Coplanar,
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (ka, kb) = (conic_of(a.carrier())?, conic_of(b.carrier())?);
    let ball = |c: &Curve3<T>| crate::ExtentBall::of_covered(c.cover().as_ref().ok());
    let arm = ball(a.carrier()).lever().max(ball(b.carrier()).lever());
    let ha = height(&ka, kb.frame().origin(), kb.frame().axis());
    let hb = height(&kb, ka.frame().origin(), ka.frame().axis());
    let swing = |(_, cp, sp): (T, T, T)| (cp.powi(2) + sp.powi(2)).sqrt();
    let (swing_a, swing_b) = (swing(ha), swing(hb));
    let (belly_a, belly_b) = (
        ka.belly_along(kb.frame().origin(), kb.frame().axis()),
        kb.belly_along(ka.frame().origin(), ka.frame().axis()),
    );
    let sides = [(swing_a, ha), (swing_b, hb)];
    // The one reading of a conic's height against the other's plane:
    // clear of it all round is apart, whatever the swing reads.
    for (swing, belly, (c0, _, _)) in [(swing_a, belly_a, ha), (swing_b, belly_b, hb)] {
        if clear_all_round(c0, swing, belly, band) {
            return Ok(Pose::Apart);
        }
    }
    // Both swings are read before either is acted on, so the route does
    // not depend on which conic comes first. A conic whose swing reads
    // zero lies in a plane parallel to the other's, in it to within the
    // slant `|c₀|` plus its swing (the larger of the two where both are
    // level). A height the screen did not clear is a slant, not a
    // meeting: the arms in one plane read the pair, and each candidate
    // they place is read on both spans, the slant included. Only where
    // neither reads zero does a swing in the gap escalate.
    let swings = sides.map(|(swing, _)| decide(row::SWING, Margin::of(swing), band));
    let mut slant: Option<T> = None;
    for (read, (swing, (c0, _, _))) in swings.iter().zip(sides) {
        if *read != Ok(Sign::Zero) {
            continue;
        }
        let level = c0.abs() + swing;
        slant = Some(slant.map_or(level, |other| other.max(level)));
    }
    let Some(slant) = slant else {
        for read in swings {
            read?;
        }
        let steep = |(c0, _, _): (T, T, T), swing: T, k: &dyn Conic<T>| {
            let (p, q) = k.semi_axes();
            (swing.powi(2) - c0.powi(2)) / p.abs().max(q.abs()).powi(2)
        };
        let by_b = steep(hb, swing_b, &kb) - steep(ha, swing_a, &ka);
        // Which conic to read across the other's plane: a heuristic's
        // choice (ledger row F33). **At a tie the first is read**: either
        // reading is sound there, each deciding or escalating on its own
        // rows, and the window certifies a tangent pair's touch either way;
        // the difference is antisymmetric bit for bit, so at `f64` only an
        // exact zero reads by argument order.
        let across_b = match choose(row::ACROSS, Margin::of(by_b), "F33") {
            Pick::Positive => true,
            Pick::Negative | Pick::Tie => false,
        };
        if across_b {
            let flipped: Vec<_> = known.iter().copied().map(flip).collect();
            let read = transverse(
                (b, &kb, &ka, a),
                hb,
                (swing_b, belly_b),
                arm,
                &flipped,
                band,
            );
            return Ok(swapped(read?));
        }
        return transverse((a, &ka, &kb, b), ha, (swing_a, belly_a), arm, known, band);
    };
    let oriented = |k: &dyn Conic<T>| k.frame().axis() * (k.semi_axes().0 * k.semi_axes().1);
    if one_curve(&ka, &kb, band)? {
        let (foot_a, foot_b) = (|q| foot(a, q), |q| foot(b, q));
        return Ok(Pose::Coincident {
            sense: sense(
                oriented(&ka).normalize().dot(oriented(&kb).normalize()),
                arm,
                band,
            )?,
            witnesses: witnesses(a, b, &foot_a, &foot_b),
        });
    }
    let place = |theta: T| {
        let point = ka.point_at(theta);
        Candidate {
            point,
            ta: theta,
            tb: foot(b, point),
            bound: None,
        }
    };
    match coplanar {
        Coplanar::Circles => coplanar_circles(&ka, &kb, slant, &place, known, band),
        Coplanar::Ellipses => coplanar_ellipses(a, &ka, &kb, &place, band),
    }
}

/// **Whether two conics read in one plane are one curve** (module docs):
/// [`one_curve_gap`] read zero.
///
/// # Errors
///
/// The band's escalation.
pub(super) fn one_curve<T: Decide>(
    e: &dyn Conic<T>,
    f: &dyn Conic<T>,
    band: Band,
) -> Result<bool, MeetFault> {
    Ok(!definite_as(
        row::ONE_CURVE,
        Margin::of(one_curve_gap(e, f)),
        band,
    )?)
}

/// **The one-curve reading's margin** (module docs): a bound on how far
/// a point of either conic lies from the other, a tilt out of the plane
/// included. Each conic is `c + L·x` over the unit circle's `x`, with
/// `L = [a·û, b·v̂]`. For any orthogonal `Q`, pairing `c_e + L_e·x` with
/// `c_f + L_f·Q·x` pairs every point of each with a point of the other
/// within `|c_e − c_f| + ‖L_e − L_f·Q‖` ([`frame_gap`]). The stored order
/// and signs of the semi-axes change `L` by an orthogonal factor, which
/// `Q` absorbs. The gap read from either side is the same but for
/// rounding, and the larger is taken, so the gap does not depend on the
/// order of the pair.
///
/// **The gap carries the rounding its conics' stored semi-axes
/// amplify**, so no reading of it is bare: carrier terms, which the run's
/// admission rule (`geom_core::tolerance::admissible`) does not cover,
/// read here on this chain's own derived count rather than
/// [`crate::carrier_resolution`]'s fitted one. With `m` a conic's larger
/// semi-axis magnitude, the computed
/// semi-axis vectors are off by `√17·u·m` (`v̂ = n̂ × û` by `4u`), the
/// computed `Q`'s columns are unit to within about `4.5u`, and its
/// products and the norms round by `4u·m` and `5u·g`: within
/// `12.1u·(m_e + m_f) + 5u·g` of a gap the bound holds for, inside
/// [`geom_core::rounding_charge`]`(m_e + m_f + g)`. A degenerate side (a zero
/// semi-axis, or `a·b·a′·b′` underflowing) has no way up, and its gap
/// is NaN: the reading escalates.
pub(super) fn one_curve_gap<T: Decide>(e: &dyn Conic<T>, f: &dyn Conic<T>) -> T {
    let reach = |k: &dyn Conic<T>| k.speed_hi();
    let gap =
        (e.frame().origin() - f.frame().origin()).norm() + frame_gap(e, f).max(frame_gap(f, e));
    gap + geom_core::rounding_charge(reach(e) + reach(f) + gap)
}

/// `‖L_e − L_f·Q‖`, the operator norm, for the `Q` that turns `f`'s
/// frame the way up `e`'s is (the sign of `(a·b·n̂)_e · (a·b·n̂)_f`) and
/// then by the rotation that best matches the two, which maximizes
/// `tr(Qᵀ·L_fᵀ·L_e)`. The square is the larger eigenvalue of a Gram
/// matrix, never negative, so its enclosure is clamped at zero: two
/// coincident conics' `d1` and `d2` enclose zero, and their self-dots
/// read as interval products reach below it.
fn frame_gap<T: Decide>(e: &dyn Conic<T>, f: &dyn Conic<T>) -> T {
    let ((e1, e2), (f1, f2)) = (e.semi_axis_vectors(), f.semi_axis_vectors());
    let up = e1.cross(e2).dot(f1.cross(f2));
    let f2 = f2 * (up / up.abs());
    let (x, y) = (f1.dot(e1) + f2.dot(e2), f2.dot(e1) - f1.dot(e2));
    let h = (x.powi(2) + y.powi(2)).sqrt();
    let (cos, sin) = (x / h, y / h);
    let d1 = e1 - (f1 * cos + f2 * sin);
    let d2 = e2 - (f2 * cos - f1 * sin);
    let half = T::from_f64(0.5);
    let (mid, off) = (
        (d1.dot(d1) + d2.dot(d2)) * half,
        (d1.dot(d1) - d2.dot(d2)) * half,
    );
    (mid + (off.powi(2) + d1.dot(d2).powi(2)).sqrt())
        .max(T::from_f64(0.0))
        .sqrt()
}

/// `ka`'s height above `kb`'s plane, `(c0, cp, sp)` with the swing read
/// definite (module docs): its crossings and touch, each polished to the
/// closest pair. `ka` is the conic the caller's across choice reads: the
/// one that crosses the other's plane the less obliquely where that
/// choice decides, and the first of the pair at a tie.
fn transverse<T: Decide>(
    (a, ka, kb, b): (&Span<'_, T>, &dyn Conic<T>, &dyn Conic<T>, &Span<'_, T>),
    (c0, cp, sp): (T, T, T),
    (swing, belly): (T, T),
    lever: Lever<T>,
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let range = Range::first_harmonic(
        c0,
        swing,
        belly,
        Azimuth::of(sp, cp).principal_as(row::BRANCH, Band::least()),
    );
    let window = window::Window {
        a: ka,
        span_a: a.params(),
        b: kb,
        span_b: b,
        height: (c0, cp, sp),
    };
    let joints: Vec<_> = known
        .iter()
        .map(|c| window::Joint {
            point: c.point,
            ta: c.ta,
        })
        .collect();
    let near = |theta: T| {
        let arc = band_arc((c0, swing, range.at_hi), theta, band);
        if window.holds_only((theta - arc, theta + arc), &joints, band) {
            return Ok(None);
        }
        let q = ka.point_at(theta);
        Polish {
            span_a: a,
            span_b: b,
            a: Side::Conic(ka),
            b: Side::Conic(kb),
            start: (theta, foot(b, q)),
            reach: ka.speed_hi() * arc,
            clear: clearance(b.carrier(), q),
            lever,
        }
        .closest(band)
    };
    let extremes = match height_extremes(&range, band)? {
        Extremes::Read(signs) => signs,
        Extremes::Gap(theta, gap) => {
            return match near(theta) {
                Ok(Some(c)) => Ok(Pose::Tangent(c)),
                Ok(None) => Ok(Pose::Apart),
                // Where the window and the polish leave the touch
                // unsettled, the band's own question is what is open: the
                // height's extreme in the gap, which a finer band decides.
                // The basin is only the polish's premise for reading it.
                Err(MeetFault::Unsettled(Unsettled::Meter { row: row::BASIN })) => {
                    Err(MeetFault::Escalated(gap))
                }
                Err(other) => Err(other),
            };
        }
    };
    Ok(match harmonic_roots(ka, &range, extremes, band)? {
        Harmonic::Apart => Pose::Apart,
        Harmonic::Touch(theta) => match near(theta)? {
            Some(c) => Pose::Tangent(c),
            None => Pose::Apart,
        },
        Harmonic::Cross(thetas) => {
            let mut out = Vec::with_capacity(2);
            for theta in thetas {
                out.extend(near(theta)?);
            }
            Pose::Generic(out)
        }
    })
}

/// Whether a quantity known to within `spread` about `value` is
/// definitely clear of zero (`true`) or in the zero band all round
/// (`false`), read under `row`.
///
/// # Errors
///
/// An end in the band's gap, or ends that straddle the band.
fn constant_as<T: Decide>(
    row: &'static str,
    value: T,
    spread: T,
    band: Band,
) -> Result<bool, MeetFault> {
    let (below, above) = crate::roots::spread_readings(row, value, spread, band)?;
    match (below.sign, above.sign) {
        (Sign::Positive, _) | (_, Sign::Negative) => Ok(true),
        (Sign::Zero, Sign::Zero) => Ok(false),
        _ => Err(MeetFault::Escalated(straddle(row, below, above, band))),
    }
}

/// Two circles in one plane (module docs): `slant` how far they lie from
/// one plane.
fn coplanar_circles<T: Decide>(
    ka: &dyn Conic<T>,
    kb: &dyn Conic<T>,
    slant: T,
    place: &impl Fn(T) -> Candidate<T>,
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (ra, rb) = (ka.speed_hi(), kb.speed_hi());
    let w = ka.frame().origin() - kb.frame().origin();
    let w = w - ka.frame().axis() * w.dot(ka.frame().axis());
    let d = w.norm();
    if !definite_as(row::CENTRES, Margin::norm3(w), band)? {
        // The one-curve gap is at most `d + |r₁ − r₂|` plus the slant, so
        // radii whose gap reads zero all round with the centres' are one
        // curve, which the one-curve reading has just denied.
        return if constant_as(row::RADII, ra - rb, d + slant, band)? {
            Ok(Pose::Concentric)
        } else {
            Err(MeetFault::Contradiction {
                readings: "two circles read not one curve, on one centre, with radii one",
            })
        };
    }
    // `|C₁(θ) − c₂|` runs between `|r₁ − d|` and `r₁ + d`; its square
    // less `r₂²` is `c₀ + 2r₁d·cos(θ − θ_hi)`, a first harmonic whose
    // swing is `4r₁d`. `θ_hi` is the angle of `w`.
    let (near, far) = ((ra - d).abs(), ra + d);
    let (lo, hi) = (near - rb, far - rb);
    let swing = T::from_f64(4.0) * ra * d;
    let toward = Azimuth::of(w.dot(ka.frame().v_ref()), w.dot(ka.frame().u_ref()))
        .principal_as(row::BRANCH, Band::least());
    let range = Range {
        lo,
        hi,
        at_hi: toward,
        cross_lo: (T::zero() - lo) * (near + rb) / swing,
        cross_hi: hi * (far + rb) / swing,
    };
    let found: Vec<T> = known.iter().map(|c| c.ta).collect();
    Ok(match owed(ka, &range, &found, band)? {
        Harmonic::Apart => Pose::Apart,
        Harmonic::Touch(theta) => {
            spread_held(angle_spread(ra, theta), band)?;
            Pose::Tangent(place(theta))
        }
        Harmonic::Cross(thetas) => Pose::Generic(thetas.into_iter().map(place).collect()),
    })
}

/// A coplanar pair's roots and touches from the certified subdivision
/// (module docs, "A touch in the subdivision"), each placed as a
/// candidate: none is [`Pose::Apart`], one touch alone is
/// [`Pose::Tangent`], and a refusal is typed.
fn subdivided<T: Decide>(
    terms: Terms<'_, T>,
    residual: &impl Fn(T) -> T,
    frame: &SubdivisionFrame<T>,
    place: &impl Fn(T) -> Candidate<T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let found = certified_subdivision_touching(
        terms,
        residual,
        frame,
        &COPLANAR_ROWS,
        &COPLANAR_TOUCH,
        band,
    )?;
    let roots: &[T] = match &found.roots {
        Roots::Miss => &[],
        Roots::Certified(r) => r.roots(),
        Roots::On(never) => match *never {},
    };
    for touch in &found.touches {
        spread_held(touch.spread + touch.spread, band)?;
    }
    Ok(match (roots, &found.touches[..]) {
        ([], []) => Pose::Apart,
        ([], &[touch]) => Pose::Tangent(place(touch.at)),
        _ => Pose::Generic(
            roots
                .iter()
                .copied()
                .chain(found.touches.iter().map(|t| t.at))
                .map(place)
                .collect(),
        ),
    })
}

/// **Two ellipses in one plane** (module docs): the first's level on
/// the second, `F(θ) = X²/A² + Y²/B² − 1` at the first's point `(X, Y)`
/// in the second's frame, is of degree two in the first's angle, and goes
/// to the certified subdivision, read locally ([`Level`]), in the metres of
/// [`geom::ellipse_level_clearance`], the lower bound on a point's
/// distance from the second that [`Curve3::clearance`] reads, signed as
/// `F`.
///
/// **The ceiling on `|F|` per metre of that bound.** `X²/A⁴ + Y²/B⁴` is
/// at most `(F + 1)/s²`, `s` the second's smaller semi-axis, so `g = |∇F|`
/// is at most `2√(1 + |F|)/s`, and the bound
/// `2|F|/(g + √(g² + 4|F|/s²))` is at least
/// `|F|·s/(√(1 + |F|) + √(1 + 2|F|))`. Where `|F| ≤ 1` that is at least
/// `|F|·s/(√2 + √3) > |F|·s/4`; past it, increasing in `|F|`, at least
/// `s/(√2 + √3) > s/4`. So the bound is at least `min(s/4, |F|/(4/s))`
/// everywhere: the ceiling `4/s`, reaching to `s/4`.
fn coplanar_ellipses<T: Decide>(
    e: &Span<'_, T>,
    ka: &dyn Conic<T>,
    kb: &dyn Conic<T>,
    place: &impl Fn(T) -> Candidate<T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (big_a, big_b) = kb.semi_axes();
    let (ub, vb) = (kb.frame().u_ref(), kb.frame().v_ref());
    let level = level_on(ka, kb);
    let small = big_a.abs().min(big_b.abs());
    let large = big_a.abs().max(big_b.abs());
    let (t0, t1) = e.params();
    let (ceiling, reach) = level_ceiling(small);
    let frame = SubdivisionFrame {
        t0,
        t1,
        speed_hi: ka.speed_hi(),
        // No harmonics are read.
        noise: T::zero(),
        // A point's distance from the second is at most `L·|F|`,
        // `L` its larger semi-axis (`pose::window`'s module docs, (ii)),
        // and the bound below it reads no more: the floor `1/L`.
        f_per_metre: T::one() / large,
        f_per_metre_hi: ceiling,
        residual_reach: Some(reach),
    };
    // Read exact, as the window reads it: the level divides by the squared
    // semi-axes, which the admission rule does not cover, and is owed a
    // re-posing as a length
    // (`work/conic/the-windows-level-jet-is-read-exact-where-its-rounding-grows-with-the-aspect.md`).
    let jet = |theta: T| level.jet_at(theta).map(geom_core::Rounded::exact);
    let local = Local {
        at: &jet,
        fourth: level.fourth(),
        degree: 2,
    };
    let residual = |theta: T| {
        let d = ka.point_at(theta) - kb.frame().origin();
        geom::ellipse_level_clearance((d.dot(ub), d.dot(vb)), (big_a, big_b))
    };
    subdivided(Terms::Local(local), &residual, &frame, place, band)
}

/// **`b`'s level along `a`**: `F(θ) = X²/A² + Y²/B² − 1` at the image of
/// `a(θ)` in `b`'s plane, `(X, Y)` in `b`'s frame and `(A, B)` its
/// semi-axes. The height of `a(θ)` above `b`'s plane plays no part.
///
/// **It is read locally**: at `θ`, from `a`'s point and derivatives there
/// in `b`'s frame ([`Level::jet_at`]), never from its global harmonics.
/// Posed about `b`'s centre those carry rounding of order `((|w| + R)/s)²`,
/// `w` the centres' offset, `R` `a`'s size and `s` `b`'s smaller
/// semi-axis, which at a far or thin pair reaches the band's zero. The
/// local form drops the far centre's share, but not the thin pair's: its
/// terms still divide by `A²` and `B²`, so its rounding as a length grows
/// with `b`'s aspect, which the admission rule does not cover
/// (`work/conic/the-windows-level-jet-is-read-exact-where-its-rounding-grows-with-the-aspect.md`).
/// A reading over a piece takes the jet at its middle and
/// [`Level::fourth`] for Taylor's remainder, a bound that needs no
/// precision.
pub(super) struct Level<'k, T: geom_core::Real> {
    a: &'k dyn Conic<T>,
    centre: Point3<T>,
    axes: (Vec3<T>, Vec3<T>),
    squares: (T, T),
    fourth: T,
}

impl<T: Decide> Level<'_, T> {
    /// `F` and its first three derivatives at `θ`. On a conic
    /// `P″ = −(P − c)` and `P‴ = −P′`, `c` its centre.
    pub(super) fn jet_at(&self, theta: T) -> [T; 4] {
        let (p, dp) = self.a.jet_at(theta);
        let ddp = self.a.frame().origin() - p;
        let (ub, vb) = self.axes;
        let q = p - self.centre;
        let side = |axis: Vec3<T>, square: T| {
            let (x, x1, x2) = (q.dot(axis), dp.dot(axis), ddp.dot(axis));
            let x3 = T::zero() - x1;
            let three = T::from_f64(3.0);
            [
                x.powi(2) / square,
                x * x1 / square,
                (x1.powi(2) + x * x2) / square,
                (three * x1 * x2 + x * x3) / square,
            ]
        };
        let (x, y) = (side(ub, self.squares.0), side(vb, self.squares.1));
        let two = T::from_f64(2.0);
        [
            x[0] + y[0] - T::one(),
            two * (x[1] + y[1]),
            two * (x[2] + y[2]),
            two * (x[3] + y[3]),
        ]
    }

    /// `|∇F|` at the image of `a(θ)`: `2·√(X²/A⁴ + Y²/B⁴)`.
    pub(super) fn gradient_at(&self, theta: T) -> T {
        let q = self.a.point_at(theta) - self.centre;
        let (ub, vb) = self.axes;
        let (x, y) = (q.dot(ub) / self.squares.0, q.dot(vb) / self.squares.1);
        T::from_f64(2.0) * (x.powi(2) + y.powi(2)).sqrt()
    }

    /// A bound on `|F⁗|` over every angle. Each of `X` and `Y` is
    /// `x₀ + ρ·cos(θ − φ)`, so `X²`'s first and second harmonics have
    /// amplitudes `2|x₀|ρ` and `ρ²/2`, and `|(X²)⁗| ≤ 2|x₀|ρ + 8ρ²`. A sum
    /// of non-negative terms, raised past its roundings ([`up`]): a dot
    /// product's five, `ρ`'s four more, and each side's six, summed.
    pub(super) fn fourth(&self) -> T {
        self.fourth
    }
}

/// `x`, non-negative and computed in `N` roundings to nearest, raised to
/// an upper bound on the real value it stands for. Each rounding is
/// within `u = ε/2` relative, so `x̂ ≥ x·(1 − u)ᴺ`, and the factor
/// `c = 1 + (N + 1)·2u`, a constant exact in `f64` for any `N` short of
/// `2⁵¹`, lifts it past `x` in one more rounding:
/// `x̂·c·(1 − u) ≥ x·(1 − (N + 1)·u)·(1 + 2(N + 1)·u) ≥ x` while
/// `(N + 1)·u ≤ ½`. At an enclosure `c` is a point and the product is
/// outward already, so this only widens it.
pub(super) fn up<const N: u32, T: Decide>(x: T) -> T {
    x * T::from_f64(1.0 + f64::EPSILON * (N + 1) as f64)
}

/// [`Level`] of `kb` along `ka`.
pub(super) fn level_on<'k, T: Decide>(ka: &'k dyn Conic<T>, kb: &dyn Conic<T>) -> Level<'k, T> {
    let (big_a, big_b) = kb.semi_axes();
    let (ub, vb) = (kb.frame().u_ref(), kb.frame().v_ref());
    let (ua, va) = (
        ka.frame().u_ref() * ka.semi_axes().0,
        ka.frame().v_ref() * ka.semi_axes().1,
    );
    let w = ka.frame().origin() - kb.frame().origin();
    let squares = (big_a.powi(2), big_b.powi(2));
    let (two, eight) = (T::from_f64(2.0), T::from_f64(8.0));
    let side = |axis: Vec3<T>, square: T| {
        let rho = (ua.dot(axis).powi(2) + va.dot(axis).powi(2)).sqrt();
        (two * w.dot(axis).abs() * rho + eight * rho.powi(2)) / square
    };
    Level {
        a: ka,
        centre: kb.frame().origin(),
        axes: (ub, vb),
        squares,
        fourth: up::<24, _>(side(ub, squares.0) + side(vb, squares.1)),
    }
}

/// **The ceiling on an ellipse's level per metre of
/// [`geom::ellipse_level_clearance`], and how far it reaches**
/// (`coplanar_ellipses`'s docs): `(4/s, s/4)`, `s` the smaller semi-axis.
pub(super) fn level_ceiling<T: Decide>(small: T) -> (T, T) {
    let four = T::from_f64(4.0);
    (four / small, small / four)
}

/// **Whether a conic's height over the other's plane is clear of it all
/// round** (`curve_pair_pose_parallel`): its extremes ([`extremes`])
/// definitely one-signed, the one nearer zero read as the belly.
fn clear_all_round<T: Decide>(c0: T, swing: T, belly: T, band: Band) -> bool {
    let (lo, hi) = extremes(c0, swing, belly);
    let read = |x: T| geom_core::k_stats::decide_reported(row::PARALLEL, Margin::of(x), band);
    let clear = read(lo).and_then(|below| Ok((below, read(hi)?)));
    matches!(clear, Ok((below, above)) if below.sign == Sign::Positive || above.sign == Sign::Negative)
}

/// **The extremes `(lo, hi)` of `c₀ + A·cos ψ`**, `A ≥ 0` the swing and
/// `belly` its [`Conic::belly_along`] `A − |c₀|`: the one nearer zero is the belly
/// (negated below), the other the sum, which cancels nothing.
fn extremes<T: Decide>(c0: T, swing: T, belly: T) -> (T, T) {
    (
        c0.select_le_zero(c0 - swing, T::zero() - belly),
        c0.select_le_zero(belly, c0 + swing),
    )
}

/// A first harmonic's range along a conic, in metres, and where its roots
/// are read from (module docs): `lo` and `hi` are its extremes, `at_hi`
/// the angle of `hi` (`lo` is half a turn on), `cross_lo` and `cross_hi`
/// how far each extreme has crossed zero over the swing, in the units the
/// roots solve in (positive where it has crossed).
struct Range<T> {
    lo: T,
    hi: T,
    at_hi: T,
    cross_lo: T,
    cross_hi: T,
}

impl<T: Decide> Range<T> {
    /// `c₀ + A·cos(θ − at_hi)`, `A ≥ 0`, solved in itself, its extreme
    /// nearer zero read as `belly` ([`Conic::belly_along`]: `A − |c₀|`), the other as
    /// the sum, which cancels nothing.
    fn first_harmonic(c0: T, swing: T, belly: T, at_hi: T) -> Self {
        let (lo, hi) = extremes(c0, swing, belly);
        let two = swing + swing;
        Self {
            lo,
            hi,
            at_hi,
            cross_lo: (T::zero() - lo) / two,
            cross_hi: hi / two,
        }
    }

    /// **Its extremes `lo` and `hi`, each read** (`curve_pair_pose_extreme`). What an extreme in the band's gap
    /// decides is the caller's: a distance's escalates ([`owed`]), a
    /// height's places a touch ([`height_extremes`]).
    fn extremes(
        &self,
        band: Band,
    ) -> (
        Result<Sign, geom_core::Indeterminate>,
        Result<Sign, geom_core::Indeterminate>,
    ) {
        let read = |x: T| decide(row::EXTREME, Margin::of(x), band);
        (read(self.lo), read(self.hi))
    }
}

/// `ℓ` in radians (module docs, "Across planes"): how far from `theta`
/// the arc about it runs on which the first harmonic
/// `c₀ + swing·cos(θ − at_hi)`, a metre residual, stays within the band's
/// escalation. Read from the harmonic's own `c₀` and swing, as the caller
/// formed them.
fn band_arc<T: Decide>((c0, swing, at_hi): (T, T, T), theta: T, band: Band) -> T {
    let e = T::from_f64(band.escalate());
    let unit_cos = |x: T| x.max(T::zero() - T::one()).min(T::one());
    let phi = geom::separation(theta, at_hi);
    let inner = unit_cos((e - c0) / swing).acos();
    let outer = unit_cos((T::zero() - e - c0) / swing).acos();
    (phi - inner).abs().max((outer - phi).abs())
}

/// **Whether a coplanar touch's reported point is one point to its
/// readers** (module docs, "A touch's spread"): `apart`, the most two
/// readings of the touch can lie apart, read in the zero band
/// (`curve_pair_pose_touch_spread`). In the band's gap it escalates,
/// since a narrower enclosure shrinks the spread; decided past the band
/// it refuses on the meter, as a root's slack past the band does
/// ([`Unsettled::Meter`]).
///
/// # Errors
///
/// [`MeetFault::Escalated`] in the gap, [`Unsettled::Meter`] past it.
fn spread_held<T: Decide>(apart: T, band: Band) -> Result<(), MeetFault> {
    match decide(row::TOUCH_SPREAD, Margin::of(apart), band) {
        Ok(Sign::Zero) => Ok(()),
        Err(gap) => Err(MeetFault::Escalated(gap)),
        Ok(Sign::Positive | Sign::Negative) => Err(MeetFault::Unsettled(Unsettled::Meter {
            row: row::TOUCH_SPREAD,
        })),
    }
}

/// **A closed-form touch's spread** (module docs, "A touch's spread"):
/// `radius·width(θ)` ([`geom_core::width`]), how far in metres along the
/// carrier the touch at `θ` can lie from another reading of it on the
/// same carrier, `radius` the carrier's top speed.
///
/// **Why `θ` is well conditioned** (the premise the zero spread at a
/// point scalar rests on). `θ` is the angle of the first harmonic's
/// coefficients, and an angle moves by at most its components' error
/// over their length. The coefficients' error is relative to that length:
/// - two circles' coefficients are `w`'s components in the first frame,
///   `w` the centres' difference. Where the centres are near, each
///   component of `w` subtracts exactly (Sterbenz: `a − b` is exact for
///   `b/2 ≤ a ≤ 2b`); otherwise it rounds at `u` of its own size. The
///   projection square to the axis and the frame's dots then round at
///   `u` of `|w|`. So `w`'s error is a few `u` of `|w|`, however near the
///   centres;
/// - a conic beside a line reads `(a·(u_ref·m), b·(v_ref·m))`, `m` a
///   computed unit direction, each a product and a short dot, at a few
///   `u` of the semi-axes.
///
/// A refactor that computes `w` another way (from points far from it,
/// say, not from the two centres) breaks this premise.
///
/// So at a point scalar `θ` is treated as exact and the spread is zero
/// (`[simple.conditioned]`: no site charges rounding). At `Interval` it
/// is the angle's own enclosure, its width either side of zero, so the
/// spread is certified there and charges nothing.
fn angle_spread<T: Decide>(radius: T, theta: T) -> T {
    radius * geom_core::width(theta)
}

/// What a first harmonic's extremes and root gap decide.
enum Harmonic<T> {
    /// No root: the extremes definitely one-signed.
    Apart,
    /// A touch at this angle: a double root, or an extreme in the zero
    /// band that has not crossed.
    Touch(T),
    /// Crossings: two, or those left once the found meetings' roots are
    /// divided out (module docs, "deflation").
    Cross(Vec<T>),
}

/// **Whether a found meeting's reflection is its harmonic's other root**
/// (module docs, "The spans' ends, and deflation"). The reflection
/// `2θ_hi − θ_J` is that root where `θ_J` is one. It is not where it
/// rings onto the found meeting's point in metres
/// (`curve_pair_meet_same`) while its angle from `θ_J`, the least way
/// round, is past the found meeting's ring in angle
/// (`curve_pair_pose_reflection_far`): the point lies within the zero of
/// the found one, but on the conic's other branch. An angle not decided
/// within the ring is read past it, so both roots are read.
///
/// The ring in angle is `e/|C′|max`, `e` the band's escalation: an
/// angle within it of `θ_J` runs at most `e` of arc, so its point is
/// within `e` of the found one, a chord no longer than that arc.
fn reflected<T: Decide>(
    k: &dyn Conic<T>,
    r: &Range<T>,
    j: &Candidate<T>,
    e: T,
    band: Band,
) -> bool {
    let mirror = r.at_hi + r.at_hi - j.tb;
    if !super::ringed_by([j.point], k.point_at(mirror), band) {
        return true;
    }
    let tau = T::pi() + T::pi();
    let turn = mirror - j.tb;
    let least = (turn - tau * (turn / tau + T::from_f64(0.5)).floor()).abs();
    let ring = e / k.speed_hi();
    matches!(
        geom_core::k_stats::param_order(row::REFLECTION_FAR, least, ring),
        Ok(Sign::Negative | Sign::Zero)
    )
}

/// **A first harmonic's roots along `k` still owed**, `found` the angles
/// of the meetings already found (module docs, "deflation"): none
/// found, every root ([`harmonic_roots`]); one, the other, `2θ_hi − θ_J`;
/// two, none.
fn owed<T: Decide>(
    k: &dyn Conic<T>,
    r: &Range<T>,
    found: &[T],
    band: Band,
) -> Result<Harmonic<T>, MeetFault> {
    Ok(match found {
        [] => {
            let (lo, hi) = r.extremes(band);
            harmonic_roots(k, r, (lo?, hi?), band)?
        }
        [theta] => Harmonic::Cross(vec![r.at_hi + r.at_hi - *theta]),
        _ => Harmonic::Cross(Vec::new()),
    })
}

/// How a height's extremes read (module docs, "A first harmonic's
/// roots").
enum Extremes<T> {
    /// Each extreme's sign.
    Read((Sign, Sign)),
    /// One extreme in the band's gap, the other past it on its far side:
    /// the angle of the one in the gap, a touch for its window, and its
    /// escalation.
    Gap(T, geom_core::Indeterminate),
}

/// **A conic's height above the other's plane, its extremes read**
/// (module docs, "A first harmonic's roots"): as a distance's are, but
/// for an extreme in the band's gap with the other definite past it,
/// which places a touch for its window ([`Extremes::Gap`]).
fn height_extremes<T: Decide>(r: &Range<T>, band: Band) -> Result<Extremes<T>, MeetFault> {
    Ok(match r.extremes(band) {
        (Err(gap), Ok(Sign::Positive)) => Extremes::Gap(r.at_hi + T::pi(), gap),
        (Ok(Sign::Negative), Err(gap)) => Extremes::Gap(r.at_hi, gap),
        (lo, hi) => Extremes::Read((lo?, hi?)),
    })
}

/// A first harmonic's roots along `k`, its extremes `lo` and `hi` read
/// (module docs).
fn harmonic_roots<T: Decide>(
    k: &dyn Conic<T>,
    r: &Range<T>,
    (lo, hi): (Sign, Sign),
    band: Band,
) -> Result<Harmonic<T>, MeetFault> {
    // The angle from `hi` of the roots read from each extreme: none
    // crossed reads a double root at that extreme.
    let past = |cross: T| arc_past(cross.max(T::zero()).min(T::one()));
    let from_hi = past(r.cross_hi);
    let from_lo = T::pi() - past(r.cross_lo);
    let at_lo = r.at_hi + T::pi();
    let psi = match (lo, hi) {
        (Sign::Positive, _) | (_, Sign::Negative) => return Ok(Harmonic::Apart),
        (Sign::Zero, Sign::Zero) => return Err(MeetFault::Unsettled(Unsettled::Tangency)),
        // An extreme in the zero band is a touch at the extreme (module
        // docs, "A first harmonic's roots").
        (Sign::Zero, Sign::Positive) => return Ok(Harmonic::Touch(at_lo)),
        (Sign::Negative, Sign::Zero) => return Ok(Harmonic::Touch(r.at_hi)),
        // Two definite crossings: read from the extreme that has crossed
        // the less, to its own relative precision.
        (Sign::Negative, Sign::Positive) => {
            (r.cross_hi - r.cross_lo).select_le_zero(from_hi, from_lo)
        }
    };
    let roots = [r.at_hi + psi, r.at_hi - psi];
    let gap = Margin::norm3(k.point_at(r.at_hi + psi) - k.point_at(r.at_hi - psi));
    // Two roots are read as two candidates, whatever their gap: where the
    // band cannot split their points they ring as one meeting. Placed at
    // the extreme between them, a touch's point would lie off the other
    // carrier where a conic curves sharply enough to leave the band
    // between two roots that close (a thin ellipse's vertex). An
    // undecided gap is roots placed past what the band reads, and
    // escalates.
    decide(row::ROOT_GAP, gap, band)?;
    Ok(Harmonic::Cross(roots.to_vec()))
}

#[cfg(test)]
mod far_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod polish_rows {
    use super::*;

    /// **A polish its basin does not hold refuses, and one it holds
    /// states its window.** Two unit circles crossing square at the
    /// origin, the polish started there and with the first `0.3` rad
    /// round. Started at the crossing, the candidate is the crossing,
    /// with a window whose floor is the gap less its bend. Started `0.3`
    /// rad off, the window's span (its region and the polish's travel)
    /// turns the tangents past the basin's premise, and it refuses on the
    /// basin (`Unsettled::Meter`): under the premise Gauss–Newton
    /// contracts to rounding within its steps, so no premise-holding pose
    /// is left unconverged; the window's escalation is pinned by
    /// `curve_pair::meet_rows::a_window_read_off_short_of_its_reach_escalates`.
    #[test]
    fn an_unconverged_polish_refuses() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let a = geom::Circle::new(geom::Frame::new(Point3::new(0.0, -1.0, 0.0), z, x), 1.0);
        let b = geom::Circle::new(geom::Frame::new(Point3::new(-1.0, 0.0, 0.0), z, x), 1.0);
        let (ca, cb) = (Curve3::Circle(a), Curve3::Circle(b));
        let tau = core::f64::consts::TAU;
        let (sa, sb) = (
            Span::new(&ca, (0.0, tau)).endless(),
            Span::new(&cb, (0.0, tau)).endless(),
        );
        let polish = |off: f64| Polish {
            span_a: &sa,
            span_b: &sb,
            a: Side::Conic(&a),
            b: Side::Conic(&b),
            start: (core::f64::consts::FRAC_PI_2 + off, 0.0),
            reach: 0.11,
            clear: 0.0,
            lever: geom_core::test_support::fixture_lever(1.0),
        };
        let got = polish(0.0).closest(band).unwrap().unwrap();
        assert!(
            got.point.to_array().iter().all(|c| c.abs() < 1e-15),
            "{got:?}"
        );
        let Some(window) = got.bound else {
            panic!("a polished candidate states its window: {got:?}")
        };
        assert!(
            window.floor.abs() < 1e-15 && window.reach > 0.0 && window.reach < 2e-8,
            "{window:?}"
        );
        let got = polish(0.3).closest(band);
        assert!(
            matches!(
                got,
                Err(MeetFault::Unsettled(Unsettled::Meter { row: row::BASIN }))
            ),
            "{got:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod one_line_rows {
    use super::*;

    /// **An end in band does not hide a later end decided off the line**
    /// ([`one_line`]'s [`read_identity`], `Any`). Line `a` runs from the
    /// origin along `x`; line `b` starts `3e-9` above the origin and
    /// rises at 45°. `a`'s first end is read first, a mid-band distance
    /// from `b`; its second end is about `0.7` off `b`, decided apart,
    /// so the lines are not one. A first-refusal loop escalated at the
    /// first end.
    #[test]
    fn an_end_decided_off_answers_past_an_end_in_band() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let a = (p(0.0, 0.0), p(1.0, 0.0));
        let la = geom::Line {
            origin: a.0,
            dir: geom::stated_direction(Vec3::new(1.0, 0.0, 0.0)),
        };
        let b = (p(0.0, 3e-9), p(1.0, 1.0 + 3e-9));
        let lb = geom::Line {
            origin: b.0,
            dir: geom_core::UnitVec3::squared(b.1 - b.0),
        };
        let first = lb.distance(a.0);
        assert!(
            first > 1e-9 && first < 1e-8,
            "the first end is in band: {first}"
        );
        assert_eq!(one_line((a, la), (b, lb), band), Ok(false));
    }
}
