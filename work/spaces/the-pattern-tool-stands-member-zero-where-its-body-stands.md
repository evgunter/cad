---
id: the-pattern-tool-stands-member-zero-where-its-body-stands
kind: issue
title: The viewer's unfused pattern tool stands member 0 where its body stands, so the first copy is drawn over the original
status: open
opened: 2026-10-28
priority: P1
cost: M
design: true
refs: [6209]
---

Found in INTENT stage 3 G (PR 5954), disclosed there for review.

**What happens.** The pattern tool's unfused row (`crates/viewer/src/session.rs:3120` `add_pattern`, through `combine::row_steps` at `crates/viewer/src/combine.rs:834`) slides member `k` by `scalar(k)·spacing`. So member 0 stands exactly where the source body is drawn: a body drawn in the world, patterned, shows its first copy on top of the original, and two coincident solids land in the product.

**What it should do.** Decide which reading the tool means. Either the family includes the original (and the original's own world placement should be re-pointed to the family, as the fused tool does), or the family starts one pitch on, `(scalar(k) + 1)·spacing`, as `fixture::linear_on` does.

**Done when.** The decision is recorded in the viewer's README pattern-tool section, with a row in `crates/viewer/tests/combine_ops.rs` asserting no member of an unfused pattern stands where its source is drawn (or that the source's placement reads the family).
