/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function dbNew(name: string): Db
export class Db {
  addIfNotExist(id: number, payload: string, img: Buffer, ext?: string | undefined | null): Promise<bigint>
  add(id: number, payload: string, img: Buffer, ext?: string | undefined | null): Promise<void>
}
