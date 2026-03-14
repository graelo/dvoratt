//! Performance tracking for the Dvorak typing practice application.
//!
//! This module contains components for tracking and analyzing typing performance,
//! including words per minute calculation, problem word identification, struggle
//! combinations analysis, and fastest/slowest word tracking.

mod fastest_slowest_words;
mod problem_words;
mod struggle_combinations;
mod word_speed_tracker;

mod performance_tracker;

pub(crate) use performance_tracker::PerformanceTracker;
