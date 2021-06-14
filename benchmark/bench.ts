import { exec } from 'child_process'
import { createServer } from 'http'

import { createApp } from '@hnsjs/core'

const PORT = 3000

function createHttpServer() {
  return new Promise<void>((resolve, reject) => {
    const server = createServer((req, res) => {
      if (req.url === '/favicon.ico') return
      if (req.url === '/') return res.end('Hello!')
    })
    server
      .listen(PORT, 'localhost')
      .on('listening', () => {
        resolve()
      })
      .on('error', (err) => {
        reject(err)
      })
  })
}

async function run() {
  await createHttpServer()
  await createApp(PORT + 1)
  await wrk(PORT)
  await wrk(PORT + 1)
  process.exit(0)
}

function wrk(port: number) {
  return new Promise<void>((resolve, reject) => {
    exec(`npx autocannon -c 8 -w 4 -d 30 http://localhost:${port}`, (err, stdout, stderr) => {
      if (err) {
        reject(err)
      } else {
        console.info(stdout)
        console.info(stderr)
        resolve()
      }
    })
  })
}

run().catch((e) => {
  console.error(e)
})
