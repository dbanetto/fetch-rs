import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";
import { Link } from "react-router-dom";

import { callFetch, getStatus } from "../actions/fetch";
import { getAllSeries } from "../actions/series";
import "../model";
import store, { IReduxState } from "../store";

interface IFetchProps {
    loading: boolean;
    results: any[];
    series: ISeries[];
    status: any;
}

class Fetch extends React.PureComponent<IFetchProps> {

    constructor(props) {
        super(props);

        this.handleFetch = this.handleFetch.bind(this);
        this.renderStatus = this.renderStatus.bind(this);
        this.renderResults = this.renderResults.bind(this);
    }

    public componentWillMount() {
        store.dispatch(getStatus());
    }

    public render() {
      return (
        <div className="container box">
          <div className="is-flex is-flex-horizontal is-flex-spacebetween">
            <div className="is-flex">
              <h1 className="title">Fetch</h1>
            </div>
            <div>
              <a
                className={`button is-success ${this.props.loading ? "is-loading" : ""}`}
                href="javascript:void(0)"
                onClick={this.handleFetch}
              >
                Fetch
              </a>
            </div>
          </div>
          <div>
            {this.renderStatus()}
          </div>
          <div>
            {this.renderResults()}
          </div>
        </div>
      );
    }

    private handleFetch() {
        store.dispatch(callFetch());
        store.dispatch(getAllSeries());
    }

    private renderResults() {
        const shows = this.props.series;
        return this.props.results.map((result, n) => {
            const show = this.props.series.find((s) => s.id === result.id);
            const success = result.success ? "success" : "failed";
            const found = result.found ? `found new, now up to ${result.count}` : "nothing new";
            return [
                (<hr key={n} />),
                (<div
                    key={`${result.id}-${n}`}
                    className="is-flex"
                >
                    <Link to={`/series/${ show.id }`} >
                        <div className="poster">
                            <img className="image" src={show.poster_url} />
                        </div>
                    </Link>
                    <div>
                        <div>
                            <h2 className="title">{show.title}</h2>
                        </div>
                        <div>
                            {success} - {found}
                        </div>
                    </div>
                </div>
                ),
            ];
            });
    }

    private renderStatus() {
      const statuses = [ "api", "transmission" ].map(
        (s, n) => {
          const service = this.props.status[s];
          let colour = "is-static";
          let title = `${s} - unknown`;
          if (service) {
            colour = service.status ? "is-success" : "is-danger";
            title = `${s} - ${service.status ? "up" : `down - ${service.message}`}`;
          }
          return (<span
            key={n}
            title={title}
            className={`button is-small ${colour} ${this.props.loading ? "is-loading" : ""}`}
          >
            {s}
          </span>);
        });

      return (
        <div className="buttons has-addons">
          {statuses}
        </div>);
    }
}

export default connect((state: IReduxState) => ({
    loading: state.fetch.loading,
    results: state.fetch.results,
    series: state.series.items,
    status: state.fetch.status,
}))(Fetch);
