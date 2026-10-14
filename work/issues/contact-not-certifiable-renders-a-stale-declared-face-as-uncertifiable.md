---
id: contact-not-certifiable-renders-a-stale-declared-face-as-uncertifiable
kind: issue
title: topo: ContactRefusal::NotCertifiable renders a stale or surfaceless declared face as a contact that cannot be certified
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## Finding

`ContactRefusal::NotCertifiable` (`crates/topo/src/contact.rs`) renders "this contact cannot be certified ({what}), and a declaration cannot move a configuration into the certifiable set". `boolean/contact_verify.rs` raises it for "the A-side declared face does not resolve" (a stale key) and "... has no surface" (corrupt input), which are not configurations and for which the sentence's advice is wrong.

Same shape as band's `BodyNotIntact` (PR 5021). `contact.rs` has no owning program. Found by that PR's §5 sweep.
