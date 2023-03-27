use std::collections::{HashMap, VecDeque};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub type Timestamp = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetUpdate {
    pub amount_diff: HashMap<String, Decimal>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    current: HashMap<String, Decimal>,
    updates: VecDeque<TargetUpdate>,
    is_active: bool,
}

impl Target {
    pub fn new(
        initial_target: HashMap<String, Decimal>,
        initial_updates: VecDeque<TargetUpdate>,
    ) -> Self {
        Self {
            current: initial_target,
            updates: initial_updates,
            is_active: false,
        }
    }
}
