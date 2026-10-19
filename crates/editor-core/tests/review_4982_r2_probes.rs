//! Second review probes for PR 4982 (a union fold step mints the
//! face–face row of a given tangency). Each probe prints what the
//! union answered against an independent oracle.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{self, insert, len, union_over};
use editor_core::{
    BooleanValue, ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId,
    ValuePayload,
};
use geom_core::Tol;

const PI: f64 = core::f64::consts::PI;

#[derive(Debug)]
struct Outcome {
    ff: usize,
    solids: usize,
    volume: Option<f64>,
    cites: Vec<usize>,
}

fn outcome(doc: &ProfileDoc, members: &[RecipeNodeId]) -> Result<Outcome, String> {
    let (doc, u) = union_over(doc.clone(), members);
    let ev = run(&doc);
    if let Some(e) = failure(&ev, u) {
        return Err(format!("{e:?}"));
    }
    let value = ev.value(u).expect("the union evaluated");
    let ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) = &value.payload else {
        return Err("not a body".into());
    };
    let mut o = Outcome {
        ff: 0,
        solids: body.solids().count(),
        volume: topo::mass_properties(body, Tol::witness())
            .ok()
            .map(|m| m.volume),
        cites: Vec::new(),
    };
    for c in contacts.rows() {
        if let topo::ContactKind::FaceFace(..) = c.record.kind() {
            o.ff += 1;
            o.cites.push(c.cites.iter().count());
        }
    }
    Ok(o)
}

/// A rod of radius 0.5 about (0, 2.5) in a sketch frame turned by
/// `lean` about x, occupying stations 0..1 along its axis, but whose
/// cylinder carrier's origin stands `back` behind station 0: the rod is
/// extruded from station −back to 1 and the part below station 0 is
/// subtracted by a tool on the station-0 frame.
fn far_origin_rod(doc: ProfileDoc, lean: f64, back: f64) -> (ProfileDoc, RecipeNodeId) {
    let (s, c) = (lean.sin(), lean.cos());
    // n = x × (0, c, s) = (0, −s, c); origin = −back·n.
    let (doc, plane) = insert(
        doc,
        fixture::frame([0.0, back * s, -back * c], [1.0, 0.0, 0.0], [0.0, c, s]),
    );
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::circle_split(0.0, 2.5, 0.5, 2, 0.0).expect("a finite circle")],
            ids: Vec::new(),
        }),
    );
    let (doc, long) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(back + 1.0),
            side: ExtrudeSide::Along,
        },
    );
    if back == 0.0 {
        return (doc, long);
    }
    let (doc, tp) = crate::fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, c, s],
        vec![vec![
            (-10.0, -10.0),
            (10.0, -10.0),
            (10.0, 10.0),
            (-10.0, 10.0),
        ]],
    );
    let (doc, tool) = insert(
        doc,
        Node::Extrude {
            profile: tp.into(),
            distance: len(back + 5.0),
            side: ExtrudeSide::Against,
        },
    );
    insert(
        doc,
        Node::Subtract {
            from: long.into(),
            tool: tool.into(),
        },
    )
}

fn sweep(label: &str, doc: &ProfileDoc, n: &[(&str, RecipeNodeId)], orders: &[&[usize]]) {
    for order in orders {
        let what: Vec<&str> = order.iter().map(|&i| n[i].0).collect();
        let members: Vec<RecipeNodeId> = order.iter().map(|&i| n[i].1).collect();
        eprintln!(
            "PROBE {label} [{}] => {:?}",
            what.join(","),
            outcome(doc, &members)
        );
    }
}

/// M1 class: the step rebuilds the line from the step's carriers'
/// stored origins (`tangent_line_on`), not at the pass's reach foot.
/// Measured on 9314545e7f (and on a scratch merge with
/// `contacthold/carry-by-lineage`): lean 0 or back 0 → ff 1 in every
/// order; lean ε/10 with back 300 or 3000 → the pair `[P, R]` ff 1, every
/// three-member order ff 0 (no refusal, same volume).
/// With the rod's carrier origin 300 behind the touch and a lean of a
/// tenth of ε, the rebuilt line stands ~30ε off the plate's face at
/// the touch. Oracle: one ff record (the pair door's), solids 3,
/// volume 16 + π/4 + 1.
#[test]
fn probe_far_origin_rod() {
    let eps = Tol::witness().eps();
    eprintln!("PROBE eps = {eps:e}; oracle V = {}", 17.0 + PI / 4.0);
    for (lean, back) in [
        (0.0, 300.0),
        (0.1 * eps, 0.0),
        (0.1 * eps, 300.0),
        (0.1 * eps, 3000.0),
    ] {
        let doc = ProfileDoc::empty_derived("rev4982far", Tol::witness());
        let (doc, p) = block(doc, (-2.0, 2.0), (-2.0, 2.0), 0.0, 1.0);
        let (doc, r) = far_origin_rod(doc, lean, back);
        let (doc, b) = block(doc, (5.0, 6.0), (5.0, 6.0), 0.0, 1.0);
        let n = [("P", p), ("R", r), ("B", b)];
        sweep(
            &format!("far lean={lean:e} back={back}"),
            &doc,
            &n,
            &[&[0, 1], &[0, 1, 2], &[2, 0, 1], &[1, 2, 0], &[0, 2, 1]],
        );
    }
}
