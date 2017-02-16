use ::schema::*;
use models::Series;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Default)]
#[table_name="info_uri"]
#[belongs_to(Series)]
pub struct InfoUri {
    id: i32,
    series_id: i32,
    uri: String,
    primary: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Default)]
#[table_name="info_uri"]
pub struct NewInfoUri {
    series_id: i32,
    uri: String,
    primary: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InfoUriForm {
    uri: String,
    primary: Option<bool>,
}

impl InfoUriForm {
    pub fn to_insertable(self, series: &Series) -> NewInfoUri {
        NewInfoUri {
            series_id: series.id,
            uri: self.uri,
            primary: match self.primary {
                Some(p) => p,
                None => false,
            }
        }
    }
}
