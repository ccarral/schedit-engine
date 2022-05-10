use crate::csv::{read_grid_records, read_subject_records};
use crate::grid::{IdList, UaemEngineParams, UaemGrid, UaemPool};
use crate::subject::Subject;
use schedule_engine::engine::engine_main;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct ApiErr {
    pub msg: String,
}

#[derive(Serialize)]
struct ApiInitPoolsResult {
    subjects: Vec<Subject<IdList>>,
    pools: Vec<UaemPool>,
}

/// Inicializar las listas de grupos y materias de las cuales el algoritmo puede obtener para
/// iterar en las combinaciones. Esta función implícitamente también revisa que el csv ingresado
/// también sea válido
#[wasm_bindgen]
pub async fn api_init_pools(csv: String) -> Result<JsValue, JsValue> {
    match read_grid_records(&csv) {
        Ok((subjects, pools)) => {
            Ok(JsValue::from_serde(&ApiInitPoolsResult { subjects, pools }).unwrap())
        }
        Err(e) => {
            let err = JsValue::from_serde(&ApiErr {
                msg: format!("schedule-engine api err:{}", e),
            })
            .unwrap();
            Err(err)
        }
    }
}

#[wasm_bindgen]
pub fn api_engine_main(params: JsValue) -> Result<JsValue, JsValue> {
    let result: Result<UaemEngineParams, _> = JsValue::into_serde(&params);

    if let Ok(engine_params) = result {
        match engine_main(engine_params) {
            Ok(schedules) => {
                return Ok(JsValue::from_serde(&schedules).unwrap());
            }
            Err(msg) => {
                return Err(JsValue::from_serde(&ApiErr {
                    msg: format!("{:?}", msg),
                })
                .unwrap());
            }
        }
    } else if let Err(err) = result {
        return Err(JsValue::from_serde(&ApiErr {
            msg: format!("{:?}", err),
        })
        .unwrap());
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_pools() {
        let csv = std::fs::read_to_string("resources/2022/A/plantilla_ico_2022A.csv").unwrap();
        let (subject_list, pools) = read_grid_records(&csv).unwrap();

        let mut grid_count = 0;
        for pool in pools {
            for _grid in pool.grid_list {
                grid_count += 1;
            }
        }

        assert_eq!(grid_count, 129);
    }
}
