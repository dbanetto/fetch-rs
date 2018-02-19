import { h, Component } from 'preact';
import { Link, Router } from 'preact-router';
import SeriesForm from './seriesForm';

interface NewProps {
    router?: Router;
    path: string;
}

export default class SeriesNew extends Component<NewProps, void> {

  render() {
    return (
        <div>
          <SeriesForm />
          <Link href="/">back</Link>
        </div>
        );
  }
}
