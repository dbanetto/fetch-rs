import { Component, h } from "preact";
import "../../model";

export default class UriDefault extends Component<IHandlerProps, void> {

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
            <i class="mdi mdi-open-in-new" />
        </a>);
    }

    private handleUpdate(event) {
        this.props.handleUpdate({ url: event.target.value });
    }

    private renderEdit() {
        return (<input
            class="input"
            type="url"
            name={this.props.name || "url"}
            value={this.props.blob.url}
            onChange={this.handleUpdate}
        />);
      }
}
