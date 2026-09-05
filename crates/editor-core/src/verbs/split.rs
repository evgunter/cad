//! **The split's correspondence** — the one-body-in, two-sides-out
//! verb, in the same per-instance-data pattern as the blends', the
//! boolean's and the sweeps'.
//!
//! # What it declares, and what stays upstairs
//!
//! `Node::Split` has two inputs and no slot: a `target` whose evaluated
//! body AND name table the lowering reads, and a `tool` whose evaluated
//! value must be a datum PLANE. The plane is the verb's whole payload —
//! a kernel value read off a datum node, exactly as the boolean's
//! declarations reach the kernel already resolved — and the body is the
//! one operand, borrowed at the split door.
//!
//! What stays in the lowering, deliberately: which input is the body
//! and which the tool (the node's own shape), and the refusal a tool
//! that is not a plane draws (`NodeErrorKind::WrongOperand` — document
//! semantics, the class `resolve_declarations` and the revolve's axis
//! check keep upstairs). What is declared HERE is the correspondence
//! proper: how a datum value becomes the verb's plane ([`SplitVerb::tool`]
//! and the label its refusal carries), how the plane becomes the kernel
//! verb (`build`), and which emitter mints names from the birth record
//! and the two sides (`emitter`). There is no slot↔parameter row because
//! the split has no scalar slot — the same fact its empty `param_flow`
//! declares kernel-side — and no flow is attached.
//!
//! # Every field is direct per-instance data
//!
//! As in the sibling modules: function pointers and literals per
//! instance, no match over a verb vocabulary anywhere in this file, so
//! a future verb never has to open it. The one match here is over the
//! DATUM vocabulary — which kind of datum is a split's tool — and it is
//! exhaustive with no wildcard arm (D3), so a datum kind added upstairs
//! is a visit here that says whether a split can take it.
//!
//! # Where an operand's expected-kind label lives, by convention
//!
//! Three labels of this shape exist and they are three different
//! things, which is why they have three homes. `tool_expected` is
//! correspondence DATA because the reading it labels is the
//! correspondence's (`tool`): whoever declares which datum kinds a
//! verb accepts declares what to call the refusal. The profile
//! lowering's inline `expected: "profile"` is the LOWERING's, because
//! every profile verb takes a profile — the operand contract belongs
//! to `wire_swept`, not to any one correspondence. The blends'
//! `selection_label` is not an operand label at all: it is the kernel
//! refusal label a SELECTION failure carries (`BlendKind`), shared
//! with the kernel's own refusal so a verb is never rendered twice.
//! The rule: a label is data on the correspondence that owns the
//! reading it names, and inline in the lowering that owns the
//! contract it names.

use std::sync::Arc;

use geom_core::{Decide, Tol, Vec3};
use topo::query::DatumValue;
use topo::splitting::SplitNaming;
use topo::{Body, SplitPlane};
use verbs::{Verb, VerbRecord};

use crate::names::{self, NameTable, NamingError};
use crate::node::RecipeNodeId;

/// The split's naming emitter: this node's id, the two sides (each
/// present or empty), the birth record, the target's id, table and
/// body, the tool plane's normal, and the tolerance, in.
pub(crate) type SplitEmitter<T> = fn(
    RecipeNodeId,
    Option<&Body<T>>,
    Option<&Body<T>>,
    &SplitNaming,
    RecipeNodeId,
    &NameTable,
    &Body<T>,
    Vec3<T>,
    Tol,
) -> Result<Arc<NameTable>, NamingError>;

/// **The split's correspondence**, as data — everything the split's
/// lowering needs to turn a `Node::Split` into a [`Verb`] and its two
/// sides into a name table. Adding a field here is how the verb
/// declares something the lowering must know.
pub(crate) struct SplitVerb<T: Decide> {
    /// **The tool datum → the verb's plane.** The reading of a datum
    /// value as a parting plane, per instance: a plane datum is one, an
    /// axis or a point is not. `None` is the document-layer refusal's
    /// cue, never a default plane.
    pub(crate) tool: fn(&DatumValue<T>) -> Option<SplitPlane<T>>,
    /// What the tool operand is EXPECTED to be, in the words the
    /// `WrongOperand` refusal carries when it is not.
    pub(crate) tool_expected: &'static str,
    /// **The plane → the kernel verb.** The one place a document's
    /// resolved tool plane becomes a [`Verb`] payload, per instance.
    pub(crate) build: fn(SplitPlane<T>) -> Verb<T>,
    /// This verb's naming emitter.
    pub(crate) emitter: SplitEmitter<T>,
    /// **This family's arm of the closed record channel**, as a
    /// projection — read through [`super::read_record`], which owns
    /// the foreign-family refusal.
    pub(crate) record: fn(VerbRecord<T>) -> Option<SplitNaming>,
    /// What a WRONG-FAMILY record is called when this verb's result
    /// arrives carrying another family's channel — the blends', the
    /// boolean's and the sweeps' `foreign_record` class exactly: a
    /// kernel bug surfaced typed, unreachable while the doors and the
    /// correspondences agree, with the sentence naming the door so
    /// the refusal does too.
    pub(crate) foreign_record: &'static str,
}

/// The split's kernel payload. A named function rather than a closure
/// so it can be a plain `fn` pointer in the struct above.
fn build_split<T: Decide>(plane: SplitPlane<T>) -> Verb<T> {
    Verb::Split { plane }
}

/// The split family's arm of the record channel. Exhaustive with no
/// wildcard (D3): a family added to the channel breaks this at compile
/// time and is routed here deliberately.
fn split_record<T: Decide>(record: VerbRecord<T>) -> Option<SplitNaming> {
    match record {
        VerbRecord::Split(naming) => Some(naming),
        VerbRecord::Blend(_)
        | VerbRecord::Boolean { .. }
        | VerbRecord::Extrude(_)
        | VerbRecord::Revolve(_) => None,
    }
}

/// A plane datum is a parting plane; no other datum kind is. A FRAME
/// carries a plane's worth of placement and is still refused: a frame
/// is a sketch's coordinate system, and reading one as a parting
/// plane would let a split be authored against a profile's frame
/// silently rather than against a plane the document names. The
/// datum's normal is unit by construction (`UnitVec3`), which is the
/// convention the kernel's plane states and does not check.
fn plane_of<T: Decide>(datum: &DatumValue<T>) -> Option<SplitPlane<T>> {
    match datum {
        DatumValue::Plane { origin, normal } => Some(SplitPlane {
            origin: *origin,
            normal: normal.get(),
        }),
        DatumValue::Axis { .. }
        | DatumValue::Point { .. }
        | DatumValue::Frame { .. }
        | DatumValue::AxisInPlane { .. } => None,
    }
}

/// The split's correspondence.
///
/// A function rather than a `const` for the same reason the siblings'
/// are: the struct is generic in the lane scalar and a module-level
/// const cannot be; the call is monomorphized and inlined.
pub(crate) fn split<T: Decide>() -> SplitVerb<T> {
    SplitVerb {
        tool: plane_of,
        tool_expected: "datum plane",
        build: build_split,
        emitter: names::name_split,
        record: split_record,
        foreign_record: "the split returned a record that is not a split's",
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)] // a fixture that will not build is a failure, not a value
mod tests {
    use geom_core::{Band, Point3};
    use topo::query::UnitVec3;
    use verbs::VerbKind;

    use super::*;

    /// The correspondence builds the verb its name claims — the
    /// structural check every sibling module makes.
    #[test]
    fn the_correspondence_builds_the_split() {
        let plane = SplitPlane {
            origin: Point3::new(0.0, 0.0, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };
        let v: Verb<f64> = (split::<f64>().build)(plane);
        assert_eq!(
            v.kind(),
            VerbKind::Split,
            "the split correspondence built a {v:?}"
        );
    }

    /// **The tool reading takes a plane and nothing else**, and the
    /// label its refusal carries says so. An axis and a point are two
    /// of the other datum kinds a document can hand a split (a frame
    /// and an in-plane axis are the rest, refused by the same
    /// exhaustive arm), and each is refused by the reading rather
    /// than read as some plane.
    #[test]
    fn the_tool_reading_takes_a_plane_only() {
        let corr = split::<f64>();
        let band = Band::linear(Tol::witness()).expect("the witnessed band");
        let up = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), band).expect("a unit direction");
        let origin = Point3::new(0.0, 0.0, 0.5);
        let plane = (corr.tool)(&DatumValue::Plane { origin, normal: up })
            .expect("a plane datum is a parting plane");
        assert_eq!(plane.normal.z, 1.0);
        assert_eq!(plane.origin.z, 0.5);
        assert!(
            (corr.tool)(&DatumValue::Axis { origin, dir: up }).is_none(),
            "an axis was read as a parting plane"
        );
        assert!(
            (corr.tool)(&DatumValue::Point { position: origin }).is_none(),
            "a point was read as a parting plane"
        );
        // Byte-exact, not `contains`: this label is DOCUMENT-REACHABLE
        // — a split whose tool is an axis datum refuses `WrongOperand
        // { expected }` with it — so a word moved here is a refusal
        // string moved at the document layer.
        assert_eq!(corr.tool_expected, "datum plane");
    }

    /// The wrong-family sentence, byte-exact. It names its door (the
    /// boolean's module pins the same property for its one instance
    /// by `contains`; this one is unreachable from a document but is
    /// pinned to the byte anyway, so the two split sentences are held
    /// to one standard).
    #[test]
    fn the_foreign_record_sentence_names_the_door() {
        assert_eq!(
            split::<f64>().foreign_record,
            "the split returned a record that is not a split's"
        );
    }
}
