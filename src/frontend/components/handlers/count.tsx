import { h, Component } from 'preact';
import '../../model';

export default class CountHandler extends Component<HandlerProps, void> {

    static TypeName(): string {
        return "Count";
    }

    handleUpdate(event) {
        let state = this.props.blob;
        state[event.target.attributes['label'].value] = event.target.value;

        console.log(event.target);
        console.log(state);
        this.props.handleUpdate(state);
    }

    renderView() {
        return (
            <div class="columns">
                <div class="column">
                    <label class="label" for="current">Current count</label>
                    <input label="current" disabled class="input" type="number" value={ this.props.blob.current } />
                </div>
                <div class="column">
                    <label class="label" for="total">Total count</label>
                    <input label="total" disabled class="input" type="number" value={ this.props.blob.total } />
                </div>
            </div>);
    }

    renderEdit() {
        return (
            <div class="columns">
                <div class="column">
                    <label class="label" for="current">Current count</label>
                    <input label="current" class="input" type="number"
                        value={ this.props.blob.current }
                        min="0"
                        max={ this.props.blob.total > 0 ?  this.props.blob.total : false }
                        onChange={ this.handleUpdate.bind(this) } />
                </div>
                <div class="column">
                    <label class="label" for="total">Total count</label>
                    <input label="total" class="input" type="number"
                        value={ this.props.blob.total }
                        min={ this.props.blob.current }
                        onChange={ this.handleUpdate.bind(this) } />
                </div>
            </div>);
    }

    render() {
        return this.props.edit ? this.renderEdit() : this.renderView();
    }
}
