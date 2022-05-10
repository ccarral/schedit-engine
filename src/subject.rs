use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject<Id: Eq> {
    pub subject_id: Id,
    pub name: String,
}

impl<Id> Subject<Id>
where
    Id: Eq,
{
    pub fn new(name: String, subject_id: Id) -> Self {
        Subject { subject_id, name }
    }
}
