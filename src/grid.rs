use schedule_engine::grid::{Grid, Pool};
use serde::Serialize;
use std::collections::hash_map::HashMap;

pub type UaemGrid = Grid<IdList, HashMap<String, String>>;

pub type UaemPool = Pool<IdList, HashMap<String, String>>;

#[derive(Debug, Eq, Clone, Serialize)]
pub struct IdList {
    pub id_list: Vec<String>,
}

impl PartialEq for IdList {
    fn eq(&self, other: &Self) -> bool {
        for inner_id in other.id_list.iter() {
            if self.id_list.contains(inner_id) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let id_list_1 = IdList {
            id_list: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        };
        let id_list_2 = IdList {
            id_list: vec!["x".to_string(), "b".to_string(), "r".to_string()],
        };
        assert_eq!(id_list_1, id_list_2);
    }
}
