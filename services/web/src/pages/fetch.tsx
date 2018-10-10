import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";

import { getStatus } from "../actions/fetch";
import "../model";
import store, { IReduxState } from "../store";

interface IFetchProps {
    loading: boolean;
    status: any;
}

class Fetch extends React.PureComponent<IFetchProps> {

    constructor(props) {
        super(props);

        this.renderStatus = this.renderStatus.bind(this);
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
                    <a className="button is-success" href="javascript:void(0)">
                        Delete
                    </a>
                </div>
            </div>
        );
    }

    private renderStatus() {
        return [
            "api",
            "transmission",
        ].map((s, n) => {
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
    status: state.fetch.status,
}))(Fetch);
