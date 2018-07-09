import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";
import { Link } from "react-router-dom";

import { upsertSeries } from "../actions";
import "../model";
import { IReduxState } from "../store";
import handler from "./handler";
import InfoList from "./infoList";

interface IFormProps {
    dispatch: (action: any) => void;
    loading: boolean;
    series?: SeriesFull;
    back: string;
}

interface IFormState {
    series: SeriesFull;
}

class SeriesForm extends React.Component<IFormProps, IFormState> {

    constructor(props) {
        super(props);

        let series = {
            id: null,
            info: [],
            poster_url: "",
            title: "",
        };
        if (props && props.series) {
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
                    <label className="label" htmlFor="title">Title</label>
                    <input
                        className="input"
                        name="title"
                        id="title"
                        type="text"
                        value={series.title.toString()}
                        required={true}
                        onChange={this.handleTitleUpdate}
                    />
                </div>
                <div>
                    <label className="label" htmlFor="poster_url">Poster URL</label>
                    {poster}
                </div>
                <div>
                    <h3 className="subtitle">Info</h3>
                    <InfoList value={series.info || []} handleUpdate={this.handleInfoUpdate} />
                </div>
                <br />
                <div>
                    <Link className="button" to={this.props.back}>Back</Link>
                    <button className={submitClass} type="submit">
                        {submitText}
                    </button>
                </div>
            </form>
        );
    }

    private handleSubmit(event) {
        event.preventDefault();

        const formData = this.state.series;

        this.props.dispatch(upsertSeries(formData));

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

    private renderId() {
        if (this.state.series.id) {
            return (<div>
                <input name="id" id="id" type="hidden" value={this.state.series.id.toString()} />
            </div>);
        } else {
            return (<div />);
        }
    }
}

export default connect((state: IReduxState, props: {back?: string, series?: object}) => ({
    back: props.back,
    loading: state.series.loading,
    series: props.series,
}))(SeriesForm);
