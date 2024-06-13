// .wrangler/tmp/bundle-ludKfy/checked-fetch.js
var urls = /* @__PURE__ */ new Set();
function checkURL(request, init) {
  const url = request instanceof URL ? request : new URL(
    (typeof request === "string" ? new Request(request, init) : request).url
  );
  if (url.port && url.port !== "443" && url.protocol === "https:") {
    if (!urls.has(url.toString())) {
      urls.add(url.toString());
      console.warn(
        `WARNING: known issue with \`fetch()\` requests to custom HTTPS ports in published Workers:
 - ${url.toString()} - the custom port will be ignored when the Worker is published using the \`wrangler deploy\` command.
`
      );
    }
  }
}
globalThis.fetch = new Proxy(globalThis.fetch, {
  apply(target, thisArg, argArray) {
    const [request, init] = argArray;
    checkURL(request, init);
    return Reflect.apply(target, thisArg, argArray);
  }
});

// build/worker/shim.mjs
import $ from "./cd67ddcd70854cc8aaec32bd555ebe45a3d7c9a3-index.wasm";
import Ce from "./cd67ddcd70854cc8aaec32bd555ebe45a3d7c9a3-index.wasm";
import { WorkerEntrypoint as Se } from "cloudflare:workers";
var D = Object.defineProperty;
var P = (t, e) => {
  for (var n in e)
    D(t, n, { get: e[n], enumerable: true });
};
var g = {};
P(g, { IntoUnderlyingByteSource: () => T, IntoUnderlyingSink: () => A, IntoUnderlyingSource: () => x, MinifyConfig: () => q, PolishConfig: () => Z, R2Range: () => L, RequestRedirect: () => Y, __wbg_String_b9412f8799faab3e: () => dt, __wbg_append_7bfcb4937d1d5e29: () => Pt, __wbg_body_17b435cb52dcf45f: () => Ct, __wbg_buffer_12d079cc21e14bdb: () => de, __wbg_buffer_dd7f74bc60f1faab: () => xe, __wbg_byobRequest_72fca99f9c32c193: () => Ot, __wbg_byteLength_58f7b4fab1919d44: () => me, __wbg_byteOffset_81d60f7392524f62: () => ke, __wbg_call_27c0f87801dedf93: () => Yt, __wbg_call_b3ca7c6051f9bec1: () => se, __wbg_cancel_6ee33d4006737aef: () => Et, __wbg_catch_0260e338d10f79ae: () => ae, __wbg_cause_3d9c85ebaf6b1155: () => ie, __wbg_cf_ab668814697435ac: () => kt, __wbg_close_184931724d961ccc: () => jt, __wbg_close_a994f9425dab445c: () => Ut, __wbg_done_298b57d23c0fc80c: () => Kt, __wbg_done_2ffa852272310e47: () => pt, __wbg_enqueue_ea194723156c0cc2: () => Wt, __wbg_entries_2f5d638b6f300812: () => mt, __wbg_error_8e3928cfb8a43e2b: () => It, __wbg_error_f851667af71bcfc6: () => ut, __wbg_getReader_ab94afcb5cb7689a: () => wt, __wbg_get_bd8e338fbd5f5cc8: () => Jt, __wbg_globalThis_d1e6af4856ba331b: () => ne, __wbg_global_207b558942527489: () => re, __wbg_headers_abb199c3be8d817c: () => qt, __wbg_httpProtocol_791d2bb087dc5b51: () => xt, __wbg_instanceof_Error_e20bb56fd5591a93: () => _e, __wbg_length_c20a40f15020d68a: () => ye, __wbg_method_83327ed2e3f3229c: () => Tt, __wbg_new_28c511d9baebfa89: () => oe, __wbg_new_63b92bc8671ed464: () => we, __wbg_new_72fb9a18b5ae2624: () => Zt, __wbg_new_81740750da40724f: () => ue, __wbg_new_ab6fd82b10560829: () => Dt, __wbg_new_abda76e883ba8a5f: () => ct, __wbg_newnoargs_e258087cd0daa0ea: () => Vt, __wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb: () => le, __wbg_newwithintounderlyingsource_a03a82aa1bbbb292: () => ht, __wbg_newwithlength_e9b4878cebadb3d3: () => he, __wbg_newwithoptbuffersourceandinit_a4fa81e77259bb96: () => Rt, __wbg_newwithoptreadablestreamandinit_0b825f969ca543d6: () => Mt, __wbg_newwithoptstrandinit_219732174c595a25: () => Ft, __wbg_next_196c84450b364254: () => Gt, __wbg_queueMicrotask_3cbae2ec6b6cd3d6: () => Bt, __wbg_queueMicrotask_481971b0d87f3dd4: () => Xt, __wbg_read_e7d0f8a49be01d86: () => zt, __wbg_redirect_53aa50ab48e7a227: () => Lt, __wbg_releaseLock_5c49db976c08b864: () => St, __wbg_resolve_b0083a7967828ec8: () => fe, __wbg_respond_b1a43b2e3a06d525: () => Nt, __wbg_self_ce0dbfc45cf2f5be: () => te, __wbg_set_1f9b04f170055d33: () => Re, __wbg_set_a47bac70306a19a7: () => pe, __wbg_sethighWaterMark_ea50ed3ec2143088: () => lt, __wbg_signal_e0b0ea9dce5137b3: () => vt, __wbg_stack_658279fe44541cf6: () => st, __wbg_then_0c86a60e8fcfe9f6: () => be, __wbg_then_a73caa9a87991566: () => ge, __wbg_toString_ffe4c9ea3b3532e9: () => ce, __wbg_url_7807f6a1fddc3e23: () => At, __wbg_value_9f6eeb1e2aab8d96: () => yt, __wbg_value_d93c65011f51a456: () => Qt, __wbg_view_7f0ce470793a340f: () => $t, __wbg_window_c6fb939a7f436783: () => ee, __wbindgen_cb_drop: () => it, __wbindgen_closure_wrapper458: () => je, __wbindgen_closure_wrapper974: () => ze, __wbindgen_debug_string: () => Fe, __wbindgen_is_function: () => Ht, __wbindgen_is_undefined: () => gt, __wbindgen_memory: () => Oe, __wbindgen_number_new: () => bt, __wbindgen_object_clone_ref: () => ft, __wbindgen_object_drop_ref: () => ot, __wbindgen_string_get: () => at, __wbindgen_string_new: () => _t, __wbindgen_throw: () => Me, fetch: () => v, getMemory: () => B });
var N = new WebAssembly.Instance($, { "./index_bg.js": g });
var o = N.exports;
function B() {
  return o.memory;
}
var H = typeof TextDecoder > "u" ? (0, module.require)("util").TextDecoder : TextDecoder;
var W = new H("utf-8", { ignoreBOM: true, fatal: true });
W.decode();
var R = null;
function O() {
  return (R === null || R.byteLength === 0) && (R = new Uint8Array(o.memory.buffer)), R;
}
function p(t, e) {
  return t = t >>> 0, W.decode(O().subarray(t, t + e));
}
var l = new Array(128).fill(void 0);
l.push(void 0, null, true, false);
var k = l.length;
function i(t) {
  k === l.length && l.push(l.length + 1);
  let e = k;
  return k = l[e], l[e] = t, e;
}
function r(t) {
  return l[t];
}
function X(t) {
  t < 132 || (l[t] = k, k = t);
}
function w(t) {
  let e = r(t);
  return X(t), e;
}
var y = 0;
var J = typeof TextEncoder > "u" ? (0, module.require)("util").TextEncoder : TextEncoder;
var j = new J("utf-8");
var V = typeof j.encodeInto == "function" ? function(t, e) {
  return j.encodeInto(t, e);
} : function(t, e) {
  let n = j.encode(t);
  return e.set(n), { read: t.length, written: n.length };
};
function h(t, e, n) {
  if (n === void 0) {
    let b = j.encode(t), m = e(b.length, 1) >>> 0;
    return O().subarray(m, m + b.length).set(b), y = b.length, m;
  }
  let _ = t.length, c = e(_, 1) >>> 0, a = O(), u = 0;
  for (; u < _; u++) {
    let b = t.charCodeAt(u);
    if (b > 127)
      break;
    a[c + u] = b;
  }
  if (u !== _) {
    u !== 0 && (t = t.slice(u)), c = n(c, _, _ = u + t.length * 3, 1) >>> 0;
    let b = O().subarray(c + u, c + _), m = V(t, b);
    u += m.written, c = n(c, _, u, 1) >>> 0;
  }
  return y = u, c;
}
function d(t) {
  return t == null;
}
var F = null;
function f() {
  return (F === null || F.byteLength === 0) && (F = new Int32Array(o.memory.buffer)), F;
}
function E(t) {
  let e = typeof t;
  if (e == "number" || e == "boolean" || t == null)
    return `${t}`;
  if (e == "string")
    return `"${t}"`;
  if (e == "symbol") {
    let c = t.description;
    return c == null ? "Symbol" : `Symbol(${c})`;
  }
  if (e == "function") {
    let c = t.name;
    return typeof c == "string" && c.length > 0 ? `Function(${c})` : "Function";
  }
  if (Array.isArray(t)) {
    let c = t.length, a = "[";
    c > 0 && (a += E(t[0]));
    for (let u = 1; u < c; u++)
      a += ", " + E(t[u]);
    return a += "]", a;
  }
  let n = /\[object ([^\]]+)\]/.exec(toString.call(t)), _;
  if (n.length > 1)
    _ = n[1];
  else
    return toString.call(t);
  if (_ == "Object")
    try {
      return "Object(" + JSON.stringify(t) + ")";
    } catch {
      return "Object";
    }
  return t instanceof Error ? `${t.name}: ${t.message}
${t.stack}` : _;
}
var C = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => {
  o.__wbindgen_export_2.get(t.dtor)(t.a, t.b);
});
function I(t, e, n, _) {
  let c = { a: t, b: e, cnt: 1, dtor: n }, a = (...u) => {
    c.cnt++;
    let b = c.a;
    c.a = 0;
    try {
      return _(b, c.b, ...u);
    } finally {
      --c.cnt === 0 ? (o.__wbindgen_export_2.get(c.dtor)(b, c.b), C.unregister(c)) : c.a = b;
    }
  };
  return a.original = c, C.register(a, c, c), a;
}
function G(t, e, n) {
  o._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha0317c550dd0ae18(t, e, i(n));
}
function K(t, e, n) {
  o._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd59cc1687fa8e181(t, e, i(n));
}
function v(t, e, n) {
  let _ = o.fetch(i(t), i(e), i(n));
  return w(_);
}
function s(t, e) {
  try {
    return t.apply(this, e);
  } catch (n) {
    o.__wbindgen_exn_store(i(n));
  }
}
var M = null;
function S() {
  return (M === null || M.byteLength === 0) && (M = new Float64Array(o.memory.buffer)), M;
}
function Q(t, e, n, _) {
  o.wasm_bindgen__convert__closures__invoke2_mut__h851c4447800cf066(t, e, i(n), i(_));
}
var Y = Object.freeze({ Error: 0, 0: "Error", Follow: 1, 1: "Follow", Manual: 2, 2: "Manual" });
var Z = Object.freeze({ Off: 0, 0: "Off", Lossy: 1, 1: "Lossy", Lossless: 2, 2: "Lossless" });
var tt = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_intounderlyingbytesource_free(t >>> 0));
var T = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, tt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_intounderlyingbytesource_free(e);
  }
  get type() {
    let e, n;
    try {
      let a = o.__wbindgen_add_to_stack_pointer(-16);
      o.intounderlyingbytesource_type(a, this.__wbg_ptr);
      var _ = f()[a / 4 + 0], c = f()[a / 4 + 1];
      return e = _, n = c, p(_, c);
    } finally {
      o.__wbindgen_add_to_stack_pointer(16), o.__wbindgen_free(e, n, 1);
    }
  }
  get autoAllocateChunkSize() {
    return o.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  start(e) {
    o.intounderlyingbytesource_start(this.__wbg_ptr, i(e));
  }
  pull(e) {
    let n = o.intounderlyingbytesource_pull(this.__wbg_ptr, i(e));
    return w(n);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    o.intounderlyingbytesource_cancel(e);
  }
};
var et = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_intounderlyingsink_free(t >>> 0));
var A = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, et.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_intounderlyingsink_free(e);
  }
  write(e) {
    let n = o.intounderlyingsink_write(this.__wbg_ptr, i(e));
    return w(n);
  }
  close() {
    let e = this.__destroy_into_raw(), n = o.intounderlyingsink_close(e);
    return w(n);
  }
  abort(e) {
    let n = this.__destroy_into_raw(), _ = o.intounderlyingsink_abort(n, i(e));
    return w(_);
  }
};
var U = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_intounderlyingsource_free(t >>> 0));
var x = class {
  static __wrap(e) {
    e = e >>> 0;
    let n = Object.create(x.prototype);
    return n.__wbg_ptr = e, U.register(n, n.__wbg_ptr, n), n;
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, U.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_intounderlyingsource_free(e);
  }
  pull(e) {
    let n = o.intounderlyingsource_pull(this.__wbg_ptr, i(e));
    return w(n);
  }
  cancel() {
    let e = this.__destroy_into_raw();
    o.intounderlyingsource_cancel(e);
  }
};
var nt = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_minifyconfig_free(t >>> 0));
var q = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, nt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_minifyconfig_free(e);
  }
  get js() {
    return o.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set js(e) {
    o.__wbg_set_minifyconfig_js(this.__wbg_ptr, e);
  }
  get html() {
    return o.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  set html(e) {
    o.__wbg_set_minifyconfig_html(this.__wbg_ptr, e);
  }
  get css() {
    return o.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  set css(e) {
    o.__wbg_set_minifyconfig_css(this.__wbg_ptr, e);
  }
};
var rt = typeof FinalizationRegistry > "u" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((t) => o.__wbg_r2range_free(t >>> 0));
var L = class {
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, rt.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o.__wbg_r2range_free(e);
  }
  get offset() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_offset(_, this.__wbg_ptr);
      var e = f()[_ / 4 + 0], n = S()[_ / 8 + 1];
      return e === 0 ? void 0 : n;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set offset(e) {
    o.__wbg_set_r2range_offset(this.__wbg_ptr, !d(e), d(e) ? 0 : e);
  }
  get length() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_length(_, this.__wbg_ptr);
      var e = f()[_ / 4 + 0], n = S()[_ / 8 + 1];
      return e === 0 ? void 0 : n;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set length(e) {
    o.__wbg_set_r2range_length(this.__wbg_ptr, !d(e), d(e) ? 0 : e);
  }
  get suffix() {
    try {
      let _ = o.__wbindgen_add_to_stack_pointer(-16);
      o.__wbg_get_r2range_suffix(_, this.__wbg_ptr);
      var e = f()[_ / 4 + 0], n = S()[_ / 8 + 1];
      return e === 0 ? void 0 : n;
    } finally {
      o.__wbindgen_add_to_stack_pointer(16);
    }
  }
  set suffix(e) {
    o.__wbg_set_r2range_suffix(this.__wbg_ptr, !d(e), d(e) ? 0 : e);
  }
};
function _t(t, e) {
  let n = p(t, e);
  return i(n);
}
function ot(t) {
  w(t);
}
function it(t) {
  let e = w(t).original;
  return e.cnt-- == 1 ? (e.a = 0, true) : false;
}
function ct() {
  let t = new Error();
  return i(t);
}
function st(t, e) {
  let n = r(e).stack, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function ut(t, e) {
  let n, _;
  try {
    n = t, _ = e, console.error(p(t, e));
  } finally {
    o.__wbindgen_free(n, _, 1);
  }
}
function ft(t) {
  let e = r(t);
  return i(e);
}
function at(t, e) {
  let n = r(e), _ = typeof n == "string" ? n : void 0;
  var c = d(_) ? 0 : h(_, o.__wbindgen_malloc, o.__wbindgen_realloc), a = y;
  f()[t / 4 + 1] = a, f()[t / 4 + 0] = c;
}
function bt(t) {
  return i(t);
}
function gt(t) {
  return r(t) === void 0;
}
function dt(t, e) {
  let n = String(r(e)), _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function lt(t, e) {
  r(t).highWaterMark = e;
}
function wt() {
  return s(function(t) {
    let e = r(t).getReader();
    return i(e);
  }, arguments);
}
function pt(t) {
  return r(t).done;
}
function yt(t) {
  let e = r(t).value;
  return i(e);
}
function ht(t, e) {
  let n = new ReadableStream(x.__wrap(t), w(e));
  return i(n);
}
function xt() {
  return s(function(t, e) {
    let n = r(e).httpProtocol, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
    f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
  }, arguments);
}
function mt() {
  return s(function(t) {
    let e = r(t).entries();
    return i(e);
  }, arguments);
}
function kt() {
  return s(function(t) {
    let e = r(t).cf;
    return d(e) ? 0 : i(e);
  }, arguments);
}
function Rt() {
  return s(function(t, e) {
    let n = new Response(r(t), r(e));
    return i(n);
  }, arguments);
}
function Ft() {
  return s(function(t, e, n) {
    let _ = new Response(t === 0 ? void 0 : p(t, e), r(n));
    return i(_);
  }, arguments);
}
function Mt() {
  return s(function(t, e) {
    let n = new Response(r(t), r(e));
    return i(n);
  }, arguments);
}
function Ot(t) {
  let e = r(t).byobRequest;
  return d(e) ? 0 : i(e);
}
function jt() {
  return s(function(t) {
    r(t).close();
  }, arguments);
}
function zt(t) {
  let e = r(t).read();
  return i(e);
}
function St(t) {
  r(t).releaseLock();
}
function Et(t) {
  let e = r(t).cancel();
  return i(e);
}
function Tt(t, e) {
  let n = r(e).method, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function At(t, e) {
  let n = r(e).url, _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function qt(t) {
  let e = r(t).headers;
  return i(e);
}
function Lt(t) {
  let e = r(t).redirect;
  return i(e);
}
function vt(t) {
  let e = r(t).signal;
  return i(e);
}
function Ct(t) {
  let e = r(t).body;
  return d(e) ? 0 : i(e);
}
function Ut() {
  return s(function(t) {
    r(t).close();
  }, arguments);
}
function Wt() {
  return s(function(t, e) {
    r(t).enqueue(r(e));
  }, arguments);
}
function It(t) {
  console.error(r(t));
}
function Dt() {
  return s(function() {
    let t = new Headers();
    return i(t);
  }, arguments);
}
function Pt() {
  return s(function(t, e, n, _, c) {
    r(t).append(p(e, n), p(_, c));
  }, arguments);
}
function $t(t) {
  let e = r(t).view;
  return d(e) ? 0 : i(e);
}
function Nt() {
  return s(function(t, e) {
    r(t).respond(e >>> 0);
  }, arguments);
}
function Bt(t) {
  let e = r(t).queueMicrotask;
  return i(e);
}
function Ht(t) {
  return typeof r(t) == "function";
}
function Xt(t) {
  queueMicrotask(r(t));
}
function Jt(t, e) {
  let n = r(t)[e >>> 0];
  return i(n);
}
function Vt(t, e) {
  let n = new Function(p(t, e));
  return i(n);
}
function Gt() {
  return s(function(t) {
    let e = r(t).next();
    return i(e);
  }, arguments);
}
function Kt(t) {
  return r(t).done;
}
function Qt(t) {
  let e = r(t).value;
  return i(e);
}
function Yt() {
  return s(function(t, e) {
    let n = r(t).call(r(e));
    return i(n);
  }, arguments);
}
function Zt() {
  let t = new Object();
  return i(t);
}
function te() {
  return s(function() {
    let t = self.self;
    return i(t);
  }, arguments);
}
function ee() {
  return s(function() {
    let t = window.window;
    return i(t);
  }, arguments);
}
function ne() {
  return s(function() {
    let t = globalThis.globalThis;
    return i(t);
  }, arguments);
}
function re() {
  return s(function() {
    let t = global.global;
    return i(t);
  }, arguments);
}
function _e(t) {
  let e;
  try {
    e = r(t) instanceof Error;
  } catch {
    e = false;
  }
  return e;
}
function oe(t, e) {
  let n = new Error(p(t, e));
  return i(n);
}
function ie(t) {
  let e = r(t).cause;
  return i(e);
}
function ce(t) {
  let e = r(t).toString();
  return i(e);
}
function se() {
  return s(function(t, e, n) {
    let _ = r(t).call(r(e), r(n));
    return i(_);
  }, arguments);
}
function ue(t, e) {
  try {
    var n = { a: t, b: e }, _ = (a, u) => {
      let b = n.a;
      n.a = 0;
      try {
        return Q(b, n.b, a, u);
      } finally {
        n.a = b;
      }
    };
    let c = new Promise(_);
    return i(c);
  } finally {
    n.a = n.b = 0;
  }
}
function fe(t) {
  let e = Promise.resolve(r(t));
  return i(e);
}
function ae(t, e) {
  let n = r(t).catch(r(e));
  return i(n);
}
function be(t, e) {
  let n = r(t).then(r(e));
  return i(n);
}
function ge(t, e, n) {
  let _ = r(t).then(r(e), r(n));
  return i(_);
}
function de(t) {
  let e = r(t).buffer;
  return i(e);
}
function le(t, e, n) {
  let _ = new Uint8Array(r(t), e >>> 0, n >>> 0);
  return i(_);
}
function we(t) {
  let e = new Uint8Array(r(t));
  return i(e);
}
function pe(t, e, n) {
  r(t).set(r(e), n >>> 0);
}
function ye(t) {
  return r(t).length;
}
function he(t) {
  let e = new Uint8Array(t >>> 0);
  return i(e);
}
function xe(t) {
  let e = r(t).buffer;
  return i(e);
}
function me(t) {
  return r(t).byteLength;
}
function ke(t) {
  return r(t).byteOffset;
}
function Re() {
  return s(function(t, e, n) {
    return Reflect.set(r(t), r(e), r(n));
  }, arguments);
}
function Fe(t, e) {
  let n = E(r(e)), _ = h(n, o.__wbindgen_malloc, o.__wbindgen_realloc), c = y;
  f()[t / 4 + 1] = c, f()[t / 4 + 0] = _;
}
function Me(t, e) {
  throw new Error(p(t, e));
}
function Oe() {
  let t = o.memory;
  return i(t);
}
function je(t, e, n) {
  let _ = I(t, e, 181, G);
  return i(_);
}
function ze(t, e, n) {
  let _ = I(t, e, 223, K);
  return i(_);
}
var z = class extends Se {
  async fetch(e) {
    return await v(e, this.env, this.ctx);
  }
  async queue(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
  async scheduled(e) {
    return await (void 0)(e, this.env, this.ctx);
  }
};
var Ee = ["IntoUnderlyingByteSource", "IntoUnderlyingSink", "IntoUnderlyingSource", "MinifyConfig", "PolishConfig", "R2Range", "RequestRedirect", "fetch", "queue", "scheduled", "getMemory"];
Object.keys(g).map((t) => {
  Ee.includes(t) | t.startsWith("__") || (z.prototype[t] = g[t]);
});
var We = z;

// ../../node_modules/.pnpm/wrangler@3.57.1_@cloudflare+workers-types@4.20240529.0/node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts
var drainBody = async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } finally {
    try {
      if (request.body !== null && !request.bodyUsed) {
        const reader = request.body.getReader();
        while (!(await reader.read()).done) {
        }
      }
    } catch (e) {
      console.error("Failed to drain the unused request body.", e);
    }
  }
};
var middleware_ensure_req_body_drained_default = drainBody;

// ../../node_modules/.pnpm/wrangler@3.57.1_@cloudflare+workers-types@4.20240529.0/node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts
function reduceError(e) {
  return {
    name: e?.name,
    message: e?.message ?? String(e),
    stack: e?.stack,
    cause: e?.cause === void 0 ? void 0 : reduceError(e.cause)
  };
}
var jsonError = async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } catch (e) {
    const error = reduceError(e);
    return Response.json(error, {
      status: 500,
      headers: { "MF-Experimental-Error-Stack": "true" }
    });
  }
};
var middleware_miniflare3_json_error_default = jsonError;

// .wrangler/tmp/bundle-ludKfy/middleware-insertion-facade.js
We.middleware = [
  middleware_ensure_req_body_drained_default,
  middleware_miniflare3_json_error_default,
  ...We.middleware ?? []
].filter(Boolean);
var middleware_insertion_facade_default = We;

// ../../node_modules/.pnpm/wrangler@3.57.1_@cloudflare+workers-types@4.20240529.0/node_modules/wrangler/templates/middleware/common.ts
var __facade_middleware__ = [];
function __facade_register__(...args) {
  __facade_middleware__.push(...args.flat());
}
function __facade_invokeChain__(request, env, ctx, dispatch, middlewareChain) {
  const [head, ...tail] = middlewareChain;
  const middlewareCtx = {
    dispatch,
    next(newRequest, newEnv) {
      return __facade_invokeChain__(newRequest, newEnv, ctx, dispatch, tail);
    }
  };
  return head(request, env, ctx, middlewareCtx);
}
function __facade_invoke__(request, env, ctx, dispatch, finalMiddleware) {
  return __facade_invokeChain__(request, env, ctx, dispatch, [
    ...__facade_middleware__,
    finalMiddleware
  ]);
}

// .wrangler/tmp/bundle-ludKfy/middleware-loader.entry.ts
var __Facade_ScheduledController__ = class {
  constructor(scheduledTime, cron, noRetry) {
    this.scheduledTime = scheduledTime;
    this.cron = cron;
    this.#noRetry = noRetry;
  }
  #noRetry;
  noRetry() {
    if (!(this instanceof __Facade_ScheduledController__)) {
      throw new TypeError("Illegal invocation");
    }
    this.#noRetry();
  }
};
function wrapExportedHandler(worker) {
  if (worker.middleware === void 0 || worker.middleware.length === 0) {
    return worker;
  }
  for (const middleware of worker.middleware) {
    __facade_register__(middleware);
  }
  const fetchDispatcher = function(request, env, ctx) {
    if (worker.fetch === void 0) {
      throw new Error("Handler does not export a fetch() function.");
    }
    return worker.fetch(request, env, ctx);
  };
  return {
    ...worker,
    fetch(request, env, ctx) {
      const dispatcher = function(type, init) {
        if (type === "scheduled" && worker.scheduled !== void 0) {
          const controller = new __Facade_ScheduledController__(
            Date.now(),
            init.cron ?? "",
            () => {
            }
          );
          return worker.scheduled(controller, env, ctx);
        }
      };
      return __facade_invoke__(request, env, ctx, dispatcher, fetchDispatcher);
    }
  };
}
function wrapWorkerEntrypoint(klass) {
  if (klass.middleware === void 0 || klass.middleware.length === 0) {
    return klass;
  }
  for (const middleware of klass.middleware) {
    __facade_register__(middleware);
  }
  return class extends klass {
    #fetchDispatcher = (request, env, ctx) => {
      this.env = env;
      this.ctx = ctx;
      if (super.fetch === void 0) {
        throw new Error("Entrypoint class does not define a fetch() function.");
      }
      return super.fetch(request);
    };
    #dispatcher = (type, init) => {
      if (type === "scheduled" && super.scheduled !== void 0) {
        const controller = new __Facade_ScheduledController__(
          Date.now(),
          init.cron ?? "",
          () => {
          }
        );
        return super.scheduled(controller);
      }
    };
    fetch(request) {
      return __facade_invoke__(
        request,
        this.env,
        this.ctx,
        this.#dispatcher,
        this.#fetchDispatcher
      );
    }
  };
}
var WRAPPED_ENTRY;
if (typeof middleware_insertion_facade_default === "object") {
  WRAPPED_ENTRY = wrapExportedHandler(middleware_insertion_facade_default);
} else if (typeof middleware_insertion_facade_default === "function") {
  WRAPPED_ENTRY = wrapWorkerEntrypoint(middleware_insertion_facade_default);
}
var middleware_loader_entry_default = WRAPPED_ENTRY;
export {
  T as IntoUnderlyingByteSource,
  A as IntoUnderlyingSink,
  x as IntoUnderlyingSource,
  q as MinifyConfig,
  Z as PolishConfig,
  L as R2Range,
  Y as RequestRedirect,
  dt as __wbg_String_b9412f8799faab3e,
  Pt as __wbg_append_7bfcb4937d1d5e29,
  Ct as __wbg_body_17b435cb52dcf45f,
  de as __wbg_buffer_12d079cc21e14bdb,
  xe as __wbg_buffer_dd7f74bc60f1faab,
  Ot as __wbg_byobRequest_72fca99f9c32c193,
  me as __wbg_byteLength_58f7b4fab1919d44,
  ke as __wbg_byteOffset_81d60f7392524f62,
  Yt as __wbg_call_27c0f87801dedf93,
  se as __wbg_call_b3ca7c6051f9bec1,
  Et as __wbg_cancel_6ee33d4006737aef,
  ae as __wbg_catch_0260e338d10f79ae,
  ie as __wbg_cause_3d9c85ebaf6b1155,
  kt as __wbg_cf_ab668814697435ac,
  jt as __wbg_close_184931724d961ccc,
  Ut as __wbg_close_a994f9425dab445c,
  Kt as __wbg_done_298b57d23c0fc80c,
  pt as __wbg_done_2ffa852272310e47,
  Wt as __wbg_enqueue_ea194723156c0cc2,
  mt as __wbg_entries_2f5d638b6f300812,
  It as __wbg_error_8e3928cfb8a43e2b,
  ut as __wbg_error_f851667af71bcfc6,
  wt as __wbg_getReader_ab94afcb5cb7689a,
  Jt as __wbg_get_bd8e338fbd5f5cc8,
  ne as __wbg_globalThis_d1e6af4856ba331b,
  re as __wbg_global_207b558942527489,
  qt as __wbg_headers_abb199c3be8d817c,
  xt as __wbg_httpProtocol_791d2bb087dc5b51,
  _e as __wbg_instanceof_Error_e20bb56fd5591a93,
  ye as __wbg_length_c20a40f15020d68a,
  Tt as __wbg_method_83327ed2e3f3229c,
  oe as __wbg_new_28c511d9baebfa89,
  we as __wbg_new_63b92bc8671ed464,
  Zt as __wbg_new_72fb9a18b5ae2624,
  ue as __wbg_new_81740750da40724f,
  Dt as __wbg_new_ab6fd82b10560829,
  ct as __wbg_new_abda76e883ba8a5f,
  Vt as __wbg_newnoargs_e258087cd0daa0ea,
  le as __wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb,
  ht as __wbg_newwithintounderlyingsource_a03a82aa1bbbb292,
  he as __wbg_newwithlength_e9b4878cebadb3d3,
  Rt as __wbg_newwithoptbuffersourceandinit_a4fa81e77259bb96,
  Mt as __wbg_newwithoptreadablestreamandinit_0b825f969ca543d6,
  Ft as __wbg_newwithoptstrandinit_219732174c595a25,
  Gt as __wbg_next_196c84450b364254,
  Bt as __wbg_queueMicrotask_3cbae2ec6b6cd3d6,
  Xt as __wbg_queueMicrotask_481971b0d87f3dd4,
  zt as __wbg_read_e7d0f8a49be01d86,
  Lt as __wbg_redirect_53aa50ab48e7a227,
  St as __wbg_releaseLock_5c49db976c08b864,
  fe as __wbg_resolve_b0083a7967828ec8,
  Nt as __wbg_respond_b1a43b2e3a06d525,
  te as __wbg_self_ce0dbfc45cf2f5be,
  Re as __wbg_set_1f9b04f170055d33,
  pe as __wbg_set_a47bac70306a19a7,
  lt as __wbg_sethighWaterMark_ea50ed3ec2143088,
  vt as __wbg_signal_e0b0ea9dce5137b3,
  st as __wbg_stack_658279fe44541cf6,
  be as __wbg_then_0c86a60e8fcfe9f6,
  ge as __wbg_then_a73caa9a87991566,
  ce as __wbg_toString_ffe4c9ea3b3532e9,
  At as __wbg_url_7807f6a1fddc3e23,
  yt as __wbg_value_9f6eeb1e2aab8d96,
  Qt as __wbg_value_d93c65011f51a456,
  $t as __wbg_view_7f0ce470793a340f,
  ee as __wbg_window_c6fb939a7f436783,
  it as __wbindgen_cb_drop,
  je as __wbindgen_closure_wrapper458,
  ze as __wbindgen_closure_wrapper974,
  Fe as __wbindgen_debug_string,
  Ht as __wbindgen_is_function,
  gt as __wbindgen_is_undefined,
  Oe as __wbindgen_memory,
  bt as __wbindgen_number_new,
  ft as __wbindgen_object_clone_ref,
  ot as __wbindgen_object_drop_ref,
  at as __wbindgen_string_get,
  _t as __wbindgen_string_new,
  Me as __wbindgen_throw,
  middleware_loader_entry_default as default,
  v as fetch,
  B as getMemory,
  Ce as wasmModule
};
//# sourceMappingURL=shim.js.map
