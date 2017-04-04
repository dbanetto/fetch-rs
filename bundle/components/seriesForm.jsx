import React, { Component } from 'react';
import Form from 'react-jsonschema-form';

class SeriesForm extends Component {

  schema()  {
    return {
      title: "Series",
      type: "object",
      required: ["title"],
      properties: {
        title: { type: "string", title: "Title" },
        start_date: { type: "string", format: "alt-date", title: "Start date" },
        end_date: { type: "string", format: "alt-date", title: "End date" },
        episodes_current: { type: "integer", format: "updown", title: "Current episode", minimum: 0, default: 0 },
        episodes_total: { type: "integer", format: "updown", title: "Total episodes", minimum: 0 },
        poster_url: { type: "string", format: "uri", title: "Poster" },
        info_uris: {
          title: "Info Uris",
          type: "array",
          items: {
            type: "object",
            properties: {
              uri: { title: "Uri", type: "string", format: "uri" },
              primary: { title: null, type: "boolean" }
            }
          }
        }
      }
    }
  }

  uiSchema() {
    return {
      info_uri: {
        items: {
          primary: {
            "ui:widget": "radio"
          }
        }
      }
    }
  }

  validate(formData, errors) {
    // add validation
    return errors;
  }

  handleSubmit(form) {
    let self = this;
    console.log(form.formData);

    fetch('/api/v1/series/new', {
      method: 'POST',
      body: JSON.stringify(form.formData),
      headers: {
        'Content-Type': 'application/json'
      }})
    .then(r => r.json())
      .then(resp => {
        if (!resp.success) {
          throw resp.error;
        }
        // redirect to view
        self.props.router.push(`/series/${ resp.data.id }`);
      })
    .catch(alert);
  }

  render() {
    return (
        <Form onSubmit={ this.handleSubmit.bind(this) }
          schema={ this.schema() }
          validate={ this.validate.bind(this) } />
        );
  }
}

export default SeriesForm;
