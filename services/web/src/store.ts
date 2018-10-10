import { connectRouter, routerMiddleware } from "connected-react-router";
import { createBrowserHistory } from "history";
import { applyMiddleware, compose, createStore } from "redux";
import combinedReducers from "./reducer";

import { IAppState } from "./reducers/app";
import { IFetchState } from "./reducers/fetch";
import { IInfoBlobState } from "./reducers/infoblob";
import { ISeriesState } from "./reducers/series";

export interface IReduxState {
  app: IAppState;
  fetch: IFetchState;
  infoBlob: IInfoBlobState;
  series: ISeriesState;
  router: any;
}

export const history = createBrowserHistory();

const store = createStore(
  connectRouter(history)(combinedReducers),
  {},
  compose(
    applyMiddleware(
      routerMiddleware(history), // for routing via dispatch
    ),
  ));

export default store;
