---
id: verdict-log-tells-no-validation-reading-from-a-building-decision
kind: issue
title: The resolver's flip lane cannot tell a reading taken off an already-built body (tier 3, the rim's validation) from a decision that built the body, so each validation predicate whose sign counts move under an edit pre-empts the true cause unless it is hand-added to first_cause's pass-over list
status: closed
closed: 2026-11-18
opened: 2026-11-03
priority: P1
cost: M
branch: doctail/verdict-log-tells-no-validation-reading-from-a-building-decision
pr: 7114
---


Filed by the ONEDOOR orchestrator on 2026-11-03, from the wedge-fold unit (PR 6443, branch `onedoor/wedge-fold`).

Each node's verdict log holds every definite decision made while evaluating the node, tier 3's validation included. `editor_core::resolve`'s `diff_verdicts` compares per-predicate sign counts. Wherever counts move between signs it reports a flip, even when no single decision turned: one instance went and another of the other sign came. `first_cause`'s flip lane runs before the group-resize, structural-parameter and edit lanes, and passes over only a hand-kept list. Today that list is the side reader's family plus, from PR 6443, `geom_brep::DIHEDRAL_SIGN`.

The witness: tier 3 now signs every corner (`dihedral_sign`). Sliding a bar so a through-cut becomes a blind pocket moves 21 stations from Positive to Negative at the cut, with no corner turning. Before the exemption, the resolver named `PredicateFlip(dihedral_sign)` ahead of `GroupResized`/`Upstream` in four rows (`resolve_piece_ladder` ×2, `resolve_upstream_scope` ×2). `material_wedge_side` has the same shape (Aligned/Opposed counts move as seams and cusps trade places); no row reaches it yet.

Two defects in one:
- **A validation reading is not marked as one.** A verdict taken off a body already built changes whether the body is accepted, never which body a name was read off, so it is no candidate cause. The log should carry that fact at the source (where the decision is recorded: tier 3, the rim, the census), so the lane needs no list. A list goes stale on the next validation predicate.
- **A count move is not a flip.** It is reported as one when instances trade signs without any decision turning. It wants either identity-keyed verdicts or a weaker word than "flip".

Both want weighing before a lane builds them, since this touches the resolver's diagnosis contract.

## The list blocks a building reader (ONEDOOR, wedge fold stage 2, 2026-11-12)

The wedge row owes moving `splitting::rules::edge_wedge`'s sign onto `geom_brep::Corner::convexity` (`work/onedoor/the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again.md`). That sign is a building decision: it sends an in-plane edge's entry Above or Below, so it decides which body the split builds. `Corner::convexity` decides under `DIHEDRAL_SIGN`, which `editor_core::resolve::first_cause` passes over, so the move would hide a genuine flip of the split's decision from the flip lane. Stage 2 keeps the split rule's sign on `enters_material` for that reason. The constructors that do read `dihedral_sign` while building (`topo::edge_wedge_description`, read by the extrude and revolve upgrades) describe a crease of either turn alike, so the pass-over still holds for them. Marking the decision at its source, the first defect above, is what frees the split rule to move.

The blend joins the split rule (ONEDOOR, wedge fold stage 3, 2026-11-13). Its convexity decides which side the ball rolls on and so which body it builds, so it reads the fold through `geom_brep::edge_wedge_over_as` with its own `FoldNames`: a corner's sign under `fillet3_convexity_sign` rather than `DIHEDRAL_SIGN`, a knife's end under `fillet3_knife_end` rather than `material_cusp_side`. Like the split rule, it keeps its own predicates until the decision is marked at its source. The fold's class (`dihedral_wedge`) and a tangency's pairing (`material_wedge_side`) also decide which body the blend builds (a crease or a knife gets a band, a seam is left as it is), and the blend reads them under tier 3's shared names: neither may join `first_cause`'s pass-over list while that holds.

## Ruled (Ev, [ev] PR 7076, 2026-11-18)

The merged N5 text in `crates/editor-core/src/names/README.md` ("Diagnosis is computable because …") is the spec: every logged decision says, from where it is taken, whether it is a choice or a check; a flip is the first choice two runs of a node take differently along one recorded path; checks are not read; `resolve/vdiff.rs` holds the comparison.

Ev, on the PR:

> nice, this sounds like a real improvement!
> not sure if the logged decisions should have a tag (or be different enum values?) or if we should store checks and choices separately / otherwise make the distinction even stronger. whatever seems like it makes sense in context

The orchestrator's merge note, on the same PR:

> Thanks, merged. The README text is representation-neutral: it says what each decision *is*, not how it is stored. So the build picks the encoding (a tag, distinct enum arms, or separate storage), whichever makes the distinction hardest to blur. It argues the choice in its PR. It measures which resolver, drive and range rows move before it pins anything.

## Closed

The first-turn reading landed as the merged N5 text states it.

- **The encoding.** Every `geom_core::k_stats::Verdict` carries a `Role` (`Choice` or `Check`), stamped in the same push as its sign from the scope the caller opened (`k_stats::checking`). There is no default and no way to reopen a check scope as a choice. `in_family`, its name prefixing, and `decision::words`' prefix stripping are gone. The symbolic recorder's `Decision::family` became `Decision::role`.
- **Where the checks are declared.** At the at-rest doors' shared bodies: tier 3 (`AtRestBody::validate`), tier 2 (`Replay::of`), and a move's re-ask (`finish::asked_after_isometry`). `relog` keeps roles. `record_at_rest`'s census builds records and `census::flush_at_rest`'s flush rows become `Product::coincidences`, a node output, so both stay choices. The side reader casts its rays and reads its volume fallback as checks, and records which side it read once, as its caller's choice (`k_stats::answered`, `topo::SIDE_AT_INFINITY`). The fold takes the role its reader gives the side (`geom_brep::edge_wedge_over_as`'s `side: Role`): the constructors and the rim routing read only the class, so they record the sign and end as checks, and the constructors get back a sign-erased `geom_brep::WedgeClass`.
- **The reading.** `resolve::vdiff::first_turn` walks the choices in decision order. It names the first choice the two runs take differently, if that is the same predicate with the other sign. `first_cause`'s pass-over list is gone, and the blend's `FoldNames` pool back into `dihedral_sign` and `material_cusp_side`. The drive's `FlipEvidence` and `range::DecisionFlip` read the same function. The ε audit's summary diff reports `SummaryShift`s, not flips.
- **Candidate marks.** Measured, then built. Without them, `m4_pr4_resolve`'s slid union named `plane_eq_parallel` `Zero → Positive`, read off two different face pairs. Each `bvh` query now marks the candidates it hands on (`k_stats::mark_candidates`), and the walk reads them. What remains unmarked is filed as `hand-written-box-filters-steer-the-first-turn-unmarked`.
- **Filed:** `work/doctail/hand-written-box-filters-steer-the-first-turn-unmarked.md`; on ONEDOOR, `the-split-rules-wedge-sign-can-read-the-fold-as-a-choice`.

The rows that moved, and what each named before and after, are in the PR body.
