import { Component, h } from "preact";
import "../model";
import handler from "./handler";

interface IInfoProps {
    value: IInfoBlob;
    handleUpdate: (value: any) => void;
    handleDelete: () => void;
}

export default class InfoElement extends Component<IInfoProps, void> {

    constructor(props) {
        super(props);

        this.handleUri = this.handleUri.bind(this);
    }

    public render() {
        const builtHandler = handler.build(
            this.props.value.blob,
            this.props.value.info_type,
            {
                edit: true,
                handleUpdate: this.handleUri,
            });

        return (<div>

            {this.renderId()}

            <div class="columns">
                <div class="column">{builtHandler} </div>

                <div class="column is-one-fifth">
                    <button class="button is-danger" type="button" onClick={this.props.handleDelete}>
                        x
                    </button>
                </div>
            </div>
        </div>);
    }

    private handleUri(blob) {
        const value = this.props.value;

        value.blob = blob;

        this.props.handleUpdate(value);
    }

    private renderId() {
        if (this.props.value.id) {
            return (<input type="hidden" name="id" className="info-element" value={this.props.value.id.toString()} />);
        } else {
            return (<div />);
        }
    }
}
