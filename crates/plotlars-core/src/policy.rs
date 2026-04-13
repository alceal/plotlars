use std::sync::OnceLock;

/// Controls how unsupported styling options are reported during rendering.
///
/// Some backends cannot express all styling options available in the IR.
/// This policy determines the behavior when such options are encountered.
///
/// # Example
///
/// ```
/// use plotlars::UnsupportedOptionPolicy;
/// use plotlars::set_unsupported_option_policy;
///
/// set_unsupported_option_policy(UnsupportedOptionPolicy::Strict);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnsupportedOptionPolicy {
    /// Silently ignore unsupported options. Plot renders, nothing printed.
    Ignore,
    /// Print to stderr. Always visible, including in Jupyter notebooks.
    /// This is the default.
    Warn,
    /// Print to stderr AND panic after listing all unsupported options.
    Strict,
}

static POLICY: OnceLock<UnsupportedOptionPolicy> = OnceLock::new();

/// Set the global unsupported option policy. First call wins; subsequent
/// calls are ignored. If never called, the default is `Warn`.
pub fn set_unsupported_option_policy(policy: UnsupportedOptionPolicy) {
    let _ = POLICY.set(policy);
}

/// Get the current policy. Returns `Warn` if never explicitly set.
pub fn unsupported_option_policy() -> UnsupportedOptionPolicy {
    POLICY
        .get()
        .copied()
        .unwrap_or(UnsupportedOptionPolicy::Warn)
}

/// Report an unsupported option. Appends to `collector` for Strict mode
/// batch reporting. Deduplicates within the collector.
///
/// Called by backend converters when an IR field has no backend equivalent.
pub fn report_unsupported(
    backend: &str,
    plot_type: &str,
    option: &str,
    collector: &mut Vec<String>,
) {
    let key = format!("{plot_type}.{option}");
    if collector.contains(&key) {
        return;
    }

    let policy = unsupported_option_policy();
    match policy {
        UnsupportedOptionPolicy::Ignore => {}
        UnsupportedOptionPolicy::Warn | UnsupportedOptionPolicy::Strict => {
            eprintln!("{backend}: `{option}` on {plot_type} not supported; ignored");
        }
    }
    collector.push(key);
}

/// Call after all traces/layout are converted in Strict mode.
/// Panics if any unsupported options were collected.
pub fn enforce_strict(backend: &str, collector: &[String]) {
    if unsupported_option_policy() == UnsupportedOptionPolicy::Strict && !collector.is_empty() {
        panic!(
            "{backend}: unsupported options encountered in Strict mode:\n  - {}",
            collector.join("\n  - ")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy_is_warn() {
        // OnceLock is process-global so we can only test the default
        // before any test calls set_unsupported_option_policy.
        assert_eq!(unsupported_option_policy(), UnsupportedOptionPolicy::Warn);
    }

    #[test]
    fn test_report_unsupported_deduplicates() {
        let mut collector = Vec::new();
        report_unsupported("test", "Scatter", "fill", &mut collector);
        report_unsupported("test", "Scatter", "fill", &mut collector);
        assert_eq!(collector.len(), 1);
    }

    #[test]
    fn test_report_unsupported_different_options() {
        let mut collector = Vec::new();
        report_unsupported("test", "Scatter", "fill", &mut collector);
        report_unsupported("test", "Scatter", "hover", &mut collector);
        assert_eq!(collector.len(), 2);
    }

    #[test]
    fn test_enforce_strict_with_empty_collector() {
        // Should not panic
        enforce_strict("test", &[]);
    }
}
