---
id: range-admission-says-a-defined-variable-is-absent
kind: issue
title: range admission answers a defined variable and an absent one with the same NotAContinuousParam sentence
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by OFFER's `certify-affordance-on-the-bounds-panel`, which makes
the kernel's admission door (`editor_core::range::admit`) the one door
that greys the viewer's `range?`.

`range.rs`'s `axis_of` answers `RangeRefusal::NotAContinuousParam` for
every `RangeField::Param` whose `doc.free(var)` is not
`FreeVar::Continuous`: a variable the document does not declare, a
`Count`, and a variable a formula DEFINES all read "X is not a
continuous parameter of this document — there is no axis to certify
over". The three have different recourses. A defined variable's is the
one `RangeRefusal::SlotIsDefined` already says for a slot ("certify the
variables that formula reads instead"); an absent one has nothing to
certify. The sampling probe the viewer used to run told them apart
(`Refusal::VariableIsDefined` against `Refusal::NoSuchVariable`); with
the probe gone, the row that pinned the distinction
(`crates/viewer/tests/gui_variables.rs`,
`range_on_a_defined_or_absent_variable_is_refused_by_the_kernels_admission`)
now pins that both read the same, and turns over when this is fixed.

The fix is a `RangeRefusal` arm for a defined variable, spoken with
that recourse, raised by `axis_of`'s `Param` arm before the
`NotAContinuousParam` fallback.
