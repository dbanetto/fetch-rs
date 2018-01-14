import { h, Component } from 'preact';


export default class ImageHandler extends Component {

  constructor() {
    super();
  }

  static name() {
    return "Image";
  }

  renderView() {
    return ( <img src={ this.props.blob.src } />);
  }

  handleUpdate(event) {
    this.props.handleUpdate({ type: "image", src: event.target.value });
  }

  renderEdit() {
    return (
      <span>
        <input type="url"
          name={ this.props.blob.name || '' }
          value={ this.props.blob.src }
      onChange={ this.handleUpdate.bind(this) }  />
      <div class="preview">
        <span>Preview</span>
        <div>
          <img src={ this.props.blob.src } />
      </div>
      </div>
      </span>
    );
  }

  render() {
    return this.props.edit ? this.renderEdit() : this.renderView();
  }
}
