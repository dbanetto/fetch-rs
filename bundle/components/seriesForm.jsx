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

    let info_uris = formData.info_uris ? formData.info_uris : [];

    formData.info_uris = info_uris.filter((uri) => uri.uri.trim().length !== 0);

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

  handleUpdate(value, event) {
    let series = this.state.series;

    console.log(this);
    console.log(event);
    console.log(value);

    series[value] = event.target.value;

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
