import { push } from "connected-react-router";
import { showError } from "../actions/app";
import { clearInfoBlob } from "../actions/infoblob";
import * as actions from "../actions/series";
import * as api from "../api";
import store from "../store";

export interface ISeriesState {
  items: ISeries[];
  loading: boolean;
}

const INITAL_STATE: ISeriesState = {
  items: [],
  loading: false,
};

const seriesReducer = (state: ISeriesState = INITAL_STATE, action): ISeriesState => {
  switch (action.type) {
    case "GET_ALL_SERIES":
      return { ...state, ...getAllSeries(state) };
    case "FINISHED_GET_ALL_SERIES":
      return { ...state, loading: false, items: action.series };

    case "GET_SERIES":
      return { ...state, ...getSeries(parseInt(action.id, 10), state.items) };

    case "FINISHED_GET_SERIES":
      const seriesList: ISeries[] = state.items.filter((ele) => ele.id !== parseInt(action.id, 10));
      if (action.series) {
        seriesList.push(action.series);
      }
      return { ...state, loading: false, items: seriesList };

    case "DELETE_SERIES":
      return { ...state, ...deleteSeries(state, parseInt(action.id, 10)) };

    case "FINISHED_DELETE_SERIES":
      return { ...state, ...finishedDeleteSeries(state, parseInt(action.id, 10)) };

    case "UPSERT_SERIES":
      return { ...state, ...upsertSeries(action.formData) };

    default:
      return state;
  }
};

const getAllSeries = (state) => {
  api.getSeries()
    .then((series) => {
      store.dispatch(actions.finishedGetAllSeries(series));
    }).catch((err) => {
      store.dispatch(showError(err.toString()));
      store.dispatch(actions.finishedGetAllSeries([]));
    });

  // do not show loading if we got something to show
  return { loading: !Boolean(state.items.length) };
};

const getSeries = (id: number, stateSeries) => {
  if (stateSeries.find((ele) => ele.id === id)) {
    return { loading: false };
  }

  api.getSeriesId(id).then((series) => {
    store.dispatch(actions.finishedGetSeries(id, series));
  }).catch((err) => {
    store.dispatch(showError(err.toString()));
    store.dispatch(actions.finishedGetSeries(id, null));
  });
  return { loading: true };
};

const upsertSeries = (formData: SeriesFull) => {

  api.upsertSeries(formData)
    .then((series) => {
      store.dispatch(actions.finishedGetSeries(series.id, series));
      store.dispatch(clearInfoBlob(series.id));
      store.dispatch(push(`/series/${series.id}`));
    }).catch((err) => {
      store.dispatch(showError(err.toString()));
    });

  return { loading: true };
};

const deleteSeries = (state, id: number) => {

  api.deleteSeriesId(id).then(() => {
    store.dispatch(clearInfoBlob(id));
    store.dispatch(actions.finishedDeleteSeries(id));
  }).catch((err) => {
    store.dispatch(showError(err.toString()));
    store.dispatch(actions.finishedDeleteSeries(null));
  });

  return { loading: true };
};

const finishedDeleteSeries = (state, id: number) => {
  const removedList: ISeries[] = state.items.filter((ele) => ele.id !== id);

  return { loading: false, items: removedList };
};

export default seriesReducer;
