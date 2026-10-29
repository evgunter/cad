//! **The resolution the tour's suites read a volume at**: 1.024 µm of
//! mean boundary displacement, the tour's stated precision
//! (`printed` in `src/main.rs`, which these rows check against). A
//! fixed length whatever ε a run is at, as the tour's is; spelled here
//! because the `all` test binary is a separate crate from the `demo-tour`
//! bin and cannot name the bin's items.

use pncad::topo::Settle;

pub fn printed() -> Settle<f64> {
    Settle::Resolution(pncad::quantity::Length::from_meters(1.024e-6))
}
