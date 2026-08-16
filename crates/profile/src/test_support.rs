//! **Test support, not API** — the banished raw loop builder, on a
//! DELETION HORIZON.
//!
//! [`LoopBuilder`] is the vertex-and-bulge chain builder the kernel
//! shipped before the PATHS lattice existed. PROFILES-V2 §V6's ratified
//! amendment (issue #377) retired it as an authoring surface: it left
//! the crate's published API entirely and survives here, behind the
//! `test-support` feature that only this crate's own dev-dependency
//! enables.
//!
//! # What it is still here for, exactly (LIB-RETTAIL, 2026-08-12)
//!
//! NOT the differential twins any more. `tests/path_differential.rs`
//! and `tests/path_property.rs` used to pin the lattice's lowering
//! against this builder bit for bit, and that was the stated reason
//! this file was not simply deleted. It no longer is: the twins verify
//! against BLESSED RECORDED FIXTURES, which pin bit-identity exactly as
//! hard. (The independence argument had also weakened on inspection —
//! `fillet_corner` here and `path::arc_fillet` in the lattice both call
//! the one ratified `sugar::arc_fillet_trims`, so on the fillet family
//! this was never a second implementation of the geometry, only a
//! second door onto it with a different error vocabulary.)
//!
//! What remains is ONE role, and it is a sequencing problem rather than
//! a design one: the arc-leg fillet suites (`tests/arc_fillet.rs`,
//! `tests/review_s2.rs`, `tests/declared_tangency.rs`,
//! `tests/interval_lane.rs`, `tests/scalar_channels.rs`,
//! `tests/review_s8_probe.rs`, `tests/common/mod.rs::bracket`, and the
//! cross-crate fixtures in `sweep`/`step-export`) author their corners
//! through `fillet`/`fillet_corner` here. Their only lattice target is
//! the §2c arc-carrier fillet family — which is
//! precisely the surface PATHS-DESIGN §2c redesigns (RATIFIED on #419,
//! 2026-08-11; carrier-typed tips, uniform arrival binders, the compound
//! register dissolved). The re-spell UNIT has not run yet, so migrating
//! these callers now means migrating them again at it: the deletion
//! rides the §2c re-spell. The plain (non-fillet) chains in those files
//! carry no such constraint and are raw-data spellings away.
//!
//! What is NOT here, deliberately: raw [`crate::ProfileLoop`]
//! DATA (`new`/`polygon`, now on the [`RawLoop`](crate::RawLoop) trait)
//! and the bulge constructors ([`crate::bulge_from_via`]
//! / [`crate::bulge_from_center`]) stay kernel
//! vocabulary — they never were `LoopBuilder`. The validate-refusal
//! demo they used to justify (the bowtie) is gone from the tour and
//! lives in `tests/rejections.rs`, authored through the lattice.

use geom_core::{Bounds, Decide, Point2, Real, Sign, Tolerance};

use crate::sugar::{
    ArcFilletOutcome, ArcSweep, ArcTrimRefusal, FilletLegShape, TrimRefusal, arc_fillet_trims,
    bulge_from_center, bulge_from_via, line_line_fillet_trims,
};
use crate::validate::{EscalationSite, FilletLeg, FilletLegCarrier, ProfileError};
use crate::{ProfileLoop, ProfileVertex};

impl<T: Real> ProfileLoop<T> {
    /// Starts a [`LoopBuilder`] at `start` — the raw chain door, kept
    /// here (and only here) so the suites that predate the lattice
    /// still spell their fixtures the way they were written.
    pub fn builder(start: Point2<T>) -> LoopBuilder<T> {
        LoopBuilder::start(start)
    }
}

/// A chain builder: `start` → (`line_to` | `arc_to` | `arc_to_via` |
/// `arc_to_center`)* → one `close*` call, yielding a [`ProfileLoop`].
///
/// Each step appends a vertex and sets the bulge of the segment
/// *arriving* at it on the previous vertex; the `close*` variants set
/// the implicit closing segment's bulge on the last vertex (closure
/// itself is by construction — there is no way to build an open chain).
#[derive(Clone, Debug)]
pub struct LoopBuilder<T: Real> {
    vertices: Vec<ProfileVertex<T>>,
    tangent: Vec<usize>,
}

impl<T: Real> LoopBuilder<T> {
    /// Starts a chain at `start`.
    pub fn start(start: Point2<T>) -> Self {
        Self {
            vertices: vec![ProfileVertex {
                pos: start,
                bulge: T::zero(),
            }],
            tangent: Vec::new(),
        }
    }

    /// The current chain end (the last vertex's position).
    fn head(&self) -> Point2<T> {
        // The vector is nonempty by construction (`start` seeds it).
        self.vertices[self.vertices.len() - 1].pos
    }

    /// Sets the bulge of the segment leaving the current last vertex.
    fn set_leaving_bulge(&mut self, bulge: T) {
        let last = self.vertices.len() - 1;
        self.vertices[last].bulge = bulge;
    }

    /// Appends a straight segment to `p`.
    pub fn line_to(mut self, p: Point2<T>) -> Self {
        self.set_leaving_bulge(T::zero());
        self.vertices.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self
    }

    /// Appends an arc segment to `p` with an explicit `bulge` (the raw
    /// form — see the crate docs for the sign convention).
    pub fn arc_to(mut self, p: Point2<T>, bulge: T) -> Self {
        self.set_leaving_bulge(bulge);
        self.vertices.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self
    }

    /// Appends the arc through `via` ending at `p` (three-point form;
    /// see [`bulge_from_via`]).
    pub fn arc_to_via(self, via: Point2<T>, p: Point2<T>) -> Self {
        let bulge = bulge_from_via(self.head(), via, p);
        self.arc_to(p, bulge)
    }

    /// Appends the arc about `center` ending at `p`, sweeping `sweep`
    /// (center form; see [`bulge_from_center`]).
    pub fn arc_to_center(self, p: Point2<T>, center: Point2<T>, sweep: ArcSweep) -> Self {
        let bulge = bulge_from_center(self.head(), p, center, sweep);
        self.arc_to(p, bulge)
    }

    /// Declares the **joint at the current chain end** tangent: the
    /// junction between the segment arriving at the last vertex and
    /// the segment that will leave it (the next `*_to`/`close*` call —
    /// or, called on a fresh builder, the junction between the closing
    /// segment and the first). The explicit hand-authoring form of the
    /// #101 discipline ([`crate::ProfileLoop::tangent_joints`]);
    /// validation *verifies* the claim and refuses
    /// `TangencyContradicted` if the carriers are definitely not
    /// tangent. Prefer [`LoopBuilder::fillet`], which computes tangent
    /// geometry and declares it by construction.
    pub fn declare_tangent(mut self) -> Self {
        self.tangent.push(self.vertices.len() - 1);
        self
    }

    /// Appends a **tangent fillet** rounding the STRAIGHT corner the
    /// chain is heading for — the primary authoring path of the #101
    /// discipline (constructive, solver-free): from the chain end H, a
    /// straight leg toward `corner` C stopping at the tangent point T₁, then
    /// the radius-`radius` arc to the tangent point T₂ on the outgoing
    /// leg C→`next`; **the joints at strictly-interior tangent points
    /// are declared tangent by construction** (exact-fit sides: see
    /// the gate below). The caller must continue toward `next`
    /// (`line_to(next)`, or another `fillet` whose `corner` is `next`)
    /// — leaving T₂ any other way is a contradicted declaration, which
    /// validation refuses loudly.
    ///
    /// Closed form, exact where inputs are exact (D9: fixed order; no
    /// transcendentals — one square root per length, the "usual sqrt
    /// forms"): with legs v₁ = C − H, v₂ = `next` − C and
    /// m = √(‖v₁‖²·‖v₂‖²),
    /// tan(φ/2) = v₁×v₂ / (m + v₁·v₂) (φ = the corner's turn angle),
    /// setback = r·|tan(φ/2)|, T₁/T₂ = C ∓ setback·v̂₁/₂, and the arc's
    /// bulge = tan(φ/4) by the quarter-angle identity
    /// tan(φ/4) = tan(φ/2) / (1 + √(1 + tan²(φ/2))). For a
    /// right-angle corner with dyadic legs everything but the bulge is
    /// *bit-exact* (tan(φ/2) = ±1), and the residual carrier-clearance
    /// margin is rounding-level (~1e-16) — definite Zero at every
    /// supported ε.
    ///
    /// # The leg-fit gate (the one decision sugar takes)
    ///
    /// A tangent point outside its leg means the arc never approaches
    /// the corner the caller asked to round — a silent intent mismatch
    /// (the exact disease this discipline refuses), and one the
    /// resulting loop can VALIDATE through (the overshot arc is still
    /// carrier-tangent at both points; #101 review, MAJOR-1). The
    /// constructor therefore refuses it: each leg's fit margin
    /// (leg length − setback, meters) is classified through the
    /// reified **`fillet_leg_fit`** predicate against the exact-order
    /// band (`crate::validate` module docs — the representation-exact
    /// device, not a new ε; the knife-edge neighborhoods on BOTH sides
    /// are covered by validation's `vertex_separation` door, so exact
    /// classification is honest here):
    ///
    /// - **Negative** (either leg) ⇒ [`ProfileError::FilletDoesNotFit`]
    ///   naming the first overrun leg with setback/length diagnostics;
    /// - **Zero** (exact fit) ⇒ that side's straight piece has zero
    ///   length and is **not emitted**, and — having no collinear
    ///   straight piece adjacent — that tangent point is **not
    ///   declared**: an exact-fit incoming leg springs the arc directly
    ///   off the chain head (the head joint's tangency, if the caller
    ///   arranged one, is the caller's to declare); an exact-fit
    ///   outgoing leg ends the arc at `next` itself (continue from the
    ///   corner AT `next` — a `line_to(next)` would be degenerate);
    /// - **Positive** ⇒ the normal emission (straight piece + declared
    ///   tangent joint);
    /// - in-band / poisoned (interval knife-edge, NaN legs) ⇒
    ///   [`ProfileError::Escalated`] at
    ///   [`crate::EscalationSite::Fillet`] — this is also where a
    ///   doubled-back corner (φ = ±π) or zero-length leg lands: the
    ///   0/0 → NaN closed form poisons the fit margin, refused typed
    ///   at the constructor door.
    ///
    /// Remaining degenerates, honestly (executed, pinned in the
    /// `declared_tangency` suite): a straight-through corner (φ = 0)
    /// or `radius` = 0 yields setback 0, T₁ = T₂ = C, and a degenerate
    /// zero-length arc that validation refuses as `DegenerateSegment`;
    /// a negative radius puts the tangent points beyond the corner and
    /// the loop fails simplicity (`NonSimple`/`Crossing`). Never a
    /// panic, never a guess, never a silently-wrong shape.
    ///
    /// # Errors
    ///
    /// [`ProfileError::FilletDoesNotFit`], [`ProfileError::Escalated`]
    /// (site [`crate::EscalationSite::Fillet`]), or
    /// [`ProfileError::Band`] (unreachable for the built-in band) — see
    /// the gate above.
    ///
    /// # Arc legs
    ///
    /// This form is the line×line corner. Corners with a circular leg
    /// (line×arc, arc×line, arc×arc) go through
    /// [`LoopBuilder::fillet_corner`], which is the same construction
    /// stated on offset carriers; `fillet_corner` with two straight legs
    /// delegates here, so the two doors emit bit-identical geometry.
    pub fn fillet(self, corner: Point2<T>, next: Point2<T>, radius: T) -> Result<Self, ProfileError>
    where
        T: Decide + Bounds,
    {
        // This door owns the bracket-read diagnostics (`.lo()`): the
        // shared trim helper stays evaluation-only (no `Bounds`) so
        // the PATHS lowering can call it too (Bounds scope rule).
        let trims =
            line_line_fillet_trims(self.head(), corner, next, radius).map_err(|refusal| {
                match refusal {
                    TrimRefusal::DoesNotFit {
                        leg,
                        setback,
                        leg_length,
                    } => ProfileError::FilletDoesNotFit {
                        leg,
                        carrier: FilletLegCarrier::Line,
                        setback: setback.lo(),
                        leg_length: leg_length.lo(),
                    },
                    TrimRefusal::Escalated(source) => ProfileError::Escalated {
                        site: EscalationSite::Fillet,
                        source,
                    },
                    TrimRefusal::Band(e) => ProfileError::Band(e),
                }
            })?;
        let mut chain = self;
        if trims.fit_in == Sign::Positive {
            chain = chain.line_to(trims.t1).declare_tangent();
        }
        chain = chain.arc_to(trims.t2, trims.bulge);
        if trims.fit_out == Sign::Positive {
            chain = chain.declare_tangent();
        }
        Ok(chain)
    }

    /// Appends a **tangent fillet** rounding a corner whose legs may be
    /// straight *or circular* — the M5 S2 generalization of
    /// [`LoopBuilder::fillet`] (#101 R4): line×arc, arc×line, arc×arc,
    /// and (by delegation) line×line.
    ///
    /// The corner is the meeting point `corner` C of the **incoming**
    /// leg (chain head H → C, shaped by `incoming`) and the **outgoing**
    /// leg (C → `next` N, shaped by `outgoing`). As in
    /// [`LoopBuilder::fillet`] this emits the trimmed incoming leg, then
    /// the radius-`radius` arc from T₁ to T₂, declaring the
    /// strictly-interior tangent joints **by construction**; the caller
    /// continues from T₂ toward `next` — `line_to(next)` for a straight
    /// outgoing leg, `arc_to_center(next, center, sweep)` for a circular
    /// one (or another `fillet_corner` whose `corner` is `next`).
    /// Leaving T₂ any other way is a contradicted declaration, refused
    /// loudly by validation.
    ///
    /// # Why arc legs are given by CARRIER, not bulge
    ///
    /// [`FilletLegShape::Arc`] names the leg's carrier circle
    /// (`center` + sweep sense), not a bulge, because a fillet **trims**
    /// its legs: a bulge describes the untrimmed chord H→C, and after
    /// trimming the chain runs T₁→C on the same circle with a different
    /// bulge. The carrier is trim-invariant, so the same
    /// [`FilletLegShape`] value describes the leg before and after any
    /// trim — including the next corner's incoming leg. The carrier is
    /// the circle about `center` **through the corner** (radius
    /// R = |C − center|); the leg's far end contributes only its angular
    /// position, so an off-carrier far end still bounds the extent by
    /// its angle (the conventional-data posture of
    /// [`LoopBuilder::arc_to_center`]).
    ///
    /// # Construction: offset carriers (solver-free, D9)
    ///
    /// The fillet's center P is the intersection of the two legs'
    /// **offset carriers**, each pushed by r toward the side the corner
    /// turns to (σ = the sign of the corner's turn):
    ///
    /// - a straight leg's carrier offsets to the parallel line through
    ///   C + σ·r·n̂ (n̂ the left normal of the leg's travel direction);
    /// - a circular leg of radius R and own sweep sense τ offsets to the
    ///   concentric circle of **signed** radius ρ = R − σ·τ·r —
    ///   internal tangency (ρ = R − r) when the fillet curves the same
    ///   way as the leg, external (ρ = R + r) when it curves the other
    ///   way. The sign carries the enclosing case (r > R) without a
    ///   branch: the tangent point is `center + (P − center)·(R/ρ)`
    ///   either way.
    ///
    /// Tangent points are then exact from P: the foot along n̂ on a
    /// straight leg, along the center-to-center direction on a circular
    /// one. The arc's bulge is `tan(θ/4)` in the half-angle form
    /// σ·(L/2) / (r + copysign(|M − P|, σ·(T₁−P)×(T₂−P))) with L the
    /// chord |T₂ − T₁| and M its midpoint — algebraic, one square root
    /// per length, correct for major arcs (|θ| > π) too.
    ///
    /// The tangent points, the fillet centre and the FILLET arc's bulge
    /// stay in those sqrt forms, and the gate quantities use `atan2`
    /// (gate inputs, not emitted geometry). The one emitted number that
    /// is *not* algebraic is the trimmed circular leg's own bulge: that
    /// piece is re-emitted through [`LoopBuilder::arc_to_center`], which
    /// derives its bulge from the endpoints' angles about the centre —
    /// `atan2` then `tan`. That is the pre-existing contract of the
    /// `arc_to_center` door (this constructor adds no transcendental of
    /// its own there), but it does mean "every emitted coordinate stays
    /// in sqrt forms" is true of the fillet arc, not of the trimmed leg.
    ///
    /// # The branch rule (deterministic, never a guess)
    ///
    /// Two carriers meet in at most two points, so at most two circles
    /// of radius r are tangent to both legs' carriers. Candidates whose
    /// tangent points lie on the legs' **corner-side extents** survive —
    /// on each leg, `0 ≤ setback ≤ extent`, the two ends classified by
    /// `fillet_leg_reach` and `fillet_leg_fit` against the exact-order
    /// band. Outcomes:
    ///
    /// - exactly one survivor ⇒ that is the fillet;
    /// - **two** survivors ⇒ the candidate **nearest the authored
    ///   corner** wins (M5 S8, Evan's ruling 2026-07-30): smallest total
    ///   tangent setback, ties falling down
    ///   [`crate::fillet_select::nearest_candidate`]'s
    ///   documented ladder. Both survivors are valid tangent fillets of
    ///   the authored legs, so the selection is between constructions,
    ///   not geometric truths — and the far circle stays deliberately
    ///   authorable as the NEAR fillet of the other carrier intersection
    ///   (the S8 reachability rows pin both directions);
    /// - none, but some candidate did reach the corner side and only
    ///   overran a leg's far end ⇒ [`ProfileError::FilletDoesNotFit`]
    ///   (the radius-does-not-fit situation, first candidate and
    ///   incoming leg first);
    /// - none at all ⇒ [`ProfileError::NoCornerForFillet`], either
    ///   because the offset carriers never meet
    ///   (`OffsetCarriersDisjoint`) or because every tangent circle
    ///   touches past the corner (`NoCornerSideCandidate`).
    ///
    /// # The gates
    ///
    /// In fixed order (D9), each a reified predicate through the k_stats
    /// funnel; every escalation lands at
    /// [`crate::EscalationSite::Fillet`] with the predicate named:
    ///
    /// 1. **`fillet_corner_arm`** (linear band) — the lever arm
    ///    `min(leg extents, circular legs' radii)`. Not definitely
    ///    positive ⇒ [`ProfileError::FilletLegDegenerate`]: with no
    ///    length scale the corner's angle means nothing (D4 ¶1; the
    ///    `dihedral_arm` gate's sibling), so nothing is classified.
    /// 2. **`fillet_corner_turn`** (linear band) — the turn margin
    ///    `sin φ · arm` in meters, φ the angle from the incoming to the
    ///    outgoing travel direction at C. Its sign is σ; Zero ⇒
    ///    [`ProfileError::FilletCornerAlreadyTangent`] (the legs meet
    ///    tangentially or double back — no corner to cut; declare the
    ///    tangency instead).
    /// 3. **`fillet_offset_line_circle`** / **`fillet_offset_circles_external`**
    ///    / **`fillet_offset_circles_internal`** (linear band) — the
    ///    offset carriers' clearances, which say whether there are two,
    ///    one, or no candidate centers.
    /// 4. **`fillet_offset_lever`** (linear band, arc×arc only) — the
    ///    outgoing leg's offset radius |ρ| against the least lever the
    ///    band can support at this corner's scale. Not Positive ⇒
    ///    [`ProfileError::FilletOffsetLeverTooShort`]: the carriers meet,
    ///    but the tangent point recovered over that lever would sit
    ///    further than ε off the carrier it claims to be on (M8; the
    ///    derivation and its measured constant are on
    ///    `sugar::ArcCarrier::offset_circles`).
    /// 5. **`fillet_leg_reach`** (exact-order band) — each candidate's
    ///    setback from the corner: the corner end of the extent test.
    /// 6. **`fillet_leg_fit`** (exact-order band) — leg extent − setback,
    ///    exactly as [`LoopBuilder::fillet`] gates it, but with the arc
    ///    leg's setback and extent measured as **arc lengths** `R·Δθ`.
    ///    Negative ⇒ [`ProfileError::FilletDoesNotFit`], now naming the
    ///    leg's carrier and carrying the angular margin `(extent −
    ///    setback)/R` in radians. On the chosen candidate, Zero (exact
    ///    fit) suppresses that side's trimmed piece and its declaration,
    ///    Positive emits both — the same three-way rule as the straight
    ///    case.
    ///
    /// # Errors
    ///
    /// [`ProfileError::FilletLegDegenerate`],
    /// [`ProfileError::FilletCornerAlreadyTangent`],
    /// [`ProfileError::NoCornerForFillet`],
    /// [`ProfileError::FilletDoesNotFit`], [`ProfileError::Escalated`]
    /// (site [`crate::EscalationSite::Fillet`]), or
    /// [`ProfileError::Band`] (only for a misconfigured ε) — see the
    /// gates above.
    pub fn fillet_corner(
        self,
        incoming: FilletLegShape<T>,
        corner: Point2<T>,
        outgoing: FilletLegShape<T>,
        next: Point2<T>,
        radius: T,
        tol: Tolerance,
    ) -> Result<Self, ProfileError>
    where
        T: Decide + Bounds,
    {
        // The construction itself is `arc_fillet_trims` (this file's
        // second shared seam): gates, offset carriers and the
        // per-candidate reach/fit pass, evaluation-only. This door owns
        // the bracket reads — the refusal diagnostics through
        // `map_arc_trim_refusal`, and the S8 pick below.
        let outcome = arc_fillet_trims(incoming, self.head(), corner, outgoing, next, radius, tol)
            .map_err(map_arc_trim_refusal)?;
        let (legs, survivors) = match outcome {
            // Two straight legs: the ratified line×line closed form,
            // bit-identical to `fillet`'s (one door, one construction).
            ArcFilletOutcome::LineLine => return self.fillet(corner, next, radius),
            ArcFilletOutcome::Arc { legs, survivors } => (legs, survivors),
        };

        // The S8 selection, on the f64 diagnostic channel (see
        // `nearest_candidate`): a representation-level choice between
        // already-classified constructions, never a re-decision.
        let setbacks: Vec<[f64; 2]> = survivors
            .iter()
            .map(|c| [c.setbacks[0].lo(), c.setbacks[1].lo()])
            .collect();
        let picked = survivors[crate::fillet_select::nearest_candidate(&setbacks)];

        let mut chain = self;
        if picked.fit_in == Sign::Positive {
            chain = match legs[0].arc {
                None => chain.line_to(picked.t1),
                Some(arc) => chain.arc_to_center(picked.t1, arc.center, arc.sweep),
            };
            chain = chain.declare_tangent();
        }
        chain = chain.arc_to(picked.t2, picked.bulge);
        if picked.fit_out == Sign::Positive {
            chain = chain.declare_tangent();
        }
        Ok(chain)
    }

    /// The finished loop: vertices plus the declared-tangent joints
    /// accumulated by `declare_tangent`/`fillet`.
    fn build(self) -> ProfileLoop<T> {
        ProfileLoop {
            vertices: self.vertices,
            tangent_joints: self.tangent,
        }
    }

    /// Closes the chain with a straight segment back to the start.
    pub fn close(mut self) -> ProfileLoop<T> {
        self.set_leaving_bulge(T::zero());
        self.build()
    }

    /// Closes the chain with an arc of the given `bulge` back to the
    /// start.
    pub fn close_with_bulge(mut self, bulge: T) -> ProfileLoop<T> {
        self.set_leaving_bulge(bulge);
        self.build()
    }

    /// Closes the chain with the arc through `via` back to the start.
    pub fn close_arc_via(self, via: Point2<T>) -> ProfileLoop<T> {
        let first = self.vertices[0].pos;
        let bulge = bulge_from_via(self.head(), via, first);
        self.close_with_bulge(bulge)
    }

    /// Closes the chain with the arc about `center` back to the start,
    /// sweeping `sweep`.
    pub fn close_arc_center(self, center: Point2<T>, sweep: ArcSweep) -> ProfileLoop<T> {
        let first = self.vertices[0].pos;
        let bulge = bulge_from_center(self.head(), first, center, sweep);
        self.close_with_bulge(bulge)
    }
}

/// [`ArcTrimRefusal`] → [`ProfileError`], the builder door's bracket-read
/// diagnostics (`.lo()`) applied exactly where the shipped constructor
/// applied them.
fn map_arc_trim_refusal<T: Bounds>(refusal: ArcTrimRefusal<T>) -> ProfileError {
    match refusal {
        ArcTrimRefusal::LegDegenerate {
            leg_in_arm,
            leg_out_arm,
            arm,
        } => ProfileError::FilletLegDegenerate {
            // Diagnostic-channel comparison (naming the leg for the
            // message), not a geometric decision.
            leg: if leg_in_arm.lo() <= leg_out_arm.lo() {
                FilletLeg::Incoming
            } else {
                FilletLeg::Outgoing
            },
            arm: arm.lo(),
        },
        ArcTrimRefusal::AlreadyTangent { align, margin, arm } => {
            ProfileError::FilletCornerAlreadyTangent {
                reversed: align.lo() < 0.0,
                margin: margin.lo(),
                arm: arm.lo(),
            }
        }
        ArcTrimRefusal::DoesNotFit {
            leg,
            carrier_radius,
            margin,
            setback,
            leg_length,
        } => ProfileError::FilletDoesNotFit {
            leg,
            carrier: match carrier_radius {
                None => FilletLegCarrier::Line,
                Some(radius) => FilletLegCarrier::Arc {
                    radius: radius.lo(),
                    angular_margin: (margin / radius).lo(),
                },
            },
            setback: setback.lo(),
            leg_length: leg_length.lo(),
        },
        ArcTrimRefusal::OffsetLeverTooShort {
            leg,
            carrier_radius,
            offset_radius,
            least_lever,
            margin,
        } => ProfileError::FilletOffsetLeverTooShort {
            leg,
            carrier_radius: carrier_radius.lo(),
            offset_radius: offset_radius.lo(),
            least_lever: least_lever.lo(),
            margin: margin.lo(),
        },
        ArcTrimRefusal::NoCorner { reason, radius } => ProfileError::NoCornerForFillet {
            reason,
            radius: radius.lo(),
        },
        ArcTrimRefusal::Escalated(source) => ProfileError::Escalated {
            site: EscalationSite::Fillet,
            source,
        },
        ArcTrimRefusal::Band(e) => ProfileError::Band(e),
    }
}
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::{Profile, SketchPlane};
    use geom_core::Tolerance;

    fn p2(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }

    #[test]
    fn builder_builds_the_two_arc_circle_by_all_three_forms() {
        let tol = Tolerance::with_eps(1e-9);
        let raw = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to(p2(1.0, 0.0), 1.0)
            .close_with_bulge(1.0);
        let via = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to_via(p2(0.0, -1.0), p2(1.0, 0.0))
            .close_arc_via(p2(0.0, 1.0));
        let center = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to_center(p2(1.0, 0.0), p2(0.0, 0.0), ArcSweep::Ccw)
            .close_arc_center(p2(0.0, 0.0), ArcSweep::Ccw);
        for lp in [raw, via, center] {
            assert_eq!(lp.vertices.len(), 2);
            for v in &lp.vertices {
                assert!((v.bulge - 1.0).abs() < 1e-12, "bulge {}", v.bulge);
            }
            let vp = Profile::new(SketchPlane::xy(), vec![lp])
                .validate(tol)
                .expect("the built circle must validate");
            match vp.loops()[0].segments()[0].kind {
                crate::SegmentKind::Arc { center, radius, .. } => {
                    assert!(center.x.abs() < 1e-12 && center.y.abs() < 1e-12);
                    assert!((radius - 1.0).abs() < 1e-12);
                }
                crate::SegmentKind::Line => panic!("must classify as an arc"),
            }
        }
    }

    #[test]
    fn builder_line_and_arc_mix() {
        // A stadium: two straight sides, two semicircular caps. Every
        // cap joint is an exact line/arc tangency — declared (the #101
        // discipline; undeclared it is refused, see the validate
        // suites).
        let tol = Tolerance::with_eps(1e-9);
        let lp = ProfileLoop::builder(p2(0.0, 0.0))
            .declare_tangent() // left cap → bottom side (closing joint)
            .line_to(p2(2.0, 0.0))
            .declare_tangent() // bottom side → right cap
            .arc_to_center(p2(2.0, 1.0), p2(2.0, 0.5), ArcSweep::Ccw)
            .declare_tangent() // right cap → top side
            .line_to(p2(0.0, 1.0))
            .declare_tangent() // top side → left cap
            .close_arc_center(p2(0.0, 0.5), ArcSweep::Ccw);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("the stadium must validate");
        let kinds: Vec<bool> = vp.loops()[0]
            .segments()
            .iter()
            .map(|s| matches!(s.kind, crate::SegmentKind::Line))
            .collect();
        // Canonical start is (0, 0); chain: line, arc, line, arc.
        assert_eq!(kinds, vec![true, false, true, false]);
    }
}
