---
id: readers-of-the-stated-image-lever-match-escalated-alone
kind: issue
title: Readers of a stated image's lever match Escalated{StatedImage} and may miss StatedImageUnsettled, its second error shape
status: open
opened: 2026-11-11
priority: P3
cost: E
---


## Finding

Filed by the orchestrator from PR 6836's second delta review (style, "unsure").

PR 6836 added `CertifyError::StatedImageUnsettled`. It is the lever for a closed-form
envelope that exceeds the band with no miss located (`certify.rs` `stated_refusal`),
and it sits beside `CertifyError::Escalated { check: CertCheck::StatedImage }`, which
is also the lever. So one ending now has two error shapes.

A consumer that matches only `Escalated { StatedImage }` misses the new shape:
- topo `merge_faces::edge_on_faces_reading` matches `Escalated` alone. The review
  judged that it probably cannot reach `Unsettled`, because its Line-on-plane case
  goes through the patch route.

## To do

1. Grep every reader of `CertCheck::StatedImage` and of the two variants.
2. Either make each reader exhaustive over both shapes, or fold them into one shape
   that carries the reason as data.

The same review noted a sibling spelling. "A bound over the span exceeds the band
with no miss located" ends two ways:
- the closed-form envelope ends `StatedImageUnsettled`, the lever;
- the fitted composite's unlocated limbs end `NoGeneralLane{Envelope}`, "not yet".

The doc defends the split, because the composite subdivides and gives up only at its
round limit. Weigh both spellings together when this row is worked.
