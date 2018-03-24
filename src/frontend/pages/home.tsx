import { Component, h } from "preact";
import { Link } from "preact-router";
import { getSeries } from "../api";
import SeriesCard from "../components/seriesCard";
import "../model";

interface IHomeState {
    series: ISeries[];
}

interface IHomeProps {
    path: string;
}

export default class Home extends Component<IHomeProps, IHomeState> {

    constructor() {
        super();

        this.state = {
            series: [],
        };
    }

    public componentDidMount() {
        this.loadSeries();
    }

    public render() {
        return (
            <div>
                {this.renderSeries()}
            </div>
        );
    }

    private loadSeries() {
        getSeries()
            .then((series) => {
                this.setState({
                    series,
                });
            })
            .catch(alert);
    }

    private renderSeries() {
        if (this.state && this.state.series) {
            return (
                <div class="card-list tile is-parent">
                    {this.state.series.map((i) => <SeriesCard key={i.id} series={i} />)}
                </div>
            );
        } else {
            return (<span>loading...</span>);
        }
    }
}
