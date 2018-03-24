import { Component, h } from "preact";
import { Link, route } from "preact-router";
import handler from "../components/handler";
import "../model";
import Store from "../store";

interface IViewState {
    series: ISeries;
    info: IInfoBlob[];
}

interface IViewProps {
    matches?: {
        id: number;
    };
    path: string;
}

export default class View extends Component<IViewProps, IViewState> {

    constructor(props) {
        super(props);

        this.state = {
            info: null,
            series: null,
        };

        this.handleDelete = this.handleDelete.bind(this);
    }

    public componentDidMount() {
        this.getSeries();
    }

    public render() {
        if (this.state.series === null) {
            return (
                <div class="container box">
                    <p>Loading...</p>
                    <Link class="button" href="/">Back</Link>
                </div>);
        }

        const series = this.state.series;

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

    private getSeries() {
        const queries: [Promise<ISeries>, Promise<IInfoBlob[]>] = [
            Store.getSeriesId(this.props.matches.id),
            Store.getSeriesInfo(this.props.matches.id),
        ];

        Promise.all(queries)
            .then((result) => {
                this.setState({
                    info: result[1],
                    series: result[0],
                });
            })
            .catch((err) => {
                console.log(err);
                route("/");
            });
    }

    private handleDelete() {
        const confirmed = confirm(`Are you sure you want to delete ${ this.state.series.title }?`);
        if (!confirmed) {
            return;
        }

        Store.deleteSeriesId(this.props.matches.id)
            .then(() => {
                route("/");
            })
            .catch((err) => {
                console.log(err);
                route("/");
            });

    }

    private renderInfoList() {

        if (this.state.info === undefined) {
            return (<div />);
        }
        const infoItems = this.state.info.map((u, i) =>
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
