//! **The sweep pair's correspondence** — the profile-operand verbs, in
//! the same per-instance-data pattern as the blends' and the boolean's.
//!
//! # What is shared, and what is not
//!
//! `Node::Extrude` and `Node::Revolve` share their OPERAND and their
//! shape downstream of the run: one validated profile in, one solid
//! out, a birth record whose emitter mints a full table, a
//! program-anchor rewrite over that table, and the same per-edge
//! parameter flow. Everything upstream of the run differs, and it
//! differs in DOCUMENT semantics rather than in literals: an extrude
//! reads one slot, while a revolve resolves an axis node, checks that
//! the axis and the profile are written against the same frame, and
//! classifies the authored angle as full or partial at its own funnel
//! site. None of that is a verb parameter, so none of it is declared
//! here — it stays in the lowering's per-node arms, exactly as the
//! boolean's `resolve_declarations` stayed there.
//!
//! What that means for the shape below: the correspondence is generic
//! in the ARGUMENT its `build` takes (`A`), because the two verbs'
//! payloads are genuinely different — a distance for one, an axis and
//! a classified revolution for the other. A single argument type would
//! have to be their union, which is a shape neither verb has.
//!
//! # Reading the record is the correspondence's, not the lowering's
//!
//! Each family's record is its door's whole bundle
//! (`verbs::VerbRecord::Extrude`, `::Revolve`), and each has exactly
//! one emitter. So the per-instance [`ProfileVerb::read`] is what pulls
//! the family's own record out of the closed channel, calls that
//! family's emitter, and hands back the three things the generic
//! lowering needs: the body, the table, and the wall faces the flow is
//! attached through. The match inside each reader is EXHAUSTIVE with no
//! wildcard arm (D3), so a record family added to the channel breaks
//! both readers at compile time.
//!
//! The alternative — one match in the lowering, routing family to
//! emitter — would put a vocabulary match back in the generic body,
//! which is the wart SEAT-4 removed from the blends' correspondence.
//! A future profile verb writes its own reader and never opens this
//! file's siblings.

use std::sync::Arc;

use geom_core::Decide;
use sweep::{Extruded, Revolution, RevolveAxis, Revolved};
use topo::{Body, FaceKey};
use verbs::{EdgeScalar, FlowSource, Verb, VerbRecord};

use crate::eval::NodeErrorKind;
use crate::names::{self, NameTable};
use crate::node::RecipeNodeId;

/// **What a sweep produced, read out of its record**: the body, the
/// names emitted from the birth record, and the wall faces per
/// CANONICAL profile loop.
///
/// The walls are here because they are what a per-edge flow source is
/// attached through — the wall swept from a profile edge is the entity
/// that stores that edge's radius — and because reading them out of a
/// family's own bundle is exactly what [`ProfileVerb::read`] is for.
/// They are grouped by loop and not flattened: which loop a wall came
/// from is what says which of the profile's radii it carries.
pub(crate) struct SweptOut<T: Decide> {
    /// The swept body, moved out of the record.
    pub(crate) body: Body<T>,
    /// The names, emitted before the record was taken apart.
    pub(crate) table: Arc<NameTable>,
    /// The wall faces, per canonical profile loop.
    pub(crate) walls: Vec<Vec<FaceKey>>,
}

/// A profile verb's record reader: this node's id and the record the
/// run door returned, in.
pub(crate) type RecordReader<T> = fn(RecipeNodeId, VerbRecord<T>) -> Result<SweptOut<T>, NodeErrorKind>;

/// **One profile-operand verb's correspondence**, as data — everything
/// the generic lowering needs once the node's own semantics have been
/// resolved to the verb's arguments.
///
/// `A` is the argument shape of this verb's payload (see the module
/// docs). Every field is direct per-instance data: a function pointer
/// per instance, no match over a verb vocabulary anywhere in this file.
pub(crate) struct ProfileVerb<T: Decide, A> {
    /// **Resolved arguments → the kernel verb.** The one place a
    /// document's evaluated distance, or its resolved axis and
    /// classified revolution, becomes a [`Verb`] payload.
    pub(crate) build: fn(A) -> Verb<T>,
    /// This verb's record reader — its family's own arm of the closed
    /// channel, its emitter, and its wall export.
    pub(crate) read: RecordReader<T>,
}

/// The extrude's kernel payload.
fn build_extrude<T: Decide>(distance: T) -> Verb<T> {
    Verb::Extrude { distance }
}

/// The revolve's kernel payload.
fn build_revolve<T: Decide>((axis, revolution): (RevolveAxis<T>, Revolution<T>)) -> Verb<T> {
    Verb::Revolve { axis, revolution }
}

/// What a WRONG-FAMILY record is called when an extrude's result
/// arrives carrying another family's channel — the blends'
/// `foreign_record` class exactly: a kernel bug surfaced typed,
/// unreachable while the doors and the correspondences agree.
const FOREIGN_EXTRUDE: &str = "the extrude returned a record that is not an extrude's";

/// The revolve's twin of [`FOREIGN_EXTRUDE`].
const FOREIGN_REVOLVE: &str = "the revolve returned a record that is not a revolve's";

/// A wrong-family refusal, in the naming vocabulary the blends' and the
/// boolean's use for the same class.
fn foreign(what: &'static str) -> NodeErrorKind {
    NodeErrorKind::Naming(names::NamingError::Emission { what })
}

/// The extrude's reader. Names FIRST — the emitter reads the whole
/// bundle — then takes the bundle apart, so nothing is cloned and the
/// body is moved rather than copied.
fn read_extrude<T: Decide>(
    id: RecipeNodeId,
    record: VerbRecord<T>,
) -> Result<SweptOut<T>, NodeErrorKind> {
    match record {
        VerbRecord::Extrude(built) => {
            let table = names::name_extrude(id, &built).map_err(NodeErrorKind::Naming)?;
            let Extruded {
                body, side_faces, ..
            } = built;
            Ok(SweptOut {
                body,
                table,
                walls: side_faces,
            })
        }
        VerbRecord::Blend(_) | VerbRecord::Boolean { .. } | VerbRecord::Revolve(_) => {
            Err(foreign(FOREIGN_EXTRUDE))
        }
    }
}

/// The revolve's reader. Its walls are per canonical segment and
/// OPTIONAL — an on-axis segment sweeps no wall at all — so the absent
/// ones are dropped rather than represented: a flow attaches to faces,
/// and a segment that minted none has none to attach to.
fn read_revolve<T: Decide>(
    id: RecipeNodeId,
    record: VerbRecord<T>,
) -> Result<SweptOut<T>, NodeErrorKind> {
    match record {
        VerbRecord::Revolve(built) => {
            let table = names::name_revolve(id, &built).map_err(NodeErrorKind::Naming)?;
            let Revolved { body, walls, .. } = built;
            let walls = walls
                .into_iter()
                .map(|loop_| loop_.into_iter().flatten().collect())
                .collect();
            Ok(SweptOut { body, table, walls })
        }
        VerbRecord::Blend(_) | VerbRecord::Boolean { .. } | VerbRecord::Extrude(_) => {
            Err(foreign(FOREIGN_REVOLVE))
        }
    }
}

/// The extrude's correspondence.
///
/// A function rather than a `const` for the reason the blends' and the
/// boolean's are: the struct is generic in the lane scalar and a
/// module-level const cannot be; the call is monomorphized and inlined.
pub(crate) fn extrude<T: Decide>() -> ProfileVerb<T, T> {
    ProfileVerb {
        build: build_extrude,
        read: read_extrude,
    }
}

/// The revolve's correspondence.
pub(crate) fn revolve<T: Decide>() -> ProfileVerb<T, (RevolveAxis<T>, Revolution<T>)> {
    ProfileVerb {
        build: build_revolve,
        read: read_revolve,
    }
}

/// **The per-edge flow source the sweeps declare**, named once here so
/// the lowering asks the declaration for it rather than spelling the
/// source kind inline.
///
/// It is not a per-verb field: both sweeps declare the same source, and
/// which FIELDS it reaches is the kernel-side declaration's answer
/// (`verbs::VerbKind::param_flow`), read at the attach.
pub(crate) const PROFILE_RADIUS: FlowSource = FlowSource::ProfileEdge(EdgeScalar::Radius);

#[cfg(test)]
mod tests {
    use geom_core::{Point2, Vec2};
    use verbs::VerbKind;

    use super::*;

    /// Each correspondence builds the verb its name claims — the check
    /// the blends' module makes for its pair.
    #[test]
    fn each_correspondence_builds_its_own_verb() {
        let e: Verb<f64> = (extrude::<f64>().build)(1.0);
        let r: Verb<f64> = (revolve::<f64>().build)((
            RevolveAxis {
                origin: Point2::new(0.0, 0.0),
                dir: Vec2::new(1.0, 0.0),
            },
            Revolution::Full,
        ));
        assert_eq!(e.kind(), VerbKind::Extrude, "the extrude built a {e:?}");
        assert_eq!(r.kind(), VerbKind::Revolve, "the revolve built a {r:?}");
    }

    /// **Each reader refuses another family's record, in its own
    /// words.** The case is unreachable while the run doors and the
    /// correspondences agree — which is exactly why it is exercised
    /// here rather than trusted: the refusal has to name the door it
    /// came from for the message to be worth anything, and a
    /// copy-paste that left the extrude's sentence on the revolve
    /// would otherwise never be seen.
    #[test]
    fn each_reader_refuses_a_foreign_record() {
        let id = RecipeNodeId(1);
        let Err(e) = read_extrude::<f64>(id, VerbRecord::Blend(None)) else {
            panic!("the extrude's reader accepted a blend's record");
        };
        let Err(r) = read_revolve::<f64>(id, VerbRecord::Blend(None)) else {
            panic!("the revolve's reader accepted a blend's record");
        };
        let (e, r) = (format!("{e}"), format!("{r}"));
        assert!(e.contains("extrude"), "the extrude's refusal reads {e}");
        assert!(r.contains("revolve"), "the revolve's refusal reads {r}");
        assert_ne!(e, r, "both sweeps share one wrong-family sentence");
    }

    /// The source the lowering attaches through is the one both verbs
    /// declare — a pin on the pair of them, so a declaration that
    /// dropped the row on one verb is not silently covered by the
    /// other.
    #[test]
    fn both_sweeps_declare_the_profile_radius_source() {
        for kind in [VerbKind::Extrude, VerbKind::Revolve] {
            let row = kind
                .param_flow()
                .iter()
                .find(|row| row.source == PROFILE_RADIUS)
                .unwrap_or_else(|| panic!("{kind:?} declares no profile-radius row"));
            assert!(
                !row.fields.is_empty(),
                "{kind:?}'s profile-radius row reaches no field"
            );
        }
    }
}
