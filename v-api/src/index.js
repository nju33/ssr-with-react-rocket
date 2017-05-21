import Koa from 'koa';
import cors from 'kcors';
import logger from 'koa-logger';
import indexRouter from 'routers/index';

const app = new Koa();

app
  .use(cors({
    origin: "http://localhost:8080"
  }))
  .use(indexRouter.routes())
  .use(indexRouter.allowedMethods())
  .use(logger);

app.listen(8100);
