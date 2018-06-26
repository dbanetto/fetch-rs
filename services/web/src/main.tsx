import { h, render } from "preact";
import { Provider } from "preact-redux";
import "./bulma.js";
import "./index.html";
import App from "./pages/app";
import store from "./store";

render(
    <Provider store={store} >
        <App />
    </Provider>,
    document.getElementById("root"));
