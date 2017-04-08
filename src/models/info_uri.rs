use ::schema::*;
use models::Series;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Default)]
#[table_name="info_uri"]
#[belongs_to(Series, foreign_key="series_id")]
pub struct InfoUri {
    pub id: i32,
    pub series_id: i32,
    pub uri: String,
    pub primary: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Default)]
#[table_name="info_uri"]
pub struct NewInfoUri {
    pub series_id: i32,
    pub uri: String,
    pub primary: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InfoUriForm {
    pub id: Option<i32>,
    pub uri: String,
    pub primary: Option<bool>,
}

impl InfoUriForm {
    pub fn into_insertable(self, series: &Series) -> NewInfoUri {
        NewInfoUri {
            series_id: series.id,
            uri: self.uri,
            primary: match self.primary {
                Some(p) => p,
                None => false,
            },
        }
    }
}
