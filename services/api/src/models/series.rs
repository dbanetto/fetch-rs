use crate::error::{Error, Result};
use crate::models::{InfoBlob, InfoBlobForm};
use crate::schema::*;

use url::Url;

/// Series object from database
#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize, Debug, Default, Clone)]
#[table_name = "series"]
pub struct Series {
    pub id: SeriesId,
    pub title: String,
    pub poster_url: Option<String>,
}

/// Series object to be insert into database
#[derive(Insertable, Serialize, Deserialize, Debug, Default, Clone)]
#[table_name = "series"]
pub struct NewSeries {
    pub title: String,
    pub poster_url: Option<String>,
}

/// Form to create or update a series with many info blobs associated to that new series
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SeriesForm {
    pub title: String,
    pub poster_url: Option<String>,
    pub info: Option<Vec<InfoBlobForm>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SeriesBlob {
    pub id: SeriesId,
    pub title: String,
    pub poster_url: Option<String>,
    pub blob: Vec<InfoBlob>,
}

pub type SeriesId = i32;

impl NewSeries {
    /// Validate
    pub fn validate(&self) -> Result<()> {
        if let Some(ref poster_url) = &self.poster_url {
            let url =
                Url::parse(poster_url).map_err::<Error, _>(|_| "Invalid poster url".into())?;

            match url.scheme() {
                "http" | "https" => (),
                _ => return Err("Invalid poster url".into()),
            }
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
    use crate::models::{NewSeries, SeriesForm};

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

    #[test]
    fn new_series_validate_url_ok_none() {
        let form = NewSeries {
            title: "test".to_owned(),
            poster_url: None,
        };

        assert!(form.validate().is_ok());
    }

    #[test]
    fn new_series_validate_url_err_empty() {
        let form = NewSeries {
            title: "test".to_owned(),
            poster_url: Some("".to_owned()),
        };

        assert!(form.validate().is_err());
    }

    #[test]
    fn new_series_validate_url_ok_http() {
        let form = NewSeries {
            title: "test".to_owned(),
            poster_url: Some("http://a".to_owned()),
        };

        assert!(form.validate().is_ok());
    }

    #[test]
    fn new_series_validate_url_ok_https() {
        let form = NewSeries {
            title: "test".to_owned(),
            poster_url: Some("https://a".to_owned()),
        };

        assert!(form.validate().is_ok());
    }

    #[test]
    fn new_series_validate_url_err_file() {
        let form = NewSeries {
            title: "test".to_owned(),
            poster_url: Some("file://a".to_owned()),
        };

        assert!(form.validate().is_err());
    }

    #[test]
    fn new_series_validate_url_err_custom() {
        let form = NewSeries {
            title: "test".to_owned(),
            poster_url: Some("customprotocl://a".to_owned()),
        };

        assert!(form.validate().is_err());
    }
}
