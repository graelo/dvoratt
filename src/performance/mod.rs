//! Performance tracking and statistics for the typing practice application.
//!
//! This module contains components for tracking and analyzing user performance:
//! - Word speed tracking (WPM calculation)
//! - Problem word identification and management
//! - Struggle combination detection
//! - Fastest/slowest word statistics
//!
//! The main entry point is the `PerformanceTracker` struct which aggregates
//! all performance metrics during a typing session.

mod fastest_slowest_words;
mod problem_words;
mod struggle_combinations;
mod word_speed_tracker;

mod performance_tracker;

pub(crate) use performance_tracker::PerformanceTracker;
