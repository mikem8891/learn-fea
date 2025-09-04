let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let WASM_VECTOR_LEN = 0;

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * @param {number} e
 * @param {number} nu
 * @param {number} g
 * @returns {Lin2DStaticModel}
 */
export function init_fea(e, nu, g) {
    const ret = wasm.init_fea(e, nu, g);
    return Lin2DStaticModel.__wrap(ret);
}

export function main() {
    wasm.main();
}

/**
 * @enum {0 | 1}
 */
export const KnownType = Object.freeze({
    Force: 0, "0": "Force",
    Displacement: 1, "1": "Displacement",
});

const Lin2DStaticModelFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_lin2dstaticmodel_free(ptr >>> 0, 1));

export class Lin2DStaticModel {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Lin2DStaticModel.prototype);
        obj.__wbg_ptr = ptr;
        Lin2DStaticModelFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Lin2DStaticModelFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_lin2dstaticmodel_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    elements_len() {
        const ret = wasm.lin2dstaticmodel_elements_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} index
     * @returns {Uint32Array}
     */
    get_element_indices(index) {
        const ret = wasm.lin2dstaticmodel_get_element_indices(this.__wbg_ptr, index);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {number} element_index
     * @param {Uint32Array} new_indices
     */
    set_element_indices(element_index, new_indices) {
        const ptr0 = passArray32ToWasm0(new_indices, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.lin2dstaticmodel_set_element_indices(this.__wbg_ptr, element_index, ptr0, len0);
    }
    step() {
        wasm.lin2dstaticmodel_step(this.__wbg_ptr);
    }
    add_elem() {
        wasm.lin2dstaticmodel_add_elem(this.__wbg_ptr);
    }
    add_node() {
        wasm.lin2dstaticmodel_add_node(this.__wbg_ptr);
    }
    /**
     * @param {number} index
     * @returns {Node2D}
     */
    get_node(index) {
        const ret = wasm.lin2dstaticmodel_get_node(this.__wbg_ptr, index);
        return Node2D.__wrap(ret);
    }
    /**
     * @param {number} index
     * @param {Node2D} node
     */
    set_node(index, node) {
        _assertClass(node, Node2D);
        wasm.lin2dstaticmodel_set_node(this.__wbg_ptr, index, node.__wbg_ptr);
    }
    /**
     * @returns {number}
     */
    nodes_len() {
        const ret = wasm.lin2dstaticmodel_nodes_len(this.__wbg_ptr);
        return ret >>> 0;
    }
}

const Node2DFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_node2d_free(ptr >>> 0, 1));

export class Node2D {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Node2D.prototype);
        obj.__wbg_ptr = ptr;
        Node2DFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Node2DFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_node2d_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get dispX() {
        const ret = wasm.node2d_get_disp_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get dispY() {
        const ret = wasm.node2d_get_disp_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} value
     */
    set dispX(value) {
        wasm.node2d_set_disp_x(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set dispY(value) {
        wasm.node2d_set_disp_y(this.__wbg_ptr, value);
    }
    /**
     * @returns {number}
     */
    get forceX() {
        const ret = wasm.node2d_get_force_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get forceY() {
        const ret = wasm.node2d_get_force_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {KnownType}
     */
    get knownX() {
        const ret = wasm.node2d_get_known_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {KnownType}
     */
    get knownY() {
        const ret = wasm.node2d_get_known_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} value
     */
    set forceX(value) {
        wasm.node2d_set_force_x(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set forceY(value) {
        wasm.node2d_set_force_y(this.__wbg_ptr, value);
    }
    /**
     * @param {KnownType} known
     */
    set knownX(known) {
        wasm.node2d_set_known_x(this.__wbg_ptr, known);
    }
    /**
     * @param {KnownType} known
     */
    set knownY(known) {
        wasm.node2d_set_known_y(this.__wbg_ptr, known);
    }
    /**
     * @returns {number}
     */
    get posX() {
        const ret = wasm.node2d_get_pos_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get posY() {
        const ret = wasm.node2d_get_pos_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} value
     */
    set posX(value) {
        wasm.node2d_set_pos_x(this.__wbg_ptr, value);
    }
    /**
     * @param {number} value
     */
    set posY(value) {
        wasm.node2d_set_pos_y(this.__wbg_ptr, value);
    }
}

const T3ElementFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_t3element_free(ptr >>> 0, 1));

export class T3Element {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        T3ElementFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_t3element_free(ptr, 0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_log_c222819a41e063d3 = function(arg0) {
        console.log(arg0);
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('learn_fea_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
