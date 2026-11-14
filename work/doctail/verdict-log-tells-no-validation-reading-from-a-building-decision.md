---
id: verdict-log-tells-no-validation-reading-from-a-building-decision
kind: issue
title: The resolver's flip lane cannot tell a reading taken off an already-built body (tier 3, the rim's validation) from a decision that built the body, so each validation predicate whose sign counts move under an edit pre-empts the true cause unless it is hand-added to first_cause's pass-over list
status: open
opened: 2026-11-03
priority: P1
cost: M
design: true
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

The blend joins the split rule (ONEDOOR, wedge fold stage 3, 2026-11-13). Its convexity decides which side the ball rolls on and so which body it builds, so it reads the fold through `geom_brep::edge_wedge_over_as` under its own `fillet3_convexity_sign` rather than `DIHEDRAL_SIGN`. Like the split rule, it keeps its own predicate until the decision is marked at its source.
