import { h, Component } from 'preact';
import { route } from 'preact-router';
import InfoList from './infoList';
import Store from '../store';
import handler from './handler';

export default class SeriesForm extends Component {

  constructor(props) {
    super();

    let series = props && props.series ? props.series : {};

    this.state = {
      series: series
    }
  }

  validate(formData) {
    let errors = [];

    let blobs = formData.blobs ? formData.blobs : [];

    // formData.blobs = blobs.filter((blob) => blobs);

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
        route(`/series/${ resp.id }`, true);
      })
      .catch(alert);

    // stops the HTML form from completing the request
    return false;
  }

  handleUpdate(key, value) {
    let series = this.state.series;

    value = value.target ? value.target.value : value;

    console.log("before series");
    console.log(series);

    series[key] = value;

    console.log("after series");
    console.log(series);

    this.setState({
      series: series
    });
  }

  handleInfoUpdate(key, value) {
    let series = this.state.series;

    series.info = value;

    this.setState({
      series: series
    });
  }

  handleUpdatePoster(blob) {
    this.handleUpdate('poster_url', blob.src);
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
            { handler.build({ src: series.poster_url }, 'image', { edit: true, name: 'poster_url',
              handleUpdate: this.handleUpdatePoster.bind(this) }) }
          </div>
          <div>
            <h3>Info</h3>.
            <InfoList value={series.info || []}
              handleUpdate={ this.handleInfoUpdate.bind(this) } />
          </div>
          <div>
            <input type="submit" />
          </div>
        </form>
        );
  }
}
