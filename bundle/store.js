class _Store {
  constructor() {
    this.cache = {};
  }

  _update_cache(endpoint, data) {
    this.cache[endpoint] = {
      value: data,
      stamp: Date.now()
    };
  }

  _api_get(endpoint, options) {
    let self = this;

    // option to force fetching new data instead of cache
    let force = options && options.force ? options.force : false;
    let storeErrors = options && options.nulls ? options.nulls : false;

    if (this.cache[endpoint] && !force) {
      // TODO: check cache timeout
      return new Promise((resolve, reject) => {
        resolve(this.cache[endpoint].value);
      });
    }

    return fetch(endpoint)
      .then(r => r.json())
      .then(resp => {

        if (resp.success || storeErrors) {
          self._update_cache(endpoint, resp.data);
        }

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
        self.cache[endpoint] = undefined;
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
        self._update_cache(endpoint, resp.data);
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
        self._update_cache(endpoint, resp.data);
        return resp.data;
      });
  }

  getSeriesCache() {
    const endpoint = '/api/v1/series';
    return this.cache[endpoint] ? this.cache[endpoint].value : null;
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
    const endpoint = `/api/v1/series/${ id }/uri_primary`;
    // prevents repeat calls to the API for non-existing data
    var options = options ? options : {}
    options.nulls = true;
    return this._api_get(endpoint, options);
  }

  getSeriesUri(id, options) {
    const endpoint = `/api/v1/series/${ id }/uri`;
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
