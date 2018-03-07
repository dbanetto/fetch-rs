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
        info_type: selection.value.toString() };

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

  buildElement(ele, index) {
      return (<div class="info-list-item">
          <InfoElement
              handleDelete={ this.handleDelete.bind(this, index) }
              handleUpdate={ this.handleUpdate.bind(this, index) }
              value={ ele }
              key={ index }
          />
      </div>
      );
  }

  render() {
    return  (
        <div>
            <div class="info-list">
                { this.props.value.map((ele, index) => this.buildElement(ele, index)) }
            </div>

            <div class="select">
                <select  id="type-selector">
                    { this.getTypes().map((t) => <option value={ t.type }>{ t.name }</option> ) }
                </select>
            </div>
            <span>&nbsp;</span>
            <button class="button" type="button" onClick={ this.handleAdd.bind(this) }>Add</button>
        </div>
        );
  }
}


interface InfoProps {
    value: InfoBlob;
    handleUpdate: (value: any) => void;
    handleDelete: () => void;
};

class InfoElement extends Component<InfoProps, void> {

  handleUri(blob) {
    let value = this.props.value;

    value.blob = blob;

    this.props.handleUpdate(value);
  }

  renderId() {
      if (this.props.value.id) {
          return (<input type="hidden" name="id" className="info-element" value={ this.props.value.id.toString() } />);
      } else {
          return (<div></div>);
      }
  }

  render() {
      return (<div>

          { this.renderId() }

          <div class="columns">
              <div class="column">
                  { handler.build(
                      this.props.value.blob,
                      this.props.value.info_type,
                      {
                          edit: true,
                          handleUpdate: this.handleUri.bind(this)
                      }
                    )
                  }
              </div>

          <div class="column is-one-fifth">
              <button class="button is-danger" type="button"
                  onClick={ this.props.handleDelete }>x</button>
          </div>
          </div>
      </div>);
  }

}
