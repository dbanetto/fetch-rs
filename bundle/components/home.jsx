import { h, Component } from 'preact';
import { Link } from 'preact-router';
import SeriesCard from './seriesCard.jsx';
import Store from '../store.js';

export default class Home extends Component {
  constructor() {
    super();

    this.state = {
      series: Store.getSeriesCache()
    };
  }

  componentDidMount() {
    this.loadSeries();
  }

  loadSeries(useCache) {
    let self = this;
    Store.getSeries()
      .then(series => {
        self.setState({
          series: series
        });
      })
      .catch(alert);
  }

  renderSeries() {
    if (this.state && this.state.series) {
      return (<div className="card-box">
        { this.state.series.map(i => <SeriesCard key={i.id} series={i} />) }
      </div>);
    } else {
      return (<span>loading...</span>);
    }
  }

  render() {
    return (
        <div>
          <h2>Series List</h2>
            { this.renderSeries() }
            <Link href="/series/new">create</Link>
          <button onClick={this.loadSeries.bind(this)}>Reload</button>
        </div>
        );
  }
}
