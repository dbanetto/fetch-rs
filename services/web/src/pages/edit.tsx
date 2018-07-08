import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";
import { Link, Route } from "react-router-dom";

import { getInfoBlobs, getSeries } from "../actions";
import SeriesForm from "../components/seriesForm";
import "../model";
import { IReduxState } from "../store";

interface IEditProps {
    dispatch: (action: any) => void;
    path: string;
    loading: boolean;
    series: ISeries;
    info: IInfoBlob[];
    match?: {
        params: {
            id: number;
        };
    };
}

class Edit extends React.PureComponent<IEditProps> {

    constructor(props) {
        super(props);
    }

    public componentDidMount() {
        this.getSeries();
    }

    public componentDidUpdate(prevProps) {
        if (!this.props.loading && (!this.props.series || !this.props.info)) {
            this.getSeries();
        }
    }

    public render() {
        if (this.props.loading || !this.props.series) {
            return (
                <div className="container box">
                    <p>Loading...</p>
                    <Link className="button" to="/">Back</Link>
                </div>);
        }

        const series: any = this.props.series;
        series.info = this.props.info;

        return (
            <div className="container box">
                <SeriesForm series={series} back={`/series/${ this.props.series.id }`} />
            </div>
        );
    }

    private getSeries() {
        this.props.dispatch(getSeries(this.props.match.params.id));
        this.props.dispatch(getInfoBlobs(this.props.match.params.id));
    }
}

export default connect((state: IReduxState, props: any) => {
    let series;
    if (state.series.items) {
        series = state.series.items.find((s) => s.id.toString() === props.match.params.id);
    }

    let info = state.infoBlob.blobs[props.match.params.id];

    if (!info) {
        info = [];
    }

    return {
        info,
        loading: state.series.loading,
        series,
    };
})(Edit);
