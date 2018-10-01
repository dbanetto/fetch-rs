import "../model";

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

export const clearInfoBlob = (seriesId: number) => ({
  seriesId,
  type: "CLEAR_INFOBLOBS",
});
