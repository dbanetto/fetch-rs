import { h, Component } from 'preact';
import { route, Link } from 'preact-router';
import Store from '../store';
import handler from './handler';
import '../model';

interface ViewState {
    series: Series;
    info: Array<InfoBlob>;
}

interface ViewProps {
    matches?: {
        id: number;
    };
    path: string;
}

export default class View extends Component<ViewProps, ViewState> {

  constructor() {
    super();

    this.state = {
      series: null,
      info: null,
    };
  }

  componentDidMount() {
    this.getSeries();
  }

  getSeries() {
    let self = this;

    let queries: [Promise<Series>, Promise<Array<InfoBlob>>] = [
      Store.getSeriesId(this.props.matches.id),
      Store.getSeriesInfo(this.props.matches.id)
    ];

    Promise.all(queries)
      .then(result => {
        self.setState({
          series: result[0],
          info: result[1]
        });
      })
      .catch(err => {
        console.log(err);
        route('/');
      });
  }

  handleDelete() {
    let self = this;

    let confirmed = confirm(`Are you sure you want to delete ${ this.state.series.title }?`);
    if (!confirmed) {
      return;
    }

    Store.deleteSeriesId(this.props.matches.id)
      .then(() => {
        route('/');
      })
    .catch(err => {
      console.log(err);
      route('/');
    });

  }

  renderInfoList() {

    if (this.state.info == undefined) {
      return (<div></div>);
    }

    return (
            <div>
              <ul>
                { this.state.info.map((u, i) =>
                <li key={i.toString()} >
                  { handler.build(u.blob, u.info_type, {}) }
                </li>
                ) }

              </ul>
            </div>
        );
  }

  render() {
    if (this.state.series === null) {
      return (
          <div>
            <p>loading...</p>
            <Link href='/'>back</Link>
          </div>);
    }

    let series = this.state.series;

    return (
        <div>
          <div>
            <h1>{ series.title }</h1>
            <p><img src={ series.poster_url }/></p>
            { this.renderInfoList() }
          </div>
          <div>
            <span>
              <Link href={ `/series/${ series.id }/edit` }>edit</Link>
            </span>
            <span>
              <a href="javascript:void(0)" onClick={ this.handleDelete.bind(this) }>delete</a>
            </span>
            <span>
              <Link href='/'>back</Link>
            </span>
          </div>
        </div>
        );
  }
}
