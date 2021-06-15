import { join } from 'path'

import { loadBinding } from '@node-rs/helper'

import { Method, Version } from './const'
import { HttpRequest, BodyExternal } from './request'

const {
  createApp: createNativeApp,
  getBodyText,
  getBodyBinary,
  getBodyJson,
} = loadBinding(join(__dirname, '..', '..'), 'hns', '@hnsjs/core')

export interface HttpResponse {}

export function createApp(port: number, onRequest?: (req: HttpRequest) => Promise<HttpResponse> | HttpResponse) {
  return new Promise<void>((resolve, reject) => {
    createNativeApp(
      port,
      (err: Error | null) => {
        if (err) {
          reject(err)
        } else {
          resolve()
        }
      },
      (err: Error | null, version: Version, method: Method, uri: string, headers: string, body: BodyExternal) => {
        if (err) {
          console.error(err)
        }
        const req: HttpRequest = {
          version,
          method,
          uri,
          headers: JSON.parse(headers),
          body: {
            text() {
              return getBodyText(body)
            },
            binary() {
              return getBodyBinary(body)
            },
            json() {
              return getBodyJson(body)
            },
          },
        }
        if (onRequest) {
          Promise.resolve(onRequest(req)).catch((e) => {
            console.error(e)
          })
        }
      },
    ).catch(reject)
  })
}
