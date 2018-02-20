import { h, Component } from 'preact';
import { Router, Link } from 'preact-router';
import Home from './home';
import View from './view';
import New from './new';
import Edit from './edit';

export default class App extends Component<any, void> {
  render() {
      return (
          <div>

              <nav class="navbar" role="navigation" aria-label="main navigation">
                  <div class="navbar-brand">

                      <div class="navbar-item">
                          ICON
                      </div>

                      <label class="navbar-burger" data-target="navbar-menu-target">
                          <span></span>
                          <span></span>
                          <span></span>
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

              <div class="container is-fluid">
                  <Router>
                      <Home path="/" />
                      <New path="/series/new" />
                      <View path="/series/:id" />
                      <Edit path="/series/:id/edit" />
                  </Router>
              </div>
          </div>
      );
  }
}
