import { Component, h } from "preact";
import { connect } from "preact-redux";
import { Link } from "preact-router";
import { getAllSeries } from "../actions";
import SeriesCard from "../components/seriesCard";
import "../model";
import store from "../store";

interface IHomeProps {
    loading: boolean;
    path: string;
    series: ISeries[];
}

class Home extends Component<any, IHomeProps> {

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
                <div class="box">
                    <span>loading...</span>
                </div>
            );
        } else {
            return (
                <div class="columns card-list tile is-parent">
                    {this.props.series.map((i) => <div key={i.id} class="column"><SeriesCard series={i} /></div>)}
                </div>
            );
        }
    }
}

export default connect((state) => ({
    loading: state.series.loading,
    series: state.series.items,
}))(Home);
