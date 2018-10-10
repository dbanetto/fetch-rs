import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";

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
                <div className="is-flex">
                    <h1 className="title">Fetch</h1>
                    {this.renderStatus()}
                </div>
                <div>
                    <a
                        className="button is-success"
                        href="javascript:void(0)"
                        onClick={this.handleFetch}
                    >
                        Fetch
                    </a>

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
            return (<div key={`${result.id}-${n}`}>
                {show.title} - {result.success} - {result.found} - {result.count}
            </div>);
            });
    }

    private renderStatus() {
        return [
            "api",
            "transmission",
        ].map(
            (s, n) => {
            const service = this.props.status[s];
            let colour = "unknown";
            let title = `${s} - unknown`;
            if (service) {
                colour = service.status ? "ok" : "err";
                title = `${s} - ${service.status ? "up" : `down - ${service.message}`}`;
            }
            return (
                <div
                    key={n}
                    title={title}
                    className={`status-circle status-${colour}`}
                />
            );
});
    }
}

export default connect((state: IReduxState) => ({
    loading: state.fetch.loading,
    results: state.fetch.results,
    series: state.series.items,
    status: state.fetch.status,
}))(Fetch);
