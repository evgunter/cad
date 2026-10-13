---
id: general-pose-cone-fuzz-row-fails-on-parallel-cone-pairs
kind: issue
title: topo: the_general_pose_cone_rows_agree_with_the_traced_section fails at seed 0x924171afe325def2 on two parallel-cone pairs (traced [Null], classified [Null, Null])
status: closed
pr: 4886
branch: germ/cone-pair-fuzz-mismatches
opened: 2026-10-10
closed: 2026-10-12
---

(Filed by the ENCL orchestrator from hosted CI on PR 4538, which does not touch section_cert.)

## What

On main (reproduced at `ad52b377c0`), `topo` `boolean::section_cert::cone_pair_search::the_general_pose_cone_rows_agree_with_the_traced_section` (`crates/topo/src/boolean/section_cert/cone_pair_search.rs:714`) fails with `CAD_FUZZ_SEED=0x924171afe325def2 CAD_FUZZ_EFFORT=1`:

> 2 mismatches … parallel cone: cone apex (-1.2242, 2.6909, 2.2698) axis (-0.7767, 0.6257, 0.0726) α 0.97804 scale 3.4804; parallel cone apex (3.1464, 3.8911, -1.0673) same axis α 0.05472, margin None: on the cone's chart traced [Null], classified [Null, Null]

The hosted run is job 114293690503 (run 38079214860). The row draws a fresh seed per run, so it fails only on some seeds. That makes it a P1: a red can land on any PR that runs topo. The row arrived with PR 4522 (cone pairs in general pose).

## Repair shape

Reproduce it with the seed. The classifier gives two Null components where the trace gives one. Find out which side is wrong for parallel cones (coaxial-direction, distinct apex), and fix that side. Then pin the seed's pair as a fixed row.

## More seeds, on more arms (2026-10-11)

A hunt of 200 fresh seeds on `cleave/section-cert-axis-pose` (PR 4657)
failed twice, and a third seed failed during that PR's review mutation
runs; each reproduces with `CAD_FUZZ_EFFORT=1`. The failing poses are a
tilted cone pair and an oblique cylinder, which `axis_pose` routes to
the ruling charts exactly as main does, and a parallel cone pair, the
same shape as above. So the arm at fault is not the parallel one alone:

- `0xc70ed950ae541691`, **parallel cone**: on the partner's chart
  traced [Null], classified [Null, Null], and a Null part's witness on
  no traced component.
- `0x36a100199db4330e`, **tilted cone** (apex `(-0.0374, 0.7404,
  -0.8717)`, α `0.9444` against apex `(-0.8757, -0.2863, 0.5784)`,
  α `0.0845`, the axes about 28° apart): on the cone's chart traced
  [Null], classified [Null, Null].
- `0x8cb02b1f1a051f77`, **oblique cylinder** (a cone of α `0.076` at
  scale 114 against a wall of radius `14.14`): on the partner's chart a
  Null part's witness on no traced component.

About 1% of fresh seeds fail, so a red can land on any PR that runs this
row.

## Another seed, another arm (2026-10-12)

Hosted CI on PR 4897 (`pcert/mint-routes-spiric-nofitted`, which does
not touch `section_cert`), slow-set step, failed on a fourth pose. It
reproduces identically on main `1f7ef8844e`, freshly compiled in its own
target dir:

- `0x6e080f554162efc0` (`CAD_FUZZ_EFFORT=1`), **cylinder near the
  apex**: a cone with apex `(45.4816, -30.8284, -11.2321)`, axis
  `(-0.3959, 0.9173, 0.0436)`, α `0.0573` and scale `62.14`, against a
  cylinder with origin `(-16.4605, -11.0784, -27.7851)`, direction
  `(0.4818, 0.7208, 0.4984)`, r `66.18` and margin `3.4743`. On the
  partner's chart it traced [Null] and classified [Null, Null], and the
  witness `(47.2421, -34.3392, -11.4032)` of a Null part lies on no
  traced component.
