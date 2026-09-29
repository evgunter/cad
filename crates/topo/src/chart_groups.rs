//! **A face scope's faces, grouped by the chart each wears.**
//!
//! A chart — one [`SurfaceKey`] — is body-wide: faces of any number of
//! solids may wear it, and a shared key says those faces lie on one
//! locus by construction (the contact ladder's structural rung,
//! `crates/topo/README.md`). What a door moves, opens or reads as one
//! is a chart's wearers IN ITS OWN SCOPE — one solid, one shell, one
//! selection — never the body's: a wearer outside the scope is another
//! solid's face, which no edge of the scope reaches.
//!
//! [`ChartGroups::within`] is the one spelling of that grouping. It
//! takes the scope, so a door cannot group without naming one; a door
//! whose scope IS the whole body says so through [`ChartGroups::of_body`].

use geom_core::Real;
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::FaceKey;
use crate::geometry::SurfaceKey;

/// A scope's faces grouped by surface key: groups in the order their
/// first face arrived, each group's faces in arrival order — so a scope
/// handed over in face-arena order yields arena-ordered groups.
#[derive(Clone, Debug)]
pub(crate) struct ChartGroups {
    groups: Vec<(SurfaceKey, Vec<FaceKey>)>,
    index: SecondaryMap<SurfaceKey, usize>,
}

impl ChartGroups {
    /// Groups `scope`'s faces by the chart each wears.
    ///
    /// # Errors
    ///
    /// The first face of `scope` that does not resolve.
    pub(crate) fn within<T: Real>(
        body: &Body<T>,
        scope: impl IntoIterator<Item = FaceKey>,
    ) -> Result<Self, FaceKey> {
        let mut out = Self::empty();
        for face in scope {
            out.push(body.get_face(face).ok_or(face)?.surface, face);
        }
        Ok(out)
    }

    /// Groups EVERY face of `body`, in face-arena order: the scope of a
    /// door whose scope is the whole body. The faces are read off the
    /// live arena with their data, so nothing can fail to resolve.
    pub(crate) fn of_body<T: Real>(body: &Body<T>) -> Self {
        let mut out = Self::empty();
        for (face, data) in body.faces() {
            out.push(data.surface, face);
        }
        out
    }

    fn empty() -> Self {
        Self {
            groups: Vec::new(),
            index: SecondaryMap::new(),
        }
    }

    fn push(&mut self, key: SurfaceKey, face: FaceKey) {
        match self.index.get(key) {
            Some(&i) => self.groups[i].1.push(face),
            None => {
                self.index.insert(key, self.groups.len());
                self.groups.push((key, vec![face]));
            }
        }
    }

    /// Every group, with the chart it wears.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (SurfaceKey, &[FaceKey])> {
        self.groups.iter().map(|(k, g)| (*k, g.as_slice()))
    }

    /// The scope's wearers of `key`; empty when the scope has none.
    pub(crate) fn of(&self, key: SurfaceKey) -> &[FaceKey] {
        self.index
            .get(key)
            .map_or(&[], |&i| self.groups[i].1.as_slice())
    }
}
