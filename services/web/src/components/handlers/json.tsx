import { Component, h } from "preact";
import "../../model";

interface IJsonHandlerState {
    isValid: boolean;
    text: string;
}

export default class JsonHandler extends Component<IHandlerProps, IJsonHandlerState> {

    public static TypeName(): string {
        return "JSON";
    }

    constructor(props) {
        super(props);

        this.state = {
            isValid: true,
            text: JSON.stringify(props.blob),
        };

        this.handleUpdate = this.handleUpdate.bind(this);
    }

    public render() {
        return this.props.edit ? this.renderEdit() : this.renderView();
    }

    private textareaClass() {
        return "textarea" + this.state.isValid ? " is-success" : " is-danger";
    }

    private renderView() {
        return (<textarea
            class="textarea"
            disabled={true}
            name={this.props.name}
            value={JSON.stringify(this.props.blob)}
            onChange={this.handleUpdate}
        />);
    }

    private handleUpdate(event) {
        let isValid = true;
        try {
            const value = JSON.parse(event.target.value);
            this.props.handleUpdate(value);
        } catch (err) {
            isValid = false;
        }

        this.setState({
            isValid,
            text: event.target.value,
        });
    }

    private renderEdit() {
        return (<textarea
            class={this.textareaClass()}
            name={this.props.name}
            value={this.state.text}
            onChange={this.handleUpdate}
        />);
    }

}
