//! **The roster: every predicate name this crate decides is routed to a
//! recourse sentence, or listed here as carrying none and why.**
//!
//! `BlendError::Escalated`'s `Display` routes a recourse by matching the
//! escalated predicate's NAME. A name the match does not know falls
//! through to `geom_core::MissingRecourse` — the sentence that names the
//! hole — and the fall-through is what these rows are about: it must be
//! reached on purpose, by a name somebody decided owes no blend recourse
//! and wrote down, never by a gate that quietly arrived.
//!
//! The crate decides for five doors, and only the blend's own gates
//! escalate onto `BlendError`; the rest reach the caller through their
//! own door's error type. That is the reason most of [`UNROUTED`]
//! carries, and it is the reason a `fillet3_*` gate added to the battery
//! cannot borrow: a new blend gate is unrouted, unlisted, and red.
//!
//! **Where the list lives differs from `profile`'s, and the reason is
//! the door.** `PathError::Escalated` really does receive every name
//! that crate decides, so its shared-clause-only list is `src`-side and
//! the Display consults it. Nothing here reaches `BlendError` but a
//! blend gate: the other names are decided BY OTHER DOORS of this crate
//! and escalate on their own error types, so listing them in `src`
//! would put a claim about the call graph into the blend door's prose.
//! They are listed here instead, and the row that reads them is the one
//! that would notice a blend gate arriving among them.
//!
//! The reader is `test_utils::source::predicate_census`, the tree's one
//! home for this walk. **What it cannot read it reports** — an
//! unreadable spelling, an indirect site whose carrier is undeclared, a
//! file it does not walk — and each is a red row here.
//!
//! Which arm answers a name is MEASURED — each name is put through the
//! door's own error value and the rendered text read — never inferred
//! from the order of the arms in the source.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag, MissingRecourse, Tol};
use std::collections::BTreeSet;
use sweep::blend::{BlendError, BlendSite};
use test_utils::source::{NameCarrier, PredicateCensus, predicate_census};

/// The carriers that hand a name to the funnel from somewhere other
/// than the call.
const CARRIERS: &[NameCarrier] = &[
    // The cosurface decision reports under the row names its caller's
    // table supplies, so the extrude's side walls and the revolve's
    // walls are separate rows of the K inventory.
    NameCarrier::Call("CosurfaceNames"),
    // The ring-clearance gate names itself once, as a const, because
    // the surgery both decides it and quotes it back in the payload.
    NameCarrier::Const("RING_CLEARANCE"),
];

/// The `decide*` calls whose name the reader cannot read at the site,
/// as `<file>: <expr>`, with what carries the name there.
const INDIRECT: &[(&str, &str)] = &[
    (
        "blend/mod.rs: name",
        "the blend module's own funnel, forwarding its parameter",
    ),
    (
        "blend/surgery.rs: RING_CLEARANCE",
        "the const the ring-clearance gate names itself by — declared as a carrier, so the \
         name it holds is on the roster",
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

/// Every decided name with no blend recourse, and why it needs none.
///
/// A name here renders the gap sentence at `BlendError::Escalated`: the
/// refusal says the blend table records nothing for it. For all of them
/// the reason is the same shape — the gate belongs to another of this
/// crate's doors, whose refusals are typed on that door's own error and
/// rendered there, so `BlendError` is not on the escalation's path.
const UNROUTED: &[(&str, &str)] = &[
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

/// Each routed name with the recourse constant its own arm owns.
///
/// Held here as an independent statement of what the door SHOULD say,
/// so a name rewired to another arm's constant reds
/// [`every_routed_name_renders_the_recourse_its_own_arm_owns`] even
/// though it stays "routed" for the roster row. The table's
/// completeness is checked against the crate's own decided names, so a
/// new blend gate cannot be routed without joining it.
const PAIRING: &[(&str, &str)] = &[
    (
        "fillet3_radius_headroom",
        sweep::blend::FILLET3_RADIUS_RECOURSE,
    ),
    (
        "fillet3_face_clearance",
        sweep::blend::FILLET3_CLEARANCE_RECOURSE,
    ),
    (
        "fillet3_spine_regularity",
        sweep::blend::FILLET3_SPINE_RECOURSE,
    ),
    ("fillet3_chain_g1", sweep::blend::FILLET3_CHAIN_RECOURSE),
    ("fillet3_chain_arm", sweep::blend::FILLET3_CHAIN_RECOURSE),
    (
        "fillet3_convexity_sign",
        sweep::blend::FILLET3_TANGENTIAL_RECOURSE,
    ),
    (
        "fillet3_ring_clearance",
        sweep::blend::FILLET3_RING_RECOURSE,
    ),
    (
        "fillet3_support_coaxiality",
        sweep::blend::FILLET3_SPINE_KIND_RECOURSE,
    ),
    (
        "fillet3_corner_independence",
        sweep::blend::FILLET3_CORNER_RECOURSE,
    ),
    (
        "fillet3_cap_transverse",
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
fn escalation(name: &'static str) -> Indeterminate {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    Indeterminate {
        margin: MarginDiag::Value((band.zero() + band.escalate()) / 2.0),
        band,
        predicate: Some(name),
    }
}

/// Renders the blend door's refusal for an escalation under `name`.
fn rendered(name: &'static str) -> String {
    BlendError::Escalated {
        site: BlendSite::Chain,
        source: escalation(name),
    }
    .to_string()
}

/// Whether the refusal for `name` is the fall-through — measured by
/// reading the text, not by reading the match.
fn falls_through(name: &'static str) -> bool {
    rendered(name).contains(&MissingRecourse(Some(name)).to_string())
}

/// **Every name the crate decides is routed to a sentence, or listed
/// with the reason it carries none.**
///
/// The two sides are read from different places and compared: the names
/// out of the crate's `src`, the routing out of the rendered refusal.
/// A battery gate added without a sentence is unrouted and unlisted, so
/// it reds here; a gate renamed under a routed arm stops being routed,
/// so it reds here too; and a name struck off `UNROUTED` without a
/// sentence reds as a stale entry.
#[test]
fn every_decided_name_is_routed_or_listed_with_its_reason() {
    let census = census();
    assert!(
        census.names.contains("fillet3_radius_headroom") && census.names.contains("tube_wall"),
        "the reader found no funnel calls it should have: {:?}",
        census.names
    );
    let listed: BTreeSet<&str> = UNROUTED.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        listed.len(),
        UNROUTED.len(),
        "a name is listed twice in UNROUTED"
    );
    for (name, _) in UNROUTED {
        assert!(
            census.names.contains(*name),
            "`{name}` is listed as carrying no blend recourse, but nothing in the crate's \
             src decides it — a stale entry"
        );
    }
    for name in &census.names {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        let unrouted = falls_through(name);
        assert_eq!(
            unrouted,
            listed.contains(name),
            "`{name}` is {} by the door and {} in UNROUTED; the refusal reads: {}",
            if unrouted { "unrouted" } else { "routed" },
            if listed.contains(name) {
                "listed"
            } else {
                "unlisted"
            },
            rendered(name)
        );
    }
}

/// **A routed name renders the recourse its OWN arm owns.**
///
/// The roster row above measures routed-or-listed by asking whether the
/// gap sentence appears, so a name rewired to another arm's constant is
/// still "routed" and stays green there. What a reader of the refusal
/// depends on is the pairing — the ring clearance's lever is not the
/// chain's — so it is pinned here, name by name, against a table
/// written independently of the match.
///
/// The table is held complete against the crate's own decided names: a
/// `fillet3_*` gate the door routes and this table does not know reds.
#[test]
fn every_routed_name_renders_the_recourse_its_own_arm_owns() {
    let paired: BTreeSet<&str> = PAIRING.iter().map(|(n, _)| *n).collect();
    for (name, recourse) in PAIRING {
        let text = rendered(name);
        assert!(
            text.contains(recourse),
            "`{name}` should carry its own arm's recourse; it renders: {text}"
        );
        for (other_name, other) in PAIRING {
            assert!(
                other == recourse || !text.contains(other),
                "`{name}` renders the recourse `{other_name}`'s arm owns: {text}"
            );
        }
    }
    let routed: BTreeSet<&str> = census()
        .names
        .iter()
        .map(|n| -> &'static str { Box::leak(n.clone().into_boxed_str()) })
        .filter(|n| !falls_through(n))
        .collect();
    assert_eq!(
        routed, paired,
        "the door routes names this pairing table does not pin (or the table pins a name \
         the door no longer routes)"
    );
}

/// **A name no arm carries renders the gap sentence, and says nothing
/// else about the escalation.**
///
/// The same text `profile`'s door renders on ITS unknown name: one
/// sentence, one home, so the two tables cannot drift into two answers
/// to the same question.
#[test]
fn an_unknown_name_names_the_hole() {
    let unknown = "roster_unknown_probe";
    let census = census();
    assert!(
        !census.names.contains(unknown),
        "the probe name must be one the crate does not decide"
    );
    let text = rendered(unknown);
    assert!(
        text.contains(&MissingRecourse(Some(unknown)).to_string()),
        "the refusal names the hole: {text}"
    );
    assert!(
        text.contains(unknown),
        "the refusal names the predicate that escalated: {text}"
    );
    // A refusal with no predicate at all says so, rather than reading
    // as a name.
    let nameless = BlendError::Escalated {
        site: BlendSite::Chain,
        source: Indeterminate {
            predicate: None,
            ..escalation(unknown)
        },
    }
    .to_string();
    assert!(
        nameless.contains(&MissingRecourse(None).to_string()),
        "an escalation with no predicate name still names the hole: {nameless}"
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
    // The const carrier is resolved, not just declared.
    assert!(
        census.names.contains("fillet3_ring_clearance"),
        "the ring-clearance gate reaches the funnel as a const and must still be rostered"
    );
}
