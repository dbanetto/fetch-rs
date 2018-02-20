import { h, Component } from 'preact';
import { Link } from 'preact-router';
import SeriesCard from '../components/seriesCard';
import Store from '../store';
import '../model';

interface HomeState {
    series: Array<Series>;
}

interface HomeProps {
    path: string;
}

export default class Home extends Component<HomeProps, HomeState> {
  constructor() {
    super();

    this.state = {
      series: [],
    };
  }

  componentDidMount() {
    this.loadSeries();
  }

  loadSeries() {
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
        return (
            <div className="columns tile is-parent">
                <ul class="card-list">
                    { this.state.series.map(i => <li class="has-gap">
                        <SeriesCard key={i.id} series={i} />
                        </li>) }
                </ul>
            </div>
        );
    } else {
      return (<span>loading...</span>);
    }
  }

    render() {
        return (
            <div>
                { this.renderSeries() }
            </div>
        );
  }
}
