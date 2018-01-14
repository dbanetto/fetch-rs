use schema::*;
use models::Series;
use serde_json::Value;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Default)]
#[table_name = "info_blob"]
#[belongs_to(Series, foreign_key = "series_id")]
pub struct InfoBlob {
    pub id: i32,
    pub series_id: i32,
    pub blob: Value,
    pub primary: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Default)]
#[table_name = "info_blob"]
pub struct NewInfoBlob {
    pub series_id: i32,
    pub blob: Value,
    pub primary: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InfoBlobForm {
    pub id: Option<i32>,
    pub blob: Value,
    pub primary: Option<bool>,
}

impl InfoBlobForm {
    pub fn into_insertable(self, series: &Series) -> NewInfoBlob {
        NewInfoBlob {
            series_id: series.id,
            blob: self.blob,
            primary: match self.primary {
                Some(p) => p,
                None => false,
            },
        }
    }
}
