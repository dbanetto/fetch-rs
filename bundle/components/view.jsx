import React, { Component } from 'react';
import { Link } from 'react-router';

class View extends Component {

  constructor() {
    super();
    
    this.state = {
      series: null,
    };
  }

  componentDidMount() {
    this.getSeries();
  }

  getSeries() {
    let self = this;

    fetch(`/api/v1/series/${ this.props.params.id }`)
      .then(r => r.json())
      .then( resp => {
        if (!resp.success) {
          self.props.router.push('/');
          return;
        }

        self.setState({
          series: resp.data
        });
      })
    .catch(alert);
  }

  back() {
    return (<Link to='/'>back</Link>);
  }

  render() {
    if (this.state.series === null) {
      return (<div><p>loading...</p>{ this.back() }</div>);
    }

    let series = this.state.series;

    return (
        <div>
          <h2>{ JSON.stringify(series) }</h2>
          <div>
            { this.back() }
          </div>
        </div>
        );
  }
}

export default View;
