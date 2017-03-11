import React, { Component } from 'react';

class SeriesForm extends Component {
  componentDidMount() {

  }

  handleSubmit(event) {
    event.preventDefault();

    let self = this;
    let form = event.target;
    let data = {};

    for (var input of form) {
      if (input.localName != 'input') {
        continue;
      }

      if (input.value && input.value !== "") {
        data[input.name] = input.value;
      }
    }

    if (form.reportValidity()) {

      // POST to backend
      fetch(form.action, {
        method: form.method,
        body: JSON.stringify(data),
        headers: {
          'Content-Type': 'application/json'
        }})
      .then(r => r.json())
      .then(resp => {
        if (!resp.success) {
          throw resp.error;
        }

        // redirect to view
        self.props.router.push(`/series/${1}`);
      })
      .catch(alert);
    }
  }


  render() {
    return (
        <form action="/api/v1/series/new" method="POST" onSubmit={ this.handleSubmit.bind(this) }>
          <div>
            <label htmlFor="title">Title</label>
            <input name="title" type="text" required />
          </div>
          <div>
            <label htmlFor="start_date">Start Date</label>
            <input name="start_date" type="date" />
          </div>
          <div>
            <label htmlFor="end_date">End Date</label>
            <input name="end_date" type="date" />
          </div>
          <div>
            <label htmlFor="episides_current">Episides Current</label>
            <input name="episides_current" type="number" min="0" placeholder="0" />
          </div>
          <div>
            <label htmlFor="episides_total">Episides Total</label>
            <input name="episides_total" type="number" min="0" />
          </div>
          <button type="submit">Submit</button>
        </form>
        );
  }
}

export default SeriesForm;
