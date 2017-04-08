import React, { Component } from 'react';

class UriList extends Component {

  handleAdd() {

    let elements = this.props.value;
    elements.push({ uri: "", primary: false });

    this.props.handleUpdate(elements);
  }

  handleDelete(key) {
    let elements = this.props.value;

    elements.splice(key, 1);

    // TODO: handle POST'ing delete of series if it has an `id`

    this.props.handleUpdate(elements);
  }

  handleUpdate(key, value) {
    let elements = this.props.value;

    // hack
    elements[key] = value;

    this.props.handleUpdate(elements);
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
    return (<UriElement
            handleDelete={ this.handleDelete.bind(this, index) }
            handleUpdate={ this.handleUpdate.bind(this, index) }
            handlePrimary={ this.handlePrimary.bind(this, index) }
            value={ ele }
            key={ index }
            />
            );
  }

  render() {
    return  (
        <div>
          { this.props.value.map((e, i) => this.buildElement(e, i)) }

          <button type="button" onClick={ this.handleAdd.bind(this) }>add</button>
        </div>
        );
  }
}


class UriElement extends Component {

  handleUri(event) {
    let value = this.props.value;
    value.uri = event.target.value;

    this.props.handleUpdate(value);
  }

  handlePrimary(event) {
    this.props.handlePrimary(event.target.checked);
  }

  render() {
    return (<div>
      <input type="hidden" name="id" className="uri" value={ this.props.value.id } />
      <input type="uri" name="uri" className="uri" value={ this.props.value.uri } onChange={ this.handleUri.bind(this) } />
      <input type="radio" name="primary" className="primary" value={this.props.value.primary} checked={ this.props.value.primary } onChange={ this.handlePrimary.bind(this) }/>

      <button type="button" onClick={ this.props.handleDelete }>x</button>
    </div>);
  }

}

export default UriList;
