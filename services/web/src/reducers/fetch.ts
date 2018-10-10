import { showError } from "../actions/app";
import * as actions from "../actions/fetch";
import * as api from "../api";
import store from "../store";

export interface IFetchState {
  loading: boolean;
  status: any;
}

const INITAL_SATE: IFetchState = {
  loading: false,
  status: {},
};

export const fetchReducer = (state: IFetchState = INITAL_SATE, action): IFetchState => {
  switch (action.type) {
    case "FETCH_GET_STATUS":
      getStatus();
      return { ...state };
    case "FETCH_FINISH_STATUS":
      return { ...state, status: action.status };
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

export default fetchReducer;
