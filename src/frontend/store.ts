import "./model";

class Store {

  public getSeries(): Promise<ISeries[]> {
    const endpoint = "/api/v1/series";
    return this.api_get(endpoint);
  }

  public getSeriesId(id: number): Promise<ISeries> {
    const endpoint = `/api/v1/series/${ id }`;
    return this.api_get(endpoint);
  }

  public getInfoType(id: number, types: string[]): Promise<IInfoBlob[]> {
    const typeQuery = types.join("+");
    const endpoint = `/api/v1/info/${ id }/types/${ typeQuery }`;
    // prevents repeat calls to the API for non-existing data
    return this.api_get(endpoint);
  }

  public getSeriesInfo(id: number): Promise<IInfoBlob[]> {
    const endpoint = `/api/v1/info/${ id }`;
    return this.api_get(endpoint);
  }

  public deleteSeriesId(id: number): Promise<void> {
    const endpoint = `/api/v1/series/${ id }`;
    return this.api_delete(endpoint);
  }

  public upsertSeries(series: ISeries): Promise<ISeries> {
    if (series.id) {
      return this.api_put(`/api/v1/series/${ series.id }`, series);
    } else {
      return this.api_post("/api/v1/series/new", series);
    }
  }

  private api_get<T>(endpoint: string): Promise<T> {

    return fetch(endpoint)
    .then((r) => r.json())
    .then((resp) => {

      if (!resp.success) {
        throw resp.error;
      }
      return resp.data;
    });
  }

  private api_delete<T>(endpoint: string): Promise<T> {
    return fetch(endpoint, {
      method: "DELETE",
    })
    .then((r) => r.json())
    .then((resp) => {
      if (!resp.success) {
        throw resp.error;
      }
      return resp.data;
    });
  }

  private api_put<T>(endpoint: string, data): Promise<T> {

    return fetch(endpoint, {
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      },
      method: "PUT",
    })
    .then((r) => r.json())
    .then((resp) => {
      if (!resp.success) {
        throw resp.error;
      }
      return resp.data;
    });
  }

  private api_post<T>(endpoint: string, data): Promise<T> {

    return fetch(endpoint, {
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
    })
    .then((r) => r.json())
    .then((resp) => {
      if (!resp.success) {
        throw resp.error;
      }
      return resp.data;
    });
  }

}

const store = new Store();
export default store;
