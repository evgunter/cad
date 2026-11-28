---
id: contact-not-certifiable-renders-a-stale-declared-face-as-uncertifiable
kind: issue
title: topo: ContactRefusal::NotCertifiable renders a stale or surfaceless declared face as a contact that cannot be certified
status: open
opened: 2026-10-13
priority: P4
cost: E
---


## Finding

`ContactRefusal::NotCertifiable` (`crates/topo/src/contact.rs`) renders "this contact cannot be certified ({what}), and a declaration cannot move a configuration into the certifiable set". `boolean/contact_verify.rs` raises it for "the A-side declared face does not resolve" (a stale key) and "... has no surface" (corrupt input), which are not configurations and for which the sentence's advice is wrong.

Same shape as band's `BodyNotIntact` (PR 5021). `contact.rs` has no owning program. Found by that PR's §5 sweep.

Moved from `work/issues/` to `work/contacthold/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

Moved from `work/contacthold/` to `work/censusread/` on 2026-11-25, when CONTACTHOLD closed (`docs/doc-ledger/contacthold-leaves-the-tracker.md`); id unchanged. The cleanup re-read the row against main at `6003877ad` and found it live. Re-banded P3 → P4: since PR 6079 retired the declared posture, only the refusal's rendering is left.
