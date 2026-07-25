//! Feasibility probes for the #91 montage refresh (throwaway; results
//! feed the demo implementations, then this bin is deleted).
//!
//! P1 sheave revolve (torus zone) + STEP-on-curved refusal
//! P2 split of a boolean result + translated halves (#84)
//! P3 project-box chain: cavity, wall tunnels, floor bosses, pockets
//! P4 editor-core recipe: pattern doc, count edit, recompute counters
//! P5 cross-lap: notched beams, mated union refusal class

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::{LoopBuilder, Profile, ProfileLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{
    Body, BooleanBody, BooleanResult, mass_properties, validate, validate_closed,
    validate_geometric, validate_pseudomanifold,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, z.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let profile = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .expect("slab profile");
    extrude(&profile, Extrusion::Distance(z.1 - z.0))
        .expect("extrude slab")
        .body
}

fn boolean(
    what: &str,
    r: Result<BooleanResult<f64>, topo::BooleanError>,
    expected: f64,
) -> BooleanBody<f64> {
    match r {
        Ok(BooleanResult::Body(bb)) => {
            let v = mass_properties(&bb.body).expect("mass props").volume;
            assert_eq!(v, expected, "{what}: exact volume");
            let t3 = validate_pseudomanifold(&bb.body, &bb.contacts);
            println!("   {what}: OK kind {:?}, V = {v} exact, 3' {:?}", bb.kind, t3.is_ok());
            if let Err(e) = t3 {
                println!("      3' errors: {e:?}");
            }
            bb
        }
        Ok(BooleanResult::Empty) => panic!("{what}: EMPTY"),
        Err(e) => panic!("{what}: refused {e:?}"),
    }
}

// ---------------- P1: sheave ----------------
fn p1_sheave() {
    println!("== P1 sheave (polyline+arc full revolve, ring-torus groove) ==");
    let lp = LoopBuilder::start(p2(0.4, 0.0))
        .line_to(p2(0.9, 0.0))
        .line_to(p2(0.9, 0.25))
        .line_to(p2(1.6, 0.25))
        .line_to(p2(1.6, 0.0))
        .line_to(p2(2.1, 0.0))
        .line_to(p2(2.1, 0.2))
        .arc_to_via(p2(1.8, 0.5), p2(2.1, 0.8))
        .line_to(p2(2.1, 1.0))
        .line_to(p2(1.6, 1.0))
        .line_to(p2(1.6, 0.75))
        .line_to(p2(0.9, 0.75))
        .line_to(p2(0.9, 1.0))
        .line_to(p2(0.4, 1.0))
        .close();
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .expect("sheave profile");
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    let body = revolve(&profile, axis, Revolution::Full)
        .expect("revolve sheave")
        .body;
    println!("   tiers: {:?} {:?} {:?}",
        validate(&body).is_ok(), validate_closed(&body).is_ok(),
        validate_geometric(&body).is_ok());
    let torus_walls = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(topo::Surface::Torus { .. })
            )
        })
        .count();
    println!("   torus faces: {torus_walls}");
    let props = mass_properties(&body).expect("mass props");
    let pi = core::f64::consts::PI;
    let oracle = 3.411 * pi - 0.189 * pi * pi;
    println!("   V = {}, closed-form = {oracle}, rel = {:.3e}",
        props.volume, ((props.volume - oracle) / oracle).abs());
    let mesh = mesh::tessellate(&body, 2e-2).expect("tessellate");
    println!("   mesh: {} patches, watertight {:?}",
        mesh.patches.len(), mesh::validate::check_mesh(&mesh).is_ok());
    match step_export::step_string(&body, &step_export::StepOptions::default()) {
        Ok(s) => println!("   STEP export: OK ({} bytes)?!", s.len()),
        Err(e) => println!("   STEP export refusal (expected, M5 arms): {e:?}"),
    }
}

// ---------------- P3 + P2: project box, then split it ----------------
fn p3_projectbox() -> Body<f64> {
    println!("== P3 project box (cavity, 6 wall tunnels, 4 bosses, 4 pockets) ==");
    let outer = slab((0.0, 3.0), (0.0, 2.0), (0.0, 1.5));
    let cavity = slab((0.25, 2.75), (0.25, 1.75), (0.25, 2.0));
    let mut vol = 9.0 - 2.5 * 1.5 * 1.25;
    let mut acc = boolean("cavity", topo::subtract(&outer, &cavity), vol);
    // Vent slots: 3 per long wall, through both wall faces.
    let xs = [(0.5, 0.875), (1.3125, 1.6875), (2.125, 2.5)];
    for (i, &x) in xs.iter().enumerate() {
        for (j, y) in [(-0.25, 0.5), (1.5, 2.25)].into_iter().enumerate() {
            let cutter = slab(x, y, (0.5, 1.25));
            vol -= 0.375 * 0.25 * 0.75;
            acc = boolean(&format!("vent[{i}{j}]"), topo::subtract(&acc.body, &cutter), vol);
        }
    }
    // Bosses: 4 on the floor, overlapping 1/16 into it.
    let bx = [(0.4375, 0.8125), (2.1875, 2.5625)];
    let by = [(0.4375, 0.8125), (1.1875, 1.5625)];
    for (i, &x) in bx.iter().enumerate() {
        for (j, &y) in by.iter().enumerate() {
            let boss = slab(x, y, (0.1875, 0.875));
            vol += 0.375 * 0.375 * 0.625;
            acc = boolean(&format!("boss[{i}{j}]"), topo::union(&acc.body, &boss), vol);
        }
    }
    // Pilot pockets: centered in each boss top.
    for (i, &x) in bx.iter().enumerate() {
        for (j, &y) in by.iter().enumerate() {
            let px = (x.0 + 0.09375, x.1 - 0.09375);
            let py = (y.0 + 0.09375, y.1 - 0.09375);
            let pocket = slab(px, py, (0.5625, 1.0625));
            vol -= 0.1875 * 0.1875 * 0.3125;
            acc = boolean(&format!("pocket[{i}{j}]"), topo::subtract(&acc.body, &pocket), vol);
        }
    }
    println!("   final V = {vol}");
    acc.body
}

fn p2_split(boxbody: &Body<f64>) {
    println!("== P2 split of the boolean-result box by a tilted plane ==");
    let plane = SplitPlane {
        origin: Point3::new(1.5, 1.0, 0.75),
        normal: Vec3::new(0.75, 0.1875, 1.0),
    };
    match split(boxbody, &plane) {
        Ok(res) => {
            for (side, part) in [("above", &res.above), ("below", &res.below)] {
                match part {
                    SplitPart::Body(b) => {
                        println!(
                            "   {side}: tiers {:?} {:?} {:?}, V = {}",
                            validate(b).is_ok(),
                            validate_closed(b).is_ok(),
                            validate_geometric(b).is_ok(),
                            mass_properties(b).expect("mass props").volume
                        );
                        let map = geom_core::Affine3::translation(Vec3::new(
                            if side == "above" { 0.75 } else { -0.75 },
                            0.1875 * 0.75,
                            if side == "above" { 1.0 } else { -1.0 },
                        ));
                        let moved = topo::transform_rigid(b, &map).expect("translate half");
                        println!(
                            "      translated: tiers {:?} {:?} {:?}",
                            validate(&moved).is_ok(),
                            validate_closed(&moved).is_ok(),
                            validate_geometric(&moved).is_ok()
                        );
                    }
                    SplitPart::Empty => println!("   {side}: EMPTY"),
                }
            }
        }
        Err(e) => println!("   split refused (fallback to sweep body): {e:?}"),
    }
}

// ---------------- P5: cross-lap ----------------
fn p5_crosslap() {
    println!("== P5 cross-lap joint ==");
    let beam_a = slab((0.0, 4.0), (1.75, 2.25), (0.0, 0.5));
    let cut_a = slab((1.75, 2.25), (1.5, 2.5), (0.25, 0.75));
    let a = boolean("notch A", topo::subtract(&beam_a, &cut_a), 1.0 - 0.0625);
    let beam_b = slab((1.75, 2.25), (0.0, 4.0), (0.0, 0.5));
    let cut_b = slab((1.5, 2.5), (1.75, 2.25), (-0.25, 0.25));
    let b = boolean("notch B", topo::subtract(&beam_b, &cut_b), 1.0 - 0.0625);
    println!("   mated union attempt (flush mating planes):");
    match topo::union(&a.body, &b.body) {
        Ok(BooleanResult::Body(bb)) => println!(
            "      SUCCEEDED?! kind {:?}, V = {}",
            bb.kind,
            mass_properties(&bb.body).expect("props").volume
        ),
        Ok(BooleanResult::Empty) => println!("      EMPTY?!"),
        Err(e) => println!("      typed refusal: {e:?}"),
    }
    let map = geom_core::Affine3::translation(Vec3::new(0.0, 0.0, 1.25));
    let lifted = topo::transform_rigid(&b.body, &map).expect("lift B");
    println!(
        "   exploded B: tiers {:?} {:?} {:?}",
        validate(&lifted).is_ok(),
        validate_closed(&lifted).is_ok(),
        validate_geometric(&lifted).is_ok()
    );
    for (name, body) in [("A", &a.body), ("B", &b.body)] {
        match step_export::step_string(body, &step_export::StepOptions::default()) {
            Ok(s) => println!("   STEP {name}: OK ({} bytes)", s.len()),
            Err(e) => println!("   STEP {name}: refused {e:?}"),
        }
    }
}

// ---------------- P4: recipe heat sink ----------------
fn p4_recipe() {
    use editor_core::{
        Doc, DocEdit, Expr, Node, PatternKind, ProfileDesc, SlotId, ValuePayload, apply,
        evaluate,
    };
    println!("== P4 heat-sink recipe (pattern count 5 -> 7, memo counters) ==");
    let base_profile = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(3.0, 0.0),
            p2(3.0, 1.0),
            p2(0.0, 1.0),
        ])],
    );
    let fin_plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.1875),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let fin_profile = Profile::new(
        fin_plane,
        vec![ProfileLoop::polygon([
            p2(0.25, 0.125),
            p2(0.4375, 0.125),
            p2(0.4375, 0.875),
            p2(0.25, 0.875),
        ])],
    );
    let mut doc: Doc<ProfileDesc> = Doc::empty();
    let mut insert = |doc: &mut Doc<ProfileDesc>, node| {
        let applied = apply(doc, &DocEdit::InsertNode { node }).expect("insert");
        *doc = applied.doc;
        applied.record.minted.expect("minted id")
    };
    let base_p = insert(&mut doc, Node::Profile(ProfileDesc(base_profile)));
    let base_e = insert(
        &mut doc,
        Node::Extrude {
            profile: base_p,
            distance: Expr::literal(0.25, editor_core::Dimension::Length).unwrap(),
        },
    );
    let fin_p = insert(&mut doc, Node::Profile(ProfileDesc(fin_profile)));
    let fin_e = insert(
        &mut doc,
        Node::Extrude {
            profile: fin_p,
            distance: Expr::literal(0.8125, editor_core::Dimension::Length).unwrap(),
        },
    );
    let pattern = insert(
        &mut doc,
        Node::Pattern {
            input: fin_e,
            count: Expr::count(5),
            kind: PatternKind::Linear {
                direction: [
                    Expr::literal(1.0, editor_core::Dimension::Scalar).unwrap(),
                    Expr::literal(0.0, editor_core::Dimension::Scalar).unwrap(),
                    Expr::literal(0.0, editor_core::Dimension::Scalar).unwrap(),
                ],
                spacing: Expr::literal(0.3125, editor_core::Dimension::Length).unwrap(),
            },
        },
    );
    let cancel = editor_core::CancelToken::new();
    let opts = editor_core::EvalOptions::default();
    let ev1 = evaluate::<f64>(&doc, None, &cancel, &opts);
    println!(
        "   eval 1: outcome {:?}, recomputed {}, reused {}",
        ev1.outcome, ev1.recomputed, ev1.reused
    );
    let heatsink = |ev: &editor_core::Evaluation<f64>, n: usize| -> Body<f64> {
        let base = match &ev.value(base_e).expect("base value").payload {
            ValuePayload::Body(b) => (**b).clone(),
            other => panic!("base payload {other:?}"),
        };
        let fins = match &ev.value(pattern).expect("pattern value").payload {
            ValuePayload::Instances(v) => v.clone(),
            other => panic!("pattern payload {other:?}"),
        };
        assert_eq!(fins.len(), n);
        let mut acc = base;
        let mut vol = 3.0 * 1.0 * 0.25;
        for (i, fin) in fins.iter().enumerate() {
            vol += 0.1875 * 0.75 * (0.8125 - 0.0625);
            let bb = boolean(&format!("fin[{i}]"), topo::union(&acc, fin), vol);
            acc = bb.body;
        }
        acc
    };
    let hs5 = heatsink(&ev1, 5);
    println!("   heatsink(5): V = {}", mass_properties(&hs5).unwrap().volume);

    // sample a stable name from the pattern's table
    let table = &ev1.value(pattern).expect("pattern").name_table;
    let sample = table.iter().take(2).map(|(n, _)| format!("{n:?}")).collect::<Vec<_>>();
    println!("   pattern names (2 of {}): {sample:?}", table.len());

    let applied = apply(
        &doc,
        &DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: Expr::count(7),
        },
    )
    .expect("count edit");
    let doc7 = applied.doc;
    let ev2 = evaluate::<f64>(&doc7, Some(&ev1), &cancel, &opts);
    println!(
        "   eval 2 (count 7): recomputed {}, reused {}",
        ev2.recomputed, ev2.reused
    );
    let hs7 = heatsink(&ev2, 7);
    println!("   heatsink(7): V = {}", mass_properties(&hs7).unwrap().volume);
    // name survival: same name entries present in both evals' tables
    let t2 = &ev2.value(pattern).expect("pattern").name_table;
    let survived = table.iter().filter(|(n, _)| t2.lookup(n).is_some()).count();
    println!(
        "   name survival: {survived}/{} of eval-1 pattern names resolve after the edit",
        table.len()
    );
}

fn main() {
    p1_sheave();
    p5_crosslap();
    let boxbody = p3_projectbox();
    p2_split(&boxbody);
    p4_recipe();
    println!("probes complete");
}
