---
id: shell-opens-a-wrapped-face-only-as-two-branches-meeting-at-a-pole
kind: issue
title: Shell opens a face whose chart wraps its period only as two branches whose seams meet at a pole, so a one-band cap refuses
status: closed
opened: 2026-11-22
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: M
closed: 2026-11-26
pr: 7435
---

Found by the LATHE one-band lane (2026-11-22, PR 7414), measured on its
branch `lathe/full-revolve-one-band`.

## What

Shell's open-face rim reader (`crates/topo/src/shell.rs`, at the
"two branches whose seams meet at one pole" refusal) accepts a face whose
designated chart wraps its period only as TWO seams meeting at a pole. Its
premise reads: "one seam ending at a pole would leave the pole a strut tip,
which tier 2 refuses at rest, so no finished operand carries it". Tier 2
now admits a wrap edge's tip, and a full revolve's curved wall is one face
whose one seam ends at the pole. So a capped cup or vessel opened at its
cap refuses: "the rim of an open face is not a shape the shell op can
build: the designated chart wraps its period, but it is not two branches
whose seams meet at one pole".

Failing on the branch: `sweep` `verbs_shell::the_record_is_a_function_of_the_construction`,
`verbs_shell::the_record_reads_against_the_body_on_every_arm` (the capped
cup); `editor-core` `lib_g17_shell_node::the_capped_vessel_opens_its_cap_into_a_seamed_band`,
and `lib_g17_shell_node::the_refusals_are_typed_and_their_texts_pinned` (the
belly's half-chart gate now meets `TogetherAxialEdge` first).

## Ground

`crates/topo/src/shell.rs`: shell. Claimed by LATHE.

The split door's sphere refusal is not this row: it gates on a sphere
face's box, which a one-band face fills. That is a fixture re-baseline in
PR 7414.
