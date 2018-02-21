import { h, Component } from 'preact';
import { Link, Router } from 'preact-router';
import SeriesForm from '../components/seriesForm';

interface NewProps {
    router?: Router;
    path: string;
}

export default class New extends Component<NewProps, void> {

  render() {
    return (
        <div class="container box">
            <SeriesForm />
            <Link class="button" href="/">back</Link>
        </div>
        );
  }
}
