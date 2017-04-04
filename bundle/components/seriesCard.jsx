import React from 'react';
import { Link } from 'react-router';
import Store from '../store.js';

class SeriesCard extends React.Component {

  constructor() {
    super();

    this.state = {
      primary: null
    }
  }

  componentDidMount() {
    let self = this;
    Store.getSeriesPrimary(this.props.series.id)
      .then(uri => {
        self.setState({
          primary: uri
        });
      }).catch(() => null);
  }

  getAiringSate() {
    let series = this.props.series;
    let today = new Date();

    let start_date = series.start_date ? new Date(series.start_date) : null;
    let end_date = series.end_date ? new Date(series.end_date) : null;

    let status = 'unknown';

    if (start_date && today.getTime() < start_date.getTime()) {
      status = 'soon';
    } else if (start_date && today.getTime() > start_date.getTime()) {

      if (end_date && today.getTime() > end_date.getTime()) {
        status = 'finished';
      } else {
        status = 'airing';
      }
    }

    return (<span className={`status status-${status}` }>{status}</span>);
  }


  render() {
    var series = this.props.series;
    return (<div className="card">
      <h1>{ series.title }</h1>
      <div>
        <p>Start date: { series.start_date || "unknown" }</p>
        <p>End date: { series.end_date || "unknown" }</p>
        <p>Episode: { series.episodes_current }/{ series.episodes_total || "??" }</p>
        <p>State: { this.getAiringSate() }</p>
        { this.state.primary && <a href={ this.state.primary.uri }>link</a> }
        <Link to={`/series/${ series.id }`}>view</Link>
      </div>
    </div>);
  }
}

export default SeriesCard;
