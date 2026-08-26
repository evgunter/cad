//! Boolean joining (ch. 15 §15.7, Programs 15.13/15.14): the ch. 14
//! join skeleton (PR 3's [`ChordJoiner`]) driven **in lockstep across
//! both solids**, with `scanjoin`'s dual requirement — a candidate
//! joins only where a topological neighbor exists in BOTH solids at
//! the SAME loose end — realized through PR 4's explicit
//! [`NullEdgePairRecord`](super::NullEdgePairRecord) correspondence
//! keys and the **germ facings** ([`HalfGerm`]) recorded at insertion
//! (F9: correspondence as data, never correlated array order — the
//! book's `ssortnulledges` ordering/orientation discipline is
//! enforced as the derived sense data below, not as a sort).
//!
//! # Matching is germ identity, not slots or senses (the below-copy
//! # audit)
//!
//! A section-polygon edge lies on the intersection line of one A-face
//! and one B-face; its two end sites offer null-edge halves *facing*
//! that germ. The neighbor test is therefore pure data:
//!
//! - the two halves' germ **face pairs agree** (both components — this
//!   subsumes the book's same-face test and is immune to mid-join face
//!   divisions);
//! - their record **parities oppose** (the book's "opposite he1/he2
//!   roles" — 15.11's IN-record→OUT-record orientation carried as
//!   data);
//! - the A-side and B-side germs of one match are the SAME face pair
//!   (one spatial polygon edge).
//!
//! Boolean runs mint copies of BOTH parities (In-runs mint
//! `NewVertexSide::Below` copies — the PR 4 interface fact); nothing
//! here assumes above-only: polygon-completion roles are resolved by
//! membership of the IN/OUT end-vertex sets built from the F9
//! attributes (`in_copy` = the loop through IN ends), and the germ
//! facings identify halves regardless of which side was minted.
//!
//! # The seam-orientation discipline (PR 5.5 — the derived form of
//! # the book's ssortnulledges / he1↔he2 crossover)
//!
//! Derived from the ratified conventions (outward normals, loops
//! CCW-from-outside: a half-edge with tangent `t` on a face with
//! normal `n` has interior to its LEFT, `n×t` pointing in), each step
//! mirror-checked in the M3-LOG PR 5.5 record:
//!
//! 1. **Required end state.** On the germ line of face pair (fA, fB),
//!    the boundary of fA's region inside B runs `tA(in) = nA×nB`
//!    (check: `nA×(nA×nB) ∝ proj(−nB)`, the into-B direction);
//!    `tA(out)`, `tB(in)`, `tB(out)` follow by the A↔B and in↔out
//!    mirrors, giving `tA(x) = −tB(x)`. Section loops are mates of
//!    region boundaries, so for every op — ∩ (IN,IN), ∪ (OUT,OUT),
//!    ∖ (A-OUT, revert(B-IN)) — the kept loops are antiparallel at
//!    the zip **iff each solid's section loops attach to its own
//!    regions geometrically-CCW-consistently**. Op-independent.
//! 2. **The sense theorem.** The half FACING germ `g` is UP (starts
//!    at `below_end`) iff `g`'s own-solid forward-wedge code is Out;
//!    geometrically, with the orbit-forward direction `w = σ·n_own×d`
//!    (σ the fixed orbit handedness, shared by both solids), UP ⟺
//!    `σ·det[n_own, d, n_other] > 0`. Mirror checks: a CROSSING-lane
//!    strut's two germs share the line with `d1 = −d0` ⇒ opposite
//!    senses (pierce-lane RING struts carry perpendicular germ dirs
//!    instead — their opposite senses come from the cross-solid
//!    anti-correlation below, not the shared-line mirror); the two
//!    facing germs of one polygon side ⇒ opposite senses within each
//!    solid (the neighbor test); `det[nA,d,nB] = −det[nB,d,nA]` ⇒
//!    **sense_A(g) = ¬sense_B(g) at every germ** — the cross-solid
//!    anti-correlation. Insertion mints attributes to this rule
//!    (struts included — their facing swap swaps the labels with it),
//!    so the attributes ARE the discipline; nothing rebinds later.
//!    The angular strut spike order (`bool_strut_order`, insert.rs)
//!    is FORCED by nesting for sector widths W ≤ π — the whole
//!    crossing-minted class (edge-interior sites are exact
//!    half-planes); reflex corners W > 3π/2 with germ angle
//!    θ ∈ (π/2, W−π) sit in an unforced window and can refuse
//!    `SeamOrientation` (ops module "Known limitations").
//! 3. **What the join controls.** Surgery never reverses existing
//!    halves, and chords close cycles forced by arc endpoints, so the
//!    directed cycles after every join are fixed by the senses alone:
//!    per polygon side the IN copies are chorded `up-site → down-site`
//!    on the region side (that direction CCW-bounds the IN region —
//!    the same determinant as step 2), and the section face receives
//!    the antiparallel copies. Role order moves only FACE identity:
//!    which cycle becomes the mef's new face vs stays with the old.
//!    That is orientation-neutral for outer-loop splits and mekr
//!    merges, and load-bearing exactly for RING splits, where the
//!    remainder becomes the old face's ring (a hole boundary must
//!    anti-enclose): the run must take the CCW-winding cycle —
//!    [`choose_roles`]' derived rule via [`ring_run_ccw`] (issue #93;
//!    equivalent to PR 5.5's "cycle opposite the residual-material
//!    side" wherever that probe's outer-loop anchor was sound, and
//!    decided intrinsically so multi-polygon faces cannot cross it).
//! 4. **Consistency theorem.** With (2) as data, (3)'s ring rule per
//!    solid, and matching that consumes the SAME germ in both solids
//!    ([`find_match`]'s slot lock), every completed polygon pair has
//!    A's IN loop antiparallel to B's IN loop, and the zip assertion
//!    ([`BooleanError::SeamOrientation`]) is a theorem with a runtime
//!    witness, not a hope.
//!
//! # The fixpoint sweep and lockstep discipline
//!
//! All pair records register up front; the sweep repeatedly executes
//! the first valid match in deterministic scan order until quiescent
//! (joins can unlock others — order sensitivity of a single greedy
//! pass is real). Joins, retirements, and completions must occur in
//! BOTH solids together; any divergence is the typed
//! [`BooleanError::JoinDesync`] refusal, never a silent mis-join.
//! There is no geometric sort and no section-area certification here:
//! boolean intersection polygons are in general non-planar (§15.7);
//! degenerate results are netted at the component stage instead.

use geom_core::{Band, Decide, Margin};
use slotmap::SecondaryMap;

use super::{BooleanError, BooleanReduction, HalfGerm, Operand};
use crate::body::Body;
use crate::chord_join::{ChordJoiner, CutOutcome, SplitJoinError};
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopKey, VertexKey};
use crate::null::NullFacePair;
use crate::validate::decide;
use geom_core::Tol;

/// One completed section-polygon **pair**: the 2-loop null face in
/// each solid, with the loop roles as F9 data (IN copy = the loop
/// through the IN-side ends).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompletedPolygonPair {
    /// The A-clone null face.
    pub a_face: FaceKey,
    /// A's IN-copy loop.
    pub a_in_loop: LoopKey,
    /// A's OUT-copy loop.
    pub a_out_loop: LoopKey,
    /// The B-clone null face.
    pub b_face: FaceKey,
    /// B's IN-copy loop.
    pub b_in_loop: LoopKey,
    /// B's OUT-copy loop.
    pub b_out_loop: LoopKey,
}

/// Per-solid joining state: the shared chord core plus the F9 side
/// data role resolution reads.
struct SolidJoin {
    joiner: ChordJoiner,
    /// IN-side end vertices (below ends), from the attributes.
    in_set: SecondaryMap<VertexKey, ()>,
    /// OUT-side end vertices (above ends).
    out_set: SecondaryMap<VertexKey, ()>,
    /// Aux partner surfaces minted into THIS body for curved germ
    /// pairs (M5 PR 9), keyed by the OTHER body's germ face — one
    /// mint per partner surface, every chord of the same germ face
    /// shares it (the descriptions stay key-coherent for D6).
    aux_partner: std::collections::BTreeMap<FaceKey, crate::geometry::SurfaceKey>,
}

impl SolidJoin {
    fn new<T: Decide>(red: &BooleanReduction<T>, operand: Operand, band: Band) -> Self {
        let mut in_set = SecondaryMap::new();
        let mut out_set = SecondaryMap::new();
        for r in red.null_edges_of(operand) {
            in_set.insert(r.attr.below_end, ());
            out_set.insert(r.attr.above_end, ());
        }
        Self {
            joiner: ChordJoiner::new(band),
            in_set,
            out_set,
            aux_partner: std::collections::BTreeMap::new(),
        }
    }

    /// The up/down sense of a null-edge half — `start ∈ in_set` ⇒ up
    /// (the below-copy audit: side is attribute data, either parity of
    /// copy resolves here).
    fn is_up<T: Decide>(&self, body: &Body<T>, he: HalfEdgeKey) -> Result<bool, BooleanError> {
        let desync = |what| BooleanError::JoinDesync { what };
        let start = body
            .get_half_edge(he)
            .ok_or(desync("half no longer resolves"))?
            .start;
        if self.in_set.contains_key(start) {
            Ok(true)
        } else if self.out_set.contains_key(start) {
            Ok(false)
        } else {
            Err(desync("null half starts at a vertex of neither side set"))
        }
    }
}

/// A completed 2-loop null-face pair before role resolution.
#[derive(Clone, Copy, Debug)]
struct UnresolvedPair {
    a_face: FaceKey,
    a_outer: LoopKey,
    a_ring: LoopKey,
    b_face: FaceKey,
    b_outer: LoopKey,
    b_ring: LoopKey,
}

/// One registered pair record: each solid's two germ facings with
/// used-in-a-join flags. Slot `i` of `a` and slot `i` of `b` are the
/// SAME spatial germ (minted from one crossing-record pair with shared
/// face-pair/direction meta) — the F9 correspondence the match walks.
#[derive(Clone, Copy, Debug)]
struct OpenRecord<T: geom_core::Real> {
    a_edge: EdgeKey,
    b_edge: EdgeKey,
    a: [(HalfGerm<T>, bool); 2],
    b: [(HalfGerm<T>, bool); 2],
}

impl<T: geom_core::Real> OpenRecord<T> {
    fn fully_used(&self) -> bool {
        self.a.iter().all(|(_, u)| *u) && self.b.iter().all(|(_, u)| *u)
    }
}

/// A resolved match: record indices plus the germ slot per record —
/// ONE slot each, consumed in BOTH solids (germ identity is shared
/// data; per-solid slot freedom was the R2 desync soup).
#[derive(Clone, Copy, Debug)]
struct Match {
    entry: usize,
    cand: usize,
    entry_slot: usize,
    cand_slot: usize,
}

/// `bool_connect`'s product: the completed pairs plus the per-operand
/// chord-mef fragment logs (naming emission, M4 PR 3 — `(new face,
/// divided-from face)` at call-time CLONE keys, A rows in the A-clone
/// arena, B rows in the B-clone arena pre-graft).
pub(super) struct Connected {
    pub completed: Vec<CompletedPolygonPair>,
    pub a_fragments: Vec<(FaceKey, FaceKey)>,
    pub b_fragments: Vec<(FaceKey, FaceKey)>,
}

/// The lockstep joining sweep (module docs). Mutates both annotated
/// clones in `red` in place; returns the completed polygon pairs in
/// completion order, with [`NullFacePair::Boolean`] records set.
pub(super) fn bool_connect<T: Decide>(
    red: &mut BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    band: Band,
    tol: Tol,
) -> Result<Connected, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut sa = SolidJoin::new(red, Operand::A, band);
    let mut sb = SolidJoin::new(red, Operand::B, band);
    let mut completed: Vec<UnresolvedPair> = Vec::new();

    // Register every pair record up front (germ facings from the
    // insertion records).
    // Per-operand germ maps (edge keys are body-lineage-scoped — one
    // map across both bodies would collide).
    let mut a_by_edge: SecondaryMap<EdgeKey, [HalfGerm<T>; 2]> = SecondaryMap::new();
    let mut b_by_edge: SecondaryMap<EdgeKey, [HalfGerm<T>; 2]> = SecondaryMap::new();
    for r in &red.null_edges {
        match r.operand {
            Operand::A => a_by_edge.insert(r.edge, r.germs),
            Operand::B => b_by_edge.insert(r.edge, r.germs),
        };
    }
    let mut open: Vec<OpenRecord<T>> = Vec::new();
    for p in &red.null_pairs {
        let a = *a_by_edge
            .get(p.a_edge)
            .ok_or(desync("pair A edge without a germ record"))?;
        let b = *b_by_edge
            .get(p.b_edge)
            .ok_or(desync("pair B edge without a germ record"))?;
        open.push(OpenRecord {
            a_edge: p.a_edge,
            b_edge: p.b_edge,
            a: [(a[0], false), (a[1], false)],
            b: [(b[0], false), (b[1], false)],
        });
    }

    // Fixpoint (module docs).
    while let Some(m) = find_match(&open, red, &sa, &sb, band)? {
        let (ea, ra) = (
            open[m.entry].a[m.entry_slot].0.he,
            open[m.cand].a[m.cand_slot].0.he,
        );
        let (eb, rb) = (
            open[m.entry].b[m.entry_slot].0.he,
            open[m.cand].b[m.cand_slot].0.he,
        );
        // Still-loose halves (choose_roles' separation constraint —
        // a role order must not wall a pending half off from its match
        // partner). The chosen halves themselves are being consumed.
        let (mut a_loose, mut b_loose) = loose_partners(&open, red, band)?;
        a_loose.remove(ea);
        a_loose.remove(ra);
        b_loose.remove(eb);
        b_loose.remove(rb);
        // Role order per solid, derived independently (module docs —
        // the PR 5.5 discipline): cross-solid seam orientation is
        // carried by the sense attributes alone; role order only
        // decides the face partition of a same-loop split, which each
        // solid resolves against its OWN geometry.
        let (a1, a2) = choose_roles(&red.a, ea, ra, &a_loose, band)?;
        let (b1, b2) = choose_roles(&red.b, eb, rb, &b_loose, band)?;
        // Curved germ pairs (M5 PR 9): each solid's chord lane comes
        // from the germ FACE PAIR — plane×plane keeps the M3
        // straight-chord lane bit-identically; plane×cylinder mints
        // the C5 section conic on both sides (the wall side through
        // the S9 window machinery with the germ plane as context, the
        // planar side against the wall face's own window, so both
        // solids select the SAME geometric arc); plane×sphere (M5
        // S13) rides the same two lanes with the exact C5 Circle and
        // the sphere chart's azimuth window; any other pair refuses
        // typed citing its C5 routing (per-arm, C12.1).
        let germ = open[m.entry].a[m.entry_slot].0;
        let surf_of = |body: &Body<T>, f: FaceKey| -> Result<geom::Surface<T>, BooleanError> {
            body.get_face(f)
                .and_then(|fd| body.get_surface(fd.surface))
                .cloned()
                .ok_or(desync("germ face surface no longer resolves"))
        };
        // The germ faces' SURFACES, deliberately unoriented (S10): what
        // the curved lanes below take from a plane germ is a
        // [`SplitPlane`] — a section datum, an operation input whose
        // normal names a chart, not a material side. The plane as a
        // point set (and hence the section conic, its azimuth window,
        // and the auxiliary surface minted for it) is identical under
        // a sense flip, so applying `sense_sign` here would rewrite an
        // input that never meant "outward"; the created faces' own
        // orientation comes from the joiner's stored winding.
        let ga = surf_of(&red.a, germ.a_face)?;
        let gb = surf_of(&red.b, germ.b_face)?;
        use crate::chord_join::{JoinLane, SectionCtx, face_azimuth_window};
        use crate::splitting::SplitPlane;
        use geom::Surface as Sf;
        match (&ga, &gb) {
            (Sf::Plane { .. }, Sf::Plane { .. }) => {
                sa.joiner
                    .join(&mut red.a, a1, a2, JoinLane::Planar, tol)
                    .map_err(BooleanError::Join)?;
                sb.joiner
                    .join(&mut red.b, b1, b2, JoinLane::Planar, tol)
                    .map_err(BooleanError::Join)?;
            }
            (Sf::Plane { origin, normal, .. }, Sf::Sphere { .. })
            | (Sf::Plane { origin, normal, .. }, Sf::Cylinder { .. }) => {
                let window = face_azimuth_window(&red.b, &gb, germ.b_face, band)
                    .map_err(BooleanError::Join)?
                    .ok_or(desync("wall germ face has no charted azimuth window"))?;
                let mut partner = sa.aux_partner.get(&germ.b_face).copied();
                sa.joiner
                    .join(
                        &mut red.a,
                        a1,
                        a2,
                        JoinLane::BoolPlanar {
                            wall: gb.clone(),
                            window,
                            partner_key: &mut partner,
                        },
                        tol,
                    )
                    .map_err(BooleanError::Join)?;
                if let Some(k) = partner {
                    sa.aux_partner.insert(germ.b_face, k);
                }
                let mut ctx = SectionCtx {
                    plane: SplitPlane {
                        origin: *origin,
                        normal: *normal,
                    },
                    plane_key: sb.aux_partner.get(&germ.a_face).copied(),
                };
                sb.joiner
                    .join(&mut red.b, b1, b2, JoinLane::Split(&mut ctx), tol)
                    .map_err(BooleanError::Join)?;
                if let Some(k) = ctx.plane_key {
                    sb.aux_partner.insert(germ.a_face, k);
                }
            }
            (Sf::Sphere { .. }, Sf::Plane { origin, normal, .. })
            | (Sf::Cylinder { .. }, Sf::Plane { origin, normal, .. }) => {
                let mut ctx = SectionCtx {
                    plane: SplitPlane {
                        origin: *origin,
                        normal: *normal,
                    },
                    plane_key: sa.aux_partner.get(&germ.b_face).copied(),
                };
                sa.joiner
                    .join(&mut red.a, a1, a2, JoinLane::Split(&mut ctx), tol)
                    .map_err(BooleanError::Join)?;
                if let Some(k) = ctx.plane_key {
                    sa.aux_partner.insert(germ.b_face, k);
                }
                let window = face_azimuth_window(&red.a, &ga, germ.a_face, band)
                    .map_err(BooleanError::Join)?
                    .ok_or(desync("wall germ face has no charted azimuth window"))?;
                let mut partner = sb.aux_partner.get(&germ.a_face).copied();
                sb.joiner
                    .join(
                        &mut red.b,
                        b1,
                        b2,
                        JoinLane::BoolPlanar {
                            wall: ga.clone(),
                            window,
                            partner_key: &mut partner,
                        },
                        tol,
                    )
                    .map_err(BooleanError::Join)?;
                if let Some(k) = partner {
                    sb.aux_partner.insert(germ.a_face, k);
                }
            }
            (a_s, b_s) => {
                // No wired join arm for this germ pair (cyl×cyl's
                // equal-radius ellipse pair, cyl×sphere's rung-3
                // fitted chords, plane×NURBS behind PR 7b): typed,
                // citing the kind whose join arm is missing.
                let (operand, face, s) = if matches!(a_s, Sf::Plane { .. }) {
                    (Operand::B, germ.b_face, b_s)
                } else {
                    (Operand::A, germ.a_face, a_s)
                };
                return Err(BooleanError::CurvedBooleanUnsupported {
                    operand,
                    face,
                    kind: geom_brep::SurfaceKind::of(s),
                });
            }
        }
        open[m.entry].a[m.entry_slot].1 = true;
        open[m.entry].b[m.entry_slot].1 = true;
        open[m.cand].a[m.cand_slot].1 = true;
        open[m.cand].b[m.cand_slot].1 = true;
        // Retire fully-used records (higher index first: removal must
        // not shift the other's index).
        let mut done: Vec<usize> = [m.entry, m.cand]
            .into_iter()
            .filter(|&i| open[i].fully_used())
            .collect();
        done.sort_unstable_by(|x, y| y.cmp(x));
        for i in done {
            let r = open.remove(i);
            cut_pair(red, &mut sa, &mut sb, &mut completed, r.a_edge, r.b_edge)?;
        }
    }

    // ---- Role resolution, deferred to quiescence: mid-join the
    // region faces are not yet final (an operand face pierced by TWO
    // polygons still spans both seams while the first completes), so
    // probing happens only after every polygon has been cut. ----
    let mut resolved = Vec::with_capacity(completed.len());
    for c in completed {
        let (a_in_loop, a_out_loop) =
            resolve_roles_geometric(&red.a, b_pristine, c.a_face, c.a_outer, c.a_ring, band, tol)?;
        let (b_in_loop, b_out_loop) =
            resolve_roles_geometric(&red.b, a_pristine, c.b_face, c.b_outer, c.b_ring, band, tol)?;
        red.a.set_null_face_pair(
            c.a_face,
            NullFacePair::Boolean {
                in_copy: a_in_loop,
                out_copy: a_out_loop,
            },
        )?;
        red.b.set_null_face_pair(
            c.b_face,
            NullFacePair::Boolean {
                in_copy: b_in_loop,
                out_copy: b_out_loop,
            },
        )?;
        resolved.push(CompletedPolygonPair {
            a_face: c.a_face,
            a_in_loop,
            a_out_loop,
            b_face: c.b_face,
            b_in_loop,
            b_out_loop,
        });
    }
    let completed = resolved;

    let leftovers: usize = open
        .iter()
        .map(|r| r.a.iter().filter(|(_, u)| !u).count())
        .sum();
    if leftovers != 0 {
        return Err(BooleanError::Join(SplitJoinError::UnpairedLooseEnds {
            count: leftovers,
        }));
    }
    Ok(Connected {
        completed,
        a_fragments: sa.joiner.take_fragments(),
        b_fragments: sb.joiner.take_fragments(),
    })
}

/// `scanjoin`, germ form (module docs): among all candidate/entry slot
/// combinations whose A-side germs carry the SAME face pair and whose
/// two sites mutually FACE each other along the germ line
/// (`bool_join_facing`, decided — the polygon edge's ends point at one
/// another), with OPPOSED senses in both solids (the sense theorem's
/// neighbor test), pick the NEAREST pair of sites (`bool_join_nearest`,
/// decided — non-adjacent same-line sites must not be chorded across
/// an intermediate one). The B side consumes the SAME slots — slot `i`
/// of the A and B germ arrays is one spatial germ (registration doc);
/// a B-side face-pair or sense disagreement at matched slots is a
/// loud desync, never an alternative pairing. Zero-distance
/// combinations (distinct pair records at one coincident site) are
/// skipped — that degeneracy gate is `bool_join_chord` (margin = the
/// chord LENGTH), a separate question from `bool_join_nearest`'s
/// selection (margin = a DIFFERENCE of chord lengths), so the two
/// populations meter separately. Deterministic scan order breaks
/// exact ties (D9).
fn find_match<T: Decide>(
    open: &[OpenRecord<T>],
    red: &BooleanReduction<T>,
    sa: &SolidJoin,
    sb: &SolidJoin,
    band: Band,
) -> Result<Option<Match>, BooleanError> {
    use geom_core::Sign;
    let desync = |what| BooleanError::JoinDesync { what };
    let point_of = |he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
        let v = red
            .a
            .get_half_edge(he)
            .ok_or(desync("germ half no longer resolves"))?
            .start;
        red.a
            .get_vertex(v)
            .and_then(|vd| red.a.get_point(vd.point).copied())
            .ok_or(desync("germ vertex has no point"))
    };
    // Unused germ slots of one record side (half tied to germ slot —
    // the mint facing is honest data for every lane, struts included).
    fn slots<T: geom_core::Real>(side: &[(HalfGerm<T>, bool); 2]) -> Vec<usize> {
        (0..2).filter(|&g| !side[g].1).collect()
    }
    let mut best: Option<(T, Match)> = None;
    for (cand, rec) in open.iter().enumerate() {
        for (entry, e) in open.iter().enumerate() {
            if entry == cand {
                continue;
            }
            for &cs in &slots(&rec.a) {
                let rga = rec.a[cs].0;
                let r_he = rga.he;
                for &es in &slots(&e.a) {
                    let ega = e.a[es].0;
                    let e_he = ega.he;
                    if ega.a_face != rga.a_face || ega.b_face != rga.b_face {
                        continue;
                    }
                    if sa.is_up(&red.a, e_he)? == sa.is_up(&red.a, r_he)? {
                        continue;
                    }
                    // Mutual facing along the germ line (spatially
                    // shared between the bodies — decided on the A
                    // clone's coincident copies).
                    let p_c = point_of(r_he)?;
                    let p_e = point_of(e_he)?;
                    let chord = p_e - p_c;
                    let dist = chord.norm();
                    let escalate = |diag| BooleanError::Escalated { diag };
                    match decide("bool_join_chord", Margin::of(dist), band).map_err(escalate)? {
                        Sign::Positive => {}
                        _ => continue, // coincident sites: no polygon edge
                    }
                    // Locus-aware mutual facing (fix pass, dev 4):
                    // straight germ lines take the M3 chord test
                    // bit-identically; conic germ loci compare
                    // rotational senses about the section frame.
                    let frame = germ_section_frame(red, &rga, band)?;
                    if !germs_face_each_other(frame, &rga, &ega, p_c, p_e, band)? {
                        continue;
                    }
                    // The B side at the SAME slots — mirror checks, not
                    // freedom: shared-germ face pairs and the
                    // anti-correlation theorem make disagreement a
                    // kernel bug, refused loudly.
                    let (rgb, egb) = (rec.b[cs].0, e.b[es].0);
                    if rgb.a_face != rga.a_face
                        || rgb.b_face != rga.b_face
                        || egb.a_face != ega.a_face
                        || egb.b_face != ega.b_face
                    {
                        return Err(desync("B germ face pair differs at matched slots"));
                    }
                    if sb.is_up(&red.b, egb.he)? == sb.is_up(&red.b, rgb.he)? {
                        return Err(desync("B senses agree at a matched pair"));
                    }
                    let m = Match {
                        entry,
                        cand,
                        entry_slot: es,
                        cand_slot: cs,
                    };
                    best = match best {
                        None => Some((dist, m)),
                        Some((bd, bm)) => {
                            match decide("bool_join_nearest", Margin::of(dist - bd), band)
                                .map_err(escalate)?
                            {
                                Sign::Negative => Some((dist, m)),
                                _ => Some((bd, bm)),
                            }
                        }
                    };
                }
            }
        }
    }
    Ok(best.map(|(_, m)| m))
}

/// The germ pair's section frame: the conic center and axis of the
/// section the germ line lies on, or `None` when that locus is
/// STRAIGHT — a plane×plane pair, and the degenerate plane×cylinder
/// outcomes whose loci ARE lines (ParallelLines/TangentLine).
///
/// **`None` is a claim, not a default.** The caller reads it as "take
/// the straight-chord facing test", so a pair whose section arm is not
/// wired must refuse ([`BooleanError::GermFrameUnsupported`]) rather
/// than fall through to it: falling through would mint a wrong chord
/// silently for every pair the dispatch later admits. The pair match
/// below is therefore EXHAUSTIVE over kinds by construction.
///
/// Section escalations propagate; non-escalation classification
/// failures at match time are a desync (the germ was minted FROM this
/// pair's crossing).
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
fn germ_section_frame<T: Decide>(
    red: &BooleanReduction<T>,
    germ: &HalfGerm<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let surf = |body: &Body<T>, f: FaceKey| -> Result<geom::Surface<T>, BooleanError> {
        body.get_face(f)
            .and_then(|fd| body.get_surface(fd.surface))
            .cloned()
            .ok_or(desync("germ face surface no longer resolves"))
    };
    let sa = surf(&red.a, germ.a_face)?;
    let sb = surf(&red.b, germ.b_face)?;
    pair_section_frame(&sa, &sb, band).map_err(|e| match e {
        FrameError::Escalated(diag) => BooleanError::Escalated { diag },
        FrameError::Desync(what) => desync(what),
        FrameError::NoArm => BooleanError::GermFrameUnsupported {
            a_face: germ.a_face,
            a_kind: geom_brep::SurfaceKind::of(&sa),
            b_face: germ.b_face,
            b_kind: geom_brep::SurfaceKind::of(&sb),
        },
    })
}

/// Why [`pair_section_frame`] could not name a frame. The keys and
/// bodies live at the call site, so this carries none of them: the
/// dispatch is a statement about the KIND PAIR alone.
pub(super) enum FrameError {
    /// A section predicate landed in the sliver band.
    Escalated(geom_core::Indeterminate),
    /// The classification contradicted the germ that was minted from
    /// it — a lockstep failure, not a frontier.
    Desync(&'static str),
    /// The kind pair has no section arm at all.
    NoArm,
}

/// **The pair-general section-frame dispatch**, keyed on the germ
/// pair's two surface KINDS and nothing else — no bodies, no keys, so
/// a lane widening the dispatch adds an arm here and every consumer
/// inherits it.
///
/// `Ok(None)` means the locus is STRAIGHT and is only ever returned by
/// an arm that PROVED it: the plane×plane pair (a line by
/// construction) and the two degenerate plane×cylinder outcomes whose
/// loci are lines. A kind pair with no arm is [`FrameError::NoArm`] —
/// never `None`, because the caller reads `None` as "run the
/// straight-chord facing test" and would mint a wrong chord from it.
#[allow(clippy::type_complexity)] // (conic center, conic axis) — one frame tuple
pub(super) fn pair_section_frame<T: Decide>(
    sa: &geom::Surface<T>,
    sb: &geom::Surface<T>,
    band: Band,
) -> Result<Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>, FrameError> {
    use geom::Surface as Sf;
    let (plane_s, cyl_s, radius) = match (sa, sb) {
        (Sf::Plane { .. }, Sf::Cylinder { radius, .. }) => (sa, sb, *radius),
        (Sf::Cylinder { radius, .. }, Sf::Plane { .. }) => (sb, sa, *radius),
        // The sphere germ pair (M5 S13): the C5 Circle's frame,
        // through THE table — same escalation plumbing.
        (Sf::Plane { .. }, Sf::Sphere { .. }) | (Sf::Sphere { .. }, Sf::Plane { .. }) => {
            let (plane_s, sph_s) = if matches!(sa, Sf::Plane { .. }) {
                (sa, sb)
            } else {
                (sb, sa)
            };
            return match geom_brep::plane_sphere_section(plane_s, sph_s, band) {
                Ok(geom_brep::PlaneSphereSection::Circle(geom::Curve3::Circle {
                    center,
                    axis,
                    ..
                })) => Ok(Some((center, axis))),
                Ok(geom_brep::PlaneSphereSection::Circle(_)) => Err(FrameError::Desync(
                    "plane×sphere classification carried a non-circle",
                )),
                // A tangent POINT / empty gap under a minted germ is a
                // touching configuration the reduction should not have
                // paired — loud, typed.
                Ok(
                    geom_brep::PlaneSphereSection::TangentPoint(_)
                    | geom_brep::PlaneSphereSection::Empty,
                ) => Err(FrameError::Desync(
                    "germ pair's plane×sphere section is not a locus",
                )),
                Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::Escalated(diag)),
                Err(_) => Err(FrameError::Desync(
                    "germ pair's section refused at match time",
                )),
            };
        }
        // The ONE structurally straight pair: a plane×plane section is
        // a line, so "no frame" is a proof here rather than a default.
        (Sf::Plane { .. }, Sf::Plane { .. }) => return Ok(None),
        _ => return Err(FrameError::NoArm),
    };
    match geom_brep::plane_cylinder_section(plane_s, cyl_s, radius, band) {
        Ok(geom_brep::PlaneCylinderSection::Rim(geom::Curve3::Circle { center, axis, .. }))
        | Ok(geom_brep::PlaneCylinderSection::TiltedEllipse(geom::Curve3::Ellipse {
            center,
            axis,
            ..
        })) => Ok(Some((center, axis))),
        Ok(
            geom_brep::PlaneCylinderSection::ParallelLines { .. }
            | geom_brep::PlaneCylinderSection::TangentLine(_),
        ) => Ok(None),
        Ok(_) => Err(FrameError::Desync(
            "germ pair's section classification is not a locus",
        )),
        Err(geom_brep::SectionError::Escalated(diag)) => Err(FrameError::Escalated(diag)),
        Err(_) => Err(FrameError::Desync(
            "germ pair's section refused at match time",
        )),
    }
}

/// Mutual germ facing along the germ LOCUS (M5 PR 9 fix pass, dev 4).
/// Straight germ lines keep the M3 chord test bit-identically: both
/// dirs definitely point at each other along the chord (Zero =
/// definite non-facing, `continue` semantics; in-band escalates in
/// `decide`). A CONIC germ locus makes the chord test structurally
/// degenerate — a semicircle arc leaves BOTH sites exactly
/// perpendicular to the chord (the two-arc disc, PR 5's canonical
/// authoring, hit exactly this as `UnpairedLooseEnds` "(kernel
/// bug)") — so the arc-aware test asks the honest question instead:
/// do the two germs bound ONE rotational traversal of the section
/// conic, i.e. do their rotational senses `axis·((p−c)×dir)` (metres:
/// |p−c| ~ radius, dir unit) definitely OPPOSE? A Zero sense is a
/// radial germ — malformed germ data, a loud desync, never a silent
/// non-match; its in-band sibling escalates through the funnel
/// (`bool_join_arc_facing`), the two-tolerance pair.
fn germs_face_each_other<T: Decide>(
    frame: Option<(geom_core::Point3<T>, geom_core::Vec3<T>)>,
    g1: &HalfGerm<T>,
    g2: &HalfGerm<T>,
    p1: geom_core::Point3<T>,
    p2: geom_core::Point3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    use geom_core::Sign;
    let escalate = |diag| BooleanError::Escalated { diag };
    match frame {
        None => {
            let chord = p2 - p1;
            // Facing margins in METRES: unit germ dir · chord = cos ×
            // separation (rim-dimensional audit: the former `/ dist`
            // stripped the metres and compared a bare cosine against
            // the length band — class (c)).
            let f1 = g1.dir.dot(chord);
            let f2 = g2.dir.dot(-chord);
            Ok(
                decide("bool_join_facing", Margin::of(f1), band).map_err(escalate)?
                    == Sign::Positive
                    && decide("bool_join_facing", Margin::of(f2), band).map_err(escalate)?
                        == Sign::Positive,
            )
        }
        Some((center, axis)) => {
            let s1 = axis.dot((p1 - center).cross(g1.dir));
            let s2 = axis.dot((p2 - center).cross(g2.dir));
            let d1 = decide("bool_join_arc_facing", Margin::of(s1), band).map_err(escalate)?;
            let d2 = decide("bool_join_arc_facing", Margin::of(s2), band).map_err(escalate)?;
            match (d1, d2) {
                (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Ok(true),
                (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Ok(false),
                (Sign::Zero, _) | (_, Sign::Zero) => Err(BooleanError::JoinDesync {
                    what: "a conic germ has no rotational sense (radial germ direction — \
                           malformed germ data)",
                }),
            }
        }
    }
}

type LooseMap = SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>;

/// Still-unused null-edge halves, each mapped to its geometric MATCH
/// PARTNER's half in the same solid: the nearest mutually-facing loose
/// germ with the same face pair — [`find_match`]'s own criteria,
/// static in the germ geometry, so a captured partner PAIR can still
/// join (same face) while splitting a pair walls one side off. Germ
/// meta is shared between the solids, so the (record, slot) partner
/// relation is computed once (A-clone points — coincident copies) and
/// translated per solid. A loose half with no partner maps to `None`
/// (conservatively separated wherever captured).
fn loose_partners<T: Decide>(
    open: &[OpenRecord<T>],
    red: &BooleanReduction<T>,
    band: Band,
) -> Result<(LooseMap, LooseMap), BooleanError> {
    use geom_core::Sign;
    let desync = |what| BooleanError::JoinDesync { what };
    let escalate = |diag| BooleanError::Escalated { diag };
    let point_of = |he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
        let v = red
            .a
            .get_half_edge(he)
            .ok_or(desync("germ half no longer resolves"))?
            .start;
        red.a
            .get_vertex(v)
            .and_then(|vd| red.a.get_point(vd.point).copied())
            .ok_or(desync("germ vertex has no point"))
    };
    let loose: Vec<(usize, usize)> = open
        .iter()
        .enumerate()
        .flat_map(|(i, r)| (0..2).filter(move |&s| !r.a[s].1).map(move |s| (i, s)))
        .collect();
    let mut a_map: LooseMap = SecondaryMap::new();
    let mut b_map: LooseMap = SecondaryMap::new();
    for &(i, s) in &loose {
        let g = open[i].a[s].0;
        let p = point_of(g.he)?;
        let mut best: Option<(T, (usize, usize))> = None;
        for &(j, t) in &loose {
            if (j, t) == (i, s) {
                continue;
            }
            let g2 = open[j].a[t].0;
            if g2.a_face != g.a_face || g2.b_face != g.b_face {
                continue;
            }
            let p2 = point_of(g2.he)?;
            let chord = p2 - p;
            let dist = chord.norm();
            match decide("bool_join_chord", Margin::of(dist), band).map_err(escalate)? {
                Sign::Positive => {}
                _ => continue,
            }
            // The SAME locus-aware facing as find_match (dev 4): the
            // separation constraint must count partners with the
            // matcher's own eyes or roles get walled off wrongly.
            let frame = germ_section_frame(red, &g, band)?;
            if !germs_face_each_other(frame, &g, &g2, p, p2, band)? {
                continue;
            }
            best = match best {
                None => Some((dist, (j, t))),
                Some((bd, bm)) => {
                    match decide("bool_join_nearest", Margin::of(dist - bd), band)
                        .map_err(escalate)?
                    {
                        Sign::Negative => Some((dist, (j, t))),
                        _ => Some((bd, bm)),
                    }
                }
            };
        }
        let partner = best.map(|(_, jt)| jt);
        a_map.insert(g.he, partner.map(|(j, t)| open[j].a[t].0.he));
        b_map.insert(open[i].b[s].0.he, partner.map(|(j, t)| open[j].b[t].0.he));
    }
    Ok((a_map, b_map))
}

/// Chooses the join role order for one solid (PR 5.5 — the enforced
/// discipline; module docs for the derivation). The three lanes:
///
/// - **Different loops** (the mekr lane): a pure loop merge — role
///   order is orientation-neutral; keep the given order.
/// - **Same loop, the face's OUTER**: the split partitions real
///   boundary between two faces; either partition names the same two
///   directed cycles (role order moves only face identity), so the
///   order is chosen by the clean-arc constraint alone: the first
///   chord's mef run `[h1 .. h2]` must not SEPARATE a still-loose
///   scaffolding pair (walling a pending site off from its partner).
///   Both arcs dirty is a loud desync.
/// - **Same loop, a RING of its face** (the closed seam-ring lane —
///   pierce-ring scaffolding): the split's remainder stays a ring of
///   the old face and must anti-enclose (a hole boundary), so the mef
///   run — the enclosed patch, the new face's outer — must wind CCW
///   around the face's outward normal. Decided intrinsically by
///   [`ring_run_ccw`] (issue #93; supersedes the PR 5.5
///   residual-material-side probe, whose outer-loop vertex anchor was
///   unsound mid-fixpoint on multi-polygon faces — see the lane
///   comment). A derived order whose run separates a loose pair is a
///   loud desync.
///
/// Cross-solid consistency needs NO coupling of the two solids' role
/// orders: the sense attributes carry the seam orientation (the
/// anti-correlation theorem), and each solid's partition is decided
/// against its own geometry — the zip's antiparallelism assertion is
/// the runtime witness.
fn choose_roles<T: Decide>(
    body: &Body<T>,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
    band: Band,
) -> Result<(HalfEdgeKey, HalfEdgeKey), BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    let l = loop_of(ea)?;
    if l != loop_of(ra)? {
        return Ok((ea, ra)); // mekr lane
    }
    let face = body
        .get_loop(l)
        .ok_or(desync("role loop no longer resolves"))?
        .face;
    let outer = body
        .get_face(face)
        .ok_or(desync("role face no longer resolves"))?
        .outer;
    if l == outer {
        // Defensive guard (PR 5.5 review, MINOR (b)): no constructible
        // both-arcs-dirty witness is known post-discipline (the
        // partner-based separation test cleared every previously
        // refusing fixture); kept as a loud backstop, never deleted.
        return clean_dir(body, ea, ra, loose)?
            .ok_or(desync("every chord arc separates a loose scaffolding pair"));
    }
    // Ring lane: intrinsic winding (issue #93). The role order is
    // fully determined by the face's own orientation — the mef run
    // (the enclosed patch, the new face's outer) must wind CCW around
    // the face's outward normal so the remainder ring anti-encloses.
    // Exactly one of the two orders satisfies it (the candidate runs
    // are antiparallel copies). This replaces the PR 5.5
    // residual-material-side probe, which anchored on the face's
    // outer-loop vertices and was UNSOUND mid-fixpoint on faces
    // hosting several pending polygons: the outer anchor classified a
    // region other pending seams still separate from the island's
    // immediate surround (the A×Z counter island — surround IN, outer
    // corners OUT — silently crossed the copies; the zip's
    // antiparallelism witness caught it). The two rules agree wherever
    // the residual anchor was sound (both parities checked in the
    // issue #93 diagnosis), so corpus surgery is unchanged.
    let (h1, h2) = if ring_run_ccw(body, face, ea, ra, band)? {
        (ea, ra)
    } else {
        (ra, ea)
    };
    match clean_dir(body, h1, h2, loose)? {
        Some((c1, _)) if c1 == h1 => Ok((h1, h2)),
        _ => Err(desync(
            "derived ring role order separates a loose scaffolding pair",
        )),
    }
}

/// Whether the prospective mef run `[h1 .. h2]` — the `next`-order arc
/// from `h1` through `h2`, closed by the chord `end(h2) → start(h1)`
/// (exactly the cycle the joiner's first `mef(Chords { he1: h1,
/// he2: next(h2, tol) })` walls off as the new face) — winds CCW around
/// `face`'s outward normal: the orientation an island's new outer loop
/// must have (the remainder ring anti-encloses iff the run encloses).
///
/// Reified (issue #93): the functional is `n · Σ (pᵢ−p₀)×(pᵢ₊₁−p₀)` —
/// the plane's Newell functional, twice the run's signed enclosed
/// area — decided through the `bool_ring_run_winding` predicate in
/// the k_stats funnel; `Indeterminate` escalates. Zero is a
/// degenerate area-free run and a loud desync (the ring lane only
/// closes full island cycles — slit-growing joins are mekr-lane
/// merges). The ring lane is planar-scoped like
/// [`super::solid_contain::point_in_solid`]'s
/// F5 gate: a non-planar carrier refuses loudly.
///
/// # Dimension (audit F4, `docs/predicate-dimension-audit.md`)
///
/// The CANONICAL statement for this predicate's three sites (the other
/// two are `merge_faces::loop_winding` and `validate`'s tier-3 check 6,
/// which cross-reference here): the Newell functional is an AREA (m²)
/// and ε is a point deviation (D4), so the decided margin divides it by
/// the run's boundary PERIMETER `P`. `2A/P` is the region's MEAN WIDTH
/// — exactly the deviation the winding sign is about: it is the
/// distance the boundary would have to move to sweep the enclosed
/// region away, so a margin above ε says "this ring encloses material
/// no ε-scale point perturbation can unwind", and one below it says the
/// ring is thinner than the model's own resolution. Precedents:
/// `validate`'s `positive_volume` (V/A) and `split_section_area`
/// (2|A|/P, the same mean width in the splitter).
///
/// `P` is the closed region's own boundary: each run half-edge
/// contributes its arc length (conics: `|Δ|·semi-major` — exact for a
/// circle, an upper bound for an ellipse, and an over-large `P`
/// understates the width, i.e. escalates rather than decides), and the
/// closing chord `end(h2) → p₀` contributes its length. A run whose
/// perimeter is exactly zero (every vertex coincident, no arcs) poisons
/// `0/0` and escalates typed rather than reaching the `Zero` desync arm
/// below — a refusal either way.
///
/// **Orientation (S10)**: the margin multiplies two differently-sourced
/// signs and needs exactly ONE of them threaded. The Newell sum is
/// winding — read off the run's STORED traversal order, which `revert`
/// reverses — so it already flips with the sense bit and must not be
/// touched. The normal is a CHART read standing in for the face's
/// outward normal, so it is multiplied by `sense_sign`. Threading both
/// would cancel (the classic double-count); threading neither leaves
/// "CCW around the outward normal" meaning "CCW around the chart
/// normal", the opposite statement on a reversed face — and this
/// verdict is what picks an island's new outer boundary.
fn ring_run_ccw<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    h1: HalfEdgeKey,
    h2: HalfEdgeKey,
    band: Band,
) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let normal = match body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface).map(|s| (f, s)))
    {
        Some((f, geom::Surface::Plane { normal, .. })) => *normal * f.sense_sign::<T>(),
        _ => return Err(desync("ring-lane face has no planar carrier")),
    };
    let point_of = |he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
        let v = body
            .get_half_edge(he)
            .ok_or(desync("run half no longer resolves"))?
            .start;
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or(desync("run vertex has no point"))
    };
    let end_point_of = |he: HalfEdgeKey| -> Result<geom_core::Point3<T>, BooleanError> {
        let v = body
            .half_edge_end(he)
            .ok_or(desync("run half no longer resolves"))?;
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point).copied())
            .ok_or(desync("run vertex has no point"))
    };
    let p0 = point_of(h1)?;
    let mut newell = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
    // The metering lever (fn docs, audit F4): the closed region's own
    // boundary length, accumulated edge by edge alongside the area.
    let mut perimeter = T::zero();
    let mut prev = p0;
    let mut he = h1;
    let mut steps = 0usize;
    let cap = body.half_edges().count(); // hoisted: O(n) guard, not O(n²)
    // Conic run edges add their BULGE term (M5 PR 9 fix pass, dev 4):
    // the chord Newell sum alone reads a two-semicircle 2-gon as zero
    // area — a structural degeneracy of the chord approximation, not
    // of the region. Each arc contributes the closed-form vector area
    // between itself and its chord, `axis · sa·sb·(Δ − sin Δ)` (twice
    // the segment area, matching the cross-sum's 2A convention;
    // signed by traversal, an odd function of Δ).
    //
    // The same walk yields the half-edge's own BOUNDARY LENGTH (the F4
    // metering lever): a conic contributes its arc length, everything
    // else its chord. One curve lookup, both terms.
    let run_term = |he: HalfEdgeKey| -> Result<(geom_core::Vec3<T>, T), BooleanError> {
        let zero = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
        let chord =
            || -> Result<T, BooleanError> { Ok((end_point_of(he)? - point_of(he)?).norm()) };
        let he_data = body
            .get_half_edge(he)
            .ok_or(desync("run half no longer resolves"))?;
        let edge = body
            .get_edge(he_data.edge)
            .ok_or(desync("run edge no longer resolves"))?;
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .and_then(crate::null::CurveGeom::certified)
        else {
            return Ok((zero, chord()?));
        };
        let (t0, t1) = curve.params();
        let (axis, sa, sb) = match *curve.carrier() {
            geom::Curve3::Circle { axis, radius, .. } => (axis, radius, radius),
            geom::Curve3::Ellipse {
                axis, major, minor, ..
            } => (axis, major, minor),
            geom::Curve3::Line { .. } | geom::Curve3::Nurbs(_) => {
                return Ok((zero, chord()?));
            }
        };
        let span = if edge.he_plus == he { t1 - t0 } else { t0 - t1 };
        // `|Δ|·sa` is the circle's exact arc length and the ellipse's
        // upper bound (fn docs: over-large P escalates, never decides).
        Ok((axis * (sa * sb * (span - span.sin())), span.abs() * sa))
    };
    loop {
        let (bulge, len) = run_term(he)?;
        newell = newell + bulge;
        perimeter = perimeter + len;
        if he != h1 {
            let p = point_of(he)?;
            newell = newell + (prev - p0).cross(p - p0);
            prev = p;
        }
        if he == h2 {
            break;
        }
        he = body
            .get_half_edge(he)
            .ok_or(desync("run half no longer resolves"))?
            .next;
        steps += 1;
        if steps > cap {
            return Err(desync("ring-run arc did not close"));
        }
    }
    let end = end_point_of(h2)?;
    newell = newell + (prev - p0).cross(end - p0);
    // The chord that closes the region (fn docs): the run is open, the
    // area it decides is not.
    perimeter = perimeter + (end - p0).norm();
    let escalate = |diag| BooleanError::Escalated { diag };
    // `normal` carries the sense, `newell` carries the traversal: one
    // factor each, never both (fn docs — the double-count hazard).
    // `/ perimeter` is the F4 metering: 2A/P, the run's mean width.
    match decide(
        "bool_ring_run_winding",
        Margin::over_lever(normal.dot(newell), perimeter),
        band,
    )
    .map_err(escalate)?
    {
        geom_core::Sign::Positive => Ok(true),
        geom_core::Sign::Negative => Ok(false),
        geom_core::Sign::Zero => Err(desync(
            "ring-run winding is degenerate (zero enclosed area)",
        )),
    }
}

/// The clean chord-arc role order, if any (doc at [`choose_roles`]):
/// `Some((h1, h2))` such that the `next`-order arc `h1 → h2` avoids
/// every `unused` half; different loops trivially clean.
fn clean_dir<T: Decide>(
    body: &Body<T>,
    ea: HalfEdgeKey,
    ra: HalfEdgeKey,
    loose: &SecondaryMap<HalfEdgeKey, Option<HalfEdgeKey>>,
) -> Result<Option<(HalfEdgeKey, HalfEdgeKey)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let loop_of = |he: HalfEdgeKey| -> Result<crate::entity::LoopKey, BooleanError> {
        Ok(body
            .get_half_edge(he)
            .ok_or(desync("role half no longer resolves"))?
            .parent_loop)
    };
    if loop_of(ea)? != loop_of(ra)? {
        return Ok(Some((ea, ra)));
    }
    // A direction is BAD iff its arc SEPARATES a loose half from its
    // match partner (captures exactly one of a partner pair, or a
    // half with no computable partner — the capture would wall it off
    // on the new face where its partner cannot reach it). Capturing a
    // complete partner pair together is harmless: they still share a
    // face and join there.
    let separates = |from: HalfEdgeKey, to: HalfEdgeKey| -> Result<bool, BooleanError> {
        let mut inside: Vec<HalfEdgeKey> = Vec::new();
        let mut he = body
            .get_half_edge(from)
            .ok_or(desync("role arc start no longer resolves"))?
            .next;
        let mut steps = 0usize;
        while he != to {
            if loose.contains_key(he) {
                inside.push(he);
            }
            he = body
                .get_half_edge(he)
                .ok_or(desync("role arc left the loop"))?
                .next;
            steps += 1;
            if steps > body.half_edges().count() {
                return Err(desync("role arc did not close"));
            }
        }
        for &h in &inside {
            match loose.get(h).copied().flatten() {
                // Last loose half of its record: its partner is at
                // another site — separated.
                None => return Ok(true),
                Some(sib) => {
                    if !inside.contains(&sib) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    };
    if !separates(ea, ra)? {
        Ok(Some((ea, ra)))
    } else if !separates(ra, ea)? {
        Ok(Some((ra, ea)))
    } else {
        Ok(None)
    }
}

/// Cut the corresponding null edges in both solids; completions must
/// coincide (lockstep). Roles stay UNRESOLVED here (resolved at
/// quiescence — see `bool_connect`).
fn cut_pair<T: Decide>(
    red: &mut BooleanReduction<T>,
    sa: &mut SolidJoin,
    sb: &mut SolidJoin,
    completed: &mut Vec<UnresolvedPair>,
    a_edge: EdgeKey,
    b_edge: EdgeKey,
) -> Result<(), BooleanError> {
    let a_out = sa
        .joiner
        .cut_core(&mut red.a, a_edge)
        .map_err(BooleanError::Join)?;
    let b_out = sb
        .joiner
        .cut_core(&mut red.b, b_edge)
        .map_err(BooleanError::Join)?;
    match (a_out, b_out) {
        (CutOutcome::Merged, CutOutcome::Merged) => Ok(()),
        (
            CutOutcome::Completed {
                face: a_face,
                ring: a_ring,
            },
            CutOutcome::Completed {
                face: b_face,
                ring: b_ring,
            },
        ) => {
            let desync = |what| BooleanError::JoinDesync { what };
            let a_outer = red
                .a
                .get_face(a_face)
                .ok_or(desync("completed A face no longer resolves"))?
                .outer;
            let b_outer = red
                .b
                .get_face(b_face)
                .ok_or(desync("completed B face no longer resolves"))?
                .outer;
            completed.push(UnresolvedPair {
                a_face,
                a_outer,
                a_ring,
                b_face,
                b_outer,
                b_ring,
            });
            Ok(())
        }
        _ => Err(BooleanError::JoinDesync {
            what: "one solid completed a polygon where the other merged slivers",
        }),
    }
}

/// GEOMETRIC loop-role resolution for a completed section polygon
/// (M3 PR 5, the cookie-cutter finding): a loop of the 2-loop null
/// face is the IN copy iff the region material adjacent to it (the
/// faces holding its chords' mates) lies inside the OTHER body —
/// decided by probing the region faces' NON-seam vertices with
/// [`super::solid_contain::point_in_solid`] against the pristine other
/// operand (seam
/// vertices sit ON the other boundary and are skipped via the
/// trilean's `OnBoundary`). The two regions flank the seam, so one
/// definitive verdict fixes both roles; agreeing verdicts on both
/// loops are the loud [`SplitJoinError::SectionLoopMixed`]. This
/// never consults strut side labels — pierce-ring struts carry
/// provisional labels (PR 4's flag), and single-face seam rings have
/// no in-solid label anchor; geometry is the anchor.
///
/// Anchor tiers (issue #93, the A×Z finding): vertices first — the
/// original M3 PR 5 anchor, exhausted over BOTH loops before any new
/// probing so the existing corpus sees a bit-identical predicate
/// stream — then EDGE MIDPOINTS of the same region loops in the same
/// iteration order. An isolated seam polygon can leave every flanking
/// region bounded entirely by seam vertices (all `OnBoundary`), yet
/// its non-seam edges' interiors classify definitively; the midpoint
/// (`lerp` at ½, the [`super::ops`] witness-point precedent) is probed
/// through the same [`super::solid_contain::point_in_solid`]
/// reified-predicate funnel — no
/// new predicate, no epsilon comparison. Seam-chord midpoints lie ON
/// the other boundary and are skipped by the trilean like seam
/// vertices. Third, REGION-INTERIOR candidates (the nested-island
/// case: an island's surround bounded entirely by seam chords of TWO
/// seam loops — every vertex and midpoint on the other boundary):
/// vertex-triple centroids accepted only when the reified
/// `point_in_face` certifies them strictly interior, then probed the
/// same way. Fourth (issue #106), VERTEX-PAIR CHORD MIDPOINTS: the
/// midpoint of the anchor vertex and every other vertex of the same
/// region face, across its outer loop AND all its rings, under the
/// same `point_in_face` certificate. The triple centroid is a local
/// guess a nonconvex or annular region defeats (a square annulus
/// between two seam loops — depth-2 island nesting, island ⊃ ring ⊃
/// island on one face — lands every consecutive-triple centroid
/// inside the hole); the chord tier is global, and finds an interior
/// point whenever the region admits any vertex-to-vertex diagonal,
/// which every polygon-with-holes region of ≥ 4 vertices does.
///
/// The typed refusal below stays LOAD-BEARING, not a dead backstop.
/// Post-#106 the known residue is: regions lying INSIDE the other
/// body's boundary surface (the coincident-plane class) exhaust all
/// four tiers — every candidate, interior or not, is `OnBoundary`
/// against the other solid — though post-N6 that class normally
/// refuses earlier, at the coincidence door; and any region whose
/// every certified interior candidate still reads `OnBoundary`.
/// Candidate generation remains a heuristic in the strict sense (it
/// is not a full constrained triangulation), so the arm is kept
/// unconditionally: an uncertified or inconclusive candidate is
/// discarded unprobed and the refusal is typed — never wrongness,
/// never classification by guess.
fn resolve_roles_geometric<T: Decide>(
    body: &Body<T>,
    other_pristine: &Body<T>,
    face: FaceKey,
    outer: LoopKey,
    ring: LoopKey,
    band: Band,
    tol: Tol,
) -> Result<(LoopKey, LoopKey), BooleanError> {
    /// Which point of a region-loop half-edge anchors the probe.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Anchor {
        /// The half-edge's start vertex (the M3 PR 5 anchor).
        Vertex,
        /// The half-edge's chord midpoint (issue #93's second tier).
        EdgeMidpoint,
        /// A verified region-interior point (issue #93's third tier —
        /// the nested-island case): the centroid of the half-edge's
        /// vertex triple, ACCEPTED only when the reified
        /// [`point_in_face`](super::solid_contain::point_in_face)
        /// certifies it strictly interior to the region face —
        /// candidates are guesses, the gate is a predicate. Needed
        /// when a region is bounded entirely by seam CHORDS (an
        /// island's surround between two seam loops): every vertex
        /// and every edge midpoint lies ON the other boundary, yet
        /// the region interior classifies definitively.
        RegionInterior,
        /// A verified region-interior point from a VERTEX-PAIR CHORD
        /// (issue #106's fourth tier): the midpoint of the half-edge's
        /// start vertex and every other vertex of the region face
        /// (its outer loop and ALL its rings), each accepted only
        /// when `point_in_face` certifies it strictly interior.
        /// Where the triple centroid is a local guess that a
        /// nonconvex/annular region defeats, this tier sweeps the
        /// face's full vertex set, so it finds an interior point
        /// whenever the region admits a vertex-to-vertex diagonal —
        /// the diagonal's midpoint is strictly interior by
        /// construction, and every polygon-with-holes region of ≥ 4
        /// vertices admits one (a triangulation without Steiner
        /// points always exists; any of its non-boundary edges is
        /// such a diagonal). Triangle regions carry no diagonal but
        /// are already covered by [`Anchor::RegionInterior`].
        RegionVertexChord,
    }
    impl Anchor {
        /// Do this tier's candidates need the `point_in_face` strict-
        /// interiority certificate before they may be probed? Tiers 1
        /// and 2 sit ON the region boundary by construction (the
        /// trilean's `OnBoundary` skips the ones that matter); tiers 3
        /// and 4 are GUESSES until a reified predicate certifies them.
        fn needs_interior_certificate(self) -> bool {
            matches!(self, Anchor::RegionInterior | Anchor::RegionVertexChord)
        }
    }
    let desync = |what| BooleanError::JoinDesync { what };
    let probe = |l: LoopKey, anchor: Anchor| -> Result<Option<bool>, BooleanError> {
        let crate::entity::LoopBoundary::Cycle { first } = body
            .get_loop(l)
            .ok_or(desync("completed section loop no longer resolves"))?
            .boundary
        else {
            return Err(desync("completed section loop is empty"));
        };
        for ch in body
            .loop_cycle(first)
            .ok_or(desync("completed section loop not walkable"))?
        {
            let mate = body.mate(ch).ok_or(desync("section half has no mate"))?;
            let region_loop = body
                .get_half_edge(mate)
                .ok_or(desync("section mate no longer resolves"))?
                .parent_loop;
            let region_face = body
                .get_loop(region_loop)
                .ok_or(desync("region loop no longer resolves"))?
                .face;
            if region_face == face {
                continue;
            }
            let f = body
                .get_face(region_face)
                .ok_or(desync("region face no longer resolves"))?;
            for rl in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                let crate::entity::LoopBoundary::Cycle { first: rf } = body
                    .get_loop(rl)
                    .ok_or(desync("region loop no longer resolves"))?
                    .boundary
                else {
                    continue;
                };
                for rhe in body
                    .loop_cycle(rf)
                    .ok_or(desync("region loop not walkable"))?
                {
                    let v = body
                        .get_half_edge(rhe)
                        .ok_or(desync("region half no longer resolves"))?
                        .start;
                    let start = body
                        .get_vertex(v)
                        .and_then(|vd| body.get_point(vd.point).copied())
                        .ok_or(desync("region vertex has no point"))?;
                    let end_of = |he: HalfEdgeKey| {
                        body.half_edge_end(he)
                            .and_then(|ev| body.get_vertex(ev))
                            .and_then(|vd| body.get_point(vd.point).copied())
                            .ok_or(desync("region half has no end point"))
                    };
                    let cands: Vec<geom_core::Point3<T>> = match anchor {
                        Anchor::Vertex => vec![start],
                        Anchor::EdgeMidpoint => vec![start.lerp(end_of(rhe)?, T::from_f64(0.5))],
                        Anchor::RegionInterior => {
                            let b = end_of(rhe)?;
                            let c = end_of(
                                body.get_half_edge(rhe)
                                    .ok_or(desync("region half no longer resolves"))?
                                    .next,
                            )?;
                            vec![start + ((b - start) + (c - start)) * T::from_f64(1.0 / 3.0)]
                        }
                        Anchor::RegionVertexChord => face_vertex_points(body, region_face)?
                            .into_iter()
                            .map(|q| start.lerp(q, T::from_f64(0.5)))
                            .collect(),
                    };
                    for p in cands {
                        if anchor.needs_interior_certificate() {
                            // The normal is only a projection frame for
                            // `point_in_face`'s ray parity, which is
                            // blind to its sign; `face_plane` hands out
                            // the oriented one regardless (S10).
                            let (_, normal) = super::solid_contain::face_plane(body, region_face)
                                .map_err(BooleanError::Containment)?;
                            if super::solid_contain::point_in_face(
                                body,
                                region_face,
                                normal,
                                p,
                                band,
                            )
                            .map_err(BooleanError::Containment)?
                                != Some(true)
                            {
                                // Not certified interior (outside, in a
                                // ring, or grazing a loop): candidate
                                // discarded, never probed.
                                continue;
                            }
                        }
                        match super::solid_contain::point_in_solid(other_pristine, p, band, tol)
                            .map_err(BooleanError::Containment)?
                        {
                            super::solid_contain::SolidContainment::In => return Ok(Some(true)),
                            super::solid_contain::SolidContainment::Out => return Ok(Some(false)),
                            super::solid_contain::SolidContainment::OnBoundary => continue,
                        }
                    }
                }
            }
        }
        Ok(None)
    };
    // One anchor tier at a time, both loops, before the next tier —
    // the vertex tier is exhausted first so the existing corpus sees
    // an unchanged predicate stream (doc above).
    let resolve = |anchor: Anchor| -> Result<Option<(LoopKey, LoopKey)>, BooleanError> {
        Ok(match probe(outer, anchor)? {
            Some(outer_in) => {
                // The two regions flank the seam: the other loop takes
                // the opposite role (checked when it also resolves).
                // Defensive guard (PR 5.5 review, MINOR (b)): no
                // constructible agreeing-verdicts witness is known
                // post-discipline; kept as a loud backstop, never
                // deleted.
                if probe(ring, anchor)? == Some(outer_in) {
                    return Err(BooleanError::Join(SplitJoinError::SectionLoopMixed {
                        face,
                    }));
                }
                if outer_in {
                    Some((outer, ring))
                } else {
                    Some((ring, outer))
                }
            }
            None => match probe(ring, anchor)? {
                Some(ring_in) => {
                    if ring_in {
                        Some((ring, outer))
                    } else {
                        Some((outer, ring))
                    }
                }
                None => None,
            },
        })
    };
    if let Some(roles) = resolve(Anchor::Vertex)? {
        return Ok(roles);
    }
    if let Some(roles) = resolve(Anchor::EdgeMidpoint)? {
        return Ok(roles);
    }
    if let Some(roles) = resolve(Anchor::RegionInterior)? {
        return Ok(roles);
    }
    match resolve(Anchor::RegionVertexChord)? {
        Some(roles) => Ok(roles),
        None => Err(desync(
            "neither section loop's regions hold a classifiable anchor \
             (vertices, edge midpoints, and verified interior candidates \
             all exhausted)",
        )),
    }
}

/// Every vertex point of `face`, outer loop then rings in arena
/// order — the candidate partner set for [`Anchor::RegionVertexChord`].
/// Deterministic; duplicates (a vertex visited by two loops) are kept
/// so the order never depends on point comparison.
fn face_vertex_points<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<geom_core::Point3<T>>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let f = body
        .get_face(face)
        .ok_or(desync("region face no longer resolves"))?;
    let mut out = Vec::new();
    for fl in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let crate::entity::LoopBoundary::Cycle { first } = body
            .get_loop(fl)
            .ok_or(desync("region loop no longer resolves"))?
            .boundary
        else {
            continue;
        };
        for he in body
            .loop_cycle(first)
            .ok_or(desync("region loop not walkable"))?
        {
            let v = body
                .get_half_edge(he)
                .ok_or(desync("region half no longer resolves"))?
                .start;
            out.push(
                body.get_vertex(v)
                    .and_then(|vd| body.get_point(vd.point).copied())
                    .ok_or(desync("region vertex has no point"))?,
            );
        }
    }
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod frame_dispatch_tests {
    use geom_core::{Point3, Tol, Vec3};

    use super::{FrameError, pair_section_frame};

    fn band() -> geom_core::Band {
        geom_core::Band::linear(Tol::witness()).expect("a linear band")
    }

    fn plane() -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cylinder(axis: Vec3<f64>) -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis,
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The trap, closed: a germ pair with no section arm must refuse,
    /// never answer `None`. `None` is the straight-chord verdict, and
    /// handing it to a curved pair mints a chord along a locus that is
    /// not a line.
    #[test]
    fn a_pair_without_an_arm_refuses_and_never_answers_straight() {
        let curved_pairs = [
            (
                cylinder(Vec3::new(0.0, 0.0, 1.0)),
                cylinder(Vec3::new(1.0, 0.0, 0.0)),
            ),
            (cylinder(Vec3::new(0.0, 0.0, 1.0)), sphere()),
            (sphere(), sphere()),
        ];
        for (a, b) in curved_pairs {
            let got = pair_section_frame(&a, &b, band());
            assert!(
                matches!(got, Err(FrameError::NoArm)),
                "a pair with no arm must refuse rather than default to the straight chord"
            );
        }
    }

    /// The one pair that EARNS the straight answer, and the wired
    /// curved pairs that name a frame — so the row above is a
    /// statement about missing arms, not about the dispatch refusing
    /// everything.
    #[test]
    fn the_wired_pairs_keep_their_verdicts() {
        assert!(
            matches!(pair_section_frame(&plane(), &plane(), band()), Ok(None)),
            "plane×plane is straight by construction"
        );
        // A plane cutting a cylinder square across its axis: the rim
        // circle, whose frame is the section's own centre and axis.
        assert!(
            matches!(
                pair_section_frame(&plane(), &cylinder(Vec3::new(0.0, 0.0, 1.0)), band()),
                Ok(Some(_))
            ),
            "plane×cylinder names its conic frame"
        );
        // A plane containing the axis: the section is two rulings, a
        // STRAIGHT locus that the arm proved rather than defaulted to.
        assert!(
            matches!(
                pair_section_frame(&plane(), &cylinder(Vec3::new(1.0, 0.0, 0.0)), band()),
                Ok(None)
            ),
            "the parallel-lines outcome is a proven straight locus"
        );
        assert!(
            matches!(pair_section_frame(&plane(), &sphere(), band()), Ok(Some(_))),
            "plane×sphere names its circle frame"
        );
    }
}
