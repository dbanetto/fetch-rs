import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";
import { Link, withRouter } from "react-router-dom";

import { showError } from "../actions/app";
import { getInfoBlobs } from "../actions/infoblob";
import { deleteSeries, getSeries } from "../actions/series";
import handler from "../components/handler";
import "../model";
import store, { IReduxState } from "../store";

interface IViewProps {
    history?: any;
    match?: {
        params: {
            id: number;
        };
    };
    loading: boolean;
    series: ISeries;
    info: IInfoBlob[];
}

class View extends React.PureComponent<IViewProps> {

    constructor(props) {
        super(props);

        this.handleDelete = this.handleDelete.bind(this);
    }

    public componentWillMount() {
        store.dispatch(getSeries(this.props.match.params.id));
        store.dispatch(getInfoBlobs(this.props.match.params.id));
    }

    public render() {
        if (this.props.loading || !this.props.series || !this.props.info) {
            return (
                <div className="container box">
                    <p>Loading...</p>
                    <Link className="button" to="/">Back</Link>
                </div>);
        }

        const series = this.props.series;

        return (
            <div className="container box">
                <div>
                    <div>
                        <h1 className="title">{series.title}</h1>
                    </div>
                    <div className="columns">
                        <div className="column">
                            {this.renderInfoList()}
                        </div>
                        <div className="column">
                            <img className="image" src={series.poster_url}/>
                        </div>
                    </div>
                </div>
                <div className="is-flex">
                    <div className="has-gap">
                        <Link className="button" to="/">Back</Link>
                    </div>
                    <div className="has-gap">
                        <Link className="button is-warning" to={`/series/${ series.id }/edit`}>Edit</Link>
                    </div>
                    <div className="has-gap margin-right">
                        <a className="button is-danger" href="javascript:void(0)" onClick={this.handleDelete}>
                            Delete
                        </a>
                    </div>
                </div>
            </div>
        );
  }

    private handleDelete() {
        // TODO: make this into a modal
        const confirmed = confirm(`Are you sure you want to delete ${ this.props.series.title }?`);
        if (!confirmed) {
            return;
        }

        store.dispatch(deleteSeries(this.props.match.params.id));
        this.props.history.push("/");
    }

    private renderInfoList() {
        if (this.props.loading) {
            return (<div />);
        }

        const infoItems = this.props.info.map((u, i) =>
            <div key={i.toString()} className="info-list-item" >
                {handler.build(u.blob, u.info_type, {})}
            </div>);

        return (
            <div>
                <div className="info-list">
                    {infoItems}
                </div>
            </div>
        );
  }
}

export default withRouter(connect((state: IReduxState, props: any): IViewProps => {
    let series;
    if (state.series && state.series.items && Array.isArray(state.series.items)) {
        series = state.series.items.find((s) => s.id.toString() === props.match.params.id);
    }

    let info = [];
    if (state.infoBlob && state.infoBlob.blobs) {
        info = state.infoBlob.blobs[props.match.params.id];
    }

    return {
        info,
        loading: state.series.loading,
        series,
    };
})(View));
