//! **The roster: every predicate name this crate decides is a blend
//! decision, or listed here as another door's and why.**
//!
//! `BlendError::Escalated` carries the closed `BlendDecision` it could
//! not take, and its subject and ending are exhaustive matches over
//! that type (D4 ¶1 (i)): a blend gate cannot escalate without naming
//! the decision whose levers its reader is handed, so there is no
//! fall-through to reach. What stays name-shaped is the `k_stats` name
//! each decision is metered under, and the rows here hold that the
//! names the crate decides are exactly the blend's closed set plus the
//! other doors' gates.
//!
//! The crate decides for five doors, and only the blend's own gates
//! escalate onto `BlendError`; the rest reach the caller through their
//! own door's error type. They are listed here, and the row that reads
//! them is the one that would notice a blend name decided outside the
//! blend's funnel.
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk. **What it cannot read it reports** — an
//! unreadable spelling, an indirect site whose carrier is undeclared, a
//! file it does not walk — and each is a red row here.
//!
//! Which lever a decision renders is MEASURED — each decision is put
//! through the door's own error value and the rendered text read —
//! never inferred from the match in the source.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag, MissingRecourse, Tol};
use std::collections::BTreeSet;
use sweep::blend::{BlendDecision, BlendError, BlendSite};
use test_utils::source::{NameCarrier, PredicateCensus, predicate_census};

/// The carriers that hand a name to the funnel from somewhere other
/// than the call.
const CARRIERS: &[NameCarrier] = &[
    // The cosurface decision reports under the row names its caller's
    // table supplies, so the extrude's side walls and the revolve's
    // walls are separate rows of the K inventory.
    NameCarrier::Call("CosurfaceNames"),
];

/// The `decide*` calls whose name the reader cannot read at the site,
/// as `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    (
        "blend/mod.rs: name",
        "the blend's one funnel, naming the closed `BlendDecision` it takes — its names are \
         `BlendDecision::ALL`'s, read by `the_decided_names_are_the_blend_decisions_and_the_listed_doors`",
    ),
    (
        "swept.rs: name",
        "the swept-traversal funnel, forwarding its parameter",
    ),
    (
        "swept.rs: names.lines",
        "a `CosurfaceNames` field — the line half of a caller's table",
    ),
    (
        "swept.rs: names.arcs",
        "a `CosurfaceNames` field — the arc half of the same table",
    ),
];

/// Every name decided outside the blend, and the door that reports it.
///
/// For all of them the reason is the same shape — the gate belongs to
/// another of this crate's doors, whose refusals are typed on that
/// door's own error and rendered there, so `BlendError` is not on the
/// escalation's path.
const OTHER_DOORS: &[(&str, &str)] = &[
    ("axis_arc_apex", AXIS),
    ("axis_arc_center", AXIS),
    ("axis_arc_clearance", AXIS),
    ("axis_arc_span", AXIS),
    ("axis_line_axial", AXIS),
    ("axis_line_radial", AXIS),
    ("axis_vertex_radius", AXIS),
    ("revolve_axis_direction", AXIS),
    ("revolve_angle", REVOLVE),
    ("revolve_angle_headroom", REVOLVE),
    ("extrusion_normal_component", EXTRUDE),
    ("extrusion_obliquity", EXTRUDE),
    ("loft_stacking", LOFT),
    ("tube_wall", TUBE),
    ("tube_wall_bore", TUBE),
    ("tube_wall_gap", TUBE),
    ("tube_window_headroom", TUBE),
    ("tube_window_span", TUBE),
    ("side_cylinders_cosurface", COSURFACE),
    ("side_planes_cosurface", COSURFACE),
    ("wall_arcs_cosurface", COSURFACE),
    ("wall_lines_cosurface", COSURFACE),
];

const AXIS: &str = "the revolve axis's own classifications: an escalation here is typed on \
                    `RevolveError` and rendered by that door";
const REVOLVE: &str = "the revolve window's classifications, typed on `RevolveError`";
const EXTRUDE: &str = "the extrusion direction's classifications, typed on `ExtrudeError`";
const LOFT: &str = "the loft's section stacking, typed on `LoftError`";
const TUBE: &str = "the tube door's window and wall classifications, typed on `TubeError` — \
                    whose Display names which of the two tube doors a wall escalation came \
                    from";
const COSURFACE: &str = "the swept traversal's cosurface decision, one row name per calling \
                         verb; the escalation is typed on that verb's error";

/// Each decision with the lever its refusals lead with.
///
/// Held here as an independent statement of what the door SHOULD say,
/// so a decision rewired to another lever reds
/// [`every_decision_renders_its_own_lever_and_no_other`]. Its
/// completeness is checked against `BlendDecision::ALL`.
const PAIRING: &[(BlendDecision, &str)] = &[
    (
        BlendDecision::RadiusHeadroom,
        sweep::blend::FILLET3_RADIUS_RECOURSE,
    ),
    (
        BlendDecision::FaceClearance,
        sweep::blend::FILLET3_CLEARANCE_RECOURSE,
    ),
    (
        BlendDecision::SpineRegularity,
        sweep::blend::FILLET3_SPINE_RECOURSE,
    ),
    (BlendDecision::ChainG1, sweep::blend::FILLET3_CHAIN_RECOURSE),
    (
        BlendDecision::ChainArm,
        sweep::blend::FILLET3_CHAIN_RECOURSE,
    ),
    (
        BlendDecision::ConvexitySign,
        sweep::blend::FILLET3_TANGENTIAL_RECOURSE,
    ),
    (
        BlendDecision::RingClearance,
        sweep::blend::FILLET3_RING_RECOURSE,
    ),
    (
        BlendDecision::SupportCoaxiality,
        sweep::blend::FILLET3_SPINE_KIND_RECOURSE,
    ),
    (
        BlendDecision::ContactSecondOrder,
        sweep::blend::FILLET3_CONTACT_RECOURSE,
    ),
    (
        BlendDecision::CornerIndependence,
        sweep::blend::FILLET3_CORNER_INDEPENDENCE_RECOURSE,
    ),
    (
        BlendDecision::CapTransverse,
        sweep::blend::FILLET3_CORNER_RECOURSE,
    ),
];

fn census() -> PredicateCensus {
    predicate_census(
        &test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src"),
        CARRIERS,
    )
}

/// An escalation carrying `name`, its margin inside the run's band.
fn escalation(name: Option<&'static str>) -> Indeterminate {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    Indeterminate {
        margin: MarginDiag::value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: name,
        terminal_sliver: false,
    }
}

/// Renders the blend door's refusal for `decision` escalating with a
/// payload that names `name`.
fn rendered(decision: BlendDecision, name: Option<&'static str>) -> String {
    BlendError::Escalated {
        site: BlendSite::Chain,
        decision,
        source: escalation(name),
    }
    .to_string()
}

/// **The names the crate decides are the blend's closed set and the
/// listed doors' gates, and nothing else.**
///
/// The blend's names reach the funnel only through
/// `BlendDecision::predicate`, so none may appear at a decide site as a
/// literal: one that did would be a blend gate metered outside the
/// closed type. Every other name is listed with its door, and a listed
/// name nothing decides is a stale entry. The blend's names are
/// distinct, since two decisions under one name would fuse two K rows.
#[test]
fn the_decided_names_are_the_blend_decisions_and_the_listed_doors() {
    let census = census();
    assert!(
        census.names.contains("tube_wall"),
        "the reader found no funnel calls it should have: {:?}",
        census.names
    );
    let blend: BTreeSet<&str> = BlendDecision::ALL.iter().map(|d| d.predicate()).collect();
    assert_eq!(
        blend.len(),
        BlendDecision::ALL.len(),
        "two blend decisions share a k_stats name"
    );
    let listed: BTreeSet<&str> = OTHER_DOORS.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        listed.len(),
        OTHER_DOORS.len(),
        "a name is listed twice in OTHER_DOORS"
    );
    for (name, _) in OTHER_DOORS {
        assert!(
            census.names.contains(*name),
            "`{name}` is listed as another door's, but nothing in the crate's src decides it \
             — a stale entry"
        );
        assert!(
            !blend.contains(name),
            "`{name}` is listed as another door's and is a blend decision's name"
        );
    }
    for name in &census.names {
        assert!(
            !blend.contains(name.as_str()),
            "`{name}` is a blend decision's name decided at a literal site, outside the \
             blend's funnel"
        );
        assert!(
            listed.contains(name.as_str()),
            "`{name}` is decided in the crate's src and is neither a blend decision nor \
             listed with its door"
        );
    }
}

/// **Each decision renders the lever its own refusals lead with, and no
/// other decision's.**
///
/// Pinned against a table written independently of the match, and held
/// complete against `BlendDecision::ALL`.
#[test]
fn every_decision_renders_its_own_lever_and_no_other() {
    let paired: Vec<BlendDecision> = PAIRING.iter().map(|(d, _)| *d).collect();
    assert_eq!(
        paired,
        BlendDecision::ALL,
        "the pairing table and the closed set disagree"
    );
    for (decision, lever) in PAIRING {
        let text = rendered(*decision, Some(decision.predicate()));
        assert!(
            text.contains(lever),
            "{decision:?} should carry its own lever; it renders: {text}"
        );
        for (other_decision, other) in PAIRING {
            assert!(
                other == lever || !text.contains(other),
                "{decision:?} renders the lever {other_decision:?} owns: {text}"
            );
        }
    }
}

/// **The payload's name is data, never routing.**
///
/// An escalation reads by the decision it carries whatever name its
/// payload holds — another decision's, one the crate never decides, or
/// none — and never renders the gap sentence, which no blend refusal
/// can reach.
#[test]
fn the_payloads_name_does_not_route_the_refusal() {
    let decision = BlendDecision::RadiusHeadroom;
    let own = rendered(decision, Some(decision.predicate()));
    for name in [
        Some(BlendDecision::ChainG1.predicate()),
        Some("roster_unknown_probe"),
        None,
    ] {
        let text = rendered(decision, name);
        assert_eq!(text, own, "the payload's name {name:?} changed the refusal");
        assert!(!text.contains(&MissingRecourse(name).to_string()), "{text}");
    }
}

/// Every in-band reading either lane reports, on both sides of zero,
/// with whether it is negative.
fn in_band_readings() -> Vec<(MarginDiag, bool)> {
    let b = Band::linear(Tol::witness()).expect("the run's band forms");
    let mid = (b.zero() + b.escalate()) / 2.0;
    vec![
        (MarginDiag::value(mid), false),
        (MarginDiag::value(-mid), true),
        (MarginDiag::enclosure(mid * 0.9, mid), false),
        (MarginDiag::enclosure(-mid, -mid * 0.9), true),
    ]
}

/// The decisions no smaller tolerance can truthfully be offered for:
/// the three that pass only at zero (a refused margin is a miss, D4 ¶1
/// (i)), and the must-carry relay, whose in-band verdict may be a
/// first-order wedge reading that a smaller tolerance refuses.
const NO_TOLERANCE: &[BlendDecision] = &[
    BlendDecision::ChainG1,
    BlendDecision::SupportCoaxiality,
    BlendDecision::CapTransverse,
    BlendDecision::ContactSecondOrder,
];

/// The decisions that pass on a negative sign as well as a positive one.
const TWO_SIDED: &[BlendDecision] = &[BlendDecision::ConvexitySign];

/// **No tolerance is offered where none decides the margin passing**,
/// on any in-band reading — including the relay's wedge reading, which
/// its payload names and its decision cannot see.
#[test]
fn a_decision_no_tolerance_decides_passing_is_offered_none() {
    for decision in NO_TOLERANCE {
        for (m, _) in in_band_readings() {
            for name in [decision.predicate(), "dihedral_wedge"] {
                let text = BlendError::Escalated {
                    site: BlendSite::Chain,
                    decision: *decision,
                    source: Indeterminate {
                        margin: m,
                        ..escalation(Some(name))
                    },
                }
                .to_string();
                assert!(!text.contains("tighten"), "{decision:?} {m}: {text}");
                assert!(text.contains(decision.lever()), "{decision:?} {m}: {text}");
            }
        }
    }
}

/// **Each sized decision's pass set, read on both sides of zero**: an
/// in-band reading is offered the tolerance exactly where the decision
/// passes on that reading's sign, and never where it passes only at
/// zero.
#[test]
fn an_in_band_reading_is_offered_the_tolerance_exactly_where_its_sign_passes() {
    for decision in BlendDecision::ALL {
        for (m, negative) in in_band_readings() {
            let text = BlendError::Escalated {
                site: BlendSite::Chain,
                decision,
                source: Indeterminate {
                    margin: m,
                    ..escalation(Some(decision.predicate()))
                },
            }
            .to_string();
            let want =
                !NO_TOLERANCE.contains(&decision) && (!negative || TWO_SIDED.contains(&decision));
            assert_eq!(
                text.contains("tighten the tolerance below "),
                want,
                "{decision:?} {m}: {text}"
            );
        }
    }
}

/// **`fillet3_chain_arm` tells one story on its band-decided arms**
/// (D4 ¶1 (iv)): a junction arm decided zero ends as an in-band one
/// does — the chain lever and the tolerance its own reading gives — and
/// neither reads as an unreadable margin. The ending is pinned whole,
/// and its offer followed: below the offered tolerance the same
/// tangent junction passes.
#[test]
fn a_decided_zero_chain_arm_ends_as_its_in_band_sibling_does() {
    use sweep::blend::battery::chain_g1;
    let b = Band::linear(Tol::witness()).expect("the run's band forms");
    let k = b.escalate() / b.zero();
    let v = topo::VertexKey::default();
    let x = geom_core::Vec3::new(1.0, 0.0, 0.0);
    let mid = (b.zero() + b.escalate()) / 2.0;
    let short = b.zero() / 2.0;
    for (arm, what) in [(mid, "in band"), (short, "decided zero")] {
        let err = chain_g1(x, x, arm, v, b).unwrap_err();
        assert!(
            matches!(
                err,
                BlendError::Escalated {
                    decision: BlendDecision::ChainArm,
                    ..
                }
            ),
            "{what}: {err:?}"
        );
        let text = err.to_string();
        let offered = arm / k;
        assert!(
            text.ends_with(&format!(
                "Recourse: {}, or, if this link length is intended, tighten the tolerance \
                 below {offered:e} m",
                sweep::blend::FILLET3_CHAIN_RECOURSE,
            )),
            "{what}: {text}"
        );
        assert!(
            !text.contains(geom_core::UNREADABLE_MARGIN_NOTE),
            "{what}: a short link is the caller's geometry: {text}"
        );
        let tighter = Band::new(offered / 2.0, offered / 2.0 * k).expect("a tighter band");
        chain_g1(x, x, arm, v, tighter)
            .unwrap_or_else(|e| panic!("{what}: below the offer the junction passes: {e}"));
    }
}

/// **The convexity sign's tolerance offer names its margin as a length**:
/// the dihedral's sine levered by the arm, "wedge opening", in metres.
#[test]
fn the_convexity_sign_offer_names_the_wedge_opening() {
    let b = Band::linear(Tol::witness()).expect("the run's band forms");
    let mid = (b.zero() + b.escalate()) / 2.0;
    let text = BlendError::Escalated {
        site: BlendSite::Chain,
        decision: BlendDecision::ConvexitySign,
        source: escalation(Some(BlendDecision::ConvexitySign.predicate())),
    }
    .to_string();
    assert!(
        text.ends_with(&format!(
            "Recourse: {}, or, if this wedge opening is intended, tighten the tolerance below \
             {:e} m",
            sweep::blend::FILLET3_TANGENTIAL_RECOURSE,
            mid / (b.escalate() / b.zero())
        )),
        "{text}"
    );
}

/// **Nothing in the crate's `src` is invisible to the reader.**
///
/// A name built somewhere other than the call — a const, a struct
/// field, a `concat!` — is exactly the name a census misses, and two
/// hand-rolled readers were defeated that way before the shared one
/// existed. So the reader reports rather than skips, and each kind of
/// report is a red row here: an indirect site whose carrier nobody has
/// declared, a spelling it cannot parse, and a file it does not walk.
#[test]
fn nothing_in_src_is_invisible_to_the_reader() {
    let census = census();
    let declared: BTreeSet<String> = INDIRECT.iter().map(|(site, _)| site.to_string()).collect();
    assert_eq!(
        census.indirect, declared,
        "the funnel calls whose name the reader cannot read at the site are not the ones \
         declared in INDIRECT"
    );
    assert!(
        census.unreadable.is_empty(),
        "a `decide*` call is spelled in a way the reader cannot parse, so the gate it names \
         is outside the roster: {:?}",
        census.unreadable
    );
    assert!(
        census.unwalked.is_empty(),
        "an `include!` or `#[path]` pulls source into this crate from a file the reader does \
         not walk, so a gate there is outside the roster: {:?}",
        census.unwalked
    );
}

/// **`split_edge`'s in-band interiority reads whole through the blend's
/// operator door**, on a constructed value (no blend fixture reaches
/// the band): the decision as its subject, one recourse the blend can
/// take — the geometry, and the tolerance the positive margin gives —
/// and no declaration, which a blend does not take. The split's and
/// the Boolean's doors have their rows in `topo`
/// (`boolean::refusal_routes`).
#[test]
fn the_split_param_escalation_reads_whole_through_the_blend_door() {
    use test_utils::refusal::{recourse_markers, stage_prefixes, subjectless_escalations};
    let text = BlendError::Op {
        site: "meridian split",
        source: topo::EulerOpError::SplitParamEscalated {
            edge: topo::EdgeKey::default(),
            diag: escalation(Some("split_edge_param_interior")),
        },
    }
    .to_string();
    assert_eq!(recourse_markers(&text), 1, "{text}");
    assert!(subjectless_escalations(&text).is_empty(), "{text}");
    assert!(stage_prefixes(&text, &[]).is_empty(), "{text}");
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    let below = (band.zero() + band.escalate()) / 2.0 / (band.escalate() / band.zero());
    assert!(
        text.contains("whether a crossing lands strictly inside its edge is undecided: margin ")
            && text.ends_with(&format!(
                "Recourse: move the geometry so the crossing lands clearly away from the edge's \
                 ends, or, if this distance from the edge's end is intended, tighten the \
                 tolerance below {below:e} m",
            ))
            && !text.contains("declare"),
        "{text}"
    );
}
