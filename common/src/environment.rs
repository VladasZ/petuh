use std::sync::OnceLock;

static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

pub enum Environment {
    Staging,
    Production,
}

impl Environment {
    fn get() -> &'static Self {
        ENVIRONMENT.get_or_init(|| {
            if std::env::var("STAGING").is_ok() {
                Self::Staging
            } else if std::env::var("PRODUCTION").is_ok() {
                Self::Production
            } else {
                Self::Staging
            }
        })
    }

    pub(crate) fn string() -> &'static str {
        Self::select("staging", "production")
    }

    pub(crate) fn staging() -> bool {
        matches!(Self::get(), Self::Staging)
    }

    pub(crate) fn production() -> bool {
        matches!(Self::get(), Self::Production)
    }

    pub fn select<T>(staging: T, production: T) -> T {
        if Self::staging() { staging } else { production }
    }
}
