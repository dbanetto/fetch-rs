import * as actions from "../actions";
import * as api from "../api";
import store from "../store";

export interface IInfoBlobState {
  blobs: object;
  isAll: object;
  loading: boolean;
}

const INITAL_STATE: IInfoBlobState = {
  blobs: {},
  isAll: {},
  loading: false,
};

const infoBlobReducer = (state: IInfoBlobState = INITAL_STATE, action): IInfoBlobState => {
  switch (action.type) {
    // All types of a series
    case "GET_ALL_INFOBLOBS":
      return { ...state, ...getInfoBlob(state, action.seriesId) };
    case "FINISHED_GET_INFOBLOBS":
      return { ...state, ...finishedGetInfoBlobs(state, action.seriesId, action.infoBlobs) };

    // Single types of a series
    case "GET_TYPE_INFOBLOBS":
      return { ...state, ...getInfoBlobType(state, action.seriesId, action.types) };
    case "FINISHED_GET_TYPE_INFOBLOBS":
      return { ...state, ...finishedGetInfoBlobType(state, action.seriesId, action.types, action.blobs) };

    case "CLEAR_INFOBLOBS":
      return { ...state, ...clearInfoBlob(state, action.seriesId) };

    case "DELETE_SERIES":
      return { ...state, ...clearInfoBlob(state, action.id) };

    default:
      return state;
  }
};

const getInfoBlob = (state, seriesId: number) => {
  if (state.blobs[seriesId] && state.isAll[seriesId]) {
    return { loading: false };
  }

  api.getSeriesInfo(seriesId).then((blobs) => {
    store.dispatch(actions.finishedGetInfoBlobs(seriesId, blobs));
  }).catch((err) => {
    store.dispatch(actions.finishedGetInfoBlobs(seriesId, []));
    store.dispatch(actions.showError(err.toString()));
  });

  return { loading: true };
};

const finishedGetInfoBlobs = (state, seriesId: number, infoblobs: IInfoBlob[]) => {

  if (!infoblobs) {
    return { loading: false };
  }

  const newBlob = {};
  newBlob[seriesId] = infoblobs;

  const updatedAll = {};
  updatedAll[seriesId] = true;

  return {
    blobs: { ...state.blobs, ...newBlob },
    isAll: { ...state.isAll, ...updatedAll },
    loading: false,
  };
};

const getInfoBlobType = (state, seriesId: number, types: string[]) => {

  // If we know we already have all of the items, don't bother
  if (state.blobs[seriesId] && state.isAll[seriesId]) {
      return { loading: false };
  }

  if (state.blobs[seriesId]) {
    const hasTypes = state.blobs[seriesId].map((b) => b.info_type);

    // remove types that are already held
    types = types.filter((t) => !hasTypes.includes(t));

    if (types.length === 0) {
      return { loading: false };
    }
  }

  api.getInfoType(seriesId, types).then((blobs) => {
    store.dispatch(actions.finishGetInfoBlobType(seriesId, types, blobs));
  }).catch((err) => {
    store.dispatch(actions.finishGetInfoBlobType(seriesId, [], []));
    store.dispatch(actions.showError(err.toString()));
  });

  return { loading: true };
};

const finishedGetInfoBlobType = (state, seriesId: number, types: string[], newBlobs: IInfoBlob[]) => {
  if (!newBlobs) {
    return { loading: false };
  }

  let blobs = [];
  if (state.blobs[seriesId]) {
    blobs = state.blobs[seriesId];
  }

  blobs = blobs.filter((b) => !types.includes(b.info_type)).concat(newBlobs);
  const newBlob = {};
  newBlob[seriesId] = blobs;

  return {
    blobs: { ...state.blobs, ...newBlob },
    loading: false,
  };
};

const clearInfoBlob = (state, seriesId: number) => {
  const blobs = { ...state.blobs };
  const isAll = { ...state.isAll };

  if (blobs[seriesId]) {
    delete blobs[seriesId];
  }

  if (isAll[seriesId]) {
    delete isAll[seriesId];
  }

  return { blobs, isAll };
};

export default infoBlobReducer;
