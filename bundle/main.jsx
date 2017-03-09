import React from 'react';
import ReactDOM, { render } from 'react-dom';
import Series from './series.jsx';

window.onload = function() {
  var series = {title: "hi"};
  render(<Series series={ series } />, document.getElementById('greeter'));
}
