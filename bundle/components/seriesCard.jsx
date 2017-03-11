import React from 'react';
import { Link } from 'react-router';

class SeriesCard extends React.Component {

  getAiringSate() {
    let series = this.props.series;
    let today = new Date();

    let start_date = series.start_date ? new Date(series.start_date) : null;
    let end_date = series.end_date ? new Date(series.end_date) : null;

    let status = 'unkown';

    if (start_date && today.getTime() < start_date.getTime()) {
      status = 'soon';
    } else if (start_date && today.getTime() > start_date.getTime()) {

      if (end_date && today.getTime() > end_date.getTime()) {
        status = 'finished';
      } else {
        status = 'airing';
      }
    }

    return (<span className={`status-${status}` }>{status}</span>);
  }


  render() {
    var series = this.props.series;
    return (<div>
      <h1>{ series.title }</h1>
      <div>
        <p>Start date: { series.start_date || "unkown" }</p>
        <p>End date: { series.end_date || "unkown" }</p>
        <p>Episode: { series.episodes_current }/{ series.episodes_total || "??" }</p>
        <p>State: { this.getAiringSate() }</p>
        <Link to={`/series/${ series.id }`}>view</Link>
      </div>
    </div>);
  }
}

export default SeriesCard;
