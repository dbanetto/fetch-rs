import "../model";

export const getAllSeries = () => ({
  type: "GET_ALL_SERIES",
});

export const finishedGetAllSeries = (series: ISeries[]) => ({
  series,
  type: "FINISHED_GET_ALL_SERIES",
});

export const getSeries = (id: number) => ({
    id,
    type: "GET_SERIES",
});

export const finishedGetSeries = (id: number, series: ISeries) => ({
  id,
  series,
  type: "FINISHED_GET_SERIES",
});

export const deleteSeries = (id: number) => ({
  id,
  type: "DELETE_SERIES",
});

export const finishedDeleteSeries = (id: number) => ({
  id,
  type: "FINISHED_DELETE_SERIES",
});

export const upsertSeries = (formData: SeriesFull) => ({
  formData,
  type: "UPSERT_SERIES",
});
