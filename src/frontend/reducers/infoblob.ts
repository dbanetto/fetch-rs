import * as actions from "../actions";
import * as api from "../api";
import store from "../store";

const INITAL_SATE = {
  blobs: {},
  isAll: {},
  loading: false,
};

const infoBlobReducer = (state, action) => {
  if (!state) {
    return INITAL_SATE;
  }

  switch (action.type) {

    case "GET_ALL_INFOBLOBS":
      return { ...state, ...getInfoBlob(state, action.seriesId) };
    case "FINISHED_GET_INFOBLOBS":
      return { ...state, ...finishedGetInfoBlobs(state, action.seriesId, action.infoBlobs) };

    case "GET_TYPE_INFOBLOBS":
      return { ...state, ...getInfoBlobType(state, action.seriesId, action.types) };
    case "FINISHED_GET_TYPE_INFOBLOBS":
      return { ...state, ...finishedGetInfoBlobType(state, action.seriesId, action.types, action.blobs) };

    default:
      return state;
  }
};

const getInfoBlob = (state, seriesId: number) => {
  console.log(state.isAll);
  if (state.blobs[seriesId] && state.isAll && state.isAll[seriesId]) {
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

  if (state.blobs[seriesId]) {
    const cacheBlobs = state.blobs[seriesId];

    // remove types that are already held
    types = types.filter((t) => {
      return !cacheBlobs.find((b) => b.info_type === t);
    });

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

export default infoBlobReducer;
