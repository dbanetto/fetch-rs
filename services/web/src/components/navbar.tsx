import * as React from "react";
import { Link, RouteComponentProps, withRouter } from "react-router-dom";

class NavBar extends React.PureComponent<RouteComponentProps<any, any>> {

    public render() {

        return (
                <nav className="navbar" role="navigation" aria-label="main navigation">
                    <div className="navbar-brand">

                        <div className="navbar-item" >
                            <Link to="/">
                                <h1 className="title">ICON</h1>
                            </Link>
                        </div>

                        <label className="navbar-burger" data-target="navbar-menu-target">
                            <span />
                            <span />
                            <span />
                        </label>
                    </div>
                    <div id="navbar-menu-target" className="navbar-menu">
                        <div className="navbar-start">
                            <Link className="navbar-item" to="/">
                                <h1>Home</h1>
                            </Link>

                            <Link className="navbar-item" to="/series/new">
                                <h1>New</h1>
                            </Link>
                        </div>
                    </div>
                </nav>
        );
    }
}

export default withRouter(NavBar);
