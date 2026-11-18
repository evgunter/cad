---
id: a-parts-placed-copy-past-the-box-is-said-as-a-kernel-defect
kind: issue
title: A part whose product grafts a copy past the session box is said as a kernel defect (ProductErrorKind::Frame routes to KernelDefect)
status: open
opened: 2026-11-18
priority: P2
cost: E
---


`crates/editor-core/src/eval/parts.rs` `product_recourse` (`:567-585`) routes `ProductErrorKind::Frame` to `ProductRecourse::KernelDefect`: "a class a nested evaluation cannot reach".

The gather raises `ProductError::Frame` (`crates/editor-core/src/product.rs`, `gather_sources`) in two cases:
- a source's relative motion overflows;
- a source's image in the product's frame reaches past the session box.

The second is user-reachable. A part whose product reads a family of two members 2 km apart reaches it (`product_finishes_once::a_member_past_the_box_refuses_the_product_at_its_source`). An instance of such a part is then said with the kernel-defect recourse, though the recourse is the user's: bring the copy inside the box.

This is main's behaviour as well: before PR 7099 the gather's `read_in` raised the same `Frame` on the same refusal. Fix: route `Frame` by its `kind`, so that `NotFinished` holding `OutsideSessionBox` goes to `Carried` (the finding states the recourse), and an overflowing motion stays `KernelDefect`. Pin it with an instance of that part.

`ProductInvalid` → `Carried` is the honest class as it stands. Since PR 7099 the aggregate's finish can refuse an image whose rounding under its motion tips a fact its source held within the band, and the findings name what to change. (Found by PR 7099's FULL review, item 7.)
