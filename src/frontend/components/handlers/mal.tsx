import { h, Component } from 'preact';
import '../../model';

export default class MalHandler extends Component<HandlerProps, void> {

    static TypeName(): string {
        return "MAL";
    }

    handleUpdate(event) {
        let state = this.props.blob;

        state[event.target.attributes['label'].value] = parseInt(event.target.value, 10);

        this.props.handleUpdate(state);
    }

    renderView() {
        return (
            <div class="columns">
                <div class="column">
                    <label class="label">MAL ID</label>
                    <a href={ `https://myanimelist.net/anime/${ this.props.blob.id }` } target="_blank" rel="noopener noreferrer">{ this.props.blob.id } <span class="icon is-small"><i class="mdi mdi-open-in-new" /></span></a>
                </div>
                <div class="column">
                    <label class="label">Episode Offset</label>
                    <input disabled class="input" type="number" value={ this.props.blob.offset } />
                </div>
            </div>);
    }

    renderEdit() {
        return (
        <div class="columns">
            <div class="column">
                <label class="label" for="id">MAL id</label>
                <input label="id" class="input" type="number"
                    value={ this.props.blob.id }
                    min="0"
                    onChange={ this.handleUpdate.bind(this) } />
            </div>
            <div class="column">
                <label class="label" for="offset">Episode count offset</label>
                <input label="offset" class="input" type="number"
                    value={ this.props.blob.offset }
                    min="0"
                    onChange={ this.handleUpdate.bind(this) } />
            </div>
        </div>);
    }

    render() {
        return this.props.edit ? this.renderEdit() : this.renderView();
    }
}
