---
id: join-census-blind-spots
kind: issue
title: The join's source censuses and its pre-surgery reads have blind spots no pin covers
status: open
opened: 2026-10-14
priority: P3
cost: E
refs: [boolean-join-resolves-germ-face-surfaces-in-three-closures, boolean-walks-face-to-surface-inline-at-two-dozen-sites]
---

From PR 5061's narrow re-review (verdict MERGE), filed rather than held
for. All in `crates/topo/src/boolean/join.rs` and `offer_rows.rs`.

## What

- **Nothing pins that frames are read before surgery** (re-review
  NOTE-2). `SegmentPair::read` reads every kind arm and one-site loop
  in the matched state, and `segment_arm`'s doc argues why; moving the
  read back after an earlier segment's surgery survives every row (the
  re-review's mutant, and this branch's own: 330 cylinder, ruling, rod
  and seam rows and a one-face-wall probe stayed green). A pin needs a
  pose whose chord end lands outside both walls' vertex ball, so a
  later segment's lever grows.
- **The face-walk census is narrow** (NOTE-3).
  `face_surface_tests::the_join_walks_to_a_faces_surface_in_one_place`
  counts `get_surface(`, `face_surface_linked(` and `carrier_surface(`
  in `join.rs` alone. It cannot see a walk through
  `query::face_surface_kind` / `face_surface_matches`, which return an
  `Option` and so read a face that does not resolve as "no", and it
  covers one file.
- **The site census keys `Coincide::Section` at its constructor**
  (NOTE-5). `every_site_names_the_decision_it_raises` counts mentions by
  enclosing function, so `FrameError::section` stands for the nine or
  so frame arms that call it; a call site raising the wrong reading
  through it is invisible there.
- **One-site records are classified twice** (`bool_connect`'s pre-loop
  count). `wrap_site_segments` runs `one_site` over every open record
  in `join_segments`, and `bool_connect` runs it again over the records
  no segment took, to count the unjoined loops. Carrying the count
  forward means `wrap_site_segments` and `join_segments` return it and
  both callers (`bool_connect`, `segment_sites`) take it: four
  functions, not one.

## Direction

The first wants the pose above; the other three are census and
plumbing work in the join's own files.


## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEJOIN (`work/honejoin/`) by `git mv`, id and body unchanged: whose charter is the curved and chord join's residue. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
