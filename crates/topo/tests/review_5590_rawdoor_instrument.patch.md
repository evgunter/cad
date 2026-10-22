Review 5590 raw-door instrumentation (not applied): replace the
`measure_unvalidated` re-export in crates/topo/src/lib.rs with a
#[track_caller] wrapper that runs `T::gate_at_rest(body, tol)` and appends
`file:line VALID|REFUSED[kinds]` to $RAWDOOR_LOG, then forwards to
`raw_door::measure_body`. Run at bff1f2e99 over topo's lib (8 filters) and
tests/all.rs: 33 raw-door sites reached, every body REFUSED except the
declared valid/inverted helpers (props.rs:6931, geometric_cube.rs:101,
offset_doors_read_charts_not_material.rs:111).
