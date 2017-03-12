import React, { Component } from 'react';
import { Link } from 'react-router';
import Store from '../store.js';

class View extends Component {

  constructor() {
    super();

    this.state = {
      series: null,
    };
  }

  componentDidMount() {
    this.getSeries();
  }

  getSeries() {
    let self = this;

    Store.getSeriesId(this.props.params.id)
      .then(series => {
        self.setState({
          series: series
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
          </div>);
    }

    let series = this.state.series;

    return (
        <div>
          <div>
            <h1>{ series.title }</h1>
            <p>Start date: { series.start_date || "unkown" }</p>
            <p>End date: { series.end_date || "unkown" }</p>
            <p>Episode: { series.episodes_current }/{ series.episodes_total || "??" }</p>
          </div>
          <div>
            <span>
              <Link to={ `/series/${ series.id }/edit` }>edit</Link>
            </span>
            <span>
              <a href="javascript:void(0)">delete</a>
            </span>
            <span>
              <Link to='/'>back</Link>
              </span>
          </div>
        </div>
        );
  }
}

export default View;
