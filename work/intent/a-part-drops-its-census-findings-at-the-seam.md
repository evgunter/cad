---
id: a-part-drops-its-census-findings-at-the-seam
kind: issue
title: A part's census findings do not cross into its parent, which re-finds them inside one copy and refuses them
status: closed
opened: 2026-10-16
closed: 2026-10-19
priority: P2
cost: M
---

A part's product crosses into the document that places it as a `PartValue`,
built in `crates/editor-core/src/eval/parts.rs` from
`product::product_recorded` alone. It carries the mated pairs refused at the
mint (`refused: Arc::new(product.refused_at_rest)`) and the contact records,
but not the child's at-rest census findings (`Product::census`).

So a child whose copies interfere, or which holds a contact its census could
not record (an escalated in-band crossing, a same-key curved conformal patch,
a Zero with no margin), arrives in the parent as unrecorded touches inside
one copy. The parent's gather re-finds them and refuses them unattributed:
`crates/editor-core/src/checks/at_rest.rs` module docs, "Inside one copy's
body an overlap stays the kernel's typed error". That contradicts D10's "nothing
it finds refuses" for the census at rest, and it reports the child's finding
as the parent's.

**Fix shape** (census-over-solids designers, PR 4666): carry the child's
census findings across the seam with the part, re-keyed by the graft as
`refused` already is; report each once, as the child's; match the parent's
re-found in-copy touches against them by solid pair. An in-copy touch that
matches nothing is reported as the copy's door's unrecorded contact. Pairs
whose finding names no entity need
`work/restread/an-escalated-census-finding-names-no-pair.md` to match.

Found by both census-over-solids designers (fork-log row 113 follow-up,
PR 4666, 2026-10-16). Not run; read from `parts.rs` and `checks/at_rest.rs`.

## Closed (2026-10-19)

The part's verdict crosses the seam as a verdict, in the shape the
orchestrator approved.

- **At the seam.** The part store judges the part's census the way the
  part's own gate does (`assembly::seam_verdict`).
  - Overlaps between the part's copies ride `PartValue::interference`.
  - If its gate refuses over a finding that names none of its
    declarations, the whole refusal rides `PartValue::at_rest`, its
    attributed findings included. Both are routed like a carried
    declaration.
  - A refusal whose every finding is tied to one of the part's
    declarations stays out. Those records cross already, and the outer
    census verifies them again under `Attribution::Carried`, so a carried
    decline still reaches `Uncertified`.
  - The pairs of solids the crossing findings are about ride
    `PartValue::census_pairs`. Each graft re-keys them by its solid
    order (`Product::census_pairs`).
- **At the gate.** `checks::at_rest::less_carried` drops every verdict
  about a carried pair. An in-copy verdict that matches nothing is
  refused as before, as the copy's door's own unrecorded contact.
  - Carried overlaps are reported on `Assembly::carried_interference` and
    never refuse.
  - Carried refusals raise `AssemblyError::CarriedAtRest` after
    `CarriedMintRefusal` and before this document's own refusals.
  - Each is reported once per part row, by its first route in gather
    order.
- **Python.** `Assembly.carried_interference`, the `CarriedInterference`
  and `CarriedAtRestFinding` classes, and the `carried_at_rest` tag.
- **Consequence for the scalar set.** `EvalScalar` gains
  `checks::at_rest::LocalizeLane`, because the seam judges overlaps at
  every evaluation scalar. `Sym` and every `Dual` take the default,
  `Unlocalized::Lane`. `e4_dual_door`'s literal records the term.

**The gap this leaves.** A census escalation (`CensusEscalated`), like
any finding that names no pair of solids, names no pair, so the outer census's re-finding of one inside the copy matches
nothing. It is never reported: while a part carries a refusal, the
instantiating gate refuses before its own at-rest stage, and a document
whose gate refuses there crosses no census finding of its own to the
document above it. That holds at any depth (the review's MAJOR, fixed in
the fix pass). It holds until
`work/contacthold/an-escalated-census-finding-names-no-pair.md` makes
every escalation name its pair.

**Not covered.** A part's loose spaces, and a part read one member at a
time. Filed as
`work/intent/a-parts-loose-spaces-and-member-reads-drop-its-census-rows.md`.
The reviews' untaken style findings (one carried-row type, the two
homes of `finding_solids`, a carried overlap with no sentence, the
seam's cancel point) are
`work/intent/carried-census-rows-left-unshared.md`.

Rows: `crates/editor-core/tests/intent_part_census_seam.rs`, plus the
Python row in `crates/pncad-py/tests/test_interference.py`.

