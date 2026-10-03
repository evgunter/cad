---
id: the-strut-cover-on-cylinder-pairs
kind: issue
title: The strut source of the one-sided cover covers plane x cylinder only; cylinder x cylinder and sphere x cylinder wait for a witness
status: open
opened: 2026-10-02
---


## What

`tangency_certifies_side` (`crates/topo/src/boolean/mod.rs`) is the one
table for which tangencies along a curve give the one-sided cover a
global side. Its seam column admits plane × cylinder, two cylinders,
sphere × cylinder, and sphere or plane × torus. Its strut column (an
operand edge described `TangentIntersection`) admits plane × cylinder
only.

PR 3849's first fix pass widened the strut column to the whole table.
Its delta review restored the old strut table and measured the effect:
zero outcomes changed over 3,992 tests. The reviewer's two probes, a
strut between two cylinder walls and a strut between a sphere and a
cylinder (the one-profile capsule's joint), refuse with and without the
widening. So the widening was unproven and unexercised, and it was
reverted (fix pass 2).

## The work

The rows are true for a strut as for a seam: the table's doc argues each
one, and `the_table_matches_which_carriers_keep_to_one_side` measures
them. What is missing is a fixture whose outcome turns on them. Find
where the two probes stop (if it is a later door, the widening cannot
show until that door moves), build a row whose union needs the strut
cover on cylinder × cylinder or sphere × cylinder, and widen the strut
column with that row as its witness.
