import { Version, Method } from './const'

export type BodyExternal = {
  __type: 'native:external:body'
}

export interface HttpRequest {
  version: Version
  method: Method
  uri: string
  headers: Record<string, string>
  body: RequestBody
}

export interface RequestBody {
  binary: () => Promise<Buffer>
  json: <T>() => Promise<T>
  text: () => Promise<string>
}
