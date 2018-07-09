import * as React from "react";
import { render } from "react-dom";
import "../../model";

export default class NyaaHandler extends React.PureComponent<IHandlerProps> {

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

        const update = {};
        update[event.target.attributes.id.value] = event.target.value;

        this.props.handleUpdate({ ...state, ...update });
    }

    private renderView() {
        return (
            <div className="columns">
                <div className="column">
                    <label className="label" >Nyaa ID</label>
                    <a
                        href={`https://nyaa.si/user/${ this.props.blob.user_id }`}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {this.props.blob.user_id}
                        <span className="icon is-small">
                            <i className="mdi mdi-open-in-new" />
                        </span>
                    </a>
                </div>
                <div className="column">
                    <label className="label">Query</label>
                    <input
                        disabled={true}
                        className="input"
                        type="text"
                        value={this.props.blob.query}
                    />
                </div>
                <div className="column">
                    <label className="label">Search Title</label>
                    <input
                        disabled={true}
                        className="input"
                        type="text"
                        value={this.props.blob.search_title}
                    />
                </div>
            </div>);
    }

    private renderEdit() {
        return (
        <div className="columns">
            <div className="column">
                <label className="label" htmlFor="user_id">Nyaa ID</label>
                <input
                    id="user_id"
                    className="input"
                    type="text"
                    value={this.props.blob.user_id}
                    onChange={this.handleUpdate}
                />
            </div>
            <div className="column">
                <label className="label" htmlFor="query">Query</label>
                <input
                    id="query"
                    className="input"
                    type="text"
                    value={this.props.blob.query}
                    onChange={this.handleUpdate}
                />
            </div>
            <div className="column">
                <label className="label" htmlFor="search_title">Search Title</label>
                <input
                    id="search_title"
                    className="input"
                    type="text"
                    value={this.props.blob.search_title}
                    onChange={this.handleUpdate}
                />
            </div>
        </div>);
    }
}
