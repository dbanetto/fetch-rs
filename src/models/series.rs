use ::schema::*;
use ::error::*;
use models::InfoUriForm;
use chrono::NaiveDate;

#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize, Debug, Default)]
#[table_name="series"]
pub struct Series {
    pub id: i32,
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub episodes_total: Option<i32>,
    pub episodes_current: i32,
}


#[derive(Insertable, Serialize, Deserialize, Debug, Default)]
#[table_name = "series"]
pub struct NewSeries {
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub episodes_total: Option<i32>,
    pub episodes_current: i32,
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SeriesForm {
    pub title: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub episodes_total: Option<i32>,
    pub episodes_current: i32,
    pub info_uris: Vec<InfoUriForm>,
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

        if self.episodes_current < 0 {
            return Err(ErrorKind::InvalidForm("series".to_owned(),
                                              "current episode must be greater or equal to than \
                                               0"
                                                  .to_owned())
                .into());
        }

        Ok(())
    }
}

impl SeriesForm {
    pub fn into_new(self) -> (NewSeries, Vec<InfoUriForm>) {
        (NewSeries {
             title: self.title,
             start_date: match self.start_date {
                 Some(date) => date.parse::<NaiveDate>().ok(),
                 None => None,
             },
             end_date: match self.end_date {
                 Some(date) => date.parse::<NaiveDate>().ok(),
                 None => None,
             },
             episodes_total: self.episodes_total,
             episodes_current: self.episodes_current,
         },
         self.info_uris)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn newseries_validate_end_before_start_date() {
        let series = NewSeries {
            start_date: Some(NaiveDate::from_ymd(2017, 1, 1)),
            end_date: Some(NaiveDate::from_ymd(2017, 2, 1)),
            ..Default::default()
        };

        assert!(series.validate().is_ok());
    }

    #[test]
    fn newseries_validate_end_after_start_date() {
        let series = NewSeries {
            start_date: Some(NaiveDate::from_ymd(2017, 2, 1)),
            end_date: Some(NaiveDate::from_ymd(2017, 1, 1)),
            ..Default::default()
        };

        assert!(series.validate().is_err());
    }

    #[test]
    fn newseries_validate_episodes_current_greater_zero() {
        let series = NewSeries { episodes_current: 1, ..Default::default() };

        assert!(series.validate().is_ok());
    }

    #[test]
    fn newseries_validate_episodes_current_equal_zero() {
        let series = NewSeries { episodes_current: 0, ..Default::default() };

        assert!(series.validate().is_ok());
    }

    #[test]
    fn newseries_validate_episodes_current_less_than_zero() {
        let series = NewSeries { episodes_current: -1, ..Default::default() };

        assert!(series.validate().is_err());
    }

    #[test]
    fn newseries_validate_end_on_start_date() {
        let series = NewSeries {
            start_date: Some(NaiveDate::from_ymd(2017, 1, 1)),
            end_date: Some(NaiveDate::from_ymd(2017, 1, 1)),
            ..Default::default()
        };

        assert!(series.validate().is_ok());
    }

    #[test]
    fn seriesform_to_newseries_date_parse_start_date_valid() {
        let series_form =
            SeriesForm { start_date: Some("2017-01-01".to_owned()), ..Default::default() };

        let (series, _) = series_form.into_new();

        assert!(series.start_date.is_some());
        assert_eq!(NaiveDate::from_ymd(2017, 1, 1), series.start_date.unwrap());
    }

    #[test]
    fn seriesform_to_newseries_date_parse_start_date_invalid() {
        let series_form =
            SeriesForm { start_date: Some("invalid date".to_owned()), ..Default::default() };

        let (series, _) = series_form.into_new();

        assert!(series.start_date.is_none());
    }

    #[test]
    fn seriesform_to_newseries_date_parse_end_date_valid() {
        let series_form =
            SeriesForm { end_date: Some("2017-01-01".to_owned()), ..Default::default() };

        let (series, _) = series_form.into_new();

        assert!(series.end_date.is_some());
        assert_eq!(NaiveDate::from_ymd(2017, 1, 1), series.end_date.unwrap());
    }

    #[test]
    fn seriesform_to_newseries_date_parse_end_date_invalid() {
        let series_form =
            SeriesForm { end_date: Some("invalid date".to_owned()), ..Default::default() };

        let (series, _) = series_form.into_new();

        assert!(series.end_date.is_none());
    }
}
