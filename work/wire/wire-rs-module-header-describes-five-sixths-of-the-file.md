---
id: wire-rs-module-header-describes-five-sixths-of-the-file
kind: issue
title: eval/wire.rs's header says each F4 node maps to an existing kernel op, and a 720-line union-declaration-routing subsystem inside it maps to none
status: open
opened: 2026-09-11
refs: [2376]
---


## Finding

Found by the style review of PR 2376 under Q8 — the reviewer read
`crates/editor-core/src/eval/wire.rs` end to end, 4731 lines, which
nothing in this project's process otherwise does. Confidence `sure`,
the reviewer's. Accurate at `af8bbca`.

`wire.rs:1-5` says each F4 node maps to an **existing public kernel op**.
`wire.rs:3092-3810` is a ~720-line union-declaration-routing subsystem —
`DeclSite`, `declared_bucket`, `latest_member`, `decl_site`,
`route_declarations`, `look_through_merges`, `step_diagnosis`,
`union_refusal`, `refusal_menu`, `face_name`, `resolve_declarations` —
with its own vocabulary (arrivals, buckets, look-through) that maps to no
kernel op at all. Q5's shape: a claim that was true when written is the
most common kind of false one.

Two accumulation facts the reviewer measured in the same read, recorded
here so they are not re-derived:

- **The file is 41% comment** — 1885 comment lines against 2712 code
  lines, excluding a 365-line inline test module. `unit` at `:752-758`
  carries ~35 doc lines over 4 lines of body; `mod ladder` at `:2182`
  carries ~90 over ~60. Each is individually defensible, which is the
  point: no diff ever sees the ratio. (`likely` that it is a problem.)
- **`wire_sweep` (`:4340-4366`) exists to fail** — it evaluates two
  structural slots into `_stations`/`_v_degree`, calls `section_of`
  twice into `let _`, and unconditionally returns
  `CurvedSolidFrontier`. Deliberate and documented, and still reads as
  dead on every future encounter. (`likely`.)

## What this row is, and is not

It is **not** "split `wire.rs`". The header is the cheap, certain half:
it makes a claim about the file that the file falsifies, and correcting
it costs a sentence. Whether the declaration-routing subsystem wants its
own module is the open question the corrected header would make visible,
and a lane may answer it either way — including by saying the header
should describe two jobs because the file honestly has two.
