use crate::csv::{read_grid_records, read_subject_records};
use crate::grid::{UaemEngineParams, UaemGrid};
use schedule_engine::engine::engine_main;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct ApiErr {
    pub msg: String,
}

/// Inicializar las listas de *grupos* de las cuales el algoritmo puede obtener para
/// iterar en las combinaciones.
#[wasm_bindgen]
pub fn api_init_pools(csv: &str) -> Result<JsValue, JsValue> {
    match read_grid_records(csv) {
        Ok(pools) => Ok(JsValue::from_serde(&pools).unwrap()),
        Err(e) => {
            let err = JsValue::from_serde(&ApiErr { msg: e.to_string() }).unwrap();
            Err(err)
        }
    }
}

/// Inicializar las listas de materias dentro de las cuales el algoritmo
/// puede generar combinaciones. Toma como parámetro el csv de las materias con el siguiente
/// formato:
/// id, 'Nombre Profesor', 'Clave', 'Nombre de la materia','Grupo','ini_lun','fin_lun',...,'ini_dom','fin_dom'
#[wasm_bindgen]
pub fn api_leer_materias(csv: &str) -> Result<JsValue, JsValue> {
    if let Ok(subjects) = read_subject_records(csv) {
        Ok(JsValue::from_serde(&subjects).unwrap())
    } else {
        let err = JsValue::from_serde(&ApiErr {
            msg: "API Error!".to_string(),
        })
        .unwrap();
        Err(err)
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
        let pools = read_grid_records(&csv).unwrap();

        let mut grid_count = 0;
        for pool in pools {
            for _grid in pool.grid_list {
                grid_count += 1;
            }
        }

        assert_eq!(grid_count, 129);
    }
}
