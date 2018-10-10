import { showError } from "../actions/app";
import * as actions from "../actions/fetch";
import * as api from "../api";
import store from "../store";

export interface IFetchState {
  results: any[];
  loading: boolean;
  status: any;
}

const INITAL_SATE: IFetchState = {
  loading: false,
  results: [],
  status: {},
};

export const fetchReducer = (state: IFetchState = INITAL_SATE, action): IFetchState => {
  switch (action.type) {
    case "FETCH_GET_STATUS":
      getStatus();
      return { ...state };
    case "FETCH_FINISH_STATUS":
      return { ...state, status: action.status };
    case "FETCH_CALL":
      callFetch();
      return { ...state, loading: true };
    case "FETCH_CALL_FINISHED":
      return { ...state, loading: false, results: action.results };
    default:
      return state;
  }
};

const getStatus = () => {
  api.getFetchStatus()
    .then((status) => {
      store.dispatch(actions.finishedStatus(status));
    }).catch(showError);
};

const callFetch = () => {
  api.callFetch()
    .then((res) => {
      if (res.success) {
        store.dispatch(actions.finishedCallFetch(res.results));
      } else {
        store.dispatch(actions.finishedCallFetch([]));
        throw res.message;
      }
    }).catch(showError);
};

export default fetchReducer;
