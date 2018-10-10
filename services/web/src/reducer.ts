import { combineReducers } from "redux";
import appReducer from "./reducers/app";
import fetchReducer from "./reducers/fetch";
import infoBlobReducer from "./reducers/infoblob";
import seriesReducer from "./reducers/series";

const combined = combineReducers({
  app: appReducer,
  fetch: fetchReducer,
  infoBlob: infoBlobReducer,
  series: seriesReducer,
});

export default combined;
