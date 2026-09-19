---
id: exact-fit-close-skips-the-fillet-tangency-re-read
kind: issue
title: The exact-fit arc close's fillet arc is never re-read for its stored tangency: build() accepts a flattened closing arc and only validation refuses it
status: open
opened: 2026-09-19
---

Filed by the `radius-r1` review lane of PR #2892 (EDIT-RADIUS), which
disclosed this as "Observation, not filed" in its PR body. `work/README.md`
§Rules: a disclosed residue gets its own file at the moment it is
disclosed. This is PATHS's ground (`crates/profile/src/path/family.rs`),
and it predates that unit.

## The finding

`family::resolve_arc_close` (`crates/profile/src/path/family.rs`, the
`else` arm after `if trims.fit_out == Sign::Positive`) is the EXACT-FIT
close: the fillet arc is the whole arrival side and becomes the closing
segment through `core.set_leaving(trims.bulge, FirstSeg::Arc)`. That
site does not go through `Core::record_fillet_arc`
(`crates/profile/src/path.rs`), so the arc is absent from
`Core::fillet_arcs`, and `Core::fillets_carry_their_tangency` — the
re-read `Core::build` runs before every close — never reads its stored
form. The other four fillet-arc emission sites (`emit_fillet_arc`,
`resolve_fillet`'s two arms, `resolve_arc_pending_ray_arrival`'s seam
arm) all record.

Measured on the frozen head of PR #2892 with a mutant that flattens
that one arc — `core.set_leaving(T::zero(), FirstSeg::Arc)` in the
exact-fit arm:

- every close through that arm still SUCCEEDS (`build` returns `Ok`);
  the review probe
  `path_program::r1_an_exact_fit_close_records_its_fillet_at_the_closing_segment`
  reds only on its own bulge assertion, after a successful close;
- the three pre-existing rows that take the arm
  (`cert4r2_e2e::a_true_tangency_classifies_as_an_exact_fit_through_the_public_door`,
  `arc_fillet::fillet_offset_line_circle_trio`,
  `fillet_stored_tangency::an_exact_outgoing_fit_leaves_its_joint_undeclared_and_still_validates`)
  red only at their later `validate` call.

So the seam `junction_check` in that arm does NOT cover the stored
form: it reads `end_ang` off the carrier (`carrier_tangent`), never off
the bulge that was stored. What catches the flattened arc is validation
downstream, with validation's sentence, not the door's
`FilletArcFlattenedInStorage` / `FilletCarrierBelowSceneResolution`
naming the radius.

Two doc sentences are false for this branch as a result:

- `Core::build`'s doc (`path.rs`): "the door's output is a loop whose
  fillet declarations the stored form carries, or it is a refusal".
- `fillets_carry_their_tangency`'s reach doc lists "every door but
  two" at which a fillet's own joint is declared; the exact-fit close is
  a third door, and there the arc is not in the list at all.

## What a taker does

Decide whether the exact-fit close's arc belongs in `fillet_arcs` (route
the arm through `record_fillet_arc`, which now also writes the radius
emission — PR #2892 wrote that emission by hand at this site with
`core.record_radius(meta.bound_at, RadiusRole::Fillet)` precisely
because the arm skips `record_fillet_arc`), or that the door's promise
is scoped and say so at `build`'s doc and the reach list. If the arc
is added, the `PAIRED` debug assertion's seam form
(`leaving == verts.len() - 1`) is the one that applies.

The review probes on branch `review/radius-r1` pin the emission at that
site (`r1_an_exact_fit_close_records_its_fillet_at_the_closing_segment`,
`edit_step_segments::r1_an_exact_fit_closing_fillet_arc_reaches_its_wall`);
a row for the re-read itself would flatten the stored bulge and expect
the DOOR to refuse.
