//! Corpus document **symmetric_plate** — a mirror is a construction
//! (D10 `[repetition]`): an L-shaped plate, `2 × 2` with a `1 × 1`
//! notch off its `+x, +y` corner, mirrored across the plane of its own
//! `−x` face, and the plate and its image unioned into one symmetric
//! part, the face read twice gone into the interior.
//!
//! Dyadic mass pin: the union's footprint is the plate's and its
//! reflection's, area `6`, perimeter `12`, so at thickness `t` the
//! volume is `6t` and the area `2·6 + 12t`.
//!
//! D2 bump: the thickness `t` (mid-DAG — its cone is the extrude, the
//! plane read off its face, the mirror and the union; the profile is
//! reused).

use editor_core::{Dimension, DocEdit, Formula, FreeVar, Node, VarName};

use crate::fixture::desc;

use super::{CorpusDoc, MassPin, PlacePart, Recorder};

/// The plate's thickness, meters — the parameter `t`'s nominal.
const T: f64 = 0.5;

/// The document parameter the plate's thickness reads.
pub const THICKNESS: &str = "t";

/// The symmetric-plate corpus document.
pub fn document() -> CorpusDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(THICKNESS),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, T)),
    });
    let profile = r.insert(Node::Profile(desc(vec![vec![
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ]])));
    let plate = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: Formula::named(VarName::from_static(THICKNESS), Dimension::Length),
    });
    let plane = crate::fixture::plane_on(&r.doc, plate, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]);
    let image = r.insert(Node::mirror(plate, plane));
    let whole = r.insert(Node::Union {
        members: editor_core::Bodies::Spelled(vec![plate.into(), image.into()]),
    });
    r.place_part(whole);

    CorpusDoc {
        name: "symmetric_plate",
        about: "a mirror is a construction: an L plate and its mirror across its own face, unioned",
        edits: r.edits,
        doc: r.doc,
        result: Some(whole),
        pin: Some(MassPin {
            volume: 6.0 * T,
            area: Some(2.0 * 6.0 + 12.0 * T),
        }),
        bump: DocEdit::SetVarValue {
            var: VarName::from_static(THICKNESS).into(),
            value: editor_core::FreeValue::Continuous(0.25),
        },
        bump_root: plate,
    }
}
