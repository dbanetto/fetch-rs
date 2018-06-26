import { Component, h } from "preact";
import { connect } from "preact-redux";
import { Link, route } from "preact-router";
import { getInfoBlobs, getSeries } from "../actions";
import SeriesForm from "../components/seriesForm";
import "../model";

interface IEditProps {
    path: string;
    loading: boolean;
    series: ISeries;
    info: IInfoBlob[];
    matches?: {
        id: number;
    };
}

class Edit extends Component<any, IEditProps> {

    constructor() {
        super();
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
                <div class="container box">
                    <p>Loading...</p>
                    <Link class="button" href="/">Back</Link>
                </div>);
        }

        const series: any = this.props.series;
        series.info = this.props.info;

        return (
            <div class="container box">
                <SeriesForm series={series} back={`/series/${ this.props.series.id }`} />
            </div>
        );
    }

    private getSeries() {
        this.props.dispatch(getSeries(this.props.matches.id));
        this.props.dispatch(getInfoBlobs(this.props.matches.id));
    }
}

export default connect((state, props: any) => {
    let series;
    if (state.series.items) {
        series = state.series.items.find((s) => s.id.toString() === props.matches.id);
    }

    let info = state.infoBlob.blobs[props.matches.id];
    if (!info) {
        info = [];
    }

    return {
        info,
        loading: state.series.loading,
        series,
    };
})(Edit);
