---
id: product-instance-output-body-index-saturates
kind: issue
title: product gather's instance output-body index saturates at u32::MAX instead of refusing
status: open
opened: 2026-09-25
priority: P4
cost: E
---

`product.rs::sources_of`, on its `ValuePayload::Instances` arm, tags
instance `i` with the output-body index
`u32::try_from(i).unwrap_or(u32::MAX)`. Past `u32::MAX` every later
instance would share index `u32::MAX`, so `product_named` would read one
body's rows for all of them without anything saying so.

It cannot be reached at a holdable size, and an upstream refusal
already stands in front of it: `names::emit::name_pattern` keys each
instance's rows through `output_body`, which refuses an index past
`u32::MAX` with `NamingError::Emission`, so a pattern that wide fails
before it is ever a product root. The site still breaks the fail-loud
rule, and it reads as though saturation were the intended answer.

The fix is not a one-liner: `sources_of` answers `Option` (`None` is
"this root denotes no body"), so declining on overflow would be a
silent wrong answer. It wants either a typed refusal through the
gather's own error (`ProductError`), or an explicit statement that the
upstream refusal is the guard, with the narrowing spelled as a checked
conversion that names that invariant.

Found by the second sweep of `naming-index-casts-saturate-silently-at-u32-max`
(pattern `try_from(..).unwrap_or(u32::MAX)` over `crates/editor-core/src`).
