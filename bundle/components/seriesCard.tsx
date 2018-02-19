import { h, Component } from 'preact';
import { Link } from 'preact-router';
import Store from '../store';
import '../model';

interface CardProps {
    series: Series;
}

interface CardState {
    primary: InfoBlob;
}

export default class SeriesCard extends Component<CardProps, CardState> {

  constructor() {
    super();

    this.state = {
      primary: null
    }
  }

  componentDidMount() {
    let self = this;
    Store.getSeriesPrimary(this.props.series.id)
      .then(blob => {
        self.setState({
          primary: blob
        });
      }).catch(() => null);
  }

  getAiringSate(): preact.VNode {
    let series = this.props.series;

    let status = 'unknown';

    return (<span className={`status status-${status}` }>{status}</span>);
  }


  render() {
    var series = this.props.series;
    return (<div className="card">
      <div className="poster">
        <img src={ series.poster_url } />
      </div>
      <div className="card-body">
        <h1>{ series.title }</h1>
        <p>State: { this.getAiringSate() }</p>
        { this.state.primary && this.state.primary.info_type === "url"  && <a href={ this.state.primary.blob.url }>link</a> }
        <Link href={`/series/${ series.id }`}>view</Link>
      </div>
    </div>);
  }
}
