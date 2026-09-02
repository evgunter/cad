//! Boundary-loop walking: traversal extraction, chart inversion, and
//! periodic unwrapping — turning a curved face's loop into a UV polygon
//! whose sides are exactly straight (bitwise-constant u or v). That
//! held only for a side carried by ONE edge until #653 (this module's
//! header asserted the stronger thing for a milestone and
//! [`crate::curved`] had to discover it was false); it now holds for
//! every side, and the paragraph below says how.
//!
//! Every curved face this project's SWEEPS author is a swept UV
//! rectangle whose boundary edges are iso-curves (extrude/revolve
//! structure, PR 4/5): **rims** (circles around the surface axis,
//! v = const) and **meridians** (u = const:
//! struts/generators/profile copies/`Seam` edges). That is a fact
//! about authoring, **not** about input — an iso-bounded domain need
//! not be a rectangle (a keyway on a cylinder is bounded by lines and
//! circles and is a U), and this walk handles such a loop perfectly
//! well; it is [`crate::curved`]'s interior grid that needs the
//! rectangle, and that lane checks it
//! (`TessellateError::UnsupportedCurvedDomain`). The walk classifies
//! each traversal structurally, assigns the constant coordinate once
//! per ISO SIDE — never per point, so no side wobbles from point to
//! point and the CDT sees no sliver of that kind — and unwraps the
//! periodic coordinate(s) by continuity (chord steps ≤ π/4 make branch
//! choice unambiguous away from poles).
//!
//! **"Once per edge" was not "once per side" (issue #653, closed
//! here).** A side that IS one edge always came out bitwise straight.
//! A side carried by two or more edges — every exporter emits one the
//! moment a vertex lands mid-side, and [`topo::Body::split_edge`]
//! mints one directly — used to have each sub-edge derive its own
//! column from [`topo::chart_iso::mid_azimuth`] → [`Chart::u_of`], an `atan2` of a
//! DIFFERENT point of the same carrier. Those agree analytically, and
//! agree bitwise on axis-aligned dyadic fixtures, but under a general
//! rigid placement they differ by ulps — which is how 71 rows of a
//! 1524-row sweep returned `Ok` from `tessellate` and failed
//! `check_mesh`. [`iso_side_starts`] now groups consecutive traversals
//! into RUNS and gives each run ONE coordinate, so bitwise
//! straightness is this module's GUARANTEE rather than its intent.
//! [`crate::curved`]'s walk-consistency check still bands its comparison
//! in metres (`curved::entries_off_bbox`) — as a backstop, no longer as
//! the thing that keeps valid parts from being refused.
//!
//! One structural exception to the continuity rule: a
//! rim-anchored loop's **final** meridian contains the loop's closing
//! vertex, so its column takes the branch nearest the first polygon
//! entry (`out[0].u`) rather than continuity — the right BRANCH by
//! construction for every wedge angle (continuity would pick the wrong
//! one for θ > 3π/2, where the complement 2π − θ < π/2 is closer).
//!
//! That final column is taken from `out[0].u` ITSELF, not merely from
//! the branch nearest it, so a rim-anchored loop closes **bitwise**:
//! the closing polygon side is exactly vertical by construction, at no
//! tolerance. It did not always. The column used to come from the
//! meridian carrier's MIDPOINT (`mid_azimuth`) — the same analytic
//! azimuth down a different float path — and the gap between the two
//! was closed by an ε-scaled snap. Imported geometry can make that gap
//! nonzero (it is source-coordinate rounding, not accumulation), which
//! is what made a tolerance look necessary. Taking the vertex's own
//! value makes the quantity identically zero instead. How far the
//! carrier's own azimuth was is a fact about the SOURCE and not about
//! the mesh, and it is measured from the body by
//! `topo::coherence`'s `MeridianClosure` rather than here.
//!
//! Pole handling (chart singularities; the surface's `normal` is never
//! sampled): a pole/apex is always an edge **endpoint** (valence 2). A
//! pole junction between two meridians emits *two* polygon entries —
//! one closing the incoming meridian's column, one opening the
//! outgoing column — both mapping to the single pole mesh vertex; the
//! collapsed side between them becomes the fan (see the curved-face
//! module — that the two entries yield ONE fan rather than two
//! overlapping ones is a property of the interior grid, held by
//! `curved::pole_columns`, not of this emission; issue #678).
//! Two documented value-level resolutions (display-layer, backstopped
//! by the per-triangle certificates and the mesh validator):
//!
//! - **Tie at a pole junction** (the two meridians sit exactly π
//!   apart, e.g. a wire-case cone band): the unwrap candidates are
//!   equidistant; the branch nearest the polygon's first entry wins
//!   (closure consistency).
//! - **Pole-to-pole bands** (no rim in the loop — the sphere bands of
//!   a ball): continuity gives no anchor at all, so the loop's 3-D
//!   area vector disambiguates: it points into the face's azimuth
//!   half (interior-left + the face's OUTWARD normal; verified
//!   derivation in the PR log), and each meridian takes the branch
//!   nearest `atan2(A·v_ref, A·u_ref)`.
//!
//!   Only `A`'s DIRECTION is read, and that direction is the band's own
//!   only because the fold is anchored on the loop rather than on the
//!   world origin — see [`loop_area`], which carries that premise. A
//!   fixed anchor makes the branch pick a function of where the body
//!   was placed, and the result is a wrong half-sphere, not a refusal.
//!
//!   That derivation is stated in the outward frame while `u_ref` /
//!   `v_ref` live in the surface's CHART frame, so since M5 S10 the
//!   area vector is multiplied by the face's `sense_sign` before the
//!   `atan2` — the one orientation-sense read in this crate. The old
//!   "assumes outward-oriented shells (true of every M2 body)" caveat
//!   is thereby discharged rather than restated: a reversed face
//!   stores its loop the other way round, so `A` flips and an
//!   unmultiplied azimuth would be π off, selecting the complementary
//!   branch and meshing the wrong half of the sphere. This is the
//!   direct analogue of `geom_brep::props::curved`'s rimless-sphere
//!   `s_f` — the same face kind, the same missing bit, the same fix.
//!   Every face this build mints has `sense: true`, so the multiply is
//!   `· 1.0` and bitwise inert today.

use std::collections::HashMap;

use geom_core::{Point3, Vec3};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, LoopKey};

use crate::sizing::Eps;
use crate::types::TessellateError;

// The chart's closed forms and the boundary classification built on
// them live in `topo` (`topo::chart`, `topo::chart_iso`), where the
// body-side coherence examination runs the SAME expressions this walk
// does rather than a second copy of them. What stays here is this
// crate's own disposition of their answers: the typed refusal a
// non-iso carrier becomes, the band the separation test is read at,
// the walk's rotation and pole handling, and the emission itself.
pub(crate) use topo::chart::{Chart, ChartKind};
pub(crate) use topo::chart_iso::{TAU, TravKind, unwrap_near};

/// One directed boundary traversal: an edge's chord ids in the loop's
/// walking direction, plus its iso classification.
///
/// The KIND moved to `topo::chart_iso`; this struct did not, and the
/// `ids` field is why — a chord-id list is a mesh, and a mesh is what
/// the body-side consumer of that classification does not have.
pub(crate) struct Trav {
    /// Chord ids, traversal order (endpoints included).
    pub ids: Vec<u32>,
    /// Rim (`v = const`) or meridian (`u = const`) data.
    pub kind: TravKind,
}

/// The loop's half-edge traversal list `(edge, forward)` in `next`
/// order.
pub(crate) fn loop_edges(
    body: &Body<f64>,
    lk: LoopKey,
    face: FaceKey,
) -> Result<Vec<(EdgeKey, bool)>, TessellateError> {
    let lp = body
        .get_loop(lk)
        .ok_or(TessellateError::MissingEntity { what: "loop" })?;
    let LoopBoundary::Cycle { first } = lp.boundary else {
        return Err(TessellateError::EmptyLoop { face });
    };
    let cycle = body
        .loop_cycle(first)
        .ok_or(TessellateError::MissingEntity { what: "loop cycle" })?;
    let mut out = Vec::with_capacity(cycle.len());
    for hek in cycle {
        let he = body
            .get_half_edge(hek)
            .ok_or(TessellateError::MissingEntity { what: "half-edge" })?;
        let edge = body
            .get_edge(he.edge)
            .ok_or(TessellateError::MissingEntity { what: "edge" })?;
        out.push((he.edge, edge.he_plus == hek));
    }
    Ok(out)
}

/// Classifies and directs every traversal of a curved face's loop.
pub(crate) fn traversals(
    body: &Body<f64>,
    chart: &Chart,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    face: FaceKey,
    lk: LoopKey,
) -> Result<Vec<Trav>, TessellateError> {
    let mut out = Vec::new();
    for (ek, forward) in loop_edges(body, lk, face)? {
        let edge = body
            .get_edge(ek)
            .ok_or(TessellateError::MissingEntity { what: "edge" })?;
        let curve = body
            .get_curve_geom(edge.curve)
            .ok_or(TessellateError::MissingEntity { what: "edge curve" })?
            .certified()
            .ok_or(TessellateError::NullScaffoldEdge { edge: ek })?;
        let mut ids = chords
            .get(&ek)
            .ok_or(TessellateError::MissingEntity {
                what: "edge chords",
            })?
            .clone();
        if !forward {
            ids.reverse();
        }
        let kind = classify(chart, curve, ek)?;
        out.push(Trav { ids, kind });
    }
    Ok(out)
}

/// Rim-vs-meridian classification (module docs): `Seam` descriptions
/// and line carriers are meridians; circle carriers split on axis
/// alignment (structurally either parallel — a rim — or perpendicular
/// — a meridian; 0.5 splits the two classes with maximal margin).
///
/// A meridian's column u always comes from the mid-point chart
/// inversion — **never** from the edge kind: a `Seam` edge is the
/// surface's `u_ref`-half-plane meridian, whose chart u is 0 on
/// ordinary kinds but π on a cone's mirror nappe (the kernel defines
/// the seam spatially via `u_ref`; `u_of` carries the nappe
/// correction).
fn classify(
    chart: &Chart,
    curve: &geom_brep::EdgeCurve<f64>,
    _ek: EdgeKey,
) -> Result<TravKind, TessellateError> {
    // The classification is `topo::chart_iso::classify_kind`'s; what
    // is here is THIS crate's disposition of its `None`.
    //
    // RETIRED refusal (M5 PR 11): a conic/B-spline trim carrier routes
    // the whole face to the pcurve-driven trimmed lane BEFORE this
    // walk runs (`crate::trimmed::has_trim_carrier`), so an
    // unclassifiable carrier here is the router's backstop, not a
    // frontier — reaching one is a dispatch defect, surfaced typed.
    topo::chart_iso::classify_kind(chart, curve).ok_or(TessellateError::MissingEntity {
        what: "non-iso trim carrier reached the iso-rectangle walk (router defect)",
    })
}

/// One UV polygon entry of a curved face's boundary walk.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UvPoint {
    /// Unwrapped azimuth.
    pub u: f64,
    /// (Unwrapped, for torus) v.
    pub v: f64,
    /// The mesh vertex this UV location maps to (several entries may
    /// share an id: seam double-traversals, pole corners).
    pub id: u32,
    /// Whether this entry is a pole/apex corner.
    pub pole: bool,
}

/// The ids that appear at two or more DISTINCT UV locations among
/// `entries` — the "one mesh vertex, two parameter locations" state
/// that both tessellation lanes have to reason about, with ONE home.
///
/// **"Distinct" means what SPADE means by it**, which is why this
/// compares `f64`s and not their bits. Spade's vertex lookup is
/// `PartialEq` on `Point2<f64>` — plain `==` — so `-0.0` and `0.0` are
/// ONE spade vertex and two bit patterns. A `to_bits` compare would
/// report "apart" exactly where spade dedupes, which is an invariant
/// restated in a spelling that disagrees with the module it is about.
/// Two entries spade merges are one CDT vertex and cannot be fanned
/// apart; two it keeps can.
///
/// Both callers used to carry their own copy of that rule against
/// their own polygon type (`curved::identified_ids` over
/// [`UvPoint`], `trimmed::id_repeats_apart` over `(u, v, id)`
/// triples), one of them documenting itself AS a copy. The rule is
/// spade's, not either lane's, so it lives once and both lanes hand it
/// their entries.
pub(crate) fn ids_at_two_uvs(
    entries: impl IntoIterator<Item = (f64, f64, u32)>,
) -> std::collections::HashSet<u32> {
    let mut seen: HashMap<u32, (f64, f64)> = HashMap::new();
    let mut repeated: std::collections::HashSet<u32> = std::collections::HashSet::new();
    for (u, v, id) in entries {
        #[allow(clippy::float_cmp)]
        if seen.insert(id, (u, v)).is_some_and(|p| p != (u, v)) {
            repeated.insert(id);
        }
    }
    repeated
}

/// The undirected key of the edge `(a, b)`: endpoints ascending.
///
/// One home for the spelling every edge census in this crate uses —
/// the two in [`mod@crate::tessellate`], the pole/seam one in
/// [`crate::curved`] and [`crate::trimmed`], `planar`'s crossing
/// bookkeeping and [`crate::validate::check_mesh`]'s manifoldness
/// census. The censuses themselves are deliberately NOT unified: they
/// ask different questions (which edges to count at all, what count is
/// legal, whether winding is tracked), and folding them together would
/// state a shared conclusion they do not share. What they do share is
/// this key, and a key spelled six ways is six chances to spell it
/// wrong.
pub(crate) const fn edge_key(a: u32, b: u32) -> (u32, u32) {
    if a < b { (a, b) } else { (b, a) }
}

/// The edge incident to an IDENTIFIED vertex that this patch uses more
/// than twice, if any — the emitted form of #678's class, re-derived
/// over the emission rather than argued from the grid, for whichever
/// lane hands it a patch.
///
/// A fan edge around an identified vertex is interior to the patch and
/// used twice, or on the patch boundary and used once with the
/// neighbouring face supplying the other use. THREE or more uses in
/// ONE patch means the collapse left something other than a fan, which
/// is the non-manifold state; four is #678's own signature, and the
/// threshold is at three because three is already the state — a row
/// pins that, since `n > 3` is a mutant this census would otherwise
/// survive.
///
/// An empty `identified` set means there is nothing to re-derive and
/// the scan does not run at all: a wedge wall, an untrimmed patch or
/// any face whose walk repeats no id pays one branch.
#[cfg(debug_assertions)]
pub(crate) fn overused_identified_edge_in(
    identified: &std::collections::HashSet<u32>,
    triangles: &[[u32; 3]],
) -> Option<((u32, u32), usize)> {
    if identified.is_empty() {
        return None;
    }
    let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
    for t in triangles {
        for k in 0..3 {
            let (a, b) = (t[k], t[(k + 1) % 3]);
            if identified.contains(&a) || identified.contains(&b) {
                *uses.entry(edge_key(a, b)).or_insert(0) += 1;
            }
        }
    }
    uses.iter().find(|&(_, &n)| n > 2).map(|(&e, &n)| (e, n))
}

/// [`unwrap_near`] with half-period tie resolution toward `anchor`
/// (module docs: the wire-band pole junction, meridians exactly π
/// apart).
fn unwrap_tie(raw: f64, prev: f64, anchor: f64) -> f64 {
    let k = (prev - raw) / TAU;
    if (k - k.round()).abs() < 0.25 {
        raw + TAU * k.round()
    } else {
        let c1 = raw + TAU * k.floor();
        let c2 = raw + TAU * k.ceil();
        if (c1 - anchor).abs() <= (c2 - anchor).abs() {
            c1
        } else {
            c2
        }
    }
}

/// **This crate's ε band, and its only spelling.** Is a UV discrepancy
/// of `gap` (radians for u and for a periodic v, metres otherwise), at
/// a point whose lever arm for that axis is `lever` metres, float noise
/// rather than a defect in the geometry?
///
/// `gap * lever` is the LENGTH the two values disagree by, and `eps` is
/// a length — the only unit the two are honestly comparable in. A
/// scale-free radian constant necessarily mis-ranks features, because a
/// fixed source-coordinate error subtends a larger angle on a smaller
/// one (`topo::coherence` keeps the trace: 21 pm reads as 3.6e-9 rad
/// on a 0.117-inch hole). At `lever == 0` the coordinate carries no length
/// at all, so every gap is noise — the correct limit, reached without a
/// special case or a division. The comparison is
/// [`Eps::dominates`](crate::sizing::Eps::dominates) — a strict `<`,
/// band excluded — so a NaN gap or lever is never noise: a poisoned
/// coordinate stays a refusal (`curved`) rather than being admitted.
/// The strictness is
/// also what makes a ZERO band admit nothing, which is the exact form
/// `curved`'s band fixtures compare this predicate against.
///
/// **The two sentences above meet at one corner, and it resolves in
/// favour of the band**: at `lever == 0` AND `eps == 0` the product is
/// `0` and `0 < 0` is false, so the gap is NOT noise — the zero band
/// wins, and "every gap is noise at zero lever" is a statement about a
/// POSITIVE band, which every run has. The corner is reachable only in
/// `curved`'s exact-form fixtures, which pass a zero band deliberately
/// and assert exactly this refusal.
///
/// The lever arms are [`Chart::radial`] (u — the point's own distance
/// from the axis, so a cone and a sphere get a varying one) and
/// [`Chart::v_lever`] (v — constant per kind).
///
/// # The one consumer
///
/// [`crate::curved`]'s walk-consistency check (`entries_off_bbox`) —
/// which decides whether a face is REFUSED. #648 compared exactly
/// there, on the then-false premise that every iso side is bitwise
/// straight, and false-refused valid parts. #653 made the premise
/// true, so the band is now a backstop that no in-tree fixture needs;
/// the row that witnesses it is synthetic.
///
/// # Where the COHERENCE conditions are, which is not here
///
/// A gap between two of a BODY's own accounts of one chart coordinate
/// — a carrier's midpoint azimuth against its own endpoint vertex's,
/// two sub-edges of one iso side against each other — is measured
/// nowhere in this crate. It is a fact about the body, computable from
/// the body alone with no mesh state and no δ, and it cannot move a
/// mesh byte: `topo::coherence` states all three such conditions as a
/// non-gating findings report, with the ledger and the two π-rad
/// witnesses. A tessellator is not a lint for other people's data.
///
/// [`iso_side_starts`] is NOT a consumer either, and the distinction is
/// narrow enough to be worth stating rather than leaving to the
/// absence: it does not read THIS PREDICATE — which traversals share a
/// side is decided by kind and by a pole test, not by a gap-against-a-
/// lever band — but it does read **ε**, through
/// [`Eps::separates`](crate::sizing::Eps::separates), and its read
/// DECIDES which `f64` the entries of a side carry. A reader taking
/// the consumer above for the crate's ε ledger would be short by one:
/// this list is `gap_is_noise`'s callers and nothing wider.
/// [`crate::sizing::Eps`] carries the four operations every ε read in
/// this crate is spelled with; the inventory of where they are is
/// computed by `mesh/tests/all.rs`'s `the_eps_inventory_is_pinned`.
///
/// It is deliberately NOT named for the closure any more: it was
/// `closure_is_snappable` while a snap read it, and the closure is no
/// longer among its consumers at all.
///
/// # Its own D2 row
///
/// **Row 1**, wholly: its one consumer refuses a face typed
/// ([`TessellateError::UnsupportedCurvedDomain`]), so every read of
/// this predicate disposes of a reachable-by-input INVALID state and
/// there is no deviation to record.
pub(crate) fn gap_is_noise(gap: f64, lever: f64, eps: Eps) -> bool {
    eps.dominates(gap * lever)
}

/// Which traversals OPEN an iso side, cyclically (issue #653).
///
/// An iso side of a curved face may be carried by SEVERAL edges — a
/// vertex dropped on it by [`topo::Body::split_edge`], a boolean, or an
/// exporter emitting two collinear `EDGE_CURVE`s (which is what every
/// exporter emits when a vertex lands on that edge). Each such edge
/// derives its own constant coordinate from its own mid-parameter
/// point, so the side is straight only to ulps under a general rigid
/// placement — analytically equal, bitwise equal only on axis-aligned
/// dyadic fixtures. Grouping the edges into RUNS and giving each run
/// ONE coordinate restores the bitwise-straight side, and with it the
/// premise `curved`'s interior grid and its domain guard rest on.
///
/// # The test is structural, not a band
///
/// Two consecutive traversals belong to one iso side iff they are the
/// same kind and their shared junction vertex is a REGULAR point of the
/// chart. That is not an approximation:
///
/// - A point off the axis has exactly ONE azimuth, so two meridians
///   meeting there are necessarily co-azimuthal — there is no such
///   thing as a meridian-meridian corner away from the axis.
/// - Two coaxial circles at different `v` are disjoint, so two rims
///   meeting anywhere are necessarily co-`v`.
///
/// The only way consecutive same-kind traversals can be genuinely
/// different sides is a CHART SINGULARITY at the junction — which is
/// exactly the pole-fan corner the walk already emits two entries for,
/// and exactly the π-apart wire-band case [`unwrap_tie`] exists for.
///
/// ## The premise that argument needs, which nothing here checks
///
/// Both bullets say "meridian" and "rim" meaning ISO-CURVES of this
/// chart. [`classify`] does not establish that: a `Line` carrier is a
/// meridian unconditionally, and a `Circle` is a rim iff
/// `|n · axis| > 0.5` — a cone of directions, not coaxiality. So the
/// real premise is **every boundary edge is an iso-curve of this
/// chart**, and it is inherited from upstream rather than verified.
/// Per kind:
///
/// - **cylinder, cone** — safe by geometry. A circle on either surface
///   whose axis is within 60° of the chart's IS coaxial.
/// - **torus** — leaks harmlessly. A Villarceau circle classifies as a
///   Rim whenever `minor/major < 0.866` and is not coaxial, but its
///   centre lies in the equatorial plane, so [`Chart::rim_v`] reads
///   `0.0` for every one of them and merging a pair lands on the value
///   they already shared.
/// - **sphere** — the real leak. EVERY plane section of a sphere is a
///   [`Curve3::Circle`], and `trimmed::has_trim_carrier` diverts only
///   `Ellipse` and `Nurbs`, so an obliquely-cut sphere face would
///   arrive here carrying two non-iso circles. Two consecutive such
///   arcs meeting off-axis get collapsed onto ONE coordinate.
///
/// That last one is a severity FLIP, not a wash, and it is worth being
/// explicit: per-edge, the two sub-arcs took two different wrong
/// coordinates and the polygon would very likely have failed
/// [`crate::curved`]'s spatial check as a typed
/// `UnsupportedCurvedDomain`; collapsed, it can BE its own bounding
/// rectangle and be admitted. The #653 sweep cannot see this — it
/// measures straightness and watertightness, not whether two sides
/// were correctly distinguished. **Executed, not argued**: an
/// obliquely cut sphere assembled through the Euler doors (the one
/// route no certification fronts) walked to a polygon on ONE `v` and
/// passed the spatial check; with debug assertions on, `tessellate`
/// panicked at the S65 cross-face census (a chord segment used by no
/// triangle); with them off it returned `Ok` on an EMPTY mesh — 12
/// positions, two patches of 0 triangles — which `check_mesh` PASSES,
/// having nothing to find non-manifold
/// (`curved::tests::the_lens_walk_collapses_onto_one_rim_level_and_
/// the_spatial_check_admits_it`, `tests/iso_rectangle_door.rs`).
///
/// **CLOSED AS WORDED, by the shape door — and only as worded.** The
/// case this qualification instanced is two consecutive same-kind
/// traversals whose CARRIERS are not iso curves, and that is what
/// `geom_brep::props::require_iso_rectangle` refuses per edge, on
/// every kind, before `curved::tessellate_curved` walks a face: the
/// oblique section on `props_rim_axis_parallel`, a torus Villarceau
/// circle on `props_rim_fit`. **The premise itself is NOT established
/// by that door.** Props certifies the carrier — a great circle
/// through the sphere centre — not that the traversed ARC stays on one
/// chart meridian; a great circle contains both poles, so one meridian
/// arc can cross a pole mid-edge, where `u` jumps by π, and such a
/// face passes the door on every face and measures its exact volume
/// (issue 1571, executed: `tests/mesh7r1_probes.rs`, pinned in
/// `tests/iso_rectangle_door.rs`). The sentence above this list —
/// "inherited from upstream rather than verified" — therefore still
/// stands; what changed is that the two-non-iso-carriers instance is
/// refused typed, and the upstream gates that kept it unreachable
/// (the boolean's typed refusal of every sphere-face cut, tier 3 on
/// the import route) are no longer what that instance rests on. The
/// cheap hardening once recorded here (a coaxiality test in
/// `classify`) is not taken: the door decides coaxiality at props'
/// band; what it does not decide, arc membership, is issue 1571's.
///
/// # One test for every singularity — of the run-breaking DECISION
///
/// Every chart singularity lies ON THE AXIS — a sphere's poles, a
/// cone's apex — so `radial(junction) > eps` covers them with one
/// comparison rather than a match on [`ChartKind`], and in particular
/// without consulting [`Chart::poles`].
///
/// "One comparison for all of them" is a claim about the DECISION MADE
/// HERE and not about pole HANDLING downstream. [`Chart::poles`] is
/// empty for `Torus`, so `loop_polygon`'s `pole_v` returns `None` at a
/// toroidal axis point and the walk emits one ordinary entry instead
/// of the two-entry fan. Dormant — `revolve` refuses horn and spindle
/// at construction, and `topo::validate`'s tier-3 `DegenerateTorus`
/// covers the import door that reads both radii verbatim — but it is a
/// different question from this one and this function does not answer
/// it.
///
/// **Not consulting `poles()` is deliberate but is NOT tested, because
/// this build cannot construct the case that would separate them.**
/// The one chart singularity `poles()` does not list is the axis point
/// of a horn or spindle torus (`major + minor·cos v` vanishing), which
/// `revolve` refuses at construction (`sweep::revolve`'s profile check —
/// the sweep's own skip lines show it) and `topo::validate` reports at
/// rest whatever door minted it. So a `poles()`-based test would behave
/// identically on every body this build can mint. The radial test is
/// chosen because it is the smaller thing to state, not because a live
/// case demands it; if
/// horn tori ever become constructible, this line already covers them,
/// and that is a bonus, not evidence. The unit row below exercises a
/// SPHERE chart, which is the only pole case there is here.
///
/// The bar is a LENGTH, as everywhere else in this module: within ε of
/// the axis an azimuth carries no distinguishable direction, so the
/// answer there is to break the run and keep the per-edge coordinate.
///
/// **In WHICH DIRECTION that is conservative, because the word alone
/// overstates it.** Breaking a run can never merge two genuinely
/// different sides, which is the failure that would corrupt a mesh
/// silently in a new way. It is NOT refusal-safe. If the bar breaks a
/// run at a LEGITIMATE boundary vertex — one that happens to sit
/// within ε of the axis — then `starts[k]` is `true` there and that
/// junction gets exactly main's per-edge assignment: the side stops
/// being bitwise straight, `curved::entries_off_bbox` is banded and
/// will not refuse it, and `tessellate` does not run `check_mesh`. The
/// fallback therefore **reinstates #653 for that face rather than
/// refusing it** — an unfixed residue, not a regression, since the
/// walk for that junction is byte-for-byte what shipped before #653.
///
/// Nothing has reached it: swept across cone slopes 1 … 1e-5, split
/// distances 1e-9 … 1e-3 from both ends of every line edge, ε in
/// {1e-9, 1e-7, 1e-3}, identity and oblique placements — no junction
/// ever landed at `0 < radial <= eps`. `split_edge`'s interiority gate
/// is metred against the same band, so it will not drop a vertex
/// within ~ε of an endpoint, and `revolve` refuses near-horn toroids
/// at a certified bar. The route with NO such gate, and so the
/// settling experiment nobody has run, is import: a STEP fixture
/// stating a cone face whose generator is two collinear `EDGE_CURVE`s
/// meeting within ε of the apex — the same door `split_oblique.step`
/// walks through.
///
/// # What this cannot change
///
/// On a loop whose every iso side is one edge, consecutive traversals
/// always differ in kind or meet at a pole, so EVERY entry is `true`
/// and every rule downstream is the one that ran before #653. That is
/// why unsplit bodies mesh bitwise as they did.
fn iso_side_starts(
    travs: &[Trav],
    chart: &Chart,
    positions: &[Point3<f64>],
    eps: Eps,
) -> Vec<bool> {
    // THE BAND READ IS THIS CRATE'S and stays here: `Eps` owns every
    // terminal ε read in `mesh` and deliberately has no accessor, so
    // the separation ANSWER is what crosses the crate boundary, never
    // the band. The rule those answers feed —
    // same kind ∧ separated, cyclically — is
    // `topo::chart_iso::iso_side_starts`, in one home, and everything
    // this function's docs argue above is about what the walk then
    // does with it.
    let separated: Vec<bool> = travs
        .iter()
        .map(|t| eps.separates(chart.radial(positions[t.ids[0] as usize])))
        .collect();
    let kinds: Vec<TravKind> = travs.iter().map(|t| t.kind).collect();
    topo::chart_iso::iso_side_starts(&kinds, &separated)
}

/// Where the walk starts (index into `travs`), or `None` to leave the
/// cycle as it is.
///
/// A rim if the loop has one — that much predates #653 and is what
/// makes the closure rule apply to a MERIDIAN. The refinement is
/// "among rims, one that OPENS its row": a run that wrapped past index
/// 0 would have its first traversal marked a continuation with nothing
/// before it to continue from, and the `starts[0] = true` patch in
/// [`loop_polygon`] would then split that run in the middle of a side.
///
/// # The failing condition, named
///
/// This can only differ from the `position(Rim)` it replaces when the
/// loop's FIRST rim is a continuation — i.e. some rim row is carried by
/// several edges AND the cycle happens to begin inside it. On an
/// unsplit loop every rim opens its row and this returns the same index
/// the old form did, so nothing rotates differently. Rows:
/// `the_walk_anchors_on_a_rim_that_opens_its_row`.
///
/// The `or_else` fallback (any rim at all) is for the degenerate loop
/// whose rims form a single cyclic run with no opening — unreachable
/// through `traversals`, since such a loop has no meridian and so no
/// junction the run could break at, but stated rather than indexed
/// blind.
fn walk_anchor(travs: &[Trav], starts: &[bool]) -> Option<usize> {
    let has_rim = travs.iter().any(|t| matches!(t.kind, TravKind::Rim { .. }));
    if has_rim {
        (0..travs.len())
            .find(|&k| matches!(travs[k].kind, TravKind::Rim { .. }) && starts[k])
            .or_else(|| {
                travs
                    .iter()
                    .position(|t| matches!(t.kind, TravKind::Rim { .. }))
            })
    } else {
        // No rim: the walk did not rotate before #653 and still does
        // not on an unsplit loop, where `starts[0]` is already true.
        (0..travs.len()).find(|&k| starts[k])
    }
}

/// The index that OPENS the walk's last iso side — the side carrying
/// the loop's closing traversal, and so the side the closure rule and
/// the pole-band seed both belong to.
///
/// `m - 1` on an unsplit loop, where every traversal opens its own
/// side. It differs exactly when the LAST iso side is carried by
/// several edges, and that is the condition both of its consumers need:
///
/// - `loop_polygon`'s meridian arm takes the closing vertex's own
///   azimuth at `k == closing_side`, not at `k == m - 1`, so the rule
///   belongs to the side; the continuations after it repeat that
///   column bitwise.
/// - the pole-to-pole band seeds `prev_u` with `band_u[closing_side]`.
///   Seeding with `band_u[m - 1]` — the last traversal's OWN value —
///   was wrong for a rimless band whose last iso side is carried by
///   several edges: the last traversal emits its RUN's column, so the
///   pole entry that closes the loop at `k == 0` carried a neighbour's
///   value instead. Measured on the ball, that put one walk 4.4e-16 m
///   off its own UV bounding box.
///
/// Rows: `the_closing_side_is_the_last_run_start_not_the_last_index`.
fn closing_side(starts: &[bool]) -> usize {
    (0..starts.len()).rev().find(|&k| starts[k]).unwrap_or(0)
}

/// The 3-D vector area of the closed cycle `pts`,
/// `Σᵢ (pᵢ − o) × (pᵢ₊₁ − o)`, with `o` the points' own bbox centre.
///
/// The fold wraps (`i + 1` mod `n`), so the cycle is closed by
/// construction and the sum is anchor-independent over ℝ: an anchor's
/// extra terms telescope to `o × Σᵢ(pᵢ₊₁ − pᵢ)`, and that displacement
/// sum is zero around a cycle. The anchor is therefore a CONDITIONING
/// choice and not a value one, and a position-derived anchor is the
/// one that keeps every operand at the loop's own scale: a fixed
/// anchor pays cancellation in proportion to the loop's distance from
/// it, which the direction-only consumer in [`loop_polygon`] reads as
/// a rotated — eventually reversed — area vector. Overflow-robust
/// midpoint, the spelling `validate::signed_volume` uses.
///
/// An empty cycle has no anchor and no area.
fn loop_area(pts: &[Point3<f64>]) -> Vec3<f64> {
    let Some(&first) = pts.first() else {
        return Vec3::new(0.0, 0.0, 0.0);
    };
    let (lo, hi) = pts
        .iter()
        .fold((first, first), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let o = lo + (hi - lo) * 0.5;
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, p) in pts.iter().enumerate() {
        let q = pts[(i + 1) % pts.len()];
        area = area + (*p - o).cross(q - o);
    }
    area
}

/// Walks a curved face's loop into its UV polygon (module docs: the
/// classification, unwrapping, pole, and disambiguation rules).
///
/// The order of business, because the function is long and each step
/// depends on the one before:
///
/// 1. [`traversals`] — one entry per boundary edge, classified `Rim`
///    or `Meridian` with its raw constant coordinate.
/// 2. [`iso_side_starts`] — which traversals OPEN an iso side (#653).
///    Everything after this reads `starts`, not the edge list.
/// 3. [`walk_anchor`] + `rotate_left` — put index 0 on a side that
///    opens, preferring a rim so the closure rule applies to a
///    meridian. `starts[0]` is then forced `true`.
/// 4. [`closing_side`] — the opening index of the LAST iso side, which
///    is what the closure rule and the pole-band seed key on.
/// 5. `band_u` — for a rimless pole-to-pole band only, every column
///    precomputed from the loop's 3-D area vector.
/// 6. the emission loop — per traversal, take the run's coordinate (a
///    continuation repeats it bitwise, discarding its own) and unwrap
///    the other axis by continuity.
/// 7. the closure assertion — a revert detector, not a runtime guard.
pub(crate) fn loop_polygon(
    body: &Body<f64>,
    chart: &Chart,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    positions: &[Point3<f64>],
    face: FaceKey,
    lk: LoopKey,
    eps: Eps,
) -> Result<Vec<UvPoint>, TessellateError> {
    let mut travs = traversals(body, chart, chords, face, lk)?;
    let m = travs.len();
    // ISO-SIDE RUNS (#653): which traversals open a side, and so take
    // a fresh constant coordinate rather than the running one.
    let mut starts = iso_side_starts(&travs, chart, positions, eps);
    let has_rim = travs.iter().any(|t| matches!(t.kind, TravKind::Rim { .. }));
    let anchor_at = walk_anchor(&travs, &starts);
    if let Some(start) = anchor_at {
        travs.rotate_left(start);
        starts.rotate_left(start);
    }
    // Traversal 0 opens the walk whatever the cycle looked like: it has
    // no predecessor to continue. (Only reachable as a `false` in the
    // degenerate case where the whole loop is one cyclic run and no
    // rotation could open it — then this is the pre-#653 behaviour for
    // that one junction, which is no worse than before.)
    if let Some(first) = starts.first_mut() {
        *first = true;
    }
    let closing_side = closing_side(&starts);
    let no_rim = !has_rim;
    // The face's S10 orientation sense as a `±1` (module docs, the
    // pole-to-pole band). Read once here; consumed at exactly one site
    // below.
    let sense_sign: f64 = body
        .get_face(face)
        .ok_or(TessellateError::MissingEntity { what: "face" })?
        .sense_sign();
    // JUNCTIONS ONLY: `t.ids[0]` is a topology vertex's mesh id
    // (`chords::compute_chords` takes it from `vids`), so it is a
    // DECLARED vertex; `ids[1..len - 1]` are chord subdivision points,
    // whose spacing is delta-driven by design and drops under any eps
    // at a fine enough delta. Comparing the whole id set would fire on
    // correct input.
    //
    // D2 addendum row 5. Two vertices declared separately and placed
    // within eps of each other can only be read as one by assuming
    // intent from a numerical coincidence, which this project never
    // does; so reaching this state is very likely a kernel bug (Evan's
    // conjecture, #884). It is not observable in a branch — seeing it
    // takes a re-derivation over every junction PAIR — which is what
    // makes it a `debug_assert` and not an `unreachable!`.
    //
    // SCOPE, because a near neighbour is easy to mistake for this one:
    // this compares DECLARED vertex against DECLARED vertex. The pole
    // classification below compares a junction against an UNDECLARED
    // analytic chart point (`Chart::poles()` computes it from the
    // surface), and where the pole carries no vertex of its own this
    // guard cannot see it. That comparison has its own row-5 guard
    // beside `pole_v` below (issue 896), scoped to junctions the
    // classification does NOT identify with a pole.
    //
    // Equal ids are skipped rather than compared: one declared vertex
    // visited twice by one loop (a seam walked both ways) is that
    // vertex at distance 0, and is legal.
    let coincident_declared = || -> Option<(u32, u32, f64)> {
        for (i, a) in travs.iter().enumerate() {
            for b in &travs[i + 1..] {
                let (ja, jb) = (a.ids[0], b.ids[0]);
                if ja == jb {
                    continue;
                }
                let d = (positions[ja as usize] - positions[jb as usize]).norm();
                if eps.coincident(d) {
                    return Some((ja, jb, d));
                }
            }
        }
        None
    };
    debug_assert!(
        coincident_declared().is_none(),
        "face {face:?} loop {lk:?}: two DECLARED vertices lie within eps {eps} of each \
         other (id, id, separation in metres) {:?}. Intent is never read from a \
         numerical coincidence, so this is a kernel bug and not a re-declaration.",
        coincident_declared()
    );
    let poles = chart.poles();
    // ONE home for the pole-membership find: the classification
    // (`pole_v`) and the row-5 guard below both consume this index,
    // so they cannot disagree about which pole a junction is
    // identified with — the equivalence is structural, not prose.
    // The band edge is INCLUSIVE (`Eps::coincident`): a junction
    // exactly ε from a pole is identified with it.
    let pole_index = |id: u32| -> Option<usize> {
        let p = positions[id as usize];
        poles
            .iter()
            .position(|&(pp, _)| eps.coincident((p - pp).norm()))
    };
    let pole_v = |id: u32| -> Option<f64> { pole_index(id).map(|ix| poles[ix].1) };
    // D2 addendum row 5, beside the declared-vertex guard above, and
    // the case that guard's SCOPE paragraph names as out of its reach
    // (issue 896): a chart pole is an UNDECLARED analytic point
    // (`Chart::poles` computes it from the surface), so a junction
    // coinciding with one is invisible to any declared-vs-declared
    // comparison. The invariant: a junction the emission below will
    // carry as `pole: false` lies within eps of no chart pole. Inside
    // that band the coordinate the entry carries (the vertex's own
    // measured v) and the entry count (one, not a fan seed) can only
    // be read as correct by taking a numerical coincidence for
    // intent, which this project never does — so the state is a
    // kernel bug, and it is not observable in a branch: seeing it
    // takes this re-derivation over every junction × pole pair.
    //
    // The band's identified side is NOT asserted on: a meridian
    // junction within eps of the pole it is identified with is the
    // classification working as specified (the pole's exact v is
    // substituted and the fan seed emitted), and whether that
    // junction is REALLY the pole is precisely the intent question
    // the project refuses to ask. What this leaves uncaught is a
    // misidentification in the identified direction; what it
    // catches is every junction × pole pair the classification
    // passes over — a rim junction on a pole (a zero-radius row
    // stated as an ordinary one), and a junction within eps of a
    // SECOND pole beyond the one it is identified with (a
    // degenerate chart).
    //
    // JUNCTIONS ONLY, for the same load-bearing reason as the guard
    // above: chord interior points on a pole-incident edge approach
    // the pole legitimately as delta shrinks, so comparing them
    // would fire on correct input.
    //
    // REACHABILITY (the issue-896 fixture question): measured shut
    // at every minting door probed, with the route argument's single
    // home in `step-import/tests/poleguard.rs` (tier_gate points
    // there too) — the sketch: EITHER firing branch forces a
    // sub-band boundary feature (a rim junction inside the band
    // forces a rim of radius <= eps; the second-pole branch forces a
    // sphere of radius <= eps, its poles being 2r apart), so some
    // edge measures at most 2π·eps, which the ratified K = 10 span
    // certification cannot clear; below K = 2π the span argument
    // voids and the import door is MEASURED to shift to the adoption
    // transversality bar, not to open. Construction doors:
    // `mesh/tests/issue896_pole_guard.rs` (revolve, profile) and the
    // adopted review rows (plane split and boolean, both refusing
    // sphere-face cuts typed at every probed rho). Doors named
    // unmeasured, and what would open a route — a consumer
    // assembling a Body through the Euler doors directly, which no
    // certification fronts — live in the route argument's home; the
    // Euler residue is why this guard exists and runs in release
    // builds. The firing itself is demonstrated at the mechanism:
    // `tests::a_rim_junction_inside_the_pole_band_trips_the_guard`.
    #[cfg(debug_assertions)]
    {
        let mut offending: Option<(u32, Point3<f64>, f64, f64)> = None;
        'guard: for t in &travs {
            let jid = t.ids[0];
            let p = positions[jid as usize];
            let identified = matches!(t.kind, TravKind::Meridian { .. })
                .then(|| pole_index(jid))
                .flatten();
            for (ix, &(pp, pv)) in poles.iter().enumerate() {
                let gap = (p - pp).norm();
                if eps.coincident(gap) && identified != Some(ix) {
                    offending = Some((jid, pp, pv, gap));
                    break 'guard;
                }
            }
        }
        debug_assert!(
            offending.is_none(),
            "face {face:?} loop {lk:?}: a junction lies within eps {eps} of a chart pole \
             it is not being identified with (junction id, pole point, pole v, gap in \
             metres) {offending:?}. Intent is never read from a numerical coincidence, \
             so this is a kernel bug and not a near-pole vertex to admit."
        );
    }
    // Pole-to-pole bands: precompute every column from the loop's 3-D
    // area vector (module docs).
    let band_u: Option<Vec<f64>> = if no_rim {
        let pts: Vec<Point3<f64>> = travs
            .iter()
            .flat_map(|t| t.ids[..t.ids.len() - 1].iter())
            .map(|&id| positions[id as usize])
            .collect();
        // Only the DIRECTION of `area` is read below, and only a
        // loop-local anchor makes that direction the loop's own rather
        // than its placement's ([`loop_area`]).
        let area = loop_area(&pts);
        // CATEGORY A (S10). `area` is the loop's 3-D vector area, so it
        // points along the face's OUTWARD normal side — but it is read
        // here as a direction in the CHART frame (`u_ref`/`v_ref`), to
        // pick which azimuth half the band occupies. Those two frames
        // differ by exactly `sense_sign`: a reversed face stores its
        // loop the other way round, `area` flips, and the raw `atan2`
        // would land π off — selecting the complementary meridian
        // branch and meshing the wrong half of the sphere. Multiplying
        // recovers the chart-frame azimuth for either sense. This is
        // NOT the double-count hazard that forbids a multiply in
        // `planar`/`curved`: nothing downstream re-derives this sign
        // from the winding — `mid_az` only chooses a `2πk` branch, and
        // the polygon's own winding (which does flip with the sense) is
        // consumed separately by `curved`'s `flip`.
        let chart_area = area * sense_sign;
        let mid_az = chart_area
            .dot(chart.v_ref)
            .atan2(chart_area.dot(chart.u_ref));
        Some(
            travs
                .iter()
                .map(|t| match t.kind {
                    TravKind::Meridian { u_raw } => unwrap_near(u_raw, mid_az),
                    TravKind::Rim { .. } => f64::NAN,
                })
                .collect(),
        )
    } else {
        None
    };

    let mut out: Vec<UvPoint> = Vec::new();
    // The value the FINAL traversal will emit, so the pole entry that
    // closes it at `k == 0` carries that column and not a neighbour's.
    // On a pole-to-pole band the last traversal takes its RUN's column,
    // which is the run start's band value — `m - 1` on an unsplit loop,
    // and `closing_side` in the failing case: a rimless pole-to-pole
    // band whose last iso side is carried by several edges. See
    // `closing_side`'s docs for the mechanism and the measurement.
    let mut prev_u = match &band_u {
        Some(us) => us[closing_side],
        None => f64::NAN,
    };
    let mut prev_v = f64::NAN;
    let mut prev_was_rim = false;
    for (k, cur) in travs.iter().enumerate() {
        let jid = cur.ids[0];
        let jpole = pole_v(jid);
        match cur.kind {
            TravKind::Rim { v_raw } => {
                let v_own = if chart.v_periodic() && k > 0 {
                    unwrap_near(v_raw, prev_v)
                } else {
                    v_raw
                };
                let v_edge = if starts[k] {
                    v_own
                } else {
                    // Same iso side as the previous traversal: ONE row
                    // for the whole side, bitwise (#653). The two
                    // values are the same analytic v down two float
                    // paths (two carrier circles' centres), so the
                    // difference the substitution discards is a
                    // statement about the INPUT and not about the mesh,
                    // which is the same row either way. It is measured
                    // where such statements belong, from the body:
                    // `topo::coherence`'s `RimContinuation`.
                    prev_v
                };
                let ju = if k == 0 {
                    chart.u_of(positions[jid as usize])
                } else if prev_was_rim {
                    unwrap_near(chart.u_of(positions[jid as usize]), prev_u)
                } else {
                    prev_u
                };
                out.push(UvPoint {
                    u: ju,
                    v: v_edge,
                    id: jid,
                    pole: false,
                });
                prev_u = ju;
                for &id in &cur.ids[1..cur.ids.len() - 1] {
                    let u = unwrap_near(chart.u_of(positions[id as usize]), prev_u);
                    out.push(UvPoint {
                        u,
                        v: v_edge,
                        id,
                        pole: false,
                    });
                    prev_u = u;
                }
                prev_v = v_edge;
                prev_was_rim = true;
            }
            TravKind::Meridian { u_raw } => {
                let ut = if starts[k] {
                    match &band_u {
                        Some(us) => us[k],
                        None => {
                            // `band_u.is_none()` ⟺ `!no_rim` (`no_rim`
                            // is computed AFTER the `rotate_left`), so
                            // this arm implies `travs[0]` is a `Rim`,
                            // which implies `k >= 1` — a meridian is
                            // never `travs[0]` here — and the rim arm
                            // above pushes UNCONDITIONALLY. So `out` is
                            // non-empty on every path that reaches this
                            // line.
                            //
                            // Stated as `unreachable!` rather than as a
                            // default: D2 addendum row 4, a kernel bug
                            // the code can observe in a branch. A dead
                            // branch is still a place to state the
                            // invariant.
                            let Some(first) = out.first() else {
                                unreachable!(
                                    "a rim-anchored loop reaches its meridians with a non-empty polygon"
                                )
                            };
                            let anchor = first.u;
                            if k == closing_side {
                                // Final ISO SIDE: its column contains
                                // the loop's closing vertex (`out[0]`
                                // lies on this meridian plane), so the
                                // column is that vertex's own azimuth,
                                // EXACTLY — `anchor` itself, bitwise,
                                // whatever skew the source geometry
                                // carries. That is what makes the loop
                                // close bitwise (asserted after the
                                // walk) and the closing polygon side
                                // exactly vertical before the CDT sees
                                // it as a constraint. `u_raw` is
                                // deliberately unread here: taking the
                                // vertex's own azimuth one level up is
                                // what makes the closure residue
                                // identically zero, so there is no
                                // branch to select and no snap to
                                // absorb. `closing_side` is `m - 1`
                                // unless that side is carried by several
                                // edges, in which case the rule belongs
                                // to the SIDE and not to its last edge;
                                // the continuations below then repeat
                                // this column bitwise, so the closure
                                // assertion after the loop still holds.
                                anchor
                            } else {
                                unwrap_tie(u_raw, prev_u, anchor)
                            }
                        }
                    }
                } else {
                    // Same iso side as the previous traversal: ONE
                    // column for the whole side, bitwise (#653). This
                    // is the substitution the loop closure makes at the
                    // seam, one level over — the sub-edges are each
                    // other's float-path twins the way the closure's
                    // two paths are. `u_raw` is the twin discarded
                    // here, and how far it was is a statement about the
                    // INPUT and not about the mesh, which is the same
                    // column either way. It is measured where such
                    // statements belong, from the body:
                    // `topo::coherence`'s `MeridianContinuation`.
                    prev_u
                };
                if let Some(vp) = jpole {
                    // Close the incoming column, open the outgoing one.
                    out.push(UvPoint {
                        u: prev_u,
                        v: vp,
                        id: jid,
                        pole: true,
                    });
                    out.push(UvPoint {
                        u: ut,
                        v: vp,
                        id: jid,
                        pole: true,
                    });
                    prev_v = vp;
                } else {
                    let jv = if prev_was_rim {
                        prev_v
                    } else {
                        let v = chart.v_of(positions[jid as usize]);
                        if chart.v_periodic() && k > 0 {
                            unwrap_near(v, prev_v)
                        } else {
                            v
                        }
                    };
                    out.push(UvPoint {
                        u: ut,
                        v: jv,
                        id: jid,
                        pole: false,
                    });
                    prev_v = jv;
                }
                prev_u = ut;
                for &id in &cur.ids[1..cur.ids.len() - 1] {
                    let v_raw_pt = chart.v_of(positions[id as usize]);
                    let v = if chart.v_periodic() {
                        unwrap_near(v_raw_pt, prev_v)
                    } else {
                        v_raw_pt
                    };
                    out.push(UvPoint {
                        u: ut,
                        v,
                        id,
                        pole: false,
                    });
                    prev_v = v;
                }
                prev_was_rim = false;
            }
        }
    }
    // CLOSURE, exact by construction. If the walk ends in a meridian,
    // `out[0]` is that column's junction — and the meridian arm took
    // the column from `out[0].u` itself, so the two agree bitwise and
    // the closing polygon side is exactly vertical.
    //
    // WHAT THIS REPLACES (S22, and the ε-vs-δ question it asked). The
    // column used to come from the carrier midpoint, so the closing
    // side's two ends were one analytic azimuth down two float paths;
    // the difference was SNAPPED away here when `residue * radius <
    // eps`. That snap was this crate's second structural read of ε,
    // and it made the mesh a function of (body, δ, ε) against `lib.rs`
    // and D9's claim of (body, δ). Whether the bar should have been ε
    // or δ is moot: taking the vertex's own azimuth one level up makes
    // the residue identically zero.
    //
    // WHAT THIS ASSERTION IS: a REVERT DETECTOR, not a runtime guard.
    // the closing column IS `anchor`, which is `out[0].u`, and nothing
    // mutates `out[0]` afterwards — so this compares `out[0].u` with
    // itself and CANNOT go red for any input, however defective.
    // What reds it is a SOURCE EDIT that puts the column back on a
    // derived value, and it does that job well: putting
    // `unwrap_near(u_raw, anchor)` back in that arm reds this on
    // #648's obliquely-placed split frustum wedge at 1 ulp, through the
    // whole-body path rather than through a unit row written for it.
    // Keep it as the acceptance criterion made executable; do not read
    // it as evidence that the closure is being CHECKED at runtime.
    if !no_rim && matches!(travs[m - 1].kind, TravKind::Meridian { .. }) && !out.is_empty() {
        debug_assert_eq!(
            out[0].u, prev_u,
            "a rim-anchored loop must close BITWISE: the final meridian's column is \
             the closing vertex's own azimuth, so these are the same f64"
        );
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom::Surface;

    /// A chart about +z, anchored at the origin.
    fn z_chart(kind: ChartKind) -> Chart {
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        Chart {
            axis,
            u_ref,
            v_ref: axis.cross(u_ref),
            anchor: Point3::new(0.0, 0.0, 0.0),
            kind,
        }
    }

    /// The bar is SPATIAL. The gap that falsified the old bare-radian
    /// form — `nist_ftc_09_asme1_rd.stp` closes at 3.56e-9 rad, over the
    /// old 1e-9 constant — reads as noise at any real lever arm, and
    /// only fails to at an absurd one.
    #[test]
    fn the_closure_bar_is_spatial_not_angular() {
        let eps = Eps::exactly(3.38e-5);
        let gap = 3.56e-9;
        assert!(
            gap_is_noise(gap, 0.05, eps),
            "3.56e-9 rad at 50 mm displaces ~1.8e-10 m — far under eps"
        );
        assert!(
            !gap_is_noise(gap, 1e5, eps),
            "the same gap must NOT pass at a 100 km lever arm"
        );
    }

    /// Growing the lever arm tightens the angular bar proportionally —
    /// the property a bare radian constant did not have.
    #[test]
    fn the_closure_bar_tightens_as_the_lever_arm_grows() {
        let eps = Eps::exactly(1e-5);
        let gap = 1e-6;
        assert!(gap_is_noise(gap, 1.0, eps), "1e-6 m < eps");
        assert!(
            !gap_is_noise(gap, 100.0, eps),
            "1e-4 m > eps — the same angle, ten thousand times the arc"
        );
    }

    /// On the axis the azimuth carries no length, so every gap is
    /// noise. Must be a plain comparison, not a NaN or a division.
    #[test]
    fn on_the_axis_every_gap_is_noise() {
        assert!(gap_is_noise(core::f64::consts::PI, 0.0, Eps::exactly(1e-9)));
    }

    /// **THE TWO SPELLINGS OF THE BAND AGREE, rung by rung.**
    ///
    /// The band has two spellings and cannot have one:
    /// [`gap_is_noise`] here reads it through [`Eps::dominates`], the
    /// newtype MESH-4 made `mesh`-local and gave no accessor, so its
    /// value cannot cross a crate boundary as a number;
    /// `topo::coherence::gap_is_noise` takes the run's ε as an `f64`
    /// because a body-side examination has no `Eps` to read. Two
    /// spellings of one comparison is exactly the shape that drifts,
    /// and prose reconciling them is not a mechanism.
    ///
    /// This is the mechanism: every rung of this file's own ladder —
    /// the spatial-vs-angular pair, the lever-arm scaling pair, the
    /// zero-lever limit, both band edges, and a poisoned operand on
    /// each side — run through BOTH spellings and compared. A change
    /// to either that moves one and not the other reds here, whichever
    /// crate it was made in.
    #[test]
    fn the_two_spellings_of_the_band_agree() {
        const LADDER: [(f64, f64, f64); 10] = [
            // `nist_ftc_09`'s residue at a real lever and an absurd one
            (3.56e-9, 0.05, 3.38e-5),
            (3.56e-9, 1e5, 3.38e-5),
            // the same angle, a hundred times the arc
            (1e-6, 1.0, 1e-5),
            (1e-6, 100.0, 1e-5),
            // the zero-lever limit, and the corner where a zero band
            // meets it
            (core::f64::consts::PI, 0.0, 1e-9),
            (core::f64::consts::PI, 0.0, 0.0),
            // both band edges: exactly on it, and just inside
            (1.0, 1e-9, 1e-9),
            (1.0, 1e-9, 1.000_001e-9),
            // a poisoned operand on either side is never noise
            (f64::NAN, 1.0, 1e-9),
            (1.0, f64::NAN, 1e-9),
        ];
        for (gap, lever, band) in LADDER {
            assert_eq!(
                gap_is_noise(gap, lever, Eps::exactly(band)),
                topo::coherence::gap_is_noise(gap, lever, band),
                "the two spellings of the band disagree at \
                 (gap {gap}, lever {lever}, eps {band})"
            );
        }
    }

    /// **[`unwrap_near`] lands within half a period of its anchor, for
    /// any raw whatever.** Two live consumers rest on that and neither
    /// can state it: the walk's wedge rule for θ > 3π/2 needs the
    /// branch nearest `prev` and no other, and
    /// `topo::coherence::wrapped` reads this same function as a
    /// MEASUREMENT — the gap it reports is `|unwrap_near(a, b) − b|`,
    /// which is a fact about the data only while the selection cannot
    /// exceed π. A selector that overshot by a turn would make that
    /// examination report 2π-sized gaps on bodies that agree exactly.
    ///
    /// `skew` is an ABSOLUTE radian offset and not a count of ulps,
    /// because the row does not depend on the count: its job is to be
    /// a real difference, and the non-vacuity counter below is what
    /// checks that. The raws are a whole number of turns from the
    /// anchor plus a hair inside the branch, the shape imported
    /// geometry produces.
    #[test]
    fn unwrap_near_lands_within_half_a_period_of_its_anchor() {
        let mut skewed = 0_u32;
        const E: f64 = f64::EPSILON;
        for anchor in [0.0, 0.7, -2.4, core::f64::consts::PI, 5.9] {
            for turns in [-2.0_f64, -1.0, 0.0, 1.0, 2.0] {
                for skew in [0.0_f64, E, -E, 4096.0 * E, -4096.0 * E] {
                    let raw = anchor + turns * TAU + skew;
                    let carrier = unwrap_near(raw, anchor);
                    assert!(
                        (carrier - anchor).abs() < TAU / 2.0,
                        "the selection must stay inside the anchor's own branch \
                         (anchor {anchor}, turns {turns}, skew {skew} rad)"
                    );
                    assert!(
                        (topo::chart_iso::unwrap_near(raw, anchor) - anchor).abs() <= TAU / 2.0,
                        "and so must the measurement `coherence::wrapped` takes \
                         through it (anchor {anchor}, turns {turns}, skew {skew})"
                    );
                    if carrier != anchor {
                        skewed += 1;
                    }
                }
            }
        }
        // Non-vacuity: the selection really does differ from the anchor
        // on this fixture, so the rows above are live comparisons rather
        // than an accident of exact arithmetic.
        assert!(
            skewed >= 20,
            "fixture must exercise real skews; only {skewed} rows differed"
        );
    }

    // ---- iso-side runs (#653) -----------------------------------

    fn trav(kind: TravKind, ids: &[u32]) -> Trav {
        Trav {
            ids: ids.to_vec(),
            kind,
        }
    }

    fn rim(ids: &[u32]) -> Trav {
        trav(TravKind::Rim { v_raw: 0.0 }, ids)
    }

    fn meridian(ids: &[u32]) -> Trav {
        trav(TravKind::Meridian { u_raw: 0.0 }, ids)
    }

    /// Positions for the rows below: id 0 is the chart's north pole,
    /// id 1 the south pole, and ids 2.. are ordinary surface points.
    fn unit_sphere_positions() -> Vec<Point3<f64>> {
        vec![
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.6, 0.8, 0.0),
        ]
    }

    /// A loop whose every iso side is ONE edge — the shape every
    /// unsplit body has — must open a side at every traversal, so
    /// nothing downstream of `starts` can behave differently from the
    /// pre-#653 walk.
    #[test]
    fn alternating_rims_and_meridians_open_a_side_every_time() {
        let c = z_chart(ChartKind::Sphere { r: 1.0 });
        let p = unit_sphere_positions();
        let travs = vec![
            rim(&[2, 3]),
            meridian(&[3, 4]),
            rim(&[4, 2]),
            meridian(&[2, 2]),
        ];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, Eps::exactly(1e-9)),
            vec![true, true, true, true]
        );
    }

    /// Two meridians meeting AWAY from the axis are one iso side: a
    /// point off the axis has exactly one azimuth, so there is no such
    /// thing as a meridian-meridian corner there.
    #[test]
    fn consecutive_meridians_off_the_axis_are_one_side() {
        let c = z_chart(ChartKind::Sphere { r: 1.0 });
        let p = unit_sphere_positions();
        let travs = vec![rim(&[2, 3]), meridian(&[3, 4]), meridian(&[4, 2])];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, Eps::exactly(1e-9)),
            vec![true, true, false]
        );
    }

    /// A POLE junction is a corner, not one side — the fan the walk
    /// emits two entries for, and `unwrap_tie`'s π-apart wire band.
    ///
    /// The chart here is a SPHERE, and that is the whole exercised
    /// class: the predicate is spatial, so a cone's apex takes the same
    /// path with no extra case, and a horn or spindle torus's axis
    /// point would too — but `revolve` refuses those at construction,
    /// so no row here or anywhere can exercise one. Read this as
    /// covering the sphere pole and the cone apex; the torus claim in
    /// `iso_side_starts`' docs is labelled unexercised there.
    #[test]
    fn a_pole_junction_always_breaks_the_side() {
        let c = z_chart(ChartKind::Sphere { r: 1.0 });
        let p = unit_sphere_positions();
        // A pole-to-pole band: two meridians, both junctions a pole.
        let travs = vec![meridian(&[1, 2, 0]), meridian(&[0, 3, 1])];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, Eps::exactly(1e-9)),
            vec![true, true]
        );
        // ... and the same two meridians with a vertex dropped on the
        // FIRST one are one side across that vertex and two sides
        // across each pole.
        let split = vec![meridian(&[1, 2]), meridian(&[2, 0]), meridian(&[0, 3, 1])];
        assert_eq!(
            iso_side_starts(&split, &c, &p, Eps::exactly(1e-9)),
            vec![true, false, true]
        );
    }

    /// Rims run too: two coaxial circles at different `v` are
    /// disjoint, so two rims that meet are necessarily co-`v`.
    #[test]
    fn consecutive_rims_off_the_axis_are_one_row() {
        let c = z_chart(ChartKind::Cylinder { r: 1.0 });
        let p = unit_sphere_positions();
        let travs = vec![rim(&[2, 3]), rim(&[3, 4]), meridian(&[4, 2])];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, Eps::exactly(1e-9)),
            vec![true, false, true]
        );
    }

    /// **The rim-anchor row** (#653). The walk must start at a rim
    /// that OPENS its row, not merely at the first rim.
    ///
    /// The failing condition, which is what makes this row a revert
    /// detector rather than a restatement: the loop's first rim is a
    /// CONTINUATION — a rim row carried by two edges, with the cycle
    /// beginning inside it. `travs[0]` is then the second half of a
    /// row, and the `position(Rim)` this replaces would anchor there
    /// and split the row in the middle. Reverting `walk_anchor` to
    /// `position(Rim)` reds the first assertion; the second and third
    /// pin that nothing else moved.
    #[test]
    fn the_walk_anchors_on_a_rim_that_opens_its_row() {
        let c = z_chart(ChartKind::Cylinder { r: 1.0 });
        let p = unit_sphere_positions();
        // A rim row carried by two edges, with the cycle starting in
        // the middle of it: traversal 0 continues traversal 2.
        let travs = vec![rim(&[2, 3]), meridian(&[3, 4]), rim(&[4, 2])];
        let starts = iso_side_starts(&travs, &c, &p, Eps::exactly(1e-9));
        assert_eq!(starts, vec![false, true, true], "fixture precondition");
        assert_eq!(
            walk_anchor(&travs, &starts),
            Some(2),
            "the first rim (index 0) continues the last one; the walk must \
             anchor on index 2, which opens the row"
        );
        // Unsplit shape: every rim opens its row, so this is the
        // `position(Rim)` it replaces, index for index.
        let plain = vec![meridian(&[2, 3]), rim(&[3, 4]), meridian(&[4, 2])];
        let plain_starts = iso_side_starts(&plain, &c, &p, Eps::exactly(1e-9));
        assert_eq!(walk_anchor(&plain, &plain_starts), Some(1));
        // Rimless: anchor on any side that opens.
        let band = vec![meridian(&[1, 2]), meridian(&[2, 0]), meridian(&[0, 3, 1])];
        let sphere = z_chart(ChartKind::Sphere { r: 1.0 });
        let band_starts = iso_side_starts(&band, &sphere, &p, Eps::exactly(1e-9));
        assert_eq!(walk_anchor(&band, &band_starts), Some(0));
    }

    /// **The closing-side row** (#653). The closure rule and the
    /// pole-band `prev_u` seed both belong to the last iso SIDE, so
    /// `closing_side` must be that side's opening index, not `m - 1`.
    ///
    /// The failing condition: the last iso side is carried by several
    /// edges. Reverting either consumer to `m - 1` makes the closing
    /// meridian take a column its continuations do not repeat (the
    /// closure assertion in `loop_polygon`), or seeds the pole entry
    /// with a neighbour's band value (4.4e-16 m off-box on the ball).
    #[test]
    fn the_closing_side_is_the_last_run_start_not_the_last_index() {
        // Unsplit: every side is one edge, so it IS `m - 1`.
        assert_eq!(closing_side(&[true, true, true]), 2);
        // The last side is carried by two edges: its opening index is
        // 2, and index 3 continues it.
        assert_eq!(closing_side(&[true, true, true, false]), 2);
        // ... by three.
        assert_eq!(closing_side(&[true, true, false, false]), 1);
        // Degenerate: nothing opens (the whole loop is one cyclic run,
        // which `loop_polygon` patches to `true` at 0 before asking).
        assert_eq!(closing_side(&[false, false]), 0);
    }

    /// The junction test is CYCLIC — traversal 0's predecessor is the
    /// last one — which is what lets `loop_polygon` rotate the walk
    /// onto a side that opens rather than into the middle of one.
    #[test]
    fn the_side_test_wraps_around_the_loop() {
        let c = z_chart(ChartKind::Cylinder { r: 1.0 });
        let p = unit_sphere_positions();
        // ids[0] of traversal 0 is 2, shared with the last traversal.
        let travs = vec![meridian(&[2, 3]), rim(&[3, 4]), meridian(&[4, 2])];
        assert_eq!(
            iso_side_starts(&travs, &c, &p, Eps::exactly(1e-9)),
            vec![false, true, true]
        );
    }

    /// A rimless pole-to-pole band's own point cycle: south pole, up
    /// the meridian at azimuth `a0` in `n` steps, north pole, down the
    /// meridian at azimuth `a1`. Sphere of radius `r` centred at `c`,
    /// about the `+z` axis of [`z_chart`] — the shape `loop_polygon`
    /// collects from `travs` before folding it.
    fn band_cycle(r: f64, c: Point3<f64>, a0: f64, a1: f64, n: usize) -> Vec<Point3<f64>> {
        let on = |a: f64, t: f64| {
            Point3::new(
                c.x + r * a.cos() * t.sin(),
                c.y + r * a.sin() * t.sin(),
                c.z + r * t.cos(),
            )
        };
        let step = core::f64::consts::PI / (n as f64);
        let mut pts = vec![Point3::new(c.x, c.y, c.z - r)];
        pts.extend((1..n).map(|k| on(a0, core::f64::consts::PI - (k as f64) * step)));
        pts.push(Point3::new(c.x, c.y, c.z + r));
        pts.extend((1..n).map(|k| on(a1, (k as f64) * step)));
        pts
    }

    /// The band's chart-frame azimuth, spelled as `loop_polygon`'s
    /// pole-to-pole arm spells it. `sense_sign` is omitted because it
    /// is a bitwise `±1` scale on `area`: it cannot change how well
    /// conditioned the fold that produced `area` was.
    fn band_mid_az(chart: &Chart, pts: &[Point3<f64>]) -> f64 {
        let area = loop_area(pts);
        area.dot(chart.v_ref).atan2(area.dot(chart.u_ref))
    }

    /// A 1.3 mm band's own geometry, placed at seven distances from
    /// the world origin. Non-symmetric, non-dyadic placement on
    /// purpose: an axis-symmetric or dyadic offset cancels the fold's
    /// terms pairwise-exactly and hides the defect.
    fn placed_band(d: f64) -> Vec<Point3<f64>> {
        band_cycle(
            1.3e-3,
            Point3::new(1.3 * d, -2.7 * d, 0.9 * d),
            0.37,
            2.27,
            8,
        )
    }

    /// **Issue 1362.** The band's area vector is read for its
    /// DIRECTION only, so the direction must be the band's own and not
    /// its placement's. The fold is over a closed cycle, so it is
    /// anchor-independent over ℝ; in f64 the anchor is the whole
    /// question, and a world-origin anchor pays cancellation growing
    /// with the placement distance.
    ///
    /// Budget `1e-11·d` radians: the placement enters the POSITIONS
    /// themselves, so the honest floor is the coordinate ulp against
    /// the band's own size — `ulp(2.7·d)/1.3e-3 ≈ 4.6e-13·d` — and
    /// agreement beyond that is not available from the inputs. The
    /// budget sits ~20x above it; the local-anchor fold lands one to
    /// two orders under it at every row, and the origin-anchored
    /// spelling misses the FIRST row (`d = 1e2`, i.e. a band a few
    /// hundred metres out) by 1.6e-6 rad against a 1e-9 budget, then
    /// degrades to 3.4 rad — a direction pointing the other way.
    #[test]
    fn the_band_area_direction_is_the_bands_not_its_placements() {
        let chart = z_chart(ChartKind::Sphere { r: 1.3e-3 });
        let at_origin = band_mid_az(&chart, &placed_band(0.0));
        for d in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8] {
            let far = band_mid_az(&chart, &placed_band(d));
            let drift = (far - at_origin).abs();
            let budget = 1e-11 * d;
            assert!(
                drift < budget,
                "placement {d:e}: azimuth {far} vs {at_origin} at the origin \
                 (drift {drift:e} rad, budget {budget:e})"
            );
        }
    }

    /// The consumer, pinned end to end: `mid_az` exists only to pick
    /// each meridian's `2πk` branch, so a placement that moves the
    /// azimuth far enough moves the COLUMN — the mesh takes the
    /// complementary half of the sphere. Both of this band's meridians
    /// must take the same branch wherever the band sits.
    ///
    /// The rows are the same placements as the direction row. Under an
    /// origin-anchored fold BOTH the `1e6` and `1e8` rows flip the
    /// azimuth by 3.4 rad — over π, so a whole `2π` off in `u`. The row
    /// aborts at the first failure and so only names `1e6` when it
    /// fires; the drift is not monotone in the placement, because how
    /// much of the true area survives the cancellation depends on the
    /// bit patterns of the particular coordinates, not on their size
    /// alone.
    #[test]
    fn a_far_placement_picks_the_same_meridian_branch() {
        let chart = z_chart(ChartKind::Sphere { r: 1.3e-3 });
        let at_origin = band_mid_az(&chart, &placed_band(0.0));
        for d in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8] {
            let far = band_mid_az(&chart, &placed_band(d));
            for u_raw in [0.37, 2.27] {
                let (near_col, far_col) = (unwrap_near(u_raw, at_origin), unwrap_near(u_raw, far));
                assert!(
                    (near_col - far_col).abs() < 1e-9,
                    "placement {d:e}: meridian u_raw {u_raw} takes column \
                     {far_col} there and {near_col} at the origin \
                     ({} branches apart)",
                    ((far_col - near_col) / TAU).round()
                );
            }
        }
    }

    /// **The direction itself, against known geometry.** Every other
    /// row here compares one fold against another fold, so all of them
    /// would pass a `loop_area` that ignored its input and answered a
    /// constant. This one pins the ANSWER: for a band bounded by
    /// meridians at azimuths `a0` and `a1`, the cycle runs up `a0` and
    /// down `a1`, so by the right-hand rule its vector area points
    /// radially INWARD at the pair's bisector — chart-frame azimuth
    /// `(a0 + a1)/2 − π`. Closed form, derived from the geometry and
    /// not from the fold, checked over a spread of openings and at two
    /// placements.
    #[test]
    fn the_area_direction_is_the_bands_inward_bisector() {
        let chart = z_chart(ChartKind::Sphere { r: 1.0 });
        for (a0, a1) in [
            (0.37, 2.27),
            (0.0, 1.0),
            (-1.1, 0.4),
            (2.0, 4.5),
            (0.2, 3.0),
        ] {
            for d in [0.0, 1e3] {
                let c = Point3::new(1.3 * d, -2.7 * d, 0.9 * d);
                let got = band_mid_az(&chart, &band_cycle(1.0, c, a0, a1, 8));
                let want = (a0 + a1) / 2.0 - core::f64::consts::PI;
                // Compare modulo 2π: the azimuth is a branch, not a
                // number, and `atan2` answers in (−π, π].
                let err = (unwrap_near(got, want) - want).abs();
                assert!(
                    err < 1e-12,
                    "meridians {a0} and {a1} at placement {d:e}: area azimuth \
                     {got} is {err:e} rad off the inward bisector {want}"
                );
            }
        }
    }

    /// An empty cycle has no anchor and no area. The guarded hazard is
    /// the anchor, not the fold: deriving a bbox centre needs a first
    /// point, so `loop_area` returns zero before reaching for one.
    /// (The fold body's own `% pts.len()` is unreachable on an empty
    /// slice — the loop never runs — so it is not what this row pins.)
    #[test]
    fn an_empty_cycle_has_no_area() {
        let a = loop_area(&[]);
        assert_eq!((a.x, a.y, a.z), (0.0, 0.0, 0.0));
    }

    /// **Issue 896's guard, demonstrated firing** — the red-first half
    /// of the row, at the mechanism, because no minting door in this
    /// build can place the state in front of `tessellate` (the measured
    /// no-route verdict beside the guard; the door rows live in
    /// `mesh/tests/issue896_pole_guard.rs` and
    /// `step-import/tests/poleguard.rs`).
    ///
    /// The body is real and valid — the nearest-the-pole sphere band
    /// the revolve door admits (top rim 0.1 m off the axis, junction
    /// 0.1001 m from the undeclared pole; every smaller radius refuses
    /// at a certified bar, see the integration row). The band the
    /// guard reads is `loop_polygon`'s `eps` PARAMETER, and 0.15 m
    /// puts that junction inside it while keeping the two top seam
    /// vertices (0.2 m apart) outside the declared-vertex guard's
    /// reach — so the panic this row expects is exactly the new
    /// guard's, not #895's, and not a construction refusal.
    ///
    /// Positions and chords are minted exactly as `tessellate` mints
    /// them, so the walk sees the shape a real call would hand it.
    #[test]
    #[should_panic(expected = "of a chart pole it is not being identified with")]
    fn a_rim_junction_inside_the_pole_band_trips_the_guard() {
        use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
        use std::collections::HashMap;
        let tol = geom_core::Tol::witness();
        let rho = 0.1f64;
        let (h, rc) = (0.5f64.sin(), 0.5f64.cos());
        let yt = (1.0 - rho * rho).sqrt();
        let bulge = ((yt.atan2(rho) - h.atan2(rc)) / 4.0).tan();
        let profile_loop = ProfileLoop::new(vec![
            ProfileVertex::new(geom_core::Point2::new(rc, h), bulge),
            ProfileVertex::new(geom_core::Point2::new(rho, yt), 0.0),
            ProfileVertex::new(geom_core::Point2::new(0.3, 1.3), 0.0),
            ProfileVertex::new(geom_core::Point2::new(1.1, 0.9), 0.0),
        ]);
        let profile = Profile::new(SketchPlane::xy(), vec![profile_loop])
            .validate(tol)
            .unwrap();
        let axis = sweep::RevolveAxis {
            origin: geom_core::Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        };
        let body = sweep::revolve(&profile, axis, sweep::Revolution::Full, tol)
            .unwrap()
            .body;
        let mut positions = Vec::new();
        let mut vids = HashMap::new();
        for (vk, v) in body.vertices() {
            vids.insert(vk, u32::try_from(positions.len()).unwrap());
            positions.push(*body.get_point(v.point).unwrap());
        }
        let mut bounds = crate::nurbs_cert::FaceBounds::new();
        let chords = crate::chords::compute_chords(
            &body,
            crate::sizing::sizing_target(0.05),
            &vids,
            &mut positions,
            &mut bounds,
        )
        .unwrap();
        let (fk, face) = body
            .faces()
            .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
            .expect("the revolve mints a sphere wall");
        let chart = Chart::of(body.get_surface(face.surface).unwrap()).unwrap();
        for (lk, _) in body.loops().filter(|(_, l)| l.face == fk) {
            let _ = loop_polygon(
                &body,
                &chart,
                &chords.ids,
                &positions,
                fk,
                lk,
                Eps::exactly(0.15),
            );
        }
        unreachable!("a loop of this face holds the in-band junction, so the guard fires");
    }
}
