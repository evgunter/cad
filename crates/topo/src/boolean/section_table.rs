//! **The section table**: the section of every cross-operand surface
//! pair whose faces' boxes meet and whose kinds the section value builds
//! ([`geom_brep::pair_section()`]), read once per operation at the glue
//! door ([`super::glue::section_door`]) and held on the reduction.
//!
//! Keyed by surface pair, A's surface then B's: two faces on one carrier
//! share its geometry, so the pieces a split leaves of one face, and the
//! faces of one solid that share a carrier, read one section, and chords
//! meeting at a vertex cut the same bits. Its readers:
//!
//! - the coincidence records ([`super::glue::section_door`]): a pair
//!   whose section stands on a coincidence decided `Zero` is recorded
//!   from it, never by a second reading;
//! - the interior-loop guard ([`super::section_cert::of_pair_section`]),
//!   which takes the pair's components, counted or carried, and decides
//!   its verdict from them with the reduction's events, so the verdict is
//!   the guard's, not the table's;
//! - the germ mint ([`super::join::germ_branch`]), which takes a site's
//!   component, its foot's on the carried section;
//! - the join's matcher, which takes a site's foot on its component,
//!   once per germ ([`SectionTable::germ_foot`]), keyed by the germ's
//!   operand, its half-edge and the pair it was read on.
//!
//! The table is classified when it is built, and a pair is carried on
//! its first read that needs a carrier or a list no closed form counts,
//! through the scalar's march door ([`crate::AtRestPolicy::ssi_lane`])
//! held from the build: so only a pair with a germ, or one the guard
//! reads whose components are the march's, is ever marched. The table is read
//! through a [`SectionReader`], which takes the operation's one band at
//! its making, so every pair is classified and carried at that band. An
//! empty table (`Default`) is a reduction that read no section: a
//! sweep-only door's.

use std::cell::{OnceCell, RefCell};
use std::collections::BTreeMap;

use geom_brep::{
    CarriedComponent, CarriedSection, CarryError, PairSection, Seated, SectionError, SiteFoot,
    SiteRefusal, SsiLane,
};
use geom_core::{Band, Indeterminate, Real};

use super::Operand;
use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey};
use crate::geometry::SurfaceKey;

/// One surface pair's section, and the band its reader classified it at
/// (the table's one band), which it is carried at.
#[derive(Debug)]
struct Held<T: Real> {
    read: Result<PairSection<T>, SectionError>,
    band: Band,
    carried: OnceCell<Result<CarriedSection<T>, CarryError>>,
    /// Each carried component, seated on the first ask for it
    /// ([`SectionTable::seated`]).
    seated: OnceCell<Vec<OnceCell<Seated<T>>>>,
}

/// **The table's key**: A's surface, then B's. Both are keys of their
/// own body's arena, and the two arenas reuse keys, so the order is the
/// whole of the key's meaning. The two bodies are one type, so the
/// order is not a type: every reader names A's face then B's, the
/// `(fa, fb)` order of every face-pair reading in the Boolean, and the
/// key is made in one place ([`SectionTable::key`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PairKey {
    a: SurfaceKey,
    b: SurfaceKey,
}

/// A germ's place on its section: its site's foot, and whether it leaves
/// that foot toward increasing parameter ([`SectionTable::germ_foot`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct GermFoot<T: Real> {
    pub(crate) foot: SiteFoot<T>,
    pub(crate) up: bool,
}

/// Why a germ has no place on its section.
#[derive(Clone, Debug)]
pub(crate) enum FootRefusal {
    /// The site's own reading.
    Site(SiteRefusal),
    /// The germ's heading along its component, undecided.
    Heading(Indeterminate),
    /// A desync between the germ and its section.
    Desync(&'static str),
}

/// **A germ's key in the foot cache**: the operand whose half-edge
/// faces the germ, that half-edge, and the pair the foot is read on.
/// The two operands' arenas reuse keys, so the half-edge alone does not
/// name a germ; and the pair is the section the foot lies on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FootKey {
    side: Operand,
    he: HalfEdgeKey,
    pair: PairKey,
}

/// **The section table** (module docs).
#[derive(Debug)]
pub(crate) struct SectionTable<T: Real> {
    pairs: BTreeMap<PairKey, Held<T>>,
    lane: Option<SsiLane<T>>,
    feet: RefCell<BTreeMap<FootKey, Result<GermFoot<T>, FootRefusal>>>,
}

/// **The table's one reader**: it reads pairs into a table at the band
/// it was made with, so one table holds one band
/// ([`SectionTable::reader`]); [`Self::finish`] hands the table on.
#[derive(Debug)]
pub(crate) struct SectionReader<T: Real> {
    table: SectionTable<T>,
    band: Band,
}

impl<T: Real> Default for SectionTable<T> {
    fn default() -> Self {
        Self {
            pairs: BTreeMap::new(),
            lane: None,
            feet: RefCell::new(BTreeMap::new()),
        }
    }
}

impl<T: geom_core::Decide> SectionReader<T> {
    /// Reads surface `ka` of A against `kb` of B at the reader's band,
    /// the band the pair is classified and carried at: the section on the
    /// pair's first read, the held one after, at the same band. `None`
    /// where the value does not build the kind pair. The glue door's
    /// filter (`pair_section::serves`) keeps a plane and a spline out:
    /// read here, with no march door, their section would hold
    /// `SectionError::NoMarch`, which the guard reads as R-lane
    /// (`Section::NoLane`).
    pub(crate) fn read(
        &mut self,
        (ka, sa): (SurfaceKey, &geom::Surface<T>),
        (kb, sb): (SurfaceKey, &geom::Surface<T>),
    ) -> Option<&Result<PairSection<T>, SectionError>> {
        use std::collections::btree_map::Entry;
        let band = self.band;
        match self.table.pairs.entry(PairKey { a: ka, b: kb }) {
            Entry::Occupied(held) => Some(&held.into_mut().read),
            Entry::Vacant(slot) => {
                let read = match geom_brep::pair_section(sa, sb, band) {
                    Ok(geom_brep::SectionRead::Elsewhere(_)) => return None,
                    Ok(geom_brep::SectionRead::Built(section)) => Ok(section),
                    Err(e) => Err(e),
                };
                Some(
                    &slot
                        .insert(Held {
                            read,
                            band,
                            carried: OnceCell::new(),
                            seated: OnceCell::new(),
                        })
                        .read,
                )
            }
        }
    }

    /// The table read.
    pub(crate) fn finish(self) -> SectionTable<T> {
        self.table
    }
}

impl<T: geom_core::Decide> SectionTable<T> {
    /// The reader of a table that classifies every pair at `band`, the
    /// operation's, and carries through `lane`.
    pub(crate) fn reader(lane: Option<SsiLane<T>>, band: Band) -> SectionReader<T> {
        SectionReader {
            table: Self {
                lane,
                ..Self::default()
            },
            band,
        }
    }

    /// The section held for A's face `fa` against B's `fb`, by their
    /// surfaces' keys in `a` and `b`. `None` where the table holds no
    /// section for the pair: its kinds are read elsewhere.
    pub(crate) fn held(
        &self,
        a: (&Body<T>, FaceKey),
        b: (&Body<T>, FaceKey),
    ) -> Option<&Result<PairSection<T>, SectionError>> {
        self.entry(a, b).map(|h| &h.read)
    }

    /// **The pair's section carried** ([`PairSection::carry`]), on the
    /// first read, at the band the pair was read at: `None` where the
    /// table holds no section for the pair, the section's own refusal
    /// beside the carrying's.
    pub(crate) fn carried(
        &self,
        a: (&Body<T>, FaceKey),
        b: (&Body<T>, FaceKey),
    ) -> Option<Result<&CarriedSection<T>, Uncarried<'_>>> {
        let held = self.entry(a, b)?;
        let section = match &held.read {
            Ok(section) => section,
            Err(e) => return Some(Err(Uncarried::Section(e))),
        };
        Some(
            held.carried
                .get_or_init(|| section.carry(self.lane, held.band))
                .as_ref()
                .map_err(Uncarried::Carry),
        )
    }

    /// **Component `k` of the pair's carried section, seated**
    /// ([`PairSection::seat`]) at its parameter `at`, on the first ask
    /// for that component, and held for the operation: every chord cut
    /// from the component reads the one seated carrier, so both
    /// operands' chords cut the same bits. `None` where the table holds
    /// no section for the pair.
    ///
    /// **`at` is the component's seat, the same on every ask**: the join
    /// reads it once per component, from the section and its sites alone
    /// (`join::carried_seats`), before any chord is cut, so the seat held
    /// is the one every later ask names, whichever segment asks first.
    pub(crate) fn seated(
        &self,
        a: (&Body<T>, FaceKey),
        b: (&Body<T>, FaceKey),
        k: usize,
        at: T,
    ) -> Option<Result<&CarriedComponent<T>, Uncarried<'_>>> {
        let held = self.entry(a, b)?;
        let section = match &held.read {
            Ok(section) => section,
            Err(e) => return Some(Err(Uncarried::Section(e))),
        };
        let carried = match self.carried(a, b)? {
            Ok(carried) => carried,
            Err(e) => return Some(Err(e)),
        };
        let Some(component) = carried.components().get(k) else {
            return Some(Err(Uncarried::Component));
        };
        let slots = held.seated.get_or_init(|| {
            carried
                .components()
                .iter()
                .map(|_| OnceCell::new())
                .collect()
        });
        Some(
            slots[k]
                .get_or_init(|| section.seat(self.lane, component, at, held.band))
                .as_ref()
                .map_err(Uncarried::Carry),
        )
    }

    /// **The place on the section of A's face `fa` against B's `fb` of
    /// the germ that operand `side`'s half `he` faces**, read by `read`
    /// on the first ask and held for the operation: a germ's site and
    /// direction do not move while the join matches, so every candidate
    /// pairing reads the one answer. Held by operand, half-edge and pair
    /// ([`FootKey`]). A desync where a face has no surface.
    pub(crate) fn germ_foot(
        &self,
        (side, he): (Operand, HalfEdgeKey),
        a: (&Body<T>, FaceKey),
        b: (&Body<T>, FaceKey),
        read: impl FnOnce() -> Result<GermFoot<T>, FootRefusal>,
    ) -> Result<GermFoot<T>, FootRefusal> {
        let Some(pair) = Self::key(a, b) else {
            return Err(FootRefusal::Desync("a germ's face has no surface"));
        };
        let key = FootKey { side, he, pair };
        if let Some(held) = self.feet.borrow().get(&key) {
            return held.clone();
        }
        let answer = read();
        self.feet.borrow_mut().insert(key, answer.clone());
        answer
    }

    fn entry(&self, a: (&Body<T>, FaceKey), b: (&Body<T>, FaceKey)) -> Option<&Held<T>> {
        self.pairs.get(&Self::key(a, b)?)
    }

    /// The key of A's face `fa` against B's `fb`.
    fn key((a, fa): (&Body<T>, FaceKey), (b, fb): (&Body<T>, FaceKey)) -> Option<PairKey> {
        let surface = |body: &Body<T>, f| body.get_face(f).map(|face| face.surface);
        Some(PairKey {
            a: surface(a, fa)?,
            b: surface(b, fb)?,
        })
    }
}

/// Why a held pair's section is not carried: the section's own refusal,
/// or the carrying's.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Uncarried<'t> {
    /// The pair's section refused at the glue door.
    Section(&'t SectionError),
    /// The section did not carry, or a component did not seat.
    Carry(&'t CarryError),
    /// A germ names a component the carried section does not have.
    Component,
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{FootRefusal, SectionTable};
    use crate::boolean::Operand;
    use crate::entity::HalfEdgeKey;

    /// A body, and two of its faces on different surfaces: built twice
    /// alike, its keys coincide across the two builds.
    fn twins() -> (crate::body::Body<f64>, crate::FaceKey, crate::FaceKey) {
        let mut body = crate::fixtures::ops_genus2(geom_core::Tol::witness());
        let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
        let (f, g) = (faces[0], faces[1]);
        let other = body.add_surface(crate::fixtures::test_surface(geom_core::Point3::origin()));
        body.get_face_mut(g).expect("a face of the body").surface = other;
        (body, f, g)
    }

    /// **A germ's foot is its own operand's, on its own pair**: the two
    /// operands' arenas reuse keys, so an A germ and a B germ can face
    /// half-edges with one key, and each reads its own foot; one
    /// operand's half-edge read on two pairs reads each pair's. (Red on
    /// a cache keyed by the half-edge alone, which served the second
    /// read the first's answer.)
    #[test]
    fn two_operands_germs_on_one_key_read_their_own_feet() {
        let table = SectionTable::<f64>::default();
        let he = HalfEdgeKey::default();
        let (a, fa, ga) = twins();
        let (b, fb, _) = twins();
        let foot = |side, (fa, fb), what| {
            table.germ_foot((side, he), (&a, fa), (&b, fb), || {
                Err(FootRefusal::Desync(what))
            })
        };
        assert!(matches!(
            foot(Operand::A, (fa, fb), "A's"),
            Err(FootRefusal::Desync("A's"))
        ));
        let b_read = foot(Operand::B, (fa, fb), "B's");
        assert!(
            matches!(b_read, Err(FootRefusal::Desync("B's"))),
            "B read {b_read:?}"
        );
        let other = foot(Operand::A, (ga, fb), "A's other pair");
        assert!(
            matches!(other, Err(FootRefusal::Desync("A's other pair"))),
            "the other pair read {other:?}"
        );
        assert!(
            matches!(
                foot(Operand::A, (fa, fb), "again"),
                Err(FootRefusal::Desync("A's"))
            ),
            "a second ask reads the held answer"
        );
    }
}
