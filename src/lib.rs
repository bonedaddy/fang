#![doc = include_str!("../README.md")]

use std::time::Duration;
use thiserror::Error;
use typed_builder::TypedBuilder;

/// Represents a schedule for scheduled tasks.
///
/// It's used in the [`AsyncRunnable::cron`] and [`Runnable::cron`]
#[derive(Debug, Clone)]
pub enum Scheduled {
    /// A cron pattern for a periodic task
    ///
    /// For example, `Scheduled::CronPattern("0/20 * * * * * *")`
    CronPattern(String),
    /// A datetime for a scheduled task that will be executed once
    ///
    /// For example, `Scheduled::ScheduleOnce(chrono::Utc::now() + std::time::Duration::seconds(7i64))`
    ScheduleOnce(DateTime<Utc>),
}

/// List of error types that can occur while working with cron schedules.
#[derive(Debug, Error)]
pub enum CronError {
    /// A problem occured during cron schedule parsing.
    #[error(transparent)]
    LibraryError(#[from] cron::error::Error),
    /// [`Scheduled`] enum variant is not provided
    #[error("You have to implement method `cron()` in your AsyncRunnable")]
    TaskNotSchedulableError,
    /// The next execution can not be determined using the current [`Scheduled::CronPattern`]
    #[error("No timestamps match with this cron pattern")]
    NoTimestampsError,
}

/// All possible options for retaining tasks in the db after their execution.
///
/// The default mode is [`RetentionMode::RemoveAll`]
#[derive(Clone, Debug)]
pub enum RetentionMode {
    /// Keep all tasks
    KeepAll,
    /// Remove all tasks
    RemoveAll,
    /// Remove only successfully finished tasks
    RemoveFinished,
}

impl Default for RetentionMode {
    fn default() -> Self {
        RetentionMode::RemoveAll
    }
}

/// Configuration parameters for putting workers to sleep
/// while they don't have any tasks to execute
#[derive(Clone, Debug, TypedBuilder)]
pub struct SleepParams {
    /// the current sleep period
    pub sleep_period: Duration,
    /// the maximum period a worker is allowed to sleep.
    /// After this value is reached, `sleep_period` is not increased anymore
    pub max_sleep_period: Duration,
    /// the initial value of the `sleep_period`
    pub min_sleep_period: Duration,
    /// the step that `sleep_period` is increased by on every iteration
    pub sleep_step: Duration,
}

impl SleepParams {
    /// Reset the `sleep_period` if `sleep_period` > `min_sleep_period`
    pub fn maybe_reset_sleep_period(&mut self) {
        if self.sleep_period != self.min_sleep_period {
            self.sleep_period = self.min_sleep_period;
        }
    }

    /// Increase the `sleep_period` by the `sleep_step` if the `max_sleep_period` is not reached
    pub fn maybe_increase_sleep_period(&mut self) {
        if self.sleep_period < self.max_sleep_period {
            self.sleep_period += self.sleep_step;
        }
    }
}

impl Default for SleepParams {
    fn default() -> Self {
        SleepParams {
            sleep_period: Duration::from_secs(5),
            max_sleep_period: Duration::from_secs(15),
            min_sleep_period: Duration::from_secs(5),
            sleep_step: Duration::from_secs(5),
        }
    }
}

/// An error that can happen during executing of tasks
#[derive(Debug)]
pub struct FangError {
    /// A description of an error
    pub description: String,
}

#[doc(hidden)]
#[cfg(feature = "blocking")]
extern crate diesel;

#[doc(hidden)]
#[cfg(feature = "blocking")]
pub use diesel::pg::PgConnection;

#[doc(hidden)]
pub use typetag;

#[doc(hidden)]
pub extern crate serde;

#[doc(hidden)]
pub extern crate chrono;

#[doc(hidden)]
pub use serde_derive::{Deserialize, Serialize};

#[doc(hidden)]
pub use chrono::DateTime;
#[doc(hidden)]
pub use chrono::Utc;

#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "blocking")]
pub use blocking::*;

#[cfg(feature = "asynk")]
pub mod asynk;

#[cfg(feature = "asynk")]
pub use asynk::*;

#[cfg(feature = "asynk")]
#[doc(hidden)]
pub use bb8_postgres::tokio_postgres::tls::NoTls;

#[cfg(feature = "asynk")]
#[doc(hidden)]
pub use async_trait::async_trait;
