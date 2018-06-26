import { Component, h } from "preact";
import { connect } from "preact-redux";
import { Link, route } from "preact-router";
import { deleteSeries, getInfoBlobs, getSeries, showError } from "../actions";
import handler from "../components/handler";
import "../model";
import store from "../store";

interface IViewProps {
    matches?: {
        id: number;
    };
    loading: boolean;
    path: string;
    series: ISeries;
    info: IInfoBlob[];
}

class View extends Component<any, IViewProps> {

    constructor(props) {
        super(props);

        this.handleDelete = this.handleDelete.bind(this);
    }

    public componentWillMount() {
        store.dispatch(getSeries(this.props.matches.id));
        store.dispatch(getInfoBlobs(this.props.matches.id));
    }

    public render() {
        if (this.props.loading || !this.props.series || !this.props.info) {
            return (
                <div class="container box">
                    <p>Loading...</p>
                    <Link class="button" href="/">Back</Link>
                </div>);
        }

        const series = this.props.series;

        return (
            <div class="container box">
                <div>
                    <div>
                        <h1 class="title">{series.title}</h1>
                    </div>
                    <div class="columns">
                        <div class="column">
                            {this.renderInfoList()}
                        </div>
                        <div class="column">
                            <img class="image" src={series.poster_url}/>
                        </div>
                    </div>
                </div>
                <div class="is-flex">
                    <div class="has-gap">
                        <Link class="button" href="/">Back</Link>
                    </div>
                    <div class="has-gap">
                        <Link class="button is-warning" href={`/series/${ series.id }/edit`}>Edit</Link>
                    </div>
                    <div class="has-gap margin-right">
                        <a class="button is-danger" href="javascript:void(0)" onClick={this.handleDelete}>
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

        store.dispatch(deleteSeries(this.props.matches.id));
        route("/");
    }

    private renderInfoList() {
        if (this.props.loading) {
            return (<div />);
        }

        const infoItems = this.props.info.map((u, i) =>
            <div key={i.toString()} class="info-list-item" >
                {handler.build(u.blob, u.info_type, {})}
            </div>);

        return (
            <div>
                <div class="info-list">
                    {infoItems}
                </div>
            </div>
        );
  }
}

export default connect((state, props: any) => {

    let series;
    if (state.series && state.series.items && Array.isArray(state.series.items)) {
        series = state.series.items.find((s) => s.id.toString() === props.matches.id);
    }

    let info = [];
    if (state.infoBlob && state.infoBlob.blobs) {
        info = state.infoBlob.blobs[props.matches.id];
    }

    return {
        info,
        loading: state.series.loading,
        series,
    };
})(View);
