import { execSync } from 'child_process'
import { createServer } from 'http'

import { createApp } from '../index'

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
  execSync(`npx autocannon -c 8 -w 4 -d 30 http://localhost:${PORT}`, {
    stdio: 'inherit',
  })
  execSync(`npx autocannon -c 8 -w 4 -d 30 http://localhost:${PORT + 1}`, {
    stdio: 'inherit',
  })
  process.exit(0)
}

run().catch((e) => {
  console.error(e)
})
