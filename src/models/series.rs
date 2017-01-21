
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
