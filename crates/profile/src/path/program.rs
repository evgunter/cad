//! **Profiles-as-programs (v2) — the profile side, and the transition
//! table both surfaces are projected from.** The PATHS algebra's
//! authoring surface, recorded as data and replayed through a driver
//! that mirrors the typestate lattice at runtime — the two being
//! MECHANICAL PROJECTIONS of one declaration (PATHS-DESIGN §2c rounds
//! 13–15).
//!
//! Four things carry the design; the rest of the module is their
//! vocabulary ([`Target`], [`ArcData`], [`TipState`], [`ReplayError`],
//! [`DynTip`]) and the mode dispatchers the arms call.
//!
//! 1. `transition_table!` — the one declaration. One row per
//!    (state, verb, kernel fn, next state), expanded into all four
//!    artifacts: the typed method, the driver arm, the [`Step`]
//!    variant, and the [`Verb`] tag.
//! 2. [`Step`] — the step vocabulary: one variant per authoring verb,
//!    storing **authored data only**. `ArcVia`/`ArcCenter` keep the
//!    points the author wrote; their bulges are DERIVED at replay, by
//!    the same binders the typed surface calls. Storing a derived bulge
//!    would re-type a computed value as authored and kill its
//!    parametricity (PROFILES-V2 §V1/§V2).
//! 3. [`ClosedLoop`] — what a closing verb now returns: the lowered
//!    [`ProfileLoop`] *and* the program that produced it. One authoring
//!    surface, two consumers; no second spelling of any verb.
//! 4. [`replay`] — the driver. It holds the in-flight tip as [`DynTip`],
//!    an enum over the lattice states each of which carries the TYPED
//!    [`super::PartialPath`] value, and applying a step is a
//!    match on (state, verb) whose arm bodies can only call the ONE typed
//!    binder that is well-typed at that state.
//!
//! # What the construction makes unwritable — precisely
//!
//! The binder bodies are never duplicated here, the lattice is never
//! re-stated as data, and — since the table shipped — neither surface
//! is written twice. Two distinct properties hold, and both are worth
//! stating exactly rather than generously:
//!
//! **Unwritable, by the table:** a transition present in one surface
//! and absent (or different) in the other. A row IS the transition;
//! delete it and the typed method, the driver arm, the `Step` variant
//! and the `Verb` tag all vanish together, so every consumer of any of
//! the four breaks at COMPILE. There is no second place to write a
//! transition, so an inconsistent pair cannot be spelled.
//!
//! **Unwritable, by the types:** calling a binder on the CARRIED VALUE
//! of a state where that binder is not well-typed. Every arm
//! destructures the real `PartialPath`, so `.tangent()` on a
//! `PlainPoint` tip or a leg on an `Angle` tip is a compile error, not
//! a test failure.
//!
//! **Still writable, and NOT prevented by either:** a row whose arm
//! IGNORES the carried value — a no-op arm returning the tip unchanged,
//! or a laundering arm that MINTS a fresh tip from the step's own
//! arguments — and a row whose arm is merely OVER-STRICT, refusing a
//! pair the typed method accepts. Those compile. The table's grammar
//! rules out only the emptiest case: `arms { }` is a parse error, so a
//! row cannot ship with NO driver projection at all.
//!
//! The rest is backstopped downstream, by tests, and it is worth being
//! exact about which test catches which shape, because a stale claim
//! here is how an unpinned arm hides:
//!
//! - the **no-op** shape → the refusal census
//!   (`lattice_violations_refuse_as_the_transition_class`);
//! - the **over-strict / missing** shape → the replay-coverage census
//!   (`every_table_verb_is_replayed_by_the_corpus`), which replays a
//!   chain for every verb the table declares, anchored on
//!   [`Verb::ALL`] so a new verb cannot join unpinned. It is VERB
//!   granular: an arm of a multi-row verb is covered only where the
//!   corpus reaches its state;
//! - a **laundering** arm, and everything finer than verb granularity
//!   → review of the table, plus the blanket record→replay differential
//!   every closing verb in the corpus funnels through
//!   (`common::pinned`: the recorded program must replay to a
//!   bit-identical loop).
//!
//! Most arms are one expression of the form "destructure, call the one
//! binder, re-wrap". The exceptions are honest and deliberate: the
//! fused `FilletArc`/`ArcFilletArc` arms compose the two HALVES of
//! their verb (open the incoming side, then apply the arrival spec)
//! rather than calling the fused binder, because the wire's arrival
//! mode is an `ArcData` value while the binder takes the mode's own
//! type. They are pinned by the fused-family corpus.
//!
//! # Serde plays no role
//!
//! The `profile` crate stays serde-free (G1 layering). The Expr-bearing,
//! wire-shaped step type is `editor-core`'s; this one is the resolved,
//! scalar-valued form the driver consumes.

use geom_core::{Decide, Point2, Real};

use super::{
    ArcCarrierScalar, Bulge, Center, Flavor, HasAng, HasPos, NoAng, NoPos, Open, PartialPath,
    PathError, Plain, Start, Via, WithIncoming,
};
use crate::ProfileLoop;
use crate::sugar::ArcSweep;

// ------------------------------------------------------------------
// The step vocabulary
// ------------------------------------------------------------------

/// Where a target-taking verb ends: an authored absolute point, or the
/// entry vertex ([`Start`]).
///
/// The distinction is STRUCTURAL — it is the verb's shape, never a
/// value — so it stays literal in the step when the continuous
/// arguments become expressions (PROFILES-V2 §V2). Targeting `Start` IS
/// closing, here exactly as in the typed surface.
#[derive(Clone, Copy, Debug)]
pub enum Target<T: Real> {
    /// An authored absolute point in the profile frame.
    Point(Point2<T>),
    /// The entry vertex: this step closes the loop.
    Start,
}

/// **The unified arc-spec record (§2c rounds 5–9)**: one enum, every
/// authored mode, exactly as the surface's standalone spec types
/// authored it (record-as-you-lower keeps the mode; the VQ contracts
/// rely on that distinctness). The typed surface consumes the
/// standalone types ([`Radius`](super::Radius), [`Bulge`], [`Via`],
/// [`Center`], [`Sweep`](super::Sweep), [`ArcLen`](super::ArcLen))
/// through the
/// state-keyed trait matrix; the wire and the replay driver match THIS
/// enum exhaustively, which is the round-9 forcing argument for the
/// whole family shipping at once.
#[derive(Clone, Copy, Debug)]
pub enum ArcData<T: Real> {
    /// `Radius { r, side }` — arrival mode: centre derived from the
    /// arrival's directed anchor.
    Radius {
        /// The carrier radius.
        r: T,
        /// Which side of the tangent the centre sits on.
        side: super::ArcSide,
    },
    /// `Bulge { p, b }` — chord-relative: leg targets and fused
    /// incoming specs.
    Bulge {
        /// The authored endpoint.
        target: Target<T>,
        /// The authored bulge (M2 convention).
        b: T,
    },
    /// `Via { q, p }` — the arc through an authored point.
    Via {
        /// The through-point.
        q: Point2<T>,
        /// The authored endpoint.
        target: Target<T>,
    },
    /// `Center { c, winding, p }` — the arc about an authored centre.
    Center {
        /// The carrier centre.
        c: Point2<T>,
        /// Travel sense (structural).
        winding: ArcSweep,
        /// The authored anchor/endpoint (`Start` closes).
        target: Target<T>,
    },
    /// `Sweep { r, side, angle }` — endpoint-free: tangent-departing,
    /// endpoint derived.
    Sweep {
        /// The carrier radius.
        r: T,
        /// Which side of the departure tangent the centre sits on.
        side: super::ArcSide,
        /// The swept central angle, radians.
        angle: T,
    },
    /// `ArcLen { r, side, len }` — endpoint-free, extent as arc length.
    ArcLen {
        /// The carrier radius.
        r: T,
        /// Which side of the departure tangent the centre sits on.
        side: super::ArcSide,
        /// The arc length, meters.
        len: T,
    },
}

/// **The transition table — ONE declaration, FOUR projections**
/// (PATHS-DESIGN §2c rounds 13–15, lean (a)).
///
/// A verb is declared exactly once, with one `on` row per lattice
/// state it is well-typed at, and the macro expands each row into all
/// four artifacts: the **typed method** on that state, the **driver
/// arm** in [`apply`], the [`Step`] variant, and the [`Verb`] tag. So
/// none of those four is written twice and no two of them can drift: a
/// missing row is missing from all four, consistently and loudly, and
/// an inconsistent pair is unwritable because there is no second place
/// to write it. Those four are the whole of what the macro expands.
///
/// **The round-9 exhaustiveness pressure does NOT ride this table.**
/// That pressure is over the ARC-MODE vocabulary — [`ArcData`] — and
/// this table is over the VERB vocabulary. `ArcData` is written out
/// above by hand and has no `ALL`; every site that must handle each of
/// its modes is hand-written too, including `do_arc_to_point`,
/// `do_arc_to_directed` and the fused dispatchers below this
/// invocation, none of which the macro produces. The pressure is real
/// — rustc enforces it at each of those matches — but it is bought by
/// hand at every site, not projected from one declaration. That the
/// two vocabularies are unified to different depths is smell-scan
/// **S195**.
///
/// # What the table does not reach
///
/// The count is FOUR because four is what fits inside this crate, not
/// because four is all there are. `editor-core` spells the same
/// vocabulary twice more — an expression-valued document form and a
/// serde-bearing persisted form — because a step there carries `Expr`s
/// and must serialize, and G1 layering keeps both out of `profile`.
///
/// Adding a row here does not add a verb there. It breaks that crate's
/// two exhaustive matches on [`Step`] (its content-key tag table and
/// its lifting door) and stops. The function that goes the other way —
/// document form to [`Step`] — matches the DOCUMENT type and
/// CONSTRUCTS this one, so it cannot see a row added here, and the
/// document, wire and expression-slot vocabularies can stay short
/// while everything compiles. That they do not is a census rather than
/// a compile error, and it lives where it can see both sides:
/// `editor-core/tests/switch_program_vocabulary.rs`, anchored on
/// [`Verb::ALL`] exactly as this crate's own replay-coverage census
/// is.
///
/// # Row grammar
///
/// ```text
/// verb Name { field: Ty }          // the Step payload (+ its rustdoc)
/// bind { field }                   // how an arm destructures the step
/// rows {
///     row {
///         /// rustdoc for the generated typed method
///         on [generics] SelfTy;
///         fn name [ (mut self, a: A) -> Out ] { body }
///         arms { DynTip::X(p) => expr, … }
///     }
/// }
/// ```
///
/// The row's `fn` body is the DECLARATION side: it constructs the step
/// and calls the kernel — the pure geometry, which stays in `path.rs`
/// / `family.rs` under its own name. The row records the step itself
/// where the state's `core` is in scope, and HANDS it to the kernel
/// where it is not (the `family` arrival states, whose fields stay
/// private to their module); either way the `Step::` construction is
/// the row's, which is what makes a deleted row break the variant. The `arms` name the
/// concrete [`DynTip`] variants the row's (possibly marker-generic)
/// state covers; a state the row does not name falls through to the
/// table's one lattice-violation arm.
///
/// `free fn` rows are for the complete-loop program forms, which are
/// free functions rather than methods on a tip; `path` re-exports
/// them so their public paths are the module's own.
macro_rules! transition_table {
    (
        $(
            $(#[doc = $doc:literal])*
            verb $name:ident
                $({ $($(#[doc = $fdoc:literal])* $f:ident : $ft:ty),* $(,)? })?
                $(( $($tt:ty),* ))?
            bind $bind:tt
            rows {
                $(
                    row {
                        $(#[doc = $mdoc:literal])*
                        on [ $($gen:tt)* ] $self_ty:ty ;
                        fn $mname:ident [ $($sig:tt)* ] $mbody:block
                        arms { $tip0:pat => $arm0:expr $(, $tip:pat => $arm:expr)* $(,)? }
                    }
                )*
                $(
                    free {
                        $(#[doc = $fndoc:literal])*
                        fn $fname:ident [ $($fsig:tt)* ] $fbody:block
                        arms { $ftip0:pat => $farm0:expr $(, $ftip:pat => $farm:expr)* $(,)? }
                    }
                )*
            }
        )*
    ) => {
        /// One recorded authoring verb (**authored data only** — every
        /// field is a number or structural tag the author wrote;
        /// derived quantities are re-derived at replay by the same
        /// binders the typed surface calls).
        #[derive(Clone, Copy, Debug)]
        pub enum Step<T: Real> {
            $( $(#[doc = $doc])* $name $({ $($(#[doc = $fdoc])* $f : $ft),* })? $(( $($tt),* ))? ),*
        }

        /// Which verb a step names — the `verb` half of a
        /// [`ReplayErrorKind::Transition`]. One value per [`Step`]
        /// variant, projected from the same declaration.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Verb {
            $( $(#[doc = $doc])* $name ),*
        }

        impl Verb {
            /// Every verb the table declares, in declaration order —
            /// the row set, enumerated from the same declaration, so
            /// the replay-coverage census cannot fall behind a verb
            /// the table gains (`tests/path_program.rs`).
            #[doc(hidden)]
            pub const ALL: &'static [Verb] = &[$( Verb::$name ),*];
        }

        impl<T: Real> Step<T> {
            /// The verb this step names.
            pub fn verb(&self) -> Verb {
                match self {
                    $( Step::$name { .. } => Verb::$name ),*
                }
            }
        }

        $($(
            impl< $($gen)* > $self_ty {
                $(#[doc = $mdoc])*
                pub fn $mname $($sig)* $mbody
            }
        )*)*

        $($(
            $(#[doc = $fndoc])*
            pub fn $fname $($fsig)* $fbody
        )*)*

        /// Applies ONE step to the tip.
        ///
        /// Every arm is a row of the table above, so it can only call
        /// the typed binder the row declares — the one well-typed at
        /// that state. The trailing arm is the lattice violation: a
        /// (state, verb) pair no row declares, which is therefore a
        /// pair the authoring surface cannot spell.
        #[allow(clippy::too_many_lines)]
        fn apply<T: ArcCarrierScalar>(tip: DynTip<T>, step: Step<T>) -> Applying<T> {
            match (tip, step) {
                $($(
                    ($tip0, Step::$name $bind) => $arm0,
                    $( ($tip, Step::$name $bind) => $arm, )*
                )*$(
                    ($ftip0, Step::$name $bind) => $farm0,
                    $( ($ftip, Step::$name $bind) => $farm, )*
                )*)*
                (other, unusable) => Err(ReplayErrorKind::Transition {
                    state: other.state(),
                    verb: Some(unusable.verb()),
                }),
            }
        }
    };
}

transition_table! {
    #[doc = " `.at(p)` — bind the position bit."]
    verb At(Point2<T>) bind (p) rows {
        row {
            /// Binds the entry position: `Open → Point` (plain flavor — the
            /// entry has no incoming carrier; its junction check happens at
            /// the seam).
            on [] Open;
            fn at [<T: Real>(self, p: Point2<T>) -> PartialPath<T, HasPos<Plain>, NoAng>] {
                let mut path = self.at_kernel(p);
                path.core.record(Step::At(p));
                path
            }
            arms {
                DynTip::Entry => Ok(Applied::Tip(DynTip::PlainPoint(Open.at(p)))),
            }
        }
        row {
            /// Adds the position bit (`Open → Point`, `Angle → Directed`) —
            /// written once, generic over the angle slot it does not touch.
            ///
            /// On a fillet arrival whose angle is already bound, completing
            /// the position resolves the fillet (both carriers fixed): the
            /// corner construction and anchor-fit gates run here — see
            /// [`PathError`]. On the angle-first entry, this seeds the chain.
            /// `p` is absolute (profile frame), a real on-path point (the
            /// side's anchor).
            on [T: Decide, A: super::AngMarker] PartialPath<T, NoPos, A>;
            fn at [(mut self, p: Point2<T>)
                -> Result<PartialPath<T, HasPos<Plain>, A>, PathError<T>>] {
                self.core.record(Step::At(p));
                self.at_kernel(p)
            }
            arms {
                DynTip::Open(p0) => Ok(Applied::Tip(DynTip::PlainPoint(p0.at(p)?))),
                DynTip::Angle(p0) => Ok(Applied::Tip(DynTip::DirectedPlain(p0.at(p)?))),
            }
        }
        row {
            /// Binds the arrival's anchor — a real on-path point on the
            /// derived carrier.
            on [T: Decide] super::family::RadiusArrival<T>;
            fn at [(self, p: Point2<T>) -> super::family::RadiusArrivalAt<T>] {
                self.at_kernel(Step::At(p), p)
            }
            arms {
                DynTip::RadiusArrival(p0) => Ok(Applied::Tip(DynTip::RadiusArrivalAt(p0.at(p)))),
            }
        }
        row {
            /// Completes the arrival with its anchor; the fillet resolves.
            on [T: Decide] super::family::RadiusArrivalDir<T>;
            fn at [(self, p: Point2<T>)
                -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.at_kernel(Step::At(p), p)
            }
            arms {
                DynTip::RadiusArrivalDir(p0) => Ok(Applied::Tip(DynTip::DirectedPoint(p0.at(p)?))),
            }
        }
    }

    #[doc = " `.angle(θ)` — bind the outgoing direction as an angle (radians)."]
    verb Angle(T) bind (theta) rows {
        row {
            /// Binds the entry direction first: `Open → Angle` (radians, in
            /// the sketch plane; position pending).
            on [] Open;
            fn angle [<T: Real>(self, theta: T) -> PartialPath<T, NoPos, HasAng>] {
                let mut path = self.director(super::Dir::from_angle(theta));
                path.core.record(Step::Angle(theta));
                path
            }
            arms {
                DynTip::Entry => Ok(Applied::Tip(DynTip::Angle(Open.angle(theta)))),
            }
        }
        row {
            /// Adds the angle bit wherever it is missing (`Point → Directed`,
            /// `Open → Angle`) — one generic function; the junction check
            /// reads the flavor's optional incoming tangent at runtime.
            ///
            /// On a directed point this classifies `theta` against the
            /// incoming tangent and its reverse (PATHS-DESIGN §4 item 1):
            /// definitely-sharp proceeds; within ε_input of tangent refuses
            /// [`PathError::JunctionTangent`] (one refusal, one recourse:
            /// `.tangent()` makes intended tangency exact by construction);
            /// within ε_input of the reverse refuses
            /// [`PathError::JunctionCusp`] (no declaration door — #131). On a
            /// plain point there is nothing to check (an arrival side meets
            /// its fillet arc tangentially by construction; the entry's check
            /// happens at the seam). On a fillet arrival whose position is
            /// already bound, completing the direction resolves the fillet.
            on [T: Decide, P: super::PosMarker] PartialPath<T, P, NoAng>;
            fn angle [(mut self, theta: T)
                -> Result<PartialPath<T, P, HasAng>, PathError<T>>] {
                self.core.record(Step::Angle(theta));
                self.director(super::Dir::from_angle(theta))
            }
            arms {
                DynTip::Open(p0) => Ok(Applied::Tip(DynTip::Angle(p0.angle(theta)?))),
                DynTip::PlainPoint(p0) => Ok(Applied::Tip(DynTip::DirectedPlain(p0.angle(theta)?))),
                DynTip::DirectedPoint(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedIncoming(p0.angle(theta)?))),
            }
        }
        row {
            /// Binds the arrival direction (angle-first order).
            on [T: Decide] super::family::RadiusArrival<T>;
            fn angle [(self, theta: T) -> super::family::RadiusArrivalDir<T>] {
                self.angle_kernel(Step::Angle(theta), theta)
            }
            arms {
                DynTip::RadiusArrival(p0) =>
                    Ok(Applied::Tip(DynTip::RadiusArrivalDir(p0.angle(theta)))),
            }
        }
        row {
            /// Completes the arrival with its direction; the fillet resolves.
            on [T: Decide] super::family::RadiusArrivalAt<T>;
            fn angle [(self, theta: T)
                -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.angle_kernel(Step::Angle(theta), theta)
            }
            arms {
                DynTip::RadiusArrivalAt(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedPoint(p0.angle(theta)?))),
            }
        }
        row {
            /// Completes the directed anchor with an angle; the fillet resolves.
            on [T: Decide] super::family::ViaArrival<T>;
            fn angle [(self, theta: T)
                -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.angle_kernel(Step::Angle(theta), theta)
            }
            arms {
                DynTip::ViaArrival(p0) => Ok(Applied::Tip(DynTip::DirectedPoint(p0.angle(theta)?))),
            }
        }
        row {
            /// Completes the close with an angle at the entry anchor.
            on [T: Decide] super::family::ViaArrivalStart<T>;
            fn angle [(self, theta: T) -> Result<ClosedLoop<T>, PathError<T>>] {
                self.angle_kernel(Step::Angle(theta), theta)
            }
            arms {
                DynTip::ViaArrivalStart(p0) => Ok(Applied::Closed(p0.angle(theta)?.loop_)),
            }
        }
    }

    #[doc = " `.toward(dx, dy)` — bind it as exact components (ratio-only)."]
    verb Toward {
        #[doc = " x component."]
        dx: T,
        #[doc = " y component."]
        dy: T,
    } bind { dx, dy } rows {
        row {
            /// Binds the entry direction first as exact COMPONENTS
            /// (`Open → Angle`): the direction-valued alternative to
            /// [`angle`](Self::angle) — see [`PartialPath::toward`] for the
            /// exactness contract and the refusal.
            on [] Open;
            fn toward [<T: Decide>(
                self,
                dx: T,
                dy: T,
            ) -> Result<PartialPath<T, NoPos, HasAng>, PathError<T>>] {
                let mut path = self.director(super::unit_from_components(dx, dy)?);
                path.core.record(Step::Toward { dx, dy });
                Ok(path)
            }
            arms {
                DynTip::Entry => Ok(Applied::Tip(DynTip::Angle(Open.toward(dx, dy)?))),
            }
        }
        row {
            /// The direction-valued director (G1 constructor 5): binds the same
            /// angular DOF as [`angle`](Self::angle) — the same lattice slot,
            /// set at most once per side — from exact COMPONENTS instead of an
            /// angle. `(dx, dy)` is normalized and the unit ray stored verbatim,
            /// so the departure never makes a trig round-trip: `.toward(-1, 0)`
            /// gives the ray `(-1, 0)` exactly, where `.angle(PI)` gives
            /// `(-1, 1.2246e-16)` and carries that ulp into every corner and
            /// trim point downstream. Only the components' RATIO is read
            /// (magnitude is not a length and binds nothing).
            ///
            /// `(0, 0)` — and any norm within ε_input of zero — refuses
            /// [`PathError::ZeroDirection`]: it names no direction, and the
            /// recourse is free, since scaling the components changes nothing
            /// else. Junction/fillet semantics are otherwise identical to
            /// [`angle`](Self::angle), including the §4 item 1 check on a
            /// directed point and the fillet resolution on a bound arrival.
            on [T: Decide, P: super::PosMarker] PartialPath<T, P, NoAng>;
            fn toward [(mut self, dx: T, dy: T)
                -> Result<PartialPath<T, P, HasAng>, PathError<T>>] {
                self.core.record(Step::Toward { dx, dy });
                self.director(super::unit_from_components(dx, dy)?)
            }
            arms {
                DynTip::Open(p0) => Ok(Applied::Tip(DynTip::Angle(p0.toward(dx, dy)?))),
                DynTip::PlainPoint(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedPlain(p0.toward(dx, dy)?))),
                DynTip::DirectedPoint(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedIncoming(p0.toward(dx, dy)?))),
            }
        }
        row {
            /// Binds the arrival direction as exact components.
            on [T: Decide] super::family::RadiusArrival<T>;
            fn toward [(self, dx: T, dy: T)
                -> Result<super::family::RadiusArrivalDir<T>, PathError<T>>] {
                self.toward_kernel(Step::Toward { dx, dy }, dx, dy)
            }
            arms {
                DynTip::RadiusArrival(p0) =>
                    Ok(Applied::Tip(DynTip::RadiusArrivalDir(p0.toward(dx, dy)?))),
            }
        }
        row {
            /// Completes the arrival with exact components; the fillet resolves.
            on [T: Decide] super::family::RadiusArrivalAt<T>;
            fn toward [(self, dx: T, dy: T)
                -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.toward_kernel(Step::Toward { dx, dy }, dx, dy)
            }
            arms {
                DynTip::RadiusArrivalAt(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedPoint(p0.toward(dx, dy)?))),
            }
        }
        row {
            /// Completes the directed anchor with exact components.
            on [T: Decide] super::family::ViaArrival<T>;
            fn toward [(self, dx: T, dy: T)
                -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.toward_kernel(Step::Toward { dx, dy }, dx, dy)
            }
            arms {
                DynTip::ViaArrival(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedPoint(p0.toward(dx, dy)?))),
            }
        }
        row {
            /// Completes the close with exact components at the entry anchor.
            on [T: Decide] super::family::ViaArrivalStart<T>;
            fn toward [(self, dx: T, dy: T) -> Result<ClosedLoop<T>, PathError<T>>] {
                self.toward_kernel(Step::Toward { dx, dy }, dx, dy)
            }
            arms {
                DynTip::ViaArrivalStart(p0) => Ok(Applied::Closed(p0.toward(dx, dy)?.loop_)),
            }
        }
    }

    #[doc = " `.tangent()` — inherit the incoming end tangent and DECLARE the joint."]
    verb Tangent bind {} rows {
        row {
            /// Consumes a **directed point only**: re-uses the incoming end
            /// tangent as the departure — exact by construction, nothing for
            /// verification to contradict — and emits the DECLARED flag on
            /// lowering. Ill-typed on plain points (no direction to inherit),
            /// which is what makes "fillets sit between defined geometry"
            /// structural rather than a rule.
            on [T: Decide] PartialPath<T, HasPos<WithIncoming>, NoAng>;
            fn tangent [(mut self) -> PartialPath<T, HasPos<WithIncoming>, HasAng>] {
                self.core.record(Step::Tangent);
                self.tangent_kernel()
            }
            arms {
                DynTip::DirectedPoint(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedIncoming(p0.tangent()))),
            }
        }
    }

    #[doc = " `.turn(δ)` — depart at the incoming tangent rotated by δ."]
    verb Turn(T) bind (delta) rows {
        row {
            /// `.angle(incoming + δ)` sugar on a directed point: turns by `δ`
            /// radians from the incoming tangent. `turn(0)` lands in the
            /// tangent band and refuses (use [`tangent`](Self::tangent));
            /// `turn(±π)` lands in the reverse band and refuses as a cusp.
            on [T: Decide] PartialPath<T, HasPos<WithIncoming>, NoAng>;
            fn turn [(
                mut self,
                delta: T,
            ) -> Result<PartialPath<T, HasPos<WithIncoming>, HasAng>, PathError<T>>] {
                self.core.record(Step::Turn(delta));
                self.turn_kernel(delta)
            }
            arms {
                DynTip::DirectedPoint(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedIncoming(p0.turn(delta)?))),
            }
        }
    }

    #[doc = " `line(len)` — a straight leg along the bound direction."]
    verb Line(T) bind (len) rows {
        row {
            /// A straight leg of length `len` along the bound departure,
            /// terminating at a directed point. After a fillet this extends
            /// the arrival side's one leg past its anchor (no collinear
            /// neighbor is minted — §4 item 4's by-construction exemption).
            ///
            /// A declared straight continuation of a straight leg
            /// (`.tangent().line(len)` after a line) IS the same carrier and
            /// refuses [`PathError::SameCarrierJunction`] — extend the
            /// original leg instead.
            ///
            /// `len` must classify definitely positive
            /// ([`PathError::NonpositiveLeg`] otherwise): a negative length
            /// would run the side backward, silently detaching the tip's
            /// anchored points from the final path — the §4 item 3 invariant
            /// is gated here, at the one verb that takes a signed length.
            on [T: Decide, F: Flavor] PartialPath<T, HasPos<F>, HasAng>;
            fn line [(
                mut self,
                len: T,
            ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.core.record(Step::Line(len));
                self.line_kernel(len)
            }
            arms {
                DynTip::DirectedPlain(p0) => Ok(Applied::Tip(DynTip::DirectedPoint(p0.line(len)?))),
                DynTip::DirectedIncoming(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedPoint(p0.line(len)?))),
            }
        }
    }

    #[doc = " `line_to(target)` — a straight leg to the target."]
    verb LineTo(Target<T>) bind (target) rows {
        row {
            /// `.angle(toward target).line(distance)` in one call
            /// (`Point → Point`, also from arrivals): on a directed point the
            /// junction check runs on the computed direction; on a fillet
            /// arrival this binds the arrival direction toward the target,
            /// resolves the fillet, and ends the side at the target.
            /// `line_to(Start)` is the sharp straight seam (both seam-side
            /// junction checks run; a within-band-tangent straight closer is
            /// the overdetermined tangent line close and refuses always).
            on [T: Decide, F: Flavor] PartialPath<T, HasPos<F>, NoAng>;
            fn line_to [<Tgt: super::LineTarget<T, F>>(self, target: Tgt) -> Tgt::Out] {
                <Tgt as super::LineTarget<T, F>>::line_from(self, target)
            }
            arms {
                DynTip::PlainPoint(p0) => do_line_to(p0, target),
                DynTip::DirectedPoint(p0) => do_line_to(p0, target),
            }
        }
    }
    #[doc = " `arc_to(spec)` — the sharp arc leg, every mode in the one"]
    #[doc = " unified [`ArcData`] record; the mode the author wrote is"]
    #[doc = " what is kept, because the VQ contracts rely on it."]
    verb ArcTo(ArcData<T>) bind (spec) rows {
        row {
            /// **§2c**: the SHARP arc leg from a point tip — one verb over the
            /// endpoint-full `ArcData` modes (`Bulge{p, b}` chord-relative,
            /// `Via{q, p}` through a point, `Center{c, winding, p}` about a
            /// centre). `p: Start` is the sharp arc seam.
            ///
            /// Each mode's authored data is stored VERBATIM and its bulge
            /// derived by the one closed form the raw chain uses
            /// ([`crate::bulge_from_via`] / [`crate::bulge_from_center`]), so
            /// the doors emit the same bits. On a directed point the §4 item 1
            /// junction check runs on the arc's START TANGENT.
            ///
            /// Refusals: a through-point within ε_input of the chord LINE
            /// ([`PathError::ArcViaCollinear`] — the whole collinear class);
            /// coincident endpoints ([`PathError::DegenerateArcChord`]); a
            /// centre whose two radii disagree definitely
            /// ([`PathError::ArcCenterNotEquidistant`] — checked, never
            /// repaired: re-projecting would move an authored point, which §4
            /// item 3 forbids) or sits within ε_input of an endpoint
            /// ([`PathError::DegenerateArcCenter`]).
            on [T: Decide, F: Flavor] PartialPath<T, HasPos<F>, NoAng>;
            fn arc_to [<S: super::family::PointLeg<T, F>>(self, spec: S) -> S::Out] {
                <S as super::family::PointLeg<T, F>>::leg_from(self, spec)
            }
            arms {
                DynTip::PlainPoint(p0) => do_arc_to_point(p0, spec, TipState::PlainPoint),
                DynTip::DirectedPoint(p0) => do_arc_to_point(p0, spec, TipState::DirectedPoint),
            }
        }
        row {
            /// **§2c**: the endpoint-free SHARP arc legs — `arc_to(spec)` with
            /// `Sweep`/`ArcLen`, the arc analogs of `line(len)`: tangent
            /// departure (already junction-checked when the director bound),
            /// endpoint derived, terminating at a directed point.
            on [T: ArcCarrierScalar, F: Flavor] PartialPath<T, HasPos<F>, HasAng>;
            fn arc_to [<S: super::family::TangentIncoming<T>>(
                mut self,
                spec: S,
            ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.core
                    .record(Step::ArcTo(super::family::TangentIncoming::to_wire(&spec)));
                self.arc_to_kernel(spec)
            }
            arms {
                DynTip::DirectedPlain(p0) => do_arc_to_directed(p0, spec, TipState::DirectedPlain),
                DynTip::DirectedIncoming(p0) =>
                    do_arc_to_directed(p0, spec, TipState::DirectedIncoming),
            }
        }
    }

    #[doc = " `tangent_arc_to(target)` — the unique tangent arc to the target."]
    verb TangentArcTo(Target<T>) bind (target) rows {
        row {
            /// The unique arc tangent to the bound departure through the
            /// target: `tangent_arc_to(p)` continues to a directed point;
            /// `tangent_arc_to(Start)` is the tangent-seam close (the seam's
            /// junction check runs at `Start` with both directions known).
            on [T: Decide, F: Flavor] PartialPath<T, HasPos<F>, HasAng>;
            fn tangent_arc_to [
                <Tgt: super::TangentArcTarget<T, F>>(self, target: Tgt) -> Tgt::Out
            ] {
                <Tgt as super::TangentArcTarget<T, F>>::tangent_arc_from(self, target)
            }
            arms {
                DynTip::DirectedPlain(p0) => do_tangent_arc_to(p0, target),
                DynTip::DirectedIncoming(p0) => do_tangent_arc_to(p0, target),
            }
        }
    }

    #[doc = " `arc_continue(target)` — the declared-subdivision step."]
    verb ArcContinue(Point2<T>) bind (p) rows {
        row {
            /// **The declared-subdivision step** (LIB-SWITCH §5-1 fallback,
            /// ruled 2026-08-08): continue the incoming ARC CARRIER to
            /// `target`, minting a STRUCTURAL subdivision vertex — a vertex the
            /// author placed on the carrier deliberately (the half-disc's
            /// equator vertex, which revolve naming's pole elimination anchors
            /// on), not a junction claim of any kind.
            ///
            /// Semantics, precisely: the leg runs on the SAME carrier circle as
            /// the incoming leg, in the same travel sense, from the tip to
            /// `target`. The junction at the tip is a same-carrier IDENTITY —
            /// exactly the class [`circle`]'s two poles are — so NO §4 junction
            /// check runs (there is no departure to classify: the carrier
            /// continues) and NOTHING is declared tangent (there is no tangency
            /// claim to verify; #101's same-carrier-is-identity rule applies at
            /// validation unchanged). The bulge is DERIVED from the carrier and
            /// the target — authored data is the target alone.
            ///
            /// Refusals: no incoming arc carrier
            /// ([`PathError::ArcContinueNeedsArcCarrier`] — a straight leg has
            /// nothing to subdivide); a target off the carrier
            /// ([`PathError::ArcContinueOffCarrier`] — authored points never
            /// re-project); a degenerate chord
            /// ([`PathError::DegenerateArcChord`]).
            on [T: Decide] PartialPath<T, HasPos<WithIncoming>, NoAng>;
            fn arc_continue [(
                mut self,
                target: Point2<T>,
            ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.core.record(Step::ArcContinue(target));
                self.arc_continue_kernel(target)
            }
            arms {
                DynTip::DirectedPoint(p0) =>
                    Ok(Applied::Tip(DynTip::DirectedPoint(p0.arc_continue(p)?))),
            }
        }
    }

    #[doc = " `.fillet(r)` — line incoming (the tangent ray), line arrival."]
    verb Fillet {
        #[doc = " The fillet radius."]
        radius: T,
    } bind { radius } rows {
        row {
            /// Opens a corner fillet of radius `radius`: consumes the incoming
            /// Directed (the departure ray) and opens the arrival side Open,
            /// bound in either order (`.at(dd).angle(θ)`, `.angle(θ).at(dd)`,
            /// or `.to(Start)` for the seam). Once the arrival is Directed the
            /// r-arc tangent to both carriers is inserted at their implicit
            /// virtual corner, trimming both — the corner is never authored
            /// (it exists only as the carrier intersection), and authoring a
            /// point then filleting it away is unrepresentable.
            ///
            /// `radius` must classify definitely positive
            /// ([`PathError::NonpositiveFilletRadius`] otherwise), gated here —
            /// before an arrival can be authored against a fillet that has no
            /// tangent construction to offer.
            on [T: Decide, F: Flavor] PartialPath<T, HasPos<F>, HasAng>;
            fn fillet [(
                mut self,
                radius: T,
            ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>>] {
                self.core.record(Step::Fillet { radius });
                self.fillet_kernel(radius)
            }
            arms {
                DynTip::DirectedPlain(p0) => Ok(Applied::Tip(DynTip::Open(p0.fillet(radius)?))),
                DynTip::DirectedIncoming(p0) => Ok(Applied::Tip(DynTip::Open(p0.fillet(radius)?))),
            }
        }
        row {
            /// **§2c round 10 — RAY EXTENSION**: bare `fillet(r)` directly on a
            /// leg end. The incoming contact sits on the TANGENT RAY ahead of
            /// the directed point, as new path: the surviving ray piece is a
            /// genuine line leg extending from the leg's end (declared tangent
            /// by construction — the ray IS the tangent), whatever leg came
            /// before. Line arrival.
            on [T: ArcCarrierScalar] PartialPath<T, HasPos<WithIncoming>, NoAng>;
            fn fillet [(
                mut self,
                radius: T,
            ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>>] {
                self.core.record(Step::Fillet { radius });
                self.fillet_kernel(radius)
            }
            arms {
                DynTip::DirectedPoint(p0) => Ok(Applied::Tip(DynTip::Open(p0.fillet(radius)?))),
            }
        }
    }

    #[doc = " `fillet_arc(r, spec)` — line incoming, ARC arrival per the spec."]
    verb FilletArc {
        #[doc = " The fillet radius."]
        radius: T,
        #[doc = " The arc-arrival spec."]
        spec: ArcData<T>,
    } bind { radius, spec } rows {
        row {
            /// **§2c**: line incoming, ARC arrival — consumes the directed tip
            /// (the incoming side is its ray) and opens/resolves the arc
            /// arrival per the spec mode's own completion story.
            on [T: ArcCarrierScalar, F: Flavor] PartialPath<T, HasPos<F>, HasAng>;
            fn fillet_arc [<S: super::family::ArrivalSpec<T>>(
                mut self,
                radius: T,
                spec: S,
            ) -> S::Out] {
                self.core.record(Step::FilletArc {
                    radius,
                    spec: super::family::ArrivalSpec::to_wire(&spec),
                });
                self.fillet_arc_kernel(radius, spec)
            }
            arms {
                DynTip::DirectedPlain(p0) => do_arrival(
                    p0.fillet(radius)?,
                    spec,
                    TipState::DirectedPlain,
                    Verb::FilletArc,
                ),
                DynTip::DirectedIncoming(p0) => do_arrival(
                    p0.fillet(radius)?,
                    spec,
                    TipState::DirectedIncoming,
                    Verb::FilletArc,
                ),
            }
        }
        row {
            /// Ray extension with an ARC arrival (`fillet_arc` off a leg end).
            on [T: ArcCarrierScalar] PartialPath<T, HasPos<WithIncoming>, NoAng>;
            fn fillet_arc [<S: super::family::ArrivalSpec<T>>(
                mut self,
                radius: T,
                spec: S,
            ) -> S::Out] {
                self.core.record(Step::FilletArc {
                    radius,
                    spec: super::family::ArrivalSpec::to_wire(&spec),
                });
                self.fillet_arc_kernel(radius, spec)
            }
            arms {
                DynTip::DirectedPoint(p0) => do_arrival(
                    p0.fillet(radius)?,
                    spec,
                    TipState::DirectedPoint,
                    Verb::FilletArc,
                ),
            }
        }
    }

    #[doc = " `arc_fillet(spec, r)` — fused ARC incoming, line arrival."]
    verb ArcFillet {
        #[doc = " The fused incoming-arc spec."]
        spec: ArcData<T>,
        #[doc = " The fillet radius."]
        radius: T,
    } bind { spec, radius } rows {
        row {
            /// **§2c, the entry fused verb**: authors the ENTRY side ON an arc
            /// carrier — the spec's `p` is the entry anchor, the direction is
            /// the carrier's tangent there (derived, never authored) — and
            /// opens a fillet of `radius` off that carrier, line arrival.
            ///
            /// The entry's carrier and the fillet that trims it are ONE
            /// authoring act, which is what the axiom demands: a fillet that
            /// needs an arc carrier cannot learn it, so it authors it.
            on [] Open;
            fn arc_fillet [<T: ArcCarrierScalar>(
                self,
                spec: Center<T, Point2<T>>,
                radius: T,
            ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>>] {
                let step = Step::ArcFillet {
                    spec: super::family::PointIncoming::to_wire(&spec),
                    radius,
                };
                self.arc_fillet_kernel(step, spec, radius)
            }
            arms {
                DynTip::Entry => Ok(Applied::Tip(DynTip::Open(do_fused_entry(spec, radius)?))),
            }
        }
        row {
            /// **§2c**: fused arc incoming from a PLAIN point tip — the
            /// endpoint-full modes author the incoming side's carrier and its
            /// anchor `p` in one act. Line arrival.
            on [T: ArcCarrierScalar] PartialPath<T, HasPos<Plain>, NoAng>;
            fn arc_fillet [<S: super::family::PointIncoming<T>>(
                mut self,
                spec: S,
                radius: T,
            ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>>] {
                self.core.record(Step::ArcFillet {
                    spec: super::family::PointIncoming::to_wire(&spec),
                    radius,
                });
                self.arc_fillet_kernel(spec, radius)
            }
            arms {
                DynTip::PlainPoint(p0) => Ok(Applied::Tip(DynTip::Open(do_fused_point(
                    p0,
                    spec,
                    radius,
                    TipState::PlainPoint,
                    Verb::ArcFillet,
                )?))),
            }
        }
        row {
            /// **§2c**: fused arc incoming from a DIRECTED POINT — the
            /// endpoint-full modes (junction-checked at the tip, as the sharp
            /// legs check theirs) plus `Radius`: ARC EXTENSION, the arc analog
            /// of ray extension (see [`LegEndIncoming`](super::LegEndIncoming)). Line arrival.
            on [T: ArcCarrierScalar] PartialPath<T, HasPos<WithIncoming>, NoAng>;
            fn arc_fillet [<S: super::family::LegEndIncoming<T>>(
                mut self,
                spec: S,
                radius: T,
            ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>>] {
                self.core.record(Step::ArcFillet {
                    spec: super::family::LegEndIncoming::to_wire(&spec),
                    radius,
                });
                self.arc_fillet_kernel(spec, radius)
            }
            arms {
                DynTip::DirectedPoint(p0) => Ok(Applied::Tip(DynTip::Open(do_fused_leg_end(
                    p0,
                    spec,
                    radius,
                    TipState::DirectedPoint,
                    Verb::ArcFillet,
                )?))),
            }
        }
        row {
            /// **§2c**: fused ARC incoming from a directed tip — the
            /// endpoint-free pair departs tangentially and derives its
            /// endpoint, which becomes the incoming side's anchor; the fillet
            /// of `radius` trims off its far end. Line arrival.
            on [T: ArcCarrierScalar, F: Flavor] PartialPath<T, HasPos<F>, HasAng>;
            fn arc_fillet [<S: super::family::TangentIncoming<T>>(
                mut self,
                spec: S,
                radius: T,
            ) -> Result<PartialPath<T, NoPos, NoAng>, PathError<T>>] {
                self.core.record(Step::ArcFillet {
                    spec: super::family::TangentIncoming::to_wire(&spec),
                    radius,
                });
                self.arc_fillet_kernel(spec, radius)
            }
            arms {
                DynTip::DirectedPlain(p0) => Ok(Applied::Tip(DynTip::Open(do_fused_directed(
                    p0,
                    spec,
                    radius,
                    TipState::DirectedPlain,
                    Verb::ArcFillet,
                )?))),
                DynTip::DirectedIncoming(p0) => Ok(Applied::Tip(DynTip::Open(do_fused_directed(
                    p0,
                    spec,
                    radius,
                    TipState::DirectedIncoming,
                    Verb::ArcFillet,
                )?))),
            }
        }
    }

    #[doc = " `arc_fillet_arc(spec, r, spec₂)` — fused arc incoming, arc arrival."]
    verb ArcFilletArc {
        #[doc = " The fused incoming-arc spec."]
        spec: ArcData<T>,
        #[doc = " The fillet radius."]
        radius: T,
        #[doc = " The arc-arrival spec."]
        spec2: ArcData<T>,
    } bind { spec, radius, spec2 } rows {
        row {
            /// The entry fused verb with an ARC arrival: `arc_fillet` whose
            /// arrival is the spec₂ mode's own completion (a `Center` interior
            /// anchor resolves at the verb; `Center { p: Start }` would close a
            /// two-sided loop; `Radius`/`Via` await their binders).
            on [] Open;
            fn arc_fillet_arc [<T: ArcCarrierScalar, S2: super::family::ArrivalSpec<T>>(
                self,
                spec: Center<T, Point2<T>>,
                radius: T,
                spec2: S2,
            ) -> S2::Out] {
                let step = Step::ArcFilletArc {
                    spec: super::family::PointIncoming::to_wire(&spec),
                    radius,
                    spec2: super::family::ArrivalSpec::to_wire(&spec2),
                };
                self.arc_fillet_arc_kernel(step, spec, radius, spec2)
            }
            arms {
                DynTip::Entry => do_arrival(
                    do_fused_entry(spec, radius)?,
                    spec2,
                    TipState::Entry,
                    Verb::ArcFilletArc,
                ),
            }
        }
        row {
            /// **§2c**: fused arc incoming (point modes) AND arc arrival.
            on [T: ArcCarrierScalar] PartialPath<T, HasPos<Plain>, NoAng>;
            fn arc_fillet_arc [
                <Si: super::family::PointIncoming<T>, S2: super::family::ArrivalSpec<T>>(
                    mut self,
                    spec: Si,
                    radius: T,
                    spec2: S2,
                ) -> S2::Out
            ] {
                self.core.record(Step::ArcFilletArc {
                    spec: super::family::PointIncoming::to_wire(&spec),
                    radius,
                    spec2: super::family::ArrivalSpec::to_wire(&spec2),
                });
                self.arc_fillet_arc_kernel(spec, radius, spec2)
            }
            arms {
                DynTip::PlainPoint(p0) => do_arrival(
                    do_fused_point(p0, spec, radius, TipState::PlainPoint, Verb::ArcFilletArc)?,
                    spec2,
                    TipState::PlainPoint,
                    Verb::ArcFilletArc,
                ),
            }
        }
        row {
            /// **§2c**: fused arc incoming (directed-point modes) AND arc
            /// arrival.
            on [T: ArcCarrierScalar] PartialPath<T, HasPos<WithIncoming>, NoAng>;
            fn arc_fillet_arc [
                <Si: super::family::LegEndIncoming<T>, S2: super::family::ArrivalSpec<T>>(
                    mut self,
                    spec: Si,
                    radius: T,
                    spec2: S2,
                ) -> S2::Out
            ] {
                self.core.record(Step::ArcFilletArc {
                    spec: super::family::LegEndIncoming::to_wire(&spec),
                    radius,
                    spec2: super::family::ArrivalSpec::to_wire(&spec2),
                });
                self.arc_fillet_arc_kernel(spec, radius, spec2)
            }
            arms {
                DynTip::DirectedPoint(p0) => do_arrival(
                    do_fused_leg_end(
                        p0,
                        spec,
                        radius,
                        TipState::DirectedPoint,
                        Verb::ArcFilletArc,
                    )?,
                    spec2,
                    TipState::DirectedPoint,
                    Verb::ArcFilletArc,
                ),
            }
        }
        row {
            /// **§2c**: fused arc incoming AND arc arrival.
            on [T: ArcCarrierScalar, F: Flavor] PartialPath<T, HasPos<F>, HasAng>;
            fn arc_fillet_arc [
                <Si: super::family::TangentIncoming<T>, S2: super::family::ArrivalSpec<T>>(
                    mut self,
                    spec: Si,
                    radius: T,
                    spec2: S2,
                ) -> S2::Out
            ] {
                self.core.record(Step::ArcFilletArc {
                    spec: super::family::TangentIncoming::to_wire(&spec),
                    radius,
                    spec2: super::family::ArrivalSpec::to_wire(&spec2),
                });
                self.arc_fillet_arc_kernel(spec, radius, spec2)
            }
            arms {
                DynTip::DirectedPlain(p0) => do_arrival(
                    do_fused_directed(
                        p0,
                        spec,
                        radius,
                        TipState::DirectedPlain,
                        Verb::ArcFilletArc,
                    )?,
                    spec2,
                    TipState::DirectedPlain,
                    Verb::ArcFilletArc,
                ),
                DynTip::DirectedIncoming(p0) => do_arrival(
                    do_fused_directed(
                        p0,
                        spec,
                        radius,
                        TipState::DirectedIncoming,
                        Verb::ArcFilletArc,
                    )?,
                    spec2,
                    TipState::DirectedIncoming,
                    Verb::ArcFilletArc,
                ),
            }
        }
    }

    #[doc = " `.to(anchor)` — the far-end anchor: end the arrival side there."]
    verb FarEndTo(Point2<T>) bind (anchor) rows {
        row {
            /// **The far-end anchor** (G1 constructor 4, the W5 wall): binds an
            /// arrival side's position bit to `anchor` AND ends the side there —
            /// the `to`-family's combined step, read on the arrival side.
            ///
            /// PATHS-DESIGN §3 already says every side is anchored by a real
            /// on-path point plus a direction, and `.angle(θ).at(p)` binds
            /// exactly that pair. What was missing was only the ability for the
            /// side to STOP at its anchor: `.at(p)` leaves the tip Directed at
            /// `p`, and the only continuations run PAST it, so a side whose
            /// natural end is its far vertex had to be authored as a synthetic
            /// mid-side anchor plus a length — a point that is not a vertex, and
            /// a number nobody measured. `.to(p)` says the natural thing: this
            /// side ends at `p`.
            ///
            /// It adds no geometry and no new determination — `.angle(θ).to(p)`
            /// fixes exactly what `.angle(θ).at(p)` fixes (the arrival carrier
            /// is the line through `p` in direction θ; the corner is still the
            /// carrier intersection, never authored). The difference is where
            /// the leg terminates, so the fillet resolution, its corner gates,
            /// and the anchor-fit checks are all `.at(p)`'s, unchanged; `p` is
            /// on the final path either way, authored once. The result is a
            /// directed point (incoming tangent θ), so the next verb's junction
            /// check runs exactly as after any leg.
            ///
            /// The direction must be bound FIRST (`.angle(θ).to(p)` /
            /// `.toward(dx, dy).to(p)`): with the anchor as the terminus, the
            /// side's carrier is what the director supplies.
            ///
            /// **Exact trim fit** — the fillet arc reaching `anchor` with no
            /// straight run left — is not an error. The side simply IS the arc:
            /// no degenerate segment is emitted, the tip carries the arc as its
            /// incoming carrier, the arc's outgoing joint is left UNDECLARED
            /// (the side ends here, so the next direction is free — declaring
            /// would be a claim, not a construction), and the authored anchor is
            /// ABSORBED into the tangent point the fit gate just classified as
            /// coincident with it, rather than emitted as a second vertex a
            /// hair away. That absorption is the hand door's behaviour too, and
            /// keeping it is what keeps the two doors emitting identical bits.
            ///
            /// At the ENTRY (direction bound, no fillet open) there is no
            /// arrival side to end, and this refuses
            /// [`PathError::FarEndAnchorWithoutFillet`] — the entry authors its
            /// first side with `.at(p)`, and the seam is authored at the back
            /// (§2's entry rule). Targeting [`Start`] with the far-end form is
            /// deliberately NOT in this surface; see the module docs.
            on [T: Decide] PartialPath<T, NoPos, HasAng>;
            fn to [(
                mut self,
                anchor: Point2<T>,
            ) -> Result<PartialPath<T, HasPos<WithIncoming>, NoAng>, PathError<T>>] {
                self.core.record(Step::FarEndTo(anchor));
                self.end_side_at(anchor)
            }
            arms {
                DynTip::Angle(p0) => Ok(Applied::Tip(DynTip::DirectedPoint(p0.to(anchor)?))),
            }
        }
    }

    #[doc = " `.to(Start)` — the seam-fillet close (entry vertex retrimmed)."]
    verb CloseTo bind {} rows {
        row {
            /// The combined binder consuming a directed-point VALUE
            /// (`Open → Directed` in one step). [`Start`] is its canonical
            /// argument, and using it is closing: `.angle(θ).fillet(r)
            /// .to(Start)` is the seam fillet — both carriers bound, nothing
            /// pending, loop closed. (Curve-pose arguments — `c.start()` /
            /// `c.end()` — arrive with NURBS legs in v2; see the module docs.)
            on [T: Decide] PartialPath<T, NoPos, NoAng>;
            fn to [(mut self, target: Start) -> Result<ClosedLoop<T>, PathError<T>>] {
                let Start = target;
                self.core.record(Step::CloseTo);
                self.close_at_seam()
            }
            arms {
                DynTip::Open(p0) => Ok(Applied::Closed(p0.to(Start)?.loop_)),
            }
        }
    }
    #[doc = " `circle(centre, r)` — the one-step complete-loop program form."]
    verb Circle {
        #[doc = " The circle's centre."]
        centre: Point2<T>,
        #[doc = " The circle's radius (definitely positive)."]
        radius: T,
    } bind { centre, radius } rows {
        free {
            /// The circle primitive (G1 constructor 1): a **one-step complete-loop
            /// program form**, not a chain — `circle(center, r)` IS the whole loop,
            /// so it returns the lowered [`ProfileLoop`] directly and there is
            /// nothing to continue, close, or bind.
            ///
            /// **It authors no seam.** That is the whole point, and it is what
            /// keeps PQ4 (PATHS-DESIGN §6: a chain's seam sits at a junction or
            /// fillet, never mid-carrier) untouched: a chain still cannot close
            /// mid-carrier, because the split this primitive uses is not authored
            /// at all. The conventional split — two semicircles at the ±x poles,
            /// counterclockwise — is the primitive's PRIVATE lowering, exactly the
            /// M2 closed-carrier precedent: a detail of how a closed carrier
            /// reaches a vertex+bulge document, not a junction anyone said. The two
            /// joints are same-carrier identities, so nothing is declared tangent
            /// (there is no tangency to declare — it is one circle).
            ///
            /// `radius` must classify definitely positive
            /// ([`PathError::NonpositiveCircleRadius`]), through the same funnel as
            /// the other sign gates. A circle is one loop among others: profiles
            /// mix circle loops and chain loops freely (per-loop wholesale, which
            /// is the mixed-authoring rule of §6 read at loop granularity).
            fn circle [<T: Decide>(
                center: Point2<T>,
                radius: T,
            ) -> Result<ClosedLoop<T>, PathError<T>>] {
                Ok(ClosedLoop {
                    loop_: super::circle_kernel(center, radius)?,
                    program: vec![Step::Circle {
                        centre: center,
                        radius,
                    }],
                })
            }
            arms {
                DynTip::Entry => Ok(Applied::Closed(circle(centre, radius)?.loop_)),
            }
        }
    }

    #[doc = " `circle_split(centre, r, n, phase)` — the declared-subdivision closed carrier."]
    verb CircleSplit {
        #[doc = " The carrier's centre."]
        centre: Point2<T>,
        #[doc = " The carrier's radius (definitely positive)."]
        radius: T,
        #[doc = " The subdivision count (STRUCTURAL, ≥ 2)."]
        n: usize,
        #[doc = " The first vertex's angle from +x (continuous)."]
        phase: T,
    } bind { centre, radius, n, phase } rows {
        free {
            /// The declared-subdivision closed carrier (LIB-SWITCH §0 corpus
            /// ruling): one circle, authored WITH its seam structure — `n` arcs of
            /// equal sweep, the first vertex at angle `phase` from the +x axis,
            /// counterclockwise. Like [`circle`] it is a **one-step complete-loop
            /// program form**, not a chain, so PQ4 is untouched: the vertices are
            /// STRUCTURAL subdivisions of one carrier (same-carrier identities,
            /// nothing declared tangent), not junctions anyone claimed — the
            /// difference from [`circle`] is only that here the subdivision COUNT
            /// and PHASE are authored data rather than a private lowering detail,
            /// for the loops whose downstream naming depends on the seam count
            /// (the boss corpus document is the recorded use case).
            ///
            /// Numerics, stated plainly: vertex `k` sits at
            /// `center + radius·(cos θ_k, sin θ_k)`, `θ_k = phase + k·2π/n`, and
            /// every bulge is `tan(π/(2n))` — all through the scalar's libm-pure
            /// trig (D9-deterministic; no exactness promise at axis crossings, the
            /// same posture as `.angle(θ)` directors).
            ///
            /// `radius` must classify definitely positive (the [`circle`] gate,
            /// same funnel row); `n` must be ≥ 2 ([`PathError::CircleSplitCount`]
            /// — a one-vertex full turn has no bulge representation). `n` is
            /// structural (a count, never a value); `phase` is continuous.
            fn circle_split [<T: Decide>(
                center: Point2<T>,
                radius: T,
                n: usize,
                phase: T,
            ) -> Result<ClosedLoop<T>, PathError<T>>] {
                Ok(ClosedLoop {
                    loop_: super::circle_split_kernel(center, radius, n, phase)?,
                    program: vec![Step::CircleSplit {
                        centre: center,
                        radius,
                        n,
                        phase,
                    }],
                })
            }
            arms {
                DynTip::Entry => Ok(Applied::Closed(
                    circle_split(centre, radius, n, phase)?.loop_,
                )),
            }
        }
    }
}

/// A closing verb's result: the lowered loop AND the program that
/// produced it.
///
/// Closing verbs return this pair so that one chain yields both
/// consumers' values
/// (PROFILES-V2 §V1: "one authoring surface, two consumers"). Kernel-
/// direct call sites that only want the geometry adapt with
/// [`From`]/`.into()` or by reading [`ClosedLoop::loop_`].
#[derive(Clone, Debug)]
pub struct ClosedLoop<T: Real> {
    /// The lowered loop.
    pub loop_: ProfileLoop<T>,
    /// The recorded program: replaying it reproduces `loop_` exactly.
    pub program: Vec<Step<T>>,
}

impl<T: Real> From<ClosedLoop<T>> for ProfileLoop<T> {
    fn from(closed: ClosedLoop<T>) -> Self {
        closed.loop_
    }
}

// ------------------------------------------------------------------
// Replay refusals
// ------------------------------------------------------------------

/// Which lattice state the driver's tip was in — the `state` half of a
/// [`ReplayErrorKind::Transition`], and the vocabulary the driver's
/// match is written over: the four lattice states (with the marker
/// flavors the surface distinguishes), the §2c arc-arrival states, and
/// the two ends of a run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TipState {
    /// Before the first verb — the `Open` value itself.
    Entry,
    /// `Open` = {}: a fillet's freshly opened LINE arrival side.
    Open,
    /// `Angle` = {angle}: direction bound, position pending.
    Angle,
    /// `Point` = {position}, plain flavor: no incoming carrier.
    PlainPoint,
    /// `Point` = {position}, directed flavor: a leg end, incoming
    /// tangent available.
    DirectedPoint,
    /// `Directed` = {both}, over a plain position.
    DirectedPlain,
    /// `Directed` = {both}, over a leg-end position.
    DirectedIncoming,
    /// **§2c**: a `Radius` arrival awaiting both binders.
    RadiusArrival,
    /// **§2c**: a `Radius` arrival with its anchor bound.
    RadiusArrivalAt,
    /// **§2c**: a `Radius` arrival with its director bound.
    RadiusArrivalDir,
    /// **§2c**: a `Via` arrival awaiting its director.
    ViaArrival,
    /// **§2c**: a `Via` CLOSE awaiting its director.
    ViaArrivalStart,
    /// The loop is closed; no verb may follow.
    Closed,
}

/// Why a replay refused, and at which step.
///
/// The `step` index is into the program slice; for a program that ENDS
/// without closing it is the length (one past the last step), and the
/// kind's `verb` is then `None`.
#[derive(Clone, Debug)]
pub struct ReplayError<T: Real> {
    /// The index of the offending step.
    pub step: usize,
    /// The refusal class.
    pub kind: ReplayErrorKind<T>,
}

/// The two classes of replay refusal, deliberately different
/// (PROFILES-V2 §V1).
#[derive(Clone, Debug)]
pub enum ReplayErrorKind<T: Real> {
    /// **Lattice violation** — the step's verb (or its spec MODE, for
    /// the ArcData-carrying steps: an inadmissible (state, mode) pair
    /// is unrepresentable at the typed surface, so at the wire it is
    /// this class) is not well-typed at the tip's state. No authoring
    /// surface can produce this: it is the corrupt-or-hand-edited-file
    /// class, refused typed at the door.
    ///
    /// `verb` is `None` when the program simply ENDED in `state`
    /// without a closing verb.
    Transition {
        /// The tip's lattice state.
        state: TipState,
        /// The verb that is ill-typed there, or `None` for
        /// end-of-program.
        verb: Option<Verb>,
    },
    /// **Geometry refusal** — the chain is well-typed but the geometry
    /// refuses. This class CAN exist at rest: once arguments carry
    /// expressions, whether a program elaborates depends on the
    /// parameter binding.
    Path(PathError<T>),
}

impl<T: Real> ReplayError<T> {
    fn transition(step: usize, state: TipState, verb: Verb) -> Self {
        Self {
            step,
            kind: ReplayErrorKind::Transition {
                state,
                verb: Some(verb),
            },
        }
    }
}

impl<T: Real> core::fmt::Display for ReplayError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            ReplayErrorKind::Transition {
                state,
                verb: Some(verb),
            } => write!(
                f,
                "step {}: {verb:?} is not a legal continuation of a {state:?} tip \
                 (lattice violation — no authoring surface can produce this program)",
                self.step
            ),
            ReplayErrorKind::Transition { state, verb: None } => write!(
                f,
                "step {}: the program ends at a {state:?} tip without closing the loop \
                 (lattice violation — a chain must end at Start)",
                self.step
            ),
            ReplayErrorKind::Path(source) => {
                write!(f, "step {}: {source}", self.step)
            }
        }
    }
}

impl<T: Real> std::error::Error for ReplayError<T> {}

// ------------------------------------------------------------------
// The replay driver
// ------------------------------------------------------------------

impl<T: Real> From<PathError<T>> for ReplayErrorKind<T> {
    fn from(source: PathError<T>) -> Self {
        ReplayErrorKind::Path(source)
    }
}

/// The in-flight tip: one variant per lattice state, each carrying the
/// TYPED value for that state — a match arm on (variant, verb) can only
/// call a binder that is well-typed at that state, so an illegal
/// transition cannot be spelled and no binder body is duplicated.
enum DynTip<T: Real> {
    Entry,
    Open(PartialPath<T, NoPos, NoAng>),
    Angle(PartialPath<T, NoPos, HasAng>),
    PlainPoint(PartialPath<T, HasPos<Plain>, NoAng>),
    DirectedPoint(PartialPath<T, HasPos<WithIncoming>, NoAng>),
    DirectedPlain(PartialPath<T, HasPos<Plain>, HasAng>),
    DirectedIncoming(PartialPath<T, HasPos<WithIncoming>, HasAng>),
    RadiusArrival(super::family::RadiusArrival<T>),
    RadiusArrivalAt(super::family::RadiusArrivalAt<T>),
    RadiusArrivalDir(super::family::RadiusArrivalDir<T>),
    ViaArrival(super::family::ViaArrival<T>),
    ViaArrivalStart(super::family::ViaArrivalStart<T>),
}

impl<T: Real> DynTip<T> {
    fn state(&self) -> TipState {
        match self {
            DynTip::Entry => TipState::Entry,
            DynTip::Open(_) => TipState::Open,
            DynTip::Angle(_) => TipState::Angle,
            DynTip::PlainPoint(_) => TipState::PlainPoint,
            DynTip::DirectedPoint(_) => TipState::DirectedPoint,
            DynTip::DirectedPlain(_) => TipState::DirectedPlain,
            DynTip::DirectedIncoming(_) => TipState::DirectedIncoming,
            DynTip::RadiusArrival(_) => TipState::RadiusArrival,
            DynTip::RadiusArrivalAt(_) => TipState::RadiusArrivalAt,
            DynTip::RadiusArrivalDir(_) => TipState::RadiusArrivalDir,
            DynTip::ViaArrival(_) => TipState::ViaArrival,
            DynTip::ViaArrivalStart(_) => TipState::ViaArrivalStart,
        }
    }
}

/// What applying one step produced.
enum Applied<T: Real> {
    /// The chain continues at this tip.
    Tip(DynTip<T>),
    /// The step closed the loop.
    Closed(ProfileLoop<T>),
}

type Applying<T> = Result<Applied<T>, ReplayErrorKind<T>>;

fn violation<T: Real>(state: TipState, verb: Verb) -> Applying<T> {
    Err(ReplayErrorKind::Transition {
        state,
        verb: Some(verb),
    })
}

// The flavor-generic target dispatchers. Each branch names exactly ONE
// binder — the one well-typed at that (state, verb, target/mode).

fn do_line_to<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    t: Target<T>,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(p.line_to(q)?))),
        Target::Start => Ok(Applied::Closed(p.line_to(Start)?.loop_)),
    }
}

/// The sharp arc leg's mode dispatch: the endpoint-full modes from a
/// Point tip — one row per admissible (state, mode) pair of the §2c
/// matrix, each calling the one typed `arc_to(spec)` binder.
fn do_arc_to_point<T: ArcCarrierScalar, F: Flavor>(
    p: PartialPath<T, HasPos<F>, NoAng>,
    spec: ArcData<T>,
    state: TipState,
) -> Applying<T> {
    match spec {
        ArcData::Bulge {
            target: Target::Point(q),
            b,
        } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(Bulge { p: q, b })?,
        ))),
        ArcData::Bulge {
            target: Target::Start,
            b,
        } => Ok(Applied::Closed(p.arc_to(Bulge { p: Start, b })?.loop_)),
        ArcData::Via {
            q,
            target: Target::Point(t),
        } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(Via { q, p: t })?,
        ))),
        ArcData::Via {
            q,
            target: Target::Start,
        } => Ok(Applied::Closed(p.arc_to(Via { q, p: Start })?.loop_)),
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(Applied::Tip(DynTip::DirectedPoint(p.arc_to(Center {
            c,
            winding,
            p: t,
        })?))),
        ArcData::Center {
            c,
            winding,
            target: Target::Start,
        } => Ok(Applied::Closed(
            p.arc_to(Center {
                c,
                winding,
                p: Start,
            })?
            .loop_,
        )),
        ArcData::Radius { .. } | ArcData::Sweep { .. } | ArcData::ArcLen { .. } => {
            violation(state, Verb::ArcTo)
        }
    }
}

/// The endpoint-free sharp legs from a Directed tip.
fn do_arc_to_directed<T: ArcCarrierScalar, F: Flavor>(
    p: PartialPath<T, HasPos<F>, HasAng>,
    spec: ArcData<T>,
    state: TipState,
) -> Applying<T> {
    match spec {
        ArcData::Sweep { r, side, angle } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(super::Sweep { r, side, angle })?,
        ))),
        ArcData::ArcLen { r, side, len } => Ok(Applied::Tip(DynTip::DirectedPoint(
            p.arc_to(super::ArcLen { r, side, len })?,
        ))),
        // Spelled out rather than `_`: a mode the table gains must be
        // ADJUDICATED at every dispatcher, not silently refused here.
        ArcData::Radius { .. }
        | ArcData::Bulge { .. }
        | ArcData::Via { .. }
        | ArcData::Center { .. } => violation(state, Verb::ArcTo),
    }
}

fn do_tangent_arc_to<T: Decide, F: Flavor>(
    p: PartialPath<T, HasPos<F>, HasAng>,
    t: Target<T>,
) -> Applying<T> {
    match t {
        Target::Point(q) => Ok(Applied::Tip(DynTip::DirectedPoint(p.tangent_arc_to(q)?))),
        Target::Start => Ok(Applied::Closed(p.tangent_arc_to(Start)?.loop_)),
    }
}

/// Applies an ARC-ARRIVAL spec to an opened fillet — the shared second
/// half of the `FilletArc` / `ArcFilletArc` rows, dispatching each mode
/// to its own `ArrivalSpec` impl (the typed surface's exact code).
fn do_arrival<T: ArcCarrierScalar>(
    open: PartialPath<T, NoPos, NoAng>,
    spec: ArcData<T>,
    state: TipState,
    verb: Verb,
) -> Applying<T> {
    use super::family::ArrivalSpec;
    let core = open.core;
    match spec {
        ArcData::Center {
            c,
            winding,
            target: Target::Point(p),
        } => Ok(Applied::Tip(DynTip::DirectedPoint(ArrivalSpec::apply(
            core,
            super::Center { c, winding, p },
        )?))),
        ArcData::Center {
            c,
            winding,
            target: Target::Start,
        } => Ok(Applied::Closed(
            ArrivalSpec::apply(
                core,
                super::Center {
                    c,
                    winding,
                    p: Start,
                },
            )?
            .loop_,
        )),
        ArcData::Radius { r, side } => Ok(Applied::Tip(DynTip::RadiusArrival(ArrivalSpec::apply(
            core,
            super::Radius { r, side },
        )?))),
        ArcData::Via {
            q,
            target: Target::Point(p),
        } => Ok(Applied::Tip(DynTip::ViaArrival(ArrivalSpec::apply(
            core,
            super::Via { q, p },
        )?))),
        ArcData::Via {
            q,
            target: Target::Start,
        } => Ok(Applied::Tip(DynTip::ViaArrivalStart(ArrivalSpec::apply(
            core,
            super::Via { q, p: Start },
        )?))),
        ArcData::Bulge { .. } | ArcData::Sweep { .. } | ArcData::ArcLen { .. } => {
            violation(state, verb)
        }
    }
}

/// The fused incoming from a PLAIN point tip (the endpoint-full modes).
fn do_fused_point<T: ArcCarrierScalar>(
    p: PartialPath<T, HasPos<Plain>, NoAng>,
    spec: ArcData<T>,
    radius: T,
    state: TipState,
    verb: Verb,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Bulge {
            target: Target::Point(q),
            b,
        } => Ok(p.arc_fillet(super::Bulge { p: q, b }, radius)?),
        ArcData::Via {
            q,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Via { q, p: t }, radius)?),
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Center { c, winding, p: t }, radius)?),
        ArcData::Bulge {
            target: Target::Start,
            ..
        }
        | ArcData::Via {
            target: Target::Start,
            ..
        }
        | ArcData::Center {
            target: Target::Start,
            ..
        }
        | ArcData::Radius { .. }
        | ArcData::Sweep { .. }
        | ArcData::ArcLen { .. } => Err(ReplayErrorKind::Transition {
            state,
            verb: Some(verb),
        }),
    }
}

/// The fused incoming from a DIRECTED POINT (leg end): the endpoint-full
/// modes plus `Radius` — arc extension (§2c dissolution).
fn do_fused_leg_end<T: ArcCarrierScalar>(
    p: PartialPath<T, HasPos<WithIncoming>, NoAng>,
    spec: ArcData<T>,
    radius: T,
    state: TipState,
    verb: Verb,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Bulge {
            target: Target::Point(q),
            b,
        } => Ok(p.arc_fillet(super::Bulge { p: q, b }, radius)?),
        ArcData::Via {
            q,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Via { q, p: t }, radius)?),
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(p.arc_fillet(super::Center { c, winding, p: t }, radius)?),
        ArcData::Radius { r, side } => Ok(p.arc_fillet(super::Radius { r, side }, radius)?),
        ArcData::Bulge {
            target: Target::Start,
            ..
        }
        | ArcData::Via {
            target: Target::Start,
            ..
        }
        | ArcData::Center {
            target: Target::Start,
            ..
        }
        | ArcData::Sweep { .. }
        | ArcData::ArcLen { .. } => Err(ReplayErrorKind::Transition {
            state,
            verb: Some(verb),
        }),
    }
}

/// The fused incoming from a DIRECTED tip (the endpoint-free modes).
fn do_fused_directed<T: ArcCarrierScalar, F: Flavor>(
    p: PartialPath<T, HasPos<F>, HasAng>,
    spec: ArcData<T>,
    radius: T,
    state: TipState,
    verb: Verb,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Sweep { r, side, angle } => {
            Ok(p.arc_fillet(super::Sweep { r, side, angle }, radius)?)
        }
        ArcData::ArcLen { r, side, len } => {
            Ok(p.arc_fillet(super::ArcLen { r, side, len }, radius)?)
        }
        ArcData::Radius { .. }
        | ArcData::Bulge { .. }
        | ArcData::Via { .. }
        | ArcData::Center { .. } => Err(ReplayErrorKind::Transition {
            state,
            verb: Some(verb),
        }),
    }
}

/// The fused incoming at the ENTRY (`Center` alone can seed).
fn do_fused_entry<T: ArcCarrierScalar>(
    spec: ArcData<T>,
    radius: T,
) -> Result<PartialPath<T, NoPos, NoAng>, ReplayErrorKind<T>> {
    match spec {
        ArcData::Center {
            c,
            winding,
            target: Target::Point(t),
        } => Ok(Open.arc_fillet(super::Center { c, winding, p: t }, radius)?),
        ArcData::Center {
            target: Target::Start,
            ..
        }
        | ArcData::Radius { .. }
        | ArcData::Bulge { .. }
        | ArcData::Via { .. }
        | ArcData::Sweep { .. }
        | ArcData::ArcLen { .. } => Err(ReplayErrorKind::Transition {
            state: TipState::Entry,
            verb: Some(Verb::ArcFillet),
        }),
    }
}

/// **The replay driver**: elaborates a recorded program into the loop it
/// describes, through the typed binders and every check they carry —
/// the only path from steps to geometry, and the dynamic mirror of the
/// typestate lattice (every transition the markers forbid statically is
/// a typed [`ReplayErrorKind::Transition`] here).
///
/// A chain program must end in a `Start`-targeting verb; a
/// [`Step::Circle`] program is exactly one step. **The program replay
/// re-records is DISCARDED** — the input program is the authority, and
/// the round-trip is pinned rather than trusted: bit-identity by the
/// blanket differential every closing verb funnels through
/// (`common::pinned`), and per-verb reachability by the
/// replay-coverage census (see the module docs).
///
/// # Errors
///
/// [`ReplayError`], carrying the offending step index.
pub fn replay<T: ArcCarrierScalar>(steps: &[Step<T>]) -> Result<ProfileLoop<T>, ReplayError<T>> {
    let mut tip = DynTip::Entry;
    for (i, step) in steps.iter().enumerate() {
        let applied = apply(tip, *step).map_err(|kind| ReplayError { step: i, kind })?;
        match applied {
            Applied::Tip(next) => tip = next,
            Applied::Closed(lowered) => {
                return match steps.get(i + 1) {
                    Some(extra) => Err(ReplayError::transition(
                        i + 1,
                        TipState::Closed,
                        extra.verb(),
                    )),
                    None => Ok(lowered),
                };
            }
        }
    }
    Err(ReplayError {
        step: steps.len(),
        kind: ReplayErrorKind::Transition {
            state: tip.state(),
            verb: None,
        },
    })
}
