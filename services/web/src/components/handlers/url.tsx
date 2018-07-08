import * as React from "react";
import { render } from "react-dom";
import "../../model";

export default class UriDefault extends React.PureComponent<IHandlerProps> {

    public static TypeName(): string {
        return "Link";
    }

    constructor(props) {
        super(props);

        this.handleUpdate = this.handleUpdate.bind(this);
    }

    public render() {
        return this.props.edit ? this.renderEdit() : this.renderView();
    }

    private renderView() {
        return (<a
            href={this.props.blob.url}
            target="_blank"
            rel="noopener noreferrer"
        >
            {this.props.blob.url}
            <i className="mdi mdi-open-in-new" />
        </a>);
    }

    private handleUpdate(event) {
        this.props.handleUpdate({ url: event.target.value });
    }

    private renderEdit() {
        return (<input
            className="input"
            type="url"
            name={this.props.name || "url"}
            value={this.props.blob.url}
            onChange={this.handleUpdate}
        />);
      }
}
