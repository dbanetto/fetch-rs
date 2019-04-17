use crate::error::{Error, Result};
use crate::models::{InfoBlob, InfoBlobForm, NewInfoBlob};
use crate::schema::*;

use diesel::dsl::*;
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use url::Url;

/// Series object from database
#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize, Debug, Default)]
#[table_name = "series"]
pub struct Series {
    pub id: i32,
    pub title: String,
    pub poster_url: Option<String>,
}

/// Series object to be insert into database
#[derive(Insertable, Serialize, Deserialize, Debug, Default)]
#[table_name = "series"]
pub struct NewSeries {
    pub title: String,
    pub poster_url: Option<String>,
}

/// Form to create or update a series with many info blobs associated to that new series
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SeriesForm {
    pub title: String,
    pub poster_url: Option<String>,
    pub info: Option<Vec<InfoBlobForm>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SeriesBlob {
    pub id: i32,
    pub title: String,
    pub poster_url: Option<String>,
    pub blob: Vec<InfoBlob>,
}

impl Series {
    /// Retrieve all series in the database
    pub fn all(conn: &PgConnection) -> Result<Vec<Self>> {
        series::dsl::series
            .load::<Series>(conn)
            .map_err(|err| err.into())
    }

    /// Get a series by id
    pub fn get(conn: &PgConnection, id: i32) -> Result<Self> {
        series::dsl::series
            .filter(series::id.eq(id))
            .select(series::all_columns)
            .first(&*conn)
            .map_err(|err| err.into())
    }

    /// Delete a series by id
    pub fn delete(conn: &PgConnection, id: i32) -> Result<Self> {
        diesel::delete(series::dsl::series.filter(series::id.eq(id)))
            .returning(series::all_columns)
            .get_result(&*conn)
            .map_err(|err| err.into())
    }

    /// Update a series and associated info blobs
    pub fn update(conn: &PgConnection, id: i32, form: SeriesForm) -> Result<SeriesBlob> {
        let (series_put, blobs_put) = form.into_new();

        let _ = series_put.validate()?;

        conn.transaction::<_, Error, _>(|| {
            let series: Series = update(series::dsl::series.filter(series::id.eq(id)))
                .set((
                    series::title.eq(series_put.title),
                    series::poster_url.eq(series_put.poster_url),
                ))
                .returning(series::all_columns)
                .get_result(&*conn)
                .map_err::<Error, _>(|err| err.into())?;

            let mut blobs = vec![];
            if let Some(blobs_put) = blobs_put {
                // list of blobs to NOT delete

                for blob in blobs_put {
                    if let Some(blob_id) = blob.id {
                        // update
                        blobs.push(InfoBlob::update(conn, id, blob_id, blob)?)
                    } else {
                        // create
                        blobs.push(InfoBlob::new(conn, id, blob)?)
                    }
                }

                // delete info_blobs that were not apart of the PUT
                delete(
                    info_blob::dsl::info_blob
                        .filter(not(
                            info_blob::id.eq(any(blobs.iter().map(|b| b.id).collect::<Vec<i32>>()))
                        ))
                        .filter(info_blob::series_id.eq(id)),
                )
                .execute(&*conn)
                .map_err::<Error, _>(|err| err.into())?;
            }

            Ok(SeriesBlob {
                id: series.id,
                title: series.title,
                poster_url: series.poster_url,
                blob: blobs,
            })
        })
    }

    /// Create a series and associated info blobs
    pub fn new(conn: &PgConnection, form: SeriesForm) -> Result<SeriesBlob> {
        let (new_series, blobs) = form.into_new();

        let _ = new_series.validate()?;

        conn.transaction::<_, Error, _>(|| {
            let series = insert_into(series::table)
                .values(&new_series)
                .returning(series::all_columns)
                .get_result(&*conn)
                .map_err::<Error, _>(|err| err.into())?;

            let blobs = if let Some(blobs) = blobs {
                let insertable_blobs = blobs
                    .into_iter()
                    .map(|i| i.into_insertable(&series))
                    .collect::<Vec<NewInfoBlob>>();

                let inserted_blobs = insert_into(info_blob::table)
                    .values(&insertable_blobs)
                    .returning(info_blob::all_columns)
                    .get_results(&*conn)
                    .map_err::<Error, _>(|err| err.into())?;

                inserted_blobs
            } else {
                Vec::new()
            };

            Ok(SeriesBlob {
                id: series.id,
                title: series.title,
                poster_url: series.poster_url,
                blob: blobs,
            })
        })
    }
}

impl NewSeries {
    /// Validate
    pub fn validate(&self) -> Result<()> {
        if let Some(ref poster_url) = &self.poster_url {
            let _ = Url::parse(poster_url).map_err::<Error, _>(|err| err.into())?;
        }

        Ok(())
    }
}

impl SeriesForm {
    /// Transforms form into insertable objects
    pub fn into_new(self) -> (NewSeries, Option<Vec<InfoBlobForm>>) {
        (
            NewSeries {
                title: self.title,
                poster_url: self.poster_url,
            },
            self.info,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::models::SeriesForm;

    #[test]
    fn form_to_insertable() {
        let form = SeriesForm {
            title: "test".to_owned(),
            poster_url: None,
            info: None,
        };

        let (series, blobs) = form.into_new();

        assert_eq!("test".to_owned(), series.title);
        assert_eq!(None, series.poster_url);
        assert!(blobs.is_none());
    }

}
