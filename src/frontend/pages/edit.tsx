import { Component, h } from "preact";
import { Link, route } from "preact-router";
import { getSeriesId, getSeriesInfo } from "../api";
import SeriesForm from "../components/seriesForm";
import "../model";

interface IEditState {
    series: ISeries;
    info: IInfoBlob[];
}

interface IEditProps {
    path: string;
    matches?: {
        id: number;
    };
}

export default class Edit extends Component<IEditProps, IEditState> {

    constructor() {
        super();

        this.state = {
            info: null,
            series: null,
        };
    }

    public componentDidMount() {
        this.getSeries();
    }

    public render() {
        if (this.state.series === null) {
            return (
                <div>
                    <p>loading...</p>
                    <Link href="/">back</Link>
                </div>
            );
        }

        const series: any = this.state.series;
        series.info = this.state.info;

        return (
            <div class="container box">
                <SeriesForm series={series} back={`/series/${ this.state.series.id }`} />
            </div>
        );
    }

    private getSeries() {
        Promise.all([
            getSeriesId(this.props.matches.id),
            getSeriesInfo(this.props.matches.id),
        ]).then((result) => {

            this.setState({
                info: result[1],
                series: result[0],
            });
        }).catch((err) => {
            // FIXME: should show an error modal
            route("/");
        });
    }
}
