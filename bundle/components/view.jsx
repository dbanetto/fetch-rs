import { h, Component } from 'preact';
import { route, Link } from 'preact-router';
import Store from '../store.js';

export default class View extends Component {

  constructor() {
    super();

    this.state = {
      series: null,
      uri: null,
    };
  }

  componentDidMount() {
    this.getSeries();
  }

  getSeries() {
    let self = this;

    console.log(this);
    console.log(this.props);

    Promise.all([Store.getSeriesId(this.props.matches.id),
        Store.getSeriesUri(this.props.matches.id)])
      .then(result => {
        self.setState({
          series: result[0],
          uri: result[1]
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

  renderUriList() {

    if (this.state.uri == undefined) {
      return (<div></div>);
    }

    return (
            <div>
              <ul>
                { this.state.uri.map((u, i) =>
                <li key={i}>
                  <a href={u.uri} className={ `link link-${u.primary ? 'primary' : 'other'}` }>{ u.uri }</a>
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
            { this.renderUriList() }
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
