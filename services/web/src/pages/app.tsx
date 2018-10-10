import { ConnectedRouter as Router } from "connected-react-router";
import * as React from "react";
import { render } from "react-dom";
import { connect } from "react-redux";
import { Link, Route, Switch } from "react-router-dom";
import { hideError, showError } from "../actions/app";

import NavBar from "../components/navbar";
import store, { history, IReduxState } from "../store";
import EditPage from "./edit";
import FetchPage from "./fetch";
import HomePage from "./home";
import NewPage from "./new";
import ViewPage from "./view";

interface IAppProps {
    errorMessage: string;
    history: any;
    showError: boolean;
}

class App extends React.PureComponent<IAppProps> {

    constructor(props) {
        super(props);
        this.handleClose = this.handleClose.bind(this);
        this.handleError = this.handleError.bind(this);
    }

    public render() {
        let modalClass = "modal";
        if (this.props.showError) {
            modalClass += " is-active";
        }

        return (
            <Router history={history}>
                <div>
                    <NavBar />
                    <div className="container is-fluid app-main">
                        <Switch>
                            <Route exact={true} path="/" component={HomePage} />
                            <Route exact={true} path="/fetch" component={FetchPage} />
                            <Route exact={true} path="/series/new" component={NewPage} />
                            <Route exact={true} path="/series/:id" component={ViewPage} />
                            <Route exact={true} path="/series/:id/edit" component={EditPage} />
                        </Switch>
                    </div>
                    <div className={modalClass} >
                        <div className="modal-background" onClick={this.handleClose} />
                        <div className="modal-card">
                            <header className="modal-card-head">
                                <p className="modal-card-title">Error!</p>
                                <button className="delete" aria-label="close" onClick={this.handleClose} />
                            </header>
                            <section className="modal-card-body">
                                {this.props.errorMessage}
                            </section>
                        </div>
                        <button className="modal-close is-large" aria-label="close" onClick={this.handleClose} />
                    </div>
                </div>
            </Router>
        );
    }

    private handleClose() {
        store.dispatch(hideError());
    }

    private handleError() {
        store.dispatch(showError("Something went wrong!"));
    }
}

export default connect((state: IReduxState) => ({
    errorMessage: state.app.errorMessage,
    showError: state.app.showError,
}))(App);
