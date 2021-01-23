use crate::csv::read_grid_records;
use crate::grid::UaemGrid;
use schedule_engine::engine::{engine_main, EngineParams};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct ApiErr {
    pub msg: String,
}

#[wasm_bindgen]
pub fn api_init_pools(csv: &str) -> Result<JsValue, JsValue> {
    if let Ok(pools) = read_grid_records(csv) {
        Ok(JsValue::from_serde(&pools).unwrap())
    } else {
        let err = JsValue::from_serde(&ApiErr {
            msg: "API Error!".to_string(),
        })
        .unwrap();
        Err(err)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_pools() {
        let csv = std::fs::read_to_string("resources/2021/A/schedules.csv").unwrap();
        let pools = read_grid_records(&csv).unwrap();

        let mut grid_count = 0;
        for pool in pools {
            for grid in pool.grid_list {
                grid_count += 1;
            }
        }

        assert_eq!(grid_count, 130);
    }
}
