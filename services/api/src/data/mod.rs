pub mod memory;
pub mod pgsql;

use crate::error::Result;
use crate::models::{InfoBlob, InfoBlobForm, InfoBlobId, Series, SeriesBlob, SeriesForm, SeriesId};

pub type DatabaseFiltered = Box<dyn DataSource + Send>;
pub type DatabaseFilter = warp::filters::BoxedFilter<(DatabaseFiltered,)>;

pub trait DataSource {
    fn all_infoblobs(&self, series_id: SeriesId) -> Result<Vec<InfoBlob>>;

    fn get_infoblob(&self, series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob>;

    fn new_infoblob(&mut self, series_id: SeriesId, form: InfoBlobForm) -> Result<InfoBlob>;

    fn update_infoblob(
        &mut self,
        series_id: SeriesId,
        blob_id: InfoBlobId,
        form: InfoBlobForm,
    ) -> Result<InfoBlob>;

    fn delete_infoblob(&mut self, series_id: SeriesId, id: InfoBlobId) -> Result<InfoBlob>;

    fn get_info_types(&self, series_id: SeriesId, types: Vec<&str>) -> Result<Vec<InfoBlob>>;

    /// Retrieve all series in the database
    fn all_series(&self) -> Result<Vec<Series>>;

    /// Get a series by id
    fn get_series(&self, id: SeriesId) -> Result<Series>;

    /// Delete a series by id
    fn delete_series(&mut self, id: SeriesId) -> Result<Series>;

    /// Update a series and associated info blobs
    fn update_series(&mut self, id: SeriesId, form: SeriesForm) -> Result<SeriesBlob>;

    /// Create a series and associated info blobs
    fn new_series(&mut self, form: SeriesForm) -> Result<SeriesBlob>;

    fn healthcheck(&self) -> Result<bool>;
}
