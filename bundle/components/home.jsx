import React, { Component } from 'react';
import { Link } from 'react-router';
import SeriesCard from './seriesCard.jsx';
import Store from '../store.js';

class Home extends Component {
  constructor() {
    super();

    this.state = {
      series: Store.getSeriesCache()
    };
  }

  componentDidMount() {
    this.loadSeries();
  }

  loadSeries(useCache) {
    let self = this;
    Store.getSeries()
      .then(series => {
        self.setState({
          series: series
        });
      })
      .catch(alert);
  }

  renderSeries() {
    if (this.state && this.state.series) {
      return (<div className="card-box">
        { this.state.series.map(i => <SeriesCard key={i.id} series={i} />) }
      </div>);
    } else {
      return (<span>loading...</span>);
    }
  }

  render() {
    return (
        <div>
          <h2>Series List</h2>
            { this.renderSeries() }
            <Link to="/series/new">create</Link>
          <button onClick={this.loadSeries.bind(this)}>Reload</button>
        </div>
        );
  }
}

export default Home;
