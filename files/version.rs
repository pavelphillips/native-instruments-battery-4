//! Static build info for Native Instruments Battery 4.

pub const BUILD_VERSION: &str  = "v2.0.22";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "69463415e4b8";
pub const BUILD_TIMESTAMP: u64 = 1717144803;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Native Instruments Battery 4", BUILD_VERSION, BUILD_CHANNEL)
}
