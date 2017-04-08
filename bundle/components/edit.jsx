import React, { Component } from 'react';
import { Link } from 'react-router';
import SeriesForm from './seriesForm.jsx';
import Store from '../store.js';

class SeriesEdit extends Component {

  constructor() {
    super();

    this.state = {
      series: null
    }
  }

  componentDidMount() {
    this.getSeries();
  }

  getSeries() {
    let self = this;

    Promise.all([Store.getSeriesId(this.props.params.id),
        Store.getSeriesUri(this.props.params.id)])
      .then(result => {
        let series = result[0];
        series.info_uris = result[1];

        self.setState({
          series: series,
        });
      })
    .catch(err => {
      console.log(err);
      self.props.router.push('/');
    });
  }


  render() {
    if (this.state.series === null) {
      return (
          <div>
            <p>loading...</p>
            <Link to='/'>back</Link>
            </div>
          );
    }

    console.log(this.state.series);
    return (
        <div>
          <SeriesForm router={ this.props.router } series={ this.state.series } />
          <Link to="/">back</Link>
        </div>
        );
  }
}

export default SeriesEdit;
