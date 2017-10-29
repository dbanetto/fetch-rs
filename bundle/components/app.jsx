import { h, Component } from 'preact';
import { Router } from 'preact-router';
import Home from './home.jsx';
import View from './view.jsx';
import SeriesNew from './new.jsx';
import SeriesEdit from './edit.jsx';

export default class App extends Component {
  render({}, {}) {
    return (<div>
      <h1>App</h1>
      <Router>
        <Home path="/" />
        <SeriesNew path="/series/new" />
        <View path="/series/:id" />
        <SeriesEdit path="/series/:id/edit" />
      </Router>
    </div>);
  }
}
