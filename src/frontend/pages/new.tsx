import { Component, h } from "preact";
import { Link, Router } from "preact-router";
import SeriesForm from "../components/seriesForm";

interface INewProps {
    router?: Router;
    path: string;
}

export default class New extends Component<INewProps, void> {

  public render() {
    return (
        <div class="container box">
            <SeriesForm back="/" />
        </div>
        );
  }
}
