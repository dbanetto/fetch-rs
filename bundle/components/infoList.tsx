import { h, Component } from 'preact';
import handler from './handler';
import '../model';


interface InfoListProps {
    value: Array<InfoBlob>;
    handleUpdate: (key: string, value: InfoBlob[]) => void;
}

export default class InfoList extends Component<InfoListProps, void> {

  getTypes() {
    return handler.listTypes();
  }

  handleAdd() {
    let elements = this.props.value;
    let selection = document.getElementById('type-selector') as HTMLInputElement;
    let built = {
        id: null, series_id: null, blob: {},
        info_type: selection.value.toString(), primary: false };

    elements.push(built);

    this.props.handleUpdate("info", elements);
  }

  handleDelete(key) {
    let elements = this.props.value;

    elements.splice(key, 1);

    // TODO: handle POST'ing delete of series if it has an `id`

    this.props.handleUpdate("info", elements);
  }

  handleUpdate(index: number, value: InfoBlob) {
    let blobs = this.props.value;

    blobs[index] = value;

    this.props.handleUpdate("info", blobs);
  }

  handlePrimary(index: number, checked: boolean) {
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
    
    this.props.handleUpdate("info", elements);
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


interface InfoProps {
    value: InfoBlob;
    handlePrimary: (checked: boolean) => void;
    handleUpdate: (value: any) => void;
    handleDelete: () => void;
};

class InfoElement extends Component<InfoProps, void> {

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
      <input type="hidden" name="id" className="info-element" value={ this.props.value.id.toString() } />

      { handler.build(
        this.props.value.blob,
        this.props.value.info_type,
        {
          edit: true,
          handleUpdate: this.handleUri.bind(this)
        })
      }

      <input type="radio" name="primary" className="primary" value={ this.props.value.primary.toString() }
        checked={ this.props.value.primary } onChange={ this.handlePrimary.bind(this) }/>

      <button type="button" onClick={ this.props.handleDelete }>x</button>
    </div>);
  }

}
