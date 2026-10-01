//! **The cluster-record maintenance** (ASM-R2a D-3) — keeping the
//! placement registry keyed on the mate graph's clusters.
//!
//! - **What it re-keys.** The document's placement registry: one
//!   frame per cluster, keyed by the cluster's gauge (its
//!   document-order-first instance, [`groups`]). When an edit joins,
//!   splits, or empties a cluster, or kills its gauge, the rows move
//!   so each surviving cluster's frame still leaves its gauge's world
//!   pose where the prior document had it ([`reconcile`]). Each move
//!   is a [`ClusterMaintenance`] row carried on the edit's record,
//!   and the registry is rebuilt from those rows alone
//!   ([`registry_after`]).
//! - **When the edit door runs it.** After every accepted edit that
//!   can move the mate graph, through [`maintain`]: the live door
//!   derives the rows, solving the PRIOR document at most once, and
//!   only when a gauge moved; replay re-applies the logged rows and
//!   never solves ([`Maintain`]).
//! - **Why a solve with no verdict refuses the edit.** A moved gauge's
//!   new frame is minted from its pose under the prior solve. When
//!   that solve reached no verdict for the cluster (no band, an
//!   in-band split, a part not in hand), a pose may exist that nothing
//!   here knows, and recording a frame would write one nothing
//!   decided; the edit refuses instead ([`undecided`]).

use std::collections::{BTreeMap, BTreeSet};

use geom_core::Tol;

use super::reach::{FacePoseRefusal, MateReach};
use super::solve::{SolvedPoses, groups, has_mates, solve_document};
use super::{FaceRefusal, MateFault};
use crate::doc::Doc;
use crate::edit::EditError;
use crate::node::RecipeNodeId;
use crate::placement::Frame;

/// One recorded act of cluster-record maintenance (D-3). Each is a
/// consequence of an ordinary recorded edit, carried on that edit's
/// [`crate::EditRecord`]; undo is keeping the prior document value, so
/// every one of them restores exactly.
///
/// On the wire, beside the edit that performed it (`edit::LoggedEdit`):
/// replay re-applies these rows to the registry rather than solving
/// again, so the log carries what the maintenance decided.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ClusterMaintenance {
    /// Two clusters became one. The surviving gauge keeps its frame;
    /// the absorbed cluster's frame is CONSUMED into this record (it
    /// no longer keys anything).
    Join {
        /// The gauge that survived: the earlier of the two.
        survived: RecipeNodeId,
        /// The absorbed cluster's former gauge.
        absorbed: RecipeNodeId,
        /// Its frame, as recorded — `None` when the row was absent
        /// (the identity).
        absorbed_frame: Option<Frame>,
    },
    /// A cluster split off and its new gauge's frame was RE-MINTED
    /// from the solved pose.
    ///
    /// **The claim is GAUGE-exact** (review MINOR-1): the new gauge's
    /// world pose is preserved BIT for bit, and every other member's
    /// is preserved as a value, not as bits. Members are placed by
    /// composition from the gauge, and the split re-associates that
    /// composition — `(F ∘ rel_gauge) ∘ rel_member` where it used to
    /// be `F ∘ (rel_gauge ∘ rel_member)` — so a deep member can move
    /// by a rounding step. Stating this exactly is what makes rows 4b
    /// and 4c assertable: they pin the gauge's bits, which is the
    /// claim, rather than a bit-identity the arithmetic cannot offer.
    Split {
        /// The gauge of the cluster it separated from.
        from: RecipeNodeId,
        /// The new cluster's gauge.
        to: RecipeNodeId,
        /// The minted frame, `None` for the identity.
        frame: Option<Frame>,
    },
    /// The gauge instance died and the key moved to the next
    /// representative, composing with the already-solved relative
    /// pose. GAUGE-exact, on [`ClusterMaintenance::Split`]'s terms and
    /// for its reason.
    GaugeRewrite {
        /// The dead gauge.
        from: RecipeNodeId,
        /// Its successor.
        to: RecipeNodeId,
        /// The rewritten frame, `None` for the identity.
        frame: Option<Frame>,
    },
    /// A cluster's last instance died; its record goes with it.
    Drop {
        /// The dead gauge.
        gauge: RecipeNodeId,
        /// The frame that was recorded, `None` when the row was absent.
        frame: Option<Frame>,
    },
}

impl ClusterMaintenance {
    /// The gauge the act moves: the absorbed cluster's for a `Join`,
    /// the new gauge for a `Split` or `GaugeRewrite`, the dropped one
    /// for a `Drop`. [`EditError::MaintenanceUnrecorded`] names it.
    pub(crate) fn moved_gauge(&self) -> RecipeNodeId {
        match self {
            Self::Join { absorbed, .. } => *absorbed,
            Self::Split { to, .. } | Self::GaugeRewrite { to, .. } => *to,
            Self::Drop { gauge, .. } => *gauge,
        }
    }

    /// The frame the row carries, if any: an absorbed cluster's, a
    /// minted or rewritten gauge's, a dropped gauge's.
    pub fn frame(&self) -> Option<&Frame> {
        match self {
            Self::Join { absorbed_frame, .. } => absorbed_frame.as_ref(),
            Self::Split { frame, .. }
            | Self::GaugeRewrite { frame, .. }
            | Self::Drop { frame, .. } => frame.as_ref(),
        }
    }
}

impl core::fmt::Display for ClusterMaintenance {
    /// Each act in prose, F6-shaped. It lives beside the enum because
    /// the sentence is about the mate graph's motion, which is what
    /// this module knows; [`crate::Maintenance`] delegates here for
    /// its cluster arm rather than keeping a second copy of these
    /// four sentences in the edit vocabulary.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Join {
                survived, absorbed, ..
            } => write!(
                f,
                "the cluster gauged by node {} was absorbed into the one gauged by node {}",
                absorbed, survived
            ),
            Self::Split { from, to, .. } => write!(
                f,
                "a cluster separated from the one gauged by node {} and is now gauged by node {}",
                from, to
            ),
            Self::GaugeRewrite { from, to, .. } => write!(
                f,
                "the cluster gauged by node {} lost that instance and is now gauged by node {}",
                from, to
            ),
            Self::Drop { gauge, .. } => write!(
                f,
                "the cluster gauged by node {} lost its last instance, and its placement record \
                 went with it",
                gauge
            ),
        }
    }
}

/// **Where the maintenance's rows come from** for one edit: derived
/// from the edit's motion of the mate graph with a solved frame
/// obtained one of two ways when a row needs one (a cluster whose
/// gauge moved; every other row is structural), or replayed verbatim
/// from the log.
#[derive(Clone, Copy)]
pub(crate) enum Maintain<'a> {
    /// The live edit door: solve the PRIOR document through this
    /// reach, once, the first time a row needs it.
    Solve(&'a dyn MateReach),
    /// Replay of a logged edit that recorded no rows: the entry claims
    /// the edit performs no cluster maintenance, so any row refuses — a
    /// row that needs a solved frame because replay never solves, and a
    /// row derived from the documents alone because re-deriving it
    /// would make the replay disagree with its entry.
    Never,
    /// Replay of a logged edit that recorded rows: they are what the
    /// maintenance decided, re-applied verbatim, and nothing is
    /// derived.
    Recorded(&'a [ClusterMaintenance]),
}

impl<'a> Maintain<'a> {
    /// The reach a decision at this door levers through: the live
    /// door's, and none under either replay arm.
    ///
    /// **The replay rules, and why they differ.** The per-mate
    /// admission ([`admit_mate`]) with no reach DECLINES what needs
    /// the parts — the rider on a coincidence, decided over a lever,
    /// and a `FromFace` side's frame, resolved from the part's own
    /// face (`solve`'s `resolve_side`) — under `Never` and `Recorded` alike,
    /// because the door that recorded the entry decided them over the
    /// parts it had in hand, and re-deciding here would need a store
    /// replay never holds; everything decided on the datum alone is
    /// decided again (each authored side's frame ladder, the table's
    /// static gaps). A `FromFace` side declined is a face not read:
    /// the name is the datum, and the next solve resolves it. The
    /// maintenance under `Never` REFUSES any row
    /// ([`EditError::MaintenanceUnrecorded`]), because the entry
    /// claimed none and a frame nothing decided cannot be recorded;
    /// under `Recorded` it re-applies the rows the recording door
    /// minted. A decision
    /// declined leaves nothing false in the document — the datum is
    /// what it was and the next solve decides it again; a frame
    /// invented would.
    ///
    /// [`admit_mate`]: super::solve::admit_mate
    pub(crate) fn reach(&self) -> Option<&'a dyn MateReach> {
        match self {
            Self::Solve(reach) => Some(*reach),
            Self::Never | Self::Recorded(_) => None,
        }
    }
}

/// **The maintenance for one accepted edit** — [`reconcile`] deriving
/// the rows, or the recorded rows re-applied — leaving `after`'s
/// registry keyed on its clusters and answering the rows that got it
/// there. The one dispatch over [`Maintain`].
///
/// # Errors
///
/// [`reconcile`]'s.
pub(crate) fn maintain<P: crate::ProfilePayload>(
    before: &Doc<P>,
    after: &mut Doc<P>,
    tol: Tol,
    how: Maintain<'_>,
) -> Result<Vec<ClusterMaintenance>, EditError> {
    match how {
        Maintain::Recorded(rows) => {
            after.set_placements(registry_after(before.placements(), rows));
            Ok(rows.to_vec())
        }
        Maintain::Solve(reach) => reconcile(before, after, tol, Some(reach)),
        // An entry with no rows claims the edit performed no cluster
        // maintenance, and replay holds it to exactly that: a row that
        // needs a solved frame refuses inside `reconcile`, and a row
        // derived from the documents alone (a `Join`, a `Drop`) refuses
        // here. Re-deriving it instead would give the log two answers
        // to "what did this edit do" — the entry's and the replay's.
        Maintain::Never => {
            let acts = reconcile(before, after, tol, None)?;
            match acts.first() {
                None => Ok(acts),
                Some(act) => Err(EditError::MaintenanceUnrecorded {
                    gauge: act.moved_gauge(),
                }),
            }
        }
    }
}

/// **The registry after the maintenance's acts**: the prior registry
/// with every act applied in order — a surviving gauge keeps its row
/// verbatim, a `Join` drops the absorbed row, a `Split` writes the new
/// gauge's minted frame, a `GaugeRewrite` moves the key, a `Drop`
/// removes the row. The live door and every replay door build the
/// registry through this one fold, from the acts alone, so a replayed
/// log reproduces the live registry bit for bit (D9) without a solve.
pub(crate) fn registry_after(
    before: &BTreeMap<RecipeNodeId, Frame>,
    acts: &[ClusterMaintenance],
) -> BTreeMap<RecipeNodeId, Frame> {
    let mut rows = before.clone();
    let set =
        |rows: &mut BTreeMap<RecipeNodeId, Frame>, key: RecipeNodeId, frame: Option<Frame>| {
            match frame {
                Some(f) => {
                    rows.insert(key, f);
                }
                None => {
                    rows.remove(&key);
                }
            }
        };
    for act in acts {
        match act {
            ClusterMaintenance::Join { absorbed, .. } => {
                rows.remove(absorbed);
            }
            ClusterMaintenance::Split { to, frame, .. } => set(&mut rows, *to, *frame),
            ClusterMaintenance::GaugeRewrite { from, to, frame } => {
                rows.remove(from);
                set(&mut rows, *to, *frame);
            }
            ClusterMaintenance::Drop { gauge, .. } => {
                rows.remove(gauge);
            }
        }
    }
    rows
}

/// The fault the prior solve recorded that explains a gauge with no
/// pose: the gauge's OWN — a cluster's refusal reaches every member
/// the solve could not pose, so a gauge with no pose and no fault is
/// a state the solve's invariants exclude, and `None` reports it
/// rather than borrowing another cluster's fault (which could be a
/// decided one, and would let the door proceed to write a frame).
fn unsolved_because(poses: &SolvedPoses, gauge: RecipeNodeId) -> Option<Box<MateFault>> {
    poses.fault(gauge).cloned().map(Box::new)
}

/// **Whether a fault means the solve reached NO verdict** about the
/// cluster — as against deciding, from the document's own content,
/// that the cluster has no pose.
///
/// The maintenance's invariant is that a cluster's frame leaves its
/// gauge's world pose where the prior document had it. When the prior
/// solve DECIDED the cluster has no pose (a contradictory or
/// under-determined fold, a table gap, a self-mate, a dangling or
/// mis-selected head, a malformed frame, a class the solve does not
/// admit), there is no pose to preserve and the members' own frames
/// were consumed when they joined: the orphan keeps the cluster's
/// frame, which is what deleting the offending mate — the recourse
/// every such refusal names — has always done. When the solve could
/// NOT decide — no band, an in-band case split, a part whose extent
/// or resolution is not in hand, a placer whose pose could not be
/// derived, a read mispaired against another document's solve — a
/// pose may well exist and nothing here knows it, so the edit refuses
/// rather than record a frame nothing decided.
fn undecided(fault: &MateFault) -> bool {
    match fault {
        MateFault::Band { .. }
        | MateFault::Indeterminate { .. }
        | MateFault::Unleverable { .. }
        | MateFault::PlacerRefused { .. }
        // A caller's mispairing is no verdict about the document.
        | MateFault::PosesOfAnotherDocument { .. } => true,
        // A face frame that did not resolve: the part not in hand, a
        // product whose scalar pins nothing, a body whose table names
        // a key it lacks, or a member on no instance — nothing here
        // knows the pose; a name the part's table has no row for or
        // ties, a carrier with no canonical frame — the document's own
        // content decided there is no frame, and re-authoring the mate
        // is the recourse.
        //
        // `NotAnInstance` is classified as the lever's arm is: the
        // whole of `Unleverable` is no verdict, its `NotAnInstance`
        // included, and the two arms are one fact (the member walk's
        // own rule broken, which no door reaches) met by two readers.
        // Nothing decided a pose there, so the edit refuses rather than
        // record a frame.
        MateFault::FaceUnresolved { refusal, .. } => match refusal.as_ref() {
            FaceRefusal::NotAnInstance { .. } => true,
            FaceRefusal::Reach { refusal, .. } => match refusal {
                FacePoseRefusal::PartUnresolved { .. } | FacePoseRefusal::Unpinned => true,
                FacePoseRefusal::Readback(error) => !matches!(
                    error,
                    topo::readback::ReadbackError::NoCanonicalFrame { .. }
                ),
                FacePoseRefusal::NoSuchName
                | FacePoseRefusal::Ambiguous { .. }
                | FacePoseRefusal::NotAFace { .. } => false,
            },
        },
        // An unsupported mate (a class the solve does not admit, a
        // primitive the coset table lacks) has no pose, and deleting
        // it is its recourse.
        MateFault::ClassNotAdmitted { .. }
        | MateFault::TableLacks { .. }
        | MateFault::Frame { .. }
        | MateFault::Contradictory { .. }
        | MateFault::Under { .. }
        | MateFault::DanglingHead { .. }
        | MateFault::PartSelectsAnotherCopy { .. }
        | MateFault::SelfMate { .. } => false,
    }
}
/// **The keying maintenance** (D-3): re-key `after`'s placement
/// registry onto its cluster representatives, preserving every
/// surviving cluster's GAUGE world pose BIT for bit (and every other
/// member's as a value — see [`ClusterMaintenance::Split`]), and
/// report what it did.
///
/// The one invariant, from which all four acts follow: *a cluster's
/// frame is the frame that leaves its gauge's world pose where the
/// prior document had it.* When the gauge did not change, that is the
/// prior row VERBATIM — bit-identical, which is what makes a mate-less
/// document's registry unchanged by this machinery existing.
///
/// The before/after clusters are computed first and the prior
/// document is SOLVED only when a row needs a solved frame — a cluster
/// whose gauge moved (`Split`, `GaugeRewrite`) — and then once, through
/// `how`. A `Join`, a drop, a gauge that stayed put, an edit on an
/// unmated document: none of these asks the reach, so the store is
/// consulted exactly when a frame is minted from a solve. The registry
/// itself is [`registry_after`] over the acts.
///
/// # Errors
///
/// [`EditError::MaintenanceRefused`] when the prior solve reached no
/// verdict for a gauge that moved (carrying the solve's fault;
/// [`undecided`] says which faults those are);
/// [`EditError::MaintenanceUnrecorded`] when `solve` is `None` (a
/// replay — [`Maintain::Never`]) and a row needed a solved frame.
pub(crate) fn reconcile<P: crate::ProfilePayload>(
    before: &Doc<P>,
    after: &mut Doc<P>,
    tol: Tol,
    solve: Option<&dyn MateReach>,
) -> Result<Vec<ClusterMaintenance>, EditError> {
    // Neither side has a mate: every cluster is a singleton on both,
    // so the registry is already keyed by its own gauges and the
    // maintenance has nothing to do. The invariant below would compute
    // exactly this, at the cost of a recipe pass per edit.
    if !has_mates(before) && !has_mates(after) {
        return Ok(Vec::new());
    }
    let before_clusters = groups(before);
    let before_gauge: BTreeMap<RecipeNodeId, RecipeNodeId> = before_clusters
        .iter()
        .flat_map(|c| c.iter().map(|&id| (id, c[0])))
        .collect();
    let after_clusters = groups(after);
    // The prior solve, made on the first row that needs a solved frame
    // and never otherwise.
    let mut before_poses: Option<SolvedPoses> = None;

    let mut acts: Vec<ClusterMaintenance> = Vec::new();
    let mut carried: BTreeSet<RecipeNodeId> = BTreeSet::new();
    let mut after_gauges: BTreeSet<RecipeNodeId> = BTreeSet::new();
    for cluster in &after_clusters {
        let gauge = cluster[0];
        after_gauges.insert(gauge);
        // Which prior clusters this one is made of, gauge-ordered.
        let mut sources: BTreeSet<RecipeNodeId> = cluster
            .iter()
            .filter_map(|id| before_gauge.get(id).copied())
            .collect();
        let Some(&old_gauge) = before_gauge.get(&gauge) else {
            // A freshly inserted instance has no prior world pose: the
            // identity is its frame, and an absent row IS the identity.
            continue;
        };
        carried.insert(old_gauge);
        sources.remove(&old_gauge);
        if old_gauge != gauge {
            // The relative pose of this cluster's NEW gauge under the
            // PRIOR mate graph — the one number the maintenance must
            // solve for; a gauge that stayed put is the identity by
            // construction and asks nothing.
            let relative = match solve {
                Some(reach) => {
                    let poses =
                        before_poses.get_or_insert_with(|| solve_document(before, reach, tol));
                    match poses.relative(gauge) {
                        Some(relative) => relative,
                        None => {
                            let fault = unsolved_because(poses, gauge);
                            if fault.as_deref().is_none_or(undecided) {
                                return Err(EditError::MaintenanceRefused { gauge, fault });
                            }
                            // Decided: no pose to preserve. The orphan
                            // keeps the cluster's frame ([`undecided`]).
                            Frame::IDENTITY
                        }
                    }
                }
                None => return Err(EditError::MaintenanceUnrecorded { gauge }),
            };
            let prior = before.placements().get(&old_gauge).copied();
            let frame = match (prior, relative.is_identity_bits()) {
                (row, true) => row,
                (Some(f), false) => Some(f.compose(&relative)),
                (None, false) => Some(relative),
            };
            let act = if after.node(old_gauge).is_some() {
                ClusterMaintenance::Split {
                    from: old_gauge,
                    to: gauge,
                    frame,
                }
            } else {
                ClusterMaintenance::GaugeRewrite {
                    from: old_gauge,
                    to: gauge,
                    frame,
                }
            };
            acts.push(act);
        }
        for absorbed in sources {
            carried.insert(absorbed);
            acts.push(ClusterMaintenance::Join {
                survived: gauge,
                absorbed,
                absorbed_frame: before.placements().get(&absorbed).copied(),
            });
        }
    }
    for (&gauge, &frame) in before.placements() {
        if !carried.contains(&gauge) && !after_gauges.contains(&gauge) {
            acts.push(ClusterMaintenance::Drop {
                gauge,
                frame: Some(frame),
            });
        }
    }
    after.set_placements(registry_after(before.placements(), &acts));
    Ok(acts)
}
