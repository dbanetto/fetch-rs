import * as actions from "../actions";
import * as api from "../api";
import store from "../store";

const INITAL_SATE = {
  items: [],
  loading: false,
};

const seriesReducer = (state, action) => {
  if (!state) {
    return INITAL_SATE;
  }

  switch (action.type) {
    case "GET_ALL_SERIES":
      return { ...state, ...getAllSeries() };
    case "FINISHED_GET_ALL_SERIES":
      return { ...state, loading: false, items: action.series };

    case "GET_SERIES":
      return { ...state, ...getSeries(action.id, state.items) };

    case "FINISHED_GET_SERIES":
      const seriesList: ISeries[] = state.items.filter((ele) => ele.id !== action.id);
      if (action.series) {
        seriesList.push(action.series);
      }
      return { ...state, loading: false, items: seriesList };
    default:
      return state;
  }
};

const getAllSeries = () => {
  api.getSeries()
    .then((series) => {
      store.dispatch(actions.finishedGetAllSeries(series));
    }).catch((err) => {
      store.dispatch(actions.showError(err.toString()));
      store.dispatch(actions.finishedGetAllSeries([]));
    });
  return { loading: true };
};

const getSeries = (id, stateSeries) => {
  // // should we be caching?
  if (stateSeries.find((ele) => ele.id === id)) {
    return { loading: false };
  }

  api.getSeriesId(id).then((series) => {
    store.dispatch(actions.finishedGetSeries(id, series));
  }).catch((err) => {
    store.dispatch(actions.showError(err.toString()));
    store.dispatch(actions.finishedGetSeries(id, null));
  });
  return { loading: true };
};

export default seriesReducer;
