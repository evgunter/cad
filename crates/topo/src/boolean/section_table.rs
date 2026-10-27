//! **The section table**: the section of every cross-operand surface
//! pair whose faces' boxes meet and whose kinds the section value builds
//! ([`geom_brep::pair_section`]), read once per operation at the glue
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
//!   which takes the pair's components and decides its verdict from them
//!   with the reduction's events, so the verdict is the guard's, not the
//!   table's;
//! - the germ mint ([`super::join::germ_branch`]), which takes a site's
//!   component;
//! - the join's matcher, which takes a site's foot on its component,
//!   once per germ ([`SectionTable::germ_foot`]).
//!
//! The table is classified when it is built, and a pair is carried on
//! its first read that needs a carrier, through the scalar's march door
//! ([`crate::AtRestPolicy::ssi_lane`]) held from the build: so only a
//! pair whose germs the join matches is ever marched. An empty table
//! (`Default`) is a reduction that read no section: a sweep-only door's.

use std::cell::{OnceCell, RefCell};
use std::collections::BTreeMap;

use geom_brep::{
    CarriedSection, CarryError, PairSection, SectionError, SiteFoot, SiteRefusal, SsiLane,
};
use geom_core::{Band, Indeterminate, Real};

use crate::body::Body;
use crate::entity::{FaceKey, HalfEdgeKey};
use crate::geometry::SurfaceKey;

/// One surface pair's section, read at the band it was read at.
#[derive(Debug)]
struct Held<T: Real> {
    read: Result<PairSection<T>, SectionError>,
    band: Band,
    carried: OnceCell<Result<CarriedSection<T>, CarryError>>,
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

/// **The section table** (module docs).
#[derive(Debug)]
pub(crate) struct SectionTable<T: Real> {
    pairs: BTreeMap<PairKey, Held<T>>,
    lane: Option<SsiLane<T>>,
    feet: RefCell<BTreeMap<HalfEdgeKey, Result<GermFoot<T>, FootRefusal>>>,
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

impl<T: geom_core::Decide> SectionTable<T> {
    /// An empty table that carries through `lane`.
    pub(crate) fn new(lane: Option<SsiLane<T>>) -> Self {
        Self {
            lane,
            ..Self::default()
        }
    }

    /// Reads surface `ka` of A against `kb` of B at `band`, the band the
    /// pair is classified and carried at: the section on the pair's first
    /// read, the held one after. `None` where the value does not build
    /// the kind pair.
    pub(crate) fn read(
        &mut self,
        (ka, sa): (SurfaceKey, &geom::Surface<T>),
        (kb, sb): (SurfaceKey, &geom::Surface<T>),
        band: Band,
    ) -> Option<&Result<PairSection<T>, SectionError>> {
        use std::collections::btree_map::Entry;
        match self.pairs.entry(PairKey { a: ka, b: kb }) {
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
                        })
                        .read,
                )
            }
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

    /// **The germ facing half `he`'s place on its section**, read by
    /// `read` on the first ask and held for the operation: a germ's site
    /// and direction do not move while the join matches, so every
    /// candidate pairing reads the one answer.
    pub(crate) fn germ_foot(
        &self,
        he: HalfEdgeKey,
        read: impl FnOnce() -> Result<GermFoot<T>, FootRefusal>,
    ) -> Result<GermFoot<T>, FootRefusal> {
        if let Some(held) = self.feet.borrow().get(&he) {
            return held.clone();
        }
        let answer = read();
        self.feet.borrow_mut().insert(he, answer.clone());
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
    /// The section did not carry.
    Carry(&'t CarryError),
}
