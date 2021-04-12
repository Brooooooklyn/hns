import test from 'ava'
// @ts-expect-error
import { request } from 'undici'

import { createApp } from '../index'

const PORT = 3000

test('should be able to create app', async (t) => {
  createApp(PORT)
  const { statusCode } = await request(`http://127.0.0.1:${PORT}`)
  t.is(statusCode, 200)
})
