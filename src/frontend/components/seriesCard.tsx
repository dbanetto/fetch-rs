import { h, Component } from 'preact';
import { Link } from 'preact-router';
import Store from '../store';
import '../model';

interface CardProps {
    series: Series;
}

interface CardState {
    link: InfoBlob;
}

export default class SeriesCard extends Component<CardProps, CardState> {

  constructor() {
    super();

    this.state = {
      link: null
    }
  }

  componentDidMount() {
    let self = this;
    Store.getInfoType(this.props.series.id, ["url"])
      .then(blobs => {
        let link = blobs.find((b) => b.info_type === "url");
        self.setState({
          link: link
        });
      }).catch(() => null);
  }

  renderLink() {

      if (this.state.link) {
          return (
              <a href={ this.state.link.blob.url } target="_blank" rel="noopener noreferrer">
                  <span class="icon is-small">
                      <i class="mdi mdi-open-in-new" />
                  </span>
              </a>
          );
      } else {
          return (<div />)
      }
  }

  render() {
    var series = this.props.series;
      return (
          <div class="card has-gap">
              <div >
                  <Link href={`/series/${ series.id }`} >
                      <div class="poster">
                          <img class="image" src={ series.poster_url } />
                      </div>
                  </Link>
              </div>
              <div class="card-body">
                      <div class="card-title">
                          <h2 class="subtitle is-truncated">{ series.title }</h2>
                          { this.renderLink() }
                      </div>
                  <div>
                      &nbsp;
                      <div class="is-pulled-right">
                          <Link class="button" href={`/series/${ series.id }`}>View</Link>
                      </div>
                  </div>
              </div>
          </div>);
  }
}
