import React, { Component } from 'react';
import { Link } from 'react-router';
import Store from '../store.js';

class View extends Component {

  constructor() {
    super();

    this.state = {
      series: null,
      uri: null,
    };
  }

  componentDidMount() {
    this.getSeries();
  }

  getSeries() {
    let self = this;

    Promise.all([Store.getSeriesId(this.props.params.id),
        Store.getSeriesUri(this.props.params.id)])
      .then(result => {
        self.setState({
          series: result[0],
          uri: result[1]
        });
      })
    .catch(err => {
      console.log(err);
      self.props.router.push('/');
    });
  }

  handleDelete() {
    let self = this;

    let confirmed = confirm(`Are you sure you want to delete ${ this.state.title }?`);
    if (!confirmed) {
      return;
    }

    Store.deleteSeriesId(this.props.params.id)
      .then(() => {
        self.props.router.push('/');
      })
    .catch(err => {
      console.log(err);
      self.props.router.push('/');
    });

  }

  render() {
    if (this.state.series === null) {
      return (
          <div>
            <p>loading...</p>
            <Link to='/'>back</Link>
          </div>);
    }

    let series = this.state.series;

    return (
        <div>
          <div>
            <h1>{ series.title }</h1>
            <p>Start date: { series.start_date || "unkown" }</p>
            <p>End date: { series.end_date || "unkown" }</p>
            <p>Episode: { series.episodes_current }/{ series.episodes_total || "??" }</p>
            <div>
              { this.state.uri && <ul>{
                this.state.uri.map(u => <li>
                  <a href={u.uri} className={ u.primary ? 'primary' : 'other' }>{ u.uri }</a>
                </li>)
                }
              </ul> }
            </div>
          </div>
          <div>
            <span>
              <Link to={ `/series/${ series.id }/edit` }>edit</Link>
            </span>
            <span>
              <a href="javascript:void(0)" onClick={ this.handleDelete.bind(this) }>delete</a>
            </span>
            <span>
              <Link to='/'>back</Link>
            </span>
          </div>
        </div>
        );
  }
}

export default View;
