import { createStore } from "redux";
import combinedReducers from "./reducer";

const store = createStore(combinedReducers);

export default store;
