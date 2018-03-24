import { Component, h } from "preact";
import { Link, route } from "preact-router";
import { upsertSeries } from "../api";
import "../model";
import handler from "./handler";
import InfoList from "./infoList";

interface IFormProps {
    series?: SeriesFull;
    back: string;
}

interface IFormState {
    series: SeriesFull;
}

export default class SeriesForm extends Component<IFormProps, IFormState> {

    constructor(props) {
        super();

        let series = {
            id: null,
            info: [],
            poster_url: "",
            title: "",
        };
        if (props &&  props.series) {
            series = props.series;
        }

        this.state = {
            series,
        };

        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleTitleUpdate = this.handleTitleUpdate.bind(this);
        this.handleUpdatePoster = this.handleUpdatePoster.bind(this);
        this.handleInfoUpdate = this.handleInfoUpdate.bind(this);
    }

    public render() {

        const series = this.state.series;
        const poster = handler.build({ src: series.poster_url },
            "image",
            { edit: true, name: "poster_url", handleUpdate: this.handleUpdatePoster});

        const submitText = this.state.series.id ? "Update" : "Create";
        const submitClass = "is-pulled-right button " + (this.state.series.id ? "is-warning" : "is-success");

        return (
            <form onSubmit={this.handleSubmit}>
                {this.renderId()}
                <div>
                    <label class="label" for="title">Title</label>
                    <input
                        class="input"
                        name="title"
                        id="title"
                        type="text"
                        value={series.title.toString()}
                        required={true}
                        onChange={this.handleTitleUpdate}
                    />
                </div>
                <div>
                    <label class="label" for="poster_url">Poster URL</label>
                    {}
                </div>
                <div>
                    <h3 class="subtitle">Info</h3>
                    <InfoList value={series.info || []} handleUpdate={this.handleInfoUpdate} />
                </div>
                <br />
                <div>
                    <Link class="button" href={this.props.back}>Back</Link>
                    <button class={submitClass} type="submit">
                        {submitText}
                    </button>
                </div>
            </form>
        );
    }

    private handleSubmit(event) {
        event.preventDefault();

        const formData = this.state.series;

        upsertSeries(formData)
            .then((resp) => {
                // redirect to view
                route(`/series/${ resp.id }`, true);
            })
            .catch(alert);

        // stops the HTML form from completing the request
        return false;
    }

    private handleUpdate(key, value) {
        const series = this.state.series;

        value = value.target ? value.target.value : value;

        series[key] = value;

        this.setState({
            series,
        });
    }

    private handleTitleUpdate(value) {
        this.handleUpdate("title", value);
    }

    private handleInfoUpdate(key, value) {
        const series = this.state.series;

        series.info = value;

        this.setState({
            series,
        });
    }

    private handleUpdatePoster(blob) {
        this.handleUpdate("poster_url", blob.src);
    }

    private renderId(): preact.VNode {
        if (this.state.series.id) {
            return (<div>
                <input name="id" id="id" type="hidden" value={this.state.series.id.toString()} />
            </div>);
        } else {
            return (<div />);
        }
    }
}
