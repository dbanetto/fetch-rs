import "./model";

export function getSeries(): Promise<ISeries[]> {
  const endpoint = "/api/v1/series";
  return api_get(endpoint);
}

export function getSeriesId(id: number): Promise<ISeries> {
  const endpoint = `/api/v1/series/${ id }`;
  return api_get(endpoint);
}

export function getInfoType(id: number, types: string[]): Promise<IInfoBlob[]> {
  const typeQuery = types.join("+");
  const endpoint = `/api/v1/info/${ id }/types/${ typeQuery }`;
  // prevents repeat calls to the API for non-existing data
  return api_get(endpoint);
}

export function getSeriesInfo(id: number): Promise<IInfoBlob[]> {
  const endpoint = `/api/v1/info/${ id }`;
  return api_get(endpoint);
}

export function deleteSeriesId(id: number): Promise<void> {
  const endpoint = `/api/v1/series/${ id }`;
  return api_delete(endpoint);
}

export function upsertSeries(series: ISeries): Promise<ISeries> {
  if (series.id) {
    return api_put(`/api/v1/series/${ series.id }`, series);
  } else {
    return api_post("/api/v1/series/new", series);
  }
}

function api_get<T>(endpoint: string): Promise<T> {

  return fetch(endpoint)
  .then((r) => r.json())
  .then((resp) => {

    if (!resp.success) {
      throw resp.error;
    }
    return resp.data;
  });
}

function api_delete<T>(endpoint: string): Promise<T> {
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

function api_put<T>(endpoint: string, data): Promise<T> {

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

function api_post<T>(endpoint: string, data): Promise<T> {

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
