import { h, Component } from 'preact';


export default class UriImage extends Component {

  constructor() {
    super();
  }

  static name() {
    return "Image";
  }

  renderView() {
    return ( <img src={ this.props.uri } />);
  }

  renderEdit() {
    return (
      <span>
        <input type="url"
          name={ this.props.name || 'uri' }
          value={ this.props.uri }
      onChange={ this.props.handleUpdate }  />
      <div class="preview">
        <span>Preview</span>
        <div>
          <img src={ this.props.uri } />
      </div>
      </div>
      </span>
    );
  }

  render() {
    return this.props.edit ? this.renderEdit() : this.renderView();
  }
}
