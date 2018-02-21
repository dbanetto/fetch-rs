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
      return (
          <div class="card is-full box">
              <Link href={`/series/${ series.id }`} >
                  <div class="poster">
                      <img class="image" src={ series.poster_url } />
                  </div>
              </Link>
              <div class="card-body">
                  <h2 class="subtitle">{ series.title }</h2>
                  <div>
                      &nbsp;
                      <div class="is-pulled-right">
                          { this.state.primary && this.state.primary.info_type === "url"  && <a class="button" href={ this.state.primary.blob.url }>link</a> }
                          <Link class="button" href={`/series/${ series.id }`}>View</Link>
    </div>
</div>
              </div>
          </div>);
  }
}
