import cluster from 'cluster'
import http from 'http'
import { cpus } from 'os'

const CPU_CORES = cpus().length

if (cluster.isMaster) {
  console.info(`Master ${process.pid} is running`)

  // Fork workers.
  for (let i = 0; i < CPU_CORES; i++) {
    cluster.fork()
  }

  cluster.on('exit', (worker) => {
    console.info(`worker ${worker.process.pid} died`)
  })
} else {
  // Workers can share any TCP connection
  // In this case it is an HTTP server
  http
    .createServer((_req, res) => {
      res.writeHead(200)
      res.end('Hello!')
    })
    .listen(3000)

  console.info(`Worker ${process.pid} started`)
}
