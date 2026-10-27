//! **The section table**: the section of every cross-operand surface
//! pair whose faces' boxes meet and whose kinds the section value builds
//! ([`geom_brep::pair_section`]), read once per operation at the glue
//! door ([`super::glue::section_door`]) and held on the reduction.
//!
//! Keyed by surface pair, A's surface then B's: two faces on one carrier
//! share its geometry, so the pieces a split leaves of one face, and the
//! faces of one solid that share a carrier, read one section, and chords
//! meeting at a vertex cut the same bits. Per face pair it keeps which
//! faces it was read for. Its readers:
//!
//! - the coincidence records ([`super::glue::section_door`]): a pair
//!   whose section stands on a coincidence decided `Zero` is recorded
//!   from it, never by a second reading;
//! - the interior-loop guard ([`super::section_cert::of_pair_section`]),
//!   which takes the pair's components;
//! - the germ mint ([`super::join::germ_branch`]), which takes a site's
//!   component;
//! - the join's matcher, which takes a site's foot on its component.
//!
//! The table is classified when it is built, and a pair is carried on
//! its first read that needs a carrier, through the scalar's march door
//! ([`crate::AtRestPolicy::ssi_lane`]) held from the build: so only a
//! pair whose germs the join matches is ever marched.

use std::cell::OnceCell;
use std::collections::BTreeMap;

use geom_brep::{CarriedSection, CarryError, PairSection, SectionError, SsiLane};
use geom_core::{Band, Real};

use crate::body::Body;
use crate::entity::FaceKey;
use crate::geometry::SurfaceKey;

/// One surface pair's section, with the face pairs it was read for.
#[derive(Debug)]
struct Held<T: Real> {
    read: Result<PairSection<T>, SectionError>,
    faces: Vec<(FaceKey, FaceKey)>,
    carried: OnceCell<Result<CarriedSection<T>, CarryError>>,
}

/// **The section table** (module docs).
#[derive(Debug)]
pub(crate) struct SectionTable<T: Real> {
    pairs: BTreeMap<(SurfaceKey, SurfaceKey), Held<T>>,
    lane: Option<SsiLane<T>>,
    band: Option<Band>,
}

impl<T: Real> Default for SectionTable<T> {
    fn default() -> Self {
        Self {
            pairs: BTreeMap::new(),
            lane: None,
            band: None,
        }
    }
}

impl<T: geom_core::Decide> SectionTable<T> {
    /// An empty table that carries through `lane`, read at `band`.
    pub(crate) fn new(lane: Option<SsiLane<T>>, band: Band) -> Self {
        Self {
            pairs: BTreeMap::new(),
            lane,
            band: Some(band),
        }
    }

    /// Reads A's face `fa` on surface `ka` against B's `fb` on `kb`: the
    /// section on the pair's first face pair, the face pair recorded on
    /// every later one. `None` where the value does not build the kind
    /// pair.
    pub(crate) fn read(
        &mut self,
        (fa, ka, sa): (FaceKey, SurfaceKey, &geom::Surface<T>),
        (fb, kb, sb): (FaceKey, SurfaceKey, &geom::Surface<T>),
        band: Band,
    ) -> Option<&Result<PairSection<T>, SectionError>> {
        use std::collections::btree_map::Entry;
        let held = match self.pairs.entry((ka, kb)) {
            Entry::Occupied(held) => held.into_mut(),
            Entry::Vacant(slot) => {
                let read = match geom_brep::pair_section(sa, sb, band) {
                    Ok(geom_brep::SectionRead::Elsewhere(_)) => return None,
                    Ok(geom_brep::SectionRead::Built(section)) => Ok(section),
                    Err(e) => Err(e),
                };
                slot.insert(Held {
                    read,
                    faces: Vec::new(),
                    carried: OnceCell::new(),
                })
            }
        };
        held.faces.push((fa, fb));
        Some(&held.read)
    }

    /// The section held for A's face `fa` against B's `fb`, by their
    /// surfaces' keys in `a` and `b`. `None` where the table holds no
    /// section for the pair: its kinds are read elsewhere.
    pub(crate) fn held(
        &self,
        (a, fa): (&Body<T>, FaceKey),
        (b, fb): (&Body<T>, FaceKey),
    ) -> Option<&Result<PairSection<T>, SectionError>> {
        self.entry((a, fa), (b, fb)).map(|h| &h.read)
    }

    /// **The pair's section carried** ([`PairSection::carry`]), on the
    /// first read, at the band the table was built at: `None` where the
    /// table holds no section for the pair, the section's own refusal
    /// beside the carrying's.
    pub(crate) fn carried(
        &self,
        (a, fa): (&Body<T>, FaceKey),
        (b, fb): (&Body<T>, FaceKey),
    ) -> Option<Result<&CarriedSection<T>, Uncarried<'_>>> {
        let held = self.entry((a, fa), (b, fb))?;
        let section = match &held.read {
            Ok(section) => section,
            Err(e) => return Some(Err(Uncarried::Section(e))),
        };
        let band = self.band?;
        Some(
            held.carried
                .get_or_init(|| section.carry(self.lane, band))
                .as_ref()
                .map_err(Uncarried::Carry),
        )
    }

    fn entry(
        &self,
        (a, fa): (&Body<T>, FaceKey),
        (b, fb): (&Body<T>, FaceKey),
    ) -> Option<&Held<T>> {
        let key = |body: &Body<T>, f| body.get_face(f).map(|face| face.surface);
        self.pairs.get(&(key(a, fa)?, key(b, fb)?))
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
