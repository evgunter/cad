//! Review 6430 r1 probes: a payload `Display` that still carries an
//! ending fixed at a build, through a helper the gate cannot see.
use geom_brep::recourse::{Ended, Reading};
use geom_core::Indeterminate;

fn in_band() -> Indeterminate {
    Indeterminate {
        margin: geom_core::MarginDiag::value(5e-9),
        band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
        predicate: Some("probe"),
        terminal_sliver: false,
    }
}

fn show(what: &str, payload: String, build: String, rest: String) {
    eprintln!("== {what}\nDISPLAY: {payload:?}\nBUILD:   {build:?}\nATREST:  {rest:?}");
}

#[test]
fn euler_split_param_display_is_payload_only() {
    let e = topo::EulerOpError::SplitParamEscalated {
        edge: topo::EdgeKey::default(),
        diag: in_band(),
    };
    let (p, b, r) = (e.to_string(), e.ended(Reading::Build.into()), e.ended(Reading::AtRest.into()));
    show("EulerOpError::SplitParamEscalated", p.clone(), b, r);
    assert!(!p.contains("Recourse"), "Display carries an ending: {p}");
}

#[test]
fn split_reduce_crossing_display_is_payload_only() {
    let e = topo::SplitReduceError::CrossingEscalated {
        edge: topo::EdgeKey::default(),
        fault: topo::ConicRootFault::CrossingInterior(in_band()),
    };
    let (p, b, r) = (e.to_string(), e.ended(Reading::Build.into()), e.ended(Reading::AtRest.into()));
    show("SplitReduceError::CrossingEscalated", p.clone(), b, r);
    assert!(!p.contains("Recourse"), "Display carries an ending: {p}");
}

#[test]
fn split_param_poisoned_display_names_a_door() {
    let mut d = in_band();
    d.margin = geom_core::MarginDiag::INVALID;
    let e = topo::EulerOpError::SplitParamEscalated { edge: topo::EdgeKey::default(), diag: d };
    let (p, b, r) = (e.to_string(), e.ended(Reading::Build.into()), e.ended(Reading::AtRest.into()));
    show("EulerOpError::SplitParamEscalated poisoned", p.clone(), b.clone(), r.clone());
    assert!(b != r, "build and at-rest endings coincide");
    assert!(p != b, "the payload Display is the build's text: {p}");
}
