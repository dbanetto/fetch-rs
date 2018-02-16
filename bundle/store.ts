import './model'

class _Store {
    constructor() {
    }

    _api_get<T>(endpoint: string): Promise<T> {
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

    _api_delete<T>(endpoint: string): Promise<T> {
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

    _api_put<T>(endpoint: string, data): Promise<T> {
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

    _api_post<T>(endpoint: string, data): Promise<T> {
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

    getSeries(): Promise<Array<Series>> {
        const endpoint = '/api/v1/series';
        return this._api_get(endpoint);
    }

    getSeriesId(id: number): Promise<Series> {
        const endpoint = `/api/v1/series/${ id }`;
        return this._api_get(endpoint);
    }

    getSeriesPrimary(id: number): Promise<InfoBlob> {
        const endpoint = `/api/v1/info/${ id }/primary`;
        // prevents repeat calls to the API for non-existing data
        var options = options ? options : {}
        options.nulls = true;
        return this._api_get(endpoint);
    }

    getSeriesInfo(id: number): Promise<Array<InfoBlob>> {
        const endpoint = `/api/v1/info/${ id }`;
        return this._api_get(endpoint);
    }

    deleteSeriesId(id: number): Promise<void> {
        const endpoint = `/api/v1/series/${ id }`;
        return this._api_delete(endpoint);
    }

    upsertSeries(series: Series): Promise<Series> {
        if (series.id) {
            return this._api_put(`/api/v1/series/${ series.id }`, series)
        } else {
            return this._api_post('/api/v1/series/new', series);
        }
    }
}

const Store = new _Store();
export default Store;
