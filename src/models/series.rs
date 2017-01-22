use ::schema::*;
use ::error::*;
use chrono::NaiveDate;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Series {
    pub id: i32,
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub episodes_total: Option<i32>,
    pub episodes_current: i32,
    pub info_link: Option<String>,
}


#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "series"]
pub struct NewSeries {
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub episodes_total: Option<i32>,
    pub episodes_current: i32,
    pub info_link: Option<String>,
}

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct SeriesForm {
    pub title: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub episodes_total: Option<i32>,
    pub episodes_current: i32,
    pub info_link: Option<String>,
}

impl NewSeries {
    pub fn validate(&self) -> Result<()> {
        if self.start_date.is_some() && self.end_date.is_some() {
            let start = self.start_date.unwrap();
            let end = self.end_date.unwrap();

            if end < start {
                return Err(ErrorKind::InvalidForm("series".to_owned(),
                                                  "end date is before start date".to_owned())
                    .into());
            }
        }

        Ok(())
    }
}

impl From<SeriesForm> for NewSeries {
    fn from(form: SeriesForm) -> Self {
        NewSeries {
            title: form.title,
            start_date: match form.start_date {
                Some(date) => date.parse::<NaiveDate>().ok(),
                None => None,
            },
            end_date: match form.end_date {
                Some(date) => date.parse::<NaiveDate>().ok(),
                None => None,
            },
            episodes_total: form.episodes_total,
            episodes_current: form.episodes_current,
            info_link: form.info_link,
        }
    }
}
