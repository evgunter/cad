//! The certified determinant measured over the corpus with a WIDER
//! aim than the tie-break row: every mesh vertex (subsampled), six
//! axis directions, three reaches, on the gallery ring and every
//! parametric corpus document at open and after the first edit. The
//! row came in as a review probe (branch `review/pick-r2`) against a
//! box-entry guard the unit withdrew; it now asserts the behaviour of
//! the certified determinant and pins what that mechanism refuses on
//! this corpus.
//!
//! Three claims, one row:
//!
//! 1. **No genuine determinant is refused.** A candidate whose
//!    conditioning `|det| / (|e1|·|e2|·|d|)` is at or above `1e-12`
//!    — four orders above the certification's own bound — is never
//!    refused at the determinant. Red = the mechanism refuses a
//!    crossing it could certify.
//! 2. **The refusals are pinned.** The count of candidates refused at
//!    the determinant (a non-zero determinant the certification
//!    cannot vouch for) and the count of rays carrying one are pinned
//!    to the corpus as it is: a change in either is a change in the
//!    class the mechanism refuses — or a retessellation — and is
//!    re-derived by `cargo test -p viewer --test all --
//!    review_pick_r2 --nocapture`, which prints the tally.
//! 3. **The aim reaches the graze class**: the count of rays answered
//!    at the aimed vertex is pinned the same way.
//! 4. **No winner's barycentric bound reaches `1`.** A value inside
//!    `[0, 1]` whose rounding interval is that wide covers the range,
//!    and `ray_triangle` refuses it, so zero is the only count this
//!    can have. Asserted rather than pinned — the count is derivable
//!    from the acceptance and a pinned `0` would read as a baseline —
//!    and asserted HERE as well as in `index_memo` because this is
//!    the aim that walks 441 126 rays.
//!
//! What is NOT pinned here, and why: the answers that moved against
//! `main`'s kernel before this unit. That predicate is the copy the
//! unit removed from the tree, and an independent oracle of another
//! algebra differs from it at rounding level in exactly the class
//! being counted, so the moved-answer table is a one-shot measurement
//! (the PR's), not a row.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use editor_core::resolve::{crossing, ray_triangle};
use viewer::pickindex::PickIndex;

use crate::common::corpus_pick::{FlatReference, over_every_landing, too_wide, wide_aim};
use crate::fixture::pick::det_and_conditioning;

#[derive(Default)]
struct Tally {
    rays: usize,
    grazes: usize,
    refused_candidates: usize,
    rays_with_a_refusal: usize,
    genuine_refused: Vec<String>,
    /// Winners whose barycentric bounds are not all below `1`. A
    /// value inside `[0, 1]` with a bound that wide covers the range
    /// and the exact test refuses it, so this is empty by
    /// construction — over the WIDE aim as well as `index_memo`'s,
    /// which is the point of asserting it in both places.
    wide_winners: Vec<String>,
    /// The best-conditioned candidate refused at the determinant —
    /// the class the mechanism refuses, at its edge (reported, not
    /// pinned).
    worst_refused_conditioning: f64,
}

/// The pinned tally over the aim below (docs: re-derive with
/// `--nocapture`).
///
/// Last moved when validation began keeping each loop's authored start
/// (`profile` README V3): from `(442782, 141992, 20016, 10536)`. The
/// rays are unchanged, because the point sets are. Per document, the
/// move is entirely three documents' triangulations, whose loops start
/// at another vertex, so the caps fan and the walls seam from there:
/// - `cut_cylinder` takes 7236 fewer determinant refusals (3660 rays)
///   and 63 fewer grazes, over its open and edited states.
/// - `gallery_ring` gains 36 grazes and 6 refusals.
/// - `boss_union` gains 18 grazes.
///
/// No genuine crossing is refused either way.
const PINNED: (usize, usize, usize, usize) = (442_782, 141_983, 12_786, 6_882);

fn sweep(name: &str, step: &str, index: &PickIndex, tally: &mut Tally) {
    let reference = FlatReference::of(index);
    for aim in wide_aim(index) {
        let (v, dir, reach) = (aim.at, aim.dir, aim.reach);
        tally.rays += 1;
        let ray = aim.ray();
        let mut best: Option<(f64, Option<[f64; 3]>)> = None;
        let mut refused_here = 0usize;
        for flat in &reference.parts {
            for cand in flat.tree.ray(&ray) {
                let tri = &flat.corners[cand.item];
                let cross = crossing(&ray, tri);
                if cross.is_none() {
                    let (det, cond) = det_and_conditioning(&ray, tri);
                    if det != 0.0 {
                        refused_here += 1;
                        tally.worst_refused_conditioning =
                            tally.worst_refused_conditioning.max(cond);
                    }
                    if cond >= 1e-12 {
                        tally.genuine_refused.push(format!(
                            "{name} after {step}: {dir:?} through {v:?}: det {det:e} \
                             (conditioning {cond:e}) refused on {tri:?}"
                        ));
                    }
                }
                if let Some(span) = ray_triangle(&ray, tri)
                    && best.is_none_or(|(b, _)| span.t < b)
                {
                    best = Some((span.t, too_wide(&ray, tri)));
                }
            }
        }
        tally.refused_candidates += refused_here;
        if refused_here > 0 {
            tally.rays_with_a_refusal += 1;
        }
        if let Some((t, wide)) = best {
            if (t - reach).abs() < 1e-9 {
                tally.grazes += 1;
            }
            if let Some(bounds) = wide {
                tally.wide_winners.push(format!(
                    "{name} after {step}: {dir:?} through {v:?} at reach {reach}: \
                     winner at t {t} with bounds {bounds:?}"
                ));
            }
        }
    }
}

#[test]
fn the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus() {
    let mut tally = Tally::default();
    over_every_landing(|name, step, index, _| sweep(name, step, index, &mut tally));
    let counts = (
        tally.rays,
        tally.grazes,
        tally.refused_candidates,
        tally.rays_with_a_refusal,
    );
    println!(
        "# review_pick_r2 tally (rays, answered at the aimed vertex, candidates refused at the \
         determinant, rays with a refusal): {counts:?}; best conditioning refused {:e}",
        tally.worst_refused_conditioning
    );
    assert!(
        tally.wide_winners.is_empty(),
        "{} winners over the wide aim carry a barycentric bound of 1 or more, which covers the \
         admissible range and the exact test refuses:\n{}",
        tally.wide_winners.len(),
        tally.wide_winners.join("\n")
    );
    assert!(
        tally.genuine_refused.is_empty(),
        "{} candidates with a genuine determinant refused at the certification:\n{}",
        tally.genuine_refused.len(),
        tally.genuine_refused.join("\n")
    );
    assert_eq!(
        counts, PINNED,
        "the tally over the corpus moved from its pin; if the corpus or the certification \
         changed on purpose, re-derive with --nocapture and re-pin"
    );
}
