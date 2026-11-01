---
id: a-declared-contacts-class-admits-continuation
kind: issue
title: DeclaredContact.class is a GlueClass, so a declared contact can claim Continuation, which no contact is
status: open
opened: 2026-10-24
priority: P4
cost: E
---


Found by review 1 of PR 5637 (stage 4 I, `mates-declare-no-contact`).

## What

Stage 4 I retired `ContactClass` (`Rest | Tangent`), the class a
mate declared, and gave `topo::DeclaredContact` (`crates/topo/src/contact.rs:256`)
the boolean glue's class instead: `class: GlueClass`, which is
`Rest | Continuation | Tangent`. The topo README says a continuation
is not a contact (the paragraph under **Continuation**), and every
construction site writes `Rest` or `Tangent`: the census's found pairs
(`census.rs` `report_found_pairs`, `sweep_conformal_patches`), its
tangent pairs (`same_solid_tangent_pair`, `confirm_face_pair`), and
the Rest door (`rest_door_one`). The type admits a value no site
writes, and the readers (`boolean/mod.rs:4640`, `validate.rs:2494`,
`:4291`) word whatever class they are handed.

## The question

Which is right?

- Narrow the field to a two-variant contact class. That re-introduces
  a `Rest | Tangent` enum one stage after the spec retired `ContactClass`
  as a mate's word, so it needs a ruling that a kernel record's class
  is not the retired mate class.
- Keep `GlueClass` and refuse `Continuation` where a declared contact
  is read.
- Leave it, since stage 5 C retires the declared-pair readers this
  field serves.

Not decided in PR 5637: the spec does not settle it.
