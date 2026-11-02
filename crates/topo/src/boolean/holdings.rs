//! **Which member faces each finished face holds area of** ([`Holdings`]).
//!
//! A boolean's doors publish, for each face of the finished body, exactly
//! the set of member faces whose area it holds, or a typed reason it
//! cannot say ([`Held`]), and the same for the region each discarded
//! face lost ([`BooleanNaming::discards`](super::BooleanNaming::discards)).
//! A member face is the member's place and the face in that member's
//! keys ([`MemberFace`]); the pair boolean's two members are its
//! operands, A at place 0 and B at place 1.
//!
//! **The pair door** fills them from its records alone
//! ([`Holdings::of_records`]): a fragment holds the operand face it is a
//! fragment of; a kept fragment also holds the face of every dropped
//! fragment a [`SubstitutedFace`](super::SubstitutedFace) row says it
//! holds, and the dropped face an [`UncutFace`](super::UncutFace) row
//! names; a merged face holds what each face it absorbed holds.
//!
//! **The n-ary door** fills them per fold step (`nary`'s `lift`): a
//! fragment of the joining member's face holds that face; a fragment of
//! an accumulation face holds all the accumulation face holds where that
//! is one member face, and otherwise exactly those of them it meets in
//! area ([`super::overlap`]). A piece lies inside the face it was cut from
//! and can hold only what that face holds, and each reading is the
//! geometric truth, so the holdings do not depend on member order. Which
//! pieces a fold makes does, and so where an in-band reading can refuse.

use std::collections::{BTreeMap, BTreeSet};

use super::nary::{FaceDescent, MemberFace};
use super::overlap::OverlapRefusal;
use super::{BooleanNaming, Operand, OperandKeys};
use crate::body::Body;
use crate::entity::FaceKey;

/// **What one face holds**: exactly the member faces whose area it
/// holds, or why that could not be read.
#[derive(Clone, Debug)]
pub enum Held {
    /// The member faces, each one whose area the face holds.
    Faces(BTreeSet<MemberFace>),
    /// A member face whose area the face might hold could not be read
    /// against it.
    Refused(HeldRefusal),
}

/// **Why a face's holdings could not be read**: the member face it was
/// read against, and the overlap door's refusal.
#[derive(Clone, Debug)]
pub struct HeldRefusal {
    /// The member face.
    pub member_face: MemberFace,
    /// The refusal.
    pub refusal: OverlapRefusal,
}

impl Held {
    /// The member faces, where read.
    #[must_use]
    pub fn faces(&self) -> Option<&BTreeSet<MemberFace>> {
        match self {
            Self::Faces(faces) => Some(faces),
            Self::Refused(_) => None,
        }
    }
}

/// **Per finished face, and per discarded face, what it holds** (module
/// docs).
#[derive(Clone, Debug, Default)]
pub struct Holdings {
    /// Finished face → what it holds.
    faces: BTreeMap<FaceKey, Held>,
    /// What the region each discard row lost held, by the row's place in
    /// [`BooleanNaming::discards`].
    lost: Vec<Held>,
}

impl Holdings {
    /// What finished face `f` holds; `None` for a face the body does not
    /// hold.
    #[must_use]
    pub fn of(&self, f: FaceKey) -> Option<&Held> {
        self.faces.get(&f)
    }

    /// Every finished face, ascending, with what it holds.
    pub fn faces(&self) -> impl Iterator<Item = (FaceKey, &Held)> {
        self.faces.iter().map(|(&f, h)| (f, h))
    }

    /// What the region discard row `row` lost held.
    #[must_use]
    pub fn lost(&self, row: usize) -> Option<&Held> {
        self.lost.get(row)
    }

    /// The region each discard row lost, in row order.
    #[must_use]
    pub fn lost_rows(&self) -> &[Held] {
        &self.lost
    }

    /// The holdings of member `member`'s own body: each face its own.
    #[must_use]
    pub fn of_member<T: geom_core::Real>(member: usize, body: &Body<T>) -> Self {
        Self {
            faces: body
                .faces()
                .map(|(f, _)| (f, Held::Faces(BTreeSet::from([(member, f)]))))
                .collect(),
            lost: Vec::new(),
        }
    }

    pub(crate) fn from_parts(faces: BTreeMap<FaceKey, Held>, lost: Vec<Held>) -> Self {
        Self { faces, lost }
    }

    /// **The pair boolean's holdings, read from its records** (module
    /// docs): `naming`'s rows over `body`, its result, operand A at place
    /// 0 and B at place 1.
    ///
    /// # Panics
    ///
    /// Where a [`SubstitutedFace`](super::SubstitutedFace) or
    /// [`UncutFace`](super::UncutFace) row's kept face reads into no face
    /// of the result: the boolean records only a face it keeps there.
    #[must_use]
    pub fn of_records<T: geom_core::Real>(naming: &BooleanNaming, body: &Body<T>) -> Self {
        let descent = FaceDescent::of(naming);
        let place = |operand: Operand| usize::from(operand == Operand::B);
        let root = |operand: Operand, f: FaceKey| (place(operand), descent.clone_face(operand, f));
        // A kept face's clone key → its key in the result before the
        // merge.
        let graft: BTreeMap<FaceKey, FaceKey> = naming.graft_faces.iter().copied().collect();
        let in_result = |operand: Operand, f: FaceKey| match operand {
            Operand::A => match naming.a_keys {
                OperandKeys::Direct => Some(f),
                OperandKeys::Grafted | OperandKeys::Absent => None,
            },
            Operand::B => match naming.b_keys {
                OperandKeys::Direct => Some(f),
                OperandKeys::Grafted => graft.get(&f).copied(),
                OperandKeys::Absent => None,
            },
        };
        let mut held_through: BTreeMap<FaceKey, BTreeSet<MemberFace>> = BTreeMap::new();
        let rows = naming
            .substituted
            .iter()
            .map(|s| (s.operand, s.face, s.by))
            .chain(naming.uncut.iter().map(|u| (u.operand, u.face, u.by)));
        for (dropped, face, by) in rows {
            let kept = in_result(dropped.other(), by).unwrap_or_else(|| {
                unreachable!(
                    "a dropped {dropped:?} face {face:?} is held by kept face {by:?}, which reads \
                     into no face of the result, though the boolean records only a face it keeps"
                )
            });
            held_through
                .entry(kept)
                .or_default()
                .insert(root(dropped, face));
        }
        let merged = descent.merged();
        let faces = body
            .faces()
            .map(|(f, _)| {
                let mut set = BTreeSet::new();
                for &g in merged.get(&f).map_or(&[f][..], Vec::as_slice) {
                    let (operand, r) = descent.result_face(g);
                    set.insert((place(operand), r));
                    if let Some(more) = held_through.get(&g) {
                        set.extend(more.iter().copied());
                    }
                }
                (f, Held::Faces(set))
            })
            .collect();
        let lost = naming
            .discards
            .iter()
            .map(|d| Held::Faces(BTreeSet::from([root(d.operand, d.face)])))
            .collect();
        Self { faces, lost }
    }
}
