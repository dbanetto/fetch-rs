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
                  <h2 class="subtitle">{ series.title }</h2>
                  <div>
                      &nbsp;
                      <div class="is-pulled-right">
                          { this.state.link && <a class="button" href={ this.state.link.blob.url }>link</a> }
                          <Link class="button" href={`/series/${ series.id }`}>View</Link>
                      </div>
                  </div>
              </div>
          </div>);
  }
}
