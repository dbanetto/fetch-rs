import "./model";

// Series actions
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

export const finishDeleteSeries = (id: number) => ({
  id,
  type: "FINISHED_DELETE_SERIES",
});

// Infoblob actions
export const getInfoBlobs = (seriesId: number) => ({
  seriesId,
  type: "GET_ALL_INFOBLOBS",
});

export const getInfoBlobType = (seriesId: number, types: string[]) => ({
  seriesId,
  type: "GET_TYPE_INFOBLOBS",
  types,
});

export const finishGetInfoBlobType = (seriesId: number, types: string[], blobs: IInfoBlob[]) => ({
  blobs,
  seriesId,
  type: "FINISHED_GET_TYPE_INFOBLOBS",
  types,
});

export const finishedGetInfoBlobs = (seriesId: number, infoBlobs: IInfoBlob[]) => ({
  infoBlobs,
  seriesId,
  type: "FINISHED_GET_INFOBLOBS",
});

// App actions
export const showError = (message: string) => ({
  message,
  type: "SHOW_ERROR",
});

export const hideError = () => ({
  type: "HIDE_ERROR",
});
