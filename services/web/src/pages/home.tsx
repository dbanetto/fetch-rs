import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";
import { getAllSeries } from "../actions";
import SeriesCard from "../components/seriesCard";
import "../model";
import store, { IReduxState } from "../store";

interface IHomeProps {
    loading: boolean;
    path: string;
    series: ISeries[];
}

class Home extends React.PureComponent<IHomeProps> {

    constructor(props) {
        super(props);
    }

    public componentWillMount() {
        store.dispatch(getAllSeries());
    }

    public render() {
        return (
            <div>
                {this.renderSeries()}
            </div>
        );
    }

    private renderSeries() {
        if (this.props.loading) {
            return (
                <div className="box">
                    <span>loading...</span>
                </div>
            );
        } else {
            return (
                <div className="columns card-list tile is-parent">
                    {this.props.series.map((i) => <div key={i.id} className="column"><SeriesCard series={i} /></div>)}
                </div>
            );
        }
    }
}

export default connect((state: IReduxState) => ({
    loading: state.series.loading,
    series: state.series.items,
}))(Home);
