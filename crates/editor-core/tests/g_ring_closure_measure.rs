//! **A ring's seats under the coincidence door** (§8 Q1, rule H): a
//! ring of `N` planar wedges about z, each wedge the trapezoid on the
//! declared radii `ri < ro` from its near side `(ri, 0)–(ro, 0)` to its
//! far side at the wedge's angle, extruded 1, member `k` spun by its
//! spin value about z, its members touching their neighbours side to
//! side. Each side seat — member `k`'s far side on member `k + 1`'s
//! near side, the closing member `N − 1`'s on member 0's — is one
//! `SameOpposite` row of the census, and the rows read which of them
//! the door proves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::fixture::{self, insert, len};
use editor_core::{
    Coord, Dimension, DocEdit, Formula, FreeVar, LoopProgram, Node, Operand, ProfileDoc,
    ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, TargetAxis, VarDecl, VarName,
};
use geom_core::Tol;

/// How the wedge's far side is written.
#[derive(Clone, Copy, Debug)]
enum Far {
    /// At `turn/N`, through `cos(turn/N)` and `sin(turn/N)`.
    Turn,
    /// At the free angle `step`, through `cos(step)` and `sin(step)`.
    Step,
    /// At `turn/N` through the half angle `h = turn/2N`: `r·(1 −
    /// 2·sin²h)` and `r·2·sin h·cos h`.
    Half,
}

/// How member `k` is spun.
#[derive(Clone, Copy, Debug)]
enum Spin {
    /// By the exact `k/N` of a turn.
    Turns,
    /// By `k · step`, the free angle the far side reads.
    Step,
    /// By a free angle of its own, at the value `k/N` of a turn.
    Free,
    /// By `k/N` of a turn about z, then tilted by a seventh of a turn
    /// about x: the ring on a tilted axis.
    Tilted,
}

/// A free variable `name` of `dim` at `value`, and the document.
fn declare(doc: ProfileDoc, name: &str, dim: Dimension, value: f64) -> ProfileDoc {
    fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::new(name).unwrap(),
            def: VarDecl::Free(FreeVar::continuous(dim, value)),
        },
    )
    .0
}

/// The wedge of a ring of `n`, its far side written `far`.
fn wedge(doc: ProfileDoc, n: u64, far: Far) -> (ProfileDoc, RecipeNodeId) {
    let mut doc = doc;
    let mut params = BTreeMap::new();
    for (name, value) in [("ri", 2.0), ("ro", 3.0)] {
        doc = declare(doc, name, Dimension::Length, value);
        params.insert(VarName::new(name).unwrap(), Dimension::Length);
    }
    let turn = std::f64::consts::TAU;
    doc = declare(doc, "step", Dimension::Angle, turn / n as f64);
    params.insert(VarName::new("step").unwrap(), Dimension::Angle);
    let f = |text: &str| editor_core::parse_formula(text, &params).unwrap();
    let zero = || Formula::zero(Dimension::Length);
    let at = |r: &str| match far {
        Far::Turn => [
            f(&format!("{r} * cos(turn / {n})")),
            f(&format!("{r} * sin(turn / {n})")),
        ],
        Far::Step => [
            f(&format!("{r} * cos(step)")),
            f(&format!("{r} * sin(step)")),
        ],
        Far::Half => {
            let h = format!("(turn / {})", 2 * n);
            [
                f(&format!("{r} - 2 * {r} * sin({h}) * sin({h})")),
                f(&format!("2 * {r} * sin({h}) * cos({h})")),
            ]
        }
    };
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At([f("ri"), zero()]),
                ProgramStep::LineTo(ProgramTarget::Point([f("ro"), zero()])),
                ProgramStep::LineTo(ProgramTarget::Point(at("ro"))),
                ProgramStep::LineTo(ProgramTarget::Point(at("ri"))),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// Member `k`'s moves.
fn moves(doc: &mut ProfileDoc, n: u64, k: u64, spin: Spin) -> Vec<(Coord, Formula)> {
    let about = |axis| Coord::Spin(Some(axis));
    let turns = |num: u64, den: u64| {
        Formula::mul(
            Formula::ratio(i64::try_from(num).unwrap(), den).unwrap(),
            Formula::turn(),
        )
        .unwrap()
    };
    match spin {
        Spin::Turns => vec![(about(TargetAxis::Z), turns(k, n))],
        Spin::Step => {
            let step = doc.var_named("step").unwrap();
            vec![(
                about(TargetAxis::Z),
                Formula::mul(
                    Formula::ratio(i64::try_from(k).unwrap(), 1).unwrap(),
                    Formula::var(step, Dimension::Angle),
                )
                .unwrap(),
            )]
        }
        Spin::Free => {
            let name = format!("a{k}");
            let value = std::f64::consts::TAU * k as f64 / n as f64;
            *doc = declare(doc.clone(), &name, Dimension::Angle, value);
            let a = doc.var_named(&name).unwrap();
            vec![(about(TargetAxis::Z), Formula::var(a, Dimension::Angle))]
        }
        Spin::Tilted => vec![
            (about(TargetAxis::Z), turns(k, n)),
            (about(TargetAxis::X), turns(1, 7)),
        ],
    }
}

/// N world placements of the wedge, member `k` spun by `spin`, its
/// bundle mateless; member 0 is the wedge where `spin` leaves it
/// unturned.
fn ring(n: u64, far: Far, spin: Spin) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty_derived("g-ring", Tol::witness());
    let (mut doc, wedge) = wedge(doc, n, far);
    let body = doc.output(wedge, 0).expect("the body");
    let mut members = Vec::new();
    for k in 0..n {
        let moves = moves(&mut doc, n, k, spin);
        let placed = editor_core::Placed::of(&doc, Vec::new(), moves);
        let node = fixture::place_node(
            &doc,
            editor_core::BodyRead::plain(Operand::Var(body)),
            placed.bundle(),
        );
        let (d, p) = insert(doc, node);
        doc = fixture::assemble_world_copies(d);
        members.push(p);
    }
    (doc, members)
}

/// One seat's members and whether the door proves it.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Seat {
    members: (usize, usize),
    proven: bool,
}

/// The ring's side seats (its `SameOpposite` rows), member pair
/// ordered, with the door's verdict on each, and the seconds the door
/// took over all of them.
fn side_seats(doc: &ProfileDoc, members: &[RecipeNodeId]) -> (Vec<Seat>, f64) {
    let ev = fixture::run(doc, &Default::default());
    let product = editor_core::product_recorded(doc, &ev, Tol::witness()).expect("gathers");
    let at = |var| {
        members
            .iter()
            .position(|&m| doc.output(m, 0) == Some(var))
            .expect("a member's copy")
    };
    let t = std::time::Instant::now();
    let mut seats = Vec::new();
    for r in &product.coincidences {
        if format!("{:?}", r.row.relation) != "SameOpposite" {
            continue;
        }
        let (a, b) = &r.copies;
        let node = doc.operation_of(a.var).expect("a copy's placement");
        let rows = [r.row.clone()];
        let proof = editor_core::coincide::prove(
            doc,
            &[(node, &rows[..])],
            None,
            Tol::witness(),
            &editor_core::CancelToken::new(),
        )
        .expect("door");
        let (x, y) = (at(a.var), at(b.var));
        seats.push(Seat {
            members: (x.min(y), x.max(y)),
            proven: matches!(proof[0][0], editor_core::Proof::Structural(_)),
        });
    }
    seats.sort();
    (seats, t.elapsed().as_secs_f64())
}

/// The seats the door leaves unproven.
fn unproven(seats: &[Seat]) -> Vec<(usize, usize)> {
    seats
        .iter()
        .filter(|s| !s.proven)
        .map(|s| s.members)
        .collect()
}

/// **A ring stepped by an exact `turn/N` proves every side seat, the
/// closing one included** (rule H): `N ∈ {5, 6, 8, 12}`, each with its
/// `N` seats. Red if a seat whose two faces meet by angle addition is
/// left to the numeric channel.
#[test]
fn a_ring_stepped_by_an_exact_turn_proves_every_side_seat() {
    for n in [5, 6, 8, 12] {
        let (doc, members) = ring(n, Far::Turn, Spin::Turns);
        let (seats, _) = side_seats(&doc, &members);
        assert_eq!(seats.len(), n as usize, "N = {n}: {seats:?}");
        assert_eq!(unproven(&seats), [], "N = {n}");
    }
}

/// **A ring with a member at a rational cosine proves every side
/// seat** (rule H): a ring of 9 or of 24 has members at a third and two
/// thirds of a turn, whose cosine `−1/2` meets the irrational steps
/// beside it as a constant atom of the rewrite (`geom_core`'s
/// `sym::turn` docs). Red if a seat beside a rational member is left to
/// the numeric channel.
#[test]
fn a_ring_with_a_member_at_a_rational_cosine_proves_every_side_seat() {
    for n in [9, 24] {
        let (doc, members) = ring(n, Far::Turn, Spin::Turns);
        let (seats, _) = side_seats(&doc, &members);
        assert_eq!(seats.len(), n as usize, "N = {n}: {seats:?}");
        assert_eq!(unproven(&seats), [], "N = {n}");
    }
}

/// **A ring stepped by a FREE step proves every seat but the closing
/// one**: member `k`'s far side at `(k + 1)·step` meets member `k + 1`'s
/// near side as an identity in `step`, and member `N − 1`'s at `N·step`
/// meets member 0's only where `N·step` is a whole turn, which no
/// identity says. Red if the closing seat is proven (a closure claimed
/// for a step that is not a fraction of a turn) or another is left.
#[test]
fn a_ring_stepped_by_a_free_step_leaves_the_closing_seat_alone_unproven() {
    for n in [6, 12] {
        let (doc, members) = ring(n, Far::Step, Spin::Step);
        let (seats, _) = side_seats(&doc, &members);
        assert_eq!(seats.len(), n as usize, "N = {n}: {seats:?}");
        assert_eq!(unproven(&seats), [(0, n as usize - 1)], "N = {n}");
    }
}

/// **A ring whose members' angles are free literals proves no seat**:
/// each member spun by an angle of its own, at the ring's value, so no
/// two members' angles are related by anything the document says. Red
/// if a seat is proven from values the door was not given as a relation.
#[test]
fn a_ring_of_free_member_angles_proves_no_seat() {
    let n = 6;
    let (doc, members) = ring(n, Far::Turn, Spin::Free);
    let (seats, _) = side_seats(&doc, &members);
    assert_eq!(seats.len(), n as usize, "{seats:?}");
    assert!(seats.iter().all(|s| !s.proven), "{seats:?}");
}

/// **A far side written through the half angle proves as the far side
/// at the whole one does**: `r·(1 − 2·sin²h)` and `r·2·sin h·cos h` at
/// `h = turn/2N` are `r·cos(turn/N)` and `r·sin(turn/N)` by the product
/// rewrite alone. Red if the door needs the two spellings to be one.
#[test]
fn a_far_side_written_through_the_half_angle_proves_every_seat() {
    let n = 12;
    let (doc, members) = ring(n, Far::Half, Spin::Turns);
    let (seats, _) = side_seats(&doc, &members);
    assert_eq!(seats.len(), n as usize, "{seats:?}");
    assert_eq!(unproven(&seats), []);
}

/// One wedge with its far side at the free angle `a`, placed spun by
/// `spin`, against a second wedge whose near side is written along
/// `near` and which stands unturned: their one side seat.
fn two_wedges(near: &str, spin: impl Fn(&ProfileDoc) -> Formula) -> Vec<Seat> {
    let mut doc = ProfileDoc::empty_derived("g-a-plus-b", Tol::witness());
    let mut params = BTreeMap::new();
    for (name, dim, value) in [
        ("ri", Dimension::Length, 2.0),
        ("ro", Dimension::Length, 3.0),
        ("a", Dimension::Angle, 0.4),
        ("b", Dimension::Angle, 0.7),
        ("c", Dimension::Angle, 0.5),
    ] {
        doc = declare(doc, name, dim, value);
        params.insert(VarName::new(name).unwrap(), dim);
    }
    let f = |text: &str| editor_core::parse_formula(text, &params).unwrap();
    let ray = |r: &str, angle: &str| {
        [
            f(&format!("{r} * cos({angle})")),
            f(&format!("{r} * sin({angle})")),
        ]
    };
    let mut wedges = Vec::new();
    let far = format!("{near} + c");
    for (near, far) in [("0 * a", "a"), (near, far.as_str())] {
        let (d, profile) = insert(
            doc,
            Node::Profile(ProfileProgram {
                loops: vec![LoopProgram::Chain(vec![
                    ProgramStep::At(ray("ri", near)),
                    ProgramStep::LineTo(ProgramTarget::Point(ray("ro", near))),
                    ProgramStep::LineTo(ProgramTarget::Point(ray("ro", far))),
                    ProgramStep::LineTo(ProgramTarget::Point(ray("ri", far))),
                    ProgramStep::LineTo(ProgramTarget::Start),
                ])],
                ids: Vec::new(),
                on: None,
            }),
        );
        let (d, wedge) = insert(
            d,
            Node::Extrude {
                profile: profile.into(),
                distance: len(1.0),
            },
        );
        doc = d;
        wedges.push(wedge);
    }
    let mut members = Vec::new();
    for (wedge, spin) in [
        (wedges[0], spin(&doc)),
        (wedges[1], Formula::zero(Dimension::Angle)),
    ] {
        let body = doc.output(wedge, 0).unwrap();
        let placed = editor_core::Placed::of(
            &doc,
            Vec::new(),
            vec![(Coord::Spin(Some(TargetAxis::Z)), spin)],
        );
        let node = fixture::place_node(
            &doc,
            editor_core::BodyRead::plain(Operand::Var(body)),
            placed.bundle(),
        );
        let (d, p) = insert(doc, node);
        doc = fixture::assemble_world_copies(d);
        members.push(p);
    }
    side_seats(&doc, &members).0
}

/// **What the door does not reach, pinned as a limit: a seat between
/// two CONSTRUCTIONS.** A face at angle `a` spun by `b` against a face
/// written at `a + b` in another body is left unproven, and so is the
/// control with no angle sum in it at all — the same face at `a`
/// written in two bodies, neither turned — so what stands is the door's
/// verdict between two constructions of one carrier (the residual's
/// words: "two constructions, placed or reflected apart"), not rule H:
/// with rule H shut both rows freeze on the budget, with it on both
/// come back not shown zero. A ring's seats are between two copies of
/// one construction and prove. Red when the door reaches either.
#[test]
fn a_seat_between_two_constructions_of_one_carrier_is_unproven() {
    let spun = two_wedges("a + b", |doc| {
        Formula::var(doc.var_named("b").unwrap(), Dimension::Angle)
    });
    let control = two_wedges("a", |_| Formula::zero(Dimension::Angle));
    let unproven = [Seat {
        members: (0, 1),
        proven: false,
    }];
    assert_eq!(
        (spun.as_slice(), control.as_slice()),
        (&unproven[..], &unproven[..])
    );
}

/// **Unmeasured risk, measured**: a ring of 12 on a tilted axis — each
/// member spun by `k/12` of a turn about z and then tilted by a seventh
/// of a turn about x — so every seat's residual multiplies the tilt's
/// irrational atoms into the spin's. Red if the tilt costs a seat the
/// untilted ring proves.
#[test]
fn a_ring_on_a_tilted_axis_proves_every_side_seat() {
    let n = 12;
    let (doc, members) = ring(n, Far::Turn, Spin::Tilted);
    let (seats, _) = side_seats(&doc, &members);
    assert_eq!(seats.len(), n as usize, "{seats:?}");
    assert_eq!(unproven(&seats), []);
}

/// MEASUREMENT: the door's time over each ring's side seats, and which
/// it proves, at the shipped 256 terms.
#[test]
#[ignore = "measurement: cargo nextest run -p editor-core --run-ignored only all g_ring_closure_measure --no-capture"]
fn measure_ring_closure() {
    for (n, far, spin) in [
        (4, Far::Turn, Spin::Turns),
        (5, Far::Turn, Spin::Turns),
        (6, Far::Turn, Spin::Turns),
        (8, Far::Turn, Spin::Turns),
        (9, Far::Turn, Spin::Turns),
        (12, Far::Turn, Spin::Turns),
        (16, Far::Turn, Spin::Turns),
        (24, Far::Turn, Spin::Turns),
        (12, Far::Step, Spin::Step),
        (6, Far::Turn, Spin::Free),
        (12, Far::Half, Spin::Turns),
        (12, Far::Turn, Spin::Tilted),
    ] {
        let (doc, members) = ring(n, far, spin);
        let (seats, seconds) = side_seats(&doc, &members);
        println!(
            "N = {n:>2} far {far:?} spin {spin:?}: {} seats, unproven {:?}, door {seconds:.2} s",
            seats.len(),
            unproven(&seats)
        );
    }
}
