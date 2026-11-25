---
id: a-copy-out-of-a-space-carries-every-outline-in-it
kind: issue
title: a copy out of an unshown space carries every outline in that space, including profiles of another body there
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Reported by the `vgeom/profile-display` lane (PR 7771), not driven.
`sketch::shown_placings` (and `world::showings`, which it is built on)
compose a space's outlines through every copy out of that space, so a
copy of body X out of an unshown space S draws, at the copy's place,
every profile drawn in S, including a profile read only by a body Y
that was not copied. Then an outline appears beside a body that is
not built from it, which is a wrong picture rather than a refusal. It
predates PR 7771.

The question is what carries an outline through a copy: the copy's
body being built from the profile, or the profile merely sharing the
copy's source space. Build the fixture (two bodies in S from two
profiles, one copied into the world) and see what is drawn.
