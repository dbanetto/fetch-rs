import { h, Component } from 'preact';
import { Link } from 'preact-router';
import SeriesForm from './seriesForm.jsx';

export default class SeriesNew extends Component {

  render() {
    return (
        <div>
          <SeriesForm router={ this.props.router } />
          <Link href="/">back</Link>
        </div>
        );
  }
}
