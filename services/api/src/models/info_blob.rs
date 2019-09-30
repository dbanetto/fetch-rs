use crate::models::{Series, SeriesId};
use crate::schema::*;

use serde_json::Value;

pub type InfoBlobId = i32;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Default, Clone)]
#[table_name = "info_blob"]
#[belongs_to(Series, foreign_key = "series_id")]
pub struct InfoBlob {
    pub id: InfoBlobId,
    pub series_id: SeriesId,
    pub blob: Value,
    pub info_type: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Default, Clone)]
#[table_name = "info_blob"]
pub struct NewInfoBlob {
    pub series_id: SeriesId,
    pub blob: Value,
    pub info_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct InfoBlobForm {
    pub id: Option<InfoBlobId>,
    pub blob: Value,
    pub info_type: String,
}

impl InfoBlobForm {

    pub fn into_insertable(self, series_id: SeriesId) -> NewInfoBlob {
        NewInfoBlob {
            series_id: series_id,
            blob: self.blob,
            info_type: self.info_type,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::{InfoBlobForm, Series};
    use serde_json::json;

    #[test]
    fn form_to_insertable() {
        let form = InfoBlobForm {
            id: None,
            blob: json!({}),
            info_type: "test".to_owned(),
        };

        let series = Series {
            id: 1,
            title: "".to_owned(),
            poster_url: None,
        };

        let insertable = form.into_insertable(series.id);

        assert_eq!(series.id, insertable.series_id);
        assert_eq!(json!({}), insertable.blob);
        assert_eq!("test".to_owned(), insertable.info_type);
    }
}
