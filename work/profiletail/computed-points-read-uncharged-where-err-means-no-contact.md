---
id: computed-points-read-uncharged-where-err-means-no-contact
kind: issue
title: seg reads computed points uncharged in four places where a definite reading means no contact; binds only for a tiny circle against a far segment end (probe first)
status: open
opened: 2026-10-21
priority: P3
cost: M
---


Found by PR 5552's FULL review (NOTE, likely).

**What.** PR 5552 charged the computed points its radial certificates
read (`circle_foot_past_segment`, `circle_extreme_past_arc`). Four other
places in `crates/profile/src/seg.rs` read a computed point uncharged in
the same role, where a definite reading answers no contact:
- `arc_clear_of_carrier` (`seg.rs:1739`, from #5481) reads `arc_span` at
  the computed foot;
- `touches_from` (`seg.rs:2329`);
- `end_in_stretch` (`seg.rs:2219`);
- `ray_crossings` (`seg.rs:2369`).

The crossings and tangency points `line_arc` and `arc_arc` compute are
read the same way.

**Where it would bind.** By the review's `√(2rKε)` argument, only with a
tiny circle (`r ≲ 10⁻⁶` m) against a segment whose far end is `≳ 10⁴` m
out. That is the regime where PR 5552's foot charge proved load-bearing:
at a sixteenth of it, the review's exact oracle found 26 unsound fires
per 100k draws.

**Next.** Probe first: an exact oracle sweep over that regime for each
site, as the review ran for PR 5552. Charge only the sites a witness
reaches. No witness yet.

## Re-homed at PATHS's close (2026-10-21)

Filed on PATHS's slate on the day it closed (PR 5552's review) and moved to PROFILETAIL (`work/profiletail/`) with the merge that closes PATHS. PROFILEREST holds validation's other reads but was at budget. The item's id, content and history are unchanged.
