import * as React from "react";
import { render } from "react-dom";
import "../../model";

export default class ImageHandler extends React.PureComponent<IHandlerProps> {

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
            <div className="columns">
                <div className="column">
                    <input
                        type="url"
                        className="input"
                        name={this.props.name || ""}
                        value={this.props.blob.src}
                        onChange={this.handleUpdate}
                    />
                </div>
                <div className="column">
                    <h3 className="subtitle">Preview</h3>
                    <div className="poster preview">
                        <img className="image" src={this.props.blob.src}/>
                    </div>
                </div>
            </div>);
    }
}
