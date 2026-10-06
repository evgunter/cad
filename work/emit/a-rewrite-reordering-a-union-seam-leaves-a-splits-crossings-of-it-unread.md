---
id: a-rewrite-reordering-a-union-seam-leaves-a-splits-crossings-of-it-unread
kind: issue
title: A rewrite that reorders a union seam's sides re-reads a boolean's crossings of it but not a Split's: CrossingVertex keeps its sense and rank
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [a-second-crossing-by-one-face-renames-the-first-and-its-pieces]
---


## What

A rewrite of a published name re-reads a crossing's sense and rank where it
reorders the sides of the seam the crossed edge lies on
(`names::canonical`, `RankRule` and `flip_senses`). Both read only a seam
vertex's own head — `Crossing`, `EdgeCrossing`, `Seam` (`seam_pair::head`)
— so a Split's `CrossingVertex { side, edge, sense }` of a union seam edge,
ranked or not, keeps its sense and rank when a re-map reorders that seam's
sides, though both are read along the seam's first side
(`emit_topo::crossed_edge_orientation`). The rank half predates the sense.

## Fix

Read `CrossingVertex`'s `edge` in `RankRule::derive` and `flip_senses` as
the crossed edge, as they read a `Crossing`'s, with a canonical-form row
like `a_crossings_sense_along_a_seam_an_embedded_name_writes_flips_with_its_rank`.
