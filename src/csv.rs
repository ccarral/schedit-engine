use crate::grid::{IdList, UaemGrid, UaemPool};
use crate::subject::Subject;
use anyhow::Result;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::hash_map::HashMap;

#[derive(Deserialize, Debug)]
struct ScheduleRecord {
    id: u32,
    name: String,
    subject_key: String,
    subject_name: String,
    group: String,
    time_values: [String; 14],
}

pub fn read_grid_records(csv: &str) -> Result<Vec<UaemPool>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .quote(b'\'')
        .from_reader(csv.as_bytes());

    let mut pool_list: Vec<UaemPool> = vec![];

    for record in reader.deserialize() {
        let record: ScheduleRecord = record?;

        let mut id_list: Vec<String> = record
            .subject_key
            .split('/')
            .map(|s| s.to_owned())
            .collect();

        // Remove whitespace
        id_list
            .iter_mut()
            .for_each(|s| s.retain(|c| !c.is_whitespace()));

        let mut data = HashMap::new();

        data.insert("profesor".to_string(), record.name);
        data.insert("grupo".to_string(), record.group);
        data.insert("nombre".to_string(), record.subject_name);

        let grid = UaemGrid::from_vec(
            IdList {
                id_list: id_list.to_owned(),
            },
            record.time_values,
            "%H:%M",
            data,
        )?;

        let mut found = false;

        for pool in &mut pool_list {
            if grid.pool_id == pool.pool_id {
                pool.push(grid);
                found = true;
                break;
            }
        }

        if !found {
            let new_pool = {
                let id_list = IdList { id_list };
                UaemPool::new(id_list)
            };

            pool_list.push(new_pool);
        }
    }

    Ok(pool_list)
}

pub fn read_subject_records(csv: &str) -> Result<Vec<Subject>> {
    const ID_IDX: usize = 0;
    const NAME_IDX: usize = 1;

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .quote(b'\'')
        .from_reader(csv.as_bytes());

    let mut subjects = vec![];

    for result in reader.records() {
        let record = result?;

        let mut id_list: Vec<String> = record[ID_IDX].split('/').map(|s| s.to_owned()).collect();

        // Remove whitespace from id's
        id_list
            .iter_mut()
            .for_each(|s| s.retain(|c| !c.is_whitespace()));

        let name = record[NAME_IDX].to_owned();

        subjects.push(Subject::new(name, id_list));
    }

    Ok(subjects)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_csv_subjects() {
        let csv = std::fs::read_to_string("resources/2021/A/subjects.csv").unwrap();

        let subjects = read_subject_records(&csv).unwrap();
        for subject in subjects {
            for id in subject.id_list {
                assert!(!id.contains(char::is_whitespace));
            }
        }
    }
}
