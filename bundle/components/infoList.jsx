import { h, Component } from 'preact';
import handler from './handler.jsx';

export default class InfoList extends Component {

  getTypes() {
    return handler.listTypes();
  }

  handleAdd() {

    let elements = this.props.value;
    let selection = document.getElementById('type-selector');
    let built = { blob: {}, info_type: selection.value, primary: false };

    console.log("built new handler");
    console.log(built);
    elements.push(built);

    console.log(elements);

    this.props.handleUpdate("info", elements);
  }

  handleDelete(key) {
    let elements = this.props.value;

    elements.splice(key, 1);

    // TODO: handle POST'ing delete of series if it has an `id`

    this.props.handleUpdate("info", elements);
  }

  handleUpdate(index, value) {
    let blobs = this.props.value;

    // hack
    console.log("info list update");
    console.log(blobs[index]);
    console.log(value);
    blobs[index] = value;
    console.log(blobs);

    this.props.handleUpdate("info", blobs);
  }

  handlePrimary(index, checked) {
    let elements = this.props.value;

    elements.map( (e, i) => {
      if (i === index) {
        e.primary = checked;
        return e;
      } else {
        e.primary = false;
        return e;
      }
    });
    
    this.props.handleUpdate(elements);
  }

  buildElement(ele, index) {
    console.log("Build element");
    console.log(ele);
    return (<InfoElement
            handleDelete={ this.handleDelete.bind(this, index) }
            handleUpdate={ this.handleUpdate.bind(this, index) }
            handlePrimary={ this.handlePrimary.bind(this, index) }
            value={ ele }
            key={ index }
            />
            );
  }

  render() {
    console.log(this.props.value);
    return  (
        <div>
          { this.props.value.map((ele, index) => this.buildElement(ele, index)) }

          <select id="type-selector">
            { this.getTypes().map((t) => <option value={ t.type }>{ t.name }</option> ) }
          </select>
          <button type="button" onClick={ this.handleAdd.bind(this) }>add</button>
        </div>
        );
  }
}


class InfoElement extends Component {

  handleUri(blob) {
    let value = this.props.value;

    console.log("element update");
    console.log(value);
    console.log(blob);
    console.log("----");

    value.blob = blob;

    console.log(value);
    console.log("done");

    this.props.handleUpdate(value);
  }

  handlePrimary(event) {
    this.props.handlePrimary(event.target.checked);
  }

  render() {
    return (<div>
      <input type="hidden" name="id" className="info-element" value={ this.props.value.id } />

      { handler.build(
        this.props.value.blob,
        this.props.value.info_type,
        {
          edit: true,
          handleUpdate: this.handleUri.bind(this)
        })
      }

      <input type="radio" name="primary" className="primary" value={this.props.value.primary} checked={ this.props.value.primary } onChange={ this.handlePrimary.bind(this) }/>

      <button type="button" onClick={ this.props.handleDelete }>x</button>
    </div>);
  }

}
