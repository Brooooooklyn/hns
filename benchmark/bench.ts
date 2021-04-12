import { createServer } from 'http'
 
import b from 'benny'

import { createApp } from '../index'

const PORT = 3000

async function createHttpServer() {
  return new Promise<void>((resolve, reject) => {
    const server = createServer()
    server.listen(PORT, () => {
      server.close((err) => {
        if (err) {
          reject(err)
        } else {
          resolve()
        }
      })
    }).addListener('error', (err) => { reject(err) })
  })
}

async function run() {
  await b.suite(
    'Listen',

    b.add('Node.js http', async () => {
      await createHttpServer()

    }),

    b.add('hns createApp', async () => {
      await createApp(PORT)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
