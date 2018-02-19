import { h, Component } from 'preact';
import { route, Link } from 'preact-router';
import SeriesForm from './seriesForm';
import Store from '../store';
import '../model';

interface EditState {
    series: Series;
    info: Array<InfoBlob>;
}

interface EditProps {
    path: string;
    matches?: {
        id: number;
    };
}

export default class SeriesEdit extends Component<EditProps, EditState> {

    constructor() {
        super();

        this.state = {
            series: null,
            info: null,
        }
    }

    componentDidMount() {
        this.getSeries();
    }

    getSeries() {
        let self = this;

        Promise.all([
            Store.getSeriesId(this.props.matches.id),
            Store.getSeriesInfo(this.props.matches.id)
        ])
            .then(result => {

                self.setState({
                    series: result[0],
                    info: result[1],
                });
            })
            .catch(err => {
                console.log(err);
                route('/');
            });
    }


    render() {
        if (this.state.series === null) {
            return (
                <div>
                <p>loading...</p>
                <Link href='/'>back</Link>
                </div>
            );
        }

        let series: any = this.state.series;
        series.info = this.state.info;

        return (
            <div>
            <SeriesForm series={ series } />
            <Link href={ `/series/${ this.state.series.id }` }>back</Link>
            </div>
        );
    }
}