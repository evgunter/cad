---
id: python-census-finding-rows-read-a-rest-contact-since-the-planar-face-pair-census
kind: issue
title: four test_validate rows read a Rest contact where they expect vertex_on_face and edge_face_overlap, since PR 5605
status: open
opened: 2026-10-23
priority: P1
cost: E
---


Found by the python suite of PR 5628's gate (run 40030217793). It is red
on origin/main at `52a71ab1` too: a dev wheel built from a clean main
worktree fails the same four rows. The PR gate skips the python suite
on main, so main's CI does not show it. Every row lives in
`crates/pncad-py/tests/test_validate.py`, `TestTheRefusalsShape`:

- `test_the_census_findings_arrive_as_prose`: "vertex" is not in the
  message, which now reads "two faces lying against each other (a Rest
  contact) at their shared region is an unrecorded contact".
- `test_the_per_arm_words_cross_and_the_census_says_so`: the set of
  contact kinds is `{conformal_patch}` where the row expects
  `{edge_face_overlap, vertex_on_face}`.
- `test_each_finding_s_word_is_the_message_s_own (contact='conformal_patch')`:
  "conformal" is not in the message.
- `test_a_finding_is_a_frozen_value_that_compares_structurally`: 1 finding,
  where 2 are expected.

Cause, read from the log: PR 5605 (`intent/census-planar-face-pair`)
merged after my last green local run, and it is the one census change
in that window. The census now reads the fixture's planar face pair as
one Rest contact, where it used to read a vertex-on-face and an
edge-face overlap. Decide whether the fixture's new reading is right
(it then needs a fixture with the old findings, or new expectations),
or whether the census regressed.
