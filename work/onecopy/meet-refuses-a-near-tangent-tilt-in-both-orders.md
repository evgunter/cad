---
id: meet-refuses-a-near-tangent-tilt-in-both-orders
kind: issue
title: meet refuses two conics tilted 1e-6 to 1e-4 off tangency at a shared end, in both orders: an unanchored window that reads off still goes to the polish
status: open
opened: 2026-11-04
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(b) of the decision on that PR. The review of PR 6525 measured it.

## What

Take the band-turn needle's rim pair (`geom-brep` `curve_pair::order_rows::needle_pair`) and turn the near-circle about the shared vertex by about 1e-6 to 1e-4 rad. At band 1e-12/1e-11, `meet` then refuses `Unsettled::Meter` in both orders, on main and on PR 6525 alike. `order_rows::the_needle_spun_about_its_vertex_reads_one_answer_in_either_order` reads tilts up to 1e-8 only, and names this row for the rest.

The mechanism, as the review found it: the tilt splits the height's double root. The second plane root sits beside the joint, in a window that no found meeting anchors. There `pose/window.rs` `Window::holds_only` checks that an arc bounded by no ring reads off throughout, as its module docs require. But it returns `anchored`, so a window turn with no ring anywhere returns `false` even when every piece reads off. The root then goes to the polish. Its basin meets near-parallel tangents and refuses.

The docs, "whether the window holds no meeting but the found joints", read `true` for a window that certifies no meeting at all. The code reads `false`.

## Owed

Decide whether a window that reads off throughout, with no found meeting in it, certifies that its root holds no meeting. If it does, `holds_only` returns that, and the near-tangent tilts decide. Pin them in the spun row.

**A plausible second cause** (the delta review of PR 6525): `transverse`'s `Range` still reads the height's first harmonic charged with the pose's `noise` (`Range::first_harmonic(.., noise)` → `band_arc`, `extreme_sign(row::EXTREME, …, r.noise)`). At a spin-round tilt of 1e-5 it is the likely source of the review's `curve_pair_pose_extreme` escalation. See `a-rounding-charge-through-rounding-charge-escapes-the-bare-epsilon-gate`.

