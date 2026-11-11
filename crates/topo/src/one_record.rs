//! **A coincidence verdict's token and the one act that consumes it**
//! (DESIGN `[coincidence.door]`, its mechanics in this crate's README
//! `[topo.one-record]`).
//!
//! A verdict that faces lie on one carrier ends with them on one
//! surface record. The deciding reading mints a [`OneRecord`]: the
//! faces it moves onto the kept record ([`moved_closure`]), every one of
//! them read over by the verdict; the reading records its own row. [`Body::one_record`]
//! is the one act that consumes a token; no caller hands it a record.
//! A verdict decided from values reaches only the faces it was read
//! over, so a face the token does not move keeps its record, whatever
//! record it wears.
//!
//! Which record is kept is a census of keys ([`kept_of`]): the one that
//! alone is also worn by faces the verdict did not decide, else the
//! first-written side's.

use std::collections::BTreeSet;

use geom_core::{Decide, Real, Tol};

use crate::SurfaceKey;
use crate::body::Body;
use crate::entity::FaceKey;
use crate::euler::EulerOpError;

/// **One value-decided coincidence verdict, ready to be acted on**: the
/// faces it moves onto `kept` from its other records.
/// Only a deciding reading mints one ([`Self::minted`]).
#[derive(Clone, Debug)]
pub(crate) struct OneRecord {
    /// The record the moved faces are left on.
    pub(crate) kept: SurfaceKey,
    /// The records they leave, each with whether its chart faces as the
    /// kept one's does: a moved face's sense is restated where it does
    /// not, so its material side is unchanged.
    pub(crate) others: Vec<(SurfaceKey, bool)>,
    /// The faces the verdict was read over that wear an other record: each
    /// decided face on it, every fragment of one, and their closure
    /// under same-record adjacency ([`moved_closure`]).
    pub(crate) moved: BTreeSet<FaceKey>,
}

impl OneRecord {
    /// The token a deciding reading mints: `moved`, read over with the
    /// decided faces, onto `kept` from `other`.
    pub(crate) fn minted(
        (kept, other): (SurfaceKey, SurfaceKey),
        charts_agree: bool,
        moved: BTreeSet<FaceKey>,
    ) -> Self {
        Self {
            kept,
            others: vec![(other, charts_agree)],
            moved,
        }
    }

    /// The record the moved faces are left on.
    pub(crate) fn kept(&self) -> SurfaceKey {
        self.kept
    }
}

/// **Whether `face`'s record is also worn by a face of `body` outside
/// `decided`**, the faces a verdict decided: the census [`kept_of`]
/// reads on each side. Keys only.
pub(crate) fn worn_outside<T: Real>(
    body: &Body<T>,
    face: FaceKey,
    decided: &BTreeSet<FaceKey>,
) -> bool {
    let Some(record) = body.get_face(face).map(|f| f.surface) else {
        return false;
    };
    body.faces()
        .any(|(f, g)| g.surface == record && f != face && !decided.contains(&f))
}

/// **Which side's record a verdict keeps**: the side that alone is also
/// worn by faces the verdict did not decide, else `first`, the
/// first-written side. `worn` is [`worn_outside`] per side. A key census:
/// it reads no value, so it cannot choose by rounding between two
/// descriptions in band of each other.
pub(crate) fn kept_of<S: Copy + PartialEq>(worn: [(S, bool); 2], first: S) -> S {
    match worn {
        [(a, true), (_, false)] => a,
        [(_, false), (b, true)] => b,
        _ => first,
    }
}

/// **The faces a verdict moves, from those it decided on the moving
/// side**: `decided`, and every face joined to one of these by an edge
/// across which the two wear one record, to closure. The closure stays
/// on the record each starts from and never follows an edge both of
/// whose sides are one face (a seam, a strut). Keys only.
///
/// # Panics
///
/// Where a face's loop does not walk: a link, which every door keeps
/// live.
pub(crate) fn moved_closure<T: Real>(body: &Body<T>, decided: &[FaceKey]) -> BTreeSet<FaceKey> {
    let mut out: BTreeSet<FaceKey> = decided
        .iter()
        .copied()
        .filter(|&f| body.get_face(f).is_some())
        .collect();
    let mut pending: Vec<FaceKey> = out.iter().copied().collect();
    while let Some(f) = pending.pop() {
        let key = body.proven_face(f).record().surface;
        for he in body.proven_face(f).half_edges() {
            let Some(mate) = body.mate(he).and_then(|m| body.face_of_half_edge(m)) else {
                continue;
            };
            if mate == f || out.contains(&mate) {
                continue;
            }
            if body.get_face(mate).is_some_and(|m| m.surface == key) {
                out.insert(mate);
                pending.push(mate);
            }
        }
    }
    out
}

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// **Leaves the faces a verdict decided one carrier on one record**
    /// (module docs): each face of `token`'s moved set that still wears
    /// its other record is re-pointed onto the kept one through the
    /// describing re-chart ([`crate::attach::Rechart::shared`]), its
    /// sense restated so its material side is unchanged. The edges the
    /// move strands are restated: a description naming the other record
    /// names the kept one ([`Body::carried_redescriptions`]), and one
    /// naming the pair of them between two faces now both on the kept
    /// record rests in its chart. Every moved face's boundary is
    /// certified on the kept chart at the ordinary write gate. A face
    /// the token does not move keeps its record. Reads no value.
    ///
    /// # Errors
    ///
    /// The re-chart door's
    /// ([`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing)).
    /// After a verdict decided Zero over every face it moves, a boundary
    /// that does not certify on the kept chart is a kernel defect, which
    /// the caller types as one.
    pub(crate) fn one_record(
        &mut self,
        site: crate::policy_lane::KernelSite,
        token: &OneRecord,
        tol: Tol,
    ) -> Result<(), EulerOpError> {
        self.recharted_onto(site, token, tol)
    }
}

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// **The two faces either side of `edge`, decided one carrier by the
    /// smooth join a sweep read there, left on one record**: the act
    /// ([`Body::one_record`]) on the token that reading mints. The
    /// verdict is the sweep's (the dihedral it read across `edge`,
    /// decided smooth), read nowhere again here, and its row the sweep's
    /// to record; a smooth join that is no slit faces one way across the
    /// edge, so the two faces' material sides agree. The kept record is the one alone also
    /// worn by faces outside the two ([`kept_of`]), else `first`'s, the
    /// face the sweep wrote first; the moved face's closure under
    /// same-record adjacency moves with it ([`moved_closure`]).
    ///
    /// Returns the two faces, `first`'s first, for the row's cells.
    /// `None`, and nothing written, where they already wear one record.
    ///
    /// # Errors
    ///
    /// [`BadArgument::Stale`](crate::euler::BadArgument::Stale) where
    /// `edge` does not resolve or `first` is not one of its faces, and
    /// the re-chart's ([`Body::one_record`]).
    pub fn one_record_across(
        &mut self,
        edge: crate::entity::EdgeKey,
        first: FaceKey,
        tol: Tol,
    ) -> Result<Option<[FaceKey; 2]>, EulerOpError> {
        crate::live::lookup(
            &self.edges,
            edge,
            crate::entity::EntityId::Edge,
            crate::live::Arg("edge"),
        )?;
        let faces = crate::wedge_reach::edge_faces(self, edge);
        if !faces.contains(&first) {
            return Err(EulerOpError::Argument(crate::euler::BadArgument::Stale {
                role: "first",
                key: crate::entity::EntityId::Face(first),
            }));
        }
        let second = if faces[0] == first {
            faces[1]
        } else {
            faces[0]
        };
        let (k1, k2) = (
            self.proven_face(first).record().surface,
            self.proven_face(second).record().surface,
        );
        if k1 == k2 {
            return Ok(None);
        }
        let decided: BTreeSet<FaceKey> = [first, second].into();
        let kept = kept_of(
            [
                (first, worn_outside(self, first, &decided)),
                (second, worn_outside(self, second, &decided)),
            ],
            first,
        );
        let moving = if kept == first { second } else { first };
        let (kept_key, other_key) = (
            self.proven_face(kept).record().surface,
            self.proven_face(moving).record().surface,
        );
        let charts_agree =
            self.proven_face(kept).record().sense == self.proven_face(moving).record().sense;
        let token = OneRecord::minted(
            (kept_key, other_key),
            charts_agree,
            moved_closure(self, &[moving]),
        );
        self.one_record(crate::policy_lane::KernelSite::OneRecord, &token, tol)?;
        Ok(Some([first, second]))
    }
}

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// **A merge's run of faces its margins decided one carrier, left on
    /// one record without the surgery** (a curved run the merge records
    /// as skipped): the act ([`Body::one_record`]) on the token the
    /// run's verdicts mint, moving every record the run wears besides the
    /// kept one in one move. The kept record is the one alone also worn by faces outside
    /// the run ([`kept_of`]'s rule), else its first face's; each other
    /// record's faces in the run move with their closure under
    /// same-record adjacency ([`moved_closure`]). A run faces one way
    /// across every edge it was decided over, so each record's charts
    /// face as its faces' senses say.
    ///
    /// # Errors
    ///
    /// The re-chart's ([`Body::one_record`]).
    pub(crate) fn one_record_of_run(
        &mut self,
        run: &[FaceKey],
        tol: Tol,
    ) -> Result<(), EulerOpError> {
        let record = |body: &Self, f: FaceKey| {
            let face = body.proven_face(f).record();
            (face.surface, face.sense)
        };
        let mut records: Vec<(SurfaceKey, bool)> = Vec::new();
        for &f in run {
            let r = record(self, f);
            if !records.iter().any(|&(k, _)| k == r.0) {
                records.push(r);
            }
        }
        let [first, ..] = records[..] else {
            return Ok(());
        };
        if records.len() == 1 {
            return Ok(());
        }
        let worn: Vec<SurfaceKey> = records
            .iter()
            .map(|&(k, _)| k)
            .filter(|&k| {
                self.faces()
                    .any(|(f, face)| face.surface == k && !run.contains(&f))
            })
            .collect();
        let kept = match worn[..] {
            [k] => records
                .iter()
                .copied()
                .find(|&(r, _)| r == k)
                .unwrap_or(first),
            _ => first,
        };
        let others: Vec<(SurfaceKey, bool)> = records
            .iter()
            .filter(|&&(k, _)| k != kept.0)
            .map(|&(k, sense)| (k, sense == kept.1))
            .collect();
        let on: Vec<FaceKey> = run
            .iter()
            .copied()
            .filter(|&f| record(self, f).0 != kept.0)
            .collect();
        let token = OneRecord {
            kept: kept.0,
            others,
            moved: moved_closure(self, &on),
        };
        self.one_record(crate::policy_lane::KernelSite::OneRecord, &token, tol)?;
        Ok(())
    }
}
