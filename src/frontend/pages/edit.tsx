import { Component, h } from "preact";
import { Link, route } from "preact-router";
import SeriesForm from "../components/seriesForm";
import "../model";
import Store from "../store";

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
            Store.getSeriesId(this.props.matches.id),
            Store.getSeriesInfo(this.props.matches.id),
        ]).then((result) => {

            this.setState({
                info: result[1],
                series: result[0],
            });
        }).catch((err) => {
            console.log(err);
            route("/");
        });
    }
}
