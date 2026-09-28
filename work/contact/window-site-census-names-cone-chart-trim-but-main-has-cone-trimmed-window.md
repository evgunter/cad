---
id: window-site-census-names-cone-chart-trim-but-main-has-cone-trimmed-window
kind: issue
title: Main is red: wall_section_rows' window-site census lists cone_chart_trim, but solid_contain.rs names the site cone_trimmed_window
status: open
opened: 2026-09-28
priority: P1
cost: E
---


## What

`cargo test -p topo --lib` fails on `origin/main` at `435247c3a` in
`boolean::solid_contain::wall_section_rows::the_window_construction_sites_are_the_ones_listed`
(`crates/topo/src/boolean/wall_section_rows.rs`).

The census collects the cosine-window construction's sites from
`solid_contain.rs` by their marks. It finds `cone_trimmed_window`. The
expected list names `cone_chart_trim`, and that name is not in the
source. Every other site agrees.

This looks like two merges crossing: `#3331` (CONTACT-3's shared-lexer
census) and the GERM cone containment unit, which renamed or added the
cone window site. The row is red on main itself, and the GERM
interior-oval stopgap PR does not touch either file. Found by that PR's
local gate.

## Fix

Name the site the source has in the census list, or rename the
function back, whichever the cone unit meant. Say in the fix which one
it was.
