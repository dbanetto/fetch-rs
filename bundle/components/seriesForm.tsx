import { h, Component } from 'preact';
import { route } from 'preact-router';
import InfoList from './infoList';
import Store from '../store';
import handler from './handler';
import '../model';

interface FormProps {
    series?: SeriesFull;
}

interface FormState {
    series: SeriesFull;
}

export default class SeriesForm extends Component<FormProps, FormState> {

    constructor(props) {
        super();

        var series = {
            id: null,
            title: '',
            poster_url: '',
            info: [],
        };
        if (props &&  props.series) {
            series = props.series;
        }

        this.state = {
            series: series
        }
    }

    validate(formData) {
        let errors = [];

        let blobs = formData.blobs ? formData.blobs : [];

        return errors;
    }

    handleSubmit(event) {
        event.preventDefault();

        let formData = this.state.series;

        let errors = this.validate(formData);
        if (errors.length > 0) {
            console.log(errors);
            // TODO: report errors
            return;
        }

        let self = this;

        Store.upsertSeries(formData)
            .then(resp => {
                // redirect to view
                route(`/series/${ resp.id }`, true);
            })
            .catch(alert);

        // stops the HTML form from completing the request
        return false;
    }

    handleUpdate(key, value) {
        let series = this.state.series;

        value = value.target ? value.target.value : value;

        series[key] = value;

        this.setState({
            series: series
        });
    }

    handleInfoUpdate(key, value) {
        let series = this.state.series;

        series.info = value;

        this.setState({
            series: series
        });
    }

    handleUpdatePoster(blob) {
        this.handleUpdate('poster_url', blob.src);
    }

    renderId(): preact.VNode {
        if (this.state.series.id) {
            return (<div>
                <input name="id" id="id" type="hidden" value={ this.state.series.id.toString() } />
            </div>);
        } else{
            return (<div></div>);
        }
    }

    render() {
        let series = this.state.series;
        return (
            <form onSubmit={this.handleSubmit.bind(this)}>
                { this.renderId() }
                <div>
                    <label class="label" for="title">Title</label>
                    <input class="input" name="title" id="title" type="text" value={series.title.toString()} required
                        onChange={ this.handleUpdate.bind(this, 'title') } />
                </div>
                <div>
                    <label class="label" for="poster_url">Poster URL</label>
                    { handler.build({ src: series.poster_url }, 'image', { edit: true, name: 'poster_url',
                    handleUpdate: this.handleUpdatePoster.bind(this) }) }
                </div>
                <div>
                    <h3 class="subtitle">Info</h3>.
                    <InfoList value={series.info || []}
                        handleUpdate={ this.handleInfoUpdate.bind(this) } />
                </div>
                <br />
                <div>
                    <button class={ 'button ' + (this.state.series.id ? 'is-warning' : 'is-success') } type="submit">
                        { this.state.series.id ? "Update" : "Create" }
                    </button>
                </div>
            </form>
        );
}
}
