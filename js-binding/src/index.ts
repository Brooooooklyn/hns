import { join } from 'path'

import { loadBinding } from '@node-rs/helper'

const { createApp: createNativeApp } = loadBinding(join(__dirname, '..', '..'), 'hns', '@hnsjs/core')

export const enum Version {
  HTTP_09 = 'HTTP/0.9',
  HTTP_10 = 'HTTP/1.0',
  HTTP_1_1 = 'HTTP/1.1',
  H2 = 'HTTP/2.0',
  H3 = 'HTTP/3.0',
}

export const enum Method {
  Options = 'OPTIONS',
  Get = 'GET',
  Post = 'POST',
  Put = 'PUT',
  Delete = 'DELETE',
  Head = 'HEAD',
  Trace = 'TRACE',
  Connect = 'CONNECT',
  Patch = 'PATCH',
}

export interface HttpRequest {
  version: Version
  method: Method
  uri: string
  headers: Record<string, string>
  body: RequestBody
}

export interface RequestBody {}

export interface HttpResponse {}

type BodyExternal = {
  __type: 'native:external:body'
}

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
          body,
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
