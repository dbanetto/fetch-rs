import { h, Component } from 'preact';
import '../../model';


export default class ImageHandler extends Component<HandlerProps, void> {

  static TypeName(): string {
    return "Image";
  }

  renderView() {
    return ( <img src={ this.props.blob.src } />);
  }

  handleUpdate(event) {
    this.props.handleUpdate({ src: event.target.value });
  }

  renderEdit() {
    return (
      <span>
        <input type="url"
          name={ this.props.name || '' }
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