//! The `t`-interval door over the corpus, through the REAL door.
//!
//! Two claims are pinned here, in ONE pass over every landing of the
//! parametric corpus and the gallery ring:
//!
//! - **`Pruned == Every`.** `PickIndex::pick` — `pick_face` over the
//!   picture's parts — answers what an EXHAUSTIVE walk of every
//!   candidate box the ray meets answers — the one hit, the miss, or
//!   the whole LIST of a refusal. The exhaustive walk calls
//!   `ray_triangle` and `TSpan::survivors`, the door's own two doors,
//!   so it restates neither the traversal nor the order: what it does
//!   not have is the early-out, which is exactly what is under test.
//!   With the certified tie refused rather than broken, the margin is
//!   what makes a refusal COMPLETE as well as what makes a hit right:
//!   a pruned candidate is a tied face that would have gone unlisted.
//! - **The closed acceptance loses no aimed vertex** that `main`'s
//!   rounded-`t` order answered, and gains some.
//!
//! **The acceptance question is CLOSED and no longer measured here.**
//! The ruling left "closed ∧ INFORM stays, or MEET ∧ INFORM returns"
//! to the unit; the unit measured it (2026-09-16, the PR body and the
//! unit's row carry the numbers: MEET reaches `0 Pruned ≠ Every` under
//! the clamp but loses 513 of the wide aim's aimed vertices against
//! `main`'s 141 106 and gains none, so closed stays). A measurement
//! that has been made is not a permanent gate, and keeping MEET here
//! cost this file a second copy of `t_span` and of the acceptance —
//! the two things a reviewer greps for. Both are gone.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use std::collections::{BTreeMap, BTreeSet};

use editor_core::{Evaluation, HitTestError, RecipeNodeId};
use viewer::pickindex::PickIndex;

use crate::common::corpus_pick::{
    FlatHit, FlatReference, over_every_landing, tie_rays_for, too_wide, wide_aim,
};
use editor_core::test_support::listed;

// ---------------------------------------------------------------
// The exhaustive walk is `common::corpus_pick`'s `FlatReference`: every
// candidate box the ray meets, through `ray_triangle` and the door's
// own `answer_of`. No early-out, no order of its own.
// ---------------------------------------------------------------

/// `main`'s order: the rounded `t`, then position. The OLD door, kept
/// as the oracle the wide aim's `aim_lost` column is measured against
/// — not a second spelling of this one.
fn by_rounded_t(seen: &[FlatHit]) -> Option<FlatHit> {
    seen.iter().copied().reduce(|b, c| {
        if c.span.t < b.span.t || (c.span.t == b.span.t && (c.part, c.item) < (b.part, b.item)) {
            c
        } else {
            b
        }
    })
}

// ---------------------------------------------------------------
// The two aims.
// ---------------------------------------------------------------

/// The tie-break aim: what the early-out column is measured over.
#[derive(Default, Debug)]
struct TieTable {
    rays: usize,
    /// The door's answer differs from the exhaustive walk's — the
    /// hit, the miss, or the refusal's whole list. The early-out is
    /// the only difference between them, so any count here is the
    /// early-out pruning a candidate the set rule keeps.
    pruned_differs: usize,
    /// The door refused: the survivors named more than one face. The
    /// tie-break aim points at shared edges and vertices by
    /// construction, so this is most of it.
    ///
    /// **A LIVENESS guard, and nothing more.** It is asserted non-zero
    /// so that a corpus which stopped producing refusals cannot leave
    /// the rows below passing over an aim that never reached one; it
    /// measures nothing about the refusal itself. What does is the
    /// per-ray `Pruned == Every` above — which compares the whole LIST
    /// — and the wide aim's `aim_lost`, which asks by identity whether
    /// the face a ray was aimed at is among the ones the door names.
    refused: usize,
    /// The aimed point is not answered: the winner is beyond it, or
    /// the ray misses. Printed, not pinned — the corpus's shape, and
    /// the row above it is
    /// `work/edit/pick-closed-acceptance-loses-a-graze-to-rounding`.
    beyond_or_miss: usize,
    /// The winner carries a barycentric bound of `1` or more.
    /// **Structurally 0**: `admits` refuses `err >= 1` outright, so
    /// this column cannot red while INFORM stands. It is here as the
    /// statement of that, and it would red the day INFORM moved.
    wide_winners: usize,
    examples: Vec<String>,
}

/// The wide aim against `main`'s order over the same candidates.
#[derive(Default, Debug)]
struct WideTable {
    rays: usize,
    /// The door refused on this ray: the wide aim's own count of the
    /// class the ruling opened. Printed, not pinned — it is the
    /// corpus's shape.
    refused: usize,
    /// Of those refusals, the ones a hit at the AIMED PARAMETER is in
    /// — the reading the `aim_lost` column used to take for "the aim
    /// was kept".
    refused_at_the_aim: usize,
    /// And of THOSE, the ones that name the aimed vertex's own face:
    /// what survives the identity reading. The gap between the two is
    /// what the looser rule was counting.
    refused_naming_the_aim: usize,
    /// How many faces each refusal named, counted by arity. Incidence
    /// at a box corner is three, so four and six are the arities that
    /// say something else is going on — coincident faces of two
    /// instances, or a ray through two vertices of one body — and
    /// `wide_arity_examples` names the first of each.
    tied_arity: BTreeMap<usize, usize>,
    /// The first refusal of each arity above three, with the vertex it
    /// was aimed through and every face it named.
    wide_arity_examples: BTreeMap<usize, String>,
    moved: usize,
    moved_farther: usize,
    aimed_main: usize,
    aimed_interval: usize,
    aim_lost: usize,
    aim_gained: usize,
    aim_lost_examples: Vec<String>,
}

/// `Pruned == Every` over the tie-break aim, and the aim's own shape.
fn tie_sweep(
    name: &str,
    step: &str,
    index: &PickIndex,
    eval: &Evaluation<f64>,
    table: &mut TieTable,
) {
    let reference = FlatReference::of(index);
    for (ray, reach) in tie_rays_for(index) {
        table.rays += 1;
        let (exhaustive, _) = reference.pick(&ray);
        let door = listed(index.pick(eval, &ray));
        if door.len() > 1 {
            table.refused += 1;
        }
        // `Pruned == Every` over hit AND refusal: the two lists agree
        // face for face, in order, on all three numbers.
        let same = door.len() == exhaustive.len()
            && door.iter().zip(&exhaustive).all(|(d, e)| {
                (d.t.to_bits(), d.t_lo.to_bits(), d.t_hi.to_bits())
                    == (
                        e.span.t.to_bits(),
                        e.span.t_lo.to_bits(),
                        e.span.t_hi.to_bits(),
                    )
            });
        if !same {
            table.pruned_differs += 1;
            if table.examples.len() < 8 {
                table.examples.push(format!(
                    "{name}/{step}: PRUNED DIFFERS {:?} reach {reach}: door {:?} exhaustive {:?}",
                    ray.dir,
                    door.iter().map(|d| d.t).collect::<Vec<_>>(),
                    exhaustive.iter().map(|e| e.span.t).collect::<Vec<_>>()
                ));
            }
        }
        // The NEAREST of the list, not its first entry: the list's
        // order is the door's target order and decides nothing, so
        // "did the walk reach the aimed point" is a question about the
        // smallest parameter the answer holds.
        let nearest = exhaustive
            .iter()
            .min_by(|left, right| left.span.t.total_cmp(&right.span.t));
        match nearest {
            None => table.beyond_or_miss += 1,
            Some(win) => {
                if win.span.t > reach + 1e-9 {
                    table.beyond_or_miss += 1;
                }
                for win in &exhaustive {
                    if too_wide(&ray, &reference.parts[win.part].corners[win.item]).is_some() {
                        table.wide_winners += 1;
                    }
                }
            }
        }
    }
}

/// The wide aim: the door against `main`'s rounded-`t` order over the
/// same candidates.
fn wide_sweep(
    name: &str,
    step: &str,
    index: &PickIndex,
    eval: &Evaluation<f64>,
    table: &mut WideTable,
) {
    let reference = FlatReference::of(index);
    // Each part's patch names, and which patches each of its positions
    // belongs to: the aim knows WHICH FACE it aimed at, and a hit on
    // another face at the same depth is not that aim kept.
    let names: Vec<Vec<Result<pncad::prelude::StableName, HitTestError>>> = index
        .parts()
        .iter()
        .map(|part| {
            part.patch_names(eval)
                .expect("the parts are of this evaluation")
        })
        .collect();
    let faces_at: Vec<BTreeMap<u32, BTreeSet<usize>>> = index
        .parts()
        .iter()
        .map(|part| {
            let mut faces_at: BTreeMap<u32, BTreeSet<usize>> = BTreeMap::new();
            for (patch, p) in part.mesh().patches.iter().enumerate() {
                for tri in &p.triangles {
                    for &corner in tri {
                        faces_at.entry(corner).or_default().insert(patch);
                    }
                }
            }
            faces_at
        })
        .collect();
    for aim in wide_aim(index) {
        let (pi, part, v, dir, reach) = (
            aim.part,
            &index.parts()[aim.part],
            aim.at,
            aim.dir,
            aim.reach,
        );
        // The faces this vertex is a corner of, as the keys a hit
        // carries.
        let aimed_faces: Vec<(RecipeNodeId, u32, &pncad::prelude::StableName)> = faces_at[pi]
            .get(&(aim.vertex as u32))
            .into_iter()
            .flatten()
            .filter_map(|&patch| names[pi][patch].as_ref().ok())
            .map(|name| (part.node(), part.body(), name))
            .collect();
        let is_aimed = |node: RecipeNodeId, body: u32, name: &pncad::prelude::StableName| {
            aimed_faces
                .iter()
                .any(|(n, b, aimed)| *n == node && *b == body && *aimed == name)
        };
        let ray = aim.ray();
        table.rays += 1;
        let main = by_rounded_t(&reference.every(&ray));
        let here = listed(index.pick(eval, &ray));
        // The aim is ANSWERED FOR when the door names the
        // aimed vertex's OWN face at the aimed parameter —
        // one of the faces it names when it answers, any of
        // them when it refuses, because a refusal that
        // lists the aimed face has not lost it. By identity
        // and not by depth alone: a different face at the
        // same parameter is a different answer.
        let at_reach = |t: f64| (t - reach).abs() < 1e-9;
        let aimed_main = main.is_some_and(|s| {
            at_reach(s.span.t)
                && names[s.part][s.patch].as_ref().is_ok_and(|name| {
                    is_aimed(
                        index.parts()[s.part].node(),
                        index.parts()[s.part].body(),
                        name,
                    )
                })
        });
        let aimed_here = here
            .iter()
            .any(|h| at_reach(h.t) && is_aimed(h.node, h.body, &h.name));
        if here.len() > 1 {
            table.refused += 1;
            *table.tied_arity.entry(here.len()).or_default() += 1;
            if here.len() > 3 {
                table
                    .wide_arity_examples
                    .entry(here.len())
                    .or_insert_with(|| {
                        format!(
                            "{name}/{step}: {} faces {dir:?} through {v:?} reach \
                             {reach}: {}",
                            here.len(),
                            here.iter()
                                .map(|h| format!(
                                    "[node {:?} body {} {} {:?} t {}]",
                                    h.node, h.body, h.name, h.name.path, h.t
                                ))
                                .collect::<Vec<_>>()
                                .join(" | ")
                        )
                    });
            }
            // What the identity rule costs, counted rather
            // than argued: refusals the DEPTH rule alone
            // would have called the aim kept, and how many
            // of them the identity rule still does.
            if here.iter().any(|h| at_reach(h.t)) {
                table.refused_at_the_aim += 1;
                table.refused_naming_the_aim += usize::from(aimed_here);
            }
        }
        table.aimed_main += usize::from(aimed_main);
        table.aimed_interval += usize::from(aimed_here);
        table.aim_gained += usize::from(aimed_here && !aimed_main);
        if aimed_main && !aimed_here {
            table.aim_lost += 1;
            if table.aim_lost_examples.len() < 8 {
                table.aim_lost_examples.push(format!(
                    "{name}/{step}: AIM LOST {dir:?} through {v:?} reach {reach}: \
                     main {:?} door {:?}",
                    main.map(|s| s.span.t),
                    here.iter().map(|h| h.t).collect::<Vec<_>>()
                ));
            }
        }
        // "Moved" reads the NEAREST of what the door names,
        // not the first entry of a list whose order decides
        // nothing.
        let front = here.iter().min_by(|left, right| left.t.total_cmp(&right.t));
        if let (Some(m), Some(h)) = (main, front)
            && m.span.t.to_bits() != h.t.to_bits()
        {
            table.moved += 1;
            if h.t > m.span.t {
                table.moved_farther += 1;
            }
        }
    }
}

/// **The door answers what an exhaustive walk answers, and the closed
/// acceptance loses no aimed vertex.**
///
/// One pass over every landing, both aims.
///
/// `pruned_differs` is the property the early-out's proof claims:
/// `PickIndex::pick` runs `pick_face`, whose traversal skips a
/// candidate when the running bound is below its box entry by the
/// derived margin, and the exhaustive walk here runs the same two
/// doors over EVERY candidate box the ray meets. The two answers agree
/// on every ray or the margin is wrong.
///
/// `aim_lost` is the column the ruling read for the acceptance. An aim
/// is kept when the door names the aimed vertex's OWN face at the
/// aimed parameter — the refusal that LISTS that face has not lost it,
/// and a different face at the same depth is not it — and it can red:
/// a door that reordered the candidates in a way that answered a
/// nearer face alone would lose aims here.
///
/// `refused` is a LIVENESS guard on the tie-break aim, not a
/// measurement of the refusal: the aim points at shared edges and
/// vertices by construction, so a zero there would mean the corpus
/// stopped producing the case and the two claims above were passing
/// over nothing. The refusal itself is measured by `pruned_differs`
/// over the whole list and by `aim_lost`'s identity reading.
///
/// The counts are printed. The asserted ones are named below; the
/// rest are the corpus's shape, and `wide_winners` is disclosed above
/// as structurally zero.
#[test]
fn the_door_answers_the_exhaustive_walk_and_keeps_every_aimed_vertex() {
    let mut tie = TieTable::default();
    let mut wide = WideTable::default();
    over_every_landing(|name, step, index, eval| {
        tie_sweep(name, step, index, eval, &mut tie);
        wide_sweep(name, step, index, eval, &mut wide);
    });
    println!("# EDIT-PICK3 tie-break aim: {tie:#?}");
    println!("# EDIT-PICK3 wide aim: {wide:#?}");
    assert!(
        tie.rays > 19_000,
        "the tie-break aim is the whole corpus's, not a subset: {} rays",
        tie.rays
    );
    assert!(
        wide.rays > 400_000,
        "the wide aim is the whole corpus's: {} rays",
        wide.rays
    );
    assert_eq!(
        tie.pruned_differs, 0,
        "the door's answer differs from the exhaustive walk's on {} of {} rays — the early-out \
         is pruning a candidate the certified tie keeps, which costs a hit its answer or a \
         refusal an entry",
        tie.pruned_differs, tie.rays
    );
    assert!(
        tie.refused > 0,
        "the tie-break aim reached no refusal at all, so `Pruned == Every` over a refusal's \
         list ran over no refusal: the corpus, not the door, is what this says moved: {tie:#?}"
    );
    assert_eq!(
        tie.wide_winners, 0,
        "{} winners carry a barycentric bound of 1 or more, which INFORM refuses",
        tie.wide_winners
    );
    assert_eq!(
        wide.aim_lost, 0,
        "the door names none of {} aimed vertices main's rounded-t order answered",
        wide.aim_lost
    );
    assert!(
        wide.aim_gained > 0,
        "the interval order gains no aimed vertex over main's, which is what makes it more than \
         a re-spelling: {wide:#?}"
    );
}
