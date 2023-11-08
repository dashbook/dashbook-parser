class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const fetchCompile = url => fetch(url).then(WebAssembly.compileStreaming);

const instantiateCore = WebAssembly.instantiate;

function toInt32(val) {
  return val >> 0;
}

const utf8Decoder = new TextDecoder();

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== 'string') throw new TypeError('expected a string');
  if (s.length === 0) {
    utf8EncodedLen = 0;
    return 1;
  }
  let allocLen = 0;
  let ptr = 0;
  let writtenTotal = 0;
  while (s.length > 0) {
    ptr = realloc(ptr, allocLen, 1, allocLen + s.length);
    allocLen += s.length;
    const { read, written } = utf8Encoder.encodeInto(
    s,
    new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal),
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  if (allocLen > writtenTotal)
  ptr = realloc(ptr, allocLen, 1, writtenTotal);
  utf8EncodedLen = writtenTotal;
  return ptr;
}

let exports0;
let memory0;
let realloc0;
let postReturn0;
let postReturn1;

function parse(arg0) {
  const ptr0 = utf8Encode(arg0, realloc0, memory0);
  const len0 = utf8EncodedLen;
  const ret = exports0['dashbook-parser#parse'](ptr0, len0);
  let variant9;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      const len7 = dataView(memory0).getInt32(ret + 8, true);
      const base7 = dataView(memory0).getInt32(ret + 4, true);
      const result7 = [];
      for (let i = 0; i < len7; i++) {
        const base = base7 + i * 24;
        let variant1;
        switch (dataView(memory0).getUint8(base + 0, true)) {
          case 0: {
            variant1= {
              tag: 'markdown',
            };
            break;
          }
          case 1: {
            variant1= {
              tag: 'code',
            };
            break;
          }
          case 2: {
            variant1= {
              tag: 'query',
            };
            break;
          }
          default: {
            throw new TypeError('invalid variant discriminant for CellType');
          }
        }
        const ptr2 = dataView(memory0).getInt32(base + 8, true);
        const len2 = dataView(memory0).getInt32(base + 12, true);
        const result2 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr2, len2));
        const len6 = dataView(memory0).getInt32(base + 20, true);
        const base6 = dataView(memory0).getInt32(base + 16, true);
        const result6 = [];
        for (let i = 0; i < len6; i++) {
          const base = base6 + i * 12;
          let variant5;
          switch (dataView(memory0).getUint8(base + 0, true)) {
            case 0: {
              const ptr3 = dataView(memory0).getInt32(base + 4, true);
              const len3 = dataView(memory0).getInt32(base + 8, true);
              const result3 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr3, len3));
              variant5= {
                tag: 'text',
                val: result3
              };
              break;
            }
            case 1: {
              const ptr4 = dataView(memory0).getInt32(base + 4, true);
              const len4 = dataView(memory0).getInt32(base + 8, true);
              const result4 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr4, len4));
              variant5= {
                tag: 'html',
                val: result4
              };
              break;
            }
            default: {
              throw new TypeError('invalid variant discriminant for CellOutput');
            }
          }
          result6.push(variant5);
        }
        result7.push({
          cellType: variant1,
          size: dataView(memory0).getInt32(base + 4, true),
          source: result2,
          outputs: result6,
        });
      }
      variant9= {
        tag: 'ok',
        val: result7
      };
      break;
    }
    case 1: {
      let variant8;
      switch (dataView(memory0).getUint8(ret + 4, true)) {
        case 0: {
          variant8= {
            tag: 'parse-error',
          };
          break;
        }
        default: {
          throw new TypeError('invalid variant discriminant for Error');
        }
      }
      variant9= {
        tag: 'err',
        val: variant8
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  postReturn0(ret);
  if (variant9.tag === 'err') {
    throw new ComponentError(variant9.val);
  }
  return variant9.val;
}

function generate(arg0) {
  const vec7 = arg0;
  const len7 = vec7.length;
  const result7 = realloc0(0, 0, 4, len7 * 24);
  for (let i = 0; i < vec7.length; i++) {
    const e = vec7[i];
    const base = result7 + i * 24;const {cellType: v0_0, size: v0_1, source: v0_2, outputs: v0_3 } = e;
    const variant1 = v0_0;
    switch (variant1.tag) {
      case 'markdown': {
        dataView(memory0).setInt8(base + 0, 0, true);
        break;
      }
      case 'code': {
        dataView(memory0).setInt8(base + 0, 1, true);
        break;
      }
      case 'query': {
        dataView(memory0).setInt8(base + 0, 2, true);
        break;
      }
      default: {
        throw new TypeError(`invalid variant ${JSON.stringify(variant1.tag)} specified for CellType`);
      }
    }
    dataView(memory0).setInt32(base + 4, toInt32(v0_1), true);
    const ptr2 = utf8Encode(v0_2, realloc0, memory0);
    const len2 = utf8EncodedLen;
    dataView(memory0).setInt32(base + 12, len2, true);
    dataView(memory0).setInt32(base + 8, ptr2, true);
    const vec6 = v0_3;
    const len6 = vec6.length;
    const result6 = realloc0(0, 0, 4, len6 * 12);
    for (let i = 0; i < vec6.length; i++) {
      const e = vec6[i];
      const base = result6 + i * 12;const variant5 = e;
      switch (variant5.tag) {
        case 'text': {
          const e = variant5.val;
          dataView(memory0).setInt8(base + 0, 0, true);
          const ptr3 = utf8Encode(e, realloc0, memory0);
          const len3 = utf8EncodedLen;
          dataView(memory0).setInt32(base + 8, len3, true);
          dataView(memory0).setInt32(base + 4, ptr3, true);
          break;
        }
        case 'html': {
          const e = variant5.val;
          dataView(memory0).setInt8(base + 0, 1, true);
          const ptr4 = utf8Encode(e, realloc0, memory0);
          const len4 = utf8EncodedLen;
          dataView(memory0).setInt32(base + 8, len4, true);
          dataView(memory0).setInt32(base + 4, ptr4, true);
          break;
        }
        default: {
          throw new TypeError(`invalid variant ${JSON.stringify(variant5.tag)} specified for CellOutput`);
        }
      }
    }
    dataView(memory0).setInt32(base + 20, len6, true);
    dataView(memory0).setInt32(base + 16, result6, true);
  }
  const ret = exports0['dashbook-parser#generate'](result7, len7);
  const ptr8 = dataView(memory0).getInt32(ret + 0, true);
  const len8 = dataView(memory0).getInt32(ret + 4, true);
  const result8 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr8, len8));
  postReturn1(ret);
  return result8;
}

const $init = (async() => {
  const module0 = fetchCompile(new URL('./index.core.wasm', import.meta.url));
  const instanceFlags0 = new WebAssembly.Global({ value: "i32", mutable: true }, 3);
  const instanceFlags1 = new WebAssembly.Global({ value: "i32", mutable: true }, 3);
  ({ exports: exports0 } = await instantiateCore(await module0));
  memory0 = exports0.memory;
  realloc0 = exports0.cabi_realloc;
  postReturn0 = exports0['cabi_post_dashbook-parser#parse'];
  postReturn1 = exports0['cabi_post_dashbook-parser#generate'];
})();

await $init;
const dashbookParser = {
  generate: generate,
  parse: parse,
  
};

export { dashbookParser }