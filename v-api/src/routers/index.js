import KoaRouter from 'koa-router';
import React from 'react';
import {renderToString} from 'react-dom/server';
import {
  StaticRouter as Router,
  Route
} from 'react-router';
import Index from 'components/index/index';
import Hello from 'components/index/hello';

const router = new KoaRouter();

router.get('/', (ctx, next) => {
  ctx.body = renderToString(
    <Router location={'/'} context={{}}>
      <div>
        <Route exact path="/" component={Index}/>
        <Route exact path="/hello" component={Hello}/>
      </div>
    </Router>
  );
});

router.get('/a', (ctx, next) => {
  ctx.body = 'Hello Koa';
});

export default router;
