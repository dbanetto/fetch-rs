import * as React from "react";
import { render } from "react-dom";
import "../../model";

export default class CountHandler extends React.PureComponent<IHandlerProps> {

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
            <div className="columns">
                <div className="column">
                    <label className="label" htmlFor="current">Current count</label>
                    <input
                        disabled={true}
                        className="input"
                        type="number"
                        value={this.props.blob.current}
                    />
                </div>
                <div className="column">
                    <label className="label" htmlFor="total">Total count</label>
                    <input
                        disabled={true}
                        className="input"
                        type="number"
                        value={this.props.blob.total}
                    />
                </div>
            </div>);
    }

    private renderEdit() {
        return (
            <div className="columns">
                <div className="column">
                    <label className="label" htmlFor="current">Current count</label>
                    <input
                        className="input"
                        type="number"
                        value={this.props.blob.current}
                        min="0"
                        max={this.props.blob.total > 0 ? this.props.blob.total : undefined}
                        onChange={this.handleUpdate}
                    />
                </div>
                <div className="column">
                    <label className="label" htmlFor="total">Total count</label>
                    <input
                        className="input"
                        type="number"
                        value={this.props.blob.total}
                        min={this.props.blob.total > 0 ? this.props.blob.current : undefined}
                        onChange={this.handleUpdate}
                    />
                </div>
            </div>);
    }

}
