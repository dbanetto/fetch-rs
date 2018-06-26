import { combineReducers } from "redux";
import appReducer from "./reducers/app";
import infoBlobReducer from "./reducers/infoblob";
import seriesReducer from "./reducers/series";

const combined = combineReducers({
  app: appReducer,
  infoBlob: infoBlobReducer,
  series: seriesReducer,
});

export default combined;
