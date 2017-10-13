import React, { Component } from 'react';
import UriList from './uriList.jsx';
import Store from '../store.js';

class SeriesForm extends Component {

  constructor(props) {
    super();

    let series = props && props.series ? props.series : {};

    this.state = {
      series: series
    }
  }

  validate(formData) {
    let errors = [];

    let nonEmptyUri = formData.info_uris.filter((uri) => uri.uri.trim().length !== 0);
    formData.info_uris = nonEmptyUri;

    return errors;
  }

  handleSubmit(event) {
    event.preventDefault();

    let formData = this.state.series;

    let errors = this.validate(formData);
    if (errors.length > 0) {
      console.log(errors);
      return;
    }

    let self = this;

    Store.upsertSeries(formData)
      .then(resp => {
        // redirect to view
        self.props.router.push(`/series/${ resp.id }`);
      })
      .catch(alert);

    // stops the HTML form from completing the request
    return false;
  }

  handleUpdate(event, value) {
    let series = this.state.series;
    let isNumber = false;
    if (isNumber) {
      let valueNum = parseInt(event.target.value);
      if (isNaN(valueNum)) {
        console.error('expected ' + event.target.value + ' to be a number');
      } else {
        series[value] = valueNum;
      }
    } else {
      series[value] = event.target.value;
    }
    this.setState({
      series: series
    });
  }

  handleInfoUriUpdate(value) {
    let series = this.state.series;

    series.info_uris = value;

    this.setState({
      series: series
    });
  }

  render() {
    let series = this.state.series;
    return (
        <form onSubmit={this.handleSubmit.bind(this)}>
          <div>
            <input name="id" id="id" type="hidden" value={series.id} />
          </div>
          <div>
            <label htmlFor="title">Title</label>
            <input name="title" id="title" type="text" value={series.title} required
              onChange={ this.handleUpdate.bind(this, 'title') } />
          </div>
          <div>
            <label htmlFor="start_date">Start Date</label>
            <label htmlFor="end_date">End Date</label>
            <input name="start_date" id="start_date" type="date" value={series.start_date}
              max={series.end_date}
              onChange={ this.handleUpdate.bind(this, 'start_date') } />

            <input name="end_date" id="end_date" type="date" value={series.end_date}
              min={series.start_date}
              onChange={ this.handleUpdate.bind(this, 'end_date') } />
          </div>
          <div>
            <label htmlFor="episodes_current">Current Episodes</label>
            <input name="episodes_current" id="episodes_current" type="number"
              min="0" max={series.episodes_total} value={series.episodes_current}
              onChange={ this.handleUpdate.bind(this, 'episodes_current', true) } />

            <label htmlFor="episodes_total">Total Episodes</label>
            <input name="episodes_total" id="episodes_total" type="number"
              min={series.episodes_current} value={series.episodes_total}
              onChange={ this.handleUpdate.bind(this, 'episodes_total', true) } />
          </div>
          <div>
            <label htmlFor="poster_url">Poster URL</label>
            <input name="poster_url" id="poster_url" type="url" value={series.poster_url}
              onChange={ this.handleUpdate.bind(this, 'poster_url') } />
          </div>
          <div>
            <h3>Info URIs</h3>
            <UriList value={series.info_uris || []}
              handleUpdate={ this.handleInfoUriUpdate.bind(this) } />
          </div>
          <div>
            <input type="submit" />
          </div>
        </form>
        );
  }
}

export default SeriesForm;
