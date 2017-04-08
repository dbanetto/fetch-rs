import React, { Component } from 'react';
import UriList from './uriList.jsx';

class SeriesForm extends Component {

  constructor(props) {
    super();

    let series = props && props.series ? props.series : {};

    this.state = {
      series: series
    }
  }

  validate(formData) {
    // add validation
    return errors;
  }

  formData(form) {

  }

  handleSubmit(event) {
    event.preventDefault();

    let formData = this.state.series;
    console.log(JSON.stringify(formData));

    let self = this;
    let action = {
      method: 'POST',
      url: '/api/v1/series/new'
    };

    if (formData.id) {
      action = {
        method: 'PUT',
        url: `/api/v1/series/${formData.id}`
      };
    }

    fetch(action.url, {
      method: action.method,
      body: JSON.stringify(formData),
      headers: {
        'Content-Type': 'application/json'
      }})
    .then(r => r.json())
      .then(resp => {
        if (!resp.success) {
          throw resp.error;
        }
        // redirect to view
        self.props.router.push(`/series/${ resp.data.id }`);
      })
    .catch(alert);

    return false;
  }

  handleUpdate(value, event) {
    let series = this.state.series;
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
            <input name="title" id="title" type="text" value={series.title} required
              onChange={ this.handleUpdate.bind(this, 'title') } />
          </div>
          <div>
            <input name="start_date" id="start_date" type="date" value={series.start_date}
              max={series.end_date}
              onChange={ this.handleUpdate.bind(this, 'start_date') } />

            <input name="end_date" id="end_date" type="date" value={series.end_date}
              min={series.start_date}
              onChange={ this.handleUpdate.bind(this, 'end_date') } />
          </div>
          <div>
            <input name="episodes_current" id="episodes_current" type="number"
              min="0" max={series.episodes_total} value={series.episodes_current}
              onChange={ this.handleUpdate.bind(this, 'episodes_current') } />

            <input name="episodes_total" id="episodes_total" type="number"
              min={series.episodes_current} value={series.episodes_total}
              onChange={ this.handleUpdate.bind(this, 'episodes_total') } />
          </div>
          <div>
            <input name="poster_url" id="poster_url" type="url" value={series.poster_url}
              onChange={ this.handleUpdate.bind(this, 'poster_url') } />
          </div>
          <div>
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
