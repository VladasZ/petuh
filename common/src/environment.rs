use std::sync::OnceLock;

static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

pub enum Environment {
    Staging,
    Production,
}

impl Environment {
    fn get() -> &'static Self {
        ENVIRONMENT.get_or_init(|| {
            if let Ok(_) = std::env::var("STAGING") {
                Self::Staging
            } else if let Ok(_) = std::env::var("PRODUCTION") {
                Self::Production
            } else {
                panic!("No STAGING or PRODUCTION env var supplied")
            }
        })
    }

    pub fn staging() -> bool {
        matches!(Self::get(), Self::Staging)
    }

    pub fn production() -> bool {
        matches!(Self::get(), Self::Production)
    }
}
