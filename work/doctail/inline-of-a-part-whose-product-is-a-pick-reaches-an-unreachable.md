---
id: inline-of-a-part-whose-product-is-a-pick-reaches-an-unreachable
kind: issue
title: inline refuses an instance whose part's product slot reads a pick or a source (UnplaceablePart), where it could place that piece
status: open
opened: 2026-11-17
priority: P3
cost: M
---


## What

Before PR 7087, `inline` (`crates/editor-core/src/refactor.rs`, `inline`) took the part's product as `part.product().filter(|&read| part.operation_of(read).is_some())`. A pick passes that check, because `Doc::operation_of` follows a pick to its placement. `inline` then read the spliced product as `read_map.get(&product)`, which holds only the carried operations' outputs, so it reached `unreachable!("the part's live product … is spliced with its operation")`. The review of PR 7087 reproduced the panic.

PR 7087 makes this a typed refusal. The gate now asks `part.defined_by(read)`, so a part whose product reads no operation's output (a pick of a copy's piece, or a source) refuses `InlineError::UnplaceablePart`. The row is `p2_split::inline_of_a_part_whose_product_is_a_pick_refuses_typed`. It was re-banded from P2 to P3 because the panic is gone; what remains is a capability inline lacks.

## Owed

Inline should place the spliced product as the instance copied it. For a pick, the new `Place`'s source is a source over a pick of the spliced copy by the spliced path (`host_path`), and the instance's bundle sides are authored over that read. A pick of a pick is walked the same way, and a source is read through to the body it reads. `flatten`'s `in_source`/`unpicked` assume the placement's source is a copy or a `⊔`, and owe the same case.

Done when the row above is turned around: a part whose product picks one piece of a copy of a `⊔` is instanced, inlined and flattened, and delivers that piece (an `extent` equality).
