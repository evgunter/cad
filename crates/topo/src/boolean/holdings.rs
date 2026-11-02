//! **Which member faces each finished face holds area of** ([`Holdings`]).
//!
//! A boolean publishes, for each face of the finished body, exactly the
//! set of member faces whose area it holds, or a typed reason it cannot
//! say ([`Held`]); the same for the region each discarded face lost
//! ([`BooleanNaming::discards`](super::BooleanNaming::discards)), and for
//! the region of each dropped face a containment fallback could not
//! divide ([`BooleanNaming::uncut`](super::BooleanNaming::uncut)). A
//! member face is the member's place and the face in that member's keys
//! ([`MemberFace`]); the pair boolean's two members are its operands, A
//! at place 0 and B at place 1.
//!
//! **The pair door** reads them from its records once its naming is
//! final ([`Holdings::of_records`]), at the one place a boolean result
//! leaves the door: a fragment holds the operand face it is a fragment
//! of; a merged face holds what each face it absorbed holds; and a kept
//! face holds the face of each dropped fragment a
//! [`SubstitutedFace`](super::SubstitutedFace) row, or the dropped face
//! an [`UncutFace`](super::UncutFace) row, says it holds. That last read
//! is the row's alone where its kept face is the only face the result
//! holds of its operand face; where the result holds that operand face
//! as several faces (cut before the substitution, or parted along a line
//! after it), each of them holds the dropped face where its region meets
//! it ([`super::overlap`]).
//!
//! **The n-ary door** reads them per fold step (`nary`'s `lift`): a
//! fragment of the joining member's face holds that face; a fragment of
//! an accumulation face holds all the accumulation face holds where that
//! is one member face, and otherwise exactly those of them it meets in
//! area. A piece lies inside the face it was cut from and can hold only
//! what that face holds, and each reading is the geometric truth, so the
//! holdings do not depend on member order. Which pieces a fold makes
//! does, and so where an in-band reading can refuse.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Band, Decide};

use super::nary::{FaceDescent, MemberFace};
use super::overlap::{OverlapRefusal, overlap_as};
use super::{BooleanNaming, BooleanOp, Operand};
use crate::body::Body;
use crate::entity::FaceKey;

/// **What one face holds**: exactly the member faces whose area it
/// holds, or why that could not be read.
#[derive(Clone, Debug, PartialEq)]
pub enum Held {
    /// The member faces, each one whose area the face holds.
    Faces(BTreeSet<MemberFace>),
    /// What the face holds could not be read.
    Refused(HeldRefusal),
}

/// **Why a face's holdings could not be read.**
///
/// Two refusals are equal where they refuse at one face for one kind of
/// reason; the reason's diagnostics are not compared.
#[derive(Clone, Debug)]
pub enum HeldRefusal {
    /// A member face the face might hold could not be read against it:
    /// the member face, and the overlap door's refusal.
    Overlap {
        /// The member face.
        member_face: MemberFace,
        /// The refusal.
        refusal: OverlapRefusal,
    },
    /// A fold step's pair boolean could not read whether the face holds
    /// area of an accumulation face: that face, in the accumulation's
    /// keys, and the overlap door's refusal.
    Accumulation {
        /// The accumulation face.
        face: FaceKey,
        /// The refusal.
        refusal: OverlapRefusal,
    },
    /// The region an [`UncutFace`](super::UncutFace) row's dropped face
    /// lost is no face the fold step cut, so where the dropped face held
    /// several member faces, which of them that region held is not read:
    /// the dropped face, in the accumulation's keys.
    UncutRemainder {
        /// The accumulation face.
        face: FaceKey,
    },
    /// A lost region of a face of a sphere shell a re-chart turned
    /// before the boolean ran: the turn moved the face over the input
    /// shell's faces, so which of them the region held is not read
    /// (`BooleanNaming::recharted`). The turned face, at its operand's
    /// place.
    Recharted {
        /// The turned face.
        face: MemberFace,
    },
}

impl PartialEq for HeldRefusal {
    fn eq(&self, other: &Self) -> bool {
        let kind = core::mem::discriminant::<OverlapRefusal>;
        match (self, other) {
            (
                Self::Overlap {
                    member_face: m,
                    refusal: r,
                },
                Self::Overlap {
                    member_face: n,
                    refusal: s,
                },
            ) => m == n && kind(r) == kind(s),
            (
                Self::Accumulation {
                    face: f,
                    refusal: r,
                },
                Self::Accumulation {
                    face: g,
                    refusal: s,
                },
            ) => f == g && kind(r) == kind(s),
            (Self::UncutRemainder { face: f }, Self::UncutRemainder { face: g }) => f == g,
            (Self::Recharted { face: f }, Self::Recharted { face: g }) => f == g,
            _ => false,
        }
    }
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

/// **Per finished face, per discarded face and per uncut face, what it
/// holds** (module docs).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Holdings {
    /// Finished face → what it holds.
    faces: BTreeMap<FaceKey, Held>,
    /// What the region each discard row lost held, by the row's place in
    /// [`BooleanNaming::discards`].
    lost: Vec<Held>,
    /// What the dropped face of each uncut row held, by the row's place
    /// in [`BooleanNaming::uncut`].
    uncut: Vec<Held>,
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

    /// What the dropped face of each uncut row held, in row order: the
    /// face whose region outside its kept faces the result lost, in no
    /// discard row.
    #[must_use]
    pub fn uncut_rows(&self) -> &[Held] {
        &self.uncut
    }

    /// The holdings of member `member`'s own body: each face its own.
    pub(crate) fn of_member<T: geom_core::Real>(member: usize, body: &Body<T>) -> Self {
        Self {
            faces: body
                .faces()
                .map(|(f, _)| (f, Held::Faces(BTreeSet::from([(member, f)]))))
                .collect(),
            lost: Vec::new(),
            uncut: Vec::new(),
        }
    }

    /// A body's holdings before the door reads them: what a result
    /// carries until it leaves the door ([`Self::of_records`]).
    pub(crate) fn unread() -> Self {
        Self::default()
    }

    /// Holdings stated outright: a test's fixture for a record it built
    /// by hand, which no door read.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn fixture(faces: BTreeMap<FaceKey, Held>, lost: Vec<Held>, uncut: Vec<Held>) -> Self {
        Self::from_parts(faces, lost, uncut)
    }

    pub(crate) fn from_parts(
        faces: BTreeMap<FaceKey, Held>,
        lost: Vec<Held>,
        uncut: Vec<Held>,
    ) -> Self {
        Self { faces, lost, uncut }
    }

    /// Every member face these holdings name, each once.
    pub(crate) fn named(&self) -> BTreeSet<MemberFace> {
        let mut out = BTreeSet::new();
        for held in self.faces.values().chain(&self.lost).chain(&self.uncut) {
            if let Held::Faces(f) = held {
                out.extend(f.iter().copied());
            }
        }
        out
    }

    /// **The pair boolean's holdings, read from its final records**
    /// (module docs): `naming`'s rows over `body`, its result, and
    /// `operands`, A's then B's, whose faces the rows root in.
    ///
    /// # Panics
    ///
    /// Where a [`SubstitutedFace`](super::SubstitutedFace) or
    /// [`UncutFace`](super::UncutFace) row's kept face reads into no face
    /// the result holds, or a finished face holds no operand face: the
    /// boolean keeps every kept face, and every finished face is a piece
    /// of one.
    pub(crate) fn of_records<T: Decide>(
        op: BooleanOp,
        naming: &BooleanNaming,
        body: &Body<T>,
        (operands, clones): ([&Body<T>; 2], [Option<&Body<T>>; 2]),
        band: Band,
    ) -> Self {
        // A subtract's result holds B's faces turned: a finished face is
        // read against a face of B the other way round.
        let turned = |place: usize| place == 1 && op == BooleanOp::Subtract;
        let descent = FaceDescent::of(naming);
        let place = |operand: Operand| usize::from(operand == Operand::B);
        let root = |operand: Operand, f: FaceKey| (place(operand), descent.clone_face(operand, f));
        // The operand faces a root is: itself, or, for a face of a sphere
        // shell a re-chart turned, the faces of the input shell it was
        // turned over (`BooleanNaming::recharted`), any of which its
        // region may meet.
        let input = |(p, r): MemberFace| -> Vec<MemberFace> {
            naming.recharted[p]
                .input_faces(super::Cell::Face(r))
                .map_or_else(|| vec![(p, r)], |fs| fs.iter().map(|&f| (p, f)).collect())
        };
        let merged = descent.merged();
        // Every face of the result before the merge, with the operand face
        // it is a piece of.
        let constituents =
            |f: FaceKey| -> Vec<FaceKey> { merged.get(&f).cloned().unwrap_or_else(|| vec![f]) };
        let mut pieces: BTreeMap<MemberFace, usize> = BTreeMap::new();
        for (f, _) in body.faces() {
            for g in constituents(f) {
                let (operand, r) = descent.result_face(g);
                *pieces.entry((place(operand), r)).or_default() += 1;
            }
        }
        // Kept operand face → the dropped operand faces some piece of it
        // holds.
        let mut held_through: BTreeMap<MemberFace, BTreeSet<MemberFace>> = BTreeMap::new();
        let rows = naming
            .substituted
            .iter()
            .map(|s| (s.operand, s.face, s.by))
            .chain(naming.uncut.iter().map(|u| (u.operand, u.face, u.by)));
        for (dropped, face, by) in rows {
            let kept = root(dropped.other(), by);
            if !pieces.contains_key(&kept) {
                unreachable!(
                    "a dropped {dropped:?} face {face:?} is held by kept face {by:?}, whose \
                     operand face {kept:?} the result holds no piece of, though the boolean \
                     keeps every kept face"
                );
            }
            held_through
                .entry(kept)
                .or_default()
                .insert(root(dropped, face));
        }
        let faces = body
            .faces()
            .map(|(f, _)| {
                let mut set = BTreeSet::new();
                let mut maybe = BTreeSet::new();
                for g in constituents(f) {
                    let (operand, r) = descent.result_face(g);
                    let kept = (place(operand), r);
                    match input(kept)[..] {
                        [one] => {
                            set.insert(one);
                        }
                        ref several => maybe.extend(several.iter().copied()),
                    }
                    if let Some(more) = held_through.get(&kept) {
                        for &m in more {
                            match input(m)[..] {
                                [one] if pieces[&kept] == 1 => {
                                    set.insert(one);
                                }
                                ref several => maybe.extend(several.iter().copied()),
                            }
                        }
                    }
                }
                let mut held = Held::Faces(BTreeSet::new());
                for m in maybe.difference(&set.clone()) {
                    match overlap_as((body, f), ((operands[m.0], m.1), turned(m.0)), band) {
                        Ok(true) => {
                            set.insert(*m);
                        }
                        Ok(false) => {}
                        Err(refusal) => {
                            held = Held::Refused(HeldRefusal::Overlap {
                                member_face: *m,
                                refusal,
                            });
                            break;
                        }
                    }
                }
                if let Held::Faces(_) = held {
                    held = Held::Faces(set);
                }
                (f, held)
            })
            .collect();
        // What the region `face` of `operand`'s clone lost held: its
        // operand face, or where a re-chart turned that face, those of
        // the input shell's faces its region meets, read in the clone the
        // operation kept (its own sense, as the operand's).
        let lost_region = |operand: Operand, face: FaceKey| {
            let r = root(operand, face);
            let several = match input(r)[..] {
                [one] => return Held::Faces(BTreeSet::from([one])),
                ref several => several.to_vec(),
            };
            let Some(clone) = clones[r.0].filter(|c| c.get_face(face).is_some()) else {
                return Held::Refused(HeldRefusal::Recharted { face: r });
            };
            let mut set = BTreeSet::new();
            for m in several {
                match overlap_as((clone, face), ((operands[m.0], m.1), false), band) {
                    Ok(true) => {
                        set.insert(m);
                    }
                    Ok(false) => {}
                    Err(refusal) => {
                        return Held::Refused(HeldRefusal::Overlap {
                            member_face: m,
                            refusal,
                        });
                    }
                }
            }
            Held::Faces(set)
        };
        let lost = naming
            .discards
            .iter()
            .map(|d| lost_region(d.operand, d.face))
            .collect();
        let uncut = naming
            .uncut
            .iter()
            .map(|u| lost_region(u.operand, u.face))
            .collect();
        Self { faces, lost, uncut }
    }
}
