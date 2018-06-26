import { Component, h } from "preact";
import "../../model";

export default class KitsuHandler extends Component<IHandlerProps, void> {

    public static TypeName(): string {
        return "Kitsu";
    }

    constructor(props) {
        super(props);

        this.handleUpdate = this.handleUpdate.bind(this);
    }

    public render() {
        return this.props.edit ? this.renderEdit() : this.renderView();
    }

    private handleUpdate(event) {
        const state = this.props.blob;

        state[event.target.attributes.label.value] = parseInt(event.target.value, 10);

        this.props.handleUpdate(state);
    }

    private renderView() {
        return (
            <div class="columns">
                <div class="column">
                    <label class="label">Kitsu id</label>
                    <a
                        href={`https://kitsu.io/anime/${ this.props.blob.id }`}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {this.props.blob.id}
                        <span class="icon is-small">
                            <i class="mdi mdi-open-in-new" />
                        </span>
                    </a>
                </div>
                <div class="column">
                    <label class="label">Episode Offset</label>
                    <input
                        disabled={true}
                        class="input"
                        type="number"
                        value={this.props.blob.offset}
                    />
                </div>
            </div>);
    }

    private renderEdit() {
        return (
        <div class="columns">
            <div class="column">
                <label class="label" for="id">Kitsu id</label>
                <input
                    label="id"
                    class="input"
                    type="number"
                    value={this.props.blob.id}
                    min="0"
                    onChange={this.handleUpdate}
                />
            </div>
            <div class="column">
                <label class="label" for="offset">Episode count offset</label>
                <input
                    label="offset"
                    class="input"
                    type="number"
                    value={this.props.blob.offset}
                    min="0"
                    onChange={this.handleUpdate}
                />
            </div>
        </div>);
    }

}
