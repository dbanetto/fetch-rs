import { Component,  h } from "preact";
import "../../model";

export default class CountHandler extends Component<IHandlerProps, void> {

    public static TypeName(): string {
        return "Count";
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
                    <label class="label" for="current">Current count</label>
                    <input
                        label="current"
                        disabled={true}
                        class="input"
                        type="number"
                        value={this.props.blob.current}
                    />
                </div>
                <div class="column">
                    <label class="label" for="total">Total count</label>
                    <input
                        label="total"
                        disabled={true}
                        class="input"
                        type="number"
                        value={this.props.blob.total}
                    />
                </div>
            </div>);
    }

    private renderEdit() {
        return (
            <div class="columns">
                <div class="column">
                    <label class="label" for="current">Current count</label>
                    <input
                        label="current"
                        class="input"
                        type="number"
                        value={this.props.blob.current}
                        min="0"
                        max={this.props.blob.total > 0 ?  this.props.blob.total : false}
                        onChange={this.handleUpdate}
                    />
                </div>
                <div class="column">
                    <label class="label" for="total">Total count</label>
                    <input
                        label="total"
                        class="input"
                        type="number"
                        value={this.props.blob.total}
                        min={this.props.blob.total > 0 ? this.props.blob.current : false}
                        onChange={this.handleUpdate}
                    />
                </div>
            </div>);
    }

}
