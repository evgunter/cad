---
id: a-parameter-gap-metered-by-hand-at-a-span-end-has-no-gate
kind: issue
title: A parameter gap metered by hand against a span's end has no gate: its unsound use (a Zero read as at the end) is not a text shape
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
branch: onecopy/edge-ends-turns
---


Found by `onecopy/needle-basin` (PR 6687), whose P0
(`a-plane-crossing-past-the-band-of-a-thin-ellipses-fast-end-read-at-the-end`)
was this shape: `(t − t₀)·meter` at an INF speed, its Zero read as "at
the end".

## Why no text gate can judge it

A parameter gap metered at an inf speed under-reads arc length. Its
Positive is sound ("clear of the end"), and so is an ordering. Its Zero
is not: a Zero spans up to `(speed_max/speed_min)·z` of arc. Text cannot
tell a site that reads the Zero as "at the end" from one that only
accepts the Positive, so a gate on `Margin::metered(a − b, …)` would flag
every sound one too. The one door for a parameter against a span's ends
is `geom_brep::curve_pair::Span::place_param` / `place_param_read` (an
end by its chord; the side at the sup speed).

## The hits (production, `onecopy/edge-ends-turns`)

`grep -rnP "Margin::metered(_sup|_inf)?\(\s*\(?\s*[\w.]*\s*-\s*[\w.]*\s*\)?\s*," crates`, outside tests:
- `topo/src/chart_region.rs:3517`, `shell.rs:3582`, `offset_axial.rs:2783`,
  `boolean/join.rs:2012`, `boolean/trace.rs:348`, `:371`,
  `boolean/ops.rs:7158`, `chord_join.rs:2463`, `pcurves.rs:1910`,
  `:1914`, `:4938`, `:4939`;
- `geom-brep/src/pair_section.rs:2763`, `certify.rs:3979`,
  `pcurve_cache/projected.rs:2963`, `pcurve_cache.rs:8371`, `:8380`.

Routed to `Span` by `onecopy/edge-ends-turns`: `topo/src/split.rs`'s
`split_edge_param_interior` (the Euler split's interiority gate, at the
speed floor). Its Zero refused as `SplitParamNotInterior`, so it was
sound, but it refused a split at a root the plane door now places past
the band by chord.

Not routed: `pair_section.rs` `within_span`. It meters `hi − t` at the
pointwise speed at the witness, which is no inf over `[t, hi]`. It is a
witness reading with its own reach (`section_site_in_span_past`, twice
the zero past the band), not a span's place, so `Span::place_param`
does not answer it.

## What closes it

Each hit read once: an inside-only or ordering read (sound; say so at
the site if it is not evident), or an end read (route to `Span`).
`within_span`'s meter gets a bound over `[t, hi]`.

## Closed by `onecopy/edge-ends-turns`: a site table instead

`scripts/gates/span-end-gap-sites.sh` (its table `span-end-gap-sites.tsv`)
holds every production `Margin::metered(_sup|_inf)?(a − b, …)` whose
first argument has exactly one operand a span end's name, keyed by
(file, function), each row naming its direction argument (`sup`,
`exact-rate`, `inside-only`, `witness`). A new site raises its function
above its row and fails, and is argued in review or routed to `Span`.
Both operands ends is an extent and is not read; a free gap between two
parameters is not read. The 17 hits above were that wider shape. Of
them, four are end gaps on this branch:
- `pcurve_cache.rs` `side_of` ×2: `sup`;
- `chord_join.rs` `chord_spec`: `exact-rate` (a line's length);
- `pair_section.rs` `within_span`: `witness`, still owing its bound.

`split.rs` and the plane door, the two `inside-only` / end-read sites
the P0 found, now read `Span` and are not in the table.
