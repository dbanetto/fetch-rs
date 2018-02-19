import { h, Component } from 'preact';
import { Router } from 'preact-router';
import Home from './home';
import View from './view';
import SeriesNew from './new';
import SeriesEdit from './edit';

export default class App extends Component<any, void> {
  render() {
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
