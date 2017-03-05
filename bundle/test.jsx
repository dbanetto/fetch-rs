import React from 'react';

class Greeter extends React.Component {

  render() {
    return(
        <div>
          <p>Hi {this.props.name}</p>
        </div>
        );
  }
}

export default Greeter;
