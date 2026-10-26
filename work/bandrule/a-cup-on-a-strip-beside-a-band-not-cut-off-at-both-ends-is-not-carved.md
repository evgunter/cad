---
id: a-cup-on-a-strip-beside-a-band-not-cut-off-at-both-ends-is-not-carved
kind: issue
title: blend: a cup on a support's strip beside a band that is not one link cut off at both ends (a run of joined links, a mitre, a turn, a corner) refuses at the screen, though S − R / S ∪ F takes it
status: open
opened: 2026-10-15
priority: P3
cost: M
---

## Finding

The strip half of step 3 of `a-concave-fill-and-a-whole-shell-inside-a-bands-region`
takes a dimple (convex) or a boss (concave) on a support's strip only
where the band is a whole open chain of one plane–plane link, cut off
at both ends. Elsewhere the ring stays a boundary feature of the
support, and refuses where it did:

- the clearance screen stands down only beside a one-link open chain
  (`crates/sweep/src/blend/battery.rs`, `strip_decided`), so a ring in
  the strip of a run of joined links, or of a band ending at a mitre, a
  turn or a corner patch, refuses `FaceClearanceUncertified` there
  (`band_one_cycle_members::cups_beside_a_bore_under_two_bands_and_by_a_mitre_go`
  pins one: a cup on a mitre's own end, its mouth on the other band's
  strip);
- the ring carry-through pass reads a ring against the strip only
  where two cut-off slivers carry the link's crease
  (`crates/sweep/src/blend/surgery.rs`, `ring_clearance_pass`, arm (a),
  `cut_off_at_both_ends`).

What it needs: the strip's extent along the edge at a joint, a mitre,
a turn's overrun and a corner patch (each ends the strip at its own
foot or trimline, not at an end face the support's own boundary
follows), read by the strip meter as the cap meter reads a sliver; the
reach's core is already capped there, so `reach::prism_inside`'s strip
reading may need nothing new.
