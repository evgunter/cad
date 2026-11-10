---
id: overlap-refusal-unread-is-string-typed-across-classes
kind: issue
title: OverlapRefusal::Unread is one string-typed variant over refusals of different classes
status: closed
opened: 2026-11-06
priority: P3
cost: E
closed: 2026-11-06
pr: 6576
---


Found ruling `OverlapRefusal`'s standing (`onecopy/refusal-standing`).

## What

`boolean::overlap::OverlapRefusal::Unread { what: &'static str }` ("a
reading the door has no verdict for") carries about a dozen cases as
strings, minted across `boolean/overlap.rs`. `OverlapRefusal::refusal`
classes every one as a limit. They are not one class:
- **An abort candidate:** "a point of one face lies definitely off the
  other's carrier, which a verdict …" (in `place`). Its own words make it
  a contradiction between two readings, not a limit.
- "the two faces' normals stand square to each other, so no one carrier
  holds both" (in `one_sense`) reads the door's premise. It is returned
  by `?` and is a refusal of the whole query.
- The rest are limits: a boundary edge with no certified curve, a face
  holding a lone vertex, an edge that does not resolve, a corner bound
  with no certified curve, and so on.

A string-typed variant whose cases differ in class cannot state its
standing beside each case. So the next case added inherits "limit"
whatever it is.

## What closing it takes

Type `Unread`'s cases (an enum of the readings), each with its standing
beside it, so that `OverlapRefusal::refusal` delegates to it. The
contradiction case becomes an abort. Measure whether any `ci` row
reaches it kept beside a later witness, since that is where the outcome
would move.
