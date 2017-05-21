import React, {Component} from 'react';
import {Link} from 'react-router-dom';

export default class Tasks extends Component {
  constructor(props) {
    super(props);
    this.state = {tasks: []};
  }

  async componentDidMount() {
    const res = await fetch(`http://localhost:8000/tasks`);
    const tasks = await res.json();
    this.setState({tasks});
  }

  render() {
    return (
      <div>
        {this.state.tasks.map(task => (
          <Link to={'task/' + task.id} key={task.id}>{task.task}</Link>
        ))}
      </div>
    );
  }
}
