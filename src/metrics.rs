use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

// metrics has a new method that creates a new instance of Metrics
// with an empty HashMap
// other methods have incr, decr,  and snapshot methods
impl Metrics {
    pub fn new() -> Metrics {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn incr(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let count = data.entry(key.into()).or_insert(0);
        *count += 1;
        Ok(())
    }

    pub fn decr(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.lock().map_err(|e| anyhow!(e.to_string()))?;
        let count = data.entry(key.into()).or_insert(0);
        *count -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}
