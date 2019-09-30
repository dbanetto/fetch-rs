use crate::data::{DataSource, DatabaseFilter};
use crate::error::{Error, Result};
use crate::models::{
    InfoBlob, InfoBlobForm, InfoBlobId, NewInfoBlob, Series, SeriesBlob, SeriesForm, SeriesId,
};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MemoryDatabase {
    series: HashMap<SeriesId, Series>,
    info_blobs: HashMap<SeriesId, HashMap<InfoBlobId, InfoBlob>>
}

impl DataSource for MemoryDatabase {
    fn all_infoblobs(&self, series_id: SeriesId) -> Result<Vec<InfoBlob>> {
        match self.info_blobs.get(&series_id) {
            Some(_blobs) => Ok(vec![]),
            None => Err("Not found".into()),
        }
    }

    fn get_infoblob(&self, series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob> {
        match self.info_blobs.get(&series_id).and_then(|blobs| { blobs.get(&id) }) {
            Some(blob) => Ok(blob.clone()),
            None => Err("Not found".into()),
        }
    }

    fn new_infoblob(&self, series_id: SeriesId, form: InfoBlobForm) -> Result<InfoBlob> {
        let id: InfoBlobId = 10;
        let blob = InfoBlob {
            id: id,
            series_id: series_id, 
            blob: form.blob,
            info_type: form.info_type,
        };

        // TODO: insert into map

        Ok(blob)
    }

    fn update_infoblob(
        &self,
        series_id: SeriesId,
        blob_id: InfoBlobId,
        form: InfoBlobForm,
    ) -> Result<InfoBlob> {

        let blob = InfoBlob {
            id: blob_id,
            series_id: series_id, 
            blob: form.blob,
            info_type: form.info_type,
        };
        
        // TODO: insert into map
        Ok(blob)
    }

    fn delete_infoblob(&self, series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob> {
        // TODO
        Ok(InfoBlob::default())
    }

    fn get_info_types(&self, series_id: SeriesId, types: Vec<&str>) -> Result<Vec<InfoBlob>> {
        // TODO
        Ok(vec![])
    }

    /// Retrieve all series in the database
    fn all_series(&self) -> Result<Vec<Series>> {
        Ok(vec![])
    }

    /// Get a series by id
    fn get_series(&self, id: SeriesId) -> Result<Series>{
        match self.series.get(&id) {
            Some(series) => Ok(series.clone()),
            None => Err("Not found".into()),
        }
    }

    /// Delete a series by id
    fn delete_series(&self, id: SeriesId) -> Result<Series>{
        Ok(Series::default())
}

    /// Update a series and associated info blobs
    fn update_series(&self, id: SeriesId, form: SeriesForm) -> Result<SeriesBlob>{
        let series = Series::default();
        Ok(SeriesBlob {
            id: id,
            title: series.title,
            poster_url: series.poster_url,
            blob: vec![],
        })
    }

    /// Create a series and associated info blobs
    fn new_series(&self, form: SeriesForm) -> Result<SeriesBlob>{
        let id: SeriesId = 1;
        Ok(SeriesBlob {
            id: id,
            title: form.title,
            poster_url: form.poster_url,
            blob: vec![],
        })

    }

    fn healthcheck(&self) -> Result<bool>{
        Ok(true)
    }
}
