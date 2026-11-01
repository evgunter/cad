//! The pick INDEX's numbers, flattened once for the exception that
//! carries them.
//!
//! `crate::tags::node_pick_error_tag` answers which pick refusal
//! fired and `mesh_pick_error_tag` the arm of the index refusal it
//! holds. This module answers what that arm CARRIES — the offending
//! patch position, triangle position and out-of-range position index
//! — as a record whose every field is present on every arm of
//! `NodePickError`, `None` where the arm does not carry one.
//!
//! # Why it lives outside `py`
//!
//! The match is a DRIFT ALARM: exhaustive with no wildcard, so an arm
//! added to either enum is a compile error here rather than a
//! silently unprojected payload. `crate::py` compiles only under the
//! `python` feature, so a projection sited there is an alarm that
//! does not ring on the row that runs everywhere. Sited here it rings
//! on every build — and it is what makes the arm TESTABLE at all: a
//! mesh whose triangles index outside their own position buffer is a
//! kernel-side invariant break, unauthorable through any door, so
//! `src/tests.rs` constructing the payload is the only place the
//! three numbers can be read back.
//!
//! # What is not here
//!
//! The FORWARDING arms bring their inner refusal's tag and prose and
//! not its extra attributes — a tessellation refusal's numbers stay
//! on `TessellateError`, where `Body.tessellate` raises them. The
//! index arm is not a forwarding arm: nothing raises a
//! `MeshPickError`, so its numbers have no door of their own and
//! cross here or not at all.

use pncad::select::{MeshPickError, NodePickError};

/// The three numbers a pick-index refusal names its site by, every
/// field present.
///
/// Arena-key-free, as the kernel type is: a patch position in the
/// mesh value, a triangle position within that patch, and the
/// position index that was out of range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexPayload {
    /// Position of the offending patch in the mesh's patch list.
    pub patch: Option<usize>,
    /// Position of the offending triangle within that patch.
    pub triangle: Option<usize>,
    /// The out-of-range position index.
    pub index: Option<u32>,
}

impl IndexPayload {
    /// Which attributes this payload CARRIES, in the order the
    /// exception publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile.
    pub fn presence(&self) -> [(&'static str, bool); 3] {
        let Self {
            patch,
            triangle,
            index,
        } = self;
        [
            ("patch", patch.is_some()),
            ("triangle", triangle.is_some()),
            ("index", index.is_some()),
        ]
    }

    /// The attributes this payload carries, publication order.
    pub fn present(&self) -> Vec<&'static str> {
        self.presence()
            .into_iter()
            .filter_map(|(name, set)| set.then_some(name))
            .collect()
    }

    /// The all-`None` record every other arm answers with.
    pub const NONE: Self = Self {
        patch: None,
        triangle: None,
        index: None,
    };
}

/// The three numbers one pick refusal names its site by.
///
/// Both matches are EXHAUSTIVE with no wildcard: an arm added to
/// either enum fails this build rather than reaching Python with an
/// all-`None` payload nobody chose for it.
pub fn index_payload(err: &NodePickError) -> IndexPayload {
    match err {
        NodePickError::Index(inner) => match inner {
            MeshPickError::PositionOutOfRange {
                patch,
                triangle,
                index,
            } => IndexPayload {
                patch: Some(*patch),
                triangle: Some(*triangle),
                index: Some(*index),
            },
        },
        NodePickError::Standing(_)
        | NodePickError::NotABody { .. }
        | NodePickError::NoSuchBody { .. }
        | NodePickError::Tessellate(_) => IndexPayload::NONE,
    }
}
