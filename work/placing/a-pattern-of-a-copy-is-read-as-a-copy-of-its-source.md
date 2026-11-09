---
id: a-pattern-of-a-copy-is-read-as-a-copy-of-its-source
kind: issue
title: A pattern of a copy stands in the copy's space and the gesture vocabulary reads it as a copy of its source, so a pick on its member seats the undrawn copy
status: review
opened: 2026-11-09
priority: P2
cost: M
design: true
pr: 6581
refs: [a-place-into-the-bodys-own-space-is-no-copy-to-the-gesture-vocabulary]
---



Found by the review of PR 6593 (REFS, `viewer-boolean-tool-cannot-pick-a-read`), probed in its fix pass.

**What.** `SessionOp::AddPattern { input: <a world copy> }` lands the pattern's repeating `Place` in the world, reading the copy's output. The world is always a copy space (`world::copy_spaces`), so `world::copy_source(pattern)` answers `Some(Source { operation: <the copy>, member: false, .. })`: the gesture vocabulary reads the family as a copy of the world copy it replicates. A viewport pick on the pattern's member 1 then seats the copy. A boolean authors `Union([Node(copy), other])`, which is the undrawn copy (the pattern took it over), so the union silently takes member 0's geometry. The same holds at every seat that reads through `world::seat_read`.

**Reach.** The GUI's pattern tool does not author this document: a pick on the copy seats the body it places, so the pattern reads the body. Only the session op or the API reaches it. Pinned by `combine_ops::a_pattern_of_a_world_copy_is_read_as_a_copy_of_its_source`.

**Why here, not in PR 6593.** The narrow fix is that `copy_source` answers `None` for a placement that `world::repeats`: a pattern's family is a relation, not a copy of one body. But the README, as of PR 6580 (*A new body lands unplaced*), now says every placement is a copy to the viewer, a pattern's included, and that a pick on a copy seats that copy. `copy_spaces`' rule is this program's open row `a-place-into-the-bodys-own-space-is-no-copy-to-the-gesture-vocabulary`. How a repeating placement reads is part of that ruling's implementation, not a local patch.

**Done when** a pick on a member of a pattern of a copy seats that member of the pattern (or whatever the ruling makes it), and the pinned row says so.

## Built in PR 6581 (2026-11-09)

PR 6581's seats read no copy through to its source (`world::seat_read`), so a pick on the pattern's member 1 seats the pattern, never the copy it replicates. Here the family is read whole, because the product reads it (`world::is_seen`); a family nothing reads along a seen path gives `pattern[1]` instead (`world::seat_member`). The pinned row now says so, as does the pattern tool picked on the copy, which seats the copy.
