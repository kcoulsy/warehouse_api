use chrono_tz::Tz;
use std::borrow::Cow;
use std::str::FromStr;
use validator::ValidationError;

/// Validate IANA timezone format
///
/// Accepts:
/// - "UTC" (special case)
/// - Valid IANA timezone identifiers (e.g., "America/New_York", "Europe/London")
pub fn validate_timezone(timezone: &str) -> Result<(), ValidationError> {
    if timezone.is_empty() {
        let mut err = ValidationError::new("timezone_empty");
        err.message = Some(Cow::Borrowed("Timezone is required and cannot be empty"));
        return Err(err);
    }

    // Allow UTC as a special case
    if timezone == "UTC" {
        return Ok(());
    }

    // Try to parse as IANA timezone
    match Tz::from_str(timezone) {
        Ok(_) => Ok(()),
        Err(_) => {
            let mut err = ValidationError::new("invalid_timezone");
            err.message = Some(Cow::Borrowed(
                "Invalid timezone format. Must be a valid IANA timezone (e.g., 'America/New_York', 'Europe/London', 'UTC')",
            ));
            Err(err)
        }
    }
}
