---
id: a-levers-length-used-outside-a-margin-has-no-gate
kind: issue
title: A lever's length used outside a margin has no gate
status: open
opened: 2026-11-23
priority: P3
cost: M
---

`Lever::length` is "the exit, not an entrance" (`geom_core::Lever`'s doc), but nothing counts where its result goes. A text gate cannot hold the class. `rg '\.length\(\)' crates/*/src` matches 260 lines, and a typed read matches only 90 of them. That read marks `Lever::length` `#[deprecated]` and builds `cargo build --workspace --exclude viewer --lib --bins`, and every `use of deprecated method` warning is one call. The other 170 are other types' `length` methods. Even the 90 cannot be split by text into a margin's read and a geometry read: a length bound to a local, or handed to a parameter named `reach`, `extent` or `arm`, reads the same either way.

What would hold it exactly is a typed count: clippy's `disallowed-methods` on `geom_core::Lever::length`, with an `#[expect(clippy::disallowed_methods, reason = …)]` at each of the ~90 references that stays, or the deprecation build above run as a gate against a pinned per-file count. Both are more than a few lines, and the second needs a build in the lint job, so the `sizes-are-read-back-out-of-extent-levers` unit filed this rather than adding a gate. `scripts/gates/lever-sources.sh` and `scripts/gates/lever-doors.sh` are the home such a count would sit beside.

**A second blind spot, which neither count sees.** A length that was a lever's but is held as a bare `T` reads like any other number. One example is `RuledFaceRegion::scale`, documented as "the lever of a face's crossings" and read as the radius of a ball about the cone's apex by `solid_contain`'s trimmable cone arm (converted by PR 7628 to the face's apex ball). Others are a field or parameter named `reach`, `arm`, `extent`, `lever` or `scale`, or the result of a method documented as a lever, used as a radius, a window or a point's offset. A typed count of `Lever::length` stops at the first `let` or field that drops the type. The sites PR 7628 read for this shape were geometry reads held bare (`splitting::pieced_arc::PieceBall::reach`, a ball's radius off the piece's net; `curve_pair::span`'s `past_by` `reach`, a pose's closed-form bound; `chart_singularity`'s sampled `reach`, speed × half-span; `cell_region`, `pair_section::gap`, `census` near 4402 and `census::point`), none a lever. Only keeping the type, a `Lever` or a region, to the read makes the class countable.
