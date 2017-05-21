import React from 'react';
import {render} from 'react-dom';
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom';

import Index from 'components/index/index';
import Tasks from 'components/index/tasks';
import Task from 'components/index/task';

export default function () {
  const router = (
    <Router>
      <div>
        <Link to="/tasks">tasks</Link>
        <Route exact path="/" component={Index}/>
        <Route exact path="/tasks" component={Tasks}/>
        <Route exact path="/task/:id" component={Task}/>
      </div>
    </Router>
  );
  render(router, document.getElementById('app'));
}
