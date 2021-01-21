#![cfg(target_arch = "wasm32")]

use schedule_engine::{
    engine::{engine_main, EngineParams},
    log,
};
use uaemex_horarios::csv::read_grid_records;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_read_grid_records() {
    let pools = read_grid_records(CSV).unwrap();

    assert_eq!(pools.len(), 5);
}

#[wasm_bindgen_test]
fn test_main_no_seeds() {
    let pool_list = read_grid_records(CSV).unwrap();
    let params = EngineParams {
        seeds: vec![],
        bound: 5,
        pool_list,
    };

    let schedules = engine_main(params).unwrap();

    for schedule in schedules {
        log!("{:?}", schedule);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_main_no_seeds() {
        let pool_list = read_grid_records(CSV).unwrap();
        let params = EngineParams {
            seeds: vec![],
            bound: 3,
            pool_list,
        };

        engine_main(params);
    }
}

const CSV: &str = "1,'DIAZGONZALEZ BOYER OMAR','LINC18 / L41038','ARQUITECTURA DE COMPUTADORAS','CO01',7:00,9:00,,,7:00,9:00,,,,,,,,
2,'VALENCIA PEREZ PEDRO','LINC18 / L41038','ARQUITECTURA DE COMPUTADORAS','CO02',,,9:00,11:00,,,9:00,11:00,,,,,,
3,'LEBARIO MENCHACA JUAN','LINC18 / L41038','ARQUITECTURA DE COMPUTADORAS','CO03',,,19:00,21:00,,,19:00,21:00,,,,,,
4,'TRUJILLO FLORES EDUARDO','LINC20 / L41061','BASES DE DATOS I / FUNDAMENTOS DE BASES DE DATOS','C001',13:00,15:00,,,13:00,15:00,,,,,,,,
5,'RIVAS ARZALUZ MA DE LOURDES','LINC20 / L41061','BASES DE DATOS I / FUNDAMENTOS DE BASES DE DATOS','C002',,,,,,,18:00,20:00,,,9:00,11:00,,
6,'LEYVA PELAEZ CAROL','LINC20 / L41061','BASES DE DATOS I / FUNDAMENTOS DE BASES DE DATOS','C003',7:00,9:00,,,7:00,9:00,,,,,,,,
7,'SARABIA ORTIZ VICTOR','LINC21 / L41062','BASES DE DATOS II / BASES DE DATOS AVANZADAS','CO01',18:00,20:00,,,18:00,20:00,,,,,,,,
8,'SALAS CASTILLO PABLO','LINC21 / L41062','BASES DE DATOS II / BASES DE DATOS AVANZADAS','CO02',,,,,,,,,,,9:00,13:00,,
9,'SALAS CASTILLO PABLO','LINC21 / L41062','BASES DE DATOS II / BASES DE DATOS AVANZADAS','CO03',,,,,7:00,9:00,,,,,7:00,9:00,,
10,'MUNGUIA CEDILLO NATALIA CECILIA','LINC06 / L41019','COMUNICACION Y RELACIONES HUMANAS','C001',,,16:30,18:30,,,16:30,18:30,,,,,,
11,'ARCOS SANTOVEÑA MA. DE LOURDES','LINC06 / L41019','COMUNICACION Y RELACIONES HUMANAS','C002',9:00,11:00,,,9:00,11:00,,,,,,,,
12,'MUNGUIA CEDILLO NATALIA CECILIA','LINC06 / L41019','COMUNICACION Y RELACIONES HUMANAS','C003',,,11:00,13:00,,,11:00,13:00,,,,,,
13,'QUINTANA GUERRA MARIA ROSA','LINC09 / L41021','ELECTROMAGNETISMO / ELECTRICIDAD Y MAGNETISMO','CO01',7:00,9:00,,,7:00,9:00,,,,,,,,
14,'DIAZ ACEVES ERNESTO','LINC09 / L41021','ELECTROMAGNETISMO / ELECTRICIDAD Y MAGNETISMO','CO02',,,,,15:30,17:30,,,15:30,17:30,,,,
15,'ROSSANO DIAZ IVAN OSVALDO','LINC09 / L41021','ELECTROMAGNETISMO / ELECTRICIDAD Y MAGNETISMO','C003',13:00,15:00,,,13:00,15:00,,,,,,,,";
