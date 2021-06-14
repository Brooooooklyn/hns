import test from 'ava'
import { request } from 'undici'

import { createApp } from '../index'

const PORT = 3001

test.before(async () => {
  await createApp(PORT)
})

test('should be able to create app', async (t) => {
  const { statusCode } = await request(`http://127.0.0.1:${PORT}`)
  t.is(statusCode, 200)
})
