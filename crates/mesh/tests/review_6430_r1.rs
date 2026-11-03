//! Review 6430 r1 probe: a mesh refusal nesting an in-band props read
//! drops its at-rest lever at every door that prints the mesh refusal.
use geom_brep::props::{PropsCheck, PropsError};
use geom_brep::recourse::Reading;
use geom_core::Indeterminate;
use mesh::TessellateError;

#[test]
fn mesh_shape_refusal_keeps_the_nested_props_lever() {
    let source = PropsError::Escalated {
        cause: Indeterminate {
            margin: geom_core::MarginDiag::value(5e-9),
            band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("props_chart_branch"),
            terminal_sliver: false,
        },
        check: PropsCheck::Inventory,
    };
    let inner_at_rest = source.render(Reading::AtRest);
    let inner_payload = source.to_string();
    let outer = TessellateError::UnsupportedCurvedShape {
        face: topo::FaceKey::default(),
        source,
    }
    .to_string();
    eprintln!("INNER AT REST: {inner_at_rest}\nINNER PAYLOAD: {inner_payload}\nMESH TEXT: {outer}");
    assert!(outer.contains(&inner_at_rest), "the mesh text lost the nested lever: {outer}");
}
