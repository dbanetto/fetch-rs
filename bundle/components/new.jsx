import React, { Component } from 'react';
import { Link } from 'react-router';
import SeriesForm from './seriesForm.jsx';

class SeriesNew extends Component {

  render() {
    return (
        <div>
          <SeriesForm router={ this.props.router } />
          <Link to="/">back</Link>
        </div>
        );
  }
}

export default SeriesNew;
