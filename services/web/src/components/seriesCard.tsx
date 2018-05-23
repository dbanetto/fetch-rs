import { Component, h } from "preact";
import { connect } from "preact-redux";
import { Link } from "preact-router";
import { getInfoBlobType } from "../actions";
import "../model";

interface ICardProps {
    series: ISeries;
    link: IInfoBlob;
    count: IInfoBlob;
}

class SeriesCard extends Component<any, ICardProps> {

  constructor(props) {
    super(props);
  }

  public componentDidMount() {
      const types = [];
      if (!this.props.count) {
          types.push("count");
      }
      if (!this.props.link) {
          types.push("url");
      }
      if (types.length !== 0 && !this.props.loading) {
          this.props.dispatch(getInfoBlobType(this.props.series.id, types));
      }
  }

  public render() {
      const series = this.props.series;
      return (
          <div class="card has-gap">
              <div class="card-head" >
                  <Link href={`/series/${ series.id }`} >
                      <div class="poster">
                          <img class="image" src={series.poster_url} />
                      </div>
                  </Link>
              </div>
              <div class="card-body">
                  <div class="is-flex" >
                      <Link href={`/series/${ series.id }`} class="card-subtitle is-truncated" title={series.title}>
                          <h2 class="subtitle is-truncated">{series.title}</h2>
                      </Link>
                      {this.renderLink()}
                  </div>
                  <br />
                  <div>
                      {this.renderProgressBar()}
                  </div>
              </div>
          </div>);
  }

  private renderProgressBar() {
      if (this.props.count) {
          const value = this.props.count.blob.current;
          const max = this.props.count.blob.total > 0 ? this.props.count.blob.total : value * 2;

          let currentStatus = "is-success";
          if (this.props.count.blob.total <= 0) {
              currentStatus = "is-warning";
          } else if (this.props.count.blob.current === this.props.count.blob.total) {
              currentStatus = "is-link";
          }

          return (
              <div>
                  <progress class={`progress ${currentStatus}`}  value={value} max={max} />
              </div>
          );
      } else {
          return (<div />);
      }
  }

  private renderLink() {

      if (this.props.link) {
          return (
              <a href={this.props.link.blob.url} target="_blank" rel="noopener noreferrer">
                  <span class="icon is-small">
                      <i class="mdi mdi-open-in-new" />
                  </span>
              </a>
          );
      } else {
          return (<div />);
      }
  }
}

export default connect((state, props: any) => {
    const blobs = state.infoBlob.blobs[props.series.id];
    let link = null;
    let count = null;
    if (blobs) {
        link = blobs.find((b) => b.info_type === "url");
        count = blobs.find((b) => b.info_type === "count");
    }

    return {
        count,
        link,
        loading: state.infoBlob.loading,
    };
})(SeriesCard);
