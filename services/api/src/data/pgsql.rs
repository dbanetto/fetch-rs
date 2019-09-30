use crate::data::{DataSource, DatabaseFilter};
use crate::error::{Error, Result};
use crate::models::{
    InfoBlob, InfoBlobForm, InfoBlobId, NewInfoBlob, Series, SeriesBlob, SeriesForm, SeriesId,
};
use crate::schema::*;

use std::env;

use warp::Filter;

use diesel::connection::Connection as DieselConnection;
use diesel::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{delete, insert_into, update};

type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Connection {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Connection {
    pub fn new_environment() -> Result<Self> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

        let manager = ConnectionManager::new(database_url.to_owned());

        let pool = Pool::new(manager).map_err::<Error, _>(|err| err.into())?;

        Ok(Connection { pool: pool })
    }

    pub fn new<S: Into<String>>(database_url: S) -> Result<Self> {
        let manager = ConnectionManager::new(database_url.into());

        let pool = Pool::new(manager).map_err::<Error, _>(|err| err.into())?;

        Ok(Connection { pool: pool })
    }

    pub fn into_warp(self) -> DatabaseFilter {
        warp::any()
            .map(move || Box::new(self.clone()) as Box<dyn DataSource + Send>)
            .boxed()
    }

    fn get_connection(&self) -> Result<PooledConn> {
        self.pool.get().map_err::<Error, _>(|err| err.into())
    }

    // InfoBlob direct Postgres functions

    fn pg_all_infoblobs(conn: &PgConnection, series_id: SeriesId) -> Result<Vec<InfoBlob>> {
        info_blob::dsl::info_blob
            .filter(info_blob::series_id.eq(series_id))
            .get_results(&*conn)
            .map_err(|err| err.into())
    }

    fn pg_get_infoblob(
        conn: &PgConnection,
        series_id: SeriesId,
        id: InfoBlobId,
    ) -> Result<InfoBlob> {
        info_blob::dsl::info_blob
            .filter(info_blob::series_id.eq(series_id))
            .filter(info_blob::id.eq(id))
            .first(conn)
            .map_err(|err| err.into())
    }

    fn pg_new_infoblob(
        conn: &PgConnection,
        series_id: SeriesId,
        form: InfoBlobForm,
    ) -> Result<InfoBlob> {
        let series: Series = series::dsl::series
            .filter(series::id.eq(series_id))
            .select(series::all_columns)
            .first(&*conn)
            .map_err::<Error, _>(|err| err.into())?;

        let blob = form.into_insertable(series.id);

        insert_into(info_blob::table)
            .values(&blob)
            .returning(info_blob::all_columns)
            .get_result(&*conn)
            .map_err(|err| err.into())
    }

    fn pg_update_infoblob(
        conn: &PgConnection,
        series_id: SeriesId,
        blob_id: InfoBlobId,
        form: InfoBlobForm,
    ) -> Result<InfoBlob> {
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

    fn pg_delete_infoblob(
        conn: &PgConnection,
        series_id: SeriesId,
        id: InfoBlobId,
    ) -> Result<InfoBlob> {
        delete(
            info_blob::dsl::info_blob
                .filter(info_blob::id.eq(id))
                .filter(info_blob::series_id.eq(series_id)),
        )
        .get_result(&*conn)
        .map_err(|err| err.into())
    }

    fn pg_get_info_types(
        conn: &PgConnection,
        series_id: SeriesId,
        types: Vec<&str>,
    ) -> Result<Vec<InfoBlob>> {
        info_blob::dsl::info_blob
            .filter(info_blob::series_id.eq(series_id))
            .filter(info_blob::info_type.eq_any(types))
            .get_results(&*conn)
            .map_err(|err| err.into())
    }

    /// Retrieve all series in the database
    fn pg_all_series(conn: &PgConnection) -> Result<Vec<Series>> {
        series::dsl::series
            .load::<Series>(conn)
            .map_err(|err| err.into())
    }

    /// Get a series by id
    fn pg_get_series(conn: &PgConnection, id: SeriesId) -> Result<Series> {
        series::dsl::series
            .filter(series::id.eq(id))
            .select(series::all_columns)
            .first(&*conn)
            .map_err(|err| err.into())
    }

    /// Delete a series by id
    fn pg_delete_series(conn: &PgConnection, id: SeriesId) -> Result<Series> {
        diesel::delete(series::dsl::series.filter(series::id.eq(id)))
            .returning(series::all_columns)
            .get_result(&*conn)
            .map_err(|err| err.into())
    }

    /// Update a series and associated info blobs
    fn pg_update_series(conn: &PgConnection, id: SeriesId, form: SeriesForm) -> Result<SeriesBlob> {
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

            if let Some(blobs_put) = blobs_put {
                let mut blobs_ids: Vec<InfoBlobId> = vec![];

                // list of blobs to NOT delete
                for blob in blobs_put {
                    let result = if let Some(blob_id) = blob.id {
                        // update
                        Connection::pg_update_infoblob(conn, id, blob_id, blob)?
                    } else {
                        // create
                        Connection::pg_new_infoblob(conn, id, blob)?
                    };
                    blobs_ids.push(result.id)
                }

                // delete info_blobs that were not apart of the PUT
                delete(
                    info_blob::dsl::info_blob
                        .filter(not(info_blob::id.eq(any(blobs_ids))))
                        .filter(info_blob::series_id.eq(id)),
                )
                .execute(&*conn)
                .map_err::<Error, _>(|err| err.into())?;
            }

            Ok(SeriesBlob {
                id: series.id,
                title: series.title,
                poster_url: series.poster_url,
                blob: Connection::pg_all_infoblobs(conn, series.id)?,
            })
        })
    }

    /// Create a series and associated info blobs
    fn pg_new_series(conn: &PgConnection, form: SeriesForm) -> Result<SeriesBlob> {
        let (new_series, blobs) = form.into_new();

        let _ = new_series.validate()?;

        conn.transaction::<_, Error, _>(|| {
            let series: Series = insert_into(series::table)
                .values(&new_series)
                .returning(series::all_columns)
                .get_result(&*conn)
                .map_err::<Error, _>(|err| err.into())?;

            let blobs = if let Some(blobs) = blobs {
                let insertable_blobs = blobs
                    .into_iter()
                    .map(|i| i.into_insertable(series.id))
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

impl DataSource for Connection {
    fn all_infoblobs(&self, series_id: SeriesId) -> Result<Vec<InfoBlob>> {
        let conn = &*self.get_connection()?;
        Self::pg_all_infoblobs(conn, series_id)
    }

    fn get_infoblob(&self, series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob> {
        let conn = &*self.get_connection()?;
        Self::pg_get_infoblob(conn, series_id, id)
    }

    fn new_infoblob(&mut self, series_id: SeriesId, form: InfoBlobForm) -> Result<InfoBlob> {
        let conn = &*self.get_connection()?;
        Self::pg_new_infoblob(conn, series_id, form)
    }

    fn update_infoblob(
        &mut self,
        series_id: SeriesId,
        blob_id: InfoBlobId,
        form: InfoBlobForm,
    ) -> Result<InfoBlob> {
        let conn = &*self.get_connection()?;
        Self::pg_update_infoblob(conn, series_id, blob_id, form)
    }

    fn delete_infoblob(&mut self, series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob> {
        let conn = &*self.get_connection()?;
        Self::pg_delete_infoblob(conn, series_id, id)
    }

    fn get_info_types(&self, series_id: SeriesId, types: Vec<&str>) -> Result<Vec<InfoBlob>> {
        let conn = &*self.get_connection()?;
        Self::pg_get_info_types(conn, series_id, types)
    }

    /// Retrieve all series in the database
    fn all_series(&self) -> Result<Vec<Series>> {
        let conn = &*self.get_connection()?;
        Self::pg_all_series(conn)
    }

    /// Get a series by id
    fn get_series(&self, id: SeriesId) -> Result<Series> {
        let conn = &*self.get_connection()?;
        Self::pg_get_series(conn, id)
    }

    /// Delete a series by id
    fn delete_series(&mut self, id: SeriesId) -> Result<Series> {
        let conn = &*self.get_connection()?;
        Self::pg_delete_series(conn, id)
    }

    /// Update a series and associated info blobs
    fn update_series(&mut self, id: SeriesId, form: SeriesForm) -> Result<SeriesBlob> {
        let conn = &*self.get_connection()?;
        Self::pg_update_series(conn, id, form)
    }

    /// Create a series and associated info blobs
    fn new_series(&mut self, form: SeriesForm) -> Result<SeriesBlob> {
        let conn = &*self.get_connection()?;
        Self::pg_new_series(conn, form)
    }

    fn healthcheck(&self) -> Result<bool> {
        let conn = &*self.get_connection()?;
        conn.execute("SELECT 1;")
            .map(|_| true)
            .map_err(|err| err.into())
    }
}
