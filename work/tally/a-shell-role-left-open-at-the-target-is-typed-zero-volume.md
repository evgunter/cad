---
id: a-shell-role-left-open-at-the-target-is-typed-zero-volume
kind: issue
title: A shell role whose volume bracket is left open at the reporting target is typed zero volume, not unmeasured
status: open
opened: 2026-10-18
---


## What

`props::shell_role` reads a shell's role through `props::role_walk`,
whose `last_word` drops the target-level refusal it is handed (a face
that refused on budget), and a role left undecided becomes
`RoleUnread::refusal` → `shell_role_refusal` (`crates/topo/src/props.rs`).
That reads only the two ends' signs. A bracket `[lo, hi]` with `lo`
definite negative and `hi` in the zero band, or `lo` in the zero band and
`hi` definite positive, is typed `ShellClassifyError::ZeroVolume`.
`ZeroVolume` is only a sound verdict when both ends read zero, because
the true volume lies somewhere in `[lo, hi]`. A bracket with one end off
zero has not measured the volume. Its true value can be a void-sized
negative or an outer-sized positive. A budget refusal that left the
bracket wide is the one way such a bracket survives to the target, and
that refusal is the more accurate word (`MassPropsError` / "volume
uncomputable").

The at-infinity probe used the same mapping
(`boolean::solid_contain::at_infinity_side`'s `undecided`). RESTREAD's
`restread/at-infinity-probe` branch splits it there with
`RoleUnread::zero_at_both_ends`, so that `ZeroVolumeBody` is minted only
where both ends read zero. The shell-role side is TALLY's ground and is
left as it stands.

## Shape of a fix

In `shell_role_refusal`, type `ZeroVolume` only when both ends decide
zero. Otherwise use `Straddles` or a new unmeasured arm. Or have
`role_walk`'s `last_word` carry the budget refusal it receives, so that a
budget-exhausted read refuses as uncomputable. No reachable witness has
been measured. Building one needs a quadrature face whose budget runs out
short of the shell's sign.

## One classification, two readers (RESTREAD fix pass, PR 5311)

The probe's typing is now `RoleUnread::reads_zero`. It reads the
certified reading's ends where a certificate was taken
(`certified_by` records them as `certified_zero`), and the walk's ends
only where none was taken. `CertifiedSliver` stays a zero volume. So
`RoleUnread` now has two classifications of the same ends that disagree:
`reads_zero` for the at-infinity probe
(`boolean::solid_contain::unread_refusal`), and `shell_role_refusal` for
check 10 and the shell classification. The fix here is to route
`shell_role_refusal`'s `ZeroVolume` arm through `reads_zero` as well, and
to give the remaining non-zero, non-straddle case its own arm or fold it
into `Straddles`. That changes check 10's refusal words and the sliver
interplay `RoleRefusal` carries, which is TALLY's ground, so RESTREAD
left it here.
