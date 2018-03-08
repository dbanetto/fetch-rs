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
      <div class="columns">
        <div class="column">
          <input type="url" class="input"
            name={ this.props.name || '' }
            value={ this.props.blob.src }
            onChange={ this.handleUpdate.bind(this) }  />
        </div>
        <div class="column">
          <h3 class="subtitle">Preview</h3>
          <div class="poster preview">
            <img class="image" src={ this.props.blob.src } />
          </div>
        </div>
      </div>
    );
  }

  render() {
    return this.props.edit ? this.renderEdit() : this.renderView();
  }
}
