import { h, Component } from 'preact';
import '../../model';

interface JsonHandlerState {
    isValid: boolean;
    text: string,
};

export default class JsonHandler extends Component<HandlerProps, JsonHandlerState> {

    constructor(props) {
        super(props);

        this.state = {
            isValid: true,
            text: JSON.stringify(props.blob),
        }
    }

    static TypeName(): string {
        return "JSON";
    }

    textareaClass() {
        var className ="textarea";
        if (this.state.isValid === true) {
            className = className + " is-success";
        } else {
            className = className + " is-danger";
        }

        return className;
    }

    renderView() {
        return ( <textarea class="textarea"
            disabled
            name={ this.props.name }
            value={ JSON.stringify(this.props.blob) }
            onChange={ this.handleUpdate.bind(this) }
        />);
    }

    handleUpdate(event) {
        var isValid = true;
        try {
            let value = JSON.parse(event.target.value);
            this.props.handleUpdate(value);
        } catch(err) {
            isValid = false;
        }

        this.setState({
            isValid: isValid,
            text: event.target.value,
        })
    }

    renderEdit() {
        return ( <textarea class={ this.textareaClass() }
            name={ this.props.name }
            value={ this.state.text }
            onChange={ this.handleUpdate.bind(this) }
        />);
    }

    render() {
        return this.props.edit ? this.renderEdit() : this.renderView();
    }
}
