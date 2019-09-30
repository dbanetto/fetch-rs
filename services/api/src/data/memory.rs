use crate::data::DataSource;
use crate::error::{Error, Result};
use crate::models::{InfoBlob, InfoBlobForm, InfoBlobId, Series, SeriesBlob, SeriesForm, SeriesId};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MemoryDatabase {
    series_id_counter: SeriesId,
    info_blob_id_counter: InfoBlobId,
    series: Vec<Series>,
    info_blobs: Vec<InfoBlob>,
}

impl DataSource for MemoryDatabase {
    fn all_infoblobs(&self, series_id: SeriesId) -> Result<Vec<InfoBlob>> {
        let _ = self.get_series(series_id)?;

        let results = self
            .info_blobs
            .clone()
            .into_iter()
            .filter(|blob| blob.series_id == series_id)
            .collect();

        Ok(results)
    }

    fn get_infoblob(&self, series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob> {
        self.info_blobs
            .iter()
            .find(|blob| blob.series_id == series_id && blob.id == id)
            .map(|blob| blob.clone())
            .ok_or::<Error>("Not Found".into())
    }

    fn new_infoblob(&mut self, series_id: SeriesId, form: InfoBlobForm) -> Result<InfoBlob> {
        // Check if the series exists
        let _ = self.get_series(series_id)?;

        // Allocate an id
        let blob_id = self.info_blob_id_counter;
        self.info_blob_id_counter += 1;

        let blob = InfoBlob {
            id: blob_id,
            series_id: series_id,
            blob: form.blob,
            info_type: form.info_type,
        };

        self.info_blobs.push(blob.clone());

        Ok(blob)
    }

    fn update_infoblob(
        &mut self,
        series_id: SeriesId,
        blob_id: InfoBlobId,
        form: InfoBlobForm,
    ) -> Result<InfoBlob> {
        let _ = self.delete_infoblob(series_id, blob_id)?;

        let blob = InfoBlob {
            id: blob_id,
            series_id: series_id,
            blob: form.blob,
            info_type: form.info_type,
        };

        self.info_blobs.push(blob.clone());

        Ok(blob)
    }

    fn delete_infoblob(&mut self, _series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob> {
        let (mut deleted, kept) = self
            .info_blobs
            .iter()
            .cloned()
            .partition(|blob| blob.id == id);

        self.info_blobs = kept;

        deleted.pop().ok_or::<Error>("Not Found".into())
    }

    fn get_info_types(&self, series_id: SeriesId, types: Vec<&str>) -> Result<Vec<InfoBlob>> {
        Ok(self
            .info_blobs
            .iter()
            .cloned()
            .filter(|blob| blob.series_id == series_id)
            .filter(|blob| types.contains(&blob.info_type.as_str()))
            .collect())
    }

    /// Retrieve all series in the database
    fn all_series(&self) -> Result<Vec<Series>> {
        Ok(self.series.clone())
    }

    /// Get a series by id
    fn get_series(&self, id: SeriesId) -> Result<Series> {
        self.series
            .iter()
            .find(|s| s.id == id)
            .map(|s| s.clone())
            .ok_or::<Error>("Not Found".into())
    }

    /// Delete a series by id
    fn delete_series(&mut self, id: SeriesId) -> Result<Series> {
        // remove blobs with series
        for blob in self.all_infoblobs(id)? {
            let _ = self.delete_infoblob(id, blob.id)?;
        }

        let (mut deleted, kept) = self.series.iter().cloned().partition(|s| s.id == id);

        self.series = kept;

        deleted.pop().ok_or::<Error>("Not Found".into())
    }

    /// Update a series and associated info blobs
    fn update_series(&mut self, id: SeriesId, form: SeriesForm) -> Result<SeriesBlob> {
        let mut series = self.delete_series(id)?;

        series.poster_url = form.poster_url;
        series.title = form.title;

        let mut saved = vec![];
        if let Some(blobs) = form.info {
            for blob in blobs {
                if let Some(blob_id) = blob.id {
                    saved.push(self.update_infoblob(id, blob_id, blob)?);
                } else {
                    saved.push(self.new_infoblob(id, blob)?);
                }
            }
        }

        self.series.push(series.clone());

        Ok(SeriesBlob {
            id: id,
            title: series.title,
            poster_url: series.poster_url,
            blob: saved,
        })
    }

    /// Create a series and associated info blobs
    fn new_series(&mut self, form: SeriesForm) -> Result<SeriesBlob> {
        let id = self.series_id_counter;
        self.series_id_counter += 1;

        let series = Series {
            id: id,
            title: form.title,
            poster_url: form.poster_url,
        };

        let mut saved = vec![];
        if let Some(blobs) = form.info {
            for blob in blobs {
                saved.push(self.new_infoblob(id, blob)?);
            }
        }

        self.series.push(series.clone());

        Ok(SeriesBlob {
            id: id,
            title: series.title,
            poster_url: series.poster_url,
            blob: saved,
        })
    }

    fn healthcheck(&self) -> Result<bool> {
        Ok(true)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn not_found_series() {
        // Arrange
        let data = MemoryDatabase::default();

        // Action
        // Assert
        assert!(data.get_series(0).is_err());
    }

    #[test]
    fn not_found_infoblob() {
        // Arrange
        let data = MemoryDatabase::default();

        // Action
        // Assert
        assert!(data.get_infoblob(0, 0).is_err());
    }

    #[test]
    fn stores_series_data() {
        // Arrange
        let mut data = MemoryDatabase::default();

        // Action
        let result = data.new_series(SeriesForm::default()).unwrap();

        // Assert
        assert!(data.get_series(result.id).is_ok());
    }

    #[test]
    fn stores_info_data() {
        // Arrange
        let mut data = MemoryDatabase::default();

        // Action
        let series = data.new_series(SeriesForm::default()).unwrap();
        let result = data
            .new_infoblob(series.id, InfoBlobForm::default())
            .unwrap();

        // Assert
        assert!(data.get_infoblob(series.id, result.id).is_ok());
    }
}
