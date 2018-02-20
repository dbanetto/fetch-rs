import { h, Component } from 'preact';
import '../../model';

export default class UriDefault extends Component<HandlerProps, void> {

  static TypeName(): string {
    return "Link";
  }

  renderView() {
    return ( <a href={ this.props.blob.url }>{ this.props.blob.url }</a>);
  }

  handleUpdate(event) {
    this.props.handleUpdate({ url: event.target.value });
  }

  renderEdit() {
    return ( <input
      type="url"
      name={ this.props.name || 'url' }
      value={ this.props.blob.url }
      onChange={ this.handleUpdate.bind(this) }
    />);
  }

  render() {
    return this.props.edit ? this.renderEdit() : this.renderView();
  }
}
