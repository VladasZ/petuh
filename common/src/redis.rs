use std::{
    fmt::{Debug, Display},
    str::FromStr,
    sync::OnceLock,
};

use anyhow::Result;
use chrono::Local;
use redis::{ExistenceCheck, SetOptions, ToRedisArgs, TypedCommands};
use tracing::{info, trace};

use crate::Environment;

static REDIS_CLUSTER: OnceLock<::redis::cluster::ClusterClient> = OnceLock::new();
static REDIS: OnceLock<::redis::Client> = OnceLock::new();
static APP_NAME: OnceLock<&'static str> = OnceLock::new();

pub struct Redis {}

impl Redis {
    pub fn set(key: impl Display, value: impl ToRedisArgs + Send + Sync) -> Result<()> {
        if Environment::production() {
            redis_cluster()?.set(Self::key(key), value)?;
        } else {
            redis()?.set(Self::key(key), value)?;
        }

        Ok(())
    }

    pub fn set_if_does_not_exists(key: impl Display, value: impl ToRedisArgs + Send + Sync) -> Result<()> {
        let options = SetOptions::default().conditional_set(ExistenceCheck::NX);

        if Environment::production() {
            redis_cluster()?.set_options(Self::key(key), value, options)?;
        } else {
            redis()?.set_options(Self::key(key), value, options)?;
        }

        Ok(())
    }

    pub fn get(key: impl Display) -> Result<Option<String>> {
        let value = if Environment::production() {
            redis_cluster()?.get(Self::key(key))?
        } else {
            redis()?.get(Self::key(key))?
        };

        Ok(value)
    }

    pub fn get_typed<T: FromStr<Err: std::error::Error + Send + Sync + 'static> + Debug>(
        key: impl Display,
    ) -> Result<Option<T>> {
        match Self::get(key)? {
            None => Ok(None),
            Some(val) => Ok(Some(val.parse()?)),
        }
    }
}

impl Redis {
    pub(crate) fn set_app_name(name: &'static str) -> Result<()> {
        APP_NAME.set(name).expect("App name for Redis is set twice");
        trace!(app_name = name, "App name is set for Redis");

        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        Redis::set_if_does_not_exists("first_redis_usage", timestamp)
    }

    pub(crate) fn app_name() -> &'static str {
        APP_NAME.get().expect("App name is not set for Redis")
    }

    fn key(key: impl Display) -> String {
        format!("{}:{}:{key}", Self::app_name(), Environment::string())
    }
}

fn redis() -> Result<redis::Connection> {
    let client = REDIS.get_or_init(|| {
        let redis_url = "redis://localhost:6379";
        info!(redis_url = redis_url);
        ::redis::Client::open(redis_url).expect("Failed to open redis")
    });

    Ok(client.get_connection()?)
}

fn redis_cluster() -> Result<redis::cluster::ClusterConnection> {
    let client = REDIS_CLUSTER.get_or_init(|| {
        let redis_url = "redis://redis-cluster-master.ot-operators.svc.cluster.local:6379"; // typos:ignore
        info!(redis_url = redis_url);
        ::redis::cluster::ClusterClient::new(vec![redis_url]).expect("Failed to open redis")
    });

    Ok(client.get_connection()?)
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::redis::Redis;

    #[test]
    #[ignore]
    fn test() -> Result<()> {
        unsafe { std::env::set_var("STAGING", "") };

        Redis::set_app_name("petuh-test")?;

        Redis::set("some-test-component:user-id", 4234234)?;

        assert_eq!(
            Redis::get("some-test-component:user-id")?,
            Some("4234234".to_string())
        );

        assert_eq!(
            Redis::get_typed::<i32>("some-test-component:user-id")?,
            Some(4234234)
        );

        Ok(())
    }
}
