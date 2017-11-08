import { h, Component } from 'preact';
import { Link } from 'preact-router';
import SeriesForm from './seriesForm.jsx';
import Store from '../store.js';

export default class SeriesEdit extends Component {

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

    Promise.all([Store.getSeriesId(this.props.matches.id),
        Store.getSeriesUri(this.props.matches.id)])
      .then(result => {
        let series = result[0];
        series.info_uris = result[1];

        self.setState({
          series: series,
        });
      })
    .catch(err => {
      console.log(err);
      route('/');
    });
  }


  render() {
    if (this.state.series === null) {
      return (
          <div>
            <p>loading...</p>
            <Link href='/'>back</Link>
            </div>
          );
    }

    console.log(this.state.series);
    return (
        <div>
          <SeriesForm router={ this.props.router } series={ this.state.series } />
          <Link href={ `/series/${ this.state.series.id }` }>back</Link>
        </div>
        );
  }
}
