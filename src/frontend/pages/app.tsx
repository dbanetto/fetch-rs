import { Component, h } from "preact";
import { connect } from "preact-redux";
import { Link, Router  } from "preact-router";
import { hideError, showError } from "../actions";
import store from "../store";
import Edit from "./edit";
import Home from "./home";
import New from "./new";
import View from "./view";

interface IAppProps {
    showError: boolean;
    errorMessage: string;
}

class App extends Component<any, IAppProps> {

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
            <div>
                <nav class="navbar" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand">

                        <div class="navbar-item" >
                            <Link href="/">
                                <h1 class="title">ICON</h1>
                            </Link>
                        </div>

                        <label class="navbar-burger" data-target="navbar-menu-target">
                            <span />
                            <span />
                            <span />
                        </label>
                    </div>
                    <div id="navbar-menu-target" class="navbar-menu">
                        <div class="navbar-start">
                            <Link class="navbar-item" href="/">
                                <h1>Home</h1>
                            </Link>

                            <Link class="navbar-item" href="/series/new">
                                <h1>New</h1>
                            </Link>
                        </div>
                    </div>
                </nav>

                <div class="container is-fluid app-main">
                    <Router>
                        <Home path="/" />
                        <New path="/series/new" />
                        <View path="/series/:id" />
                        <Edit path="/series/:id/edit" />
                    </Router>
                </div>

                <div class={modalClass} >
                    <div class="modal-background" onClick={this.handleClose} />
                    <div class="modal-card">
                        <header class="modal-card-head">
                            <p class="modal-card-title">Error!</p>
                            <button class="delete" aria-label="close" onClick={this.handleClose} />
                        </header>
                        <section class="modal-card-body">
                            {this.props.errorMessage}
                        </section>
                    </div>
                    <button class="modal-close is-large" aria-label="close" onClick={this.handleClose} />
                </div>
            </div>
        );
    }

    private handleClose() {
        store.dispatch(hideError());
    }

    private handleError() {
        store.dispatch(showError("Something went wrong!"));
    }
}

export default connect((state) => ({
    errorMessage: state.app.errorMessage,
    showError: state.app.showError,
}))(App);
