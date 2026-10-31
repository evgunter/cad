//! The reduction sweep (ch. 15 §15.5, Programs 15.2–15.4 re-derived):
//! all-pairs edge×face in BOTH directions, realizing the eight-step
//! specification in one sweep with `contfp`/`contfv` typed case codes.
//!
//! - **Candidate generation through the `bvh` tree** (M5 PR 8, C10 —
//!   the documented quadratic of M3, retired): each edge fragment
//!   queries the per-direction face tree ([`super::boxes`], padded
//!   vertex-extent boxes) instead of scanning every face. THE TREE
//!   PRUNES, PREDICATES DECIDE: candidates arrive in ascending face
//!   arena order (a subsequence of the brute-force scan), the exact
//!   per-pair classification below is untouched, and the conservative
//!   pad guarantees every pair the exact predicates would accept
//!   survives — so results are bit-identical to the brute-force sweep
//!   by construction, and the idealized/realized differential suite
//!   (PERF-PLAN §4.4; `tests/m5_pr8_bvh_diff.rs`, the corpus suite in
//!   editor-core) pins it: realized candidates ⊇ idealized accepted
//!   pairs, final results bit-equal, planted degradation caught. The
//!   brute-force scan survives as [`SweepStrategy::Idealized`] — the
//!   ten-line definition of the candidate set. One documented
//!   divergence, error channel only: a pair whose boxes are disjoint
//!   can still ESCALATE the brute path's `bool_vertex_face_side` when
//!   an edge grazes a face's *infinite* plane far from the face
//!   itself; the realized path never examines it. Pruning can drop
//!   only such spurious escalations, never an accepted event — the
//!   value channel is pinned bit-equal. In the full boolean the
//!   disjoint-operands containment witness passes over a point that
//!   reads in-band against the same plane (`super::shell_witness`), so
//!   the realized path answers wherever another witness of the shell
//!   decides, and refuses where none does; the suite's grazing fixture
//!   pins one that answers.
//! - **Worklist, not recursion** (Problem 15.3 / F12): a proper
//!   crossing splits the edge through the certified `split_edge` lane
//!   and pushes BOTH children back with the *next* face index (a line
//!   crosses a plane at most once, so the split face is done with both
//!   children); each split strictly shortens spans — termination is
//!   structural.
//! - **Coplanar edge-face pairs are skipped** (a line with both
//!   endpoints ON the face plane, or a conic lying in it ⇒ endpoint
//!   processing only): every relevant crossing inside the face is
//!   caught when the edge is swept against the face's noncoplanar
//!   NEIGHBOR faces, where the crossing point lands ON the shared
//!   boundary edge (the `OnEdge` case — tested by the coplanar-overlap
//!   acceptance fixture).
//! - **Edge-on-edge crossings** are discovered as edge-face events
//!   landing ON an edge of the face: BOTH edges are split at the
//!   (bitwise-shared) intersection point — the minted vertices are a
//!   v-v contact record by construction.
//! - Sweep order (D9): direction A→B fully, then B→A; edges in arena
//!   order, faces in arena snapshot order, worklist FIFO. Then the
//!   settle stage ([`settle_deferred`]): the covered touches a direction
//!   deferred are read again on the fragments both directions left,
//!   in deferral order ([`sweep_and_settle`] runs all three).
//!
//! The module also hosts the two **pre-sweep gates**, which refuse an
//! operand pair before any edge is split: the operand gate
//! ([`gate_operand_pairs`]: each operand passes tier 2, and a kind
//! with no wired arm may not enter an undeclared pair), and the
//! maximal-faces gate ([`gate_maximal_faces`]: no operand carries two
//! coplanar neighbours).

use geom_core::{Band, Bounds, Decide, Margin, MarginDiag, Point3, Sign, Vec3};

use super::boxes;
use super::circle_roots::{CircleRoots, door_refusal};
use super::contain::{ContainError, CurvedPlacement, FaceContainment, contfp_decided};
use super::plane_eq::PlaneDesc;
use super::separating::Item;
use super::{BooleanDecision, Coincide, CrossingDecision, DeclarationRead};
use super::{BooleanError, Operand, ReductionContacts, VfContact, VvContact};
use crate::body::Body;
use crate::coincidence::{Cited, Cites};
use crate::entity::{EdgeKey, EntityId, FaceKey, VertexKey};
use crate::null::CurveGeom;
use crate::offset_nappe::NappeRefusal;
use crate::sector_shape::SlitTips;
use crate::splitting::{ConicPlaneMeet, PlaneCrossingLane, PlaneSide as SplitSide};
use crate::validate::decide;
use crate::vertex_section::{Offset, SectionPlane, VertexRefusal};
use geom_brep::roots::{Roots, Unsettled};
use geom_core::Tol;

/// Which candidate-generation path the reduction sweep runs — the
/// idealized/realized pair of PERF-PLAN §4.4 (the pattern is only
/// permitted WITH its differential suite; see the module docs).
/// Production entries always run [`SweepStrategy::Realized`]; the
/// idealized path is the executable definition of the candidate set,
/// kept alive for the suite's pins.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SweepStrategy {
    /// BVH-pruned candidate generation (the production path).
    Realized,
    /// Brute-force all-pairs (the reference definition).
    ///
    /// `sweep-testing` feature only. The idealized half of a §4.4 pair
    /// is reference surface, exactly like [`PlantedDegradation`] and
    /// [`super::sweep_traces`] — it exists so the differential suite
    /// can execute the definition, not so a production caller can
    /// choose O(n²) candidate generation. Gating the variant is what
    /// makes "production entries always run [`SweepStrategy::Realized`]"
    /// a fact the compiler enforces rather than a convention; with the
    /// feature off the brute-force scan is not merely unreachable, it
    /// is not built (see `sweep_direction`).
    #[cfg(feature = "sweep-testing")]
    Idealized,
}

/// One direction's sweep observations, for the differential suite's
/// superset pin: `examined` = pairs whose exact classification ran
/// (the candidate set), `accepted` = pairs where the exact predicates
/// accepted at least one event (a crossing inside the face, or an
/// endpoint contact). Pairs are `(edge of x, face of y)` in
/// examination order.
#[derive(Debug, Default, Clone)]
pub struct SweepTrace {
    /// Every candidate pair the exact path examined.
    pub examined: Vec<(EdgeKey, FaceKey)>,
    /// The subset of pairs that produced an accepted event.
    pub accepted: Vec<(EdgeKey, FaceKey)>,
}

/// The suite's failure-injection seam (pin iii — "the suite must be
/// able to fail"): shrink ONE face's box to the poison-free EMPTY box
/// before building the tree, so candidate generation loses whatever
/// events that face carries and the superset pin must catch it.
/// `sweep-testing` feature only — no production consumer can name a
/// failure injector (M5 PR 8 fix pass, item 2).
#[cfg(feature = "sweep-testing")]
#[derive(Debug, Clone, Copy)]
pub struct PlantedDegradation {
    /// The face whose box is planted empty.
    pub face: FaceKey,
}

/// Internal candidate-generation knobs (private plumbing; the PUBLIC
/// doors that can set anything non-default are `sweep-testing`-gated).
/// Production entries always pass `SweepKnobs::default()`.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct SweepKnobs {
    /// Pin (iii): plant this face's box empty.
    pub(super) plant: Option<FaceKey>,
    /// Pin 1(b): override [`boxes::sweep_pad`] (a DELIBERATELY
    /// breakable knob — the suite proves a too-small pad is caught).
    pub(super) pad_override: Option<f64>,
}

/// The (deduplicating, order-preserving) contact accumulator.
#[derive(Default)]
pub(super) struct ContactAcc {
    records: ReductionContacts,
    /// Every edge split the sweep made, in split order.
    pub(super) splits: super::Mints,
    /// Every decision a push made, in decision order: the rows the
    /// records cite ([`Backing::Decided`](crate::Backing::Decided) names an index here until
    /// the result's carry renumbers it).
    pending: Vec<PendingRow>,
    seen_vv: std::collections::BTreeMap<(VertexKey, VertexKey), usize>,
    seen_ab: std::collections::BTreeMap<(VertexKey, FaceKey), usize>,
    seen_ba: std::collections::BTreeMap<(VertexKey, FaceKey), usize>,
    /// The bisector plane each vertex the sweep placed on an equal
    /// cylinder pair's partner wall lies on ([`factored_crossing`]), by
    /// its operand: the pierce reads that wall through it.
    pub(super) branches: std::collections::BTreeMap<(Operand, VertexKey), super::Branch>,
    /// The chart point each vertex the sweep placed inside a spline face
    /// was placed at, on the face's own sheet, by the vertex's operand and
    /// the face: the pierce reads the face's normal there
    /// ([`crate::face_normal::spline_outward_normal_at`]). The first
    /// placement of a pair is kept.
    pub(super) charts: std::collections::BTreeMap<(Operand, VertexKey, FaceKey), (f64, f64)>,
}

/// **A vertex identity the reduction decided**: one with a margin is a
/// coincidence, whether or not a surviving record cites it, published
/// as a row ([`super::ops::carry`]); a landing
/// no margin decided only placed topology (D1). The result's carry
/// emits each coincidence as a [`crate::Coincidence`]
/// ([`super::ops::carry`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PendingRow {
    /// The two cells, each in its operand clone's keys: a vertex the
    /// reduction minted is still a vertex here, and the carry reads it
    /// back to the input cell it was minted in ([`super::Mints`]). A
    /// ring tie's pierced face, which its row also names, is kept beside
    /// the row (`ops::Ledger`).
    pub(crate) cells: [(Operand, super::Cell); 2],
    /// What was decided between them.
    pub(crate) relation: crate::Relation,
    /// The margin decided Zero, or `None` for a landing no margin put
    /// on the other cell (a transverse pierce strictly inside a face),
    /// which backs no touch.
    pub(crate) margin: Option<MarginDiag>,
}

impl PendingRow {
    /// The first cell on the second, decided by `margin`.
    pub(crate) fn on(
        first: (Operand, super::Cell),
        second: (Operand, super::Cell),
        margin: Option<MarginDiag>,
    ) -> Self {
        Self {
            cells: [first, second],
            relation: crate::Relation::OnCarrier,
            margin,
        }
    }

    /// Face `a` of A tangent to face `b` of B, decided by `margin`
    /// (the glue door's tangency, [`super::tangent_row`]'s decision).
    pub(crate) fn tangent(a: FaceKey, b: FaceKey, margin: MarginDiag) -> Self {
        Self {
            cells: [
                (Operand::A, super::Cell::Face(a)),
                (Operand::B, super::Cell::Face(b)),
            ],
            relation: crate::Relation::SurfaceTangent,
            margin: Some(margin),
        }
    }
}

impl ContactAcc {
    /// The citation of `row`: the row already pending for the same
    /// question, or `row` as the next.
    fn decide(&mut self, row: PendingRow) -> Cites {
        Cites::decided(index(pend(&mut self.pending, row)))
    }
    pub(super) fn vv(&mut self, c: VvContact, row: PendingRow) {
        let cites = self.decide(row);
        match self.seen_vv.get(&(c.a, c.b)) {
            Some(&i) => {
                let at = &mut self.records.vv[i].cites;
                *at = at.union(&cites);
            }
            None => {
                self.seen_vv.insert((c.a, c.b), self.records.vv.len());
                self.records.vv.push(Cited::new(c, cites));
            }
        }
    }
    pub(super) fn vf(&mut self, piercing: Operand, c: VfContact, row: PendingRow) {
        let cites = self.decide(row);
        let (seen, list) = match piercing {
            Operand::A => (&mut self.seen_ab, &mut self.records.a_on_b),
            Operand::B => (&mut self.seen_ba, &mut self.records.b_on_a),
        };
        match seen.get(&(c.vertex, c.face)) {
            Some(&i) => {
                let at = &mut list[i].cites;
                *at = at.union(&cites);
            }
            None => {
                seen.insert((c.vertex, c.face), list.len());
                list.push(Cited::new(c, cites));
            }
        }
    }
    /// Records the chart point `uv` of `face` that `vertex` of `x_is`
    /// was placed inside the face at ([`Self::charts`]).
    pub(super) fn chart_point(
        &mut self,
        x_is: Operand,
        vertex: VertexKey,
        face: FaceKey,
        uv: (f64, f64),
    ) {
        self.charts.entry((x_is, vertex, face)).or_insert(uv);
    }

    /// The chart points the sweep recorded ([`Self::charts`]), taken out.
    pub(super) fn take_charts(
        &mut self,
    ) -> std::collections::BTreeMap<(Operand, VertexKey, FaceKey), (f64, f64)> {
        core::mem::take(&mut self.charts)
    }

    /// The branches the sweep recorded ([`Self::branches`]), taken out.
    pub(super) fn take_branches(
        &mut self,
    ) -> std::collections::BTreeMap<(Operand, VertexKey), super::Branch> {
        core::mem::take(&mut self.branches)
    }

    /// The records, each citing rows of the decisions alongside them.
    pub(super) fn finish(self) -> (ReductionContacts, Vec<PendingRow>) {
        (self.records, self.pending)
    }
}

/// The index of `row` in `pending`: the row already there that asks the
/// same question of the same two cells (the sweep reaches one vertex
/// from every edge and face it bounds, and each reach decides the same
/// margin), else `row`, pushed. A row with a margin replaces one
/// without, so a question one reach decided and another only placed is
/// the decided one.
pub(super) fn pend(pending: &mut Vec<PendingRow>, row: PendingRow) -> usize {
    let same = |p: &PendingRow| {
        p.relation == row.relation
            && (p.cells == row.cells || p.cells == [row.cells[1], row.cells[0]])
    };
    match pending.iter().position(same) {
        Some(k) => {
            if pending[k].margin.is_none() {
                pending[k].margin = row.margin;
            }
            k
        }
        None => {
            pending.push(row);
            pending.len() - 1
        }
    }
}

/// A list index as a citation's.
pub(crate) fn index(k: usize) -> u32 {
    u32::try_from(k).unwrap_or_else(|_| unreachable!("more than u32::MAX decisions in one op"))
}

/// **The face kinds with at least one wired boolean arm** — `Plane`,
/// `Cylinder` (the PR 5 conic arms), `Sphere` (the PR 7
/// cylinder×sphere SSI arm, structurally routed), `Torus` (the
/// crossing layer's torus arms: the circle rung's enclosure and its
/// carrier-identity rung, the certified line×torus quartic, the chart
/// containment, and the sector and pierce normals) and `Nurbs` (the
/// plane×NURBS arm, routed structurally so PR 7b's flag flip alone
/// makes it live) and `Cone` (the certified line × cone quadratic and
/// circle × cone roots, the apex-closed containment, the section
/// certificate's cone rows, the sector and pierce normals, and the
/// plane × cone germ frame and join). Pair-level refusals fire at the
/// sites that EXERCISE an arm (the sweep's crossing lanes, the join's
/// section table), citing the C5 routing: a cone against a conic the
/// inventory excludes, or a partner with no frame, refuses there.
///
/// **`Approx` is absent by DECISION, not by gap.** Its fit is a
/// `Nurbs`, which is on the roster, so admitting it on the fitted
/// kind's authority would run the boolean against the APPROXIMATION
/// while reporting a result about the described surface. It stays off
/// until a rule for composing the fit's precision claim with the
/// boolean's certificates is ratified — and because it is off, the
/// refusal it earns is pair-scoped like every other kind's, naming
/// `SurfaceKind::Approx` in the germ pair.
pub(super) fn boolean_arm_exists<T: Decide>(surface: &geom::Surface<T>) -> bool {
    matches!(
        surface,
        geom::Surface::Plane { .. }
            | geom::Surface::Cylinder { .. }
            | geom::Surface::Sphere { .. }
            | geom::Surface::Torus { .. }
            | geom::Surface::Cone { .. }
            | geom::Surface::Nurbs(_)
    )
}

/// **The face kinds ∖ and ∩ have a seam lane for** — the same roster
/// minus `Nurbs`, which has no crossing layer at all
/// (`BooleanError::CurvedPairUnsupported`'s docs carry the per-class
/// argument). ONE home, beside its sibling above, so the two rosters
/// cannot drift apart in two files: the front door in `ops` reads
/// this rather than spelling a second `matches!`.
///
/// `Approx` is off this roster for the reason it is off the one
/// above, which is strictly stronger here: `Nurbs` has no crossing
/// layer at all, and an approximating surface's chart is a `Nurbs`'s.
pub(super) fn revert_arm_exists<T: Decide>(surface: &geom::Surface<T>) -> bool {
    matches!(
        surface,
        geom::Surface::Plane { .. }
            | geom::Surface::Cylinder { .. }
            | geom::Surface::Sphere { .. }
            | geom::Surface::Torus { .. }
            | geom::Surface::Cone { .. }
    )
}

/// One unsupported-kind face and the face of the other operand whose
/// box it may meet — [`first_unsupported_pair`]'s finding, and the
/// payload of the refusals built from it.
pub(super) struct UnsupportedPair {
    /// The operand carrying the unsupported-kind face.
    pub operand: Operand,
    /// That face.
    pub face: FaceKey,
    /// Its kind — the half of the germ pair with no arm.
    pub kind: geom::SurfaceKind,
    /// The other operand's face whose box overlaps it.
    pub other_face: FaceKey,
    /// That face's kind — the other half of the germ pair.
    pub other_kind: geom::SurfaceKind,
}

/// **The pair-scoped operand scan: the first face whose KIND has no
/// wired arm AND whose box may meet the other operand.**
///
/// A face kind is a property of a face, but an OPERATION is a
/// property of a pair, so a kind can only disqualify an operation
/// through a pair it could enter. Boxes decide that, at box-level
/// conservatism:
///
/// - **Non-overlap is a certificate.** Every box here is a superset
///   of its face's locus (`boxes` module contract), so two boxes that
///   do not overlap bound two loci that do not meet, and a face that
///   meets nothing of the other operand cannot enter any crossing,
///   any section, or any germ pair. Its kind is then irrelevant to
///   the operation and the gate has nothing to say about it.
/// - **The boxes separate along the world's axes only**, so an
///   overlap is followed by the narrow phase
///   ([`super::separating::apart`]), which reads both faces along
///   directions that turn with the operands. A pair it parts is
///   certified apart as a box non-overlap is, and the verdict is the
///   pair's rather than the pose's.
/// - **Overlap is a MAY, not a DOES.** Boxes over-approximate, so
///   this scan still finds pairs that exact geometry would separate.
///   That is conservative in the correct direction — it never admits
///   a pair the crossing pipeline cannot handle — and the refusals
///   built from it say the faces "may meet" rather than claiming a
///   meeting the kernel has not computed.
///
/// The pad is the sweep's own ([`super::boxes::sweep_pad`]), and the
/// sweep's curved arm reads the same narrow phase behind its tree, for
/// an edge and for either face it lies on: a face pair the gate parts,
/// the sweep parts too, for every edge of that face. The converse does
/// not hold. The sweep also skips an edge whose own reach is apart from
/// the face though its faces' reaches are not, so a pair the gate
/// refuses may be one no edge of the sweep ever examines.
///
/// **A COVERED pair is not an offending pair.** `covered` names the
/// cross-operand pairs the caller's declarations speak for. The gate
/// refuses a kind because no arm can say what the two loci do; a
/// declaration is the author saying it, verified at the front door
/// before this gate runs, and consumed by the declared descent instead
/// of by a germ arm. So a pair the declarations cover is one the
/// pipeline HAS a verdict for, and the kind roster has nothing left to
/// object to there.
///
/// The rung is kind-generic and it is not kind-blind: what a
/// declaration may name is bounded by the certified carrier inventory
/// (`validate_declarations`' `inventory_face`, which is
/// [`mod@super::carrier_eq`]'s rung list). A kind with no rung cannot
/// survive into a declaration, so it can never be covered here — one
/// list decides both, and widening this gate is exactly and only
/// widening that ladder.
///
/// Coverage is per PAIR, never per face: an offending face still
/// disqualifies the operation through any OTHER face of the far
/// operand whose box it may meet and whose contact nobody declared.
///
/// **Only cross-operand pairs are examined**, and that is the whole
/// inventory rather than an omission: the boolean pipeline crosses
/// A's edges against B's faces and B's against A's, never a body
/// against itself — a self-intersecting operand is outside the
/// supported envelope on every kind, planar included, and is a
/// precondition rather than something this gate could decide. So a
/// cone and a torus on the SAME body do not gate each other here.
///
/// A face whose surface key does not RESOLVE is neither a pair
/// question nor a kind question: there is no description to bound and
/// no kind to name, so it is reported as the arena corruption it is
/// rather than labelled with a kind it was never shown to have.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a cylinder face whose
/// axis is no direction (`boxes::face_box`).
pub(super) fn first_unsupported_pair<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
    supported: impl Fn(&geom::Surface<T>) -> bool,
    covered: impl Fn(Operand, FaceKey, FaceKey) -> bool,
) -> Result<Option<UnsupportedPair>, BooleanError> {
    let pad = super::boxes::sweep_pad(band);
    let axes = super::separating::OperandAxes::new(super::boxes::axis_key);
    for (operand, body, other) in [(Operand::A, a, b), (Operand::B, b, a)] {
        // Arena order both ways, and no box is built for an operand
        // that carries no unsupported kind at all — the common case
        // pays nothing for this gate.
        let mut offenders: Vec<(FaceKey, geom::SurfaceKind)> = Vec::new();
        for (key, f) in body.faces() {
            let s = body.face_surface_linked(key, f);
            if !supported(s) {
                offenders.push((key, s.kind()));
            }
        }
        if offenders.is_empty() {
            continue;
        }
        // The other side's boxes are built ONCE, and only now that an
        // offender exists: the scan is `offenders × other faces`, so
        // re-boxing per offender would re-walk a whole body per cone.
        let others: Vec<(FaceKey, geom::SurfaceKind, bvh::Aabb)> = other
            .faces()
            .map(|(key, f)| {
                let kind = other.face_surface_linked(key, f).kind();
                Ok((key, kind, super::boxes::face_box(other, key, pad, band)?))
            })
            .collect::<Result<_, BooleanError>>()?;
        for (face, kind) in offenders {
            let boxed = super::boxes::face_box(body, face, pad, band)?;
            for &(other_face, other_kind, ref other_box) in &others {
                if boxed.overlaps(other_box)
                    && !covered(operand, face, other_face)
                    && !super::separating::apart(
                        (body, Item::Face(face)),
                        (other, Item::Face(other_face)),
                        axes.of(a, b, band),
                        pad,
                        band,
                    )
                {
                    return Ok(Some(UnsupportedPair {
                        operand,
                        face,
                        kind,
                        other_face,
                        other_kind,
                    }));
                }
            }
        }
    }
    Ok(None)
}

/// **The operand gate, pair-scoped** (M5 PR 9, C12.1 — the F5
/// planar-only gate retires PER C5 TABLE ARM, never wholesale).
///
/// First, each operand passes [`gate_operand`]: a closed solid at rest
/// by the validator's own verdict, with supported edge carriers. Then
/// two rules, with different scopes on purpose:
///
/// - **Faces**: a kind off the face-kind `roster` — [`boolean_arm_exists`],
///   or [`revert_arm_exists`] at the ∖/∩ front door — disqualifies the
///   operation only through a PAIR it could enter
///   ([`first_unsupported_pair`]) and that the caller's declarations
///   do not cover. A torus wall whose box clears the other operand
///   does not gate anything, and neither does one whose contact with
///   the face it may meet the author has DECLARED.
/// - **Edges**: body-scoped ([`gate_operand`]).
///
/// # Errors
///
/// [`gate_operand`]'s per operand; [`BooleanError::CurvedPairUnsupported`]
/// for a germ pair with no arm; [`first_unsupported_pair`]'s.
pub(super) fn gate_operand_pairs<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    declared: &super::GluedPairs<T>,
    band: Band,
    roster: impl Fn(&geom::Surface<T>) -> bool,
) -> Result<(), BooleanError> {
    for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
        gate_operand(body, operand)?;
    }
    // A pair is covered by the certificate its consumer reads: the
    // declared descent through the carrier ladder, which runs on a pair
    // the door verified ONE carrier. A `Tangent` claim covers nothing
    // here: its arms are the witness lane's, whose kinds are all on the
    // roster.
    if let Some(p) = first_unsupported_pair(a, b, band, roster, |operand, f, other| {
        declared.verified_one_carrier(operand, f, operand.other(), other)
    })? {
        return Err(BooleanError::CurvedPairUnsupported {
            op: None,
            site: super::PairRefusalSite::OperandGate,
            operand: p.operand,
            face: p.face,
            kind: p.kind,
            other_face: p.other_face,
            other_kind: p.other_kind,
        });
    }
    Ok(())
}

/// **The operand gate's BODY-scoped half**: a closed solid at rest, by
/// the validator's own verdict ([`crate::validate_closed`]). Tier-2
/// scaffolding — a strut, an empty loop, a null edge, a split shell —
/// refuses as [`BooleanError::ScaffoldingOperand`], carrying the
/// findings. A tier-1 finding panics naming the first: every public
/// door keeps a body tier-1-valid, so only a kernel bug hands one in.
///
/// Edge carriers are not gated: a lane that reads one refuses a kind it
/// has no row for at its own site, naming the edge and the face
/// ([`BooleanError::EdgeCarrierUnsupported`]).
///
/// Orientation is not read here: a `Validated` operand's tier-3 verdict
/// includes check 7 per solid, and [`gate_unverdicted_operand`] reads it
/// for an operand that carries no verdict.
pub(super) fn gate_operand<T: Decide>(
    body: &Body<T>,
    operand: Operand,
) -> Result<(), BooleanError> {
    let scaffolding =
        crate::validate::operand_scaffolding(body, format_args!("operand {operand:?}"));
    if !scaffolding.is_empty() {
        return Err(BooleanError::ScaffoldingOperand {
            operand,
            errors: scaffolding,
        });
    }
    Ok(())
}

/// **The operand gate where no at-rest gate ran**: what the finished-body
/// type promises, read on an operand that carries no verdict (a dual's,
/// [`crate::AtRestBody::gate_unverdicted`] — the split's door reads the
/// same), refused as [`BooleanError::ScaffoldingOperand`],
/// [`BooleanError::InsideOutOperand`] or [`BooleanError::UnjoinedOperand`].
/// Its orientation read is check 7's, the reader of a shell's role, so an
/// inside-out operand refuses here as it does at the at-rest gate. It
/// runs before the pipeline reads a several-solid operand as one solid
/// (`ops::one_solid`), since the orientation read's subjects are the
/// solid and, within it, the shell.
pub(super) fn gate_unverdicted_operand<T: Decide + crate::props::AtRestPolicy>(
    body: &crate::AtRestBody<T>,
    operand: Operand,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    body.gate_unverdicted(format_args!("operand {operand:?}"), band, tol)
        .map_err(|unfinished| BooleanError::unfinished(operand, unfinished))
}

/// An operand edge's certified carrier. Tier 2 refuses a null edge and
/// tier 1 an unresolvable curve key at the operand gate, and the
/// sweep's only surgery (`split_edge`) mints certified pieces, so a
/// miss past the gate is a kernel invariant, not the operand's fault.
pub(super) fn certified<T: geom_core::Real>(
    geom: Option<&CurveGeom<T>>,
) -> Result<&geom_brep::EdgeCurve<T>, BooleanError> {
    geom.and_then(CurveGeom::certified)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "an operand edge past the tier-1/2 gate has no certified carrier",
        })
}

/// The face's plane description (post-gate: always a `Plane`), with
/// the **face's outward normal** — not the chart's.
///
/// [`PlaneDesc::normal`] is contractually the unit OUTWARD normal —
/// the surface's chart normal with [`crate::entity::Face::sense`]
/// folded in: the chart is the only place orientation is encoded, so
/// on a `sense: false` face the stored normal points INTO the
/// material, and every consumer reading a material direction off it
/// would answer backwards. The flip itself lives in
/// [`crate::face_normal`], which this function is defined in terms of
/// — one door for the planar consumers (`plane_of`, this sweep, the
/// pierce lane, and the SHARED [`crate::sector_face`]
/// walk, which is why the door sits at the crate root rather than
/// here), one flip, so those consumers stay orientation-blind.
///
/// Outside this crate the same fold is spelled through
/// [`geom_brep::OutwardNormal::from_chart`], the type's only
/// constructor, which takes the bit; there is no scalar sign on a face
/// for a reader anywhere to multiply by.
///
/// Consumers that only compare the plane RESIDUAL `(p − o)·n̂` against
/// Zero, or that hand the normal to a ray-parity test, are unaffected
/// either way (a residual's sign flip decides Zero the same, and
/// crossing parity is blind to frame handedness). The consumers that
/// read a MATERIAL side off the sign — `side_code`, the containment
/// ray's `d·n̂` — are exactly the ones this fixes.
///
/// `None` for a face that is not a plane, and for a `face` that does
/// not resolve: the key is the caller's ([`super::carrier_pair::flush_pair_relation`]
/// passes a public door's).
///
/// # Panics
///
/// Where `face` resolves and its surface does not (D2 row 4): a torn
/// surface is not a non-planar one. On the reduction's working copies
/// the link holds by [`crate::live::OPERATORS_KEEP_LINKS`].
pub(super) fn face_plane<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<PlaneDesc<T>> {
    let f = body.get_face(face)?;
    match body.face_surface_linked(face, f) {
        geom::Surface::Plane { origin, normal, .. } => Some(PlaneDesc {
            origin: *origin,
            normal: crate::face_normal::plane_outward_normal(f, *normal).vec(),
        }),
        _ => None,
    }
}

/// A vertex of the swept solid `x`, whose slit tips at rest are `tips`,
/// against the planar `face` of the other, `y`, at its offset from the
/// face's plane through `origin` along the stored `normal` the door
/// reads its side by: the split's rule
/// ([`crate::vertex_section::decide_vertex_on_plane`]). The face's
/// outward normal is read into the section table as a unit direction
/// over the face's reach ([`super::separating::planar_axis`]), and a
/// far vertex of `x` takes its side from its own offset, under this
/// door's name.
fn vertex_face_side<T: Decide>(
    (x, tips): (&Body<T>, &SlitTips),
    (vertex, at): (VertexKey, Point3<T>),
    (y, face, plane): (&Body<T>, FaceKey, &PlaneDesc<T>),
    (origin, normal): (Point3<T>, Vec3<T>),
    band: Band,
) -> Result<geom_core::Decided, VertexRefusal> {
    let section_plane = || {
        let outward = crate::face_normal::face_outward_normal(y, face).ok_or(
            VertexRefusal::Plane("the face's plane has no outward normal (not a plane)"),
        )?;
        let normal = super::separating::planar_axis(
            y,
            face,
            || crate::census::face_reach(y, face, band),
            band,
        )
        .ok_or(VertexRefusal::Plane(
            "the face's outward normal does not read as a unit direction over its reach",
        ))?;
        Ok(SectionPlane {
            origin: plane.origin,
            normal,
            reference: geom_brep::ReferenceNormal::of_face_outward(outward),
        })
    };
    let far = |far: VertexKey| {
        let q = x.resolve_vertex_point(far, crate::live::Proven);
        let decided = geom_core::k_stats::decide_reported(
            "bool_vertex_face_side",
            Margin::of(crate::sector_shape::plane_offset(
                plane.origin,
                plane.normal,
                q,
            )),
            band,
        )
        .map_err(VertexRefusal::Undecided)?;
        Ok(match decided.sign {
            Sign::Negative => SplitSide::Below,
            Sign::Positive => SplitSide::Above,
            Sign::Zero => SplitSide::On,
        })
    };
    let offset = Offset {
        value: crate::sector_shape::plane_offset_rounded(origin, normal, at),
        normal,
    };
    crate::vertex_section::decide_vertex_on_plane(
        "bool_vertex_face_side",
        (x, tips),
        (vertex, at),
        section_plane,
        (offset, far),
        band,
    )
}

/// The Boolean's refusal of a vertex the rule could not place on a
/// planar face, `read` being the declaration read of the pair.
fn vertex_refusal(refusal: VertexRefusal, read: DeclarationRead) -> BooleanError {
    match refusal {
        VertexRefusal::Undecided(diag) => {
            BooleanError::coincidence(Coincide::VertexOnFace, read, diag)
        }
        VertexRefusal::Walk(crate::splitting::SplitReduceError::SliverSector {
            reading: crate::splitting::SectorReading::Escalated(diag),
            ..
        }) => BooleanError::coincidence(Coincide::SectorSide, DeclarationRead::Moot, diag),
        VertexRefusal::Walk(_) => BooleanError::ClassificationInvariant {
            what: "the neighbourhood walk refused a vertex of an operand at rest (a null edge \
                   or a degenerate chord where the operand gate admits none)",
        },
        VertexRefusal::Plane(what) => BooleanError::ClassificationInvariant { what },
    }
}

/// F7: the maximal-faces precondition through the coincidence ladder —
/// same surface key, or Same±-oriented planes its margins decide one
/// ([`super::oriented_plane_eq`]), across any edge ⇒
/// [`BooleanError::NonMaximalFaces`]; a near-coplanar dihedral surfaces
/// as the predicate's own typed escalation instead. The ladder reads a
/// ball about the shared edge's start that holds the edge, so the offset
/// is the planes' at the edge and the tilt is levered at its reach. It runs
/// on an operand [`gate_operand`] passed, so every edge it reads holds
/// a certified carrier.
pub(super) fn gate_maximal_faces<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    band: Band,
) -> Result<(), BooleanError> {
    for (edge_key, edge) in body.edges() {
        let sides = crate::readback::edge_sides_of(body, edge_key, edge);
        let (f1, f2) = sides.faces();
        if f1 == f2 {
            continue; // seam/strut inside one face: not a coplanar PAIR
        }
        let (k1, k2) = sides.surfaces();
        if k1 == k2 {
            // Same-key CURVED adjacency is the CANONICAL maximal form
            // (M5 PR 9, C12.5): a periodic wall cannot be one face
            // without its parameterization cut, so two half-walls
            // sharing one cylinder key across a meridian strut are
            // exactly what a maximal-faced curved operand looks like
            // (the cosurface merge itself KEEPS such a cut). Only the
            // PLANAR same-key pair is the F7 defect.
            let planar = matches!(
                body.face_surface_linked(f1, crate::live::proven(&body.faces, f1, EntityId::Face)),
                geom::Surface::Plane { .. }
            );
            if planar {
                return Err(BooleanError::NonMaximalFaces {
                    operand,
                    edge: edge_key,
                });
            }
            continue;
        }
        let (Some(p1), Some(p2)) = (face_plane(body, f1), face_plane(body, f2)) else {
            continue;
        };
        let at = body.resolve_vertex_point(
            crate::live::proven(&body.half_edges, edge.he_plus, EntityId::HalfEdge).start,
            crate::live::Proven,
        );
        // The ball about the edge's start that holds all of it: a line's
        // chord; a circle's farthest point from its start, its far end or
        // the point opposite, so a closed edge, whose ends are one vertex,
        // reads its diameter; a fitted carrier's control polygon, whose
        // hull holds it (its weights are positive); any other carrier's
        // box.
        let curve = body
            .edge_curve_linked(edge_key, edge)
            .certified()
            .unwrap_or_else(|| {
                unreachable!(
                    "{operand:?}'s edge {edge_key:?}, read out of its arena, has no \
                     certified carrier: the operand gate refuses a null edge as scaffolding"
                )
            });
        let end =
            body.resolve_vertex_point(body.proven_half_edge_end(edge.he_plus), crate::live::Proven);
        let arm = match curve.carrier() {
            geom::Curve3::Line { .. } => (end - at).norm(),
            geom::Curve3::Circle { radius, .. } => {
                let (t0, t1) = curve.params();
                let half =
                    ((t1 - t0) * T::from_f64(0.5)).min(T::from_f64(core::f64::consts::FRAC_PI_2));
                radius.abs() * T::from_f64(2.0) * half.sin()
            }
            geom::Curve3::Nurbs(n) => geom_brep::control_net_reach(n.control(), at),
            _ => crate::census::edge_reach_in(body, edge_key, &super::boxes::BoxFrame::World)
                .map(|(lo, hi)| geom_brep::ExtentBall::of_box(lo, hi).lever_from(at))
                .unwrap_or_else(|| {
                    unreachable!("{operand:?}'s edge {edge_key:?}: a conic carrier has a box")
                }),
        };
        let extent = super::carrier_eq::ConsumedExtent::new(geom_brep::ExtentBall::new(at, arm));
        // One plane, decided by its margin: a pair the producing op
        // should have merged (the merge glues what this decides).
        match super::oriented_plane_eq(&p1, &p2, &extent, band) {
            Ok(super::PlaneRelation::Distinct) => {}
            Ok(_) => {
                return Err(BooleanError::NonMaximalFaces {
                    operand,
                    edge: edge_key,
                });
            }
            Err(super::PlaneEqError::Undecided { coincidence, .. }) => {
                let pair = [(operand, f1), (operand, f2)];
                return Err(BooleanError::CoplanarNeighbours {
                    operand,
                    faces: [f1, f2],
                    offset: super::readable_coincidence(coincidence, pair)?,
                });
            }
            Err(super::PlaneEqError::Escalated { rung, diag }) => {
                return Err(BooleanError::plane_identity(
                    rung,
                    super::PlaneDoor::Neighbours,
                    diag,
                ));
            }
        }
    }
    Ok(())
}

/// One sweep direction: every edge (fragment) of `x` against the faces
/// of `y` its box can touch (module docs: the tree prunes, predicates
/// decide). `x_is` names which operand `x` is (contact orientation).
///
/// `T: Decide + Bounds` is the ratified compound-bound seam
/// (2026-07-29 — geom-core `real.rs`, Bounds scope rule): the C10
/// tree is the subdivision driver, and box construction reads
/// coordinate brackets — never a value comparison in classification.
/// The realized candidate generator's per-direction face tree, built
/// ONCE over the face snapshot (arena order = input order). Mid-sweep
/// splits of `y`'s edges only mint vertices ON existing boundary
/// (within the pad), so the snapshot boxes stay conservative for the
/// whole direction.
fn face_tree<T: Decide + Bounds>(
    y: &Body<T>,
    faces: &[FaceKey],
    knobs: &SweepKnobs,
    pad: f64,
    band: Band,
) -> Result<bvh::Bvh, BooleanError> {
    let mut face_boxes = Vec::with_capacity(faces.len());
    for &f in faces {
        let planted = knobs.plant == Some(f);
        face_boxes.push(if planted {
            // Pin (iii)'s planted degradation: the inverted box
            // overlaps nothing — this face's events get lost and the
            // suite's superset pin must catch it.
            bvh::Aabb {
                min_x: f64::INFINITY,
                min_y: f64::INFINITY,
                min_z: f64::INFINITY,
                max_x: f64::NEG_INFINITY,
                max_y: f64::NEG_INFINITY,
                max_z: f64::NEG_INFINITY,
            }
        } else {
            boxes::face_box(y, f, pad, band)?
        });
    }
    Ok(bvh::Bvh::build(&face_boxes))
}

/// What the sweep's door read of the declaration ahead of `question`,
/// asked of `edge` of `x` against `face` of the other operand: each of
/// the edge's parent faces paired with `face`, looked up in turn
/// ([`super::GluedPairs::read`]). No question the sweep asks of an
/// edge is one a declaration settles (the declared cover spends the
/// declaration, and an undeclared pair's refusal is the same pose's),
/// so the door admits no class that would.
fn edge_face_read<T: geom_core::Real>(
    x: &Body<T>,
    x_is: Operand,
    edge: &crate::entity::Edge,
    face: FaceKey,
    declared: &super::GluedPairs<T>,
    question: Coincide,
) -> DeclarationRead {
    let pairs: Vec<_> = [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .flatten()
    .map(|f| (x_is, f, x_is.other(), face))
    .collect();
    declared.read(&pairs, question, &[])
}

#[allow(clippy::too_many_arguments)] // one parameter per named duty (bodies, orientation, declarations, sinks, band, strategy, plant, trace)
pub(super) fn sweep_direction<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (x, tips): (&mut Body<T>, &SlitTips),
    y: &mut Body<T>,
    x_is: Operand,
    declared: &super::GluedPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    strategy: SweepStrategy,
    knobs: &SweepKnobs,
    mut trace: Option<&mut SweepTrace>,
    deferred: &mut Vec<DeferredTouch>,
    axes: &super::separating::OperandAxes<T>,
    tol: Tol,
) -> Result<(), BooleanError> {
    let faces: Vec<FaceKey> = y.faces().map(|(k, _)| k).collect();
    let pad = knobs.pad_override.unwrap_or_else(|| boxes::sweep_pad(band));
    // With `sweep-testing`, the tree is optional so the idealized
    // reference can decline it. Without the feature there is no
    // `Idealized` variant to decline it with, so the tree is
    // unconditional and the brute-force arm below does not exist.
    #[cfg(feature = "sweep-testing")]
    let tree: Option<bvh::Bvh> = match strategy {
        SweepStrategy::Realized => Some(face_tree(y, &faces, knobs, pad, band)?),
        SweepStrategy::Idealized => None,
    };
    #[cfg(not(feature = "sweep-testing"))]
    let tree: bvh::Bvh = {
        let SweepStrategy::Realized = strategy;
        face_tree(y, &faces, knobs, pad, band)?
    };
    let mut worklist: std::collections::VecDeque<Pending> = x
        .edges()
        .map(|(edge, _)| Pending {
            edge,
            start: 0,
            lying_in: Vec::new(),
        })
        .collect();

    while let Some(Pending {
        edge: edge_key,
        start,
        mut lying_in,
    }) = worklist.pop_front()
    {
        // The fragment's candidate face indices, ascending — the
        // realized set is a subsequence of the idealized scan, so the
        // examination order (and with it every split/requeue) is
        // preserved pair-for-pair.
        #[cfg(feature = "sweep-testing")]
        let candidates: Vec<usize> = match &tree {
            Some(t) => t.overlapping(&boxes::edge_box(x, edge_key, pad)),
            // The idealized reference's candidate set: every face, in
            // arena order. Reachable only through the gated
            // `SweepStrategy::Idealized`, and compiled out with it.
            None => (0..faces.len()).collect(),
        };
        #[cfg(not(feature = "sweep-testing"))]
        let candidates: Vec<usize> = tree.overlapping(&boxes::edge_box(x, edge_key, pad));
        let mut ci = 0;
        'faces: while let Some(&j) = candidates.get(ci) {
            ci += 1;
            if j < start {
                continue;
            }
            let Some(&face) = faces.get(j) else {
                // Unreachable: candidate indices come from the face
                // snapshot itself.
                break;
            };
            if let Some(tr) = trace.as_deref_mut() {
                tr.examined.push((edge_key, face));
            }
            let edge =
                x.get_edge(edge_key)
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "worklist edge vanished mid-sweep",
                    })?;
            let ((u, pu), (v, pv)) = edge_ends(x, &edge)?;
            // Per-kind face dispatch (M5 PR 9, C12.1): planar faces run
            // the M3 lane below (bit-identically for line edges, plus
            // the conic ROOT lane); curved faces get the clearance /
            // typed-frontier arm.
            let Some(plane) = face_plane(y, face) else {
                // The tree's overlap is a world-axis one: a curved face
                // the edge, or either face the edge lies on, is apart
                // from along a direction that turns with the operands
                // meets the edge nowhere, whichever arm its kind has.
                // The parents' reading is the operand gate's, so a pair
                // the gate cleared is cleared here too.
                let apart_from_face = |item| {
                    super::separating::apart(
                        (x, item),
                        (y, Item::Face(face)),
                        axes.of(x, y, band),
                        pad,
                        band,
                    )
                };
                if apart_from_face(Item::Edge(edge_key))
                    || [edge.he_plus, edge.he_minus]
                        .into_iter()
                        .any(|he| apart_from_face(Item::Face(x.face_of_linked(he))))
                {
                    continue;
                }
                let event = curved_face_arm(
                    x, y, x_is, edge_key, &edge, u, v, face, pu, pv, declared, contacts, band, tol,
                )?;
                if !matches!(
                    event,
                    CurvedEvent::None
                        | CurvedEvent::Deferred
                        | CurvedEvent::Lies {
                            recorded: false,
                            ..
                        }
                        | CurvedEvent::Along { recorded: false }
                ) && let Some(tr) = trace.as_deref_mut()
                {
                    tr.accepted.push((edge_key, face));
                }
                // A wall crossing is split and recorded HERE, with the
                // same triple the conic × plane root above uses: the
                // event splits `x`'s edge, the contact is minted
                // against the piercing side, and the remainder fragment
                // is re-queued so the second root of the same span is
                // found on the next pass.
                let (t, p, at, on, branch, chart) = match event {
                    CurvedEvent::Pierce {
                        t,
                        p,
                        at,
                        on,
                        branch,
                        chart,
                    } => (t, p, at, on, branch, chart),
                    CurvedEvent::Deferred => {
                        deferred.push(DeferredTouch {
                            x_is,
                            edge: edge_key,
                            end: v,
                            face,
                            refusal: BooleanError::CurvedPierceUnsupported {
                                operand: x_is,
                                face,
                                edge: edge_key,
                                band,
                            },
                        });
                        continue;
                    }
                    CurvedEvent::None | CurvedEvent::Recorded => continue,
                    // An edge the arm read lying on the face, ends only:
                    // a later split's vertex is read against it too. A
                    // pierce requeues the pieces at this face, which
                    // reads them itself.
                    CurvedEvent::Lies { on, .. } => {
                        lying_in.push(LyingIn::Curved(face, on));
                        continue;
                    }
                    CurvedEvent::Along { .. } => {
                        lying_in.push(LyingIn::Along(face));
                        continue;
                    }
                };
                match at {
                    FaceContainment::Out => continue,
                    FaceContainment::In => {
                        let w =
                            split_at(x, x_is, edge_key, t, super::Cell::Face(face), tol, contacts)?;
                        push_vf(contacts, x_is, w, face, on);
                        if let Some(branch) = branch {
                            contacts.branches.insert((x_is, w), branch);
                        }
                        if let Some(uv) = chart {
                            contacts.chart_point(x_is, w, face, uv);
                        }
                        read_then_requeue(
                            &mut worklist,
                            (x, tips, x_is, edge_key, &edge),
                            y,
                            w,
                            j,
                            &lying_in,
                            declared,
                            contacts,
                            band,
                            tol,
                        )?;
                    }
                    FaceContainment::OnEdge(ey) => {
                        let w =
                            split_at(x, x_is, edge_key, t, super::Cell::Edge(ey), tol, contacts)?;
                        let wy = split_other_at_point(
                            y,
                            x_is.other(),
                            ey,
                            p,
                            super::Cell::Edge(edge_key),
                            band,
                            tol,
                            contacts,
                        )?;
                        push_vv(contacts, x_is, w, wy, on);
                        read_then_requeue(
                            &mut worklist,
                            (x, tips, x_is, edge_key, &edge),
                            y,
                            w,
                            j,
                            &lying_in,
                            declared,
                            contacts,
                            band,
                            tol,
                        )?;
                    }
                    FaceContainment::OnVertex(vy) => {
                        let w = split_logged(
                            x,
                            x_is,
                            edge_key,
                            t,
                            Some(y.resolve_vertex_point(vy, crate::live::Proven)),
                            super::Cell::Vertex(vy),
                            tol,
                            contacts,
                        )?;
                        push_vv(contacts, x_is, w, vy, on);
                        read_then_requeue(
                            &mut worklist,
                            (x, tips, x_is, edge_key, &edge),
                            y,
                            w,
                            j,
                            &lying_in,
                            declared,
                            contacts,
                            band,
                            tol,
                        )?;
                    }
                }
                break 'faces;
            };
            // Conic carriers against a plane (M5 PR 9): crossing
            // detection is ROOT-BASED and endpoint-verdict-free — the
            // splitting lane's C12.1 machinery reused verbatim (a
            // belly arc crosses between same-side endpoints, which the
            // endpoint-sign match below cannot see). Every interior
            // root is examined, and the FIRST one the face does not
            // place `Out` splits exactly like a proper line crossing;
            // both fragments re-examine the SAME face, so any other
            // root is found again on them.
            //
            // **The one-sided cover** (C4, as on the curved arm): when an
            // edge's parent carrier is certified to lie in one closed side
            // of this face's plane, a conic on it never crosses the plane,
            // and its residual along the carrier circle is a sinusoid of
            // one sign, zero at most once. An endpoint ON the plane is then
            // the edge's one incidence, a touch, and it takes the endpoint
            // rows; the root lane, which cannot separate the double root
            // there from a crossing, is not asked. With no endpoint on the
            // plane the roots still decide, so a touch in the middle of
            // the edge keeps their typed refusal.
            {
                let curve = certified(x.get_curve_geom(edge.curve))?.clone();
                let (t0, t1) = curve.params();
                let lane = crate::splitting::plane_crossing_lane(
                    curve.carrier(),
                    t0,
                    t1,
                    plane.origin,
                    plane.normal,
                    band,
                );
                let meet = match lane {
                    PlaneCrossingLane::Line => None,
                    // A spiric or a spline: its endpoints' sides neither
                    // find its crossings nor place them. A spline span is
                    // the curve × face table's ring-composite arm, whose
                    // crossings take the conic's root rows below.
                    PlaneCrossingLane::Unlaned => Some(ConicPlaneMeet::Roots(Ok(
                        spline_plane_crossings((x, x_is, edge_key), &curve, (y, face), band)?,
                    ))),
                    PlaneCrossingLane::Conic(meet) => Some(meet),
                };
                // Only the line lane's coplanar arm records a plane, and a
                // conic's pieces are conics, so no conic carries one
                // (`LyingIn`).
                debug_assert!(
                    meet.is_none() || !lying_in.iter().any(|l| matches!(l, LyingIn::Plane(_))),
                    "a conic edge carries a plane it lies in"
                );
                let covers = edge_covers(x, x_is, &edge, face, declared);
                let touch_at_end = if meet.is_none() || covers.is_empty() {
                    None
                } else {
                    let read =
                        edge_face_read(x, x_is, &edge, face, declared, Coincide::VertexOnFace);
                    // One decision per end, in the carrier's own residual:
                    // the sign the cover's side is stated in. It differs
                    // from the offset along the OUTWARD normal only by the
                    // face's sense bit, so whether an end is ON the plane
                    // reads the same either way.
                    let Some(&geom::Surface::Plane {
                        origin: chart_origin,
                        normal: chart_normal,
                        ..
                    }) = y.get_face(face).and_then(|f| y.get_surface(f.surface))
                    else {
                        return Err(BooleanError::ClassificationInvariant {
                            what: "planar lane: the face's plane resolved above",
                        });
                    };
                    let side = |w: VertexKey, p: Point3<T>| {
                        vertex_face_side(
                            (x, tips),
                            (w, p),
                            (y, face, &plane),
                            (chart_origin, chart_normal),
                            band,
                        )
                        .map_err(|refusal| vertex_refusal(refusal, read))
                    };
                    let (d1, d2) = (side(u, pu)?, side(v, pv)?);
                    let (s1, s2) = (d1.sign, d2.sign);
                    // The off end must lie where a parent's cover puts it.
                    let off_end_admitted = || {
                        let off = if s1 == Sign::Zero { s2 } else { s1 };
                        covers.iter().any(|c| c.admits(off))
                    };
                    ((s1 == Sign::Zero) != (s2 == Sign::Zero) && off_end_admitted()).then_some(
                        if s1 == Sign::Zero {
                            (true, d1.margin)
                        } else {
                            (false, d2.margin)
                        },
                    )
                };
                match meet {
                    // A line: the endpoint lane below owns it.
                    None => {}
                    Some(ConicPlaneMeet::Miss) => continue,
                    // The conic's plane is parallel to the face's: off
                    // it, a miss; in it, the line lane's `(Zero, Zero)`
                    // posture — both endpoints through
                    // `vertex_on_face`, the interior left to the
                    // neighbour faces.
                    Some(ConicPlaneMeet::Parallel { offset }) => {
                        let on = match geom_core::k_stats::decide_reported(
                            "bool_conic_face_plane_offset",
                            Margin::of(offset),
                            band,
                        ) {
                            Ok(geom_core::Decided {
                                sign: Sign::Positive | Sign::Negative,
                                ..
                            }) => continue,
                            Ok(geom_core::Decided {
                                sign: Sign::Zero,
                                margin,
                            }) => margin,
                            Err(diag) => {
                                return Err(BooleanError::coincidence(
                                    Coincide::EdgeOnPlane,
                                    edge_face_read(
                                        x,
                                        x_is,
                                        &edge,
                                        face,
                                        declared,
                                        Coincide::EdgeOnPlane,
                                    ),
                                    diag,
                                ));
                            }
                        };
                        let mut hit =
                            vertex_on_face(x_is, y, u, pu, face, &plane, on, contacts, band, tol)?;
                        if v != u {
                            hit |= vertex_on_face(
                                x_is, y, v, pv, face, &plane, on, contacts, band, tol,
                            )?;
                        }
                        if hit && let Some(tr) = trace.as_deref_mut() {
                            tr.accepted.push((edge_key, face));
                        }
                        continue;
                    }
                    Some(ConicPlaneMeet::Roots(_)) if touch_at_end.is_some() => {
                        let Some(first_end) = touch_at_end else {
                            return Err(BooleanError::ClassificationInvariant {
                                what: "conic lane: the one-sided touch lost its sides",
                            });
                        };
                        let (first_end, on) = first_end;
                        let (w, pw) = if first_end { (u, pu) } else { (v, pv) };
                        if vertex_on_face(x_is, y, w, pw, face, &plane, on, contacts, band, tol)?
                            && let Some(tr) = trace.as_deref_mut()
                        {
                            tr.accepted.push((edge_key, face));
                        }
                        continue;
                    }
                    Some(ConicPlaneMeet::Roots(Err(fault))) => {
                        return Err(BooleanError::Escalated {
                            decision: BooleanDecision::of_conic_root(
                                fault,
                                edge_face_read(
                                    x,
                                    x_is,
                                    &edge,
                                    face,
                                    declared,
                                    Coincide::EdgeOnPlane,
                                ),
                            ),
                            diag: fault.diag(),
                        });
                    }
                    Some(ConicPlaneMeet::Roots(Ok(roots))) => {
                        for &t in &roots {
                            let p = curve.carrier().eval(t);
                            let (containment, on) = contfp_decided(y, face, plane.normal, p, band)
                                .map_err(|e| esc(e, x_is.other(), face))?;
                            if !matches!(containment, FaceContainment::Out)
                                && let Some(tr) = trace.as_deref_mut()
                            {
                                tr.accepted.push((edge_key, face));
                            }
                            match containment {
                                FaceContainment::Out => {}
                                FaceContainment::In => {
                                    let w = split_at(
                                        x,
                                        x_is,
                                        edge_key,
                                        t,
                                        super::Cell::Face(face),
                                        tol,
                                        contacts,
                                    )?;
                                    push_vf(contacts, x_is, w, face, on);
                                    read_then_requeue(
                                        &mut worklist,
                                        (x, tips, x_is, edge_key, &edge),
                                        y,
                                        w,
                                        j,
                                        &lying_in,
                                        declared,
                                        contacts,
                                        band,
                                        tol,
                                    )?;
                                    break 'faces;
                                }
                                FaceContainment::OnEdge(ey) => {
                                    let w = split_at(
                                        x,
                                        x_is,
                                        edge_key,
                                        t,
                                        super::Cell::Edge(ey),
                                        tol,
                                        contacts,
                                    )?;
                                    let wy = split_other_at_point(
                                        y,
                                        x_is.other(),
                                        ey,
                                        p,
                                        super::Cell::Edge(edge_key),
                                        band,
                                        tol,
                                        contacts,
                                    )?;
                                    push_vv(contacts, x_is, w, wy, on);
                                    read_then_requeue(
                                        &mut worklist,
                                        (x, tips, x_is, edge_key, &edge),
                                        y,
                                        w,
                                        j,
                                        &lying_in,
                                        declared,
                                        contacts,
                                        band,
                                        tol,
                                    )?;
                                    break 'faces;
                                }
                                FaceContainment::OnVertex(vy) => {
                                    let w = split_logged(
                                        x,
                                        x_is,
                                        edge_key,
                                        t,
                                        Some(y.resolve_vertex_point(vy, crate::live::Proven)),
                                        super::Cell::Vertex(vy),
                                        tol,
                                        contacts,
                                    )?;
                                    push_vv(contacts, x_is, w, vy, on);
                                    read_then_requeue(
                                        &mut worklist,
                                        (x, tips, x_is, edge_key, &edge),
                                        y,
                                        w,
                                        j,
                                        &lying_in,
                                        declared,
                                        contacts,
                                        band,
                                        tol,
                                    )?;
                                    break 'faces;
                                }
                            }
                        }
                        // No interior root lands in the face:
                        // endpoint processing only.
                        let read =
                            edge_face_read(x, x_is, &edge, face, declared, Coincide::VertexOnFace);
                        let mut hit = false;
                        for end in [(u, pu), (v, pv)] {
                            hit |= read_on_plane(
                                (x, x_is, tips),
                                y,
                                end,
                                (face, &plane),
                                read,
                                contacts,
                                band,
                                tol,
                            )?;
                        }
                        if hit && let Some(tr) = trace.as_deref_mut() {
                            tr.accepted.push((edge_key, face));
                        }
                        continue;
                    }
                }
            }
            let side = |w: VertexKey, p: Point3<T>| {
                vertex_face_side(
                    (x, tips),
                    (w, p),
                    (y, face, &plane),
                    (plane.origin, plane.normal),
                    band,
                )
            };
            let read = edge_face_read(x, x_is, &edge, face, declared, Coincide::VertexOnFace);
            let on_face = |refusal| vertex_refusal(refusal, read);
            let d1 = side(u, pu).map_err(on_face)?;
            let d2 = side(v, pv).map_err(on_face)?;
            match (d1.sign, d2.sign) {
                (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
                    // Proper plane crossing: locate p on the carrier and
                    // classify it against the face.
                    let curve = certified(x.get_curve_geom(edge.curve))?.clone();
                    let (t0, t1) = curve.params();
                    let d1 = (pu - plane.origin).dot(plane.normal);
                    let d2 = (pv - plane.origin).dot(plane.normal);
                    let t = t0 + (t1 - t0) * (d1 / (d1 - d2));
                    let p = curve.carrier().eval(t);
                    let (containment, on) = contfp_decided(y, face, plane.normal, p, band)
                        .map_err(|e| esc(e, x_is.other(), face))?;
                    if !matches!(containment, FaceContainment::Out)
                        && let Some(tr) = trace.as_deref_mut()
                    {
                        tr.accepted.push((edge_key, face));
                    }
                    match containment {
                        FaceContainment::Out => {}
                        FaceContainment::In => {
                            let w = split_at(
                                x,
                                x_is,
                                edge_key,
                                t,
                                super::Cell::Face(face),
                                tol,
                                contacts,
                            )?;
                            push_vf(contacts, x_is, w, face, on);
                            read_then_requeue(
                                &mut worklist,
                                (x, tips, x_is, edge_key, &edge),
                                y,
                                w,
                                j + 1,
                                &lying_in,
                                declared,
                                contacts,
                                band,
                                tol,
                            )?;
                            break 'faces;
                        }
                        FaceContainment::OnEdge(ey) => {
                            let w = split_at(
                                x,
                                x_is,
                                edge_key,
                                t,
                                super::Cell::Edge(ey),
                                tol,
                                contacts,
                            )?;
                            let wy = split_other_at_point(
                                y,
                                x_is.other(),
                                ey,
                                p,
                                super::Cell::Edge(edge_key),
                                band,
                                tol,
                                contacts,
                            )?;
                            push_vv(contacts, x_is, w, wy, on);
                            read_then_requeue(
                                &mut worklist,
                                (x, tips, x_is, edge_key, &edge),
                                y,
                                w,
                                j + 1,
                                &lying_in,
                                declared,
                                contacts,
                                band,
                                tol,
                            )?;
                            break 'faces;
                        }
                        FaceContainment::OnVertex(vy) => {
                            let w = split_logged(
                                x,
                                x_is,
                                edge_key,
                                t,
                                Some(y.resolve_vertex_point(vy, crate::live::Proven)),
                                super::Cell::Vertex(vy),
                                tol,
                                contacts,
                            )?;
                            push_vv(contacts, x_is, w, vy, on);
                            read_then_requeue(
                                &mut worklist,
                                (x, tips, x_is, edge_key, &edge),
                                y,
                                w,
                                j + 1,
                                &lying_in,
                                declared,
                                contacts,
                                band,
                                tol,
                            )?;
                            break 'faces;
                        }
                    }
                }
                // Endpoint(s) on the face plane: `dovertexonface`
                // (steps 2–4, 7–8). A fully coplanar pair (Zero, Zero)
                // deliberately gets endpoint treatment ONLY (module
                // docs: interior events surface via neighbor faces),
                // and a vertex a later split mints on the edge is read
                // against the face as the ends are (`lying_in`).
                (za, zb) => {
                    if (za, zb) == (Sign::Zero, Sign::Zero) {
                        lying_in.push(LyingIn::Plane(face));
                    }
                    let mut hit = false;
                    if za == Sign::Zero {
                        hit |= vertex_on_face(
                            x_is, y, u, pu, face, &plane, d1.margin, contacts, band, tol,
                        )?;
                    }
                    if zb == Sign::Zero {
                        hit |= vertex_on_face(
                            x_is, y, v, pv, face, &plane, d2.margin, contacts, band, tol,
                        )?;
                    }
                    if hit && let Some(tr) = trace.as_deref_mut() {
                        tr.accepted.push((edge_key, face));
                    }
                }
            }
        }
    }
    Ok(())
}

/// An edge end: the vertex and its point.
type End<T> = (VertexKey, Point3<T>);

/// An edge's two ends, `start(he_plus)` then `start(he_minus)`.
fn edge_ends<T: Decide>(
    x: &Body<T>,
    edge: &crate::entity::Edge,
) -> Result<(End<T>, End<T>), BooleanError> {
    let vert = |he| -> Option<End<T>> {
        let vk = x.get_half_edge(he)?.start;
        Some((vk, *x.get_point(x.get_vertex(vk)?.point)?))
    };
    match (vert(edge.he_plus), vert(edge.he_minus)) {
        (Some(a), Some(b)) => Ok((a, b)),
        _ => Err(BooleanError::ClassificationInvariant {
            what: "edge endpoints unresolvable",
        }),
    }
}

/// **The reduction sweep, both directions** (D9: A's edges first),
/// then the deferred touches settled on what both directions split
/// ([`settle_deferred`]). Every boolean driver sweeps through here,
/// with each operand's slit tips read at rest
/// ([`crate::sector_shape::SlitTips`]).
///
/// `T: Decide + Bounds` is the driver seam [`sweep_direction`] carries
/// (the face tree reads brackets): this forwards to it and adds no
/// bracket read of its own.
#[allow(clippy::too_many_arguments)] // the two directions' knobs and traces, side by side
pub(super) fn sweep_and_settle<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (a, b): (&mut Body<T>, &mut Body<T>),
    [a_tips, b_tips]: &[SlitTips; 2],
    declared: &super::GluedPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    strategy: SweepStrategy,
    [ab_knobs, ba_knobs]: [&SweepKnobs; 2],
    [mut ab_trace, mut ba_trace]: [Option<&mut SweepTrace>; 2],
    tol: Tol,
) -> Result<(), BooleanError> {
    let mut deferred = Vec::new();
    // Both directions read one axis set, taken when the first curved
    // face is met.
    let axes = super::separating::OperandAxes::new(super::boxes::axis_key);
    sweep_direction(
        (a, a_tips),
        b,
        Operand::A,
        declared,
        contacts,
        band,
        strategy,
        ab_knobs,
        ab_trace.as_deref_mut(),
        &mut deferred,
        &axes,
        tol,
    )?;
    sweep_direction(
        (b, b_tips),
        a,
        Operand::B,
        declared,
        contacts,
        band,
        strategy,
        ba_knobs,
        ba_trace.as_deref_mut(),
        &mut deferred,
        &axes,
        tol,
    )?;
    settle_deferred(
        a,
        b,
        deferred,
        declared,
        contacts,
        band,
        [ab_trace, ba_trace],
        tol,
    )
}

/// A covered edge × curved-face pair whose touch lies inside the edge
/// ([`CurvedEvent::Deferred`]), deferred until both sweep directions
/// have run.
#[derive(Debug)]
pub(super) struct DeferredTouch {
    /// The operand the edge belongs to.
    x_is: Operand,
    /// The edge as it was read; a split keeps this key on its leading
    /// fragment.
    edge: EdgeKey,
    /// The edge's far end, `start(he_minus)`, where its fragments stop.
    end: VertexKey,
    /// The other operand's curved face.
    face: FaceKey,
    /// The typed frontier the pair answers if its fragments do not
    /// settle it.
    refusal: BooleanError,
}

/// **Settles the deferred touches on the edges' fragments**, after both
/// sweep directions have run.
///
/// Which operand's edges are swept first decides only which vertices
/// exist when a pair is read, and settling the deferred pairs last is
/// what makes that order immaterial to whether a covered touch is seen:
/// a touch inside an edge of one operand very often sits at a vertex of
/// the other (a fillet's tangent point, where its flat wall ends), and
/// that vertex splits the edge only when the other direction reaches
/// it. What settling adds is the ACCEPTANCE: the touch's records are
/// the other direction's (its vertex on this edge), and settling is
/// what licenses the pair once that vertex has put the touch at a
/// fragment's end, where the deferring direction had to refuse.
///
/// Each fragment, walked from the deferred key along `he_plus` to the
/// edge's far end, is read again by [`curved_face_arm`] against the
/// deferred face; a pair it accepts is written to its direction's
/// trace, as the sweep writes one. A fragment that clears or records is
/// done; one whose touch is still inside it, or that the arm reads as a
/// crossing, answers the pair's typed frontier. So does a pair nothing
/// split, read again exactly as it was deferred.
#[allow(clippy::too_many_arguments)]
pub(super) fn settle_deferred<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &mut Body<T>,
    b: &mut Body<T>,
    deferred: Vec<DeferredTouch>,
    declared: &super::GluedPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    mut traces: [Option<&mut SweepTrace>; 2],
    tol: Tol,
) -> Result<(), BooleanError> {
    for d in deferred {
        let (x, y, trace): (&Body<T>, &mut Body<T>, _) = match d.x_is {
            Operand::A => (&*a, &mut *b, traces[0].as_deref_mut()),
            Operand::B => (&*b, &mut *a, traces[1].as_deref_mut()),
        };
        let mut trace = trace;
        let mut fragment = d.edge;
        // A fragment per pass; the walk ends at the deferred edge's far
        // end within one pass per edge of `x`.
        let mut reached = false;
        for _ in 0..x.edges().count() {
            let edge =
                x.get_edge(fragment)
                    .cloned()
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "a deferred edge's fragment vanished",
                    })?;
            let ((u, pu), (v, pv)) = edge_ends(x, &edge)?;
            match curved_face_arm(
                x, y, d.x_is, fragment, &edge, u, v, d.face, pu, pv, declared, contacts, band, tol,
            )? {
                CurvedEvent::None
                | CurvedEvent::Lies {
                    recorded: false, ..
                }
                | CurvedEvent::Along { recorded: false } => {}
                CurvedEvent::Recorded
                | CurvedEvent::Lies { recorded: true, .. }
                | CurvedEvent::Along { recorded: true } => {
                    if let Some(tr) = trace.as_deref_mut() {
                        tr.accepted.push((fragment, d.face));
                    }
                }
                CurvedEvent::Deferred | CurvedEvent::Pierce { .. } => return Err(d.refusal),
            }
            if v == d.end {
                reached = true;
                break;
            }
            // A split leaves its trailing child's `he_plus` next after
            // the parent's, starting at the minted vertex.
            let next_key = x.get_half_edge(edge.he_plus).map(|he| he.next);
            let next = next_key.and_then(|k| Some((k, x.get_half_edge(k)?)));
            let Some((next_key, next)) = next else {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a deferred edge's fragment chain is unresolvable",
                });
            };
            let leads = x.get_edge(next.edge).map(|e| e.he_plus) == Some(next_key);
            if next.start != v || !leads {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a deferred edge's fragments do not chain",
                });
            }
            fragment = next.edge;
        }
        if !reached {
            return Err(BooleanError::ClassificationInvariant {
                what: "a deferred edge's fragments do not reach its far end",
            });
        }
    }
    Ok(())
}

/// The one-sided covers an edge's parent faces hold against `face`
/// ([`super::GluedPairs::cover`]), one per covering parent.
fn edge_covers<T: Decide>(
    x: &Body<T>,
    x_is: Operand,
    edge: &crate::entity::Edge,
    face: FaceKey,
    declared: &super::GluedPairs<T>,
) -> Vec<super::CoverSide> {
    [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .flatten()
    .filter_map(|f| declared.cover(x_is, f, x_is.other(), face))
    .collect()
}

/// The curved-face sweep arm: endpoint sides come from the linearized
/// implicit residual; a definite miss is PROVEN — for a LINE carrier
/// against a cylinder or sphere the residual is convex (both-inside
/// means no wall crossing, both-outside clears through the span
/// minimum), against a torus or a cone the certified roots decide, and
/// for a CONIC carrier (a circle or an ellipse, `geom_brep::Conic`) the
/// ARC's residual range is enclosed two ways (the carrier's exact
/// harmonic bounds and the arc's own chord-dip bound), so a definitely
/// one-sided arc clears. What definitely MEETS the face is split by
/// kind, and the third paragraph below is the statement of record: a
/// LINE, a CIRCLE or an ELLIPSE carrier against a CYLINDER wall, a
/// SPHERE, a TORUS or a CONE is routed through the certified roots and
/// pierces; everything else — a tangency, a line parallel to a cone's
/// generator, an undeclared on-carrier edge, a trim with no verdict —
/// refuses typed at the named frontier door
/// ([`BooleanError::CurvedPierceUnsupported`]), and a root at a cone's
/// apex at its own ([`BooleanError::CrossingAtConeApex`]). An in-band
/// clearance escalates (F6, the same margin's other half) — except an
/// uncovered conic's against one of those four kinds, where the
/// certified roots decide what the enclosures could not. Spiric and
/// NURBS carriers have no enclosure and refuse
/// ([`BooleanError::EdgeCarrierUnsupported`]) before any clearance test.
/// Never a silent fallback.
///
/// **The carrier-identity rung** comes before any enclosure on a
/// CIRCLE carrier: an edge whose parent face is declared one carrier with
/// `face`, on a carrier the ladder calls the same, lies on `face`'s
/// carrier, so its clearance is zero by that certificate
/// ([`on_declared_shared_carrier`]). The sampled enclosure cannot say so
/// of a coincident pair — it is `±charge` about an identically-zero
/// residual and reads definitely negative.
///
/// **The one-sided cover rung** (C4's cover clause at the crossing
/// layer): a zero-clearance incidence whose edge has a parent face
/// whose carrier is CERTIFIED to lie in one closed side of `face`'s
/// ([`super::GluedPairs::one_sided`]) — by a verified `Rest` or
/// continuation (the edge bounds a face on the shared carrier, so it
/// lies ON that carrier), a verified `Tangent` (the on-carrier locus IS
/// the verified tangency: the ruling a tangent edge realizes), or a
/// structural tangency on either operand to a face verified one carrier
/// with `face` (a flat wall whose fillet strut is tangent to the other
/// plate's continued corner cylinder) — takes the planar sweep's
/// endpoint posture instead of the frontier door. A roots-lane
/// "tangent" verdict is a band decision and never a source, so a
/// graze within the band keeps the frontier. The cover records
/// endpoints only, so a covered pair whose ends place nothing on the
/// face and which no enclosure or root set clears is deferred
/// ([`CurvedEvent::Deferred`]: a LINE against a wall or a sphere
/// whose roots do not settle, or a covered circle with no end on the
/// carrier, its interior unseen, [`Placement::declared`]'s rule) and
/// read again on the edge's fragments
/// once both sweep directions have run ([`settle_deferred`]), save one
/// touching the face's boundary where the two tangent sheets part,
/// which is split there ([`interior`]). Each
/// on-carrier endpoint is
/// classified through the boundary pre-pass rows
/// ([`super::contain::curved_face_containment`] — the boundary walk,
/// and behind it the face's region), producing the same v-v record
/// family, or the v-f record when the region places the endpoint
/// strictly inside.
///
/// **Each on-carrier endpoint has three outcomes, not two**
/// ([`Placement`]). It is RECORDED; or the trim certifies it OUT of
/// this face, in which case it is eventless HERE and the pair's other
/// endpoint decides; or the door returns no verdict, which keeps the
/// typed frontier refusal. The middle case is a decision and not a
/// remainder: a shared carrier is covered by several faces per side, so
/// an endpoint outside THIS face's window is a site some sibling face
/// holds, and the sweep reaches that pair on its own visit.
///
/// **An edge lying ON the carrier is asked about its interior first**
/// ([`interior`]): where it crosses `face`'s boundary between
/// its ends — a shaft's seam ruling passing a full-turn bore's rim, a
/// rim arc passing the partner's seam ruling — the crossing is split
/// and recorded as a pierce landing on that boundary. With the interior
/// certified clear, a pair with both ends `Elsewhere` lies wholly
/// outside the face and is no event; without that certificate (a
/// `Tangent`-covered arc, which only touches the carrier) it still
/// refuses, since an overlap inside this face's window with both
/// endpoints beyond it would be an incidence this arm cannot see.
///
/// UNDECLARED incidences unlock no RECORDING they did not already have
/// — the recording door only widens what a verified declaration
/// unlocks. Two undeclared arms do read the third outcome, and they
/// read it as SILENCE where the strict posture refused: a `NoInterior`
/// span — the MIXED-SIGN one (one endpoint ON the carrier, the other
/// definitely clear) and the `(Zero, Zero)` span with both ends on the
/// carrier — whose every ON endpoint is certified `Elsewhere` answers
/// NO EVENT on this face ([`Placement::undeclared_no_interior`]). That
/// is a claim of absence, and it rests on certificates, not on a
/// sibling face existing: the certified roots put every meeting of the
/// line with the carrier at a root, each interior root was placed
/// outside this face's trim, and each end was placed outside it too,
/// so the span meets this face nowhere. Where the ends' incidences live
/// — a pierce vertex interior to a sibling face, an operand's own
/// vertex on another face, or nowhere on this operand at all — is not
/// this pair's question.
///
/// **The pierce ring lane** (the definite-crossing half): a LINE edge
/// that definitely crosses a cylinder WALL, a SPHERE, a TORUS or a CONE
/// inside that face's trim is no longer the frontier. Its crossing
/// parameters are the same certified roots the ray lane has always
/// solved — the quadratic on a wall
/// ([`super::solid_contain::line_wall_roots`]) and on a sphere
/// ([`super::solid_contain::line_sphere_roots`]), the quartic on a
/// torus ([`super::solid_contain::line_torus_roots`]), the cone's
/// quadric form ([`line_cone_roots`], on the ray lane's quadratic) —
/// taken over the edge's own span instead of a ray's forward half; on
/// a torus or a cone the roots are consulted for every endpoint-sign
/// pattern, since the residual is not convex along a line and no
/// endpoint datum bounds a crossing between; the landing point is
/// placed by [`super::contain::curved_face_containment`]; the
/// split/record triple is the planar conic lane's, verbatim. Still the
/// frontier is everything the roots do not cover: a TANGENCY (an
/// in-band discriminant, or a torus root count the quartic cannot
/// certify, is not a crossing at any order this lane sees), a line
/// parallel to a cone's generator, a root at a cone's apex, a root the
/// band cannot place, and a trim the chart door declines to express.
///
/// **A CIRCLE against a SPHERE, a CYLINDER, a TORUS or a CONE takes the
/// same arms as a line.** A circle or an ellipse against a sphere or a
/// cylinder wall has a degree-2 trigonometric polynomial for its
/// residual, in its eccentric anomaly, which one door decides
/// ([`super::conic_quadric`]): a first harmonic, with closed-form roots,
/// wherever its second harmonic is in the zero band (a circle against a
/// sphere, a circle square to a wall's axis), a quartic in the tangent
/// half-angle otherwise. Against a cone the same door reads the cone's
/// quadric form, of degree two along a conic, and refuses at the apex.
/// Against a torus a circle's residual is of
/// degree two ([`super::circle_torus`]) and an ellipse's of degree four
/// ([`super::ellipse_torus`]). Every such door's
/// answer is the certified subdivision's, decided on the residual itself
/// ([`super::circle_roots`]). A conic reaches those arms only from the
/// conic rung, after the enclosures failed to clear the arc, and never
/// through a one-sided cover arm — those rest on a line's separation
/// story.
///
/// A successful wall pierce reaches the join with a ring in the
/// pierced face, and the ring's chords take the arc the pierce germs'
/// directions name (`chord_join::Leave`).
///
/// **This lane WIDENS what an undeclared pair reaches, and the widening
/// is named here rather than left to be discovered.** Before it,
/// [`vertex_on_curved_face`] was reachable only behind the declared
/// cover; the new endpoint arms (`(Zero, definite)` and the
/// `(Zero, Zero)` chord) call it on UNDECLARED pairs too. That is not
/// the C8 gate reopening: C8 protects the claim that an on-carrier EDGE
/// is cosurface, and the arms below reach the endpoint treatment only
/// after the certified roots have proved there is no interior crossing.
/// The `(Zero, Zero)` arm takes an edge two ways. A chord it takes on
/// distinct certified roots, which exclude an edge lying on the carrier
/// (on a cylinder only rulings do, and a ruling answers `Constant`; on a
/// cone only generators do, which its lead rung leaves unanswered; no
/// line lies on a torus); what the door then does is point-in-face
/// containment on a chart, which is a trim question and not a gluing
/// one. An arc lying on the carrier (`LiesOn`) or a ruling (`Constant`
/// with both ends on it) it takes only when every parent is decided a
/// DIFFERENT carrier: a curve where two carriers meet, so it asks no
/// cosurface question either.
///
/// Returns what the caller must do about the pair — see
/// [`CurvedEvent`]. The split itself needs `&mut x` and the worklist,
/// both of which live in [`sweep_direction`], so the crossing is
/// REPORTED here and performed there rather than the body being
/// threaded in for one branch.
#[allow(clippy::too_many_arguments)]
pub(super) fn curved_face_arm<T: Decide + Bounds + crate::props::AtRestPolicy>(
    x: &Body<T>,
    y: &mut Body<T>,
    x_is: Operand,
    edge_key: EdgeKey,
    edge: &crate::entity::Edge,
    u: VertexKey,
    v: VertexKey,
    face: FaceKey,
    pu: Point3<T>,
    pv: Point3<T>,
    declared: &super::GluedPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<CurvedEvent<T>, BooleanError> {
    let surface = y
        .get_face(face)
        .and_then(|f| y.get_surface(f.surface))
        .cloned()
        .ok_or(BooleanError::ClassificationInvariant {
            what: "curved sweep arm: face surface lost",
        })?;
    let frontier = || BooleanError::CurvedPierceUnsupported {
        operand: x_is,
        face,
        edge: edge_key,
        band,
    };
    // A root door's decision read in band (a classifying sign in the gap,
    // or a root finder's touch) of a span the face may hold: the decision
    // escalates, located at the edge and the face.
    let located = |decision, diag| BooleanError::CurvedPierceEscalated {
        operand: x_is,
        face,
        edge: edge_key,
        decision,
        diag,
    };
    let escalate = |decision, diag| Err(located(decision, diag));
    let at_apex = || BooleanError::CrossingAtConeApex {
        operand: x_is,
        face,
        edge: edge_key,
        band,
    };
    // The one-sided cover (docs above): one of the edge's parent faces
    // has its carrier certified to lie in one closed side of `face`'s
    // carrier, so the edge's residual against it is one-signed and an
    // on-carrier endpoint is a touch, never an entry. The numeric rows
    // then certify each incidence. `read` is what each question below
    // reads of the parent faces' declaration against `face`: none of
    // them is one a declaration settles.
    let covers = edge_covers(x, x_is, edge, face, declared);
    let covered = !covers.is_empty();
    // Whether a point of the edge whose residual against `face`'s
    // carrier decided `sign` lies where some parent's cover certifies it.
    let admitted = |sign| covers.iter().any(|c| c.admits(sign));
    let read = |which| edge_face_read(x, x_is, edge, face, declared, which);
    let curve = certified(x.get_curve_geom(edge.curve))?.clone();
    // The curve × face table's door for the pair. A spline face has no
    // residual for the arms below: it is read through the table's
    // boundary pass alone ([`spline_face`]). The analytic doors are the
    // arms below, not yet dispatched through the table.
    let kind = surface.kind();
    if geom_brep::curve_face::route(curve.carrier().kind(), kind).door
        == geom_brep::curve_face::Door::SplineFace
    {
        return spline_face(
            (x, edge_key),
            y,
            x_is,
            &curve,
            [(u, pu), (v, pv)],
            (face, kind),
            contacts,
            band,
            tol,
        );
    }
    // **An edge on a wall of an equal cylinder pair with meeting axes,
    // against the pair's other wall**: read through the two bisector
    // planes, whose crossings are first order where the quadric's are
    // not ([`factored_crossing`]).
    if !covered
        && matches!(surface, geom::Surface::Cylinder { .. })
        && let Some(event) = factored_crossing::factored_crossing(
            (x, x_is),
            (edge, curve.carrier(), curve.params()),
            [(u, pu), (v, pv)],
            (&mut *y, face),
            declared,
            contacts,
            frontier,
            band,
            tol,
        )?
    {
        return Ok(event);
    }
    // Conic carriers: a CIRCLE carrier gets a definite-miss verdict in
    // closed form, and the verdict is the edge's ARC, not the carrier
    // it rides. Two enclosures of the residual are folded, and the
    // better one decides:
    //
    // - **the carrier's**: the residual over the WHOLE circle is a
    //   degree-≤2 trigonometric polynomial against a sphere/cylinder,
    //   so its range has exact amplitude bounds
    //   (`geom_brep::circle_residual_extremes`). Tight for a full-turn
    //   edge; for a short arc it answers about geometry the edge does
    //   not occupy, which is what made a corner round's carrier — not
    //   its arc — decide a cut (#347).
    // - **the arc's**: the residual is SAMPLED across the arc at
    //   `geom_brep::ARC_RESIDUAL_SAMPLES` steps and the sample hull is
    //   widened by one sub-arc's chord-dip charge — a smooth function
    //   stays within `|F″|·h²/8` of the chord of a sub-arc of width
    //   `h` (`geom_brep::circle_arc_residual_range`). Tight for a
    //   short arc, useless for a full turn, and it is what gives the
    //   torus a verdict at all: the torus's composed residual has no
    //   harmonic form, so the sampled enclosure is the only one it
    //   has, on the arc and on the whole turn alike.
    //
    // Both are valid enclosures of the ARC's range, so the clearance
    // margin is the larger of the two one-sidedness margins: definitely
    // one-sided — the arc strictly outside, or strictly inside — means
    // no wall crossing (meters). Anything else keeps the typed frontier
    // door, and an in-band clearance escalates (two-tolerance on the
    // arm, definite ones included). The rung is the CONIC's: an ellipse
    // reads the same two enclosures (`geom_brep::Conic`).
    let side = |p: Point3<T>| {
        geom_core::k_stats::decide_reported(
            "bool_vertex_face_side",
            Margin::of(geom_brep::implicit_residual(&surface, p)),
            band,
        )
    };
    // **An uncovered arc with an end ON the carrier is never asked its
    // clearance.** Its residual is exactly zero at that end, so its true
    // one-sidedness margin is at most zero and the clearance can never
    // read `Positive`, the one answer that returns early. Every other
    // answer an uncovered arc against these kinds can get (`Zero`,
    // `Negative`, escalated) falls through to the endpoint arms below.
    // So asking would decide nothing. What it would RECORD is the
    // sampled enclosure's own chord-dip charge, read as `−charge` about
    // that zero end: a margin of the enclosure, not of the geometry,
    // ε-independent and micrometres small, which the K telemetry reads as
    // a feature crowding its floor. That is the split fragment of a
    // carved sphere's meridian, ending on the cut. The endpoint sides
    // are therefore decided first, here, and handed to the endpoint arms
    // rather than decided twice. A held escalation surfaces where it
    // always did, in those arms. It is dropped only when the clearance
    // reads definitely clear, which an in-band end cannot let happen.
    let early_ends = (!covered
        && matches!(
            curve.carrier(),
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }
        )
        && matches!(
            surface,
            geom::Surface::Torus { .. }
                | geom::Surface::Sphere { .. }
                | geom::Surface::Cylinder { .. }
                | geom::Surface::Cone { .. }
        ))
    .then(|| (side(pu), side(pv)));
    let zero = |d: &Result<geom_core::Decided, _>| matches!(d, Ok(d) if d.sign == Sign::Zero);
    let end_on_carrier = early_ends
        .as_ref()
        .is_some_and(|(d1, d2)| zero(d1) || zero(d2));
    match (geom_brep::Conic::of(curve.carrier()), curve.carrier()) {
        (None, geom::Curve3::Line { .. }) => {}
        (Some(conic), _) => 'clearance: {
            if end_on_carrier {
                break 'clearance;
            }
            // **The carrier-identity rung, consulted FIRST.** An edge
            // bounding a face whose carrier the door verified to BE
            // `face`'s carrier lies on `face`'s carrier identically, so
            // its clearance is zero by that certificate — not by an
            // enclosure. The enclosures below cannot say so: the
            // sampled one is `±charge` about an identically-zero
            // residual and reads definitely negative by construction,
            // and the harmonic one has no torus form. Reading the
            // certificate first is what lets a coincident pair reach
            // the one-sided cover rung at all.
            let on_carrier = on_declared_shared_carrier(x, x_is, edge, face, declared);
            // **A covered arc lying on a carrier its parents are all
            // decided distinct from is an ON event first**, the
            // uncovered `(Zero, Zero)` arm's `LiesOn` lane: a seam's
            // rim lies on the partner's carrier, and the cover must not
            // pre-empt the lane that places it. The sampled clearance
            // below reads an identically-zero residual as definitely
            // negative, which under a cover is the frontier, and the
            // roots that answer `LiesOn` sit behind the uncovered arm.
            if covered && !on_carrier {
                let (t0, t1) = curve.params();
                if let Ok(SpanVerdict::LiesOn) =
                    wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band, &escalate)
                    && parents_distinct_from(x, edge, y, face, band)
                {
                    let arc = ArcOnCarrier {
                        x,
                        x_is,
                        edge_key,
                        ends: [(u, pu, None), (v, pv, None)],
                        face,
                        side: Coincide::VertexOnCoveredFace,
                        band,
                        tol,
                    };
                    return lying_on(&arc, y, contacts)?.ok_or_else(frontier);
                }
            }
            let clearance = if on_carrier {
                Ok(Sign::Zero)
            } else {
                conic_clearance(&surface, &conic, curve.params(), band).ok_or_else(frontier)?
            };
            match clearance {
                Ok(Sign::Positive) => return Ok(CurvedEvent::None),
                // The cone's clearance bounds the residual's magnitude from
                // BELOW only (`conic_clearance`), so a reading short of
                // clear is not the touch the covered arm reads it as.
                Ok(Sign::Zero | Sign::Negative) | Err(_)
                    if covered && !on_carrier && matches!(surface, geom::Surface::Cone { .. }) =>
                {
                    return Err(frontier());
                }
                // The one-sided cover rung: a covered zero-clearance
                // circle takes the planar sweep's endpoint posture —
                // each endpoint's own side decides its treatment
                // (existing row): ON the carrier ⇒ boundary
                // containment (which must decide, or the frontier
                // stands); definitely off on the side the cover
                // certifies ⇒ no event at that end (a covered circle
                // touches the carrier where it meets it); off on the
                // other side ⇒ the frontier. An arc touching the face's
                // boundary where the sheets part is split there
                // ([`interior`]); one with no end on the carrier, its
                // interior otherwise unseen, is deferred to the
                // fragments both directions leave ([`settle_deferred`]),
                // read by this arm as the other operand order reads
                // them. Uncovered keeps both doors verbatim.
                //
                // **An endpoint the trim places definitely OUT of THIS
                // face is eventless HERE, not a refusal.** Reading that
                // certificate as the frontier's remainder refuses a
                // pair that has no incidence to report — and refuses
                // the whole op, since one refusing pair is enough.
                //
                // `Out` has TWO sources on the carrier, and the argument differs by
                // source, so it is made per source rather than for the
                // one that motivated the change:
                //
                // - **AZIMUTH** — the endpoint is past this face's
                //   angular window. On a closed carrier the azimuth is
                //   covered by the operand's own wall faces, so the
                //   point is a seam site a SIBLING face holds, and the
                //   sweep reaches that pair on its own visit.
                // - **HEIGHT** — the endpoint is past the window in z.
                //   Here NO sibling need hold it: the carrier simply
                //   ends, and a floating peg's rim has no face of the
                //   other operand under it at all.
                //
                // An OFF-CARRIER answer is not a third source: the
                // endpoint's residual decided `Zero` just above, so a
                // containment that puts it definitely off the carrier
                // contradicts that decision, and
                // [`vertex_on_curved_face`] reports it `Undecided`.
                //
                // The HEIGHT source has no neighbour to appeal to, so the
                // widening does NOT rest on one existing. What carries
                // it is the **interior question**, asked before either
                // end: an arc lying on the carrier that crosses this
                // face's boundary between its ends is split there
                // ([`interior`]), and only an arc whose
                // interior is certified clear of the boundary reads an
                // all-`Elsewhere` pair as lying wholly outside the face
                // ([`Placement::declared`]).
                //
                // Still keeping the door: a no-verdict endpoint
                // ([`Placement::Undecided`]) and a boundary the interior
                // question has no closed form for. An arc with no end on
                // the carrier and its interior unseen (a `Tangent` cover,
                // no carrier identity, and no boundary touch where the
                // sheets part) is deferred: it meets the face, if at all,
                // strictly inside, at one point or several, and its
                // fragments are read again once both directions have split
                // it.
                Ok(Sign::Zero) if covered => {
                    let inside = interior(
                        on_carrier,
                        (x, edge_key, edge),
                        y,
                        x_is,
                        face,
                        &curve,
                        declared,
                        band,
                        (frontier, &located),
                    )?;
                    if let Interior::Crossing(event) = inside {
                        return Ok(event);
                    }
                    let side = |p: Point3<T>| {
                        geom_core::k_stats::decide_reported(
                            "bool_vertex_face_side",
                            Margin::of(geom_brep::implicit_residual(&surface, p)),
                            band,
                        )
                    };
                    let mut ends = [None, None];
                    for (i, (w, pw)) in [(u, pu), (v, pv)].into_iter().enumerate() {
                        let decided = side(pw).map_err(|diag| {
                            let which = Coincide::VertexOnCoveredFace;
                            BooleanError::coincidence(which, read(which), diag)
                        })?;
                        match decided.sign {
                            Sign::Zero => {
                                ends[i] = Some(vertex_on_curved_face(
                                    x_is,
                                    y,
                                    w,
                                    pw,
                                    face,
                                    decided.margin,
                                    contacts,
                                    band,
                                    tol,
                                )?);
                            }
                            // Definitely off at this end, on the side the
                            // certificate puts the parent: honestly
                            // eventless, and not an endpoint the rule
                            // below weighs either way. Off on the OTHER
                            // side is a crossing the certificate rules
                            // out, and keeps the door.
                            sign if admitted(sign) => {}
                            Sign::Positive | Sign::Negative => return Err(frontier()),
                        }
                    }
                    let on = OnCurved {
                        rule: MintedRule::Declared {
                            interior_clear: inside.clear(),
                        },
                        side: Coincide::VertexOnCoveredFace,
                    };
                    return Placement::declared(ends, inside.clear())
                        .map(|event| event.lying(on))
                        .ok_or_else(frontier);
                }
                // **The circle × sphere, × cylinder, × cone and × torus
                // root lanes.** An arc the enclosures could not clear
                // against one of those kinds takes the same endpoint-sign arms as
                // a line below, and the certified roots of
                // [`super::conic_quadric`] / [`super::circle_torus`]
                // decide every one of them through [`wall_crossing`].
                // Those arms never read convexity for a circle (the
                // same-side pairs go to the roots, as a torus's do), so
                // nothing they conclude rests on the carrier being
                // straight. An
                // ESCALATED clearance goes the same way: the enclosures
                // are a shortcut in front of the roots, and a margin in
                // their escalation gap says only that the shortcut did
                // not decide — the roots still can.
                //
                // **Only an UNCOVERED circle falls through.** The
                // one-sided cover arms below rest on a line's separation
                // story, so a covered circle — whose covered-Zero case
                // returned above — keeps the frontier door here rather
                // than reaching them. That makes those arms structurally
                // line-only, which they assert.
                Ok(Sign::Zero | Sign::Negative) | Err(_)
                    if !covered
                        && matches!(
                            surface,
                            geom::Surface::Torus { .. }
                                | geom::Surface::Sphere { .. }
                                | geom::Surface::Cylinder { .. }
                                | geom::Surface::Cone { .. }
                        ) => {}
                // Uncovered, an arc reaching here with a decided
                // clearance is against a kind with no root lane; covered,
                // it definitely crosses. The frontier door either way.
                Ok(Sign::Zero | Sign::Negative) => return Err(frontier()),
                // An uncovered ESCALATED clearance cannot reach here: a
                // plane face never reaches this arm (`face_plane` routes it
                // to the planar lane first), and NURBS and `Approx` have no
                // clearance enclosure, so they returned the frontier before
                // deciding one. The four kinds left are the arm above's.
                // Reaching here is a dispatch desync.
                Err(_) if !covered => {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "an uncovered arc's escalated clearance reached the covered \
                               arm: the circle rung's kinds and its root lanes disagree",
                    });
                }
                // A declared pair's declaration is spent here. An
                // undeclared one, declared `Rest` on this face's
                // carrier, is on it by the carrier-identity rung above,
                // and takes the covered arm, whose endpoint sides then
                // refuse the same in-band pose: no declaration settles
                // it.
                Err(diag) => {
                    let which = Coincide::ArcOnCoveredFace;
                    return Err(BooleanError::coincidence(which, read(which), diag));
                }
            }
        }
        // A spline span: the curve × face table's ring-composite arm.
        (None, carrier @ geom::Curve3::Nurbs(net))
            if {
                let arm = geom_brep::curve_face::route(geom::CurveKind::Nurbs, kind);
                arm.door == geom_brep::curve_face::Door::SplineComposite && arm.implemented
            } =>
        {
            return spline_wall_event(
                (x, x_is, edge_key, edge),
                y,
                (carrier, net, curve.params(), &surface),
                [(u, pu), (v, pv)],
                face,
                (&covers, declared),
                contacts,
                band,
                tol,
            );
        }
        // A `Spiric` carrier: no enclosure of its residual along an arc
        // exists here (the sampled one needs bounds on the carrier's speed
        // and acceleration over the span), so nothing finds its crossings.
        (None, _) => {
            return Err(BooleanError::EdgeCarrierUnsupported {
                operand: x_is,
                edge: x
                    .split_root(edge_key, |_| false)
                    .map_err(BooleanError::SplitLineage)?,
                face,
                face_of: x_is.other(),
                site: super::EdgeCarrierSite::CurvedCrossing,
            });
        }
    }
    // The one-sided cover arms rest on a LINE's separation story; only an
    // uncovered circle reaches the endpoint arms (the circle rung above).
    let on_line = matches!(curve.carrier(), geom::Curve3::Line { .. });
    let on_face = |diag| {
        let which = Coincide::VertexOnCurvedFace;
        BooleanError::coincidence(which, read(which), diag)
    };
    let (d1, d2) = match early_ends {
        Some((d1, d2)) => (d1.map_err(on_face)?, d2.map_err(on_face)?),
        None => (side(pu).map_err(on_face)?, side(pv).map_err(on_face)?),
    };
    let (s1, s2, m1, m2) = (d1.sign, d2.sign, d1.margin, d2.margin);
    match (s1, s2) {
        // The one-sided cover rung: a covered line with endpoint(s) ON
        // the carrier takes the planar sweep's endpoint posture — the
        // `(za, zb)` branch mirrored: each Zero endpoint gets boundary
        // containment (which must decide, or the frontier stands).
        // What makes the `(Zero, Positive)` branch eventless is NOT
        // convexity (a convex residual's endpoint bound is its
        // MAXIMUM, not its minimum — q(0) = 0, q(1) > 0 can dip
        // negative between): it is the cover's invariant — the edge's
        // parent carrier lies wholly in ONE closed residual half-space
        // of `face`'s ([`super::GluedPairs::one_sided`]; for a
        // `Tangent` source that is the witness lane's separation
        // invariant, the contract sentence on
        // [`geom_brep::tangent_locus`], and a one-carrier source's
        // shared carrier is residual-zero identically) — so a covered
        // on-carrier edge's residual is one-signed and a Zero endpoint
        // is a touch, never an entry.
        // A configuration without that one-sign story must not be
        // admitted to the lane. The coaxial cylinder×sphere pair has
        // one (`crates/topo/tests/verbs_cylsph_tangent_residuals.rs`);
        // what keeps its circle arm out — no consumer builds the kiss
        // edge yet, and the arm must not pre-empt the rim routing — is
        // stated at [`geom_brep::tangent_locus`]. A NEGATIVE partner is
        // a genuine crossing — never the covered posture. Uncovered
        // keeps both frontier doors verbatim.
        // Both ends on the carrier. Where the carrier identity puts the
        // line ON it (a parent face verified as `face`'s carrier), its
        // interior is asked before its ends; a `Tangent`-covered line has
        // no such certificate and keeps the ends-only rule, save a touch
        // of the face's boundary where the sheets part, which is split
        // ([`interior`]). The ends-only
        // rule rests on the cover, not on the line's shape: the line lies
        // on its parent's carrier, which the certificate holds in one
        // closed side of `face`'s, so its residual is one-signed and its
        // interior touches the carrier nowhere it crosses it. (It need
        // not be a ruling: under a plane × torus seam a chord of the
        // tangent circle ends on the torus and runs inside the circle.)
        (Sign::Zero, Sign::Zero) if covered => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let on_carrier = on_declared_shared_carrier(x, x_is, edge, face, declared);
            let inside = interior(
                on_carrier,
                (x, edge_key, edge),
                y,
                x_is,
                face,
                &curve,
                declared,
                band,
                (frontier, &located),
            )?;
            if let Interior::Crossing(event) = inside {
                return Ok(event);
            }
            let hu = vertex_on_curved_face(x_is, y, u, pu, face, m1, contacts, band, tol)?;
            let hv = vertex_on_curved_face(x_is, y, v, pv, face, m2, contacts, band, tol)?;
            let on = OnCurved {
                rule: MintedRule::Declared {
                    interior_clear: inside.clear(),
                },
                side: Coincide::VertexOnCurvedFace,
            };
            Placement::declared([Some(hu), Some(hv)], inside.clear())
                .map(|event| event.lying(on))
                .ok_or_else(frontier)
        }
        (Sign::Zero, Sign::Positive) if admitted(Sign::Positive) => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let h = vertex_on_curved_face(x_is, y, u, pu, face, m1, contacts, band, tol)?;
            Placement::declared([Some(h), None], false).ok_or_else(frontier)
        }
        (Sign::Positive, Sign::Zero) if admitted(Sign::Positive) => {
            debug_assert!(
                on_line,
                "a covered circle keeps the frontier at the circle rung"
            );
            let h = vertex_on_curved_face(x_is, y, v, pv, face, m2, contacts, band, tol)?;
            Placement::declared([Some(h), None], false).ok_or_else(frontier)
        }
        // **One endpoint ON the surface, the other definitely off it.**
        // This is the shape a wall crossing LEAVES behind: the split
        // puts a vertex on the carrier and re-queues both fragments
        // against the same face, so each comes back with exactly one
        // Zero end. The exact roots are consulted FIRST, because an
        // interior crossing is still possible here and the endpoint
        // rows would silently miss it — the residual along a line is
        // convex and therefore lies BELOW its endpoint chord, so
        // `f(t₀) = 0` with `f(t₁) > 0` can still dip through zero
        // between. With no interior root the incidence is exactly the
        // endpoint's, and the v-on-curved-face door mints the same
        // record the planar sweep's `vertex_on_face` writes.
        //
        // `(Zero, Zero)` is NOT here: both endpoints on the carrier is
        // where the on-carrier-edge question could arise, so it has its
        // own arm below, which separates the chord from that question
        // structurally before it answers anything.
        (Sign::Zero, Sign::Positive | Sign::Negative)
        | (Sign::Positive | Sign::Negative, Sign::Zero) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band, &escalate)? {
                SpanVerdict::Pierce { t, p, at, on } => Ok(CurvedEvent::Pierce {
                    t,
                    p,
                    at,
                    on,
                    branch: None,
                    chart: None,
                }),
                // A constant residual cannot be zero at one end and
                // definite at the other, and a definite miss cannot
                // hold a zero endpoint: both CONTRADICT the endpoint
                // rows that routed here, and a contradiction between
                // two certified predicates keeps the door.
                SpanVerdict::Constant
                | SpanVerdict::LiesOn
                | SpanVerdict::Miss
                | SpanVerdict::Unsettled => Err(frontier()),
                SpanVerdict::AtApex => Err(at_apex()),
                SpanVerdict::NoInterior | SpanVerdict::Elsewhere | SpanVerdict::OffFace => {
                    // UNDECLARED: the undeclared `NoInterior` rule
                    // ([`Placement::undeclared_no_interior`]), over this
                    // span's one ON end.
                    //
                    // Under `OffFace` the ON end cannot place In, OnEdge
                    // or OnVertex on THIS face, so the two reads cannot
                    // disagree. The end is a meeting of the span with the
                    // carrier, so its piece never clears and it lies in a
                    // cluster, within `ℓ + |d(m)|` of the ball's foot:
                    // `escalate` inside the ball. Every vertex and edge
                    // of the face was decided farther from the foot than
                    // the ball's radius, so each is more than `escalate`
                    // from the end, beyond the band an On answer needs.
                    // In would contradict the same door's
                    // (`curved_face_placement`) `Out` at the foot across a
                    // disc of the carrier no boundary enters. The one
                    // `Recorded` left is the face-free vertex hit, a
                    // sibling face's incidence, which records here as it
                    // would there.
                    let mut ends = [None, None];
                    for (i, (on, w, pw, m)) in
                        [(s1 == Sign::Zero, u, pu, m1), (s2 == Sign::Zero, v, pv, m2)]
                            .into_iter()
                            .enumerate()
                    {
                        if on {
                            ends[i] = Some(vertex_on_curved_face(
                                x_is, y, w, pw, face, m, contacts, band, tol,
                            )?);
                        }
                    }
                    Placement::undeclared_no_interior(ends).ok_or_else(frontier)
                }
            }
        }
        // **BOTH endpoints on the carrier, the pair UNDECLARED** — the
        // chord between two pierces, or any edge whose two ends sit on
        // the carrier. The verdict decides which of two edges it is:
        //
        // - **`NoInterior` / `Elsewhere`: not an on-carrier edge.** Those
        //   are reached only through a certified root count with
        //   distinct roots. For a LINE: the lines that lie on a wall are
        //   its rulings, which answer `Constant`, and no line lies on a
        //   sphere or a torus. For a CIRCLE against a sphere: a circle
        //   lying on it is centred on its axis, answered `LiesOn`. For a
        //   CIRCLE against a torus: a circle lying on it is coaxial (a
        //   rim or latitude circle) or a meridian, both answered
        //   `LiesOn`, or a Villarceau circle, whose `F ≡ 0` puts the
        //   pole on the torus at every anchor (`Unsettled`). And its
        //   interior meets this face nowhere: the edge meets the carrier
        //   only at its certified roots, each root strictly inside the
        //   span was placed outside the trim, and each root at an end is
        //   that end's own incidence. So the ends decide, under the same rule as the
        //   mixed-sign arm ([`Placement::undeclared_no_interior`]).
        // - **`OffFace`**: every meeting of the span with the carrier, the
        //   ends' among them, lies off this face whether or not the edge
        //   lies on the carrier, so the ends decide under the same rule.
        //   Neither end can place on this face against the verdict (the
        //   mixed-sign arm above says why).
        // - **`LiesOn`: a circle or ellipse arc lying on the carrier**,
        //   exactly on by the conic root doors; **`Constant`: a ruling
        //   lying on a wall**, its residual decided Zero at both ends and
        //   its distance from the axis constant over the span. Either is an ON event (C4's
        //   one-sided cover, narrowed to touches) when every surface of a
        //   face it bounds is decided distinct from `face`'s by the
        //   carrier ladder: the edge is then a curve where two different
        //   carriers meet, not a cosurface question. It takes the
        //   coplanar conic's posture, endpoint processing only, once its
        //   interior is certified to meet this face's boundary nowhere it
        //   does not run along, and is split where it does meet it
        //   ([`lying_on`]).
        //
        // Every other answer keeps the door: the undeclared cosurface
        // question (CONTACT-DESIGN C2/C4), a parent the ladder does not
        // decide distinct, a tangency, and a trim with no verdict.
        (Sign::Zero, Sign::Zero) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band, &escalate)? {
                SpanVerdict::NoInterior | SpanVerdict::Elsewhere | SpanVerdict::OffFace => {
                    let hu = vertex_on_curved_face(x_is, y, u, pu, face, m1, contacts, band, tol)?;
                    let hv = vertex_on_curved_face(x_is, y, v, pv, face, m2, contacts, band, tol)?;
                    Placement::undeclared_no_interior([Some(hu), Some(hv)]).ok_or_else(frontier)
                }
                SpanVerdict::LiesOn | SpanVerdict::Constant
                    if parents_distinct_from(x, edge, y, face, band) =>
                {
                    let arc = ArcOnCarrier {
                        x,
                        x_is,
                        edge_key,
                        ends: [(u, pu, Some(m1)), (v, pv, Some(m2))],
                        face,
                        side: Coincide::VertexOnCurvedFace,
                        band,
                        tol,
                    };
                    lying_on(&arc, y, contacts)?.ok_or_else(frontier)
                }
                SpanVerdict::AtApex => Err(at_apex()),
                _ => Err(frontier()),
            }
        }
        // **A definite surface crossing: the pierce RING lane.** The
        // endpoints straddle the carrier, so a root exists in the span;
        // `wall_crossing` finds it exactly and the trim decides whether
        // it lands in this face. A root set that ACCOUNTS for the
        // straddle off this face ([`SpanVerdict::Elsewhere`]) is no
        // event here; a `NoInterior` that does not — no root inside the
        // span, or one at an end the endpoint signs call definite —
        // CONTRADICTS the straddle, so it keeps the door rather than
        // reporting no event: a contradiction between two certified
        // predicates is exactly what must not be resolved by picking
        // one.
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band, &escalate)? {
                SpanVerdict::Pierce { t, p, at, on } => Ok(CurvedEvent::Pierce {
                    t,
                    p,
                    at,
                    on,
                    branch: None,
                    chart: None,
                }),
                // The straddle's crossing, found and certified OFF this
                // face: every root inside the span is on the carrier
                // outside the trim, and both ends are definitely off the
                // carrier, so the span meets this face nowhere. Where the
                // crossing's incidence lives — a sibling face on the same
                // carrier, or no face of this operand — is that face's
                // pair, not this one.
                SpanVerdict::Elsewhere => Ok(CurvedEvent::None),
                SpanVerdict::AtApex => Err(at_apex()),
                _ => Err(frontier()),
            }
        }
        // **Both endpoints on one side of a TORUS or a CONE, or of
        // anything along a CIRCLE: the roots decide, with no endpoint
        // bound in front of them.** The two arms below lean on the
        // residual being CONVEX along a line, which holds for a cylinder
        // and a sphere and fails for a torus: its residual
        // `((ρ − R)² + h² − r²)/2r` carries `−2Rρ`, concave in the line
        // parameter. So a segment with both ends inside the tube can leave
        // it and come back (a chord across the hole), and one with both
        // ends outside can dip through it anywhere — no endpoint datum
        // bounds either. A cone's `ρ cos α − |h| sin α` carries `−|h|`,
        // concave too: a segment from inside one nappe to inside the other
        // crosses both. Along a circle no
        // kind's residual is convex (against a sphere it is
        // `c₀ + A₁cos(θ − φ)`), so an arc takes this arm whatever the
        // face. The certified roots do:
        // an interior root in this face's trim is a pierce, a certified
        // absence of one is no event HERE, and an uncertain count keeps
        // the door.
        (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative)
            if !on_line
                || matches!(
                    surface,
                    geom::Surface::Torus { .. } | geom::Surface::Cone { .. }
                ) =>
        {
            let (t0, t1) = curve.params();
            match wall_crossing(y, face, &surface, curve.carrier(), t0, t1, band, &escalate)? {
                SpanVerdict::Pierce { t, p, at, on } => Ok(CurvedEvent::Pierce {
                    t,
                    p,
                    at,
                    on,
                    branch: None,
                    chart: None,
                }),
                SpanVerdict::NoInterior
                | SpanVerdict::Elsewhere
                | SpanVerdict::Miss
                | SpanVerdict::OffFace => Ok(CurvedEvent::None),
                SpanVerdict::Constant | SpanVerdict::LiesOn | SpanVerdict::Unsettled => {
                    Err(frontier())
                }
                SpanVerdict::AtApex => Err(at_apex()),
            }
        }
        // Both inside: the residual along a line is convex (cylinder,
        // sphere — the torus and every circle took the arm above), so
        // its maximum is at an endpoint — definitely no wall crossing.
        (Sign::Negative, Sign::Negative) => Ok(CurvedEvent::None),
        // Both outside: clear through a DIVISION-FREE lower bound on
        // the span minimum of the convex residual. Division-free is a
        // hard requirement, not a preference: the original
        // parabola-vertex formula divided by the transverse direction
        // norm, which is 0/0 on an axis-parallel edge — and the
        // IDEALIZED sweep examines exactly those at distance, so the
        // poison took the whole op with it.
        //
        // **The vertex is CLAMPED to the segment, and that is what the
        // bound buys.** Write `q = f″·Δt²` and `m = f(t₁) − f(t₀)`,
        // both metres. For a convex quadratic the interior minimum
        // exists only when the vertex lies inside the span, which is
        // exactly `|m| < q/2`; outside that the function is monotone
        // and its minimum IS an endpoint. So the dip below the
        // endpoint chord is
        //   dip = (q/2 − |m|)² / (2q)   when |m| < q/2,  else 0
        // and since `(q/2 − |m|) ≤ q/2` the quotient is at most
        // `(q/2 − |m|)/4`. Taking that as the charge keeps the whole
        // computation in `max` and `min`:
        //   dip ≤ max(0, q/2 − |m|) / 4
        // — no division, EXACTLY ZERO when the vertex is outside the
        // segment, and exact again at the centred vertex (`m = 0`
        // gives `q/8`, the true worst case).
        //
        // Between those two it is loose, and the looseness is worth
        // stating truthfully rather than flatteringly: the ratio of
        // charge to true dip is `q / (2(q/2 − |m|))`, which is 1 at
        // `m = 0`, 4 at `m = 3q/8`, and UNBOUNDED as `|m|` approaches
        // `q/2`. What stays bounded is the ABSOLUTE charge, which
        // vanishes linearly there — so the multiplicative claim fails
        // exactly where the quantity being multiplied is going to
        // zero, and the bound never charges more than `q/8`.
        //
        // The old `q/8` charged the centred-vertex dip to every edge
        // whatever its endpoint gap, which is what made a pocket wall
        // 2 mm clear of a corner round read as a pierce (#347's
        // measured `r ≥ 5` bound).
        //
        // Conservative direction is unchanged: a too-large charge only
        // sends more pairs to the typed frontier door, never accepts.
        (Sign::Positive, Sign::Positive) => {
            let geom::Curve3::Line { origin: _, dir } = *curve.carrier() else {
                return Err(frontier()); // unreachable: matched above
            };
            let (t0, t1) = curve.params();
            // f″ per kind (the residual's second derivative along the
            // ray, constant for these kinds).
            let f2 = match surface {
                geom::Surface::Cylinder { axis, radius, .. } => {
                    let d_ax = dir.dot(axis);
                    (dir.norm_squared() - d_ax.powi(2)) / radius
                }
                geom::Surface::Sphere { radius, .. } => dir.norm_squared() / radius,
                // Post-gate/pre-check unreachable kinds keep the
                // frontier door.
                _ => return Err(frontier()),
            };
            let span = t1 - t0;
            let r_u = geom_brep::implicit_residual(&surface, pu);
            let r_v = geom_brep::implicit_residual(&surface, pv);
            // q = f''·Δt², m = the endpoint gap; both metres.
            let q = f2 * span.powi(2);
            let m = (r_v - r_u).abs();
            let dip = (q * T::from_f64(0.5) - m).max(T::zero()) * T::from_f64(0.25);
            let min_bound = Margin::of(r_u.min(r_v) - dip);
            match decide("bool_line_cylinder_clearance", min_bound, band) {
                Ok(Sign::Positive) => Ok(CurvedEvent::None),
                // **The BELLY crossing**, and the reason the dip bound
                // is a clearance test and never a crossing test: a
                // convex residual positive at both endpoints can still
                // dip through zero between them, and the bound is a
                // charge against that dip, not a measurement of it. A
                // charge that does not clear says only "not definitely
                // clear" — so the exact roots decide, and they decide
                // BOTH ways: two roots inside the span are a pierce,
                // and roots outside it (or none at all) are an
                // exactness the bound could not reach. Only what the
                // roots cannot settle keeps the frontier door.
                Ok(Sign::Zero | Sign::Negative) => {
                    // Covered, a door's escalation is the fragments' as an
                    // unsettled root set is (below).
                    let covered_escalate = |decision, diag| {
                        if covered {
                            Ok(SpanVerdict::Unsettled)
                        } else {
                            escalate(decision, diag)
                        }
                    };
                    match wall_crossing(
                        y,
                        face,
                        &surface,
                        curve.carrier(),
                        t0,
                        t1,
                        band,
                        &covered_escalate,
                    )? {
                        SpanVerdict::Pierce { t, p, at, on } => Ok(CurvedEvent::Pierce {
                            t,
                            p,
                            at,
                            on,
                            branch: None,
                            chart: None,
                        }),
                        // No interior root, or no root at all:
                        // definitely clear, and exactly so — the bound
                        // that sent us here could only ever have said
                        // "maybe".
                        SpanVerdict::NoInterior
                        | SpanVerdict::Elsewhere
                        | SpanVerdict::Miss
                        | SpanVerdict::OffFace => Ok(CurvedEvent::None),
                        // **`Constant` is NOT a clearance here.** It
                        // says the edge drifts off its distance from the
                        // axis by less than the band over its span, so
                        // the residual is constant along it, but not on
                        // which side of the wall: an edge running along
                        // the wall in band is a coincidence, and nothing
                        // upstream put this one definitely off it. So it
                        // keeps the door. (`solid_contain::cast_ray`
                        // skips the face on its own axis-parallel
                        // verdict, levered by the selection's reach,
                        // because its pre-pass has put `q` definitely off
                        // the wall.)
                        SpanVerdict::Constant | SpanVerdict::LiesOn => Err(frontier()),
                        SpanVerdict::AtApex => Err(at_apex()),
                        // Covered, the edge lies in one closed side of
                        // the carrier, so a root set the lane cannot
                        // certify (a tangency) is a touch inside the
                        // span. The residual along a line is convex here
                        // (a wall or a sphere), so its zero set is one
                        // point or one interval and a fragment's ends
                        // see it: deferred for the fragments.
                        SpanVerdict::Unsettled if covered => Ok(CurvedEvent::Deferred),
                        SpanVerdict::Unsettled => Err(frontier()),
                    }
                }
                Err(diag) => {
                    let which = Coincide::EdgeOnCurvedFace;
                    Err(BooleanError::coincidence(which, read(which), diag))
                }
            }
        }
    }
}

/// The two enclosures of a conic ARC's residual against `surface` (a
/// circle's or an ellipse's, `geom_brep::Conic`), folded: the carrier's
/// harmonic bounds and the arc's sampled chord-dip range. Both enclose
/// the arc's range, each with its own rounding charged
/// (`geom_brep::conic_residual_extremes`,
/// `geom_brep::conic_arc_residual_range`), so the clearance margin is
/// the larger of the two one-sidedness margins. `None` when the carrier
/// enclosure has no form for the kind.
///
/// **What certifies here, by kind.** Against a plane, sphere or
/// cylinder both enclosures read; at a vertex whose harmonics' phases
/// align, the carrier bound IS the residual's extreme, so its rounding
/// charge is what stands between a graze and a certified clearance.
/// Against a torus the carrier enclosure is the whole turn's sampled
/// one, levered by a curvature bound that is loose in the direction that
/// refuses: on grazes within 40 bands it certified none at ε 1e-12 and
/// 1e-9 and a handful at 1e-6, each clear under the oracle
/// (`clearance_rows`). Against a cone the residual has no harmonic form
/// either, and the margin is read off its quadric form `F`
/// ([`geom_brep::conic_cone_harmonics`]) instead: `|F| ≤ |res|` with the
/// same sign, so `F`'s whole-turn range, its rounding charged, is a
/// LOWER bound on the residual's one-sidedness — sound for a clearance,
/// and no bound on a touch.
///
/// The line row's vertex CLAMP does not port here, and the reason is
/// the curve: along a line the residual is exactly quadratic, so "the
/// vertex is outside the span" is a statement about a parabola and is
/// decided by the endpoint gap alone. Along a conic it has up to four
/// critical parameters, so an endpoint gap says nothing about where
/// its minimum sits. Subdivision is what is available without solving
/// for them.
fn conic_clearance<T: Decide>(
    surface: &geom::Surface<T>,
    conic: &geom_brep::Conic<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Option<Result<Sign, geom_core::Indeterminate>> {
    if let geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *surface
    {
        let h = geom_brep::conic_cone_harmonics(conic, apex, axis, half_angle);
        let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
        let swing =
            hypot(h.c1, h.s1) + hypot(h.c2, h.s2) + geom_core::rounding_charge(h.terms) / h.per;
        return Some(decide(
            "bool_conic_curved_clearance",
            Margin::of((h.c0 - swing).max(T::zero() - (h.c0 + swing))),
            band,
        ));
    }
    let (lo, hi) = geom_brep::conic_residual_extremes(surface, conic)?;
    let carrier_margin = lo.max(-hi);
    let arc_margin = geom_brep::conic_arc_residual_range(surface, conic, t0, t1)
        .map_or(carrier_margin, |(arc_lo, arc_hi)| arc_lo.max(-arc_hi));
    Some(decide(
        "bool_conic_curved_clearance",
        Margin::of(carrier_margin.max(arc_margin)),
        band,
    ))
}

/// **The carrier-identity rung**: does one of the edge's parent faces
/// sit on `face`'s carrier by a verified one-carrier declaration?
///
/// `Rest` and a continuation are the classes that license it, and only
/// where the declaration door's carrier ladder CALLED the pair one carrier
/// ([`super::GluedPairs::verified_one_carrier`], recorded once at the
/// door rather than re-derived per event). An edge bounding a face on
/// that carrier lies on it. `Tangent` licenses nothing of the kind — a
/// tangent pair shares a locus, not a carrier — and an undeclared pair
/// is never read as coincident by value (CONTACT-DESIGN C2/C4).
fn on_declared_shared_carrier<T: Decide>(
    x: &Body<T>,
    x_is: Operand,
    edge: &crate::entity::Edge,
    face: FaceKey,
    declared: &super::GluedPairs<T>,
) -> bool {
    [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .flatten()
    .any(|pf| declared.verified_one_carrier(x_is, pf, x_is.other(), face))
}

/// **An edge against a spline face** (C5's curve × face clause): the
/// table's boundary pass ([`geom_brep::curve_face::boundary`]), which
/// reads the edge against each of the face's boundary edges and asks
/// nothing of the surface.
///
/// - A meeting strictly inside the edge is a meeting on the face's
///   boundary: a [`CurvedEvent::Pierce`] the sweep splits both edges at,
///   placed on the boundary entity it lands on by the boundary pre-pass.
/// - An edge along a boundary edge is a coincidence the curve-pair
///   reading decided: its ends are placed on the boundary and recorded
///   against the vertices there (a boundary edge an end lands inside is
///   split), and the join glues the two edges
///   ([`CurvedEvent::Along`]).
/// - An edge whose interior the boundary leaves undecided is the
///   table's interior door ([`spline_interior`]), built for a line, a
///   circle or an ellipse; any other edge refuses the face,
///   [`BooleanError::CurvedBooleanUnsupported`]. One such edge is
///   decided without the door: an edge of `y` ([`twin_edge`]).
///
/// The pass and the boundary pre-pass are two readings of one point on
/// one boundary, so a point one puts on the boundary and the other off
/// it is a kernel fault, never a frontier.
#[allow(clippy::too_many_arguments)]
fn spline_face<T: Decide + crate::props::AtRestPolicy>(
    (x, edge_key): (&Body<T>, EdgeKey),
    y: &mut Body<T>,
    x_is: Operand,
    curve: &geom_brep::EdgeCurve<T>,
    ends: [End<T>; 2],
    (face, kind): (FaceKey, geom::SurfaceKind),
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<CurvedEvent<T>, BooleanError> {
    use super::carrier_cross::{boundary_refusal, read_boundary};
    use geom_brep::curve_face::Boundary;
    use geom_brep::curve_pair::Span;
    let unbuilt = || BooleanError::CurvedBooleanUnsupported {
        operand: x_is.other(),
        face,
        kind,
    };
    let swept = Span::new(curve.carrier(), curve.params());
    match read_boundary(y, face, &swept, band)? {
        Some(Ok(Boundary::Meets { t, .. })) => {
            let p = curve.carrier().eval(t);
            match super::contain::curved_boundary_containment(y, face, p, band)
                .map_err(|e| esc(e, x_is.other(), face))?
            {
                Some((at, margin)) => Ok(CurvedEvent::Pierce {
                    t,
                    p,
                    at,
                    on: Some(margin),
                    branch: None,
                    chart: None,
                }),
                None => Err(disagree()),
            }
        }
        Some(Ok(Boundary::Along { .. })) => {
            for end in ends {
                along_boundary(x_is, y, end, face, contacts, band, tol)?;
            }
            Ok(CurvedEvent::Along { recorded: true })
        }
        Some(Ok(Boundary::Clear)) => {
            if !twin_edge(y, (x_is, face, edge_key), &swept, ends, band)? {
                return spline_interior(
                    (x, x_is, edge_key),
                    y,
                    curve,
                    ends,
                    (face, kind),
                    contacts,
                    band,
                    tol,
                );
            }
            let mut recorded = false;
            for (w, pw) in ends {
                match super::contain::curved_boundary_containment(y, face, pw, band)
                    .map_err(|e| esc(e, x_is.other(), face))?
                {
                    Some((FaceContainment::OnVertex(vy), margin)) => {
                        push_vv(contacts, x_is, w, vy, Some(margin));
                        recorded = true;
                    }
                    None => {}
                    // An end at a vertex of `y` read inside an edge of `y`.
                    Some((
                        FaceContainment::OnEdge(_) | FaceContainment::In | FaceContainment::Out,
                        _,
                    )) => return Err(disagree()),
                }
            }
            Ok(if recorded {
                CurvedEvent::Recorded
            } else {
                CurvedEvent::None
            })
        }
        Some(Err(refusal)) => {
            boundary_refusal(refusal, (x_is, face, edge_key), false).and_then(|()| Err(unbuilt()))
        }
        None => Err(unbuilt()),
    }
}

/// **An edge's interior against a spline face** (C5's interior arm):
/// the meetings of the edge's span with the face's chart
/// ([`geom_brep::NurbsLane::chart_meetings`]), each placed on the face,
/// then the edge's ends.
///
/// - A meeting on the face's boundary, by the boundary pre-pass, or
///   inside its trim at the chart point it was found at
///   ([`super::spline_region::spline_chart_placement`]), is a
///   [`CurvedEvent::Pierce`]: the first, the sweep reading the fragments
///   again. A meeting outside the trim is no event of this face.
/// - With no meeting on the face, each end is placed on it
///   ([`super::contain::curved_face_placement_decided`]) and recorded
///   where it lies: on a vertex, on an edge (split there), or inside.
///
/// # Errors
///
/// [`BooleanError::CurvedBooleanUnsupported`] where the table has no
/// built arm for the edge's carrier or the scalar has no certified lane;
/// [`BooleanError::SplineCrossingUnsettled`] or
/// [`BooleanError::Escalated`] where the arm refuses;
/// [`BooleanError::CurvedPierceUnsupported`] where the trim reads no
/// verdict for a meeting or an end.
#[allow(clippy::too_many_arguments)]
fn spline_interior<T: Decide + crate::props::AtRestPolicy>(
    (x, x_is, edge_key): (&Body<T>, Operand, EdgeKey),
    y: &mut Body<T>,
    curve: &geom_brep::EdgeCurve<T>,
    ends: [End<T>; 2],
    (face, kind): (FaceKey, geom::SurfaceKind),
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<CurvedEvent<T>, BooleanError> {
    use geom_brep::curve_face::spline_face::SplineFaceRefusal;
    let unbuilt = || BooleanError::CurvedBooleanUnsupported {
        operand: x_is.other(),
        face,
        kind,
    };
    let frontier = || BooleanError::CurvedPierceUnsupported {
        operand: x_is,
        face,
        edge: edge_key,
        band,
    };
    if !geom_brep::curve_face::route(curve.carrier().kind(), kind).implemented {
        return Err(unbuilt());
    }
    let surface = y
        .get_face(face)
        .and_then(|f| y.get_surface(f.surface))
        .cloned()
        .ok_or(BooleanError::ClassificationInvariant {
            what: "spline face: the face's surface does not resolve",
        })?;
    let chart = surface
        .spline_chart()
        .ok_or(BooleanError::ClassificationInvariant {
            what: "spline face: the table routed a face with no spline chart",
        })?;
    let Some(found) = T::spline_chart_meetings(curve.carrier(), curve.params(), chart, band) else {
        return Err(unbuilt());
    };
    let found = found.or_else(|refusal| {
        Err(match refusal {
            SplineFaceRefusal::Escalated(diag) => BooleanError::Escalated {
                decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
                diag,
            },
            SplineFaceRefusal::Unsettled(why) => BooleanError::SplineCrossingUnsettled {
                operand: x_is,
                edge: x
                    .split_root(edge_key, |_| false)
                    .map_err(BooleanError::SplitLineage)?,
                face,
                face_of: x_is.other(),
                why,
            },
            SplineFaceRefusal::Unread(_) => unbuilt(),
            SplineFaceRefusal::Fault(what) => BooleanError::ClassificationInvariant { what },
        })
    })?;
    for m in found {
        let esc_y = |e| esc(e, x_is.other(), face);
        if let Some((at, margin)) =
            super::contain::curved_boundary_containment(y, face, m.point, band).map_err(esc_y)?
        {
            return Ok(CurvedEvent::Pierce {
                t: m.t,
                p: m.point,
                at,
                on: Some(margin),
                branch: None,
                chart: None,
            });
        }
        match super::spline_region::spline_chart_placement(y, face, m.uv, band).map_err(esc_y)? {
            Some(FaceContainment::In) => {
                return Ok(CurvedEvent::Pierce {
                    t: m.t,
                    p: m.point,
                    at: FaceContainment::In,
                    on: None,
                    branch: None,
                    chart: Some(m.uv),
                });
            }
            Some(FaceContainment::Out) => {}
            Some(FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_)) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "spline face: the trim placed a chart point on the boundary",
                });
            }
            None => return Err(frontier()),
        }
    }
    let mut recorded = false;
    for (w, pw) in ends {
        let (placement, on, chart) =
            super::contain::curved_face_placement_charted(y, face, pw, band)
                .map_err(|e| esc(e, x_is.other(), face))?;
        match placement {
            CurvedPlacement::Trim(Some(FaceContainment::OnVertex(vy))) => {
                push_vv(contacts, x_is, w, vy, on);
            }
            CurvedPlacement::Trim(Some(FaceContainment::OnEdge(ey))) => {
                let wy = split_other_at_point(
                    y,
                    x_is.other(),
                    ey,
                    pw,
                    super::Cell::Vertex(w),
                    band,
                    tol,
                    contacts,
                )?;
                push_vv(contacts, x_is, w, wy, on);
            }
            CurvedPlacement::Trim(Some(FaceContainment::In)) => {
                push_vf(contacts, x_is, w, face, on);
                if let Some(uv) = chart {
                    contacts.chart_point(x_is, w, face, uv);
                }
            }
            CurvedPlacement::Trim(Some(FaceContainment::Out)) | CurvedPlacement::OffCarrier => {
                continue;
            }
            CurvedPlacement::Trim(None) => return Err(frontier()),
        }
        recorded = true;
    }
    Ok(if recorded {
        CurvedEvent::Recorded
    } else {
        CurvedEvent::None
    })
}

/// The boundary pass and the boundary pre-pass read one point of a
/// face's boundary two ways ([`spline_face`]).
fn disagree() -> BooleanError {
    BooleanError::ClassificationInvariant {
        what: "spline face: the boundary pass and the boundary pre-pass place one point apart",
    }
}

/// **Whether the swept span is an edge of `y`**: one whose two ends are
/// the vertices of `y` at the span's ends and which the curve-pair
/// reading reads as one curve with it over a run of positive length
/// ([`geom_brep::curve_face::shares_a_run`]).
///
/// Its use rests on an edge of `y` meeting a face of `y` it does not
/// bound at most at its own vertices. That is the boolean's
/// precondition, not a check: an operand that meets itself is outside
/// the supported envelope ([`first_unsupported_pair`]'s docs), and tier 3
/// defers the whole-body self-intersection and clearance check
/// ([`mod@crate::validate`]'s "What tier 3 does NOT yet check"). `y` is the
/// reduction's working copy, split only at points of its own edges, so
/// its geometry is the operand's.
///
/// So against a face of `y` whose boundary leaves the span
/// [`Boundary::Clear`](geom_brep::curve_face::Boundary::Clear), the
/// span's ends are its whole incidence. An edge of `y` that bounds that
/// face would have read `Along`; met here, the two readings disagree.
///
/// # Errors
///
/// [`BooleanError::Escalated`] where an end's coincidence with a vertex
/// falls in the band, [`BooleanError::CurvedPierceEscalated`] at the
/// edge `edge_key` of `x_is` where the meeting reading does;
/// [`BooleanError::ClassificationInvariant`] where the meeting reading
/// faults, a vertex's edges do not resolve, or the edge found bounds
/// `face`.
fn twin_edge<T: Decide>(
    y: &Body<T>,
    (x_is, face, edge_key): (Operand, FaceKey, EdgeKey),
    swept: &geom_brep::curve_pair::Span<'_, T>,
    [(_, pu), (_, pv)]: [End<T>; 2],
    band: Band,
) -> Result<bool, BooleanError> {
    use geom_brep::curve_face::{BoundaryRefusal, shares_a_run};
    use geom_brep::curve_pair::{Span, meet};
    let vertex = |p| {
        super::vertex_at(y, p, band)
            .map(|v| v.map(|(v, _)| v))
            .map_err(|diag| BooleanError::Escalated {
                decision: BooleanDecision::VertexOnVertex,
                diag,
            })
    };
    let (Some(wu), Some(wv)) = (vertex(pu)?, vertex(pv)?) else {
        return Ok(false);
    };
    let lost = |what| BooleanError::ClassificationInvariant { what };
    let around = y
        .edges_of_vertex(wu)
        .ok_or_else(|| lost("spline face: a vertex's edges do not resolve"))?;
    for ek in around {
        let e = y
            .get_edge(ek)
            .ok_or_else(|| lost("spline face: a vertex's edge is lost"))?;
        let starts = [e.he_plus, e.he_minus].map(|he| y.get_half_edge(he).map(|h| h.start));
        if starts != [Some(wu), Some(wv)] && starts != [Some(wv), Some(wu)] {
            continue;
        }
        let Some(CurveGeom::Certified(curve)) = y.get_curve_geom(e.curve) else {
            continue;
        };
        let met = match meet(swept, &Span::new(curve.carrier(), curve.params()), band) {
            Ok(met) => met,
            Err(refusal) => match BoundaryRefusal::of(refusal.fault) {
                BoundaryRefusal::Unread(_) => continue,
                refused => {
                    super::carrier_cross::boundary_refusal(refused, (x_is, face, edge_key), false)?;
                    continue;
                }
            },
        };
        if !shares_a_run(&met) {
            continue;
        }
        if [e.he_plus, e.he_minus]
            .into_iter()
            .any(|he| y.face_of_half_edge(he) == Some(face))
        {
            return Err(lost(
                "spline face: an edge of the face's boundary the boundary pass read clear of it",
            ));
        }
        return Ok(true);
    }
    Ok(false)
}

/// **A vertex of an edge lying along `face`'s boundary**, placed on it by
/// the boundary pre-pass and recorded against the vertex of `y` there,
/// split out of a boundary edge it lands inside: an end of the edge
/// ([`spline_face`]), or a vertex a later split minted on it
/// ([`read_lying_in`]).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] where the pre-pass puts the
/// vertex off the boundary the pass put its edge along; the pre-pass's
/// own refusals, and the split's.
#[allow(clippy::too_many_arguments)]
fn along_boundary<T: Decide + crate::props::AtRestPolicy>(
    x_is: Operand,
    y: &mut Body<T>,
    (w, pw): End<T>,
    face: FaceKey,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let on = super::contain::curved_boundary_containment(y, face, pw, band)
        .map_err(|e| esc(e, x_is.other(), face))?;
    let (wy, margin) = match on {
        Some((FaceContainment::OnVertex(vy), margin)) => (vy, margin),
        Some((FaceContainment::OnEdge(ey), margin)) => (
            split_other_at_point(
                y,
                x_is.other(),
                ey,
                pw,
                super::Cell::Vertex(w),
                band,
                tol,
                contacts,
            )?,
            margin,
        ),
        Some((FaceContainment::In | FaceContainment::Out, _)) | None => return Err(disagree()),
    };
    push_vv(contacts, x_is, w, wy, Some(margin));
    Ok(())
}

/// What the declared arms know of an edge's interior against `face`.
enum Interior<T: geom_core::Real> {
    /// It meets `face`'s boundary there: a pierce landing on that
    /// boundary, split and recorded by the caller like any other.
    Crossing(CurvedEvent<T>),
    /// Certified: the interior meets `face`'s boundary nowhere, so the
    /// endpoints' placements are the pair's whole incidence.
    Clear,
    /// The edge has no certificate that it lies ON the carrier, and its
    /// interior meets `face`'s boundary nowhere the boundary pass read:
    /// what it meets of the face's interior is not known.
    Unseen,
}

impl<T: geom_core::Real> Interior<T> {
    /// Whether [`Placement::declared`] may read an all-`Elsewhere` pair
    /// as lying outside the face.
    fn clear(&self) -> bool {
        matches!(self, Self::Clear)
    }
}

/// **The interior question**: where the edge meets `face`'s boundary
/// strictly inside its span ([`super::carrier_cross`]), asked of an edge
/// the carrier-identity rung puts ON `face`'s carrier (`on_carrier`); a
/// pair whose meetings that module cannot read keeps the frontier door.
/// An edge off the carrier, its parent covered against `face`, is
/// [`Interior::Unseen`], save where it touches `face`'s boundary at a
/// point the two tangent sheets only meet at ([`sheets_part`]): a rim
/// meeting the face's own rim on the ruling the two walls share, tangent
/// to it in one plane or crossing it in two, the walls leaving the rims
/// to either side there. There both edges are split, as a vertex there
/// would split them.
#[allow(clippy::too_many_arguments)]
fn interior<T: Decide>(
    on_carrier: bool,
    (x, edge_key, edge): (&Body<T>, EdgeKey, &crate::entity::Edge),
    y: &Body<T>,
    x_is: Operand,
    face: FaceKey,
    curve: &geom_brep::EdgeCurve<T>,
    declared: &super::GluedPairs<T>,
    band: Band,
    (frontier, located): (
        impl Fn() -> BooleanError,
        &impl Fn(BooleanDecision, geom_core::Indeterminate) -> BooleanError,
    ),
) -> Result<Interior<T>, BooleanError> {
    use super::carrier_cross::{BoundaryCrossing, boundary_crossing};
    let pierce = |t, p, at, margin| {
        Interior::Crossing(CurvedEvent::Pierce {
            t,
            p,
            at,
            on: Some(margin),
            branch: None,
            chart: None,
        })
    };
    let read = boundary_crossing(
        y,
        (x_is.other(), edge_key, on_carrier),
        face,
        curve.carrier(),
        curve.params(),
        band,
    )?;
    match (read, on_carrier) {
        (BoundaryCrossing::At { t, p, at, margin }, true) => Ok(pierce(t, p, at, margin)),
        (BoundaryCrossing::At { t, p, at, margin }, false) => Ok(
            if sheets_part(
                (x, x_is, edge),
                (y, face, at),
                declared,
                curve,
                (t, p),
                band,
            )? {
                pierce(t, p, at, margin)
            } else {
                Interior::Unseen
            },
        ),
        (_, false) => Ok(Interior::Unseen),
        (BoundaryCrossing::Clear, true) => Ok(Interior::Clear),
        (BoundaryCrossing::Unread, true) => Err(frontier()),
        (BoundaryCrossing::Touch(diag), true) => Err(located(BooleanDecision::CurveMeeting, diag)),
    }
}

/// **Whether a covered edge's parent and `face` part at `p`**, where the
/// edge meets `face`'s boundary edge `at`: the parent's direction into
/// itself from the edge and `face`'s from its own boundary edge there
/// ([`crate::stands::inward`], about each face's outward normal at `p`)
/// are definitely opposed (`bool_touch_sheets_part`, the cosine levered
/// at the longer of the two faces' reaches from their edges,
/// [`crate::wedge_reach()`]). The two
/// faces are tangent sheets (the cover), so each leaves `p` along their
/// common tangent plane, across the edges there; opposed, they meet
/// nowhere else near `p`, and the meeting is a touch of the two
/// boundaries alone. Alike, the sheets run on together past `p` through
/// both faces' interiors, which the sweep does not read from a split
/// there (`face_face_touch.rs`'s nested disc), and so are directions at
/// right angles (a boundary edge running across the edge, such as a
/// seam ruling).
///
/// The boundary edge's direction at `p` is its carrier's derivative at
/// the parameter read back from `p` ([`point_param`]), as the covered
/// edge's is at `t`. A spline boundary edge, which has no such
/// parameter, refuses there, before either face's normal is read.
///
/// `false`, with nothing read, where there is nothing to read: a
/// landing on a vertex, a boundary edge both of whose halves bound
/// `face`, no single covered parent, or a normal the door refuses
/// outright (off the surface, a cone's apex, a degenerate torus).
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] where an entity the sweep holds does not
/// resolve; [`BooleanError::PointSplitCarrierUnsupported`] where the
/// boundary edge's carrier is a spline (the split a parting would make
/// there needs the same parameter);
/// [`BooleanError::Escalated`] where a normal is in band; the
/// [`Coincide::Sectors`] coincidence where the cosine is.
fn sheets_part<T: Decide>(
    (x, x_is, edge): (&Body<T>, Operand, &crate::entity::Edge),
    (y, face, at): (&Body<T>, FaceKey, FaceContainment),
    declared: &super::GluedPairs<T>,
    curve: &geom_brep::EdgeCurve<T>,
    (t, p): (T, Point3<T>),
    band: Band,
) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let FaceContainment::OnEdge(e_y) = at else {
        return Ok(false);
    };
    let mut covered = Vec::new();
    for h in [edge.he_plus, edge.he_minus] {
        let f = x
            .face_of_half_edge(h)
            .ok_or(desync("a covered edge's half has no face"))?;
        if declared.cover(x_is, f, x_is.other(), face).is_some() {
            covered.push((h, f));
        }
    }
    let [(h_x, f_x)] = covered[..] else {
        return Ok(false);
    };
    let d_y = y.get_edge(e_y).ok_or(desync(
        "a boundary edge a crossing lands on does not resolve",
    ))?;
    let mut halves = Vec::new();
    for h in [d_y.he_plus, d_y.he_minus] {
        if y.face_of_half_edge(h)
            .ok_or(desync("a boundary edge's half has no face"))?
            == face
        {
            halves.push(h);
        }
    }
    let [h_y] = halves[..] else {
        return Ok(false);
    };
    let c_y = y
        .edge_curve_linked(e_y, d_y)
        .certified()
        .ok_or(desync("a boundary edge a crossing lands on is scaffolding"))?;
    let along = |dir: Vec3<T>, forward: bool| if forward { dir } else { -dir };
    let tau_y = c_y
        .carrier()
        .deriv(point_param(y, x_is.other(), e_y, c_y, p)?);
    let normal = |body: &Body<T>, f: FaceKey| -> Result<Option<Vec3<T>>, BooleanError> {
        match crate::face_normal::face_outward_normal_at(body, f, p, band) {
            Ok(Some(n)) => Ok(Some(n.vec())),
            Ok(None) => Err(desync("a face a crossing reads does not resolve")),
            Err(crate::face_normal::NormalAtError::Escalated { decision, diag }) => {
                Err(BooleanError::Escalated {
                    decision: BooleanDecision::of_normal(decision),
                    diag,
                })
            }
            Err(_) => Ok(None),
        }
    };
    let (Some(n_x), Some(n_y)) = (normal(x, f_x)?, normal(y, face)?) else {
        return Ok(false);
    };
    let into_x = crate::stands::inward(
        n_x,
        along(curve.carrier().deriv(t), h_x == edge.he_plus).normalize(),
    );
    let into_y = crate::stands::inward(n_y, along(tau_y, h_y == d_y.he_plus).normalize());
    let arm = crate::wedge_reach(x, [f_x, f_x], curve.carrier(), curve.params()).max(
        crate::wedge_reach(y, [face, face], c_y.carrier(), c_y.params()),
    );
    let sign = decide(
        "bool_touch_sheets_part",
        Margin::levered(into_x.dot(into_y), arm),
        band,
    )
    .map_err(|diag| BooleanError::coincidence(Coincide::Sectors, DeclarationRead::Moot, diag))?;
    Ok(sign == Sign::Negative)
}

/// An arc of `x` lying on `face`'s carrier, as [`lying_on`] reads it.
struct ArcOnCarrier<'a, T: geom_core::Real> {
    x: &'a Body<T>,
    x_is: Operand,
    edge_key: EdgeKey,
    /// The arc's two ends, `he_plus`'s start first, each with the
    /// margin that decided it on the carrier where one did.
    ends: [(VertexKey, Point3<T>, Option<MarginDiag>); 2],
    face: FaceKey,
    /// The coincidence the caller refuses an end's in-band side as.
    side: Coincide,
    band: Band,
    tol: Tol,
}

/// **An arc, an ellipse or a ruling lying on `face`'s carrier, its
/// parents distinct from it:** the endpoint records, once its interior
/// is certified to cross `face`'s boundary nowhere, or a split where it
/// crosses it. Three readings for an arc, in order, and the last alone
/// for an ellipse or a ruling; anything else keeps the door (`None`):
///
/// - **off the boundary but at its ends**: the face's boundary meets the
///   arc's own circle nowhere but at the vertices of `y` the arc's ends
///   were paired with ([`boundary_meets_circle_only_at`]), so the arc's
///   interior meets the boundary nowhere and lies wholly inside the face
///   or wholly outside it. With no end on the
///   boundary the ends must agree: both placed outside is no event, both
///   recorded is the endpoint posture; with an end on it, its record is.
/// - **along edges of the partner**: both ends were paired with
///   vertices of `y`, and `y` has a chain of circle arcs between them,
///   each leaving along this arc's tangent where it starts
///   ([`super::arcs::arcs_along`]) and arriving at a point decided on
///   this arc's circle. A circle through two points with a given tangent
///   at one is unique, so the chain IS this arc, and edges of a valid
///   body cross no face's interior: the end records, with the chain's inner
///   vertices that the other direction's sweep records on this arc, are
///   every incidence it has.
/// - **the interior question**: where the span meets the face's
///   boundary strictly inside it ([`super::carrier_cross`]). A meeting
///   is a `Pierce` the caller splits both edges at, as at a wall pierce
///   landing on the boundary, and each fragment is read again here; a
///   certified absence is (a)'s conclusion. A seam ruling a rim crosses
///   mid-arc is such a meeting: (a) reads the whole circle, which the
///   ruling meets wherever the rim is turned, and (b) has no chain.
///
/// Where the interior question reads the boundary, (a) is a fast path:
/// what it certifies the interior question would too. It still decides alone
/// where a boundary ellipse lies in the arc's own plane, which (a) reads
/// through its plane crossings and the interior question leaves unread.
///
/// The interior question's candidates overlap. On a surface of
/// revolution a boundary ruling's end vertex lies on the arc at the
/// ruling's crossing, so the ruling's meeting finds nothing the vertices
/// do not. And where both of the arc's ends lie in this face, a circle
/// boundary's crossing missed here is split by the other sweep
/// direction, when that boundary edge pierces the arc's parent face at
/// it. Only a circle boundary's meeting with an arc that runs from this
/// face into another is pinned alone
/// (`sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
/// `a_rod_rim_on_the_dome_across_its_seam_meridian_builds_every_op_undeclared`).
///
/// The ends are placed, and recorded, before any certificate runs:
/// certificate (a) reads the vertices of `y` the placements pair them
/// with, minting one where an end lands on an edge. That is sound
/// because every answer but `Recorded` and `Pierce` either follows two
/// `Elsewhere` placements, which record nothing, or is `None`, which the
/// caller turns into the frontier that ends the op, so no record or
/// split made here outlives a certificate that did not hold. A `Pierce`
/// keeps the records: its fragments carry the same ends, and placing
/// them again records nothing new ([`ContactAcc`] keeps each record
/// once, by key).
fn lying_on<T: Decide + crate::props::AtRestPolicy>(
    arc: &ArcOnCarrier<'_, T>,
    y: &mut Body<T>,
    contacts: &mut ContactAcc,
) -> Result<Option<CurvedEvent<T>>, BooleanError> {
    let ArcOnCarrier {
        x,
        x_is,
        edge_key,
        ends,
        face,
        side,
        band,
        tol,
    } = *arc;
    let lost = |what| BooleanError::ClassificationInvariant { what };
    let e = x
        .get_edge(edge_key)
        .ok_or_else(|| lost("an arc on a carrier: the edge is lost"))?;
    let curve = x
        .get_curve_geom(e.curve)
        .and_then(CurveGeom::certified)
        .ok_or_else(|| lost("an arc on a carrier: the edge's curve is lost"))?;
    // A ruling or an ellipse has no circle for (a) and (b) to read: the
    // interior question alone decides it.
    let circle = match *curve.carrier() {
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => Some((center, axis, radius)),
        geom::Curve3::Line { .. } | geom::Curve3::Ellipse { .. } => None,
        _ => return Ok(None),
    };
    let mut placed = [(Placement::Undecided, None); 2];
    for (slot, (w, pw, on)) in placed.iter_mut().zip(ends) {
        *slot = vertex_on_curved_face_at(x_is, y, w, pw, face, on, contacts, band, tol)?;
    }
    if placed.iter().any(|(p, _)| *p == Placement::Undecided) {
        return Ok(None);
    }
    let at_ends: Vec<VertexKey> = placed.iter().filter_map(|(_, w)| *w).collect();
    // The ends' placements, once the arc's interior is certified to meet
    // the boundary nowhere: it lies wholly inside the face or wholly
    // outside it.
    let lies = |ends, recorded| CurvedEvent::Lies {
        recorded,
        on: OnCurved {
            rule: MintedRule::LiesOn(ends),
            side,
        },
    };
    let interior_clear = || {
        let all = |p: Placement| placed.iter().all(|(q, _)| *q == p);
        if all(Placement::Elsewhere) {
            return Some(lies(LyingEnds::Outside, false));
        }
        // With no end paired with a vertex of `y`, one end in and one
        // out would need the arc to cross a boundary the certificate has
        // just kept off its interior: only two certified answers
        // contradicting reach it, and that keeps the door. An end paired
        // with a vertex is outside that argument (the face-free vertex
        // search can pair one off this face's boundary), and the record
        // it made is its answer.
        if !at_ends.is_empty() {
            Some(lies(LyingEnds::Paired, true))
        } else {
            all(Placement::Recorded).then_some(lies(LyingEnds::Inside, true))
        }
    };
    if let Some(circle) = circle {
        if boundary_meets_circle_only_at(y, face, circle, &at_ends, band)? {
            return Ok(interior_clear());
        }
        if let [(_, Some(wu)), (_, Some(wv))] = placed {
            let (dir, _) = curve.walk_tangents(
                x.get_half_edge(e.he_plus)
                    .ok_or_else(|| lost("an arc on a carrier: its half is lost"))?
                    .start
                    == ends[0].0,
            );
            if arc_chain_reaches(y, wu, wv, dir, circle, band)? {
                return Ok(Some(lies(LyingEnds::Paired, true)));
            }
        }
    }
    use super::carrier_cross::{BoundaryCrossing, boundary_crossing};
    Ok(
        match boundary_crossing(
            y,
            (x_is.other(), edge_key, true),
            face,
            curve.carrier(),
            curve.params(),
            band,
        )? {
            BoundaryCrossing::At { t, p, at, margin } => Some(CurvedEvent::Pierce {
                t,
                p,
                at,
                on: Some(margin),
                branch: None,
                chart: None,
            }),
            BoundaryCrossing::Clear => interior_clear(),
            BoundaryCrossing::Unread => None,
            // Where the caller's frontier door stands for `None`.
            BoundaryCrossing::Touch(diag) => {
                return Err(BooleanError::CurvedPierceEscalated {
                    operand: x_is,
                    face,
                    edge: edge_key,
                    decision: BooleanDecision::CurveMeeting,
                    diag,
                });
            }
        },
    )
}

/// Whether `y` has a chain of circle arcs from `from` to `to`, the
/// first leaving `from` along `dir` and each next one along the last
/// one's arrival tangent ([`super::arcs::arcs_along`]), every vertex it
/// passes on the way decided on the circle (`center`, unit `axis`,
/// `radius`). `to` itself is not decided: the caller paired it with an
/// end of an arc on that circle.
fn arc_chain_reaches<T: Decide>(
    y: &Body<T>,
    from: VertexKey,
    to: VertexKey,
    mut dir: geom_core::Vec3<T>,
    (center, axis, radius): (Point3<T>, geom_core::Vec3<T>, T),
    band: Band,
) -> Result<bool, BooleanError> {
    let escalated =
        |diag| BooleanError::coincidence(Coincide::EdgeOnCurvedFace, DeclarationRead::Moot, diag);
    // A chain visits each edge of `y` at most once.
    let mut at = from;
    for _ in 0..y.edges().count() {
        let steps = super::arcs::arcs_along(y, at, dir, band).map_err(escalated)?;
        let [step] = steps[..] else {
            return Ok(false);
        };
        if step.to == to {
            return Ok(true);
        }
        let p = y
            .get_vertex(step.to)
            .and_then(|vd| y.get_point(vd.point))
            .copied()
            .ok_or(BooleanError::ClassificationInvariant {
                what: "an arc on a carrier: a chain vertex has no point",
            })?;
        let miss = crate::splitting::containment::circle_miss(p, center, axis, radius);
        match decide("bool_arc_chain_on_circle", Margin::of(miss), band).map_err(escalated)? {
            Sign::Zero => {}
            Sign::Positive | Sign::Negative => return Ok(false),
        }
        at = step.to;
        dir = step.arrival;
    }
    Ok(false)
}

/// Where a boundary vertex sits against the arc's plane, as
/// [`boundary_meets_circle_only_at`] reads it.
#[derive(Clone, Copy, PartialEq)]
enum PlaneSide {
    /// Decided strictly on this side.
    Off(Sign),
    /// One of the vertices the arc's ends were paired with.
    At,
    /// Decided in the plane, and decided off the circle.
    InPlaneOffCircle,
}

/// Whether `face`'s boundary meets the circle (`center`, unit `axis`,
/// `radius`) nowhere but at the vertices `at`. The circle lies in the
/// plane through `center` normal to `axis`, so the boundary is read
/// against that plane, and each point where it meets the plane must be
/// decided off the circle or be one of `at`:
///
/// - a vertex: decided strictly off the plane, or decided in it and off
///   the circle. One whose side escalates is placed nowhere, and fails;
/// - a line: no crossing between two ends on one side; its one crossing
///   between the two sides decided off the circle; an end in the plane
///   with the other strictly off. A line lying in the plane is a chord
///   only between two of `at`, and is certified when its midpoint is
///   decided off the circle;
/// - a conic: its crossings strictly inside its span (the splitting
///   lane's certified roots,
///   [`crate::splitting::plane_crossing_lane`]) each decided off
///   the circle, or a plane of its own decided parallel and off.
///
/// Everything else answers `false`: an undecided point, a conic lying
/// in the plane, and any boundary edge the certificate cannot place (a
/// NURBS or spiric carrier, or a curve that is not certified).
///
/// `face` is a key the caller carries: its miss refuses
/// [`BooleanError::ClassificationInvariant`].
///
/// # Panics
///
/// Where a record past `face` does not resolve or a loop walk does not
/// close (D2 row 4): a torn curve is not an uncertified one. `y` is a
/// working copy an end placement may just have split an edge of, whose
/// links hold by [`crate::live::OPERATORS_KEEP_LINKS`].
fn boundary_meets_circle_only_at<T: Decide>(
    y: &Body<T>,
    face: FaceKey,
    (center, axis, radius): (Point3<T>, geom_core::Vec3<T>, T),
    at: &[VertexKey],
    band: Band,
) -> Result<bool, BooleanError> {
    let lost = || BooleanError::ClassificationInvariant {
        what: "an arc on a carrier: the face's boundary is not walkable",
    };
    let f = y.get_face(face).ok_or_else(lost)?;
    let height = |p: Point3<T>| (p - center).dot(axis);
    let off_circle = |p: Point3<T>| {
        matches!(
            decide(
                "bool_arc_boundary_off_circle",
                Margin::of(crate::splitting::containment::circle_miss(
                    p, center, axis, radius
                )),
                band
            ),
            Ok(Sign::Positive)
        )
    };
    // Every vertex placed is a link of a record the walk resolved.
    let point = |v: VertexKey| y.resolve_vertex_point(v, crate::live::Proven);
    let place = |v: VertexKey| -> Option<PlaneSide> {
        if at.contains(&v) {
            return Some(PlaneSide::At);
        }
        let p = point(v);
        match decide("bool_arc_plane_side", Margin::of(height(p)), band) {
            Ok(s @ (Sign::Positive | Sign::Negative)) => Some(PlaneSide::Off(s)),
            Ok(Sign::Zero) => off_circle(p).then_some(PlaneSide::InPlaneOffCircle),
            Err(_) => None,
        }
    };
    for member in y.face_boundary_linked(face, f) {
        match member {
            crate::live::BoundaryMember::Isolated { vertex, .. } => {
                if place(vertex).is_none() {
                    return Ok(false);
                }
            }
            crate::live::BoundaryMember::Edge { ek, edge: e, .. } => {
                let end = |h, field| {
                    crate::live::linked(
                        &y.half_edges,
                        h,
                        EntityId::HalfEdge,
                        EntityId::Edge(ek),
                        field,
                    )
                    .start
                };
                let (a, b) = (end(e.he_plus, "he_plus"), end(e.he_minus, "he_minus"));
                let (Some(sa), Some(sb)) = (place(a), place(b)) else {
                    return Ok(false);
                };
                let Some(c) = y.edge_curve_linked(ek, e).certified() else {
                    return Ok(false);
                };
                let (t0, t1) = c.params();
                let clear = match crate::splitting::plane_crossing_lane(
                    c.carrier(),
                    t0,
                    t1,
                    center,
                    axis,
                    band,
                ) {
                    PlaneCrossingLane::Line => {
                        let (pa, pb) = (point(a), point(b));
                        match (sa, sb) {
                            (PlaneSide::Off(s), PlaneSide::Off(t)) if s != t => {
                                let (ha, hb) = (height(pa), height(pb));
                                off_circle(pa + (pb - pa) * (ha / (ha - hb)))
                            }
                            (PlaneSide::Off(_), _) | (_, PlaneSide::Off(_)) => true,
                            (PlaneSide::At, PlaneSide::At) => {
                                off_circle(pa.lerp(pb, T::from_f64(0.5)))
                            }
                            _ => false,
                        }
                    }
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Miss) => true,
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(roots))) => {
                        roots.iter().all(|&t| off_circle(c.carrier().eval(t)))
                    }
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { offset }) => matches!(
                        decide("bool_arc_plane_side", Margin::of(offset), band),
                        Ok(Sign::Positive | Sign::Negative)
                    ),
                    PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(_)))
                    | PlaneCrossingLane::Unlaned => false,
                };
                if !clear {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}

/// Whether the carrier ladder decides EVERY surface of a face the edge
/// bounds definitely distinct from `face`'s carrier, read undeclared.
/// One carrier, a refusal (an offset in band) or a kind outside the
/// ladder's inventory answers false.
fn parents_distinct_from<T: Decide>(
    x: &Body<T>,
    edge: &crate::entity::Edge,
    y: &Body<T>,
    face: FaceKey,
    band: Band,
) -> bool {
    [
        x.face_of_half_edge(edge.he_plus),
        x.face_of_half_edge(edge.he_minus),
    ]
    .into_iter()
    .all(|pf| {
        pf.is_some_and(|pf| {
            matches!(
                super::carrier_pair::carrier_pair_relation(x, pf, y, face, band),
                Ok(Ok(super::carrier_eq::CarrierRelation::Distinct))
            )
        })
    })
}

/// A row's `escalate` handler for [`wall_crossing`]: the door's
/// escalation, unlocated.
#[cfg(test)]
fn door_escalates<T: geom_core::Real>(
    decision: BooleanDecision,
    diag: geom_core::Indeterminate,
) -> Result<SpanVerdict<T>, BooleanError> {
    Err(BooleanError::Escalated { decision, diag })
}

/// What the certified carrier × wall roots say about ONE edge span.
#[derive(Debug, Clone, Copy)]
pub(super) enum SpanVerdict<T: geom_core::Real> {
    /// A definite crossing strictly inside the span, which the trim
    /// placed at `at` in this face.
    Pierce {
        t: T,
        p: Point3<T>,
        at: FaceContainment,
        /// The margin that decided a boundary landing.
        on: Option<MarginDiag>,
    },
    /// A certified root set (two for a line, or a circle on a sphere;
    /// two or four for a circle on a wall or a torus, a conic on a cone,
    /// or a line on a torus), none of them STRICTLY INSIDE the span on
    /// this face: each lies outside the span, sits at one of its ends, or
    /// lands on the carrier outside the face's trim or on the cone's
    /// other nappe. Distinct certified roots also
    /// certify that the edge does not LIE on the carrier (a line: not a
    /// ruling of a wall or a cone, and no line lies on a sphere or a
    /// torus; a
    /// circle on a sphere or a wall answers [`Self::LiesOn`], and on a torus a
    /// certified count needs a pole definitely off the torus, which a
    /// circle lying on it has nowhere), which is what separates a chord
    /// from an on-carrier edge. What the
    /// absence of an interior crossing licenses depends on the
    /// endpoints, so the caller decides — an endpoint incidence is
    /// still an event, it is just not an interior one.
    NoInterior,
    /// The [`Self::NoInterior`] case with its crossing ACCOUNTED FOR: at
    /// least one root lies strictly inside the span, every such root
    /// was placed on the carrier and definitely OUTSIDE this face's
    /// trim, and no root sits at either end. It is `NoInterior` in every
    /// respect a caller that accepts `NoInterior` reads; what it adds is
    /// for the straddling span, whose endpoint signs PROMISE a crossing
    /// — here the promise is kept, on the carrier, off this face. A
    /// `NoInterior` with no such root cannot keep it, and that
    /// contradiction still keeps the door.
    Elsewhere,
    /// The line is parallel to the axis, so its residual is CONSTANT
    /// along the span. Nothing about the span's interior differs from
    /// its endpoints — which makes it a clearance answer for a caller
    /// whose endpoints are definitely off the wall, and a ruling lying
    /// on the wall for one whose endpoints are on it.
    Constant,
    /// The conic LIES on the surface: its residual is a zero constant,
    /// decided exactly on by its root door
    /// ([`Roots::On`]; in band it
    /// escalates there). Unlike [`Self::Constant`] this is a verdict
    /// on the side as well as the shape.
    LiesOn,
    /// The line definitely misses the wall entirely.
    Miss,
    /// The roots did not settle, or read in band, and every touch of the
    /// span with the carrier is certified off this face
    /// ([`super::carrier_touch::off_face`]): the span meets the carrier,
    /// if at all, outside the trim. Like [`Self::Miss`] it accounts for no
    /// crossing of THIS face; unlike it, an endpoint the band puts ON the
    /// carrier is no contradiction.
    OffFace,
    /// The root door reached a limit ([`Unsettled`]), or has no lane for
    /// the pair, and the caller keeps its own typed frontier door.
    Unsettled,
    /// A root within the band of a cone's apex
    /// ([`Unsettled::AtApex`]): the caller refuses
    /// [`BooleanError::CrossingAtConeApex`].
    AtApex,
}

/// **A spline span against a plane face**: the crossings strictly inside
/// the span, ascending, by the curve × face table's ring-composite arm
/// ([`crate::props::AtRestPolicy::spline_face_crossings`]), read through the
/// scalar's certified lane. A spiric span, or a scalar with no certified
/// lane, has no arm.
///
/// # Errors
///
/// [`BooleanError::EdgeCarrierUnsupported`] where there is no arm, and,
/// where the span's meetings with the plane are not certified off the face
/// ([`super::carrier_touch::off_face`]),
/// [`BooleanError::SplineCrossingUnsettled`] where the arm cannot count
/// the crossings and [`BooleanError::Escalated`] where it reads a touch in
/// the band's gap or cannot place a crossing.
fn spline_plane_crossings<T: Bounds + crate::props::AtRestPolicy>(
    (x, x_is, edge_key): (&Body<T>, Operand, EdgeKey),
    curve: &geom_brep::EdgeCurve<T>,
    (y, face): (&Body<T>, FaceKey),
    band: Band,
) -> Result<Vec<T>, BooleanError> {
    use geom_brep::curve_face::composite::CompositeRefusal;
    use geom_brep::curve_face::{Door, route};
    let root = || {
        x.split_root(edge_key, |_| false)
            .map_err(BooleanError::SplitLineage)
    };
    let surface = y
        .get_face(face)
        .and_then(|f| y.get_surface(f.surface))
        .ok_or(BooleanError::ClassificationInvariant {
            what: "planar lane: the face's plane does not resolve",
        })?;
    let arm = route(curve.carrier().kind(), surface.kind());
    let crossings = match curve.carrier() {
        geom::Curve3::Nurbs(net) if arm.door == Door::SplineComposite && arm.implemented => {
            T::spline_face_crossings(net, curve.params(), surface, band)
        }
        _ => None,
    };
    let Some(crossings) = crossings else {
        return Err(BooleanError::EdgeCarrierUnsupported {
            operand: x_is,
            edge: root()?,
            face,
            face_of: x_is.other(),
            site: super::EdgeCarrierSite::PlanarCrossing,
        });
    };
    // Where the span may meet the plane is localized and read against the
    // face: certified off it, a touch the arm could not settle or decide is
    // no crossing of this face.
    let off_face =
        || super::carrier_touch::off_face(y, face, surface, curve.carrier(), curve.params(), band);
    match crossings {
        Ok(ts) => Ok(ts),
        Err(CompositeRefusal::Escalated(_) | CompositeRefusal::Unsettled(_)) if off_face()? => {
            Ok(Vec::new())
        }
        Err(CompositeRefusal::Escalated(diag)) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
            diag,
        }),
        Err(CompositeRefusal::Unsettled(why)) => Err(BooleanError::SplineCrossingUnsettled {
            operand: x_is,
            edge: root()?,
            face,
            face_of: x_is.other(),
            why,
        }),
        Err(CompositeRefusal::Unread(_)) => Err(BooleanError::ClassificationInvariant {
            what: "planar lane: a built spline carrier's composite against a plane \
                   did not form",
        }),
    }
}

/// **A spline span against a cylinder, cone, sphere or torus face**: the
/// curve × face table's ring-composite arm
/// ([`crate::props::AtRestPolicy::spline_face_crossings`]) counts and
/// locates every crossing strictly inside the span, each certified ON the
/// carrier and apart from the others and from the span's ends; a touch, a
/// run along the carrier or a meeting at a crease it refuses
/// (`Unsettled`). So, unlike a line's or a conic's arms, nothing here
/// rests on the residual's shape along the span.
///
/// - **Settled**: each crossing is told off a cone's far nappe
///   ([`cone_nappe`]) and placed against the trim; the first one the face
///   holds is the pierce, and one outside it is a sibling face's. With no
///   pierce, the span meets this face at most at its ends, each ON end
///   placed by [`vertex_on_curved_face`] under the undeclared rule
///   ([`Placement::undeclared_no_interior`]). Ends definitely off on
///   opposite sides with an even count, or on one side with an odd one,
///   contradict the count, and keep the door.
/// - **A touch the arm cannot settle, or reads in the band's gap**: where
///   the span may meet the carrier is localized and read against the face
///   ([`super::carrier_touch::off_face`]); certified off it, the span's
///   interior meets this face nowhere, and the ends decide as above.
///   Otherwise the arm's refusal stands
///   ([`BooleanError::SplineCrossingUnsettled`], a span lying on the
///   carrier among them, or [`BooleanError::Escalated`]), save under a
///   cover with no end on the carrier.
/// - **The one-sided cover** ([`edge_covers`]): the span lies in one
///   closed side of the carrier, so a crossing contradicts it, as does an
///   end off on the side no cover admits; both keep the door. Its ends
///   take the declared rule ([`Placement::declared`]), the interior clear
///   where the count settled or the meetings were placed off the face. A
///   span whose meetings the arm cannot settle, with no end on the
///   carrier, touches the carrier, if at all, strictly inside, and is
///   deferred to its fragments ([`CurvedEvent::Deferred`]).
///
/// # Errors
///
/// [`BooleanError::EdgeCarrierUnsupported`] for a scalar with no
/// certified lane, [`BooleanError::SplineCrossingUnsettled`] and
/// [`BooleanError::CurvedPierceUnsupported`] as above,
/// [`BooleanError::Escalated`] where a decision falls in the band's gap.
#[allow(clippy::too_many_arguments)]
fn spline_wall_event<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (x, x_is, edge_key, edge): (&Body<T>, Operand, EdgeKey, &crate::entity::Edge),
    y: &mut Body<T>,
    (carrier, net, (t0, t1), surface): (
        &geom::Curve3<T>,
        &geom::NurbsCurve3<T>,
        (T, T),
        &geom::Surface<T>,
    ),
    [(u, pu), (v, pv)]: [End<T>; 2],
    face: FaceKey,
    (covers, declared): (&[super::CoverSide], &super::GluedPairs<T>),
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<CurvedEvent<T>, BooleanError> {
    use geom_brep::curve_face::composite::CompositeRefusal;
    let frontier = || BooleanError::CurvedPierceUnsupported {
        operand: x_is,
        face,
        edge: edge_key,
        band,
    };
    let root = || {
        x.split_root(edge_key, |_| false)
            .map_err(BooleanError::SplitLineage)
    };
    let covered = !covers.is_empty();
    let Some(found) = T::spline_face_crossings(net, (t0, t1), surface, band) else {
        return Err(BooleanError::EdgeCarrierUnsupported {
            operand: x_is,
            edge: root()?,
            face,
            face_of: x_is.other(),
            site: super::EdgeCarrierSite::CurvedCrossing,
        });
    };
    // The crossings (`None` where the span's meetings were placed off the
    // face uncounted), or the arm's refusal to count them: a touch it
    // could not settle, or one it read in the band's gap.
    let crossings = match found {
        Ok(ts) => Ok(Some(ts)),
        Err(CompositeRefusal::Escalated(_) | CompositeRefusal::Unsettled(_))
            if super::carrier_touch::off_face(y, face, surface, carrier, (t0, t1), band)? =>
        {
            Ok(None)
        }
        Err(CompositeRefusal::Escalated(diag)) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
            diag,
        }),
        Err(CompositeRefusal::Unsettled(why)) => Err(BooleanError::SplineCrossingUnsettled {
            operand: x_is,
            edge: root()?,
            face,
            face_of: x_is.other(),
            why,
        }),
        Err(CompositeRefusal::Unread(_)) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "curved lane: a built spline carrier's composite against an analytic \
                       face did not form",
            });
        }
    };
    if covered
        && crossings
            .as_ref()
            .is_ok_and(|ts| ts.as_ref().is_some_and(|ts| !ts.is_empty()))
    {
        return Err(frontier());
    }
    if let Ok(Some(ts)) = &crossings {
        let nappe = cone_nappe(y, face, surface, band)?;
        for &t in ts {
            let p = carrier.eval(t);
            if let Some((apex, axis, half_angle, nappe)) = nappe {
                let off = geom_brep::cone_elevation(apex, axis, half_angle, Some(nappe), p);
                match decide("bool_cone_root_nappe", Margin::of(off), band) {
                    Ok(Sign::Positive) => continue,
                    Ok(Sign::Zero) => {}
                    // Below the face's own nappe is off the double cone
                    // the crossing was certified on.
                    Ok(Sign::Negative) | Err(_) => return Err(frontier()),
                }
            }
            match landing(y, face, p, band)? {
                Landing::Unplaced => return Err(frontier()),
                Landing::Out => {}
                Landing::At(at, on) => {
                    return Ok(CurvedEvent::Pierce {
                        t,
                        p,
                        at,
                        on,
                        branch: None,
                        chart: None,
                    });
                }
            }
        }
    }
    let which = if covered {
        Coincide::VertexOnCoveredFace
    } else {
        Coincide::VertexOnCurvedFace
    };
    let mut ends = [None, None];
    let mut sides = [Sign::Zero; 2];
    for (i, (w, pw)) in [(u, pu), (v, pv)].into_iter().enumerate() {
        let decided = geom_core::k_stats::decide_reported(
            "bool_vertex_face_side",
            Margin::of(geom_brep::implicit_residual(surface, pw)),
            band,
        )
        .map_err(|diag| {
            let read = edge_face_read(x, x_is, edge, face, declared, which);
            BooleanError::coincidence(which, read, diag)
        })?;
        sides[i] = decided.sign;
        match decided.sign {
            Sign::Zero => {}
            sign if covers.iter().any(|c| c.admits(sign)) || !covered => continue,
            Sign::Positive | Sign::Negative => return Err(frontier()),
        }
        if crossings.is_err() {
            continue;
        }
        ends[i] = Some(vertex_on_curved_face(
            x_is,
            y,
            w,
            pw,
            face,
            decided.margin,
            contacts,
            band,
            tol,
        )?);
    }
    let on_carrier = sides.contains(&Sign::Zero);
    let counted = match crossings {
        Ok(counted) => counted,
        Err(_) if covered && !on_carrier => return Ok(CurvedEvent::Deferred),
        Err(refusal) => return Err(refusal),
    };
    if covered {
        return Placement::declared(ends, true).ok_or_else(frontier);
    }
    if let Some(ts) = counted
        && !on_carrier
        && (sides[0] != sides[1]) != (ts.len() % 2 == 1)
    {
        return Err(frontier());
    }
    Placement::undeclared_no_interior(ends).ok_or_else(frontier)
}

/// The curved-wall crossing route: solve the certified roots — a
/// line's quadratic on a cylinder wall, a sphere or a cone, its quartic
/// on a torus, a circle's closed form on a sphere and its half-angle
/// quartic on a torus or a cylinder wall, an ellipse's degree-2 residual
/// on a sphere or a cylinder wall, a conic's degree-2 quadric form on a
/// cone, and an ellipse's degree-4 residual on a torus — keep the roots
/// the EDGE's span carries strictly inside, tell off a cone's roots on
/// the nappe its face does not lie on, and place the landing point in
/// the face's trim.
///
/// **Roots at the span's ends are deliberately NOT interior.** A root
/// the band cannot separate from an endpoint is that endpoint's own
/// incidence, and the endpoint rows own those (they mint a v-f or v-v
/// record rather than a split, which is what a split at `t₀` could not
/// do anyway — `split_edge`'s interiority trilean refuses it). Folding
/// the two together here would have made an endpoint touch look like a
/// crossing to every caller.
///
/// **Every root is examined, and the FIRST interior one wins.** A
/// segment through a wall meets it twice (a torus up to four times);
/// the sweep splits at one root and re-queues both fragments against
/// the SAME face, so the rest are found on later passes — the shape the
/// conic × plane lane already uses, and the reason this function does
/// not return a set.
#[allow(clippy::too_many_arguments)]
fn wall_crossing<T: Decide + Bounds>(
    y: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    band: Band,
    escalate: &impl Fn(
        BooleanDecision,
        geom_core::Indeterminate,
    ) -> Result<SpanVerdict<T>, BooleanError>,
) -> Result<SpanVerdict<T>, BooleanError> {
    // A root's distance from an end of the span is metered as a length:
    // the parameter gap times the carrier's speed AT that end, so that
    // `Zero` means the root sits at the end's own point. A `Line`'s
    // parameter runs `|dir|` metres per unit, a `Circle`'s `radius`, an
    // `Ellipse`'s `|C′(t)|`, which varies over `[b, a]` (`geom_brep::Conic`).
    // The wall and sphere quadratics take any non-zero `dir`; the torus
    // quartic ([`super::solid_contain::line_torus_roots`]) assumes it UNIT,
    // the `Line` carrier's convention, which nothing checks.
    // `None`: a kind pair with no root lane (a spiric or a spline
    // against any carrier), which takes the refusals' path below, as a
    // door's refusal does.
    // Where the span meets the carrier, localized and read against the
    // face: off it, the roots are not this face's question.
    let off_face = || super::carrier_touch::off_face(y, face, surface, carrier, (t0, t1), band);
    // Any door escalation (a classifying sign in the gap, of its pose or
    // of where its roots lie, or a root finder's touch) is the caller's
    // `escalate`, unless every touch of the span is off the face.
    let door_escalated = |refusal: BooleanError| match refusal {
        BooleanError::Escalated { .. } if off_face()? => Ok(SpanVerdict::OffFace),
        BooleanError::Escalated { decision, diag } => escalate(decision, diag),
        refusal => Err(refusal),
    };
    let found = match *carrier {
        geom::Curve3::Line { origin, dir } => {
            match line_wall_roots_of(origin, dir, (t0, t1), surface, band) {
                Ok(Some(Ok(found))) => Some(found),
                Ok(Some(Err(verdict))) => return Ok(verdict),
                Ok(None) => None,
                Err(refusal) => return door_escalated(refusal),
            }
        }
        // The conic root doors ([`super::circle_roots`]), one answer
        // shape: one door for a circle or an ellipse against a sphere, a
        // wall or a cone, whose quadric form is of degree two along it
        // ([`super::conic_quadric`]), and one per conic against a torus.
        // A conic against any other kind has no root lane here.
        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
            let door = match (carrier, surface) {
                (
                    _,
                    geom::Surface::Sphere { .. }
                    | geom::Surface::Cylinder { .. }
                    | geom::Surface::Cone { .. },
                ) => Some(super::conic_quadric::conic_quadric_roots(
                    carrier, t0, t1, surface, band,
                )),
                (geom::Curve3::Circle { .. }, geom::Surface::Torus { .. }) => Some(
                    super::circle_torus::circle_torus_roots(carrier, t0, t1, surface, band),
                ),
                (_, geom::Surface::Torus { .. }) => Some(
                    super::ellipse_torus::ellipse_torus_roots(carrier, t0, t1, surface, band),
                ),
                _ => None,
            };
            match door.transpose() {
                Ok(found) => found,
                Err(refusal) => return door_escalated(refusal),
            }
        }
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
    };
    let found = match found {
        Some(Ok(found)) => found,
        Some(Err(Unsettled::AtApex)) => return Ok(SpanVerdict::AtApex),
        // A limit of the door, or no root lane at all: where the span may
        // meet the carrier is localized and read against the face, which
        // settles it off the face or keeps the door.
        Some(Err(_)) | None => {
            return Ok(if off_face()? {
                SpanVerdict::OffFace
            } else {
                SpanVerdict::Unsettled
            });
        }
    };
    // The carrier's speed at a parameter: only the kinds `found` was
    // read for reach here, and an ellipse whose frame cannot be read is
    // a kernel bug, refused — never a zero speed that would read every
    // root as at the span's end.
    let speed_at = |t: T| {
        geom::carrier_speed::speed_at(carrier, t).ok_or(BooleanError::ClassificationInvariant {
            what: "a wall root's carrier is neither a line nor a conic",
        })
    };
    let found = match found {
        CircleRoots::Certified(found) => found,
        // A conic ON the surface: its residual is a zero constant — the
        // circle or ellipse that lies on a sphere, wall or torus. (A
        // ruling on a wall is the line door's `Constant`, above.)
        CircleRoots::On(_) => return Ok(SpanVerdict::LiesOn),
        CircleRoots::Miss => return Ok(SpanVerdict::Miss),
    };
    let ts = found.roots();
    // **A root on the far nappe is not this face's.** The roots are the
    // DOUBLE cone's, the carrier [`geom_brep::implicit_residual`]
    // states; a cone face lies on one nappe, and a root definitely off
    // THAT nappe ([`geom_brep::cone_elevation`] asked about it) lies on
    // the other: the carrier is crossed, not here ([`cone_nappe`]).
    let nappe = cone_nappe(y, face, surface, band)?;
    // Whether some root sits at an end of the span, and whether some
    // root strictly inside it was placed outside this face's trim: the
    // two facts that tell [`SpanVerdict::Elsewhere`] from
    // [`SpanVerdict::NoInterior`].
    let mut at_end = false;
    let mut crossed_elsewhere = false;
    for &t in ts {
        let mut interior = true;
        for (gap, end) in [(t - t0, t0), (t1 - t, t1)] {
            match decide(
                "bool_wall_root_in_span",
                Margin::of(gap * speed_at(end)?),
                band,
            ) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Zero) => {
                    interior = false;
                    at_end = true;
                }
                Ok(Sign::Negative) => interior = false,
                Err(diag) => {
                    return Err(BooleanError::Escalated {
                        decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
                        diag,
                    });
                }
            }
        }
        if !interior {
            continue;
        }
        let p = carrier.eval(t);
        if let Some((apex, axis, half_angle, nappe)) = nappe {
            let off = geom_brep::cone_elevation(apex, axis, half_angle, Some(nappe), p);
            match decide("bool_cone_root_nappe", Margin::of(off), band) {
                Ok(Sign::Positive) => {
                    crossed_elsewhere = true;
                    continue;
                }
                Ok(Sign::Zero) => {}
                // Below the face's own nappe: off the double cone the root
                // was certified on, two readings that part; the frontier
                // keeps it, as on main.
                Ok(Sign::Negative) => return Ok(SpanVerdict::Unsettled),
                // Whether the root stands on this face's nappe, in the
                // gap: the cone door's own question, located.
                Err(diag) => {
                    let decision = match carrier {
                        geom::Curve3::Line { .. } => BooleanDecision::ConeRoots,
                        _ => BooleanDecision::ArcConeRoots,
                    };
                    return escalate(decision, diag);
                }
            }
        }
        match landing(y, face, p, band)? {
            Landing::Unplaced => return Ok(SpanVerdict::Unsettled),
            // On the carrier and definitely outside THIS face's trim:
            // the carrier is crossed, but not here. The other roots may
            // still land in the face, so the loop continues.
            Landing::Out => crossed_elsewhere = true,
            Landing::At(at, on) => return Ok(SpanVerdict::Pierce { t, p, at, on }),
        }
    }
    Ok(no_pierce_verdict(crossed_elsewhere, at_end))
}

/// A cone's apex, axis and half-angle, and the nappe a face of it lies on.
type ConeNappe<T> = (Point3<T>, geom_core::Vec3<T>, T, geom_brep::Nappe);

/// **The nappe a cone face lies on**, with the cone's apex, axis and
/// half-angle, for telling a root of the double cone off it
/// ([`geom_brep::cone_elevation`]); `None` for any other kind, and for a
/// face whose corners do not decide its nappe (corners on both sides of
/// its apex), which leaves the question to its trim. A corner station in
/// the band escalates, as the trim's own reading would, and a face the
/// sweep holds that does not resolve is the sweep's desync.
fn cone_nappe<T: Decide>(
    y: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Option<ConeNappe<T>>, BooleanError> {
    let geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *surface
    else {
        return Ok(None);
    };
    match crate::offset_nappe::face_nappe_reading(y, face, band) {
        Ok(nappe) => Ok(Some((apex, axis, half_angle, nappe))),
        Err(NappeRefusal::Straddles { .. }) => Ok(None),
        Err(NappeRefusal::Escalated(source)) => Err(BooleanError::Escalated {
            decision: BooleanDecision::CONTAINMENT_UNNAMED,
            diag: source,
        }),
        Err(NappeRefusal::Stale) => Err(BooleanError::ClassificationInvariant {
            what: "the crossing layer read the nappe of a cone face the sweep holds, and \
                   the face did not resolve",
        }),
    }
}

/// The verdict of a root set with no pierce in this face: `Elsewhere`
/// only when some root strictly inside the span was placed outside the
/// trim AND no root sits at an end — the one case that accounts for a
/// straddle's crossing. A root set with no interior root (or with one at
/// an end) is `NoInterior`, which the straddle arm reads as a
/// contradiction and keeps the door on.
fn no_pierce_verdict<T: geom_core::Real>(crossed_elsewhere: bool, at_end: bool) -> SpanVerdict<T> {
    if crossed_elsewhere && !at_end {
        SpanVerdict::Elsewhere
    } else {
        SpanVerdict::NoInterior
    }
}

/// Where a crossing of a curved face's carrier lands in the face
/// ([`landing`]).
enum Landing {
    /// In the face or on its boundary, `at`, by the margin that decided
    /// a boundary landing.
    At(FaceContainment, Option<MarginDiag>),
    /// On the carrier and definitely outside the face's trim.
    Out,
    /// No verdict: the caller keeps its door.
    Unplaced,
}

/// **Where the crossing point `p`, certified on `face`'s carrier, lands
/// in the face's trim** ([`super::contain::curved_face_placement_decided`]).
///
/// `Unplaced` is the chart door's honest remainder (a ringed face, a
/// boundary outside its outline classes, a full-period window outside
/// the band class), never "outside". **A landing point definitely OFF
/// the carrier is not "outside the trim" either**: the crossing was
/// certified ON the surface, so a point containment decides is off it
/// contradicts that certificate, a crossing the band cannot stand behind
/// at this pose. Reading it as a sibling face's crossing would step over
/// a real one, so it is `Unplaced` too, as is a boundary read the face
/// refuses (a lone-vertex loop, a loop it cannot walk, a chart refusal,
/// an arc's end its corners disagree with).
fn landing<T: Decide>(
    y: &Body<T>,
    face: FaceKey,
    p: Point3<T>,
    band: Band,
) -> Result<Landing, BooleanError> {
    match super::contain::curved_face_placement_decided(y, face, p, band) {
        Ok((CurvedPlacement::OffCarrier | CurvedPlacement::Trim(None), _)) => Ok(Landing::Unplaced),
        Ok((CurvedPlacement::Trim(Some(FaceContainment::Out)), _)) => Ok(Landing::Out),
        Ok((CurvedPlacement::Trim(Some(at)), on)) => Ok(Landing::At(at, on)),
        Err(super::contain::ContainError::Escalated {
            decision,
            escalation,
            diag,
        }) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Containment {
                decision,
                escalation,
            },
            diag,
        }),
        Err(super::contain::ContainError::StaleFace(face)) => {
            super::contain::driver_face_stale(face)
        }
        Err(
            super::contain::ContainError::EmptyLoop(_)
            | super::contain::ContainError::LoopUnreadable(_)
            | super::contain::ContainError::Curved(_)
            | super::contain::ContainError::RayExhausted
            | super::contain::ContainError::Uncrossable(_)
            | super::contain::ContainError::ArcEndClearOfCorners(_),
        ) => Ok(Landing::Unplaced),
    }
}

/// A root door's answer: its roots, or the count it could not certify.
pub(super) type RootsOrUnsettled<T> = Result<CircleRoots<T>, Unsettled>;

/// [`line_wall_roots_of`]'s answer: `None` for a surface with no line
/// root lane, else the roots or the verdict the roots' shape has no word
/// for.
pub(super) type LineWallRoots<T> = Option<Result<RootsOrUnsettled<T>, SpanVerdict<T>>>;

/// The certified LINE × wall roots, per kind, in the root doors' answer
/// shape — or, for the one answer that shape has no word for, the
/// verdict itself: an axis-parallel line's residual is constant along it
/// without being zero ([`SpanVerdict::Constant`]). `(t0, t1)` is the run
/// of the line's parameter the edge covers: its length is the lever the
/// wall's axis-parallel rung is metered over, and the cone's apex rung
/// reads the segment it spans.
pub(super) fn line_wall_roots_of<T: Decide>(
    origin: Point3<T>,
    dir: geom_core::Vec3<T>,
    (t0, t1): (T, T),
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<LineWallRoots<T>, BooleanError> {
    use super::solid_contain::WallRoots;
    let span = (t1 - t0).abs();
    let two = |ts: [T; 2]| Ok(Roots::<T, 2>::all(ts).widen());
    // The certified roots, per kind. Every lane answers the same three
    // ways — a certified root set, a definite miss, or no certain
    // count — and the cylinder adds a fourth, the axis-parallel line
    // whose residual is constant. A line never lies on a torus or a
    // sphere, so neither has such a case. A tangency is not a crossing
    // this lane can act on — the material verdicts behind a pierce are
    // first-order, and along a tangency every first-order datum ties —
    // so it keeps the door.
    Ok(Some(Ok(match *surface {
        geom::Surface::Cylinder {
            origin: c_origin,
            axis,
            radius,
            ..
        } => match super::solid_contain::line_wall_roots(
            origin, dir, c_origin, axis, radius, span, band,
        )
        .map_err(|fault| BooleanError::Escalated {
            decision: BooleanDecision::WallRoots(fault.rung),
            diag: fault.diag,
        })? {
            WallRoots::Two(ts) => two(ts),
            WallRoots::Tangent => Err(Unsettled::Tangency),
            WallRoots::AxisParallel => return Ok(Some(Err(SpanVerdict::Constant))),
            WallRoots::Miss => Ok(CircleRoots::Miss),
        },
        // The quartic: the ray lane's own certified root door, over the
        // edge's span instead of a ray's forward half. It answers only
        // on a CERTIFIED count, so a graze or a repeated root refuses
        // and keeps the door exactly as the cylinder's tangency does.
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => door_refusal(
            super::solid_contain::line_torus_roots(
                origin,
                dir,
                center,
                axis,
                major_radius,
                minor_radius,
                band,
            ),
            BooleanDecision::TorusRoots,
        )?
        .map(Roots::widen),
        // The quadratic, the ray lane's own. No line lies on a sphere,
        // so it has no constant case either.
        geom::Surface::Sphere { center, radius, .. } => {
            match super::solid_contain::line_sphere_roots(origin, dir, center, radius, band)
                .map_err(|diag| BooleanError::Escalated {
                    decision: BooleanDecision::SphereRoots,
                    diag,
                })? {
                WallRoots::Two(ts) => two(ts),
                WallRoots::Tangent => Err(Unsettled::Tangency),
                WallRoots::AxisParallel | WallRoots::Miss => Ok(CircleRoots::Miss),
            }
        }
        geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => line_cone_roots(origin, dir, (apex, axis, half_angle), (t0, t1), band)?,
        // No root lane: the caller reads the span against the face as it
        // reads a door's refusal.
        geom::Surface::Plane { .. } | geom::Surface::Nurbs(_) | geom::Surface::Approx(_) => {
            return Ok(None);
        }
    })))
}

/// **The certified roots of the LINE `origin + dir·t` against a CONE**:
/// the cone's quadric form along it, the quadratic
/// [`super::solid_contain::line_cone_quadratic`] `G = A t² + 2B t + C`
/// (`G = −Q`, `Q = cos²α·ρ² − sin²α·h²`), whose zero set is the double
/// cone. Its rungs, each a length:
///
/// - **`bool_line_cone_apex`** — the segment `[t0, t1]`'s distance from
///   the apex. Not definitely positive: the edge reaches the apex, where
///   the cone's gradient vanishes, and the lane refuses
///   [`Unsettled::AtApex`] rather than read a root there.
/// - **`bool_line_cone_lead`** — `A/|dir|²` levered by the segment's
///   length: how far the line turns off a generator over the edge. Zero:
///   the quadratic degenerates (a ruling, or a line parallel to one) and
///   the lane does not answer ([`Unsettled::Degenerate`]).
/// - **`bool_line_cone_depth`** — the depth the line's vertex `t*`
///   reaches: `G(t*) = −disc/A`, and on the near nappe
///   `Q = res·(ρ cos α + |h| sin α)` with that factor at most `|q|`, so
///   `disc/(|A|·R)`, `R ≥ |q(t*)|`, is a lower bound on the residual's
///   magnitude there, signed as the crossing count. Positive: two roots;
///   Zero: a tangency, [`Unsettled::Tangency`]; Negative: a miss.
/// - **`bool_line_cone_root_slack`** — each root's arc-length slack: the
///   rounding of `G`'s evaluation (charged against its term bound,
///   `(|w₀| + |dir||t|)²`) over `|G′(t)| = 2√disc`, at the line's speed,
///   less the root's distance outside the span. Not definitely inside
///   the band: [`Unsettled::Meter`]. A root inside the span is
///   placed within the band; one outside it is placed no nearer the span
///   than the band, which is all the span decisions read of it.
fn line_cone_roots<T: Decide>(
    origin: Point3<T>,
    dir: geom_core::Vec3<T>,
    (apex, axis, half_angle): (Point3<T>, geom_core::Vec3<T>, T),
    (t0, t1): (T, T),
    band: Band,
) -> Result<Result<CircleRoots<T>, Unsettled>, BooleanError> {
    let escalate = |diag| BooleanError::Escalated {
        decision: BooleanDecision::ConeRoots,
        diag,
    };
    let w0 = origin - apex;
    let dd = dir.norm_squared();
    let at = |t: T| w0 + dir * t;
    let foot = ((T::zero() - w0.dot(dir)) / dd)
        .max(t0.min(t1))
        .min(t0.max(t1));
    match decide("bool_line_cone_apex", Margin::norm3(at(foot)), band).map_err(escalate)? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Ok(Err(Unsettled::AtApex)),
    }
    let [a2, b2, c2] =
        super::solid_contain::line_cone_quadratic(origin, dir, apex, axis, half_angle);
    let length = dir.norm() * (t1 - t0).abs();
    match decide(
        "bool_line_cone_lead",
        Margin::levered(a2 / dd, length),
        band,
    )
    .map_err(escalate)?
    {
        Sign::Positive | Sign::Negative => {}
        Sign::Zero => return Ok(Err(Unsettled::Degenerate)),
    }
    let disc = b2.powi(2) - a2 * c2;
    let vertex = (T::zero() - b2) / a2;
    let reach = at(t0).norm().max(at(t1).norm()).max(at(vertex).norm());
    match decide(
        "bool_line_cone_depth",
        Margin::of(disc / (a2.abs() * reach)),
        band,
    )
    .map_err(escalate)?
    {
        Sign::Positive => {}
        Sign::Zero => return Ok(Err(Unsettled::Tangency)),
        Sign::Negative => return Ok(Ok(CircleRoots::Miss)),
    }
    let roots = geom_brep::quartic::quadratic_roots(a2, b2, c2, disc);
    let two = T::from_f64(2.0);
    let speed = dir.norm();
    let (lo, hi) = (t0.min(t1), t0.max(t1));
    for t in roots {
        let terms = (w0.norm() + speed * t.abs()).powi(2);
        let slack = speed * geom_core::rounding_charge(terms) / (two * disc.sqrt());
        let outside = speed * (lo - t).max(t - hi).max(T::zero());
        match decide(
            "bool_line_cone_root_slack",
            Margin::of(slack - outside),
            band,
        ) {
            Ok(Sign::Zero | Sign::Negative) => {}
            Ok(Sign::Positive) | Err(_) => return Ok(Err(Unsettled::Meter)),
        }
    }
    Ok(Ok(Roots::<T, 2>::all(roots).widen()))
}

/// What one edge×curved-face pair asks of the sweep.
#[derive(Debug, Clone, Copy)]
pub(super) enum CurvedEvent<T: geom_core::Real> {
    /// No event: the pair is definitely clear, or the crossing lies
    /// outside this face's trim.
    None,
    /// The arm recorded the event itself (the one-sided cover rung,
    /// whose endpoint treatment mints its own contacts). Reported so
    /// the differential suite's accepted-pair channel still sees it.
    Recorded,
    /// The arm read the edge against the face by its ends only: an edge
    /// lying on the carrier, or a covered arc touching it ([`MintedRule`]).
    /// `recorded` is whether it recorded a contact. A vertex a later split
    /// mints on the edge is read against the face under `on`
    /// ([`LyingIn::Curved`]).
    Lies { recorded: bool, on: OnCurved },
    /// A covered edge with no end on the carrier, whatever it meets on
    /// the face lying strictly inside it, at one point or several: the
    /// cover puts the edge in one closed side of the carrier, and neither
    /// the roots (a line on a cylinder or sphere) nor the interior
    /// question settled it ([`Placement::declared`]). The cover records endpoints
    /// only, so the sweep defers the pair ([`DeferredTouch`]) until both
    /// directions have run and then reads it again on the edge's
    /// fragments ([`settle_deferred`]).
    Deferred,
    /// The edge lies along a boundary edge of a spline face
    /// ([`spline_face`]), its ends recorded on that boundary. A vertex a
    /// later split mints on the edge is placed on it too
    /// ([`LyingIn::Along`]).
    Along {
        /// Whether it recorded a contact.
        recorded: bool,
    },
    /// A definite wall crossing at the carrier parameter `t`, whose
    /// point `p` the trim placed at `at`. The sweep splits the edge and
    /// records the contact exactly as it does for a conic × plane root.
    Pierce {
        t: T,
        p: Point3<T>,
        at: FaceContainment,
        /// The margin that decided `p` on the boundary entity `at`
        /// names (`None` for `In`, a transverse pierce).
        on: Option<MarginDiag>,
        /// The bisector plane the crossing lies on, where the sweep read
        /// an equal cylinder pair's wall through its planes
        /// ([`factored_crossing`]); the pierce reads the partner wall
        /// through it.
        branch: Option<super::Branch>,
        /// The chart point a spline face's interior meeting was found and
        /// placed in the trim at: the pierce reads the face's normal there
        /// ([`ContactAcc::charts`]).
        chart: Option<(f64, f64)>,
    },
}

impl<T: geom_core::Real> CurvedEvent<T> {
    /// A covered arm's [`Placement::declared`] answer for an edge it read
    /// by its ends, carrying how it read them.
    fn lying(self, on: OnCurved) -> Self {
        match self {
            Self::None => Self::Lies {
                recorded: false,
                on,
            },
            Self::Recorded => Self::Lies { recorded: true, on },
            other => other,
        }
    }
}

/// Where one on-carrier endpoint's incidence lives.
///
/// The distinction between the last two is the whole point of this
/// type: `Elsewhere` is a CERTIFIED verdict — the face's region proved
/// the point is not in this face — whereas `Undecided` is the
/// containment door's honest remainder. A caller that reads them as
/// one thing refuses at a frontier for a pair that has no incidence to
/// begin with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Placement {
    /// A contact record was written for this endpoint.
    Recorded,
    /// The point is definitely not on this face and coincides with no
    /// vertex of `y`: its incidence, if it has one, belongs to another
    /// face on the same carrier, which the sweep visits separately.
    Elsewhere,
    /// No verdict — the containment door's remainder.
    Undecided,
}

impl Placement {
    /// **The covered rungs' one rule**, in one place because all of
    /// them apply it: given each of an edge's endpoints as
    /// `Some(placement)` when the residual put it ON the carrier and
    /// `None` when it is honestly not this arm's business (definitely
    /// clear, or the arm has only one on-carrier end), does this pair
    /// record an event, have none here, wait for the edge's fragments,
    /// or keep the typed frontier (`None`)?
    ///
    /// - **any `Undecided`** keeps the door: an endpoint the containment
    ///   door could not place leaves the pair unknown. Only the truth
    ///   table below holds this end to end: `Undecided` needs an
    ///   on-carrier end on a face whose trim the chart door declines (a
    ///   ringed face) while every boundary edge is a line or a circle
    ///   (anything else answers `Unread` first), and the one such bore
    ///   tried — a full-turn collar less a partial-revolve wedge —
    ///   builds under every op
    ///   (`sweep/tests/wedge_through_a_full_turn_collar.rs`) without
    ///   ever placing an end `Undecided`: measured over the full suites
    ///   of `topo`, `sweep`, `mesh`, `editor-core` and `step-import`
    ///   and the tour at ε 1e-9 (all 48 of its poses among them), no
    ///   endpoint is placed `Undecided` anywhere, and the rule reads
    ///   one only in the truth-table rows below;
    /// - **any `Recorded`** records;
    /// - **every on-carrier end `Elsewhere`** has placed nothing on this
    ///   face. That is no event only when `interior_clear` — the arm
    ///   certified the span's interior meets this face's boundary
    ///   nowhere ([`interior`]), so a span with both ends
    ///   outside the face lies wholly outside it. Without that
    ///   certificate an end ON the carrier keeps the door: an overlap
    ///   lying wholly inside this face's window, with both ends beyond
    ///   it, must not turn into a silent no-event;
    /// - **no end on the carrier**, the interior unseen, is deferred
    ///   ([`CurvedEvent::Deferred`]): whatever the edge meets on this
    ///   face, at one point or several, lies strictly inside it, and
    ///   the settle stage reads it again on the fragments both
    ///   directions leave.
    fn declared<T: geom_core::Real>(
        ends: [Option<Self>; 2],
        interior_clear: bool,
    ) -> Option<CurvedEvent<T>> {
        let on = ends.iter().flatten();
        if on.clone().any(|p| *p == Self::Undecided) {
            None
        } else if on.clone().any(|p| *p == Self::Recorded) {
            Some(CurvedEvent::Recorded)
        } else if interior_clear {
            Some(CurvedEvent::None)
        } else if ends == [None, None] {
            Some(CurvedEvent::Deferred)
        } else {
            None
        }
    }

    /// **The undeclared `NoInterior` arms' one rule** — the mixed-sign
    /// span (one end ON the carrier) and the `(Zero, Zero)` chord. Given
    /// each ON end's placement (`None` for an end that is not on the
    /// carrier), the pair:
    ///
    /// - **records** if any end was `Recorded` (the long-standing OR
    ///   rule: a sibling's answer does not veto a real incidence);
    /// - is **no event HERE** if every ON end is `Elsewhere`: the span
    ///   has no incidence strictly inside it on this face (`NoInterior`),
    ///   and each ON end is certified outside this face's trim;
    /// - otherwise keeps the typed door (`None`): an `Undecided` end is
    ///   not evidence of absence.
    ///
    /// An all-`Elsewhere` pair is eventless here, as it is under
    /// [`Self::declared`] with a clear interior: the distinct certified
    /// roots behind `NoInterior` exclude an on-carrier edge overlapping
    /// the window.
    fn undeclared_no_interior<T: geom_core::Real>(
        ends: [Option<Self>; 2],
    ) -> Option<CurvedEvent<T>> {
        let on = ends.iter().flatten();
        if on.clone().any(|p| *p == Self::Recorded) {
            Some(CurvedEvent::Recorded)
        } else if on.clone().all(|p| *p == Self::Elsewhere) {
            Some(CurvedEvent::None)
        } else {
            None
        }
    }
}

/// The declared-cosurface rung's endpoint treatment: classify an
/// on-carrier vertex of `x` against the CURVED face and record the
/// planar posture's contact kinds — `OnVertex` ⇒ v-v, `OnEdge` ⇒
/// split `y`'s boundary edge at the (bitwise-shared) point and pair
/// the minted vertex, `In` ⇒ the v-f record, exactly as
/// [`vertex_on_face`] does on a plane.
///
/// **`Out` and no-verdict are DIFFERENT answers, and this door reports
/// them apart.** `curved_face_containment` answers `Out` from a
/// certified comparison — the point is off the carrier, or the face's
/// region excludes it — so `Out`
/// says the endpoint's incidence is not this pair's. It says nothing
/// about whether the endpoint has an incidence at all: on a carrier
/// shared by several faces the endpoint is a seam site, and the face
/// that holds it records it when the sweep reaches that pair. A
/// no-verdict is the opposite: the door could not express this face's
/// trim, so nothing at all is known and the caller's typed frontier is
/// the only honest answer.
#[allow(clippy::too_many_arguments)]
pub(super) fn vertex_on_curved_face<T: Decide + crate::props::AtRestPolicy>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    on_carrier: MarginDiag,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<Placement, BooleanError> {
    vertex_on_curved_face_at(x_is, y, vx, px, face, Some(on_carrier), contacts, band, tol)
        .map(|(p, _)| p)
}

/// [`vertex_on_curved_face`], also naming the vertex of `y` a recorded
/// v-v contact paired `vx` with (`None` for a v-f record, or no record).
#[allow(clippy::too_many_arguments)]
fn vertex_on_curved_face_at<T: Decide + crate::props::AtRestPolicy>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    on_carrier: Option<MarginDiag>,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<(Placement, Option<VertexKey>), BooleanError> {
    let (placement, on, chart) = super::contain::curved_face_placement_charted(y, face, px, band)
        .map_err(|e| esc(e, x_is.other(), face))?;
    let verdict = match placement {
        CurvedPlacement::Trim(v) => v,
        CurvedPlacement::OffCarrier => None,
    };
    match verdict {
        Some(FaceContainment::OnVertex(vy)) => {
            push_vv(contacts, x_is, vx, vy, on);
            return Ok((Placement::Recorded, Some(vy)));
        }
        Some(FaceContainment::OnEdge(ey)) => {
            let wy = split_other_at_point(
                y,
                x_is.other(),
                ey,
                px,
                super::Cell::Vertex(vx),
                band,
                tol,
                contacts,
            )?;
            push_vv(contacts, x_is, vx, wy, on);
            return Ok((Placement::Recorded, Some(wy)));
        }
        // Strictly inside the curved face: the same v-f record the
        // planar sweep writes ([`vertex_on_face`]).
        Some(FaceContainment::In) => {
            push_vf(contacts, x_is, vx, face, on_carrier);
            if let Some(uv) = chart {
                contacts.chart_point(x_is, vx, face, uv);
            }
            return Ok((Placement::Recorded, None));
        }
        // Definitely outside this face, or no verdict at all:
        // fall through to the face-free question below.
        Some(FaceContainment::Out) | None => {}
    }
    // Not on THIS face's boundary. One face-free question is still
    // decidable by the same row: coincidence with a vertex of `y`
    // anywhere (arena order, D9). An on-carrier edge is a candidate
    // against EVERY face sharing the carrier, and against the faces
    // whose trim does not hold the endpoint the honest answer is "the
    // event belongs elsewhere": a valid body's vertices lie on face
    // boundaries, never interior to a face, so a vertex hit certifies
    // the endpoint is a boundary site — and the v-v record is
    // face-free, so it is the SAME record the holding face's pair
    // produces (the accumulator dedups).
    //
    // **A miss here is not automatically the frontier, and the reason
    // is the pierce ring.** The "vertices lie on face boundaries"
    // premise above holds for an operand's OWN vertices, but a wall
    // pierce mints a vertex INTERIOR to the pierced face: after such a
    // split the pierced edge's fragment endpoint sits strictly inside
    // one face of the carrier and strictly outside every sibling
    // face, and no vertex of `y` is there to be found. So the two
    // answers are reported apart — a definite `Out` says "this face
    // has no incidence", a `None` says "no verdict at all" — and the
    // caller decides what each licenses.
    if let Some((vy, on)) =
        super::vertex_at(y, px, band).map_err(|diag| BooleanError::Escalated {
            decision: BooleanDecision::VertexOnVertex,
            diag,
        })?
    {
        push_vv(contacts, x_is, vx, vy, Some(on));
        return Ok((Placement::Recorded, Some(vy)));
    }
    // Only an ON-carrier `Out` is a certified absence. Every caller
    // reaches this door with the endpoint's residual decided `Zero`, so
    // an off-carrier answer contradicts that decision and is not
    // evidence the incidence lives elsewhere: it keeps the door.
    Ok((
        match placement {
            CurvedPlacement::Trim(Some(FaceContainment::Out)) => Placement::Elsewhere,
            _ => Placement::Undecided,
        },
        None,
    ))
}

/// A containment read of `face`, a face of `operand`, refused. The body
/// is MID-OPERATION — a working copy the reduction has been splitting —
/// so the operand gate's at-rest verdict does not bind it, and every
/// arm but the stale face answers typed: a refusal the face door owns
/// is carried whole with the face it read.
pub(super) fn esc(e: ContainError, operand: Operand, face: FaceKey) -> BooleanError {
    match e {
        ContainError::Escalated {
            decision,
            escalation,
            diag,
        } => BooleanError::Escalated {
            decision: BooleanDecision::Containment {
                decision,
                escalation,
            },
            diag,
        },
        ContainError::Uncrossable(cause) => {
            BooleanError::ArcLoopContainmentUnsupported { operand, cause }
        }
        ContainError::StaleFace(face) => super::contain::driver_face_stale(face),
        refusal @ (ContainError::RayExhausted
        | ContainError::EmptyLoop(_)
        | ContainError::LoopUnreadable(_)
        | ContainError::Curved(_)
        | ContainError::ArcEndClearOfCorners(_)) => BooleanError::PointInFaceRefused {
            operand,
            face,
            refusal,
        },
    }
}

impl Operand {
    /// The other operand.
    pub fn other(self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::A,
        }
    }
}

/// Orients a v-v contact: `x_is` names the operand `wx` lives in.
/// `on` is the margin that decided the two one.
fn push_vv(
    contacts: &mut ContactAcc,
    x_is: Operand,
    wx: VertexKey,
    wy: VertexKey,
    on: Option<MarginDiag>,
) {
    let c = match x_is {
        Operand::A => VvContact { a: wx, b: wy },
        Operand::B => VvContact { a: wy, b: wx },
    };
    let row = PendingRow::on(
        (x_is, super::Cell::Vertex(wx)),
        (x_is.other(), super::Cell::Vertex(wy)),
        on,
    );
    contacts.vv(c, row);
}

/// Records `vertex` of `x_is` on `face` of the other operand, decided
/// there by `on`.
fn push_vf(
    contacts: &mut ContactAcc,
    x_is: Operand,
    vertex: VertexKey,
    face: FaceKey,
    on: Option<MarginDiag>,
) {
    let row = PendingRow::on(
        (x_is, super::Cell::Vertex(vertex)),
        (x_is.other(), super::Cell::Face(face)),
        on,
    );
    contacts.vf(x_is, VfContact { vertex, face }, row);
}

/// `dovertexonface`: an existing vertex of `x` lies on `face`'s plane,
/// decided there by `on_plane` — classify it against the face and
/// record the contact kind. Returns
/// whether the exact predicates ACCEPTED an event (anything but `Out`)
/// — the differential suite's accepted-pair channel; recording changes
/// no classification.
#[allow(clippy::too_many_arguments)]
fn vertex_on_face<T: Decide + crate::props::AtRestPolicy>(
    x_is: Operand,
    y: &mut Body<T>,
    vx: VertexKey,
    px: Point3<T>,
    face: FaceKey,
    plane: &PlaneDesc<T>,
    on_plane: MarginDiag,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<bool, BooleanError> {
    let (at, on) =
        contfp_decided(y, face, plane.normal, px, band).map_err(|e| esc(e, x_is.other(), face))?;
    match at {
        FaceContainment::Out => return Ok(false),
        FaceContainment::In => push_vf(contacts, x_is, vx, face, Some(on_plane)),
        FaceContainment::OnEdge(ey) => {
            let wy = split_other_at_point(
                y,
                x_is.other(),
                ey,
                px,
                super::Cell::Vertex(vx),
                band,
                tol,
                contacts,
            )?;
            push_vv(contacts, x_is, vx, wy, on);
        }
        FaceContainment::OnVertex(vy) => push_vv(contacts, x_is, vx, vy, on),
    }
    Ok(true)
}

/// Splits `x`'s `edge` at `t` where it crosses `by`, a cell of the
/// other operand, and logs the split, the lineage a carried
/// `(vertex, edge)` record follows onto the piece it rests on.
#[allow(clippy::too_many_arguments)]
fn split_at<T: Decide + crate::props::AtRestPolicy>(
    x: &mut Body<T>,
    x_is: Operand,
    edge: EdgeKey,
    t: T,
    by: super::Cell,
    tol: Tol,
    contacts: &mut ContactAcc,
) -> Result<VertexKey, BooleanError> {
    split_logged(x, x_is, edge, t, None, by, tol, contacts)
}

/// [`split_at`], the new vertex holding `at`'s bits where it is `Some`
/// ([`Body::split_edge_onto`]).
#[allow(clippy::too_many_arguments)]
fn split_logged<T: Decide + crate::props::AtRestPolicy>(
    x: &mut Body<T>,
    x_is: Operand,
    edge: EdgeKey,
    t: T,
    at: Option<Point3<T>>,
    by: super::Cell,
    tol: Tol,
    contacts: &mut ContactAcc,
) -> Result<VertexKey, BooleanError> {
    let created = match at {
        None => x.split_edge(edge, t, tol),
        Some(p) => x.split_edge_onto(edge, t, p, tol),
    }
    .map_err(|source| BooleanError::CrossingInsertion {
        operand: x_is,
        edge,
        source: source.from_driver(),
    })?;
    contacts.splits.push(super::EdgeSplit {
        operand: x_is,
        parent: edge,
        vertex: created.vertex,
        child: created.new_edge,
        by,
    })?;
    Ok(created.vertex)
}

/// Splits the OTHER solid's boundary edge at the (already-computed)
/// event point `p` — the both-edges-split lane that turns an edge-edge
/// crossing into a v-v pair. The new vertex holds `p`'s own bits, not
/// its carrier's at the parameter read back from `p`, so the pair it
/// joins (an existing vertex, or the piercing edge's split at `p`)
/// reads one point whichever operand's point the merge keeps. `by` is
/// the cell of the splitting operand that meets the edge at `p`.
///
/// The parameter is [`geom::Curve3::param_near`] anchored at the span's
/// MIDPOINT: a line's projection `t = (p − origin)·dir`, a circle's
/// azimuth, an ellipse's eccentric anomaly, a spiric's tube angle. That
/// function carries the derivations and the reason an anchored read
/// needs no branch selection; the midpoint is this site's anchor because
/// the parameter wanted is the one INSIDE the stored span, and within a
/// period the nearest branch to the midpoint is exactly that one. Where
/// an interval enclosure of it is too wide to place `t` strictly inside
/// the span, `split_edge`'s own interiority trilean escalates.
///
/// On an ellipse the eccentric anomaly is that of `p`'s unit-coordinate
/// image, not of its foot: for a point `ε` off the carrier it names a
/// point up to `(1 + a/b)·ε` from `p`. The children certify against `p`
/// itself, so such a parameter refuses typed (`ResidualExceeded` at the
/// child's end) and never mints a child that misses its vertex
/// (`spline_face_rows::a_split_of_a_thin_ellipse_lands_where_it_was_read_or_refuses`).
///
/// On a circle, `p` must lie ON the carrier for the azimuth to name the
/// event: the distance from `p` to the circle (radial and axial misses
/// folded, the exact hypotenuse) is classified on `bool_contact_arc`
/// before the parameter is taken, and the span is checked to be at most
/// a period. The angular half of "on the ARC" is not repeated here:
/// `split_edge`'s interiority gate is exactly that question, metered in
/// metres at the radius.
///
/// A `Nurbs` carrier has no point parameter and keeps the typed refusal
/// [`BooleanError::PointSplitCarrierUnsupported`], its own variant
/// because this precondition is NOT the operand gate's.
#[allow(clippy::too_many_arguments)]
fn split_other_at_point<T: Decide + crate::props::AtRestPolicy>(
    y: &mut Body<T>,
    y_is: Operand,
    edge: EdgeKey,
    p: Point3<T>,
    by: super::Cell,
    band: Band,
    tol: Tol,
    contacts: &mut ContactAcc,
) -> Result<VertexKey, BooleanError> {
    let curve = certified(y.get_edge(edge).and_then(|e| y.get_curve_geom(e.curve)))?.clone();
    // The circle's two preconditions, both of them this site's and
    // neither of them the shared arithmetic's.
    if let geom::Curve3::Circle {
        center,
        axis,
        radius,
        ..
    } = *curve.carrier()
    {
        // On the carrier? `point_on_circle` is the row's body, with its
        // impossible-negative arm.
        match super::contain::point_on_circle(p, center, axis, radius, band) {
            Ok(Some(_)) => {}
            // The caller placed the event ON this edge; a point
            // definitely off its carrier means two exact rows disagree,
            // which is a broken invariant, not a frontier.
            Ok(None) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "split point definitely off the circle carrier it was placed on",
                });
            }
            Err(diag) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::SplitPointOnCircle,
                    diag,
                });
            }
        }
        // A span of at most one period is what makes the MIDPOINT
        // anchor's branch the right one (past a period the azimuth
        // aliases by 2π silently, and `split_edge`'s interiority gate
        // cannot see it): the edge's certificate holds its turn, which
        // is never over one ([`geom_brep::CertifiedTurn`]).
    }
    let t = point_param(y, y_is, edge, &curve, p)?;
    split_logged(y, y_is, edge, t, Some(p), by, tol, contacts)
}

/// **The parameter of `p` on `edge`'s curve**: its carrier's
/// [`geom::Curve3::param_near`] anchored at the span's midpoint, the
/// branch inside the stored span ([`split_other_at_point`] carries the
/// derivation). `p` must lie on the carrier.
///
/// # Errors
///
/// [`BooleanError::PointSplitCarrierUnsupported`] on a spline carrier,
/// which has no point parameter; [`BooleanError::SplitLineage`] where
/// the edge's split root does not resolve while naming it.
fn point_param<T: Decide>(
    body: &Body<T>,
    operand: Operand,
    edge: EdgeKey,
    curve: &geom_brep::EdgeCurve<T>,
    p: Point3<T>,
) -> Result<T, BooleanError> {
    let (t0, t1) = curve.params();
    match curve.carrier().param_near(p, geom::mid_param(t0, t1)) {
        Some(t) => Ok(t),
        None => Err(BooleanError::PointSplitCarrierUnsupported {
            operand,
            edge: body
                .split_root(edge, |_| false)
                .map_err(BooleanError::SplitLineage)?,
        }),
    }
}

/// An edge the sweep has yet to read, from face index `start` on.
/// `lying_in` holds the faces before `start` that an arm read the edge
/// lying in by its ends only ([`LyingIn`]): a split of the edge reads
/// the vertex it mints against them ([`read_then_requeue`]), whichever
/// lane splits it.
struct Pending {
    edge: EdgeKey,
    start: usize,
    lying_in: Vec<LyingIn>,
}

/// A face an edge lies in, read at the edge's ends only, and the rule a
/// vertex a later split mints on the edge is read against it under.
///
/// A conic lying in a face's plane (`ConicPlaneMeet::Parallel`) is not
/// recorded: reading a later split's vertex against that face turns
/// operations that build sound into refusals
/// (`a_rim_in_a_face_beside_a_resting_ridge_answers_every_op_as_before`,
/// `work/tang/a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread.md`).
#[derive(Debug, Clone, Copy)]
enum LyingIn {
    /// A planar face the line lane's coplanar arm read the edge in: the
    /// vertex takes the planar side-then-[`vertex_on_face`] read.
    Plane(FaceKey),
    /// A curved face [`curved_face_arm`] read the edge on by its ends
    /// ([`CurvedEvent::Lies`]): the vertex is placed by
    /// [`vertex_on_curved_face`] and read under the arm's own rule.
    Curved(FaceKey, OnCurved),
    /// A spline face [`spline_face`] read the edge lying along the
    /// boundary of ([`CurvedEvent::Along`]): the vertex is placed on that
    /// boundary ([`along_boundary`]).
    Along(FaceKey),
}

/// How [`curved_face_arm`] read an edge against a curved face by its
/// ends only, which a vertex a later split mints on the edge is read
/// under too ([`read_lying_in`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct OnCurved {
    rule: MintedRule,
    /// The coincidence the arm refuses an end's in-band side as.
    side: Coincide,
}

/// The rule an arm combined an edge's end placements under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MintedRule {
    /// The covered arms ([`Placement::declared`]): a line with both ends
    /// on the carrier, and an arc whose clearance is zero (on the carrier,
    /// or touching it with its ends off on the side the cover admits).
    /// `interior_clear` is whether the arm certified the edge's interior
    /// clear of the face's boundary, which every piece of the edge
    /// inherits.
    Declared { interior_clear: bool },
    /// [`lying_on`]'s rule for an arc or a ruling lying on the carrier,
    /// whose interior meets the face's boundary nowhere: where its ends
    /// placed it.
    LiesOn(LyingEnds),
}

/// Where [`lying_on`] placed an edge lying on the carrier, from its
/// ends, once its interior was certified clear of the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LyingEnds {
    /// Every end certified outside the face: the edge lies wholly
    /// outside it.
    Outside,
    /// Every end recorded in the face, none paired with a vertex of the
    /// face's body: the edge lies wholly inside it.
    Inside,
    /// An end paired with a vertex of the face's body, which the
    /// inside/outside argument does not reach.
    Paired,
}

/// Why a minted vertex's read refuses ([`MintedRule::side`],
/// [`MintedRule::agrees`]).
#[derive(Debug, Clone, Copy, PartialEq)]
enum MintedRefusal {
    /// Its side against the carrier is in the band: the arm's own
    /// coincidence refusal ([`OnCurved::side`]).
    InBand(geom_core::Indeterminate),
    /// Its reading contradicts how the arm read the edge: the arm's
    /// frontier.
    Contradicts,
}

impl MintedRule {
    /// What a minted vertex's side against the carrier, `decided`, asks
    /// of its read: place it (`Some`, with the margin that put it on the
    /// carrier), or not (`None`), or refuse. Off the carrier it is no
    /// incidence under a cover, on the side the cover admits
    /// (`admitted`), as the arm takes an end there; on the other side, or
    /// on an edge [`lying_on`] took, which lies on the carrier, it
    /// contradicts the arm.
    fn side(
        self,
        decided: Result<geom_core::Decided, geom_core::Indeterminate>,
        admitted: impl Fn(Sign) -> bool,
    ) -> Result<Option<MarginDiag>, MintedRefusal> {
        let on = decided.map_err(MintedRefusal::InBand)?;
        match (on.sign, self) {
            (Sign::Zero, _) => Ok(Some(on.margin)),
            (side, Self::Declared { .. }) if admitted(side) => Ok(None),
            _ => Err(MintedRefusal::Contradicts),
        }
    }

    /// Whether a minted vertex placed `placed` on the face (`paired` when
    /// the record paired it with a vertex of the face's body) agrees with
    /// how the arm placed the edge. A minted vertex lies inside the edge,
    /// so under [`Self::LiesOn`] it lies where the edge does, but for a
    /// pairing, which the argument does not reach: placed elsewhere on an
    /// edge recorded wholly inside, or recorded unpaired on one wholly
    /// outside, is two certified answers contradicting.
    fn agrees(self, placed: Placement, paired: bool) -> bool {
        match self {
            Self::Declared { interior_clear } => {
                Placement::declared::<f64>([Some(placed), None], interior_clear).is_some()
            }
            Self::LiesOn(ends) => match (ends, placed) {
                (_, Placement::Undecided) => false,
                (LyingEnds::Paired, _) | (LyingEnds::Inside, Placement::Recorded) => true,
                (LyingEnds::Outside, Placement::Elsewhere) => true,
                (LyingEnds::Outside, Placement::Recorded) => paired,
                (LyingEnds::Inside, Placement::Elsewhere) => false,
            },
        }
    }
}

/// Reads vertex `w` of `x`, at `p`, against planar `face` of `y`, `x`'s
/// slit tips at rest `tips`: on the plane, it goes to [`vertex_on_face`]. Returns whether it
/// recorded a contact; a side the band cannot decide refuses as
/// `read` names it.
#[allow(clippy::too_many_arguments)]
fn read_on_plane<T: Decide + crate::props::AtRestPolicy>(
    (x, x_is, tips): (&Body<T>, Operand, &SlitTips),
    y: &mut Body<T>,
    (w, p): End<T>,
    (face, plane): (FaceKey, &PlaneDesc<T>),
    read: DeclarationRead,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<bool, BooleanError> {
    let on = vertex_face_side(
        (x, tips),
        (w, p),
        (y, face, plane),
        (plane.origin, plane.normal),
        band,
    )
    .map_err(|refusal| vertex_refusal(refusal, read))?;
    if on.sign != Sign::Zero {
        return Ok(false);
    }
    vertex_on_face(x_is, y, w, p, face, plane, on.margin, contacts, band, tol)
}

/// The swept solid `x`, its slit tips at rest, which operand it is, and
/// an edge of it with its key.
type XEdge<'a, T> = (
    &'a Body<T>,
    &'a SlitTips,
    Operand,
    EdgeKey,
    &'a crate::entity::Edge,
);

/// Reads `w`, a vertex a split minted on `edge` of `x`, against each
/// face of `lying_in` as the edge's ends were read, then requeues the
/// edge's two pieces from face index `next_face`, carrying `lying_in`
/// on. Every split goes on through here, so none skips the read.
#[allow(clippy::too_many_arguments)]
fn read_then_requeue<T: Decide + crate::props::AtRestPolicy>(
    worklist: &mut std::collections::VecDeque<Pending>,
    (x, tips, x_is, parent, edge): XEdge<'_, T>,
    y: &mut Body<T>,
    w: VertexKey,
    next_face: usize,
    lying_in: &[LyingIn],
    declared: &super::GluedPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    read_lying_in(
        (x, tips, x_is, parent, edge),
        y,
        w,
        lying_in,
        declared,
        contacts,
        band,
        tol,
    )?;
    requeue(worklist, x, parent, w, next_face, lying_in)
}

/// The contact half of [`read_then_requeue`]: `w` of `x`, minted on
/// `edge` (`edge_key`), against each face of `y` in `lying_in`, under
/// the read each entry names ([`LyingIn`]).
///
/// # Errors
///
/// A side the band cannot decide refuses as the arm's own ends do; a
/// curved reading that contradicts the arm's ([`MintedRule`]) keeps its
/// frontier ([`BooleanError::CurvedPierceUnsupported`]).
#[allow(clippy::too_many_arguments)]
fn read_lying_in<T: Decide + crate::props::AtRestPolicy>(
    (x, tips, x_is, edge_key, edge): XEdge<'_, T>,
    y: &mut Body<T>,
    w: VertexKey,
    lying_in: &[LyingIn],
    declared: &super::GluedPairs<T>,
    contacts: &mut ContactAcc,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let p = x.resolve_vertex_point(w, crate::live::Proven);
    for &lies in lying_in {
        match lies {
            LyingIn::Plane(face) => {
                let plane = face_plane(y, face).ok_or(BooleanError::ClassificationInvariant {
                    what: "a face an edge lies in lost its plane",
                })?;
                let read = edge_face_read(x, x_is, edge, face, declared, Coincide::VertexOnFace);
                read_on_plane(
                    (x, x_is, tips),
                    y,
                    (w, p),
                    (face, &plane),
                    read,
                    contacts,
                    band,
                    tol,
                )?;
            }
            // The vertex lies inside the edge, so on the boundary run the
            // edge lies along. Placing it may split `y`'s boundary edge,
            // as the curved arm's placement below may: a split of `y`
            // mints a vertex at a point of its own edge, which no reading
            // of `y` made before it in this sweep depends on.
            LyingIn::Along(face) => {
                along_boundary(x_is, y, (w, p), face, contacts, band, tol)?;
            }
            LyingIn::Curved(face, on_curved) => {
                let surface = y
                    .get_face(face)
                    .and_then(|f| y.get_surface(f.surface))
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "a curved face an edge lies on lost its surface",
                    })?;
                let frontier = BooleanError::CurvedPierceUnsupported {
                    operand: x_is,
                    face,
                    edge: edge_key,
                    band,
                };
                // The arm's own end side question, asked of the minted
                // vertex.
                let decided = geom_core::k_stats::decide_reported(
                    "bool_vertex_face_side",
                    Margin::of(geom_brep::implicit_residual(surface, p)),
                    band,
                );
                let covers = edge_covers(x, x_is, edge, face, declared);
                let admitted = |sign| covers.iter().any(|c| c.admits(sign));
                let margin = match on_curved.rule.side(decided, admitted) {
                    Ok(Some(margin)) => margin,
                    Ok(None) => continue,
                    Err(MintedRefusal::InBand(diag)) => {
                        let which = on_curved.side;
                        let read = edge_face_read(x, x_is, edge, face, declared, which);
                        return Err(BooleanError::coincidence(which, read, diag));
                    }
                    Err(MintedRefusal::Contradicts) => return Err(frontier),
                };
                let (placed, paired) = vertex_on_curved_face_at(
                    x_is,
                    y,
                    w,
                    p,
                    face,
                    Some(margin),
                    contacts,
                    band,
                    tol,
                )?;
                if !on_curved.rule.agrees(placed, paired.is_some()) {
                    return Err(frontier);
                }
            }
        }
    }
    Ok(())
}

/// Requeues both children of a just-split edge (parent keeps the
/// leading span and its key; `w` is the minted vertex whose emanating
/// half-edge names the trailing child), from face index `next_face`,
/// each carrying `lying_in`.
fn requeue<T: Decide>(
    worklist: &mut std::collections::VecDeque<Pending>,
    x: &Body<T>,
    parent: EdgeKey,
    w: VertexKey,
    next_face: usize,
    lying_in: &[LyingIn],
) -> Result<(), BooleanError> {
    let emanating =
        x.get_vertex(w)
            .and_then(|v| v.emanating)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "split vertex without emanating half-edge",
            })?;
    let child = x.get_half_edge(emanating).map(|h| h.edge).ok_or(
        BooleanError::ClassificationInvariant {
            what: "split child edge unresolvable",
        },
    )?;
    for edge in [parent, child] {
        worklist.push_back(Pending {
            edge,
            start: next_face,
            lying_in: lying_in.to_vec(),
        });
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod minted_rule_rows {
    //! **A vertex minted on an edge lying on a curved face reads under
    //! the arm's own rule** ([`MintedRule`]). Each row pins a refusal arm
    //! against the rule it would read under if the rules were swapped.
    use super::{LyingEnds, MintedRefusal, MintedRule, Placement};
    use geom_core::{Band, Margin, Sign, Tol};

    const CLEAR: MintedRule = MintedRule::Declared {
        interior_clear: true,
    };
    const UNCLEAR: MintedRule = MintedRule::Declared {
        interior_clear: false,
    };
    const INSIDE: MintedRule = MintedRule::LiesOn(LyingEnds::Inside);
    const OUTSIDE: MintedRule = MintedRule::LiesOn(LyingEnds::Outside);
    const PAIRED: MintedRule = MintedRule::LiesOn(LyingEnds::Paired);

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The side `residual` reads at the run's band.
    fn side(rule: MintedRule, residual: f64) -> Result<bool, MintedRefusal> {
        let band = band();
        let decided = geom_core::k_stats::decide_reported(
            "bool_vertex_face_side",
            Margin::of(residual),
            band,
        );
        rule.side(decided, |s| s == Sign::Positive)
            .map(|placed| placed.is_some())
    }

    /// An edge `lying_on` recorded wholly inside the face does not have a
    /// vertex outside it, nor one wholly outside a vertex recorded inside
    /// it, unpaired: two certified answers contradicting. A covered edge
    /// with its interior clear reads either as its own placement.
    #[test]
    fn a_vertex_placed_against_where_the_edge_lies_contradicts_it() {
        assert!(
            !INSIDE.agrees(Placement::Elsewhere, false),
            "inside, elsewhere"
        );
        assert!(
            CLEAR.agrees(Placement::Elsewhere, false),
            "covered, elsewhere"
        );
        assert!(
            !OUTSIDE.agrees(Placement::Recorded, false),
            "outside, recorded"
        );
        assert!(
            CLEAR.agrees(Placement::Recorded, false),
            "covered, recorded"
        );
        assert!(
            OUTSIDE.agrees(Placement::Recorded, true),
            "a pairing is its answer"
        );
        assert!(
            INSIDE.agrees(Placement::Recorded, false),
            "inside, recorded"
        );
        assert!(
            OUTSIDE.agrees(Placement::Elsewhere, false),
            "outside, elsewhere"
        );
        assert!(
            PAIRED.agrees(Placement::Elsewhere, false),
            "paired, elsewhere"
        );
    }

    /// Without the interior certificate a covered edge's vertex placed
    /// elsewhere keeps the door, where the certificate reads it outside.
    #[test]
    fn an_uncertified_interior_keeps_the_door_on_a_vertex_elsewhere() {
        assert!(!UNCLEAR.agrees(Placement::Elsewhere, false));
        assert!(UNCLEAR.agrees(Placement::Recorded, false));
    }

    /// No rule reads an undecided placement.
    #[test]
    fn an_undecided_placement_refuses_under_every_rule() {
        for rule in [CLEAR, UNCLEAR, INSIDE, OUTSIDE, PAIRED] {
            assert!(!rule.agrees(Placement::Undecided, false), "{rule:?}");
        }
    }

    /// Off the carrier on the side the cover admits, a covered edge's
    /// vertex is no incidence; an edge `lying_on` took lies on the
    /// carrier, so its vertex there contradicts it.
    #[test]
    fn a_vertex_off_on_the_admitted_side_is_no_incidence_only_under_a_cover() {
        assert_eq!(side(CLEAR, 1.0), Ok(false), "covered");
        assert_eq!(side(UNCLEAR, 1.0), Ok(false), "covered, uncertified");
        for rule in [INSIDE, OUTSIDE, PAIRED] {
            assert_eq!(side(rule, 1.0), Err(MintedRefusal::Contradicts), "{rule:?}");
        }
    }

    /// Off on the side no cover admits contradicts every rule.
    #[test]
    fn a_vertex_off_on_the_other_side_contradicts_every_rule() {
        for rule in [CLEAR, UNCLEAR, INSIDE, OUTSIDE, PAIRED] {
            assert_eq!(
                side(rule, -1.0),
                Err(MintedRefusal::Contradicts),
                "{rule:?}"
            );
            assert_eq!(
                side(rule, 0.0),
                Ok(true),
                "{rule:?}: on the carrier, placed"
            );
        }
    }

    /// In band refuses as the arm's ends do, under every rule: never no
    /// incidence and never the frontier.
    #[test]
    fn a_side_in_band_refuses_in_band_under_every_rule() {
        // Between the band's zero and escalate thresholds.
        let in_band = (band().zero() * band().escalate()).sqrt();
        for rule in [CLEAR, UNCLEAR, INSIDE, OUTSIDE, PAIRED] {
            assert!(
                matches!(side(rule, in_band), Err(MintedRefusal::InBand(_))),
                "{rule:?}: {:?}",
                side(rule, in_band)
            );
        }
    }
}

#[cfg(test)]
mod undeclared_rule_rows {
    //! **The undeclared `NoInterior` rule, over every placement pair.**
    //! Both undeclared arms — the mixed-sign span and the `(Zero, Zero)`
    //! span — call [`Placement::undeclared_no_interior`], so these rows
    //! hold the rule for both. The row a public fixture cannot reach is
    //! the `Undecided` one: an end on a face whose trim the chart door
    //! declines (a ringed face, a non-rectangular outline) is minted by
    //! no public door that does not refuse first (a notched three-face
    //! wall stops at the volume backstop), so it is held here.
    #![allow(clippy::panic)]

    use super::{CurvedEvent, Placement};

    fn rule(ends: [Option<Placement>; 2]) -> &'static str {
        match Placement::undeclared_no_interior::<f64>(ends) {
            Some(CurvedEvent::Recorded) => "record",
            Some(CurvedEvent::None) => "none",
            Some(
                CurvedEvent::Pierce { .. }
                | CurvedEvent::Deferred
                | CurvedEvent::Lies { .. }
                | CurvedEvent::Along { .. },
            ) => {
                panic!("the rule never pierces or holds")
            }
            None => "door",
        }
    }

    #[test]
    fn an_undecided_end_keeps_the_door_unless_the_other_end_recorded() {
        use Placement::{Elsewhere as E, Recorded as R, Undecided as U};
        let rows = [
            ([Some(R), Some(R)], "record"),
            ([Some(R), Some(E)], "record"),
            ([Some(R), Some(U)], "record"),
            ([Some(E), Some(E)], "none"),
            ([Some(E), Some(U)], "door"),
            ([Some(U), Some(E)], "door"),
            ([Some(U), Some(U)], "door"),
            // The mixed-sign span: one ON end.
            ([Some(R), None], "record"),
            ([None, Some(E)], "none"),
            ([Some(U), None], "door"),
        ];
        for (ends, want) in rows {
            assert_eq!(rule(ends), want, "{ends:?}");
        }
    }

    /// **The declared rule**: an `Undecided` end always keeps the door,
    /// an all-`Elsewhere` pair is no event only beside a certified clear
    /// interior, and an edge with no end on the carrier and no
    /// certificate waits for its fragments.
    #[test]
    fn the_declared_rule_reads_all_elsewhere_by_the_interior_certificate() {
        use Placement::{Elsewhere as E, Recorded as R, Undecided as U};
        let declared =
            |ends: [Option<Placement>; 2], clear| match Placement::declared::<f64>(ends, clear) {
                Some(CurvedEvent::Recorded) => "record",
                Some(CurvedEvent::None) => "none",
                Some(CurvedEvent::Deferred) => "defer",
                Some(
                    CurvedEvent::Pierce { .. }
                    | CurvedEvent::Lies { .. }
                    | CurvedEvent::Along { .. },
                ) => {
                    panic!("the rule never pierces or holds")
                }
                None => "door",
            };
        let rows = [
            ([Some(R), Some(E)], "record", "record"),
            ([Some(R), Some(U)], "door", "door"),
            ([Some(E), Some(E)], "door", "none"),
            ([Some(E), Some(U)], "door", "door"),
            ([Some(R), None], "record", "record"),
            ([None, Some(E)], "door", "none"),
            ([None, None], "defer", "none"),
        ];
        for (ends, unseen, clear) in rows {
            assert_eq!(declared(ends, false), unseen, "{ends:?}, interior unseen");
            assert_eq!(declared(ends, true), clear, "{ends:?}, interior clear");
        }
    }
}

#[path = "factored_crossing.rs"]
pub(super) mod factored_crossing;

#[cfg(test)]
#[path = "coplanar_conic_rows.rs"]
pub(super) mod coplanar_conic_rows;

#[cfg(test)]
#[path = "curved_lane_spline_rows.rs"]
mod curved_lane_spline_rows;
#[cfg(test)]
#[path = "planar_lane_carrier_rows.rs"]
pub(crate) mod planar_lane_carrier_rows;

/// **A curved face's escalations read no declaration ahead of them, and
/// offer none**, on the review's executed raises (its
/// `probe_wall_roots_in_band_with_and_without_declaration`, adopted
/// here): an edge in the side face of a prism against a unit cylinder
/// wall, run through `curved_face_arm` with that side face undeclared,
/// declared `Tangent`, and declared `Rest`.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod declaration_order_rows {
    use super::{ContactAcc, CurvedEvent, FaceContainment, curved_face_arm};
    use crate::boolean::{
        BooleanDecision, BooleanError, Coincide, DeclarationRead, Glue, GluedPair, GluedPairs,
        Operand,
    };
    use crate::contact::GlueClass;
    use crate::entity::VertexKey;
    use crate::test_support::finished;
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, prism_z};
    use geom_core::{Band, Point3, Tol};

    /// Whether the declaration door verifies the side face through the
    /// edge of [`run`]'s prism as `class` against the wall, along the
    /// vertical line `x = profile[3].0`.
    fn door_verifies(class: GlueClass, profile: [(f64, f64); 4], z0: f64) -> bool {
        let tol = Tol::witness();
        let prism = prism_z::<f64>(&profile, z0, z0 + 0.5, tol);
        let mut y: crate::body::Body<f64> = crate::body::Body::new();
        let wall = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let witness = geom::Curve3::Line {
            origin: Point3::new(profile[3].0, 0.0, 0.0),
            dir: geom_core::Vec3::new(0.0, 0.0, 1.0),
        };
        contact_door(
            (&prism.body, prism.side_faces[3]),
            (&y, wall),
            class,
            Some((&witness, z0, z0 + 0.5)),
            Band::linear(tol).expect("the witness band"),
        )
    }

    /// Whether the contact door verifies `x`'s face against `y`'s as
    /// `class`: a `Tangent` pair along `witness`.
    fn contact_door(
        (x, xw): (&crate::body::Body<f64>, crate::entity::FaceKey),
        (y, yw): (&crate::body::Body<f64>, crate::entity::FaceKey),
        class: GlueClass,
        witness: Option<(&geom::Curve3<f64>, f64, f64)>,
        band: Band,
    ) -> bool {
        match class {
            GlueClass::Rest => crate::boolean::rest_pair_verdict(x, xw, y, yw, band).is_ok(),
            GlueClass::Tangent => witness.is_some_and(|(carrier, t0, t1)| {
                crate::boolean::tangent_pair_relation(x, xw, y, yw, carrier, t0, t1, band).is_ok()
            }),
            GlueClass::Continuation => unreachable!("a continuation is no contact"),
        }
    }

    /// The edge from `profile[3]` to `profile[0]` at height `z0` of a
    /// prism over `profile`, against a canonical unit cylinder wall, the
    /// prism's side face through that edge declared against the wall as
    /// `class`; what `curved_face_arm` answers (the review's `run`).
    fn run(
        class: Option<GlueClass>,
        profile: [(f64, f64); 4],
        z0: f64,
    ) -> Result<(), BooleanError> {
        let tol = Tol::witness();
        let b = Band::linear(tol).expect("the witness band");
        let prism = prism_z::<f64>(&profile, z0, z0 + 0.5, tol);
        let x = prism.body;
        let (u, v) = (prism.bottom[3], prism.bottom[0]);
        let pt = |k: VertexKey| {
            *x.get_point(x.get_vertex(k).expect("a live vertex").point)
                .expect("a live point")
        };
        let (edge_key, edge) = x
            .edges()
            .find(|(_, e)| {
                let ends = [e.he_plus, e.he_minus]
                    .map(|he| x.get_half_edge(he).expect("a live half-edge").start);
                ends == [u, v] || ends == [v, u]
            })
            .map(|(k, e)| (k, e.clone()))
            .expect("the bottom edge");
        let (a, c) = (
            x.get_half_edge(edge.he_plus)
                .expect("a live half-edge")
                .start,
            x.get_half_edge(edge.he_minus)
                .expect("a live half-edge")
                .start,
        );
        let mut y: crate::body::Body<f64> = crate::body::Body::new();
        let wall = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            (-0.5, 0.5),
            (0.0, 1.0),
            tol,
        );
        let decls = Glue {
            pairs: class
                .map(|class| vec![GluedPair::new(prism.side_faces[3], wall, class)])
                .unwrap_or_default(),
            ..Glue::none()
        };
        let declared = GluedPairs::<f64>::without_struts(&decls, Default::default());
        let mut acc = ContactAcc::default();
        curved_face_arm(
            &x,
            &mut y,
            Operand::A,
            edge_key,
            &edge,
            a,
            c,
            wall,
            pt(a),
            pt(c),
            &declared,
            &mut acc,
            b,
            tol,
        )
        .map(|_: CurvedEvent<f64>| ())
    }

    /// The one refusal every declaration posture gives: the coincidence
    /// `which`, with what the lookup read of the pair (nothing, or the
    /// declared class spent), and the same sentence, which offers no
    /// declaration.
    fn same_refusal_without_declare(
        runs: &[(Option<GlueClass>, Result<(), BooleanError>)],
        which: Coincide,
        subject: &str,
    ) {
        for (class, got) in runs {
            let Err(err) = got else {
                panic!("{class:?}: the in-band arm refuses");
            };
            let BooleanError::Escalated { decision, .. } = err else {
                panic!("{class:?}: an escalation: {err:?}");
            };
            assert_eq!(
                *decision,
                BooleanDecision::Coincidence(
                    which,
                    class.map_or(DeclarationRead::Moot, DeclarationRead::Spent)
                ),
                "{class:?}"
            );
            let text = err.to_string();
            assert!(
                text.starts_with(&format!("{subject} is undecided: ")) && !text.contains("declare"),
                "{class:?}: no declaration settles the question, so none is offered: {text}"
            );
            assert_eq!(
                text,
                runs[0]
                    .1
                    .as_ref()
                    .expect_err("the undeclared run refuses")
                    .to_string(),
                "{class:?}: a declaration changes nothing here"
            );
        }
    }

    /// **The `(Positive, Positive)` line-clearance arm**: a line clear of
    /// the wall by an in-band distance, both ends definitely outside, so
    /// the clearance bound escalates before any declaration is read.
    /// The declaration door takes none at this pose (a plane an in-band
    /// gap off the wall escalates `Tangent`, and `Rest` names two kinds
    /// of surface), so the declared run is built past the door and shows
    /// only that the arm reads none. Before the review's fix it offered
    /// "declare the coincidence".
    #[test]
    fn an_in_band_line_clearance_offers_no_declaration() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let x0 = 1.0 + (b.zero() + b.escalate()) / 2.0;
        let profile = [(x0, -0.01), (x0 + 1.0, -0.01), (x0 + 1.0, 0.01), (x0, 0.01)];
        assert!(
            !door_verifies(GlueClass::Tangent, profile, 0.25),
            "no door takes the in-band plane's Tangent"
        );
        let runs: Vec<_> = [None, Some(GlueClass::Tangent)]
            .into_iter()
            .map(|class| (class, run(class, profile, 0.25)))
            .collect();
        same_refusal_without_declare(
            &runs,
            Coincide::EdgeOnCurvedFace,
            "whether an edge of one solid clears a curved face of the other or lies on it",
        );
    }

    /// **The curved arm's endpoint side**: a line in the plane `x = 1`
    /// that touches the wall exactly, one end off the tangency by the
    /// distance whose residual `y²/2` lies in the band. The endpoint's
    /// side escalates ahead of the declared-cover rungs, so a `Tangent`
    /// declaration of that plane, which the door verifies definite
    /// (`rest_pair_verdict`), changes nothing. Before the review's fix
    /// it offered "declare the coincidence".
    #[test]
    fn an_in_band_endpoint_on_a_curved_face_offers_no_declaration() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let y0 = (b.zero() + b.escalate()).sqrt();
        let profile = [(1.0, y0), (2.0, y0), (2.0, 2.0), (1.0, 2.0)];
        assert!(
            door_verifies(GlueClass::Tangent, profile, 0.25),
            "the tangent plane's declaration is one the door takes"
        );
        let runs: Vec<_> = [None, Some(GlueClass::Tangent)]
            .into_iter()
            .map(|class| (class, run(class, profile, 0.25)))
            .collect();
        same_refusal_without_declare(
            &runs,
            Coincide::VertexOnCurvedFace,
            "whether a vertex of one solid lies on a face of the other",
        );
    }

    /// The decision `err` escalated on, or a panic naming `label`.
    fn decision_of<E: core::fmt::Debug>(
        label: &str,
        got: &Result<E, BooleanError>,
    ) -> BooleanDecision {
        match got {
            Err(BooleanError::Escalated { decision, .. }) => *decision,
            other => panic!("{label}: an escalation: {other:?}"),
        }
    }

    /// Each circle edge of a sheet of the cylinder `frame` over
    /// `angles` (heights 0.25 to 0.5), its sense reversed, through
    /// `curved_face_arm` against the canonical unit wall over angles
    /// (0, 3), the sheet offered to the door against the wall as `class`
    /// and glued only where the door verifies it; with what the door
    /// answers `class` on the pair.
    ///
    /// Adopted from the review of PR 3513's fix pass
    /// (`zz_coincfr_rows.rs`, its `run`).
    fn circle_run(
        frame: CylFrame,
        angles: (f64, f64),
        class: Option<GlueClass>,
    ) -> (Option<bool>, Vec<Result<CurvedEvent<f64>, BooleanError>>) {
        let tol = Tol::witness();
        let b = Band::linear(tol).expect("the witness band");
        let mut x: crate::body::Body<f64> = crate::body::Body::new();
        let xw = cyl_wall_sheet(&mut x, frame, angles, (0.25, 0.5), tol);
        let sense = x.get_face(xw).expect("the sheet's wall").sense;
        x.set_face_sense(xw, !sense).expect("a live face");
        let mut y: crate::body::Body<f64> = crate::body::Body::new();
        let yw = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            (0.0, 3.0),
            (0.0, 1.0),
            tol,
        );
        crate::test_support_fixtures::set_sheet_sense(&mut y, yw, true, tol);
        let door = class.map(|class| contact_door((&x, xw), (&y, yw), class, None, b));
        // The index holds what the door glued, and nothing it refused.
        let glued = class.filter(|_| door == Some(true));
        let decls = Glue {
            pairs: glued
                .map(|class| vec![GluedPair::new(xw, yw, class)])
                .unwrap_or_default(),
            ..Glue::none()
        };
        let one = glued
            .filter(|class| class.is_one_carrier())
            .map(|_| (xw, yw))
            .into_iter()
            .collect();
        let declared = GluedPairs::<f64>::without_struts(&decls, one);
        let edges: Vec<_> = x.edges().map(|(k, e)| (k, e.clone())).collect();
        let mut out = Vec::new();
        for (edge_key, edge) in edges {
            let Some(super::CurveGeom::Certified(c)) = x.get_curve_geom(edge.curve).cloned() else {
                continue;
            };
            if !matches!(c.carrier(), geom::Curve3::Circle { .. }) {
                continue;
            }
            let start = |he| x.get_half_edge(he).expect("a live half-edge").start;
            let (a, c) = (start(edge.he_plus), start(edge.he_minus));
            let pt = |k: VertexKey| {
                *x.get_point(x.get_vertex(k).expect("a live vertex").point)
                    .expect("a live point")
            };
            let mut acc = ContactAcc::default();
            out.push(curved_face_arm(
                &x,
                &mut y,
                Operand::A,
                edge_key,
                &edge,
                a,
                c,
                yw,
                pt(a),
                pt(c),
                &declared,
                &mut acc,
                b,
                tol,
            ));
        }
        assert_eq!(out.len(), 2, "the sheet's two rim circles");
        (door, out)
    }

    /// **The uncovered circle's clearance offers no declaration**, on
    /// the review's executed raise: a sheet of radius `1 + mid` against
    /// the unit wall. The escalated clearance hands the arc to the root
    /// lane as a sphere's or a torus's does, and the endpoint sides that
    /// guard it read the same in-band pose. The door reads the pair in
    /// band and glues neither `Rest` nor `Tangent`, so whatever the pair
    /// is offered as, the circle is uncovered and its clearance refuses as
    /// the unoffered one's does. No posture passes, so no refusal offers a
    /// declaration, and each states what its door read.
    #[test]
    fn an_uncovered_circle_clearance_offers_no_declaration() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let frame = CylFrame::canonical(1.0 + (b.zero() + b.escalate()) / 2.0);
        for class in [None, Some(GlueClass::Rest), Some(GlueClass::Tangent)] {
            let label = format!("{class:?}");
            let (verdict, runs) = circle_run(frame, (0.5, 1.0), class);
            assert_eq!(
                verdict,
                class.map(|_| false),
                "{label}: the door glues nothing in band"
            );
            for got in &runs {
                assert_eq!(
                    decision_of(&label, got),
                    BooleanDecision::Coincidence(
                        Coincide::VertexOnCurvedFace,
                        DeclarationRead::Moot
                    ),
                    "{label}"
                );
                let text = got.as_ref().expect_err("it refuses").to_string();
                assert!(!text.contains("declare"), "{label}: {text}");
            }
        }
    }

    /// **The circle rung's decided arms, under each declaration**
    /// (`curved-pierce-frontier-tells-one-story-for-several-decisions`):
    /// its `Zero` arm, a sheet of radius `1 + zero/2`, refuses at the
    /// frontier undeclared and is recorded once a `Rest` declaration the
    /// door verifies calls the two one carrier, so the frontier's
    /// declare offer is true there; its `Negative` arm, a rim that
    /// crosses the wall (a sheet whose axis is offset by 0.3), takes no
    /// declaration either: the door refuses `Rest` (the axes are apart).
    /// Undeclared it is the circle × cylinder root lane's pierce, at the
    /// two unit circles' meeting point `(0.15, √(1 − 0.15²))`, and so it
    /// is under a `Rest` or `Tangent` the door does not verify, which it
    /// leaves unglued.
    #[test]
    fn the_circle_rungs_zero_arm_takes_a_declaration_and_its_crossing_arm_takes_none() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let frontier = |label: &str, got: &Result<CurvedEvent<f64>, BooleanError>| {
            assert!(
                matches!(got, Err(BooleanError::CurvedPierceUnsupported { .. })),
                "{label}: the frontier: {got:?}"
            );
        };
        let zero = CylFrame::canonical(1.0 + 0.5 * b.zero());
        for got in circle_run(zero, (0.5, 1.0), None).1 {
            frontier("zero arm, undeclared", &got);
        }
        let (door, runs) = circle_run(zero, (0.5, 1.0), Some(GlueClass::Rest));
        assert_eq!(door, Some(true), "the door takes the zero arm's Rest");
        for got in runs {
            assert!(got.is_ok(), "zero arm, declared Rest: recorded: {got:?}");
        }
        let crossing = CylFrame {
            origin: Point3::new(0.3, 0.0, 0.0),
            ..CylFrame::canonical(1.0)
        };
        for class in [None, Some(GlueClass::Rest), Some(GlueClass::Tangent)] {
            let (door, runs) = circle_run(crossing, (1.5, 2.5), class);
            assert_ne!(
                door,
                Some(true),
                "{class:?}: the door takes no declaration here"
            );
            let mut heights = Vec::new();
            for got in &runs {
                let Ok(CurvedEvent::Pierce { p, at, .. }) = got else {
                    panic!("crossing arm, {class:?}: a pierce, got {got:?}");
                };
                assert_eq!(
                    *at,
                    FaceContainment::In,
                    "{class:?}: inside the wall's trim"
                );
                let off = (p.x - 0.15).hypot(p.y - (1.0_f64 - 0.15 * 0.15).sqrt());
                assert!(off < 1e-12, "{class:?}: the pierce {p:?} is {off} off");
                heights.push(p.z);
            }
            heights.sort_by(f64::total_cmp);
            assert!(
                heights.len() == 2
                    && (heights[0] - 0.25).abs() < 1e-12
                    && (heights[1] - 0.5).abs() < 1e-12,
                "{class:?}: one pierce per rim, got heights {heights:?}"
            );
        }
    }

    /// **The circle × torus lane escalates as its own decision, and no
    /// declaration settles it, on the geometry of its raise.** An arc
    /// of a level circle an in-band depth under the top of a torus's
    /// tube (a parallel of a torus sheet whose centre stands 0.3 off the
    /// other's axis), with both ends
    /// definitely outside the tube and its middle over the tube's top:
    /// through `curved_face_arm` the clearance does not decide, the ends
    /// do, and the root lane asked of the arc escalates on its plane
    /// height. The lane's rungs pass on different sets and units, and the
    /// escalation names its predicate only, so it ends on its lever alone.
    /// The door takes neither class for that face pair: `Rest` meets two
    /// tori whose centres are apart, and `Tangent` has no witness for a
    /// torus and no shared rim.
    #[test]
    fn the_circle_torus_lane_escalates_as_its_own_decision_and_no_declaration_settles_it() {
        use crate::boolean::boxes::tests::torus_wall;
        let tol = Tol::witness();
        let b = Band::linear(tol).expect("the witness band");
        let mid = (b.zero() + b.escalate()) / 2.0;
        let (axis, u_ref) = (
            geom_core::Vec3::new(0.0, 0.0, 1.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
        );
        // The parallel at tube angle `asin(1 − 2·mid)` stands `mid`
        // under the top of a tube of radius ½, just inside the tube's
        // crown, where the sheet still certifies.
        let (x, xw) = torus_wall(
            Point3::new(0.3, 0.0, 0.0),
            axis,
            u_ref,
            2.0,
            0.5,
            (1.2, 2.1),
            (0.0, (1.0 - 2.0 * mid).asin()),
        );
        let (mut y, yw) = torus_wall(
            Point3::new(0.0, 0.0, 0.0),
            axis,
            u_ref,
            2.0,
            0.5,
            (0.0, 3.0),
            (0.0, 2.0),
        );
        for class in [GlueClass::Rest, GlueClass::Tangent] {
            let door = match class {
                GlueClass::Tangent => {
                    crate::boolean::verify_tangent_declaration(&x, xw, &y, yw, b).is_ok()
                }
                _ => contact_door((&x, xw), (&y, yw), class, None, b),
            };
            assert!(
                !door,
                "{class:?}: the door takes no declaration of this pair"
            );
        }
        let (edge_key, edge) = x
            .edges()
            .find(|(_, e)| {
                matches!(
                    x.get_curve_geom(e.curve),
                    Some(super::CurveGeom::Certified(g))
                        if matches!(
                            g.carrier(),
                            geom::Curve3::Circle { axis: a, center, .. }
                                if a.z.abs() > 0.9 && center.z > 0.1
                        )
                )
            })
            .map(|(k, e)| (k, e.clone()))
            .expect("the sheet's top parallel");
        let start = |he| x.get_half_edge(he).expect("a live half-edge").start;
        let (a, c) = (start(edge.he_plus), start(edge.he_minus));
        let pt = |k: VertexKey| {
            *x.get_point(x.get_vertex(k).expect("a live vertex").point)
                .expect("a live point")
        };
        let declared = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default());
        let mut acc = ContactAcc::default();
        let got = curved_face_arm(
            &x,
            &mut y,
            Operand::A,
            edge_key,
            &edge,
            a,
            c,
            yw,
            pt(a),
            pt(c),
            &declared,
            &mut acc,
            b,
            tol,
        )
        .map(|_: CurvedEvent<f64>| ())
        .expect_err("the arc over the tube's top escalates");
        let BooleanError::CurvedPierceEscalated {
            operand: Operand::A,
            face,
            edge: at,
            decision,
            diag,
        } = &got
        else {
            panic!("an escalation, located: {got:?}");
        };
        assert_eq!((*face, *at), (yw, edge_key), "at the arc and the tube");
        assert_eq!(
            *decision,
            BooleanDecision::ArcTorusRoots,
            "the lane's own decision"
        );
        assert_eq!(diag.predicate, Some("bool_circle_torus_plane_height"));
        let text = got.to_string();
        assert!(
            text.starts_with(
                "at an edge of the first operand against a curved face of the other, how \
                 many times an arc crosses a torus is undecided: "
            ) && text.ends_with(
                "Recourse: move the parts so the arc clearly crosses the torus or clearly \
                     misses it"
            ),
            "{text}"
        );
    }

    /// **A coplanar sector's in-band parallelism refuses on the sector**,
    /// on a union: a parallelepiped cornered at a point of a block's top
    /// face by a 5° wedge angle, its face there tilted by an in-band
    /// angle about the wedge's one edge, so the wedge's two edges read
    /// on that face while its normal reads in-band parallel at the
    /// corner's arm. Two poses: standing on the block, its bottom face
    /// opposed to the block's top; and sunk into it, its top face flush
    /// with the block's top and aligned with it. The sector names the
    /// one-carrier class the senses make the pair (`Rest` opposed, a
    /// continuation aligned), and its recourse is the geometry or the
    /// tolerance, where that lies above `EPS_MIN` (D4 ¶4).
    #[test]
    fn a_coplanar_sectors_in_band_parallelism_refuses_on_the_sector() {
        use crate::test_support_fixtures::{brick, mapped_cube};
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let theta = (band.zero() + band.escalate()) / 2.0;
        let phi = 5.0_f64.to_radians();
        let (ea, eb) = (
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()),
        );
        let p = Point3::new(0.5, 0.2, 1.0);
        let block = finished(
            "block",
            brick((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol),
            tol,
        );
        let poses: [(&'static str, crate::body::Body<f64>, GlueClass); 2] = [
            (
                "standing on the block",
                mapped_cube::<f64>(
                    move |u, v, w| p + ea * u + eb * v + geom_core::Vec3::new(0.0, 0.0, w),
                    tol,
                ),
                GlueClass::Rest,
            ),
            (
                "sunk into the block",
                mapped_cube::<f64>(
                    move |u, v, w| {
                        p + ea * u + eb * v + geom_core::Vec3::new(0.0, 0.0, 0.5 * (w - 1.0))
                    },
                    tol,
                ),
                GlueClass::Continuation,
            ),
        ];
        for (label, wedge, class) in poses {
            let wedge = finished(label, wedge, tol);
            let got = crate::boolean::union(&block, &wedge, tol).map(|_| ());
            assert!(
                matches!(
                    decision_of(label, &got),
                    BooleanDecision::Coincidence(Coincide::Sectors, DeclarationRead::Settles(s))
                        if s.class() == class
                ),
                "{label}: the sector names {class:?}, as the senses make the pair: {got:?}"
            );
            let Err(BooleanError::Escalated { diag, .. }) = &got else {
                panic!("{label}: an escalation: {got:?}");
            };
            let m = diag
                .margin
                .diagnostic_f64_for_error_text()
                .value()
                .expect("a point margin");
            let offers = band.offer_deciding(m).is_some();
            let text = got.expect_err("it refuses").to_string();
            assert!(
                text.starts_with("how two corners of the two solids overlap where they meet is ")
                    && text.contains(
                        "Recourse: move the parts so they clearly meet or clearly stand apart \
                         there"
                    )
                    && text.contains(", or, if this gap is intended, tighten the tolerance below ")
                        == offers
                    && !text.contains("declare"),
                "{label}: offered exactly above EPS_MIN ({offers}): {text}"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod wall_root_tests {
    use super::{BooleanDecision, BooleanError, line_wall_roots_of};
    use crate::boolean::WallRung;
    use geom_core::{Band, Point3, Tol, Vec3};

    /// **The line × wall root lane escalates as its own decision, on a
    /// real raise**: a line across the axis of a unit cylinder, at the
    /// distance from it that puts the discriminant's depth `(r² − d²)/2r`
    /// in the band. The refusal names the rung's question and the one
    /// lever that reaches it, no tolerance (the arms that read it pass on
    /// different sets) and no declaration: no face pair says where an
    /// edge crosses a wall.
    #[test]
    fn the_wall_root_lane_escalates_as_its_own_decision() {
        let b = Band::linear(Tol::witness()).expect("the witness band");
        let mid = (b.zero() + b.escalate()) / 2.0;
        let d = (1.0 - 2.0 * mid).sqrt();
        let wall = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let got = line_wall_roots_of(
            Point3::new(d, -2.0, 0.5),
            Vec3::new(0.0, 1.0, 0.0),
            (0.0, 4.0),
            &wall,
            b,
        );
        let Err(err) = got else {
            panic!("an in-band discriminant escalates");
        };
        let BooleanError::Escalated { decision, diag } = err else {
            panic!("the lane escalates: {err:?}");
        };
        assert_eq!(decision, BooleanDecision::WallRoots(WallRung::Discriminant));
        assert_eq!(diag.predicate, Some("bool_ray_cylinder_disc"));
        let text = BooleanError::Escalated { decision, diag }.to_string();
        assert_eq!(test_utils::refusal::recourse_markers(&text), 1, "{text}");
        assert_eq!(
            text,
            format!(
                "whether an edge crosses a cylinder wall, grazes it or misses it is undecided: \
                 {}. Recourse: move the parts so the edge clearly crosses the wall or clearly \
                 misses it",
                diag.payload()
            )
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod edge_span_tests {
    use super::{SpanVerdict, wall_crossing};
    use crate::{Body, entity::FaceKey};
    use geom_core::{Band, Point3, Tol, Vec3};

    /// **The axis-parallel rung is levered by the edge's own span.** A
    /// line outside a unit wall, drifting off its distance from the axis
    /// by `δ` per unit of its parameter, over two runs: `[−2, 2]`, where
    /// `4δ` is past the band, so the roots are found and lie far outside
    /// the run; and `[1000, 1001]`, where `δ` is inside the zero band, so
    /// the residual is constant over the run. A lever of `1` reads the
    /// first in band; a lever of the far end, or of the run's distance
    /// from the line's origin, reads the second past it.
    #[test]
    fn the_axis_parallel_rung_reads_the_edges_own_span() {
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let wall = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let y = Body::<f64>::new();
        let crossing = |delta: f64, (t0, t1): (f64, f64)| {
            let line = geom::Curve3::Line {
                origin: Point3::new(2.0, 0.0, 0.0),
                dir: Vec3::new(delta, 0.0, 1.0),
            };
            wall_crossing(
                &y,
                FaceKey::default(),
                &wall,
                &line,
                t0,
                t1,
                band,
                &super::door_escalates,
            )
        };
        let long = crossing(0.4 * band.escalate(), (-2.0, 2.0));
        assert!(
            matches!(long, Ok(SpanVerdict::NoInterior)),
            "drifting past the band over [−2, 2], the roots are found outside it: {long:?}"
        );
        let far = crossing(0.5 * band.zero(), (1000.0, 1001.0));
        assert!(
            matches!(far, Ok(SpanVerdict::Constant)),
            "drifting inside the band over [1000, 1001], the residual is constant: {far:?}"
        );
    }
    /// **A root's distance from an end is metered at the carrier's speed
    /// THERE.** An ellipse of semi-axes 1 and 0.05 is crossed square by a
    /// wall at `θ = π/2 − 2e-8`, 2e-8 m of arc short of the span's end
    /// `π/2` of the span `[π/2 − 0.1, π/2]` (the wall's other crossings lie
    /// outside it), where the carrier runs at 1 m/rad — past the escalation
    /// threshold, so the root is interior and goes on to the trim, a
    /// half-wall sheet holding it: a pierce. Metered at the
    /// semi-minor axis, the least speed, the same gap reads 1e-9 m, inside
    /// the zero band: the root read as the end's own incidence, and the
    /// span `NoInterior`.
    #[test]
    fn a_root_near_an_end_is_metered_at_the_ends_speed() {
        let band = Band::new(1e-9, 1e-8).expect("a band");
        let ellipse = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: 1.0,
            semi_v: 0.05,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let root = core::f64::consts::FRAC_PI_2 - 2e-8;
        let p = ellipse.eval(root);
        let mut y = Body::<f64>::new();
        let face = crate::test_support_fixtures::cyl_wall_sheet(
            &mut y,
            crate::test_support_fixtures::CylFrame {
                origin: Point3::new(p.x + 0.3, p.y, 0.0),
                axis: Vec3::unit_z(),
                radius: 0.3,
                u_ref: Vec3::unit_x(),
            },
            (
                core::f64::consts::FRAC_PI_2,
                3.0 * core::f64::consts::FRAC_PI_2,
            ),
            (-1.0, 1.0),
            Tol::witness(),
        );
        crate::test_support_fixtures::set_sheet_sense(&mut y, face, true, Tol::witness());
        let wall = y
            .get_surface(y.get_face(face).expect("the sheet's face").surface)
            .expect("the sheet's wall")
            .clone();
        let got = wall_crossing(
            &y,
            face,
            &wall,
            &ellipse,
            core::f64::consts::FRAC_PI_2 - 0.1,
            core::f64::consts::FRAC_PI_2,
            band,
            &super::door_escalates,
        );
        assert!(
            matches!(got, Ok(SpanVerdict::Pierce { .. })),
            "the root is interior, and the half-wall's trim holds it: {got:?}"
        );
    }

    /// **A spline span with no root lane is localized against the face,
    /// not refused.** The quadratic in the plane `z = 1/2` through the
    /// unit cylinder's point at azimuth `−1/2`, with offset
    /// `(2s − 1)²/10` along its normal, touches the wall there from
    /// outside, past the quarter sheet over azimuths `[0, π/2]`: no root
    /// door reads a spline, so the touch is the carrier-touch door's,
    /// and it lies off the face.
    #[test]
    fn a_spline_touch_past_the_face_is_off_it() {
        use geom_core::spline::{KnotVector, Knots};
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let mut y = Body::<f64>::new();
        let frame = crate::test_support_fixtures::CylFrame::canonical(1.0);
        let face = crate::test_support_fixtures::cyl_wall_sheet(
            &mut y,
            frame,
            (0.0, core::f64::consts::FRAC_PI_2),
            (0.0, 1.0),
            Tol::witness(),
        );
        crate::test_support_fixtures::set_sheet_sense(&mut y, face, true, Tol::witness());
        let (sin, cos) = (-0.5_f64).sin_cos();
        let (p, n, t) = (
            Point3::new(cos, sin, 0.5),
            Vec3::new(cos, sin, 0.0),
            Vec3::new(-sin, cos, 0.0),
        );
        let knots =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("clamped knots");
        let spline = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(knots),
                vec![p + n * 0.1 - t * 0.3, p - n * 0.1, p + n * 0.1 + t * 0.3],
                vec![1.0; 3],
            )
            .expect("a quadratic Bézier"),
        ));
        let got = wall_crossing(
            &y,
            face,
            &frame.surface(),
            &spline,
            0.0,
            1.0,
            band,
            &super::door_escalates,
        );
        assert!(
            matches!(got, Ok(SpanVerdict::OffFace)),
            "the touch lies past the sheet: {got:?}"
        );
    }
}

#[cfg(test)]
mod no_pierce_tests {
    use super::{SpanVerdict, no_pierce_verdict};

    /// **A straddle with no root in the span keeps the door.** Only an
    /// interior root placed off this face, with no root at an end,
    /// accounts for a straddle's crossing; every other combination is
    /// `NoInterior`, which the straddle arm refuses on.
    #[test]
    fn only_an_interior_root_off_the_face_is_elsewhere() {
        for (crossed, at_end, want_elsewhere) in [
            (true, false, true),
            (false, false, false),
            (true, true, false),
            (false, true, false),
        ] {
            let got = no_pierce_verdict::<f64>(crossed, at_end);
            assert_eq!(
                matches!(got, SpanVerdict::Elsewhere),
                want_elsewhere,
                "crossed_elsewhere {crossed}, at_end {at_end}: {got:?}"
            );
        }
    }
}

/// **The two certificates of [`lying_on`], held on their predicates.**
/// Each row poses one boundary shape the end-to-end fixtures never
/// reach: a body that would reach it builds no differently whichever
/// way the certificate answers, or no public door mints it. The faces
/// are a unit cylinder sheet (arcs at its ends, rulings at its sides)
/// and the `y = 0` side of a unit prism (four lines); the predicate
/// reads only the face's boundary, never its carrier.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod lying_on_rows {
    use super::{
        arc_chain_reaches, boundary_meets_circle_only_at, parents_distinct_from,
        split_other_at_point,
    };
    use crate::body::Body;
    use crate::boolean::Operand;
    use crate::entity::{EdgeKey, FaceKey, VertexKey};
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, prism_z};
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
    use geom_core::{Band, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    /// A unit-radius sheet about `+z` over azimuth `[0, u1]`, `z ∈ [0, 1]`.
    fn sheet(u1: f64) -> (Body<f64>, FaceKey) {
        let mut y = Body::new();
        let face = cyl_wall_sheet(
            &mut y,
            CylFrame::canonical(1.0),
            (0.0, u1),
            (0.0, 1.0),
            Tol::witness(),
        );
        (y, face)
    }

    /// The unit prism's `y = 0` side face, and its vertices `(0,0,0)`,
    /// `(1,0,0)`, `(1,0,1)`, `(0,0,1)`.
    fn side() -> (Body<f64>, FaceKey, [VertexKey; 4]) {
        let p = prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.0,
            1.0,
            Tol::witness(),
        );
        let v = [p.bottom[0], p.bottom[1], p.top[1], p.top[0]];
        (p.body, p.side_faces[0], v)
    }

    fn vertex_at(y: &Body<f64>, p: Point3<f64>) -> VertexKey {
        y.vertex_points()
            .find(|(_, q)| (*q - p).norm() < 1e-12)
            .map(|(k, _)| k)
            .unwrap_or_else(|| panic!("a vertex at {p:?}"))
    }

    fn edge_between(y: &Body<f64>, a: VertexKey, b: VertexKey) -> EdgeKey {
        y.edges()
            .find(|(_, e)| {
                let ends = [e.he_plus, e.he_minus].map(|h| y.get_half_edge(h).unwrap().start);
                ends == [a, b] || ends == [b, a]
            })
            .map(|(k, _)| k)
            .expect("an edge between the two vertices")
    }

    fn meets(
        y: &Body<f64>,
        face: FaceKey,
        (center, axis, radius): (Point3<f64>, Vec3<f64>, f64),
        at: &[VertexKey],
    ) -> bool {
        boundary_meets_circle_only_at(y, face, (center, axis, radius), at, band())
            .expect("a walkable boundary")
    }

    /// A conic edge crossing the circle's plane ON the circle meets it
    /// there; the same crossing off the circle does not. The half sheet's
    /// arcs cross `x = 0` at `(0, 1, 0)` and `(0, 1, 1)`, both on the
    /// circle of radius `1/2` about `(0, 1, 1/2)`.
    #[test]
    fn a_conic_crossing_on_the_circle_is_a_meeting() {
        let (y, face) = sheet(PI);
        let on = (Point3::new(0.0, 1.0, 0.5), Vec3::unit_x(), 0.5);
        let off = (Point3::new(0.0, 1.0, 0.5), Vec3::unit_x(), 0.25);
        assert!(
            !meets(&y, face, on, &[]),
            "the arcs cross the plane on the circle"
        );
        assert!(
            meets(&y, face, off, &[]),
            "and the same crossings off it are clear"
        );
    }

    /// A conic lying in the circle's plane does not certify, even ON
    /// the circle between two of `at`: the sheet's bottom arc is the
    /// circle itself.
    #[test]
    fn a_conic_in_the_plane_does_not_certify() {
        let (y, face) = sheet(FRAC_PI_2);
        let at = [
            vertex_at(&y, Point3::new(1.0, 0.0, 0.0)),
            vertex_at(&y, Point3::new(0.0, 1.0, 0.0)),
        ];
        let circle = (Point3::origin(), Vec3::unit_z(), 1.0);
        assert!(
            !meets(&y, face, circle, &at),
            "the bottom arc lies on the circle"
        );
        let lifted = (Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), 2.0);
        assert!(
            meets(&y, face, lifted, &[]),
            "a parallel plane between the arcs, its circle off the rulings, is clear"
        );
    }

    /// A line lying in the circle's plane certifies only as a chord
    /// between two of `at` whose midpoint is decided off the circle. The
    /// prism side's edge `x = 1` lies in the plane `x = 1`; the circles
    /// are in that plane.
    #[test]
    fn a_line_in_the_plane_certifies_only_as_a_chord_decided_off_the_circle() {
        let (y, face, v) = side();
        let crossed = (Point3::new(1.0, 0.0, 0.5), Vec3::unit_x(), 0.2);
        assert!(
            !meets(&y, face, crossed, &[]),
            "the edge crosses the circle twice"
        );
        let grazed = (Point3::new(1.0, 0.3, 0.5), Vec3::unit_x(), 0.3);
        assert!(
            !meets(&y, face, grazed, &[v[1], v[2]]),
            "between two of `at`, but touching the circle at its midpoint"
        );
        let chord = (Point3::new(1.0, 0.0, 0.5), Vec3::unit_x(), 0.5);
        assert!(
            meets(&y, face, chord, &[v[1], v[2]]),
            "a chord of the circle is clear"
        );
    }

    /// A vertex whose side of the plane escalates is placed nowhere,
    /// even when the line to it leaves the plane at once: its real
    /// crossing can lie `|h| / sin θ` away. The plane is tilted through
    /// `(1, 0, 0)` and offset from it in band.
    #[test]
    fn a_vertex_in_band_of_the_plane_does_not_certify() {
        let (y, face, _) = side();
        let b = band();
        let n = Vec3::new(1.0, 0.0, 0.1).normalize();
        let h = (b.zero() + b.escalate()) / 2.0;
        let near = (Point3::new(1.0, 0.0, 0.0) + n * h, n, 5.0);
        assert!(!meets(&y, face, near, &[]), "the corner's side escalates");
        let clear = (Point3::new(1.0, 0.0, 0.0) + n * 0.25, n, 5.0);
        assert!(
            meets(&y, face, clear, &[]),
            "the same plane off the corner is clear"
        );
    }

    /// An edge the certificate cannot place does not certify: the prism
    /// side's top edge re-described as a degree-1 NURBS line.
    #[test]
    fn a_nurbs_boundary_edge_does_not_certify() {
        let (mut y, face, v) = side();
        let plane = (Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), 5.0);
        assert!(
            meets(&y, face, plane, &[]),
            "the line-bounded face is clear"
        );
        let top = edge_between(&y, v[2], v[3]);
        let (p0, p1) = (Point3::new(1.0, 0.0, 1.0), Point3::new(0.0, 0.0, 1.0));
        let plus_start = y
            .get_half_edge(y.get_edge(top).unwrap().he_plus)
            .unwrap()
            .start;
        let (p0, p1) = if plus_start == v[2] {
            (p0, p1)
        } else {
            (p1, p0)
        };
        let (s1, s2) = crate::readback::edge_sides(&y, top).unwrap().surfaces();
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ));
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair: geom_brep::SurfacePair::new(s1, s2),
                selector: geom_brep::CurveSelector::Witness(p0.lerp(p1, 0.5)),
            },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        };
        y.set_edge_curve(top, spec, Tol::witness())
            .expect("the NURBS line attaches");
        assert!(!meets(&y, face, plane, &[]), "a NURBS edge is not placed");
    }

    /// An arc's parents count as distinct from a face's carrier only on
    /// a definite ladder `Distinct`. The sheet's bottom arc lies on a
    /// second unit sheet's cylinder, its own parent's carrier: undeclared,
    /// that is no decision. Against a radius-2 sheet it is `Distinct`.
    /// (Through the boolean, the glue door glues a same-carrier pair as a
    /// continuation before the crossing layer, so only this row holds the
    /// guard.)
    #[test]
    fn only_a_decided_distinct_parent_licenses_the_arc() {
        let (x, _) = sheet(FRAC_PI_2);
        let bottom = edge_between(
            &x,
            vertex_at(&x, Point3::new(1.0, 0.0, 0.0)),
            vertex_at(&x, Point3::new(0.0, 1.0, 0.0)),
        );
        let edge = x.get_edge(bottom).unwrap();
        let (same, same_face) = sheet(FRAC_PI_2);
        assert!(
            !parents_distinct_from(&x, edge, &same, same_face, band()),
            "one carrier, undeclared: no decision"
        );
        let mut wide = Body::new();
        let wide_face = cyl_wall_sheet(
            &mut wide,
            CylFrame::canonical(2.0),
            (0.0, FRAC_PI_2),
            (0.0, 1.0),
            Tol::witness(),
        );
        assert!(
            parents_distinct_from(&x, edge, &wide, wide_face, band()),
            "a radius-2 wall is decided distinct"
        );
    }

    /// A chain of circle arcs leaving along the tangent reaches its end
    /// only through vertices decided on the circle. The quarter sheet's
    /// bottom arc, split at 45°, leaves `(1, 0, 0)` along `+y`, as the
    /// circle of radius 2 about `(−1, 0, 0)` does; its middle vertex is
    /// off that circle, and on the sheet's own.
    #[test]
    fn a_chain_vertex_off_the_circle_breaks_the_chain() {
        let (mut y, _) = sheet(FRAC_PI_2);
        let (from, to) = (
            vertex_at(&y, Point3::new(1.0, 0.0, 0.0)),
            vertex_at(&y, Point3::new(0.0, 1.0, 0.0)),
        );
        let bottom = edge_between(&y, from, to);
        let mid = Point3::new(FRAC_PI_4.cos(), FRAC_PI_4.sin(), 0.0);
        // The splitting operand: an eighth sheet, whose corner stands at
        // the split.
        let (x, _) = sheet(FRAC_PI_4);
        split_other_at_point(
            &mut y,
            Operand::B,
            bottom,
            mid,
            crate::boolean::Cell::Vertex(vertex_at(&x, mid)),
            band(),
            Tol::witness(),
            &mut super::ContactAcc::default(),
        )
        .expect("the arc splits");
        let reach = |circle| {
            arc_chain_reaches(&y, from, to, Vec3::unit_y(), circle, band()).expect("no escalation")
        };
        assert!(
            !reach((Point3::new(-1.0, 0.0, 0.0), Vec3::unit_z(), 2.0)),
            "the middle vertex is off the tangent circle"
        );
        assert!(
            reach((Point3::origin(), Vec3::unit_z(), 1.0)),
            "and on the sheet's own circle the chain runs through"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod clearance_rows {
    //! The conic clearance rung ([`super::conic_clearance`]) on grazes:
    //! a `Positive` there is a certified "no event" that returns before
    //! any root door is asked, so it must never stand for a carrier that
    //! comes within the band of the surface.

    use core::f64::consts::FRAC_PI_2;

    use super::conic_clearance;
    use crate::boolean::conic_oracle::{distance_from_anchor, unit};
    use geom_core::{Band, Point3, Sign, Vec3};
    use test_utils::fuzz;

    /// **The reviewer's pin: a definite crossing is not certified
    /// clear.** An ellipse stored in the ordinary order and sign (major
    /// 4.85 m, minor 0.21 m) meets a 24.7 µm ball 2.71e-11 m deep at its
    /// minor vertex (a 60-digit oracle's figure), at ε = 1e-12. There
    /// the harmonics' phases align, so `c₀ − A₁ − A₂` IS the least
    /// residual, and read bare its rounding (`≈ u·a²/r ≈ 1e-10`) put it
    /// at +2.9e-11: the rung certified the crossing clear.
    #[test]
    fn a_crossing_at_the_minor_vertex_is_not_certified_clear() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let axis = Vec3::new(
            -0.962_120_141_566_121_1,
            -0.090_956_895_968_249_41,
            0.257_005_206_695_522_7,
        );
        let e = geom::Curve3::Ellipse {
            center: Point3::new(
                -0.480_548_118_168_072_4,
                -0.441_016_920_669_073_5,
                -0.888_958_285_986_655_8,
            ),
            axis,
            semi_u: 4.846_323_757_498_574,
            semi_v: 0.207_528_397_062_063_02,
            u_ref: Vec3::new(
                0.087_713_558_478_483_3,
                0.789_303_434_737_465_2,
                0.607_705_865_999_894_6,
            ),
        };
        let s = geom::Surface::Sphere {
            center: Point3::new(
                -0.426_972_420_046_788_4,
                -0.567_049_134_604_379_2,
                -0.732_997_401_751_036_8,
            ),
            radius: 2.466_073_624_731_285_6e-5,
            axis,
            u_ref: Vec3::new(
                0.272_625_811_677_447_04,
                -0.320_994_777_005_651_3,
                0.906_993_671_390_437_7,
            ),
        };
        let conic = geom_brep::Conic::of(&e).unwrap();
        let v = 3.0 * FRAC_PI_2;
        let got = conic_clearance(&s, &conic, (v - 0.5, v + 0.5), band);
        assert!(
            !matches!(got, Some(Ok(Sign::Positive))),
            "a crossing 2.71e-11 m deep certified clear: {got:?}"
        );
    }

    /// **A short arc a thousand kilometres out is charged its samples'
    /// rounding.** The third delta review's pin: an ellipse stored with
    /// `major = −0.556`, `minor = 1.02`, centred 1000 km out, an arc of
    /// ±3e-4 rad about its vertex, against a 0.118 m ball, at ε = 1e-12.
    /// The ball was placed 2.1e-11 m off by its nominal, but its stored
    /// centre rounds at that distance's ulp, and the true least distance
    /// of the STORED geometry is −5.3e-12 m — in band. On an arc that
    /// short the chord-dip charge is negligible, so the arc enclosure's
    /// margin is its samples' own: read without their rounding charge
    /// (`geom_brep::conic_arc_residual_range`'s `sample_rounding`), the
    /// rung certified the touch clear.
    #[test]
    fn a_short_arc_far_out_is_charged_its_samples_rounding() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let axis = Vec3::new(
            -0.331_547_126_719_510_5,
            -0.144_826_239_786_666_6,
            0.932_256_329_039_010_6,
        );
        let center = Point3::new(
            -793_186.577_792_401_4,
            241_909.440_601_441_78,
            -783_085.727_290_743_7,
        );
        let (major, minor) = (-0.556_277_953_009_113_9, 1.019_942_477_711_699_2);
        let u_ref = Vec3::new(
            -0.346_444_852_811_330_7,
            -0.900_421_850_752_487_9,
            -0.263_090_202_493_364_96,
        );
        let e = geom::Curve3::Ellipse {
            center,
            axis,
            semi_u: major,
            semi_v: minor,
            u_ref,
        };
        let x = Vec3::new(1.0, 0.0, 0.0);
        let s = geom::Surface::Sphere {
            center: Point3::new(
                -793_187.576_342_526_2,
                241_909.907_376_568_15,
                -783_086.009_900_933_8,
            ),
            radius: 0.117_972_233_474_760_74,
            axis,
            u_ref: (x - axis * x.dot(axis)).normalize(),
        };
        let vertex: f64 = 4.712_388_980_384_69;
        let (sv, cv) = vertex.sin_cos();
        let least = distance_from_anchor(&s, |anchor| {
            (center - anchor) + u_ref * (major * cv) + axis.cross(u_ref) * (minor * sv)
        });
        assert!(
            least.abs() <= 1e-11,
            "the pose is a touch in band: {least:e}"
        );
        let conic = geom_brep::Conic::of(&e).unwrap();
        let got = conic_clearance(
            &s,
            &conic,
            (4.712_088_980_384_689_5, 4.712_688_980_384_69),
            band,
        );
        assert!(
            !matches!(got, Some(Ok(Sign::Positive))),
            "a touch {least:e} m off certified clear: {got:?}"
        );
    }

    /// **No certified clearance on a graze, through the rung.** Circles,
    /// and ellipses in all eight stored orders and signs (eccentricity
    /// up to 40), metre- or kilometre-sized, centred up to a metre, a
    /// kilometre or a thousand kilometres out (where a sampled residual's
    /// coordinate rounding, `u·|p|`, passes the band at 1e-12), each meeting a sphere or a wall (radius 1 µm to 1 m, the
    /// wall's axis across the outward normal) at a vertex, set off along
    /// the outward normal by `gap`, −40 to 40 bands. The vertex is the
    /// carrier's least distance from the surface (the carrier bends away
    /// from it), read from the stored surface's anchor: the pose crosses
    /// when that is below `−ε` and touches in band when it is within
    /// `ε`; a certified
    /// `Positive` is wrong in both. A third of the draws put a torus's
    /// tube there instead, its spine bending away from the carrier, under
    /// the same oracle. Counts printed.
    #[test]
    fn no_certified_clearance_on_a_graze() {
        let mut rng = fuzz::start("reduce::no_certified_clearance_on_a_graze");
        for eps in [1e-12, 1e-9, 1e-6] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let (mut clear, mut refused, mut torus_clear) = (0, 0, 0);
            for i in 0..fuzz::scaled(3000) {
                let n = unit(&mut rng);
                let u = unit(&mut rng);
                let u_ref = (u - n * u.dot(n)).normalize();
                let scale = if rng.below(2) == 0 { 1.0 } else { 1000.0 };
                let big = scale * rng.range(0.5, 5.0);
                let circle = i % 9 == 8;
                let combo = i % 8;
                let small = if circle {
                    big
                } else {
                    big / rng.range(1.0, 40.0)
                };
                let (mut major, mut minor) = if combo & 1 == 0 {
                    (big, small)
                } else {
                    (small, big)
                };
                if !circle && combo & 2 != 0 {
                    major = -major;
                }
                if !circle && combo & 4 != 0 {
                    minor = -minor;
                }
                let far = [1.0, 1e3, 1e6][rng.below(3)];
                let center = Point3::new(
                    rng.range(-far, far),
                    rng.range(-far, far),
                    rng.range(-far, far),
                );
                let e = if circle {
                    geom::Curve3::Circle {
                        center,
                        axis: n,
                        radius: major,
                        u_ref,
                    }
                } else {
                    geom::Curve3::Ellipse {
                        center,
                        axis: n,
                        semi_u: major,
                        semi_v: minor,
                        u_ref,
                    }
                };
                let conic = geom_brep::Conic::of(&e).unwrap();
                let vertex = FRAC_PI_2 * f64::from(u32::try_from(rng.below(4)).unwrap());
                let p = e.eval(vertex);
                let outward = (p - center).normalize();
                let gap = eps * rng.range(-40.0, 40.0);
                let r = 10f64.powf(rng.range(-6.0, 0.0));
                let hub = p + outward * (r + gap);
                let x = Vec3::new(1.0, 0.0, 0.0);
                let kind = rng.below(3);
                let s = match kind {
                    0 => geom::Surface::Sphere {
                        center: hub,
                        radius: r,
                        axis: n,
                        u_ref: (x - n * x.dot(n)).normalize(),
                    },
                    1 => {
                        let v = unit(&mut rng);
                        let w = (v - outward * v.dot(outward)).normalize();
                        geom::Surface::Cylinder {
                            origin: hub,
                            axis: w,
                            radius: r,
                            u_ref: outward,
                        }
                    }
                    _ => {
                        let v = unit(&mut rng);
                        let w = (v - outward * v.dot(outward)).normalize();
                        let ring = r * rng.range(1.5, 10.0);
                        let axis = outward.cross(w).normalize();
                        geom::Surface::Torus {
                            center: hub + outward * ring,
                            axis,
                            major_radius: ring,
                            minor_radius: r,
                            u_ref: (x - axis * x.dot(axis)).normalize(),
                        }
                    }
                };
                let got = conic_clearance(&s, &conic, (vertex - 0.5, vertex + 0.5), band);
                let label = || {
                    format!(
                        "ε {eps}, case {i}: gap {gap:e}, {e:?} against {s:?} — {}",
                        fuzz::replay()
                    )
                };
                let certified = matches!(got, Some(Ok(Sign::Positive)));
                // The least distance, at the vertex, of the STORED
                // geometry: the surface's anchor was itself rounded where
                // it was placed (about `u·|p|`, past the band a hundred
                // kilometres out), so the oracle reads the vertex from
                // that anchor — `C₀ − anchor` is exact, the two being
                // metres apart, and the rest is metre-sized.
                let (sv, cv) = vertex.sin_cos();
                let from = |anchor: Point3<f64>| {
                    (center - anchor) + u_ref * (major * cv) + n.cross(u_ref) * (minor * sv)
                };
                let least = distance_from_anchor(&s, from);
                if certified {
                    if kind == 2 {
                        torus_clear += 1;
                    } else {
                        clear += 1;
                    }
                    assert!(least > eps, "{}: certified clear", label());
                } else {
                    refused += 1;
                }
            }
            println!(
                "ε {eps}: {clear} sphere/wall and {torus_clear} torus certified clear, {refused} not"
            );
        }
    }
}

/// **A broken body is a kernel bug at the operand gate.** A tier-1
/// finding is not scaffolding an edit left behind: the at-rest gate
/// refuses the torn body with tier 1's own findings, so it never
/// finishes into an operand; and the operand gate, which runs on a body
/// carried with no verdict, panics naming the operand and tier 1's
/// first finding, never refusing as [`BooleanError::ScaffoldingOperand`].
/// No public door tears a body, so the row tears one in-crate.
#[cfg(test)]
#[allow(clippy::expect_used)]
mod operand_gate_rows {
    use crate::AtRestBody;
    use crate::boolean::{BooleanOp, boolean_reduce};
    use crate::test_support_fixtures::brick;
    use geom_core::Tol;

    #[test]
    fn a_tier_one_broken_operand_panics_naming_it() {
        let tol = Tol::witness();
        let a = AtRestBody::validate(brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol), tol)
            .expect("the untorn brick finishes");
        let mut b = brick::<f64>((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), tol);
        let vertex = b.vertices().next().expect("a vertex").0;
        b.vertex_provenance.remove(vertex);
        let want = crate::validate::validate(&b).expect_err("the tear breaks tier 1");
        let at_rest =
            AtRestBody::validate(b.clone(), tol).expect_err("the torn body does not finish");
        assert_eq!(at_rest, want, "the at-rest refusal is tier 1's own verdict");
        let b = AtRestBody::not_run(b);
        let report = crate::surgery::tests::panic_message(|| {
            let _ = boolean_reduce(BooleanOp::Union, &a, &b, tol);
        });
        let first = format!("{:?}", want[0]);
        for fragment in [
            "operand B fails tier 1",
            first.as_str(),
            "every public door keeps the body tier-1-valid",
        ] {
            assert!(report.contains(fragment), "want {fragment:?} in: {report}");
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod neighbour_extent_rows {
    use super::gate_maximal_faces;
    use crate::boolean::{BooleanDecision, BooleanError, Operand, PlaneRung};
    use crate::entity::{EntityId, GeomRef};
    use crate::test_support_fixtures::{plant_disc_face, plant_ring_face, prism_z};
    use geom_core::{Band, Point3, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    /// The prism `[0, 4]² × [0, 1]`, its top face and the first
    /// half-edge of that face's outer loop.
    fn top() -> (crate::Body<f64>, crate::FaceKey, crate::HalfEdgeKey) {
        let square = [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
        let p = prism_z::<f64>(&square, 0.0, 1.0, Tol::witness());
        let outer = p.body.get_face(p.top_face).expect("the top").outer;
        let crate::LoopBoundary::Cycle { first } =
            p.body.get_loop(outer).expect("its loop").boundary
        else {
            panic!("the top face's outer loop is a cycle");
        };
        (p.body, p.top_face, first)
    }

    /// **An open edge that spans less than the band leaves the
    /// orientation undecided** (no over-acceptance): a unit triangle
    /// planted in the top on a key of its own holding the top's plane,
    /// its corners then drawn in to sides in the middle of the run's
    /// ambiguity band. No public door builds
    /// it, since an edge that short does not certify.
    #[test]
    fn a_short_open_edge_leaves_the_orientation_undecided() {
        let tol = Tol::witness();
        let (mut body, top, at) = top();
        let unit = [
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(2.0, 1.0, 1.0),
            Point3::new(1.5, 2.0, 1.0),
        ];
        let ring = plant_ring_face(&mut body, at, &unit, tol);
        let host = body.get_face(top).expect("the top").clone();
        let plane = body.get_surface(host.surface).expect("its plane").clone();
        // Lifts RechartStrandsDescriptions: the triangle's sides are the row, not their descriptions.
        body.set_face_surface_unvouched_for_tests(
            ring.membrane.face,
            crate::euler::FaceSurface::New {
                surface: plane,
                sense: host.sense,
            },
        )
        .expect("the triangle takes a key of its own");
        let s = (band().zero() + band().escalate()) / 2.0;
        let outer = body.faces[ring.membrane.face].outer;
        let crate::LoopBoundary::Cycle { first } = body.loops[outer].boundary else {
            panic!("the triangle's loop is a cycle");
        };
        let small = [(0.0, 0.0), (s, 0.0), (s / 2.0, s)];
        for he in body.loop_cycle(first).expect("the triangle's loop walks") {
            let key = body.vertices[body.half_edges[he].start].point;
            let i = unit
                .iter()
                .position(|&c| (c - body.points[key]).norm() == 0.0)
                .expect("a corner of the unit triangle");
            body.points[key] = Point3::new(1.0 + small[i].0, 1.0 + small[i].1, 1.0);
        }
        let err = gate_maximal_faces(&body, Operand::A, band())
            .expect_err("a short edge decides nothing");
        let BooleanError::Escalated {
            decision: BooleanDecision::Neighbours(PlaneRung::Orientation),
            ..
        } = err
        else {
            panic!("the orientation rung is undecided: {err:?}");
        };
    }

    /// **A curve lookup that fails is a kernel bug, never read as a
    /// length**: a disc planted in the top, its circle's curve entry
    /// then removed, panics naming the edge and the curve key (D2 row 4).
    #[test]
    fn a_dangling_curve_panics_naming_the_edge() {
        let (mut body, _, at) = top();
        let disc = plant_disc_face(
            &mut body,
            at,
            Point3::new(2.0, 2.0, 1.0),
            1.0,
            Tol::witness(),
        );
        let curve = body.edges[disc.edge].curve;
        body.curves.remove(curve).expect("the circle's entry");
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = gate_maximal_faces(&body, Operand::A, band());
        }));
        let want = format!(
            "{}'s curve names {}, which does not resolve",
            EntityId::Edge(disc.edge),
            GeomRef::Curve(curve)
        );
        assert!(report.contains(&want), "{report}");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod esc_tests {
    use super::{BooleanError, ContactAcc, Operand, esc, vertex_on_face};
    use crate::Body;
    use crate::boolean::contain::ContainError;
    use crate::boolean::plane_eq::PlaneDesc;
    use crate::boolean::solid_contain::PointInSolidError;
    use crate::entity::{FaceKey, LoopKey, VertexKey};
    use geom_core::{Band, Point3, Tol, Vec3};

    /// **The reduction answers `ContainError`'s refusals typed on a
    /// working copy.** Mid-operation the gate's at-rest verdict does not
    /// bind, so every refusal the face door owns — the lone-vertex loop
    /// included — is carried whole with the face and its operand.
    #[test]
    fn esc_carries_the_face_doors_refusal_whole() {
        let key = |n: u64| slotmap::KeyData::from_ffi((1 << 32) | n);
        let (face, lk) = (FaceKey::from(key(7)), LoopKey::from(key(9)));
        for e in [
            ContainError::EmptyLoop(lk),
            ContainError::LoopUnreadable(lk),
            ContainError::RayExhausted,
            ContainError::Curved(PointInSolidError::CorruptFace { face }),
        ] {
            let got = esc(e.clone(), Operand::B, face);
            assert!(
                matches!(
                    &got,
                    BooleanError::PointInFaceRefused { operand: Operand::B, face: f, refusal }
                        if *f == face && *refusal == e
                ),
                "{e:?} answered {got:?}"
            );
        }
    }

    /// **`vertex_on_face` names the operand that owns the face**, not
    /// the vertex's: a vertex of A read against a face of B whose loop
    /// is a lone vertex (the `mvfs` seed) refuses on B.
    #[test]
    fn vertex_on_face_refuses_on_the_faces_operand() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band");
        let at = Point3::new(0.0, 0.0, 0.0);
        let mut y = Body::<f64>::new();
        let seed = y.mvfs(at, true).expect("the seed state");
        let plane = PlaneDesc {
            origin: at,
            normal: Vec3::unit_z(),
        };
        for (x_is, face_is) in [(Operand::A, Operand::B), (Operand::B, Operand::A)] {
            let got = vertex_on_face(
                x_is,
                &mut y,
                VertexKey::default(),
                Point3::new(1.0, 0.0, 0.0),
                seed.face,
                &plane,
                geom_core::MarginDiag::value(0.0),
                &mut ContactAcc::default(),
                band,
                tol,
            );
            assert!(
                matches!(
                    &got,
                    Err(BooleanError::PointInFaceRefused {
                        operand,
                        face,
                        refusal: ContainError::EmptyLoop(lk),
                    }) if *operand == face_is && *face == seed.face && *lk == seed.r#loop
                ),
                "a vertex of {x_is:?} answered {got:?}"
            );
        }
    }
}

/// **`boundary_meets_circle_only_at`: a stale face refuses typed; a torn
/// ring link or curve past one that resolves panics**, where the ring
/// was stepped over and the curve read as one the certificate cannot
/// place, answering `Ok(false)`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use super::*;
    use crate::entity::GeomRef;
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
    use geom_core::{Tol, Vec3};

    #[test]
    fn a_stale_face_refuses_and_a_torn_curve_panics() {
        let band = Band::linear(Tol::witness()).unwrap();
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        // A circle in a plane every vertex of the cube is decided off.
        let circle = (Point3::new(0.5, 0.5, 10.0), Vec3::unit_z(), 0.25);
        assert!(
            boundary_meets_circle_only_at(&body, face, circle, &[], band).unwrap(),
            "the sound boundary meets the far circle nowhere"
        );
        let mut stale = body.clone();
        stale.faces.remove(face);
        assert!(
            matches!(
                boundary_meets_circle_only_at(&stale, face, circle, &[], band),
                Err(BooleanError::ClassificationInvariant { .. })
            ),
            "a face the caller carries that does not resolve refuses typed"
        );
        let mut torn = body.clone();
        let named = crate::review_d18::tear_ring(&mut torn, face);
        assert_torn_op_panics(
            "boundary_meets_circle_only_at (ring)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| boundary_meets_circle_only_at(b, face, circle, &[], band),
        );
        let outer = body.get_face(face).unwrap().outer;
        let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
        else {
            panic!("a cube face's loop is a cycle");
        };
        let edge = body.get_half_edge(first).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        body.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "boundary_meets_circle_only_at",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| boundary_meets_circle_only_at(b, face, circle, &[], band),
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod line_cone_rows {
    //! **The line × cone root rungs**, each pose read against the cone's
    //! quadric form evaluated at the line's points.

    use super::*;
    use crate::boolean::circle_roots::CircleRoots;
    use geom_core::{Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The cone about `+z` from the origin, half-angle `π/4`.
    const APEX: (f64, f64, f64) = (0.0, 0.0, 0.0);

    fn door(
        origin: [f64; 3],
        dir: [f64; 3],
        span: (f64, f64),
    ) -> Result<Result<CircleRoots<f64>, Unsettled>, BooleanError> {
        let tip = Point3::new(APEX.0, APEX.1, APEX.2);
        line_cone_roots(
            Point3::from_array(origin),
            Vec3::from_array(dir),
            (tip, Vec3::new(0.0, 0.0, 1.0), core::f64::consts::FRAC_PI_4),
            span,
            band(),
        )
    }

    fn roots(
        origin: [f64; 3],
        dir: [f64; 3],
        span: (f64, f64),
    ) -> Result<CircleRoots<f64>, Unsettled> {
        door(origin, dir, span).unwrap()
    }

    /// `ρ² − h²` at `origin + dir·t`: the quadric form of the cone.
    fn form(origin: [f64; 3], dir: [f64; 3], t: f64) -> f64 {
        let p: Vec<f64> = (0..3).map(|i| origin[i] + dir[i] * t).collect();
        p[0].powi(2) + p[1].powi(2) - p[2].powi(2)
    }

    /// **Two crossings, one on each nappe**: a line square to the axis
    /// beside it crosses one nappe twice; a line steeper than a
    /// generator, beside the apex, crosses each nappe once. Each root is
    /// a zero of the quadric form to the scalar's resolution, the two on
    /// opposite sides of the apex in the second pose.
    #[test]
    fn crossings_are_zeros_of_the_quadric_form() {
        for (label, origin, dir, nappes) in [
            (
                "square to the axis",
                [-2.0, 0.3, 1.0],
                [1.0, 0.0, 0.0],
                false,
            ),
            (
                "steep beside the apex",
                [0.2, 0.0, -1.0],
                [0.0, 0.6, 0.8],
                true,
            ),
        ] {
            let Ok(CircleRoots::Certified(found)) = roots(origin, dir, (0.0, 4.0)) else {
                panic!(
                    "{label}: two roots, got {:?}",
                    roots(origin, dir, (0.0, 4.0))
                );
            };
            let thetas: [_; 2] = found.roots().try_into().unwrap();
            for &t in &thetas {
                let f = form(origin, dir, t);
                assert!(f.abs() < 1e-14, "{label}: the form is {f} at the root {t}");
            }
            let heights: Vec<f64> = thetas.iter().map(|t| origin[2] + dir[2] * t).collect();
            assert_eq!(
                heights[0].signum() != heights[1].signum(),
                nappes,
                "{label}: the roots' heights {heights:?}"
            );
        }
    }

    /// **A miss, a graze and a ruling.** A line outside the cone that
    /// keeps clear of it is a `Miss`. A line tangent to the cone along a
    /// generator's tangent plane, moved off by a fraction of the zero
    /// band, is not a miss and not two roots: a tangency. A line parallel
    /// to a generator degenerates the quadratic, and the lane does not
    /// answer.
    #[test]
    fn a_miss_a_graze_and_a_ruling() {
        let b = band();
        assert!(
            matches!(
                roots([-2.0, 3.0, 1.0], [1.0, 0.0, 0.0], (0.0, 4.0)),
                Ok(CircleRoots::Miss)
            ),
            "a line 3 m off the axis at height 1 misses"
        );
        // Along `x` at `y = 1`, height 1: tangent to the cone's circle of
        // radius 1 there, at `(0, 1, 1)`.
        let graze = roots(
            [-2.0, 1.0 + 0.1 * b.zero(), 1.0],
            [1.0, 0.0, 0.0],
            (0.0, 4.0),
        );
        assert!(
            matches!(graze, Err(Unsettled::Tangency)),
            "a graze within the band, got {graze:?}"
        );
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let ruling = roots([0.5, 0.0, 0.0], [s, 0.0, s], (0.0, 2.0));
        assert!(
            matches!(ruling, Err(Unsettled::Degenerate)),
            "a line parallel to a generator, got {ruling:?}"
        );
    }

    /// **An edge reaching the apex refuses**, as one passing within the
    /// band of it. The apex rung reads the edge's segment, not its line:
    /// a segment ending half a metre short of the apex is not refused
    /// there (its line's quadratic has its double root at the apex, which
    /// the depth rung reads as a tangency).
    #[test]
    fn an_edge_at_the_apex_refuses() {
        let b = band();
        for (label, origin) in [
            ("through the apex", [-1.0, 0.0, 0.3]),
            ("in band of the apex", [-1.0, 0.3 * b.zero(), 0.3]),
        ] {
            let got = roots(origin, [1.0, 0.0, -0.3], (0.0, 2.0));
            assert!(
                matches!(got, Err(Unsettled::AtApex)),
                "{label}: got {got:?}"
            );
        }
        let short = roots([-1.0, 0.0, 0.3], [1.0, 0.0, -0.3], (0.0, 0.5));
        assert!(
            matches!(short, Err(Unsettled::Tangency)),
            "a segment ending half a metre short of the apex, got {short:?}"
        );
    }

    /// **Each root is placed along the edge** (the root-slack rung's row),
    /// against an independent oracle: the double cone's residual
    /// `ρ cos α − |h| sin α`, read from each point of the line and
    /// bisected where it changes sign. The poses are steep lines beside
    /// the apex — an edge along the axis, `δ` off it — on a narrow, a
    /// right and a wide cone at a millimetre, a metre and a kilometre, at
    /// each of the three ε rows. A root on a shallow crossing can lie on
    /// the cone and still be far along the edge from the true one, so the
    /// row reads the root's PLACE, never its distance from the cone:
    ///
    /// - a certified pair has exactly the oracle's crossings in a window
    ///   one span wide either side of the edge;
    /// - each lies within the zero band, along the edge, of the oracle's,
    ///   or past the span by at least its distance from it (the rung's
    ///   contract for a root outside the span), widened only by the
    ///   oracle's own resolution;
    /// - a `Miss` has no crossing in the window.
    ///
    /// Without the rung, a narrow cone at a kilometre places roots up to
    /// two micrometres off at ε 1e-9.
    #[test]
    fn roots_beside_the_apex_are_placed_along_the_edge() {
        use crate::boolean::conic_oracle::crossings;
        let axis = Vec3::new(0.0, 0.6, 0.8);
        let (u, _) = axis.orthonormal_basis();
        let (mut certified, mut judged) = (0, 0);
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for alpha in [0.01f64, 0.785, 1.5] {
                for scale in [1e-3, 1.0, 1e3] {
                    let apex = Point3::new(0.2 * scale, 0.1 * scale, -0.3 * scale);
                    for p in 0..30 {
                        let delta = scale * 10f64.powf(-15.0 + 0.5 * f64::from(p));
                        let origin = apex + u * delta - axis * scale;
                        let span = (0.0, 2.0 * scale);
                        let label = format!("ε {eps}, α {alpha}, scale {scale}, δ {delta:e}");
                        let res = |t: f64| {
                            let q = origin + axis * t - apex;
                            let h = q.dot(axis);
                            (q - axis * h).norm() * alpha.cos() - h.abs() * alpha.sin()
                        };
                        let length = span.1 - span.0;
                        let window = (span.0 - length, span.1 + length);
                        let inside = |t: &f64| {
                            *t > window.0 + 1e-3 * length && *t < window.1 - 1e-3 * length
                        };
                        let truth: Vec<f64> = crossings(res, window.0, window.1)
                            .into_iter()
                            .filter(inside)
                            .collect();
                        match line_cone_roots(origin, axis, (apex, axis, alpha), span, band) {
                            Ok(Ok(CircleRoots::Certified(found))) => {
                                let (count, thetas) = (found.count(), found.roots());
                                certified += 1;
                                let got: Vec<f64> =
                                    thetas[..count].iter().copied().filter(inside).collect();
                                assert_eq!(
                                    got.len(),
                                    truth.len(),
                                    "{label}: {got:?} vs the oracle's {truth:?}"
                                );
                                for (g, t) in got.iter().zip(&truth) {
                                    // The oracle's own resolution: its
                                    // reading's rounding over its slope.
                                    let step = 1e-6 * scale;
                                    let slope = (res(t + step) - res(t - step)) / (2.0 * step);
                                    let resolved =
                                        8.0 * f64::EPSILON * (scale + t.abs()) / slope.abs();
                                    let outside = (span.0 - g).max(g - span.1).max(0.0);
                                    let off = (g - t).abs();
                                    judged += 1;
                                    assert!(
                                        off <= eps + outside + resolved,
                                        "{label}: the root {g} lies {off:e} along the edge from \
                                         the oracle's {t} ({resolved:e} unresolved)"
                                    );
                                }
                            }
                            Ok(Ok(CircleRoots::Miss)) => {
                                assert!(truth.is_empty(), "{label}: a Miss, but {truth:?}");
                            }
                            Ok(_) | Err(_) => {}
                        }
                    }
                }
            }
        }
        assert!(
            certified > 50 && judged > 100,
            "the row reaches the rung: {certified} certified pairs, {judged} roots judged"
        );
    }
}

/// **The accumulator keeps each record once, by key**: what lets a
/// caller place an endpoint again after a split without minting a
/// second record ([`lying_on`]'s `Pierce`).
#[cfg(test)]
mod contact_acc_rows {
    use super::ContactAcc;
    use crate::boolean::{Operand, VfContact, VvContact};
    use crate::test_support_fixtures::prism_z;
    use geom_core::Tol;

    /// A pair reached twice is one record citing its one decision.
    #[test]
    fn a_record_pushed_twice_is_kept_once() {
        let p = prism_z::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
            0.0,
            1.0,
            Tol::witness(),
        );
        let (a, b, face) = (p.bottom[0], p.top[0], p.bottom_face);
        let mut acc = ContactAcc::default();
        let row = |x, y| super::PendingRow::on(x, y, Some(geom_core::MarginDiag::value(0.0)));
        let (va, vb) = (super::super::Cell::Vertex(a), super::super::Cell::Vertex(b));
        let f = super::super::Cell::Face(face);
        for _ in 0..2 {
            acc.vv(VvContact { a, b }, row((Operand::A, va), (Operand::B, vb)));
            acc.vf(
                Operand::A,
                VfContact { vertex: a, face },
                row((Operand::A, va), (Operand::B, f)),
            );
            acc.vf(
                Operand::B,
                VfContact { vertex: b, face },
                row((Operand::B, vb), (Operand::A, f)),
            );
        }
        let (r, pending) = acc.finish();
        assert_eq!(
            (r.vv.len(), r.a_on_b.len(), r.b_on_a.len()),
            (1, 1, 1),
            "one record of each kind"
        );
        assert_eq!(pending.len(), 3, "a question asked twice is one decision");
        let cited: Vec<Vec<crate::Backing>> = r.rows().map(|(_, c)| c.iter().collect()).collect();
        assert_eq!(
            cited,
            [0, 1, 2].map(|k| vec![crate::Backing::Decided(k)]).to_vec(),
            "each record cites its decision"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod arc_end_rows {
    use super::wall_crossing;
    use crate::test_support_fixtures::half_disc_cap;
    use geom_core::{Band, Tol, Vec3};

    /// **A wall root landing beside an arc's end, in the band's own
    /// parting, escalates.** A line from a sphere's centre crosses it at
    /// a point `≈ 1.34·zero` from the half-disc cap's arc end and its
    /// corner, which the band `(ε, 1.25ε)` reads clear of both: the
    /// boundary reading puts a point at an end only on its raw chord, the
    /// distance the vertex pass reads to the corner, so the two no longer
    /// part. The point's foot lies within the zero band of the end along
    /// the arc, which the side of the end cannot place beside a chord past
    /// the band, so the root's placement escalates on that straddle.
    #[test]
    fn a_root_beside_an_arc_end_in_the_bands_own_parting_escalates() {
        let tol = Tol::witness();
        let zero = Band::linear(tol).unwrap().zero();
        let (body, cap, end) = half_disc_cap(0.0, tol);
        let q = end + Vec3::new(-0.95 * zero, 0.95 * zero, 0.0);
        let narrow = Band::new(zero, 1.25 * zero).unwrap();
        let reach = (q - geom_core::Point3::origin()).norm();
        let sphere = geom::Surface::Sphere {
            center: geom_core::Point3::origin(),
            radius: reach,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let line = geom::Curve3::Line {
            origin: geom_core::Point3::origin(),
            dir: (q - geom_core::Point3::origin()) / reach,
        };
        let got = wall_crossing(
            &body,
            cap,
            &sphere,
            &line,
            0.5 * reach,
            1.5 * reach,
            narrow,
            &super::door_escalates,
        );
        assert!(
            matches!(&got, Err(crate::boolean::BooleanError::Escalated { diag, .. })
                if diag.predicate == Some("curve_pair_span_end_straddle")),
            "the root's placement escalates on the end's straddle: {got:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_face_rows {
    //! [`super::spline_face`], [`super::twin_edge`] and the boundary
    //! placement [`super::along_boundary`], on a cylinder wall sheet
    //! re-labelled onto a spline surface: the boundary pass reads only
    //! the face's boundary edges, and the surface it carries is read only
    //! where the boundary leaves a span undecided
    //! ([`super::spline_interior`]); then on a twisted spline sheet whose
    //! trim lies on its chart, where it is read.
    use super::{
        ContactAcc, CurvedEvent, LyingIn, curved_face_arm, edge_ends, read_lying_in, twin_edge,
    };
    use crate::body::Body;
    use crate::boolean::{BooleanError, Glue, GluedPairs, Operand};
    use crate::entity::{EdgeKey, FaceKey};
    use crate::null::CurveGeom;
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, prism};
    use geom::{Curve3, Surface};
    use geom_brep::curve_pair::Span;
    use geom_core::{Band, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A unit cylinder wall sheet over azimuth `(u0, u1)` and height
    /// `[0, 1]`, its face re-labelled onto a bilinear spline patch.
    fn spline_sheet(u: (f64, f64)) -> (Body<f64>, FaceKey) {
        use geom::surfaces::nurbs::NurbsSurface;
        use geom_core::spline::KnotVector;
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let face = cyl_wall_sheet(&mut body, CylFrame::canonical(1.0), u, (0.0, 1.0), tol);
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::new(1).unwrap());
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
        ];
        let patch = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
        let surface = Surface::Nurbs(std::sync::Arc::new(patch));
        body.lifting_rechart_refusals_for_tests(|body| {
            body.set_face_surfaces_describing(
                vec![crate::construction::Rechart::new(surface, face, true)],
                &[],
                tol,
            )
        })
        .unwrap();
        (body, face)
    }

    /// The edge of `body` whose carrier is a circle at height `z`.
    fn rim(body: &Body<f64>, z: f64) -> EdgeKey {
        body.edges()
            .find(|(_, e)| {
                let Some(CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) else {
                    return false;
                };
                matches!(c.carrier(), Curve3::Circle { center, .. } if (center.z - z).abs() < 1e-12)
            })
            .map(|(k, _)| k)
            .expect("the rim")
    }

    /// `curved_face_arm` of `x`'s edge `ek` against `y`'s `face`.
    fn arm(
        x: &Body<f64>,
        ek: EdgeKey,
        y: &mut Body<f64>,
        face: FaceKey,
        acc: &mut ContactAcc,
    ) -> Result<CurvedEvent<f64>, BooleanError> {
        let edge = x.get_edge(ek).unwrap().clone();
        let ((u, pu), (v, pv)) = edge_ends(x, &edge).unwrap();
        let declared = GluedPairs::<f64>::without_struts(&Glue::none(), Default::default());
        curved_face_arm(
            x,
            y,
            Operand::A,
            ek,
            &edge,
            u,
            v,
            face,
            pu,
            pv,
            &declared,
            acc,
            band(),
            Tol::witness(),
        )
    }

    /// **A rim along part of a spline face's rim lies along it**: its
    /// shared end is recorded at the face's vertex, its other end splits
    /// the face's rim and is recorded at the vertex the split mints; a
    /// vertex a later split mints on the rim is placed on the face's rim
    /// the same way ([`LyingIn::Along`]).
    #[test]
    fn a_rim_along_part_of_a_spline_faces_rim_splits_it_at_its_end() {
        let tol = Tol::witness();
        let (mut y, face) = spline_sheet((0.2, 1.4));
        let (mut x, _) = spline_sheet((0.2, 0.8));
        let ek = rim(&x, 0.0);
        let before = y.vertices().count();
        let mut acc = ContactAcc::default();
        match arm(&x, ek, &mut y, face, &mut acc) {
            Ok(CurvedEvent::Along { recorded: true }) => {}
            other => panic!("the rim lies along the face's rim: {other:?}"),
        }
        assert_eq!(acc.records.vv.len(), 2, "both ends recorded");
        assert_eq!(
            y.vertices().count(),
            before + 1,
            "the face's rim split at the far end"
        );
        // A vertex minted on the rim afterwards, at its middle.
        let edge = x.get_edge(ek).unwrap().clone();
        let w = x.split_edge(ek, 0.5, tol).unwrap().vertex;
        read_lying_in(
            (
                &x,
                &crate::sector_shape::SlitTips::of(&x),
                Operand::A,
                ek,
                &edge,
            ),
            &mut y,
            w,
            &[LyingIn::Along(face)],
            &GluedPairs::<f64>::without_struts(&Glue::none(), Default::default()),
            &mut acc,
            band(),
            tol,
        )
        .unwrap();
        assert_eq!(acc.records.vv.len(), 3, "the minted vertex recorded");
        assert_eq!(
            y.vertices().count(),
            before + 2,
            "the face's rim split there"
        );
    }

    /// **An edge joining two vertices of the face's body that is no edge
    /// of it records its ends**: a chord across the sheet, from one corner
    /// to the opposite one, ends on the face's boundary, meets it nowhere
    /// else, and is no edge of `y`, so its interior is the interior door's,
    /// which finds the chord clear of the patch (it stays on the side
    /// `x + y > 1` of the patch's plane), and each end is recorded at the
    /// corner vertex it lands on.
    #[test]
    fn a_chord_between_two_corners_of_a_spline_face_records_its_ends() {
        let tol = Tol::witness();
        let (mut y, face) = spline_sheet((0.2, 1.4));
        let at = |a: f64, z: f64| Point3::new(a.cos(), a.sin(), z);
        let mut x = Body::<f64>::new();
        let seed = x.mvfs(at(0.2, 0.0), true).unwrap();
        let chord = x
            .mev_line(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                at(1.4, 1.0),
                tol,
            )
            .unwrap();
        let mut acc = ContactAcc::default();
        match arm(&x, chord.edge, &mut y, face, &mut acc) {
            Ok(CurvedEvent::Recorded) => {}
            other => panic!("the chord's interior is clear, its ends recorded: {other:?}"),
        }
        assert_eq!(acc.records.vv.len(), 2, "both ends at corners");
    }

    /// The twisted sheet `(u, v, 0.6·u·v)` over `[¼, ¾]²`, and a body
    /// holding the one line edge from `p` to `q`.
    fn twisted_and_a_line(
        p: Point3<f64>,
        q: Point3<f64>,
    ) -> (Body<f64>, FaceKey, Body<f64>, EdgeKey) {
        use crate::boolean::spline_region::rows::{rect, sheet, twisted};
        let tol = Tol::witness();
        let chart = twisted(0.6);
        let (y, face) = sheet(&chart, &rect((0.25, 0.75), (0.25, 0.75)));
        let mut x = Body::<f64>::new();
        let seed = x.mvfs(p, true).unwrap();
        let edge = x
            .mev_line(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                q,
                tol,
            )
            .unwrap()
            .edge;
        (y, face, x, edge)
    }

    /// **A meeting is placed on its own sheet** (`spline_chart_placement`
    /// in [`super::spline_interior`]): a chart whose third segment, the
    /// plane `y = d` with `d` half the band's zero, passes within the band
    /// of the line's crossing of its first, the plane `z = 0`, outside the
    /// face. The face lies on the third segment alone, which the line
    /// crosses at a shallow slope `s`, `10·escalate` further on. Read in
    /// space, the first meeting would be in the face through the third
    /// segment's foot; read at its own chart point it is out, and the
    /// pierce is the second.
    #[test]
    fn a_meeting_is_placed_on_the_sheet_it_was_found_on() {
        use crate::boolean::contain::FaceContainment;
        use crate::boolean::spline_region::rows::{rect, ruled, sheet};
        let tol = Tol::witness();
        let b = band();
        let d = 0.5 * b.zero();
        let chart = ruled(&[(-1.0, 0.0), (1.0, 0.0), (d, -1.0), (d, 1.0)]);
        let (mut y, face) = sheet(&chart, &rect((0.1, 0.9), (0.7, 0.95)));
        let s = d / (10.0 * b.escalate());
        let dir = Vec3::new(0.0, s, 1.0).normalize();
        let mut x = Body::<f64>::new();
        let seed = x.mvfs(Point3::origin() - dir * 0.5, true).unwrap();
        let edge = x
            .mev_line(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::origin() + dir * 0.5,
                tol,
            )
            .unwrap()
            .edge;
        let mut acc = ContactAcc::default();
        match arm(&x, edge, &mut y, face, &mut acc) {
            Ok(CurvedEvent::Pierce {
                p,
                at: FaceContainment::In,
                ..
            }) => assert!(
                (p.y - d).abs() <= b.zero() && p.z > 5.0 * b.escalate(),
                "the pierce is the third segment's, not the first's: {p:?}"
            ),
            other => panic!("the line pierces the face at its second meeting: {other:?}"),
        }
    }

    /// **A line through a spline face's interior pierces it there**, at the
    /// meeting the interior arm found, placed in by the trim; one through
    /// the chart past the trim is no event of the face, its ends off the
    /// carrier recording nothing.
    #[test]
    fn a_line_pierces_a_spline_face_inside_its_trim_and_passes_it_outside() {
        use crate::boolean::contain::FaceContainment;
        let up = |x: f64, y: f64| (Point3::new(x, y, -1.0), Point3::new(x, y, 1.0));
        let (p, q) = up(0.5, 0.5);
        let (mut y, face, x, ek) = twisted_and_a_line(p, q);
        let mut acc = ContactAcc::default();
        match arm(&x, ek, &mut y, face, &mut acc) {
            Ok(CurvedEvent::Pierce {
                p,
                at: FaceContainment::In,
                ..
            }) => assert!(
                (p - Point3::new(0.5, 0.5, 0.15)).norm() < 1e-9,
                "at the sheet's point over (½, ½): {p:?}"
            ),
            other => panic!("the line pierces the face inside its trim: {other:?}"),
        }
        let (p, q) = up(0.9, 0.4);
        let (mut y, face, x, ek) = twisted_and_a_line(p, q);
        let mut acc = ContactAcc::default();
        match arm(&x, ek, &mut y, face, &mut acc) {
            Ok(CurvedEvent::None) => {}
            other => panic!("a meeting past the trim is no event of the face: {other:?}"),
        }
        assert!(
            acc.records.vv.is_empty() && acc.records.a_on_b.is_empty(),
            "nothing recorded"
        );
    }

    /// **review r2 (PR 5826): a meeting outside the trim does not hide one
    /// inside it after it.** The diagonal line meets the twisted sheet at
    /// `S(0.9, 0.9)`, past the trim, then at `S(½, ½)`, inside it.
    #[test]
    fn review_r2_a_meeting_past_the_trim_then_one_inside_it_pierces_at_the_second() {
        use crate::boolean::contain::FaceContainment;
        let at = |s: f64| {
            let x = 0.9 - 0.4 * s;
            Point3::new(x, x, 0.486 - 0.336 * s)
        };
        let (mut y, face, x, ek) = twisted_and_a_line(at(-0.2), at(1.5));
        let mut acc = ContactAcc::default();
        match arm(&x, ek, &mut y, face, &mut acc) {
            Ok(CurvedEvent::Pierce {
                p,
                at: FaceContainment::In,
                ..
            }) => assert!(
                (p - Point3::new(0.5, 0.5, 0.15)).norm() < 1e-9,
                "at the sheet's point over (½, ½): {p:?}"
            ),
            other => panic!("the line pierces the face inside its trim: {other:?}"),
        }
    }

    /// The twisted sheet over `[¼, ¾]²`, and a body holding one quadratic
    /// NURBS edge from `p` to `q`, its middle control point `bow` off the
    /// chord's middle along `+x`, stated in the plane through the chord
    /// and `x̂`, its chord along `+z`.
    fn twisted_and_a_spline(
        p: Point3<f64>,
        q: Point3<f64>,
        bow: f64,
    ) -> (Body<f64>, FaceKey, Body<f64>, EdgeKey) {
        use crate::boolean::spline_region::rows::{rect, sheet, twisted};
        let tol = Tol::witness();
        let (y, face) = sheet(&twisted(0.6), &rect((0.25, 0.75), (0.25, 0.75)));
        let mut x = Body::<f64>::new();
        let seed = x.mvfs(p, true).unwrap();
        let x_hat = Vec3::unit_x();
        let key = x
            .set_face_surface(
                seed.face,
                crate::construction::FaceSurface::New {
                    surface: Surface::Plane {
                        origin: p,
                        normal: (q - p).cross(x_hat).normalize(),
                        u_ref: x_hat,
                    },
                    sense: true,
                },
            )
            .unwrap();
        let kv = || {
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
        };
        let mid = p.lerp(q, 0.5) + x_hat * bow;
        let carrier = Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv(), vec![p, mid, q], vec![1.0; 3]).unwrap(),
        ));
        // The plane's chart is `p + u·x̂ + v·(n × x̂)`, `n × x̂ = −ẑ` here.
        let half = 0.5 * (q - p).norm();
        let image = geom_brep::Pcurve::General(std::sync::Arc::new(
            geom::NurbsCurve2::new(
                kv(),
                vec![
                    geom_core::Point2::new(0.0, 0.0),
                    geom_core::Point2::new(bow, -half),
                    geom_core::Point2::new(0.0, -2.0 * half),
                ],
                vec![1.0; 3],
            )
            .unwrap(),
        ));
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart_image(key, image),
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        };
        let edge = x
            .mev(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                q,
                spec,
                tol,
            )
            .unwrap()
            .edge;
        (y, face, x, edge)
    }

    /// **A NURBS edge through a spline face's interior pierces it there**,
    /// at the meeting the spline span arm found, placed in by the trim at
    /// its chart point; one through the chart past the trim is no event of
    /// the face.
    #[test]
    fn a_nurbs_edge_pierces_a_spline_face_inside_its_trim_and_passes_it_outside() {
        use crate::boolean::contain::FaceContainment;
        let up = |x: f64, y: f64| (Point3::new(x, y, -1.0), Point3::new(x, y, 1.0));
        let (p, q) = up(0.5, 0.5);
        let (mut y, face, x, ek) = twisted_and_a_spline(p, q, 0.1);
        let mut acc = ContactAcc::default();
        match arm(&x, ek, &mut y, face, &mut acc) {
            Ok(CurvedEvent::Pierce {
                p,
                at: FaceContainment::In,
                ..
            }) => assert!(
                (p.z - 0.6 * p.x * p.y).abs() < 1e-9 && p.x > 0.5 && p.x < 0.55,
                "on the sheet, on the bowed edge: {p:?}"
            ),
            other => panic!("the NURBS edge pierces the face inside its trim: {other:?}"),
        }
        let (p, q) = up(0.9, 0.4);
        let (mut y, face, x, ek) = twisted_and_a_spline(p, q, 0.05);
        let mut acc = ContactAcc::default();
        match arm(&x, ek, &mut y, face, &mut acc) {
            Ok(CurvedEvent::None) => {}
            other => panic!("a meeting past the trim is no event of the face: {other:?}"),
        }
        assert!(
            acc.records.vv.is_empty() && acc.records.a_on_b.is_empty(),
            "nothing recorded"
        );
    }

    /// **A point split out of a thin ellipse lands where it was read, or
    /// refuses.** The boundary placement splits the face's edge at a
    /// point the pre-pass put on it ([`super::split_other_at_point`]),
    /// whose parameter is the carrier's `param_near`: on an ellipse the
    /// eccentric anomaly of the point's unit-coordinate image, which for
    /// a point `ε` off the carrier lands up to `(1 + a/b)·ε` from it. The
    /// split's children certify against the point itself, so a parameter
    /// read that far off cannot mint a child whose end is not the
    /// point's: each split here either holds, the new vertex at the
    /// point and both children reaching it, or refuses typed.
    #[test]
    fn a_split_of_a_thin_ellipse_lands_where_it_was_read_or_refuses() {
        let tol = Tol::witness();
        let z = band().zero();
        for phi in [1.56, 1.569_796] {
            let (y, _, _) = crate::test_support_fixtures::oblique_rim_wall(phi);
            let ek = y.edges().map(|(k, _)| k).next().expect("the rim");
            let Some(CurveGeom::Certified(curve)) = y.get_curve_geom(y.get_edge(ek).unwrap().curve)
            else {
                panic!("a certified rim")
            };
            let carrier = curve.carrier().clone();
            let Curve3::Ellipse { axis, .. } = carrier else {
                panic!("an ellipse")
            };
            for t in [0.4, 1.3, 2.9] {
                let (p, d) = carrier.ders1(t);
                let p = p + axis.cross(d).normalize() * (0.5 * z);
                let mut y = y.clone();
                let mut acc = ContactAcc::default();
                // The splitting operand: a block whose corner stands at
                // the point.
                let x = crate::test_support_fixtures::brick::<f64>(
                    (p.x, p.x + 1.0),
                    (p.y, p.y + 1.0),
                    (p.z, p.z + 1.0),
                    tol,
                );
                let corner = x
                    .vertices()
                    .map(|(v, _)| v)
                    .find(|&v| (x.resolve_vertex_point(v, crate::live::Proven) - p).norm() == 0.0)
                    .expect("the block's corner at the point");
                match super::split_other_at_point(
                    &mut y,
                    Operand::B,
                    ek,
                    p,
                    crate::boolean::Cell::Vertex(corner),
                    band(),
                    tol,
                    &mut acc,
                ) {
                    Ok(w) => {
                        let at = y.resolve_vertex_point(w, crate::live::Proven);
                        assert!(
                            (at - p).norm() == 0.0,
                            "{phi} at {t}: the vertex is the point"
                        );
                        for e in y.edges_of_vertex(w).unwrap() {
                            let e = y.get_edge(e).unwrap();
                            let Some(CurveGeom::Certified(c)) = y.get_curve_geom(e.curve) else {
                                panic!("a certified child")
                            };
                            let (c0, c1) = c.params();
                            let reach = (c.carrier().eval(c0) - p)
                                .norm()
                                .min((c.carrier().eval(c1) - p).norm());
                            assert!(reach <= band().escalate(), "{phi} at {t}: {reach:e}");
                        }
                    }
                    Err(BooleanError::CrossingInsertion { .. }) => {}
                    Err(other) => panic!("{phi} at {t}: {other:?}"),
                }
            }
        }
    }

    /// **[`twin_edge`] finds an edge of the body, and nothing else**: a
    /// prism's bottom edge read against its top face is the edge; the
    /// bottom face's diagonal joins two of its vertices and is no edge;
    /// and the rest of a sheet's rim circle shares the rim's two ends and
    /// no run.
    #[test]
    fn twin_edge_finds_the_edge_and_no_other_span() {
        let tol = Tol::witness();
        let p = prism::<f64>(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0, tol);
        let pt = |k| p.body.resolve_vertex_point(k, crate::live::Proven);
        let line = |a: Point3<f64>, b: Point3<f64>| Curve3::Line {
            origin: a,
            dir: (b - a).normalize(),
        };
        let (a, b, c) = (pt(p.bottom[0]), pt(p.bottom[1]), pt(p.bottom[2]));
        let ends = |q, r| [(p.bottom[0], q), (p.bottom[0], r)];
        let edge = line(a, b);
        let span = Span::new(&edge, (0.0, (b - a).norm()));
        assert!(
            twin_edge(
                &p.body,
                (Operand::A, p.top_face, EdgeKey::default()),
                &span,
                ends(a, b),
                band()
            )
            .unwrap(),
            "the bottom edge"
        );
        let diagonal = line(a, c);
        let span = Span::new(&diagonal, (0.0, (c - a).norm()));
        assert!(
            !twin_edge(
                &p.body,
                (Operand::A, p.top_face, EdgeKey::default()),
                &span,
                ends(a, c),
                band()
            )
            .unwrap(),
            "the diagonal"
        );
        let (y, face) = spline_sheet((0.2, 1.4));
        let rim = rim(&y, 0.0);
        let Some(CurveGeom::Certified(curve)) = y.get_curve_geom(y.get_edge(rim).unwrap().curve)
        else {
            panic!("a certified rim")
        };
        let (t0, t1) = curve.params();
        let rest = Span::new(curve.carrier(), (t1, t0 + core::f64::consts::TAU));
        let (q, r) = (curve.carrier().eval(t1), curve.carrier().eval(t0));
        let ends = [(p.bottom[0], q), (p.bottom[0], r)];
        assert!(
            !twin_edge(
                &y,
                (Operand::A, face, EdgeKey::default()),
                &rest,
                ends,
                band()
            )
            .unwrap(),
            "the rest of the rim's circle"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod review_5993_rows {
    //! Review rows (PR 5993, R2): the line × torus door on the span path,
    //! at the near-perpendicular pose `r1_probes` pins
    //! (`r1_the_near_perpendicular_ray_keeps_its_roots`), whose exact
    //! Sturm count is 2 and whose roots are certified at ε = 1e-12.
    use super::*;

    fn torus() -> geom::Surface<f64> {
        geom::Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: geom_core::Vec3::new(0.0, 1.0, 0.0),
            major_radius: 1.0,
            minor_radius: 0.9,
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        }
    }

    #[test]
    fn review_5993_a_near_perpendicular_line_through_a_torus_is_no_kernel_fault() {
        let o = Point3::new(
            0.616_510_985_187_377_8,
            -0.432_236_860_832_721_6,
            -1.766_591_924_017_196_6,
        );
        let d = geom_core::Vec3::new(
            -0.779_335_113_179_378_4,
            3.340_316_168_992_811e-5,
            -0.626_607_357_321_883_2,
        );
        for eps in [1e-12, 1e-9, 1e-6] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let got = line_wall_roots_of(o, d, (-2.0, 0.0), &torus(), band);
            println!("eps {eps:e}: {got:?}");
            assert!(
                !matches!(got, Err(BooleanError::ClassificationInvariant { .. })),
                "a legal line with two certified-able roots ends as a kernel defect at ε {eps:e}: {got:?}"
            );
        }
    }
}
