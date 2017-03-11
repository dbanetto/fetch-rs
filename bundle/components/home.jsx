import React, { Component } from 'react';
import { Link } from 'react-router';
import SeriesCard from './seriesCard.jsx';

class Home extends Component {
  componentDidMount() {
    this.setState({
      series: null
    });

    this.loadSeries();
  }

  loadSeries() {
    let home = this;
    fetch('/api/v1/series')
      .then(res => res.json())
      .then(resp => {
        if (resp.success) {
          home.setState({
            series: resp.data
          });
        } else {
          throw resp.error;
        }
      })
      .catch(alert);
  }

  renderSeries() {
    if (this.state && this.state.series) {
      return (<div>
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
