class _Store {
  constructor() {
    this.cache = {};
  }

  _api_get(endpoint, options) {
    let self = this;

    // option to force fetching new data instead of cache
    let force = options && options.force ? options.force : false;

    if (this.cache[endpoint] && !force) {
      // TODO: check cache timeout
      return new Promise((resolve, reject) => {
        resolve(this.cache[endpoint].value);
      });
    }

    return fetch(endpoint)
      .then(r => r.json())
      .then(resp => {
        if (!resp.success) {
          throw resp.error;
        }
        self.cache[endpoint] = {
          value: resp.data,
          stamp: Date.now()
        };
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
}

let Store = new _Store();
export default Store;
