//! The sizing vocabulary: one home for "how fine should this be".
//!
//! # Three quantities, one word each — and one that is not a quantity
//!
//! Every sizing question in this crate is one of three things, and
//! this module fixes the word for each so a reader never has to open a
//! function to learn which kind it answers. The fourth item below is
//! not one of them and does not join the rule: [`Eps`] is the run's ε
//! carried as a BAND, it sizes nothing, and it lives here because
//! [`SizingTols`] carries it beside δ. Its own vocabulary
//! rule is a different one — named operations, stated at the type.
//!
//! - a **target** — the metres of deviation a schedule aims at. It is
//!   δ itself ([`SizingTols::delta`]), spent through the lane's own
//!   worst-triangle theorem (`crates/mesh/README.md` `[mesh.sizing]`).
//!   The edge chords, the cylinder, cone, sphere and torus lanes take
//!   the coarsest step whose worst admissible triangle lands at δ. Two
//!   lanes do not land there: the NURBS lane spends its
//!   `nurbs_cert::LATTICE_SHARE` of δ on its lattice (its sliver lands
//!   at δ, its lattice cells at an eighth of it), and the trimmed
//!   cylinder and trimmed sphere, which have no theorem, size at
//!   `trimmed::UNPROVED_HEADROOM` of δ.
//! - a **step** — an increment in a *parameter* (a curve parameter, a
//!   chart angle, a UV coordinate), always `f64`. Steps come from a
//!   closed-form deviation bound: [`sagitta_step`] and [`ellipse_step`]
//!   here, [`curvature_step`] for a second-derivative bound,
//!   [`torus_grid_steps`] for the torus chart, and
//!   [`crate::nurbs_cert::NurbsFaceBound::grid_steps`] for a certified
//!   NURBS patch. The cap on an angular one is [`MAX_ANGULAR_STEP`];
//!   this module is the only place it is applied ([`cap_angular`], and
//!   [`sagitta_step`]'s vacuous branch, which returns it directly).
//! - a **count** — how many chords or grid divisions a span is cut
//!   into, always `usize`. Every count in the crate is
//!   [`ceil_count`]`(span, step)` or a rule applied to one
//!   (`curved::pole_columns` lifts the single singular value
//!   `nu == 2` — an equality, not a floor, and its own doc says why).
//!
//! **The rule, in one sentence:** *"step" names an `f64` increment and
//! nothing else; a `usize` count is never called a step, and neither is
//! a sample count.* A new rule that produces one of the three
//! quantities says which one by its name, or it is misnamed. It
//! binds the PROSE as well as
//! the identifiers — a rename that leaves a doc sentence calling a
//! count a step has not landed.
//!
//! **Scope, and what enforces it.** The rule is this crate's. No
//! violation of it is known outside the crate: the three integer
//! constants elsewhere whose names carry "steps" — `bvh`'s
//! `ITEM_WIDEN_STEPS` and `HULL_WIDEN_STEPS`, and `geom-brep`'s
//! `SSI_MAX_STEPS` — each count a step actually taken (ULP widenings,
//! marching iterations) rather than a sampling density, which is the
//! shape this rule forbids. No sweep has read the whole tree for that
//! shape, so read it as no KNOWN violation, not a checked absence.
//! Nothing mechanical enforces either half today — the
//! guard this wants is a source-scraping row in the shape of this
//! crate's own `tests/all.rs`, which `include_str!`s its own source to
//! prove every test file is registered — and, since the ε ledger,
//! walks `crates/mesh/src` through `test_utils::source::code_only`
//! (`the_eps_inventory_is_pinned`). **The shape exists; nobody has
//! written this rule's row in it.** Until someone does, the rule is
//! held by review, which is a weaker thing than the sentence above
//! sounds.
//!
//! The one deliberate second spelling is `tess_meter::divisions`, in
//! the consumer half of the budget meter: a different cargo root, so
//! it cannot share this import, and it answers in `f64` and refuses
//! nothing because it sizes nothing. Its own docs state the
//! divergences from [`ceil_count`]; the different word is the tell
//! that it is a different function.
//!
//! # Where the policy is
//!
//! The policy is `crates/mesh/README.md` `[mesh.sizing]`: each lane
//! sizes at the coarsest step its own theorem grants, with no safety
//! factor. Each theorem is stated at its step function and pinned by
//! a test beside it — [`sagitta_step`] and [`curvature_step`] for an
//! edge chord and the curved cylinder, [`cone_grid_step`],
//! [`sphere_grid_steps`], [`torus_grid_steps`], and
//! `nurbs_cert::LATTICE_SHARE` for the NURBS lane's sliver, whose share
//! of δ is a typed choice nothing has measured as the coarsest
//! (`work/tess/the-nurbs-lattice-share-is-not-chosen-by-measurement.md`).
//! The lanes without a theorem say so at their call sites: the trimmed
//! cylinder and sphere size at `trimmed::UNPROVED_HEADROOM` of δ, and a cone face
//! whose side carries a point keeps the lattice `curved::grid_counts`
//! names.
//!
//! **One sizing question has a separate ratified answer**:
//! `docs/TESS-BUDGET.md`'s *split schedule's aspect policy*
//! (2026-08-16, PR #568) rules the NURBS split schedule's 3-D aspect
//! cap at A = 16, scoped to `nurbs_cert`'s per-cell step derivation.

use crate::types::TessellateError;
use geom_core::{Band, Tol};

/// **The kernel ε, carried as a band with four named operations —
/// this crate's whole ε vocabulary.**
///
/// The band is a length in metres, in a private field, and the four
/// methods below are the only operations on it — so **a `mesh` read of
/// ε through this type is one of the four or it does not compile**,
/// and the crate's ε inventory IS those methods, which is what
/// `mesh/tests/all.rs`'s `the_eps_inventory_is_pinned` counts.
///
/// **The bound of that claim.** ε also reaches `mesh` as
/// [`SizingTols::band`] — props' `Band`, ε AND K, minted from the same
/// `Tol` — and leaves it unread: `mesh` hands it to
/// `geom_brep::props::require_iso_rectangle` and
/// `require_one_chart_branch`, and decides nothing against it. The
/// inventory pin counts `eps` identifier carriers and the four method
/// reads per file; it does not count `band`, `Band` or K, so a
/// decision written against the band in this crate would be a bypass
/// the pin cannot see. None exists; the field's doc is the obligation.
///
/// **The scalar is not sealed in, and the honest bound is narrower
/// than "no way back".** [`Display`](core::fmt::Display) emits a
/// round-trippable decimal, so `eps.to_string().parse::<f64>()`
/// recovers the band; a `Debug` derive would do the same, which is
/// why this type carries none. What actually holds is the second
/// mechanism, not the first: **every such spelling has to NAME the
/// band at its call site**, which moves that file's carrier count in
/// the inventory pin. The type makes the bare read inconvenient and
/// unnatural; the pin makes it visible. Neither alone is the claim.
///
/// # The four operations, and their band edges
///
/// Written as edges rather than as prose, because the edge is the only
/// thing that distinguishes three of them:
///
/// | operation | true when | edge |
/// |---|---|---|
/// | [`separates`](Eps::separates)`(x)` | `x > band` | band EXCLUDED |
/// | [`coincident`](Eps::coincident)`(x)` | `x <= band` | band INCLUDED |
/// | [`dominates`](Eps::dominates)`(x)` | `x < band` | band EXCLUDED |
/// | [`pad`](Eps::pad)`(b)` | — | `b + band`, widening UP |
///
/// **`coincident` and `dominates` differ by one character of code and
/// carry the difference in a NOUN, which is a real cost of these
/// names and is recorded rather than hidden.** Nothing in either word
/// says "≤" or "<"; a reader who wants the edge reads the table or
/// the method, not the call site. The alternative considered was
/// spelling inclusivity into the names (`within` / `under`, or
/// `at_most` / `below`) — rejected because the four names are the
/// issue's own vocabulary and renaming them here would make this
/// unit's diff a naming argument on top of a port whose gate is that
/// nothing moves. **A reader at a call site who needs the edge must
/// look it up**; that is the trade, and the type rows in this
/// module's tests are what keep the table true.
///
/// **Each operation takes ONE argument, not the two the issue's
/// sketch wrote.** The band is `self` and the length is the argument,
/// because the five predicate call sites compute their length three
/// different ways — a norm, a chart radial, a product — and a
/// two-argument `separates(a, b)` would have had to pick one
/// subtraction and impose it on all of them. That is a moved byte, and
/// this unit's gate forbids it.
///
/// **`separates` and `coincident` are exact negations on ordered
/// input, and neither is written as the other's `!`.** On a NaN both
/// are FALSE — a poisoned length is neither separated nor coincident —
/// and that asymmetry is the reason: `!coincident(NaN)` is `true`,
/// which would admit a poisoned length as separated at
/// `walk::iso_side_starts`, where today it does not. Each
/// method is the bare comparison its caller used to spell, so the
/// NaN behaviour is carried through unchanged rather than re-derived.
///
/// `dominates` differs from `coincident` ONLY at the edge, and the
/// difference is load-bearing at exactly one place: [`crate::walk`]'s
/// band predicate is a strict `<` so that a zero band admits nothing,
/// which is what makes `gap_is_noise(gap, lever, 0)` the exact form
/// its own fixtures compare against.
///
/// # The seam, and where this type goes when #741 lands
///
/// The band is minted once, at the [`Tol`] seam ([`Eps::at`]) — the
/// only place in this crate that reads `Tol::eps()`. Issue 741 is
/// ε's CONFIGURATION surface and may give `geom-core` an ε type of
/// its own; the operations named here are a `mesh`-local fact about
/// `mesh`'s terminal reads, so they are minted here rather than grown
/// on contended cross-crate ground. **If #741's surface later carries
/// these same four operations, this newtype collapses onto it**: the
/// callers keep their spellings, `Eps` becomes a re-export or is
/// deleted, and the seam that has to move is [`Eps::at`] alone.
///
/// # Why it lives in `sizing`, where nothing reads it
///
/// Every consumer is in [`crate::walk`], [`crate::curved`],
/// [`crate::trimmed`] and [`mod@crate::tessellate`]; this module reads ε
/// in none of its own rules and says so at [`SizingTols`]. It is here
/// anyway because **the band arrives as part of the call's tolerance
/// bundle** — `SizingTols` is the struct that carries δ and ε
/// together, this is where that struct is defined, and a type whose
/// whole job is to be one field of it belongs beside it. The
/// alternative, a module of its own for one newtype and four methods,
/// buys a file and no invariant.
///
/// **Only `Clone` and `Copy` are derived**, and they are load-bearing:
/// the band is threaded by value through four signatures. `PartialEq`
/// and `PartialOrd` were derived at first and are gone — no consumer
/// used either, and `band_a <= band_b` compiling is an unnamed
/// operation on a type whose entire premise is that its operations are
/// named. `Debug` is absent for the reason the header gives.
#[derive(Clone, Copy)]
pub(crate) struct Eps(f64);

impl Eps {
    /// The run's committed ε, as a band. The crate's ONE read of
    /// `Tol::eps()`, so a second raw read shows up as a moved count in
    /// `the_eps_inventory_is_pinned` rather than as nothing at all.
    pub(crate) fn at(tol: Tol) -> Self {
        Self(tol.eps())
    }

    /// A chosen band, for a row that fixes ε rather than taking the
    /// run's. Test-only on purpose: in shipped code [`Eps::at`] is the
    /// only constructor, so ε cannot enter `mesh` except through the
    /// witness.
    #[cfg(test)]
    pub(crate) fn exactly(band: f64) -> Self {
        Self(band)
    }

    /// **The band SEPARATES `length`**: the length lies outside it,
    /// so the two things it measures are two and not one. Band
    /// excluded; false on a NaN length.
    ///
    /// The caller's decision is a CLASSIFICATION, not a state
    /// disposition: `walk::iso_side_starts` picks which of
    /// two analytically-equal columns an iso side's entries carry, so
    /// no D2-addendum row applies to it (rows 1–5 dispose of a state
    /// that should not exist; this one chooses between two correct
    /// spellings of a coordinate). D2 addendum **row 0** is what this
    /// type answers, for every read on it.
    pub(crate) fn separates(self, length: f64) -> bool {
        length > self.0
    }

    /// **The band finds `length` COINCIDENT**: the length lies inside
    /// it, so the two points it measures are read as one. Band
    /// included; false on a NaN length.
    ///
    /// Its callers' decisions live on **D2 addendum row 5** (kernel
    /// bug, detectable only by re-derivation) — [`crate::walk`]'s
    /// declared-vertex guard and its pole guard are both
    /// `debug_assert`s over every pair — except the pole-membership
    /// find, which is a classification like [`separates`](Eps::separates)
    /// and carries no row: it substitutes a pole's exact `v` and seeds
    /// a fan, and whether an in-band junction REALLY is the pole is
    /// the intent question this project does not ask.
    pub(crate) fn coincident(self, length: f64) -> bool {
        length <= self.0
    }

    /// **The band DOMINATES `scaled`**: a quantity already converted
    /// to metres is small enough beside it to be float noise. Band
    /// excluded, so a zero band dominates nothing; false on a NaN.
    ///
    /// **D2 addendum row 1**, for its two callers, each refusing a
    /// face typed: [`crate::curved`]'s swept-rectangle domain guard,
    /// and [`crate::cert::verdict`], which applies a certificate's
    /// rounding allowance only while the band dominates it (four times
    /// over, `cert::ALLOWANCE_SHARE`). Nothing reads it to measure the
    /// quality of a body's own coordinates: that question is
    /// `topo::coherence`'s, and it carries its own band. So this
    /// operation has no deviation to record.
    pub(crate) fn dominates(self, scaled: f64) -> bool {
        scaled < self.0
    }

    /// **The band PADS `bound`**: `bound` widened by one band,
    /// UPWARD — the conservative direction for a bound that will
    /// divide. The one operation here that answers with a length
    /// rather than a verdict, which is why it reads as a verb on a
    /// value where the three above read as verdicts about one.
    ///
    /// No D2-addendum row: the caller ([`crate::trimmed`]'s deviation
    /// probe) disposes of no state. It publishes a measurement, and
    /// this widening is why f64 evaluation rounding on a wall
    /// certifying at ~5e-17 does not read as a violation.
    pub(crate) fn pad(self, bound: f64) -> f64 {
        bound + self.0
    }
}

impl core::fmt::Display for Eps {
    /// The band's own metres, for an assertion message — which is
    /// what it exists for, and NOT a claim that the value cannot get
    /// out: `f64`'s `Display` round-trips, so this is a door and not a
    /// wall. It is an acceptable one because a caller walking back
    /// through it has to name the band to do so, and the inventory pin
    /// counts that name (see the type's header).
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

/// The call's tolerance bundle: δ (the promise, and every lane's
/// sizing target) and the run's kernel ε — fetched once by [`fn@crate::tessellate`] and
/// carried here, never re-read from `Tolerance`.
///
/// # What ε may and may not do here
///
/// **No sizing function takes it as an argument**: in this module ε
/// appears only on the field below and inside [`Eps::at`], the seam
/// that mints the band, and no step or grid rule in the crate has it
/// in its signature. **A count can still
/// move**, which is why that sentence is about arguments —
/// `curved::pole_columns(nu, has_pole)` returns 3 where it would
/// return 2, and `has_pole` is a bit an ε comparison in
/// [`crate::walk`] sets. ε is upstream of one argument of one rule.
///
/// **No consumer SNAPS a value** (the loop-closure snap that did is
/// gone, S22). What a read does instead — refuse a face, report into a
/// `debug_assert`, classify a vertex, scale a measurement — **is
/// stated at the read**. This doc holds neither a roster of the reads
/// nor a partition of their kinds: the roster is computed (below), and
/// a partition is a count by another name, which is the thing that
/// went stale here.
///
/// A CLASSIFYING read makes *"ε cannot move an emitted coordinate"*
/// FALSE as stated — pole identification substitutes the pole's exact
/// `v` and emits two entries instead of one; `walk::iso_side_starts`
/// picks which of two analytically-equal columns a side carries. Each
/// argues its own reachability where it is; what belongs here is the
/// summary: **nothing in the tree flips one, and whether anything
/// COULD is not established** (a STEP import is the plausible route
/// in). The dependence is structural and UNEXERCISED, which is not the
/// same as absent, and it is why *"mesh structure is a function of
/// (body, δ)"* is a statement about the tree rather than a theorem.
///
/// A read can also be no bar at all: [`crate::trimmed`]'s deviation
/// probe computes `d / (bound + eps)`, so ε moves the `worst_ratio` it
/// publishes at **every** call, monotonically. No mesh coordinate
/// moves; a reported number does.
///
/// **Where the reads are is not written down here** — it is pinned by
/// `mesh/tests/all.rs`'s `the_eps_inventory_is_pinned`, which counts
/// the named operations per file and reds when one lands. A list here
/// could not, and one here was short by a read for two milestones.
/// One place holds ε's bits without reading them: the patch memo's
/// key (`crate::memo`) folds the ambient ε and k as bytes, so a
/// tolerance change misses every memoised face. That is not a
/// decision and not an [`Eps`]; the inventory row names it as such.
/// **What KIND each read is, the type now carries**: every ε read in
/// this crate is one of [`Eps`]'s four operations, so a fifth read
/// picks a name or adds a method rather than spelling its own band
/// arithmetic — the D2-addendum row-0 answer to the class.
pub(crate) struct SizingTols {
    /// The chordal tolerance δ.
    pub delta: f64,
    /// The kernel ε, as a band. No sizing rule takes it; see above for
    /// what its reads may do, [`Eps`] for the four they are spelled
    /// with, and `the_eps_inventory_is_pinned` for where they are.
    pub eps: Eps,
    /// The linear decision band props classifies with, minted from
    /// the same [`Tol`] as `eps` at operation entry (the calling
    /// convention `Band::linear` prescribes). Consumed by exactly one
    /// site — `curved`'s shape door, which hands it to
    /// `geom_brep::props::require_iso_rectangle` and then to
    /// `require_one_chart_branch` — and by no rule of this crate's
    /// own: `mesh` decides nothing against it, it carries props' band
    /// to props' predicates. Not an ε read of this crate, and the
    /// inventory pin does not count it as one.
    pub band: Band,
}

/// The cap on any *angular* step (π/4). **Structural.**
///
/// Two things rest on it, both structural rather than tuning: it keeps
/// the unwrapping-by-continuity of a periodic chart along the boundary
/// walk branch-unambiguous, and it makes a full-period rim
/// polygonalize with at least 8 chords rather than degenerate. It
/// therefore binds only where a step is an angle in a periodic
/// coordinate; a NURBS parameter step is uncapped.
pub(crate) const MAX_ANGULAR_STEP: f64 = core::f64::consts::FRAC_PI_4;

/// Sanity cap on any single count (δ small enough to exceed this would
/// allocate gigabytes before failing anywhere else). **Structural.**
const MAX_COUNT: f64 = 16_777_216.0; // 2^24

/// A parameter step capped at [`MAX_ANGULAR_STEP`]. A non-finite or
/// vacuous step takes the cap.
pub(crate) fn cap_angular(step: f64) -> f64 {
    if step < MAX_ANGULAR_STEP {
        step
    } else {
        MAX_ANGULAR_STEP
    }
}

/// The per-chord angular step for sagitta ≤ `target` on a circle of
/// radius `rho`, capped at [`MAX_ANGULAR_STEP`]. Total (poison-free
/// for positive inputs): if `target ≥ ρ` the sagitta constraint is
/// vacuous and the cap rules.
///
/// # Two theorems, one step
///
/// **An edge chord.** A chord of a circle of radius ρ subtending φ
/// deviates from it by exactly the sagitta `ρ·(1 − cos(φ/2))`, so at
/// `target = δ` the chord's worst point lands at δ
/// (`edge_chord_steps_meet_the_bound_with_equality`).
///
/// **The curved cylinder lane.** A cylinder face (radius r) emits no
/// interior points: its triangles are the CDT of its two rim rows and
/// its straight sides. Every such triangle has two vertices
/// consecutive on one row and its third on the other, and its azimuth
/// extent Δu is at most the larger of the two rows' chord spacings —
/// if the third vertex `x` fell more than one spacing `h` beyond the
/// opposite edge `[a, a + s]` (`s ≤ h`), the circumcircle would cut
/// the far row in `(a + s − (x − a), x)`, an open interval longer
/// than `h` holding `x`'s neighbour, which a Delaunay triangle cannot
/// contain (the argument reads the far row only, so it holds in any
/// axis scaling of the CDT's coordinates). Its certificate
/// `r − dist(axis, T)` is then at most `r·(1 − cos(Δu/2))`: the
/// triangle's projection lies in the circular segment cut by the
/// chord of its extreme azimuths. So the cylinder's worst triangle is
/// a rim chord's, and a rim chorded at `sagitta_step(δ, r)` lands it
/// at δ (`cylinder_rim_triangle_lands_at_delta`).
pub(crate) fn sagitta_step(target: f64, rho: f64) -> f64 {
    if target < rho {
        cap_angular(2.0 * (1.0 - target / rho).acos())
    } else {
        MAX_ANGULAR_STEP
    }
}

/// The parameter step for chord deviation ≤ `target` against a
/// second-derivative bound `m = sup‖C″‖` over the step: the standard
/// C² secant bound `deviation ≤ h²·m/8` inverted.
///
/// Uncapped, because the parameter it steps need not be an angle —
/// callers whose parameter is periodic apply [`cap_angular`].
pub(crate) fn curvature_step(target: f64, m: f64) -> f64 {
    (8.0 * target / m).sqrt()
}

/// The per-chord parameter step for chord deviation ≤ `target` on an
/// ellipse with semi-axes `semi_u` and `semi_v` in either order and sign
/// (the stored semi-axes carry no order, `geom_brep::Conic`), capped at
/// [`MAX_ANGULAR_STEP`]. Below, `major` and `minor` name the larger and
/// the smaller MAGNITUDE.
///
/// Certified-conservative from [`curvature_step`]: over a parameter
/// span φ the arc length is `L ≤ major·φ` (`|dP/dθ| ≤ major`) and the
/// ellipse's maximum curvature is `κ_max = major/minor²` (at the major
/// vertices), so the effective second-derivative bound is
/// `R_eff = major·(major/minor)²`. Coarser than the circle's exact
/// sagitta near `major = minor` — conservative is the promised
/// direction.
pub(crate) fn ellipse_step(target: f64, semi_u: f64, semi_v: f64) -> f64 {
    let (major, minor) = geom::Curve3::semi_axes_by_size(semi_u, semi_v);
    let r_eff = major * (major / minor) * (major / minor);
    cap_angular(curvature_step(target, r_eff))
}

/// The per-chord parameter step for chord deviation ≤ `target` on a
/// spiric `Curve3::Spiric { major_radius: R, minor_radius: r, offset:
/// d, .. }`, capped at [`MAX_ANGULAR_STEP`].
///
/// Certified-conservative from [`curvature_step`] with
/// [`geom::spiric_curvature_sup`], the one spelling of the kind's
/// closed-form `sup|C″|`. Plain `f64` like [`ellipse_step`] — a sizing quantity,
/// conservative by the bound's slack rather than by rounding. An
/// off-regime carrier (`f_min` poison or zero) takes the cap through
/// [`cap_angular`]'s non-finite arm; certification refuses such a
/// carrier before a mesh is asked for.
pub(crate) fn spiric_step(target: f64, major: f64, minor: f64, offset: f64) -> f64 {
    cap_angular(curvature_step(
        target,
        geom::spiric_curvature_sup(major, minor, offset),
    ))
}

/// The cone chart's azimuth grid step for a face whose largest radius
/// is `rho_max`, capped at [`MAX_ANGULAR_STEP`].
///
/// # The theorem
///
/// [`crate::cert::cert_cone`] bounds a triangle by
/// `cos α · ρ_maxᵀ · (1 − cos(Δu/2))`, with `ρ_maxᵀ` its largest
/// radius and Δu its non-apex azimuth extent. The cone is ruled in v,
/// so rows never enter the bound, and the curved lane takes none: its
/// only rows are the face's two rims (or a rim and the apex), both
/// chorded within the column step `h` (the chord pass's tightening,
/// [`crate::chords`]), so the strip argument at [`sagitta_step`]
/// keeps every triangle's Δu within `h`. The worst triangle is then a
/// rim-chord triangle on the largest radius, certifying at
/// `cos α · ρ_max · (1 − cos(h/2))`; this step solves that for δ,
/// which is [`sagitta_step`] on the effective radius `ρ_max·cos α`
/// (`cone_grid_step_lands_the_worst_triangle_at_delta`).
pub(crate) fn cone_grid_step(target: f64, half_angle: f64, rho_max: f64) -> f64 {
    sagitta_step(target, rho_max * half_angle.cos())
}

/// The sphere chart's two grid steps `(h_u, h_v)` — azimuth and
/// latitude — for a face whose latitudes nearest the equator have
/// cosine `widest` ([`sphere_face_widest_cos`]), capped at
/// [`MAX_ANGULAR_STEP`].
///
/// # The theorem
///
/// Let a triangle's vertices lie on the sphere (radius r) inside a UV
/// box of extents `Δu × Δv` within the face, and let `m` be the box's
/// centre point. The haversine law,
/// `hav d = hav Δφ + cos φ₁·cos φ₂·hav Δθ`, puts every vertex within an
/// angle `ψ` of `m`, and both latitudes lie in the face's window, so
/// `cos φ₁·cos φ₂ ≤ c²` with `c = widest`:
/// `1 − cos ψ ≤ (1 − cos(Δv/2)) + c²·(1 − cos(Δu/2))`. The half-space
/// `x·m̂ ≥ r·cos ψ` is convex and holds the three vertices, so it holds
/// the triangle, and every point of the triangle is at least
/// `r·cos ψ` from the centre: the deviation
/// [`crate::cert::cert_sphere`] measures is at most
/// `r·((1 − cos(Δv/2)) + c²·(1 − cos(Δu/2)))`.
///
/// The grid's triangles are cell-halves, and the off-lattice ones
/// beside a rim or meridian chord point stay within one cell **while
/// the lattice's UV aspect `h_u/h_v` is under `2√2`**: a triangle on
/// two vertically adjacent lattice points and a boundary point a
/// distance `x` along the row from their column has a circumcircle
/// that holds the next column's point exactly when
/// `h_u − x < 2·h_v²/x`, which holds for every `x` in `(0, h_u)` once
/// `h_u² < 8·h_v²`. Past that aspect a Delaunay triangle can span two
/// rows (measured: 1.5 δ on a polar cap at latitude 1.2). The `ceil`
/// on the counts can shrink `h_v` by up to half, so the steps are
/// chosen at aspect at most [`SPHERE_CELL_ASPECT`] = √2.
///
/// Within that, to second order the bound is
/// `r·(h_v² + c²·h_u²)/8`, and the product `h_u·h_v` under it is
/// largest where the two terms are equal (AM–GM), `h_u ≈ h_v/c`.
/// Where `1/c ≤ √2` (the window reaches within 45° of the equator)
/// each direction spends `δ/2` exactly: `h_v` is the sagitta step on
/// `r` and `h_u` on `r·c²`, and the worst triangle — a cell-half at
/// the face's widest latitude, where the bound is attained as the
/// steps shrink — lands at δ. A face spanning the equator has `c = 1`
/// and the two steps equal. Past 45° the aspect binds: `h_u = √2·h_v`, with
/// `h_v` spending the whole second-order bound,
/// `h_v = √(8δ/(r·(1 + 2c²)))`, which the exact bound meets from below
/// and attains to second order. Both are pinned by
/// `sphere_grid_steps_land_the_worst_triangle_at_delta`.
pub(crate) fn sphere_grid_steps(target: f64, r: f64, widest: f64) -> (f64, f64) {
    let half = target * 0.5;
    let (hu, hv) = (
        sagitta_step(half, r * widest.powi(2)),
        sagitta_step(half, r),
    );
    if hu <= SPHERE_CELL_ASPECT * hv {
        return (hu, hv);
    }
    let hv = cap_angular(curvature_step(
        target,
        r * SPHERE_CELL_ASPECT.mul_add(SPHERE_CELL_ASPECT * widest.powi(2), 1.0),
    ));
    (cap_angular(SPHERE_CELL_ASPECT * hv), hv)
}

/// The largest UV aspect `h_u/h_v` [`sphere_grid_steps`] chooses: half
/// of the `2√2` past which the off-lattice lemma fails, the other half
/// spent on the `ceil` that can halve `h_v`. **Structural**: it keeps
/// the lattice in the shape its theorem is stated for.
pub(crate) const SPHERE_CELL_ASPECT: f64 = core::f64::consts::SQRT_2;

/// The cosine of a sphere face's latitude nearest the equator: 1 when
/// its latitude window holds the equator, else `√(1 − s²)` at the
/// window's end nearest it, `s` that end's height over the radius. The
/// window is each boundary circle's own height range along the axis,
/// `c + a·cos t + b·sin t` in its parameter, read in closed form over the
/// edge's interval (`trimmed::harmonic_range`): a tilted circle's
/// latitude turns inside its interval, so vertices alone do not bound
/// it, and the curved lane's iso faces (rims constant, meridians
/// monotone) read the same window from the same arithmetic. The chord
/// pass, the curved lane and the trimmed lane's sphere arm read it from
/// this one place.
pub(crate) fn sphere_face_widest_cos(
    body: &topo::Body<f64>,
    fk: topo::FaceKey,
    surface: &geom::Surface<f64>,
) -> Result<f64, TessellateError> {
    let geom::Surface::Sphere {
        center,
        radius,
        axis,
        ..
    } = *surface
    else {
        return Err(TessellateError::MissingEntity {
            what: "a sphere face's widest latitude on a non-sphere surface",
        });
    };
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for (ek, _) in crate::walk::loop_edges(body, face.outer, fk)? {
        let curve = body
            .get_edge(ek)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?;
        let geom::Curve3::Circle { center: cc, .. } = *curve.carrier() else {
            return Err(TessellateError::UnsupportedCurve {
                edge: ek,
                note: "a sphere face bounded by an edge that is not a circle — its \
                       lattice is sized from its circles' latitude ranges",
            });
        };
        let height = |p: geom_core::Point3<f64>| (p - center).dot(axis);
        let c = height(cc);
        let a = height(curve.carrier().eval(0.0)) - c;
        let b = height(curve.carrier().eval(core::f64::consts::FRAC_PI_2)) - c;
        let (z0, z1) = crate::trimmed::harmonic_range(c, a, b, curve.params());
        lo = lo.min(z0);
        hi = hi.max(z1);
    }
    if lo.is_nan() || hi.is_nan() || lo > hi {
        return Err(TessellateError::MissingEntity {
            what: "sphere face latitude window",
        });
    }
    Ok(if lo <= 0.0 && hi >= 0.0 {
        1.0
    } else {
        let s = (lo.abs().min(hi.abs()) / radius).min(1.0);
        (1.0 - s.powi(2)).sqrt()
    })
}

/// The torus chart's two grid steps `(h_u, h_v)` — azimuth θ and
/// minor angle φ — for chord deviation ≤ `target` on every triangle
/// of an `h_u × h_v` cell grid, from the doubly-curved interpolation
/// bound below. Shared by the curved-face grid sizing and the chord
/// pass's adjacent-torus tightening ([`grid_boundary_step`]) so a
/// boundary polyline and the interior grid agree on the rows they
/// share.
///
/// # The bound (proved; [`crate::cert::cert_torus`] is its forward form)
///
/// Write the torus as `P(θ, φ) = c + radial(θ)·(R + r·cos φ) +
/// axis·(r·sin φ)`. Its second partials have `‖P_θθ‖ = R + r·cos φ`,
/// `‖P_θφ‖ = r·|sin φ|` and `‖P_φφ‖ = r`; over the whole tube the sups
/// are `A = R + r`, `B = r`, `C = r`. For a UV triangle `T` with
/// vertices `a_i` and a point `x = Σ λ_i·a_i` of it, Taylor with the
/// integral remainder at `x` gives, with `d_i = a_i − x`,
///
/// ```text
/// P(a_i) − P(x) = DP(x)·d_i + ∫₀¹ (1−t)·D²P(x + t·d_i)(d_i, d_i) dt
/// ```
///
/// and `Σ λ_i·d_i = 0` kills the first-order term, so the affine
/// interpolant `Π(x) = Σ λ_i·P(a_i)` satisfies
///
/// ```text
/// ‖Π − P‖(x) ≤ ½·Σ λ_i·(A·d_iu² + 2B·|d_iu·d_iv| + C·d_iv²)
/// ```
///
/// (`‖D²P(d, d)‖ ≤ A·d_u² + 2B·|d_u·d_v| + C·d_v²` with the sups over
/// `T`, and `∫₀¹ (1−t) dt = ½`). The three weighted sums are the
/// λ-variance of u, the λ-covariance of |u|,|v|, and the λ-variance of
/// v over the triangle's corners: a variable confined to an interval
/// of length `Δ` has variance at most `Δ²/4` (Popoviciu), and
/// Cauchy–Schwarz bounds the middle sum by the geometric mean of the
/// two variances. With `Δu`, `Δv` the triangle's UV extents,
///
/// ```text
/// ‖Π − P‖ ≤ (A·Δu² + 2B·Δu·Δv + C·Δv²) / 8          (★)
/// ```
///
/// **The constant 8 is not slack.** On a cell-half with legs
/// `(h_u, h_v)`, at the hypotenuse midpoint (λ = 0, ½, ½) all three
/// sums attain their bounds at once, so (★) is exact for a constant
/// Hessian whose blocks add with aligned signs — and at φ = 0 on the
/// torus `P_θθ` and `P_φφ` are both radial-inward and `P_θφ` vanishes,
/// so there the second-order deviation IS (★). `cert_torus`'s
/// `certificate_is_attained_on_the_outer_equator` measures that.
///
/// # The steps: the optimum of (★) over the grid, closed-form
///
/// A grid cell splits (either diagonal) into right triangles with
/// `Δu = h_u`, `Δv = h_v`, so the grid is sound when
/// `A·h_u² + 2B·h_u·h_v + C·h_v² ≤ 8·δ`. Fewest cells is largest
/// `h_u·h_v` under that. Put `x = h_u·√A`, `y = h_v·√C`,
/// `β = B/√(A·C) = √(r/(R + r))`: the constraint reads
/// `x² + 2β·x·y + y² ≤ 8·δ`, and for a fixed product `p = x·y`,
/// `x² + y² ≥ 2p` (AM–GM), so `p ≤ 4·δ/(1 + β)` with equality iff
/// `x = y`. The optimum is therefore
///
/// ```text
/// h_u = √(4·δ / ((1 + β)·(R + r)))      h_v = √(4·δ / ((1 + β)·r))
/// ```
///
/// — the two pure terms take equal shares `4δ/(1+β)` of the budget
/// and the mixed term the rest, `8βδ/(1+β)`; equivalently the aspect
/// is fixed at `h_u·√A = h_v·√C` and the scale solved. Nothing here is
/// chosen: the split is the theorem, and
/// `torus_grid_steps_meet_the_bound_with_equality` pins that the steps
/// spend exactly `δ` of it. The only cost against this ideal is the
/// `ceil` in [`ceil_count`], `≤ (1 + 1/n_u)(1 + 1/n_v)` in cells.
///
/// Against the per-direction sagitta steps `√(8δ/A)`, `√(8δ/C)`
/// the cell count is `2(1 + β)` times higher — the 2 because a
/// cell-half's hypotenuse midpoint sees BOTH pure sagittas at once
/// (the naive schedule's deviation there is `2δ`), the `1 + β` for
/// the mixed term. `β ∈ (0, 1/√2)` on a ring torus (`R > r`), so the
/// factor lies in `(2, 2 + √2)`.
///
/// # What is deliberately NOT read: the face's φ window
///
/// A face avoiding the outer equator would admit `A_W = R + r·max_W
/// cos φ < A` and a coarser `h_u` (at most `√((R + r)/R)` fewer
/// columns), and the sharp joint maximum over φ of the two φ-dependent
/// terms would buy a few percent more. The chord pass sizes an edge
/// before any face's walk has established its window, and a window
/// read on the grid side alone would put the boundary rows off the
/// grid's; one rule for both keeps them coincident. The certificate
/// does read each triangle's own window, so the pin measures the
/// sizing's real slack. The lever and its price are
/// `work/perf/torus-sizing-reads-no-phi-window.md`.
///
/// Uncapped here, and the two consumers differ on that. The curved
/// lane steps periodic chart coordinates with these directly and
/// applies [`cap_angular`] itself; the chord pass takes one as a
/// *lower bound on a count* it has already sized from the circle
/// sagitta, and that sagitta step is capped over the same span.
/// `h_u` exceeds the cap only when `δ > (π²/64)(1 + β)(R + r)`
/// (`≥ 0.154·(R + r)`), and every rim circle a torus carries has
/// radius `≤ R + r`, over which the sagitta step is already the cap
/// once `δ ≥ (R + r)(1 − cos(π/8)) ≈ 0.076·(R + r)`; `h_v` the same
/// with `r`, the radius of every meridian circle. So in the capped regime
/// the sagitta step is exactly the cap and the capped and uncapped
/// requirements coincide, per direction. No in-tree body reaches that
/// regime, so the claim is pinned directly rather than by the mesh
/// oracles: `torus_cap_regime_is_sagitta_capped` below goes red if
/// either formula or [`MAX_ANGULAR_STEP`] moves.
///
/// The chord pass must NOT simply cap here to sidestep the argument:
/// [`ceil_count`] refuses a non-finite step typed, while
/// [`cap_angular`] turns one into the cap, so capping a poisoned torus
/// step there would swallow a refusal.
pub(crate) fn torus_grid_steps(target: f64, major: f64, minor: f64) -> (f64, f64) {
    let a = major + minor;
    let beta = (minor / a).sqrt();
    // Each direction is the plain second-derivative inversion
    // (`h² · m / 8 ≤ share`) at a `2(1 + β)`-fold smaller share of
    // δ: the 2 for the coupling of the two pure terms, the `1 + β`
    // for the mixed one.
    let share = target / (2.0 * (1.0 + beta));
    (curvature_step(share, a), curvature_step(share, minor))
}

/// The grid step a circle edge adjacent to face `fk` must be chorded
/// within, if that face is the curved lane's
/// ([`crate::tessellate::lane_of`]) and its theorem is about a grid its
/// rims have to sit on (`crates/mesh/README.md` `[mesh.sizing]`, *rim
/// chords use the face's grid step*): the torus's
/// [`torus_grid_steps`], the sphere's [`sphere_grid_steps`] at the
/// face's widest latitude ([`sphere_face_widest_cos`]), each in the
/// edge's own direction — `h_u` for a rim (its carrier parameter is
/// the azimuth), `h_v` for a meridian (the minor angle or the
/// latitude); and the cone's [`cone_grid_step`] at the face's largest
/// rim ([`cone_face_rho_max`]), every circle a cone carries being a
/// rim. Every other face answers `None`, a face another lane takes
/// included: its theorem is not about this grid.
///
/// The rim/meridian rule is [`topo::chart_iso::classify_kind`]'s — the
/// same rule the walk classifies the same edge with when it lays the
/// boundary polygon on the face's grid, so the count sized here and
/// the grid row it lands on agree by construction rather than by a
/// second copy of the threshold. That rule is total on circle
/// carriers (`|n · axis| > 0.5` splits rim from meridian, every
/// direction falling on one side), so nothing refuses HERE: a circle
/// on a sphere that is neither iso-curve and stores its projected image
/// routes its face to the trimmed lane's sphere arm, which this
/// function answers `None` for; any other non-iso circle on a torus or
/// a sphere — a Villarceau circle, an oblique small circle with no
/// stored image — is refused by the face door
/// (`geom_brep::props::require_iso_rectangle`, through
/// [`crate::curved`]) before any grid is built on the face, and a
/// whole-body refusal is what a mis-tightened count on such an edge
/// could never outlive. The `None` the classifier reserves for conic
/// and spline carriers is unreachable from a circle and is surfaced
/// typed rather than defaulted, so a widened classifier cannot make
/// this arm silently pick a direction.
pub(crate) fn grid_boundary_step(
    body: &topo::Body<f64>,
    fk: topo::FaceKey,
    curve: &geom_brep::EdgeCurve<f64>,
    edge: topo::EdgeKey,
    target: f64,
) -> Result<Option<f64>, TessellateError> {
    let surface = crate::tessellate::face_surface(body, fk)?;
    if crate::tessellate::lane_of(body, fk, surface)? != crate::tessellate::Lane::Curved {
        return Ok(None);
    }
    let steps = match *surface {
        geom::Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => torus_grid_steps(target, major_radius, minor_radius),
        geom::Surface::Sphere { radius, .. } => {
            sphere_grid_steps(target, radius, sphere_face_widest_cos(body, fk, surface)?)
        }
        geom::Surface::Cone { half_angle, .. } => {
            return Ok(Some(cone_grid_step(
                target,
                half_angle,
                cone_face_rho_max(body, fk)?,
            )));
        }
        _ => return Ok(None),
    };
    let Some(chart) = topo::chart::Chart::of(surface) else {
        unreachable!("Chart::of answers every torus and sphere, and this is one: {surface:?}")
    };
    match topo::chart_iso::classify_kind(&chart, curve) {
        Some(topo::chart_iso::TravKind::Rim { .. }) => Ok(Some(steps.0)),
        Some(topo::chart_iso::TravKind::Meridian { .. }) => Ok(Some(steps.1)),
        None => Err(TessellateError::UnsupportedCurve {
            edge,
            note: "a circle edge on a torus or sphere that the iso-curve classifier left \
                   unclassified — neither rim nor meridian, so no grid direction sizes its \
                   chords",
        }),
    }
}

/// A cone face's largest radius: the largest radius of the circle
/// carriers on its outer loop. The curved lane's iso-rectangle cone
/// face has its widest side on a rim, so this is the face's `ρ_max`;
/// the chord pass and the curved lane both read it here, so the rim
/// count and the grid's column count come from one value.
///
/// # Errors
///
/// [`TessellateError::MissingEntity`] for a face with no circle edge on
/// its outer loop, which no cone face the curved lane meshes is.
pub(crate) fn cone_face_rho_max(
    body: &topo::Body<f64>,
    fk: topo::FaceKey,
) -> Result<f64, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    let mut rho_max: Option<f64> = None;
    for (ek, _) in crate::walk::loop_edges(body, face.outer, fk)? {
        let carrier = body
            .get_edge(ek)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?
            .carrier();
        if let geom::Curve3::Circle { radius, .. } = *carrier {
            rho_max = Some(rho_max.map_or(radius, |m: f64| m.max(radius)));
        }
    }
    rho_max.ok_or(TessellateError::MissingEntity {
        what: "cone face rim",
    })
}

/// `ceil(span/step)` as a chord/grid count, with the `MAX_COUNT` (2^24)
/// sanity cap surfaced as a typed error and a floor of 1.
///
/// Four steps are not ordinary positive lengths, and each reads as
/// follows:
/// - **sign bit set** (`−0.0` and every negative, `−∞` included) or
///   **NaN**: not a length. Every caller's step is derived (`sqrt` of a
///   non-negative ratio, or capped by [`cap_angular`]), so this is a
///   kernel defect, refused as [`TessellateError::StepNotPositive`]
///   rather than floored to one chord.
/// - **`+0.0`**: the underflow end of a positive step, from a δ so fine
///   its step rounds to zero. The count is unbounded, so it refuses as
///   [`TessellateError::ResolutionOverflow`], as any count past the cap
///   does.
/// - **`+∞`**: the zero-curvature reading, "nothing constrains this
///   direction". A straight NURBS direction's grid step is `+∞`
///   (`nurbs_cert`'s `grid_steps`, as is [`curvature_step`] with a
///   bound of 0), and `span/∞ = 0` floors to the one cell such a span
///   needs. This is not a missed refusal.
///
/// # Errors
///
/// [`TessellateError::StepNotPositive`] for a step with its sign bit set
/// or NaN; [`TessellateError::ResolutionOverflow`] when the count is
/// non-finite or at/above the cap.
pub(crate) fn ceil_count(span: f64, step: f64) -> Result<usize, TessellateError> {
    if step.is_nan() || step.is_sign_negative() {
        return Err(TessellateError::StepNotPositive { span, step });
    }
    let raw = (span / step).ceil();
    if !(raw.is_finite() && raw < MAX_COUNT) {
        return Err(TessellateError::ResolutionOverflow { count: raw });
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    Ok(if raw < 1.0 { 1 } else { raw as usize })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **[`ellipse_step`] reads the semi-axes as magnitudes in either
    /// order.** An ellipse stored with `minor` the larger, or a negative
    /// semi-axis, is the same locus; read as stored the step was taken
    /// against far less than the true `sup|C″|` (or was `NaN`). Every
    /// stored frame steps as the ordered one does, and the step keeps
    /// the measured chord deviation of the whole ellipse within
    /// `target`.
    #[test]
    fn the_ellipse_step_reads_its_semi_axes_in_any_stored_frame() {
        let target = 1e-4;
        let (a, b) = (3.0_f64, 0.5_f64);
        let ordered = ellipse_step(target, a, b);
        for (major, minor) in [(b, a), (-a, b), (a, -b), (-b, -a)] {
            let got = ellipse_step(target, major, minor);
            assert!(
                (got - ordered).abs() <= 1e-15 * ordered,
                "({major}, {minor}): step {got} against the ordered {ordered}"
            );
        }
        let p = |t: f64| (a * t.cos(), b * t.sin());
        let mut worst = 0.0_f64;
        let mut t = 0.0;
        while t < core::f64::consts::TAU {
            let (p0, p1) = (p(t), p(t + ordered));
            for k in 1..32 {
                let s = t + ordered * f64::from(k) / 32.0;
                let (q, w) = (p(s), (p1.0 - p0.0, p1.1 - p0.1));
                let cross = (q.0 - p0.0) * w.1 - (q.1 - p0.1) * w.0;
                worst = worst.max(cross.abs() / w.0.hypot(w.1));
            }
            t += ordered;
        }
        assert!(worst <= target, "a chord deviates {worst} past {target}");
    }

    /// **[`Eps`]'s band edges, pinned at the type rather than at each
    /// caller.** Three of the four operations differ ONLY here, so the
    /// edge is the whole of what distinguishes them and the one thing
    /// a caller picking a name is choosing between. Red if any
    /// operation's strictness flips — which is a moved mesh byte
    /// wherever a coordinate sits exactly on the band.
    ///
    /// # These rows are the ONLY defence of the band's WIDTH
    ///
    /// The division of labour is worth stating once, here, because it
    /// is not what a reader would guess: **a byte-identity corpus
    /// carries REACH — which decisions a body actually exercises —
    /// and these type rows carry WIDTH and EDGE.** Two measurements
    /// establish it, both made against transient mutants of this file:
    ///
    /// - flipping `coincident` to a strict `<` leaves the whole tour
    ///   corpus GREEN, `step-import`'s pole-band witness included (it
    ///   sits strictly outside the default band, so inclusivity never
    ///   decides it) — and reds the row below;
    /// - doubling the pole band outright ALSO leaves the corpus green:
    ///   the witness's own row accepts any S22 panic, and a sibling
    ///   unwidened read still panics with the message it greps for, so
    ///   the identification flip is masked. The row below reds on it.
    ///
    /// So a future lane must not read a green corpus as evidence that
    /// a band edge or a band width is unchanged; it is evidence about
    /// reach. **If these rows go, nothing in the tree notices a band
    /// that has silently doubled.**
    #[test]
    fn the_band_edges_are_where_the_operations_differ() {
        let e = Eps::exactly(1e-9);
        // AT the edge: coincident includes it, the other two exclude.
        assert!(e.coincident(1e-9), "`coincident` includes the band");
        assert!(!e.separates(1e-9), "`separates` excludes the band");
        assert!(!e.dominates(1e-9), "`dominates` excludes the band");
        // Inside and outside, where all three agree with their names.
        assert!(e.coincident(0.5e-9) && e.dominates(0.5e-9) && !e.separates(0.5e-9));
        assert!(e.separates(2e-9) && !e.coincident(2e-9) && !e.dominates(2e-9));
        // A ZERO band admits nothing: `dominates` is strict, which is
        // the exact form `curved`'s band fixtures compare against.
        assert!(!Eps::exactly(0.0).dominates(0.0));
    }

    /// **`separates` and `coincident` are negations on ordered input
    /// and NOT on a NaN** — the reason neither is written as the
    /// other's `!`. Both are false on a poisoned length, so a NaN
    /// stays neither near nor far; `!coincident` would report it as
    /// separated, which is a decision this crate does not make.
    #[test]
    fn a_poisoned_length_is_neither_near_nor_far() {
        let e = Eps::exactly(1e-9);
        for x in [0.0, 1e-12, 1e-9, 1e-6, f64::INFINITY] {
            assert_ne!(e.separates(x), e.coincident(x), "ordered input at {x}");
        }
        assert!(!e.separates(f64::NAN) && !e.coincident(f64::NAN));
        assert!(!e.dominates(f64::NAN));
    }

    /// **`pad` widens UP, by exactly one band.** The direction is the
    /// claim: [`crate::trimmed`]'s probe divides by the result, so a
    /// downward pad would inflate the ratio it publishes instead of
    /// keeping evaluation rounding out of it.
    #[test]
    fn pad_widens_upward_by_one_band() {
        let e = Eps::exactly(1e-9);
        assert_eq!(e.pad(1.0), 1.0 + 1e-9);
        assert!(e.pad(0.0) > 0.0, "a zero bound pads to the band itself");
        assert_eq!(Eps::exactly(0.0).pad(4.0), 4.0, "a zero band pads nothing");
    }

    /// **The port's central claim, executed: each operation computes
    /// bit-for-bit the boolean the bare spelling computed.**
    ///
    /// Everything else about this port rests on this. The
    /// byte-identity corpus shows that no BODY IN THE TREE notices the
    /// change; this row shows the change is not noticeable — over the
    /// representable neighbours of the band edge, at ±0.0, at the
    /// subnormal floor, at the infinities and on NaN, across bands
    /// including a zero band and a NaN band. Where the corpus argues
    /// from reach, this argues from the operation.
    ///
    /// `pad` is compared ON BITS rather than by value, so a NaN bound
    /// is a real comparison and a signed zero cannot pass by
    /// numeric equality.
    ///
    /// Lifted from the review lanes, whose independent instruments are
    /// recorded under `review/r1-mesh4/` and `probes/`: the band-edge
    /// value sweep is R1's transient probe, made permanent here
    /// because nothing in-tree executed this claim; the sweep over
    /// MANY bands (zero and NaN included) is R2's differential
    /// dimension, which found the same answer over 242,040 checks
    /// against the merge base's actual spellings.
    #[test]
    fn each_operation_is_bitwise_the_bare_spelling_it_replaced() {
        for band in [1e-9f64, 1e-6, 1e-12, 0.0, 1.0, f64::MIN_POSITIVE, f64::NAN] {
            let e = Eps::exactly(band);
            // The representable neighbours of the edge, plus the
            // values a length can hold that are not ordinary lengths.
            let up = f64::from_bits(band.to_bits().wrapping_add(1));
            let down = f64::from_bits(band.to_bits().wrapping_sub(1));
            for x in [
                0.0,
                -0.0,
                f64::MIN_POSITIVE,
                down,
                band,
                up,
                2e-9,
                1e-6,
                1.0,
                -1.0,
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NAN,
            ] {
                assert_eq!(
                    e.separates(x),
                    x > band,
                    "separates at band {band:e}, x {x:e}"
                );
                assert_eq!(
                    e.coincident(x),
                    x <= band,
                    "coincident at band {band:e}, x {x:e}"
                );
                assert_eq!(
                    e.dominates(x),
                    x < band,
                    "dominates at band {band:e}, x {x:e}"
                );
                assert_eq!(
                    e.pad(x).to_bits(),
                    (x + band).to_bits(),
                    "pad at band {band:e}, x {x:e} (bitwise, NaN included)"
                );
            }
        }
    }

    /// [`torus_grid_steps`]' doc claim, which no meshing oracle
    /// reaches, per direction: above `target = (pi^2/64)(1+beta)(R+r)`
    /// the u step passes the angular cap, and there the sagitta step
    /// over the widest rim circle a torus carries (`R + r`) is EXACTLY
    /// the cap; above `(pi^2/64)(1+beta)·r` the v step does, and there
    /// the sagitta step over every meridian circle (`r`) is the cap —
    /// so the chord pass's uncapped `max` and the curved lane's capped
    /// count agree in both directions. Red if either formula or the
    /// cap moves.
    #[test]
    fn torus_cap_regime_is_sagitta_capped() {
        use core::f64::consts::PI;
        let cases: &[(f64, f64)] = &[(1.0, 0.25), (5.0, 3.0), (0.5, 0.4), (100.0, 1.0)];
        for &(major, minor) in cases {
            let beta = (minor / (major + minor)).sqrt();
            let (u_threshold, v_threshold) = (
                PI.powi(2) / 64.0 * (1.0 + beta) * (major + minor),
                PI.powi(2) / 64.0 * (1.0 + beta) * minor,
            );
            let (hu, _) = torus_grid_steps(u_threshold * 1.0001, major, minor);
            assert!(
                hu > MAX_ANGULAR_STEP,
                "the u threshold no longer predicts the uncapped regime"
            );
            assert!(
                sagitta_step(u_threshold * 1.0001, major + minor) == MAX_ANGULAR_STEP,
                "the rim sagitta step is not exactly the cap, so the two requirements diverge"
            );
            let (hu, _) = torus_grid_steps(u_threshold * 0.9999, major, minor);
            assert!(
                hu <= MAX_ANGULAR_STEP,
                "the u threshold no longer predicts the capped regime"
            );
            let (_, hv) = torus_grid_steps(v_threshold * 1.0001, major, minor);
            assert!(
                hv > MAX_ANGULAR_STEP,
                "the v threshold no longer predicts the uncapped regime"
            );
            assert!(
                sagitta_step(v_threshold * 1.0001, minor) == MAX_ANGULAR_STEP,
                "the meridian sagitta step is not exactly the cap, so the two requirements diverge"
            );
            let (_, hv) = torus_grid_steps(v_threshold * 0.9999, major, minor);
            assert!(
                hv <= MAX_ANGULAR_STEP,
                "the v threshold no longer predicts the capped regime"
            );
        }
    }

    /// **The edge-chord theorem at its step**: a circle chord at
    /// [`sagitta_step`]`(δ, ρ)` has sagitta exactly δ, and a C² chord at
    /// [`curvature_step`]`(δ, m)` spends exactly δ of `h²·m/8`, wherever
    /// the cap does not bind. A step shrunk by any factor reds the
    /// first equality.
    #[test]
    fn edge_chord_steps_meet_the_bound_with_equality() {
        for rho in [0.05_f64, 1.0, 50.0] {
            for delta in [1e-6_f64, 1e-4, 1e-3] {
                let h = sagitta_step(delta, rho);
                assert!(h < MAX_ANGULAR_STEP, "ρ {rho} δ {delta}: the cap binds");
                let sagitta = rho * (1.0 - (0.5 * h).cos());
                assert!(
                    ((sagitta - delta) / delta).abs() < 1e-6,
                    "ρ {rho} δ {delta}: the chord's sagitta is {sagitta}, not δ"
                );
                let h = curvature_step(delta, rho);
                let spent = h * h * rho / 8.0;
                assert!(
                    ((spent - delta) / delta).abs() < 1e-12,
                    "m {rho} δ {delta}: the step spends {spent} of h²·m/8"
                );
            }
        }
    }

    /// **The curved cylinder's worst triangle is a rim chord's, and it
    /// lands at δ**: a quad-half between two rims chorded at
    /// [`sagitta_step`]`(δ, r)` certifies (`cert::cert_cylinder`) at δ.
    /// And the strip argument at [`sagitta_step`], on spade's own CDT:
    /// two rows of unequal, uneven spacing at several heights, every
    /// triangle's azimuth extent within the larger row spacing.
    #[test]
    fn cylinder_rim_triangle_lands_at_delta() {
        use geom_core::{Point3, Vec3};
        let (r, delta) = (0.7_f64, 1e-3);
        let h = sagitta_step(delta, r);
        let at = |u: f64, z: f64| Point3::new(r * u.cos(), r * u.sin(), z);
        let cert = crate::cert::cert_cylinder(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            r,
            [at(0.0, 0.0), at(h, 0.0), at(h, 0.4)],
        );
        assert!(
            ((cert - delta) / delta).abs() < 1e-6,
            "the rim-chord triangle certifies at {cert}, not δ {delta}"
        );

        use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
        let rows = |n: usize, jitter: f64| -> Vec<f64> {
            #[allow(clippy::cast_precision_loss)]
            (0..=n)
                .map(|i| {
                    let t = i as f64 / n as f64;
                    let wobble = if i == 0 || i == n {
                        0.0
                    } else {
                        jitter * (7.0 * t).sin()
                    };
                    t + wobble / n as f64
                })
                .collect()
        };
        let spacing = |row: &[f64]| row.windows(2).map(|w| w[1] - w[0]).fold(0.0, f64::max);
        let mut worst = 0.0_f64;
        for (nb, nt) in [(5, 9), (7, 7), (3, 11), (6, 13)] {
            for height in [1e-3, 0.05, 0.3, 1.0, 20.0] {
                let (bottom, top) = (rows(nb, 0.3), rows(nt, -0.2));
                let h = spacing(&bottom).max(spacing(&top));
                let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
                let mut ring = Vec::new();
                for &u in &bottom {
                    ring.push(cdt.insert(Point2::new(u, 0.0)).unwrap());
                }
                for &u in top.iter().rev() {
                    ring.push(cdt.insert(Point2::new(u, height)).unwrap());
                }
                for i in 0..ring.len() {
                    cdt.add_constraint(ring[i], ring[(i + 1) % ring.len()]);
                }
                for f in cdt.inner_faces() {
                    let us = f.vertices().map(|v| v.position().x);
                    let du = us.iter().fold(f64::MIN, |m, &u| m.max(u))
                        - us.iter().fold(f64::MAX, |m, &u| m.min(u));
                    worst = worst.max(du / h);
                }
            }
        }
        assert!(
            worst <= 1.0 + 1e-12,
            "a strip triangle spans {worst} row spacings in azimuth"
        );
    }

    /// **The cone's worst triangle lands at δ** at [`cone_grid_step`]: a
    /// cell-half on the largest radius certifies (`cert::cert_cone`) at
    /// δ, and the step is coarser than the plain sagitta at that radius
    /// by the cone's `cos α` (the chord's sag lies partly along the
    /// surface).
    #[test]
    fn cone_grid_step_lands_the_worst_triangle_at_delta() {
        for half_angle in [0.1_f64, 0.6, 1.2] {
            for (rho_max, delta) in [(1.0_f64, 1e-3), (0.05, 1e-5), (3.0, 1e-4)] {
                let h = cone_grid_step(delta, half_angle, rho_max);
                assert!(
                    h < MAX_ANGULAR_STEP,
                    "α {half_angle} ρ {rho_max}: the cap binds"
                );
                let v_max = rho_max / half_angle.sin();
                let cert = crate::cert::cert_cone(
                    half_angle,
                    [[0.0, v_max], [h, v_max], [0.0, 0.9 * v_max]],
                    [false; 3],
                );
                assert!(
                    ((cert - delta) / delta).abs() < 1e-6,
                    "α {half_angle} ρ {rho_max} δ {delta}: the cell-half certifies at {cert}"
                );
                assert!(h > sagitta_step(delta, rho_max));
                // The same triangle measured against the cone itself, so a
                // wrong `cert_cone` reds here rather than agreeing with the
                // step that inverts it: dense barycentric samples, each at
                // its distance `|cos α·ρ − sin α·z|` from the generator
                // through its azimuth.
                let (sa, ca) = half_angle.sin_cos();
                let at = |u: f64, v: f64| [v * sa * u.cos(), v * sa * u.sin(), v * ca];
                let corners = [at(0.0, v_max), at(h, v_max), at(0.0, 0.9 * v_max)];
                let mut worst = 0.0_f64;
                for i in 0..=48 {
                    for j in 0..=(48 - i) {
                        let (l0, l1) = (f64::from(i) / 48.0, f64::from(j) / 48.0);
                        let l2 = 1.0 - l0 - l1;
                        let p: [f64; 3] = core::array::from_fn(|k| {
                            l0 * corners[0][k] + l1 * corners[1][k] + l2 * corners[2][k]
                        });
                        worst = worst.max((ca * p[0].hypot(p[1]) - sa * p[2]).abs());
                    }
                }
                assert!(
                    worst <= cert * (1.0 + 1e-9) && worst >= cert * 0.99,
                    "α {half_angle} ρ {rho_max} δ {delta}: sampled {worst} against the \
                     certificate {cert}"
                );
            }
        }
    }

    /// **The sphere's theorem at [`sphere_grid_steps`]**: the bound
    /// `r·((1 − cos(h_v/2)) + c²·(1 − cos(h_u/2)))` spends exactly δ at
    /// the steps, half in each direction; every cell-half of either
    /// diagonal, in latitude bands across the face's window, certifies
    /// (`cert::cert_sphere`) within δ; and the cell-half at the widest
    /// latitude attains it as the steps shrink (to second order on the
    /// equator, to within `tan φ·h_v` off it), so the bound has no slack
    /// the steps could give back. The windows are a band spanning
    /// the equator (`c = 1`, equal steps), one at [0.6, 1.0] (the equal
    /// split, `h_u ≈ 1.2·h_v`), and two past 45° where the lattice's
    /// aspect bound binds: [0.9, 1.5] and [1.45, 1.55].
    #[test]
    fn sphere_grid_steps_land_the_worst_triangle_at_delta() {
        use geom_core::Point3;
        let windows: [(f64, f64); 4] = [(-0.4, 0.4), (0.6, 1.0), (0.9, 1.5), (1.45, 1.55)];
        for (r, delta) in [(1.0_f64, 1e-2), (1.0, 1e-4), (0.02, 1e-5)] {
            for (lo, hi) in windows {
                let c = if lo <= 0.0 && hi >= 0.0 {
                    1.0
                } else {
                    lo.abs().min(hi.abs()).cos()
                };
                let (hu, hv) = sphere_grid_steps(delta, r, c);
                assert!(hv < MAX_ANGULAR_STEP, "r {r} δ {delta}: the cap binds");
                let spent = r * ((1.0 - (0.5 * hv).cos()) + c * c * (1.0 - (0.5 * hu).cos()));
                assert!(
                    hu <= SPHERE_CELL_ASPECT * hv * (1.0 + 1e-12),
                    "r {r} δ {delta} window [{lo}, {hi}]: aspect {} past the lemma's",
                    hu / hv
                );
                if hu < MAX_ANGULAR_STEP {
                    // Exactly δ where the equal split stands; from below,
                    // to second order, where the aspect binds.
                    let exact = hu < SPHERE_CELL_ASPECT * hv;
                    assert!(
                        spent <= delta * (1.0 + 1e-6)
                            && spent >= delta * if exact { 1.0 - 1e-6 } else { 1.0 - hv * hv },
                        "r {r} δ {delta} window [{lo}, {hi}]: the steps spend {spent}"
                    );
                }
                let at = |u: f64, v: f64| {
                    Point3::new(r * v.cos() * u.cos(), r * v.cos() * u.sin(), r * v.sin())
                };
                // Bands from the widest latitude across the window.
                let widest = if c == 1.0 { -0.5 * hv } else { lo };
                let mut at_widest = 0.0_f64;
                let mut v0 = widest;
                while v0 + hv <= hi.max(widest + hv) + 1e-12 {
                    let (a, b, cc, d) =
                        (at(0.0, v0), at(hu, v0), at(0.0, v0 + hv), at(hu, v0 + hv));
                    for tri in [[a, b, cc], [b, d, cc], [a, b, d], [a, d, cc]] {
                        let cert = crate::cert::cert_sphere(Point3::origin(), r, tri);
                        assert!(
                            cert <= delta * (1.0 + 1e-9),
                            "r {r} δ {delta} band {v0}: a cell-half certifies at {cert}"
                        );
                        if v0 == widest {
                            at_widest = at_widest.max(cert);
                        }
                    }
                    v0 += hv.max((hi - lo) / 7.0);
                }
                if hu < MAX_ANGULAR_STEP {
                    // Off the equator the cell's own latitudes sit up to a
                    // row inside the widest one, so the azimuth half falls
                    // short by up to `tan φ·h_v` of itself; on it, only by
                    // second order.
                    let short = if c < 1.0 {
                        lo.tan() * hv
                    } else {
                        2.0 * hu * hu
                    };
                    assert!(
                        at_widest >= delta * (1.0 - short),
                        "r {r} δ {delta} window [{lo}, {hi}]: the widest cell-half \
                         certifies at only {at_widest}"
                    );
                }
            }
        }
    }

    /// [`torus_grid_steps`] spends EXACTLY `target` of the bound (★)
    /// on a cell-half — `(A·hu² + 2B·hu·hv + C·hv²)/8 == target` — at
    /// the aspect `hu·√A == hv·√C` the closed-form optimum names, and
    /// in the uncapped regime both steps sit strictly inside the
    /// per-direction sagitta steps, which is what makes a rim or
    /// meridian edge's torus count the one that binds (the boundary
    /// and the grid coincide). The first equality is the row that goes
    /// red if a constant drifts: a bound ten times too loose passes
    /// every certifier and fails this.
    #[test]
    fn torus_grid_steps_meet_the_bound_with_equality() {
        let cases: &[(f64, f64)] = &[(0.30, 0.07), (2.0, 0.5), (1.2, 1.0), (50.0, 1.0)];
        for &(major, minor) in cases {
            for &target in &[1e-6_f64, 1e-4, 1e-2] {
                let (hu, hv) = torus_grid_steps(target, major, minor);
                let (a, b, c) = (major + minor, minor, minor);
                let spent = (a * hu * hu + 2.0 * b * hu * hv + c * hv * hv) / 8.0;
                assert!(
                    ((spent - target) / target).abs() < 1e-12,
                    "R {major} r {minor} target {target}: the steps spend {spent} of the bound"
                );
                assert!(
                    ((hu * a.sqrt() - hv * c.sqrt()) / (hv * c.sqrt())).abs() < 1e-12,
                    "R {major} r {minor}: the aspect is off the optimum"
                );
                assert!(hu < sagitta_step(target, a) && hv < sagitta_step(target, c));
                // The cost against the per-direction sagitta, in cells.
                let naive = curvature_step(target, a) * curvature_step(target, c);
                let ratio = naive / (hu * hv);
                let beta = (minor / a).sqrt();
                assert!(((ratio - 2.0 * (1.0 + beta)) / ratio).abs() < 1e-12);
                assert!(ratio > 2.0 && ratio < 2.0 + core::f64::consts::SQRT_2);
            }
        }
    }

    /// [`torus_grid_steps`] and [`crate::cert::cert_torus`] are an
    /// INVERSE PAIR through one arithmetic: the certifier's own bound
    /// (`cert::torus_chord_bound`, which `cert_torus` evaluates at a
    /// triangle's window sups and extents) on a cell-half of the
    /// sizing's steps at the whole-tube sups is exactly `target`. A
    /// loosening of the certifier's constant alone passes every
    /// per-triangle row and reds this one.
    #[test]
    fn torus_grid_steps_and_cert_torus_are_an_inverse_pair() {
        let cases: &[(f64, f64)] = &[(0.30, 0.07), (2.0, 0.5), (1.2, 1.0), (50.0, 1.0)];
        for &(major, minor) in cases {
            for &target in &[1e-6_f64, 1e-4, 1e-2] {
                let (hu, hv) = torus_grid_steps(target, major, minor);
                let certified = crate::cert::torus_chord_bound(major + minor, minor, minor, hu, hv);
                assert!(
                    ((certified - target) / target).abs() < 1e-12,
                    "R {major} r {minor} target {target}: the certifier reads {certified} on \
                     the sizing's own cell-half"
                );
            }
        }
    }

    /// VERBS-TESSFOLD review probe (PR #1045): the fold's analytic
    /// audit re-derived inside the tree, from the scenes' constants and
    /// nothing observed. `hollowring`'s two wall sizes, the
    /// `tube_along_arc` reference pair, and `diecomposed`'s pip-rim
    /// blend tori are exactly what [`torus_grid_steps`] prices — the
    /// rim torus's major radius and arc are derived here from the
    /// rolling-ball construction (ball tangent to the top plane and
    /// externally tangent to the cavity sphere), so no number in this
    /// row was read off a CSV first.
    #[test]
    fn tessfold_r1_torus_rows_rederive_from_scene_constants() {
        use core::f64::consts::PI;
        let tris = |delta: f64, major: f64, minor: f64, uspan: f64, vspan: f64| {
            let (hu, hv) = torus_grid_steps(delta, major, minor);
            2 * ceil_count(uspan, cap_angular(hu)).unwrap()
                * ceil_count(vspan, cap_angular(hv)).unwrap()
        };
        // hollowring (demos/tour/src/ring.rs): R = 0.30, walls
        // r_i = 0.05 / r_o = 0.07, delta = 2e-3; each wall is two
        // half-tube faces (uspan 2*pi, vspan pi).
        assert_eq!(tris(2e-3, 0.30, 0.05, 2.0 * PI, PI), 980);
        assert_eq!(tris(2e-3, 0.30, 0.07, 2.0 * PI, PI), 1_248);
        // tube_along_arc (demos/tour/src/tube.rs): R = 2, r = 0.5,
        // delta = 1e-2, arc T1 - T0 = 1.5.
        assert_eq!(tris(1e-2, 2.0, 0.5, 1.5, PI), 420);
        // diecomposed pip-rim blend (demos/tour/src/diefillet.rs):
        // RIM_R = 0.02 ball between the face plane and the
        // PIP_R = 0.09 cavity sphere whose centre stands
        // d = PIP_R - PIP_H = 0.04 off the plane; centre circle
        // rho = sqrt((R+r)^2 - (d+r)^2), arc acos((d+r)/(R+r));
        // delta = 5e-3, full rim (uspan 2*pi).
        let (pip_r, pip_h, rim_r) = (0.09_f64, 0.05, 0.02);
        let d = pip_r - pip_h;
        let major = ((pip_r + rim_r).powi(2) - (d + rim_r).powi(2)).sqrt();
        let vspan = ((d + rim_r) / (pip_r + rim_r)).acos();
        assert_eq!(tris(5e-3, major, rim_r, 2.0 * PI, vspan), 72);
    }

    /// [`cap_angular`]'s documented total behaviour. The obvious-looking
    /// rewrite `if step > MAX { MAX } else { step }` is NOT equivalent —
    /// it returns NaN — and nothing else in the tree would notice.
    #[test]
    fn cap_angular_is_total() {
        assert!(cap_angular(f64::NAN) == MAX_ANGULAR_STEP);
        assert!(cap_angular(f64::INFINITY) == MAX_ANGULAR_STEP);
        assert!(cap_angular(MAX_ANGULAR_STEP) == MAX_ANGULAR_STEP);
        assert!(cap_angular(0.1) == 0.1);
        assert!(cap_angular(f64::NEG_INFINITY) == f64::NEG_INFINITY);
    }

    /// [`ceil_count`]'s documented edges at the boundary rather than
    /// through a whole tessellation (`tests/errors.rs` reaches
    /// `ResolutionOverflow` only end-to-end): the floor of 1, and the
    /// AT-the-cap refusal — `MAX_COUNT` itself refuses, it is not the
    /// last accepted value.
    #[test]
    fn ceil_count_floors_at_one_and_refuses_at_the_cap() {
        assert!(ceil_count(1.0, 10.0) == Ok(1));
        assert!(ceil_count(MAX_COUNT - 1.0, 1.0) == Ok(16_777_215));
        assert!(matches!(
            ceil_count(MAX_COUNT, 1.0),
            Err(TessellateError::ResolutionOverflow { .. })
        ));
    }

    /// **A step that is not a length refuses as a kernel defect, never
    /// as a count** (D303). A step with its sign bit set, or NaN, used to
    /// floor to one chord: `span/step` is negative and finite, and
    /// `ceil` of it is below 1. `+0.0` stays the overflow it is (the
    /// underflow end of a positive step), and `+∞` stays the one cell of
    /// the zero-curvature reading.
    #[test]
    fn a_step_that_is_not_a_length_refuses_before_any_count() {
        for step in [-1.0, -0.0, -1e-300, f64::NEG_INFINITY, f64::NAN] {
            assert!(
                matches!(
                    ceil_count(1.0, step),
                    Err(TessellateError::StepNotPositive { .. })
                ),
                "step {step}: {:?}",
                ceil_count(1.0, step)
            );
        }
        assert!(matches!(
            ceil_count(1.0, 0.0),
            Err(TessellateError::ResolutionOverflow { .. })
        ));
        assert!(ceil_count(1.0, f64::INFINITY) == Ok(1));
        assert!(ceil_count(core::f64::consts::PI, 0.5) == Ok(7));
    }
}
