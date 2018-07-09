import * as React from "react";
import { render } from "react-dom";
import "../../model";

export default class MalHandler extends React.PureComponent<IHandlerProps> {

    public static TypeName(): string {
        return "MAL";
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

        const update = {};
        update[event.target.attributes.id.value] = parseInt(event.target.value, 10);

        this.props.handleUpdate({...state, ...update});
    }

    private renderView() {
        return (
            <div className="columns">
                <div className="column">
                    <label className="label">MAL ID</label>
                    <a
                        href={`https://myanimelist.net/anime/${ this.props.blob.id }`}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {this.props.blob.id}
                        <span className="icon is-small">
                            <i className="mdi mdi-open-in-new" />
                        </span>
                    </a>
                </div>
                <div className="column">
                    <label className="label">Episode Offset</label>
                    <input
                        disabled={true}
                        className="input"
                        type="number"
                        value={this.props.blob.offset}
                    />
                </div>
            </div>);
    }

    private renderEdit() {
        return (
        <div className="columns">
            <div className="column">
                <label className="label" htmlFor="id">MAL id</label>
                <input
                    id="id"
                    className="input"
                    type="number"
                    value={this.props.blob.id}
                    min="0"
                    onChange={this.handleUpdate}
                />
            </div>
            <div className="column">
                <label className="label" htmlFor="offset">Episode count offset</label>
                <input
                    id="offset"
                    className="input"
                    type="number"
                    value={this.props.blob.offset}
                    min="0"
                    onChange={this.handleUpdate}
                />
            </div>
        </div>);
    }

}
