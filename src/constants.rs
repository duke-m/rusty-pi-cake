use crate::types::TPrecision;

/// Default precision for the calculation. The time needed to calculate the approximation is
/// proportional to the precision, in a development environment it will probably take
/// 3x longer than in a production environment due to instrumentation. Each increment of the
/// precision will take ca. 3x longer to calculate.
#[cfg(not(debug_assertions))]
pub const DEFAULT_PRECISION: TPrecision = 8;
#[cfg(debug_assertions)]
pub const DEFAULT_PRECISION: TPrecision = 7;

/// Default delay for letting the browser simply paint the UI before blocking, in milliseconds.
pub const DELAY_MS: u64 = 20;