---
id: a-sliver-arc-beside-a-joinable-vertex-refuses-the-boolean
kind: issue
title: A curved arc a few ε long beside a joinable vertex refuses the whole boolean JoinUndecided, where main built the body
status: open
opened: 2026-10-07
---

## The finding

`joinable` reads the surface pair's dihedral at a valence-2 vertex
levered over the shorter of its two edges' extents
(`crates/topo/src/boolean/edge_join.rs:353`, `classify_dihedral` at
`:367`), so the reading does not depend on which edge the arena lists
first. A curved arc only a few ε long beside an otherwise joinable
vertex shrinks that lever until the wedge reads in the band, and the
whole boolean refuses `BooleanError::JoinUndecided`, where main, which
did not join curved vertices, built the body and left the vertex. The
ruling allows it (a reading in the band refuses typed), and the offer
site says so (`offer_rows::EDGE_JOIN_SITE`). No `ci` row reaches it.

## What it needs

A lever that does not shrink with the sliver: the longer edge's
extent, or the locus's own curvature scale at the vertex, so a short
arc beside a transverse pair still decides. Whichever is chosen must
stay independent of which edge the arena lists first.
