import { h, Component } from 'preact';


export default class UriDefault extends Component {

  constructor() {
    super();
  }

  static name() {
    return "Link";
  }

  renderView() {
    return ( <a href={ this.props.uri }>{ this.props.uri }</a>);
  }

  renderEdit() {
    return ( <input type="url" name={ this.props.name || 'uri' } value={ this.props.uri } onChange={ this.props.handleUpdate }  />);
  }

  render() {
    return this.props.edit ? this.renderEdit() : this.renderView();
  }
}
