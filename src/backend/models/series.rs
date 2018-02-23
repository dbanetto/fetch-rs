use schema::*;
use error::*;
use models::InfoBlobForm;

#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize, Debug, Default)]
#[table_name = "series"]
pub struct Series {
    pub id: i32,
    pub title: String,
    pub poster_url: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Default)]
#[table_name = "series"]
pub struct NewSeries {
    pub title: String,
    pub poster_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SeriesForm {
    pub title: String,
    pub poster_url: Option<String>,
    pub info: Option<Vec<InfoBlobForm>>,
}

impl NewSeries {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl SeriesForm {
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
