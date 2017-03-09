import React from 'react';

class Series extends React.Component {
  constructor() {
    super();
  } 

  renderCurrent() {
    var series = this.props.series;
    if (series.start_date) {
      return (<p>Start date: {series.start_date}</p>)
    } else {
      return (<p>Start date: unkown</p>)
    }
  }

  render() {
    var series = this.props.series;
    return (<div>
      <h1>{ series.title }</h1>
        <div>
            { this.renderCurrent() }
        </div>
    </div>);
  }
}

export default Series;
