import React from 'react';
import ReactDOM, { render } from 'react-dom';
import Greeter from './test.jsx';

window.onload = function() {
  render(<Greeter name='You'/>, document.getElementById('greeter'));
}
