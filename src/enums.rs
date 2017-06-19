//! Useful enums.

/// The state in which a module is in, either exported or unexported.
#[derive(Debug, PartialEq, Eq)]
pub enum DeviceState {
  /// Exported and available for use.
  Exported,
  /// Not exported and unavailable for use.
  Unexported,
}
