import { h, Component } from 'preact';


export default class UriDefault extends Component {

  constructor() {
    super();
  }

  static name() {
    return "Link";
  }

  renderView() {
    return ( <a href={ this.props.blob.url }>{ this.props.blob.url }</a>);
  }

  handleUpdate(event) {
    this.props.handleUpdate({ type: "url", url: event.target.value });
  }

  renderEdit() {
    console.log(this.props);
    return ( <input type="url" name={ this.props.blob.name || 'url' } value={ this.props.blob.url }
      onChange={ this.handleUpdate.bind(this) }  />);
  }

  render() {
    return this.props.edit ? this.renderEdit() : this.renderView();
  }
}
