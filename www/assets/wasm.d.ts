/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input_text
* @param {string} key
* @returns {string}
*/
export function decrypt(input_text: string, key: string): string;
/**
* @param {string} input_text
* @param {string} key
* @returns {string}
*/
export function encrypt(input_text: string, key: string): string;
/**
* @returns {string}
*/
export function generate_key(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly decrypt: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly encrypt: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly generate_key: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
