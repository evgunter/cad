---
id: a-public-rigid-map-hands-back-a-body-past-the-session-box-unasked
kind: issue
title: transform_rigid and mirror_body hand back a moved body past the session box that nobody asks
status: open
opened: 2026-11-21
priority: P3
cost: E
---

Filed by ONEROUND's `oneround/box-certify-ask` (PR 7272), at the orchestrator's ask: is there a production door that brings a body past the session box into being without asking it?

**What.** `topo::transform_rigid` (`crates/topo/src/transform.rs`, a `pub fn` over `map_isometry`) and `topo::mirror::mirror_body` (`pub fn`) map a construction-state body by a rigid motion or a reflection, carrying every edge's certificate, and ask nothing: a brick at the origin carried 1e9 m out comes back `Ok`, its edges certified, and is refused only if and when it meets a finished-body gate (`crates/topo/tests/review_m4_pr2_transform.rs`'s `an_extreme_scale_image_is_the_session_boxs_to_refuse` builds exactly that). The README's session-box bullet (`crates/geom-core/README.md`, "Float rounding") has "the door that stands a body by a map of that body where it stands" ask the box. The finished-body path does (`FinishedBody::moved`, `finish::moved_and_asked`); these two public construction-state maps do not.

**Where it is reached.** No production caller outside `topo` uses either (swept: `grep -rn 'transform_rigid\|mirror_body' crates/*/src`, every hit outside `transform.rs`/`mirror.rs` a decision name, not a call). Inside `topo`, `finish::moved_body` calls `map_isometry` behind the asked door. So it is a public-API hole, not a live production path: a consumer of `topo` who maps a construction body and keeps building on it gets certified edges past the box until a gate reads it. ONEROUND's certification ask (PR 7272) does not reach it: a map carries certificates, it does not re-certify.

**Wanted.** Either the public maps ask the box of the image (as `in_session_box_at` does, off the body unmoved), or they are made crate-private with the asked door the one public way to move a body. `work/onehome/the-doors-that-ask-where-a-body-stands-are-a-hand-kept-class.md` is the same class one layer up.


