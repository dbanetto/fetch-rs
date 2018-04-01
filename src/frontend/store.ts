import { createStore } from "redux";
import combinedReducers from "./reducer";

const INITAL_SATE = {
  app: {
    message: "",
    showError: false,
  },
  infoBlob: {
    blobs: {},
    loading: false,
  },
  series: {
    items: [],
    loading: false,
  },
};

const store = createStore(
  combinedReducers,
  INITAL_SATE,
);

export default store;
