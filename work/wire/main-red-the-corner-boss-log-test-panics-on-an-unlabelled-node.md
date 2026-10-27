---
id: main-red-the-corner-boss-log-test-panics-on-an-unlabelled-node
kind: issue
title: main red: emit_union_borders::the_corner_boss_logs_its_corner_alike_in_every_order_and_every_pair_boolean panics 'no entry found for key' in its label helper
status: closed
opened: 2026-10-27
priority: P1
cost: E
closed: 2026-10-27
---


Reported by `wire-meetrow` (PR 5798, 2026-10-27). It fails identically on main and on that PR's head, so it is not that PR's.

The test panics at `crates/editor-core/tests/emit_union_borders.rs:680`, in its row-label helper's `labels[&n.node]`, with "no entry found for key". A published row now names a node the test's `labels` map does not hold. That may be a fold step's own node, or a node that a re-cite or PR 5753's ring-tie spelling now cites.

The test's pinning row, `cleave/a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold`, is closed.

Root-cause it. Either the row names a node it should not, which is a real defect, or the helper's labels are incomplete, which is a test fix. Keep what the test asserts.

Closed by PR 5820 (2026-10-27). The test helper was at fault, not the kernel: PR 5636 made placed copies carry their source's names and fixed the face helper, while PR 5753 brought in an edge helper still keyed by `labels[&n.node]`. Both helpers now share `holder`, and no assertion changed.
