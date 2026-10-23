---
id: the-contact-recourse-names-a-class-nobody-can-declare
kind: issue
title: CONTACT_RECOURSE tells the author to declare a contact class, and since stage 4 I no channel declares one
status: open
opened: 2026-10-23
priority: P3
cost: E
---


Found by stage 4 I (`mates-declare-no-contact`).

`crates/topo/src/contact.rs:105` `CONTACT_RECOURSE` ("declare the named
contact class, or move the geometry") ends every `UnrecordedContact`
finding (`CONTACT_RECOURSE_MARKED`, `contact.rs:110`). The boolean
declare channel retired with stage 4 F and a mate's class with stage 4
I, so nothing an author writes declares a contact class; the first half
of the recourse points at a door that no longer exists. The Python
suite pins the sentence verbatim
(`crates/pncad-py/tests/test_validate.py`,
`test_the_message_is_the_kernel_s_own_diagnosis`), and so does
`crates/pncad-py/src/tests.rs`
`the_census_findings_read_as_prose_by_this_crate_s_own_rule`.

Where an unrecorded contact still refuses (inside one op's result, D1
(iii)) the recourse is the op's: move the geometry, or say which glue
the op should have recorded. The sentence and its pins move together.

`StaleContactDeclaration`'s recourse (`crates/topo/src/validate.rs`,
the `StaleContactDeclaration` arm of `ValidationError`'s `Display`)
said "the mate named with it, where one is"; stage 4 I dropped that
clause, since no mate is named with a record any more.
