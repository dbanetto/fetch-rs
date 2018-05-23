import { Component, h } from "preact";
import "../../model";

export default class NyaaHandler extends Component<IHandlerProps, void> {

    public static TypeName(): string {
        return "Nyaa";
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

        state[event.target.attributes.label.value] = event.target.value;

        this.props.handleUpdate(state);
    }

    private renderView() {
        return (
            <div class="columns">
                <div class="column">
                    <label class="label" >Nyaa ID</label>
                    <a
                        href={`https://nyaa.si/user/${ this.props.blob.user_id }`}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {this.props.blob.user_id}
                        <span class="icon is-small">
                            <i class="mdi mdi-open-in-new" />
                        </span>
                    </a>
                </div>
                <div class="column">
                    <label class="label">Query</label>
                    <input
                        disabled={true}
                        class="input"
                        type="text"
                        value={this.props.blob.query}
                    />
                </div>
                <div class="column">
                    <label class="label">Search Title</label>
                    <input
                        disabled={true}
                        class="input"
                        type="text"
                        value={this.props.blob.search_title}
                    />
                </div>
            </div>);
    }

    private renderEdit() {
        return (
        <div class="columns">
            <div class="column">
                <label class="label" for="user_id">Nyaa ID</label>
                <input
                    label="user_id"
                    class="input"
                    type="text"
                    value={this.props.blob.user_id}
                    onChange={this.handleUpdate}
                />
            </div>
            <div class="column">
                <label class="label" for="query">Query</label>
                <input
                    label="query"
                    class="input"
                    type="text"
                    value={this.props.blob.query}
                    onChange={this.handleUpdate}
                />
            </div>
            <div class="column">
                <label class="label" for="search_title">Search Title</label>
                <input
                    label="search_title"
                    class="input"
                    type="text"
                    value={this.props.blob.search_title}
                    onChange={this.handleUpdate}
                />
            </div>
        </div>);
    }
}
