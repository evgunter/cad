---
id: step-import-reads-its-arc-end-in-a-half-open-window-by-hand
kind: issue
title: step-import's endpoint_params and rim split rename an arc's end into (t0, t0 + τ] by a while-loop, which Period::near about t0 + π cannot reproduce exactly
status: open
opened: 2026-11-03
---


Found by `onecopy/angle-reads`, which was told to retire step-import
`normalize`'s `while t <= t0 { t += τ }` into `Period::near` and stopped
that part, as the tree contradicts the premise.

## The finding

`step-import/src/geometry.rs` `endpoint_params` (the conic arm: `t0`
placed in `[0, τ)` by `if t0 < 0 { t0 += τ }`, then
`while t1 <= t0 { t1 += τ }`) and `step-import/src/normalize.rs`'s rim
split (`while t <= spec.t0 { t += TAU }`, "the same angle convention as
endpoint_params") rename an angle into the half-open window
`(t0, t0 + τ]`, EXACTLY at its lower end: an end that reads `t0` itself
moves a whole turn, so a self-meeting arc is never zero-length.

`Period::near(t, t0 + π)` names the same window only up to the rounding
of the reference `t0 + π`: its mark is decided on `fl(t0 + π) − t`
against `fl(π)`, so an end equal to `t0` can land on `t0` (a zero-length
arc) where `fl(t0 + π) − t0 < π`. The `[0, τ)` start is a lower-closed
window too, which `near(π)`'s upper-closed one would move by a turn on
every seam start (common in STEP), and the writer round-trip is a fixed
point of this convention.

A third site of the same shape: `step-import/src/normalize/singularity.rs`'s
`walked` (`rem_euclid(τ)`, then `+ τ` at or below `lead`: the window
`(lead, lead + τ]`).

## Fix

Decide whether `Period` owes a second rename, "the copy in
`(a, a + P]`", decided on the gap from the window's open end `t − a`
against the marks `0` and `P` rather than about a rounded centre, and
retire both loops (and the `[0, τ)` start) into it; or argue that the
loops are the window's own reading and leave them. The two sites read
`Conic::azimuth(p).near(0)` for the angle itself already.
