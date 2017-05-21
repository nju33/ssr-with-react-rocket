import React, {Component} from 'react';
import {Link} from 'react-router-dom';

export default class Tasks extends Component {
  constructor(props) {
    super(props);
    this.state = {task: null};
  }

  async componentDidMount() {
    const {id} = this.props.match.params;
    const res = await fetch(`http://localhost:8000/tasks?id=${id}`);
    const [task] = await res.json();
    this.setState({task});
  }

  render() {
    if (this.state.task === null) {
      return <div>Null</div>;
    }

    return (
      <div>
        <div>{this.state.task.task}</div>
        <div><input type="checkbox" value={this.state.task.done}/></div>
      </div>
    );
  }
}
