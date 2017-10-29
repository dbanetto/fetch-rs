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

    let status = 'unknown';

    return (<span className={`status status-${status}` }>{status}</span>);
  }


  render() {
    var series = this.props.series;
    return (<div className="card">
      <div className="poster">
        <img src={ series.poster_url } />
      </div>
      <div className="card-body">
        <h1>{ series.title }</h1>
        <p>State: { this.getAiringSate() }</p>
        { this.state.primary && <a href={ this.state.primary.uri }>link</a> }
        <Link to={`/series/${ series.id }`}>view</Link>
      </div>
    </div>);
  }
}

export default SeriesCard;
