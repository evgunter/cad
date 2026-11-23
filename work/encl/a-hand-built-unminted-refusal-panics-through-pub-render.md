---
id: a-hand-built-unminted-refusal-panics-through-pub-render
kind: issue
title: A hand-built refusal on an unminted side still panics through pub render at a build
status: closed
pr: 5393
opened: 2026-10-19
branch: encl/total-render
closed: 2026-10-19
---


Found by ENCL's param-span-doors final review (PR 5365).

`SizedDecision::recourse` (`crates/geom-brep/src/recourse.rs`, its
`# Panics` section) panics on a reading certainly on a
`RefusedSide::Unminted` side read at a build. No public door returns
such a refusal (each reports it as `CertifyError::StatedInterval`), but
a caller can build one and render it through `pub` API:

- `geom_brep::CertifyError::WindingExceeded.render(Reading::Build)`
  (`CertifyError::render`, `crates/geom-brep/src/certify.rs:938`);
- `geom_brep::certify::recourse(CertCheck::ParamSpan,
  RefusedArm::SignCertain(None), Reading::Build)`
  (`crates/geom-brep/src/certify.rs:1306`);
- a hand-built `SizedDecision { side: RefusedSide::Unminted, .. }` with
  `.recourse(..)` on a certainly-refused arm.

D2 says the kernel never panics on input, and a `pub` render of a
caller-built value is input. The repair: a total render that ends an
`Unminted` reading at a build in the kernel-defect ending
(`KERNEL_OR_FILE_DEFECT_ENDING`), keeping the panic only at topo's
kernel call sites (`topo::policy_lane::kernels_own`), where the input
is the kernel's own.
