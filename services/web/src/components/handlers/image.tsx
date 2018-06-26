import { Component, h } from "preact";
import "../../model";

export default class ImageHandler extends Component<IHandlerProps, void> {

    public static TypeName(): string {
        return "Image";
    }

    constructor(props) {
        super(props);

        this.handleUpdate = this.handleUpdate.bind(this);
    }

    public render() {
        return this.props.edit ? this.renderEdit() : this.renderView();
    }

    private renderView() {
        return (<img src={this.props.blob.src}/>);
    }

    private handleUpdate(event) {
        this.props.handleUpdate({ src: event.target.value });
    }

    private renderEdit() {
        return (
            <div class="columns">
                <div class="column">
                    <input
                        type="url"
                        class="input"
                        name={this.props.name || ""}
                        value={this.props.blob.src}
                        onChange={this.handleUpdate}
                    />
                </div>
                <div class="column">
                    <h3 class="subtitle">Preview</h3>
                    <div class="poster preview">
                        <img class="image" src={this.props.blob.src}/>
                    </div>
                </div>
            </div>);
    }
}
