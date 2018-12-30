use crate::error::{Error, Result};
use crate::models::{schema::*, Series};
use serde_json::Value;

use diesel::dsl::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug, Default)]
#[table_name = "info_blob"]
#[belongs_to(Series, foreign_key = "series_id")]
pub struct InfoBlob {
    pub id: i32,
    pub series_id: i32,
    pub blob: Value,
    pub info_type: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Default)]
#[table_name = "info_blob"]
pub struct NewInfoBlob {
    pub series_id: i32,
    pub blob: Value,
    pub info_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InfoBlobForm {
    pub id: Option<i32>,
    pub blob: Value,
    pub info_type: String,
}

impl InfoBlob {
    pub fn all(conn: &PgConnection, series_id: i32) -> Result<Vec<Self>> {
        info_blob::dsl::info_blob
            .filter(info_blob::series_id.eq(series_id))
            .get_results(&*conn)
            .map_err(|err| err.into())
    }

    pub fn get(conn: &PgConnection, series_id: i32, id: i32) -> Result<Self> {
        info_blob::dsl::info_blob
            .filter(info_blob::series_id.eq(series_id))
            .filter(info_blob::id.eq(id))
            .first(conn)
            .map_err(|err| err.into())
    }

    pub fn new(conn: &PgConnection, series_id: i32, form: InfoBlobForm) -> Result<Self> {
        let series = series::dsl::series
            .filter(series::id.eq(series_id))
            .select(series::all_columns)
            .first(&*conn)
            .map_err::<Error, _>(|err| err.into())?;

        let blob = form.into_insertable(&series);

        insert_into(info_blob::table)
            .values(&blob)
            .returning(info_blob::all_columns)
            .get_result(&*conn)
            .map_err(|err| err.into())
    }

    pub fn update(conn: &PgConnection, series_id: i32, form: InfoBlobForm) -> Result<Self> {
        let blob_id = match form.id {
            Some(id) => id,
            None => return Err("id not given".into()),
        };

        update(
            info_blob::dsl::info_blob
                .filter(info_blob::id.eq(blob_id))
                .filter(info_blob::series_id.eq(series_id)),
        )
        .set((
            info_blob::blob.eq(form.blob),
            info_blob::info_type.eq(form.info_type),
        ))
        .returning(info_blob::all_columns)
        .get_result(&*conn)
        .map_err(|e| e.into())
    }

    pub fn delete(conn: &PgConnection, series_id: i32, id: i32) -> Result<Self> {
        delete(
            info_blob::dsl::info_blob
                .filter(info_blob::id.eq(id))
                .filter(info_blob::series_id.eq(series_id)),
        )
        .get_result(&*conn)
        .map_err(|err| err.into())
    }

    pub fn get_types(conn: &PgConnection, series_id: i32, types: Vec<&str>) -> Result<Vec<Self>> {
        info_blob::dsl::info_blob
            .filter(info_blob::series_id.eq(series_id))
            .filter(info_blob::info_type.eq_any(types))
            .get_results(&*conn)
            .map_err(|err| err.into())
    }
}

impl InfoBlobForm {
    pub fn into_insertable(self, series: &Series) -> NewInfoBlob {
        NewInfoBlob {
            series_id: series.id,
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

        let insertable = form.into_insertable(&series);

        assert_eq!(series.id, insertable.series_id);
        assert_eq!(json!({}), insertable.blob);
        assert_eq!("test".to_owned(), insertable.info_type);
    }

}
