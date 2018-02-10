class _Store {
  constructor() {
  }

  _api_get(endpoint, options) {
    let self = this;

    return fetch(endpoint)
      .then(r => r.json())
      .then(resp => {

        if (!resp.success) {
          throw resp.error;
        }
        return resp.data;
      });
  }

  _api_delete(endpoint) {
    let self = this;

    return fetch(endpoint, {
      method: 'DELETE'
    })
    .then(r => r.json())
      .then(resp => {
        if (!resp.success) {
          throw resp.error;
        }
        return resp.data;
      });
  }

  _api_put(endpoint, data) {
    let self = this;

    return fetch(endpoint, {
      method: 'PUT',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    })
    .then(r => r.json())
      .then(resp => {
        if (!resp.success) {
          throw resp.error;
        }
        return resp.data;
      });
  }

  _api_post(endpoint, data) {
    let self = this;

    return fetch(endpoint, {
      method: 'POST',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    })
    .then(r => r.json())
      .then(resp => {
        if (!resp.success) {
          throw resp.error;
        }
        return resp.data;
      });
  }

  getSeries(options) {
    const endpoint = '/api/v1/series';
    return this._api_get(endpoint, options);
  }

  getSeriesId(id, options) {
    const endpoint = `/api/v1/series/${ id }`;
    return this._api_get(endpoint, options);
  }

  getSeriesPrimary(id, options) {
    const endpoint = `/api/v1/info/${ id }/primary`;
    // prevents repeat calls to the API for non-existing data
    var options = options ? options : {}
    options.nulls = true;
    return this._api_get(endpoint, options);
  }

  getSeriesInfo(id, options) {
    const endpoint = `/api/v1/info/${ id }`;
    return this._api_get(endpoint, options);
  }

  deleteSeriesId(id) {
    const endpoint = `/api/v1/series/${ id }`;
    return this._api_delete(endpoint);
  }

  upsertSeries(series) {
    return series.id ? this._api_put(`/api/v1/series/${ series.id }`, series) : this._api_post('/api/v1/series/new', series);
  }

}

const Store = new _Store();
export default Store;
