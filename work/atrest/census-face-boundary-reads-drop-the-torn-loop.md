---
id: census-face-boundary-reads-drop-the-torn-loop
kind: issue
title: Two census face-boundary reads map unchecked_face_boundary's torn LoopKey to a keyless Unbuilt/Corrupt, and corner_cause drops the sector face's key
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Found by the sweep of `contain-error-drops-the-loop-its-carrier-named`
(INSIDE, lane `inside-contain-small`). `crates/topo/src/census.rs` is
claimed by several programs; filed on ATREST as the best charter fit
(`work/README.md`, "Filing and ownership"). Not verified: whether the
reader of these refusals is handed the entity some other way (the touch
site's subject); the conversions themselves are as stated.

## The findings

- `census.rs`, `face_cone` (`FaceContainment::OnVertex` arm): each
  `body.unchecked_face_boundary(face)` read is mapped with
  `.map_err(|_| Unread::Unbuilt)`. The iterator's error is the torn
  loop's `LoopKey`; `Unread::Unbuilt` carries none.
- `census.rs`, `tangent_sectors`: the same read, mapped with
  `.map_err(|_| TouchUnread::Corrupt)`; `TouchUnread::Corrupt` carries
  none.
- `census.rs`, `corner_cause`: `SectorFaceError::{ApexUndecided,
  Unsupported, Unplaced, Crease}` each carry the corner `face` (and
  most a `kind`), matched with `{ .. }` into `TouchUnread`s that carry
  no face.

## The repair

Where the census's unread causes are meant to be subjectless (the touch
site is the subject), say so on `TouchUnread`/`Unread` and close this;
otherwise carry the loop or face and render it.

## More sites (2026-11-25, PR 7748 review)

The same `unchecked_face_boundary` read, its torn `LoopKey` mapped to a
keyless `TouchUnread::Corrupt`:

- `census.rs`, `snapshot`: `Err(_) => { walked = Err(TouchUnread::Corrupt); … }`.
- `census.rs`, `chord_on_edge` (the review cited `half_edge_tangent`;
  the read is in `chord_on_edge` beside it):
  `read.map_err(|_| TouchUnread::Corrupt)`.
- `census.rs`, `round_face`: `read.map_err(|_| TouchUnread::Corrupt)`.
- `link.rs`, `of_body_face` and `cleared`: `read.map_err(|_| TouchUnread::Corrupt)`.
  `link.rs` is INSIDE's ground by `work.py territory`, not shared. It is
  listed here so the census's one cause type is decided in one place;
  move it if the owner would rather.
