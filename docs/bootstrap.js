/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".bootstrap.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"../pkg/wasm_game_of_life_bg.wasm": function() {
/******/ 			return {
/******/ 				"./wasm_game_of_life_bg.js": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_17fdb5cd280d476d": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_instanceof_Window_17fdb5cd280d476d"](p0i32);
/******/ 					},
/******/ 					"__wbg_document_c26d0f423c143e0c": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_document_c26d0f423c143e0c"](p0i32);
/******/ 					},
/******/ 					"__wbg_getElementById_df597d226f179219": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_getElementById_df597d226f179219"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_WebGlRenderingContext_f732dd95c50b903a": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_instanceof_WebGlRenderingContext_f732dd95c50b903a"](p0i32);
/******/ 					},
/******/ 					"__wbg_bufferData_94b2c19e17eafe74": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_bufferData_94b2c19e17eafe74"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_texImage2D_f4f837ad2dcc6a75": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_texImage2D_f4f837ad2dcc6a75"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32);
/******/ 					},
/******/ 					"__wbg_uniform2fv_a32f8a0005d9de69": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_uniform2fv_a32f8a0005d9de69"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_activeTexture_ee6eed2472803dd2": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_activeTexture_ee6eed2472803dd2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_attachShader_c2f7771e6f4b91d8": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_attachShader_c2f7771e6f4b91d8"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindBuffer_f4ad79795655c1c4": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_bindBuffer_f4ad79795655c1c4"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindFramebuffer_487c12d3f606b2db": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_bindFramebuffer_487c12d3f606b2db"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bindTexture_751d66bbae4822ab": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_bindTexture_751d66bbae4822ab"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_clear_42b42c27d041ce11": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_clear_42b42c27d041ce11"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_clearColor_ba6ba6886092ab6a": function(p0i32,p1f32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_clearColor_ba6ba6886092ab6a"](p0i32,p1f32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_compileShader_8aec8947f553f5b6": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_compileShader_8aec8947f553f5b6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createBuffer_f26187e1b465a677": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_createBuffer_f26187e1b465a677"](p0i32);
/******/ 					},
/******/ 					"__wbg_createFramebuffer_f9dccb7089aaebb6": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_createFramebuffer_f9dccb7089aaebb6"](p0i32);
/******/ 					},
/******/ 					"__wbg_createProgram_10f7bf07e21fe904": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_createProgram_10f7bf07e21fe904"](p0i32);
/******/ 					},
/******/ 					"__wbg_createShader_4060106dc88c8bca": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_createShader_4060106dc88c8bca"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createTexture_d7a4df257a9410a7": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_createTexture_d7a4df257a9410a7"](p0i32);
/******/ 					},
/******/ 					"__wbg_deleteBuffer_432a5312db151097": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_deleteBuffer_432a5312db151097"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteProgram_4df15a60f6fa0bf4": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_deleteProgram_4df15a60f6fa0bf4"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteTexture_8bec893146319ac5": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_deleteTexture_8bec893146319ac5"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_drawElements_90d9e4b571f4de3d": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_drawElements_90d9e4b571f4de3d"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_enableVertexAttribArray_2e2bfba7f3a5fb74": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_enableVertexAttribArray_2e2bfba7f3a5fb74"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_framebufferTexture2D_245d63ed1d5212e4": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_framebufferTexture2D_245d63ed1d5212e4"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_getAttribLocation_ea61e93124c45a64": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_getAttribLocation_ea61e93124c45a64"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_getShaderParameter_4306f019f7eb9f82": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_getShaderParameter_4306f019f7eb9f82"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getUniformLocation_277279212040ec65": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_getUniformLocation_277279212040ec65"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_linkProgram_9258ef1fcd3afc43": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_linkProgram_9258ef1fcd3afc43"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_shaderSource_ef8be775578bf902": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_shaderSource_ef8be775578bf902"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_texParameteri_e2db4aa7650962eb": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_texParameteri_e2db4aa7650962eb"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform1f_fb0383a4c61faacf": function(p0i32,p1i32,p2f32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_uniform1f_fb0383a4c61faacf"](p0i32,p1i32,p2f32);
/******/ 					},
/******/ 					"__wbg_uniform1i_595e085d9c3aadf2": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_uniform1i_595e085d9c3aadf2"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_useProgram_67487c5ef197884d": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_useProgram_67487c5ef197884d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribPointer_52d124f67ddb572d": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_vertexAttribPointer_52d124f67ddb572d"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_viewport_5f99aff932f780aa": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_viewport_5f99aff932f780aa"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_log_eb1108411ecc4a7f": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_log_eb1108411ecc4a7f"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_ff7be16a6a6bdf51": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_ff7be16a6a6bdf51"](p0i32);
/******/ 					},
/******/ 					"__wbg_width_aeeb90e24b778e64": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_width_aeeb90e24b778e64"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_66b10992a66b71e3": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_height_66b10992a66b71e3"](p0i32);
/******/ 					},
/******/ 					"__wbg_getContext_0dcf09cb63d08f77": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_getContext_0dcf09cb63d08f77"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_8aad4a6554f38345": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_newnoargs_8aad4a6554f38345"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_1f85aaa5836dfb23": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_call_1f85aaa5836dfb23"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_c0d3a5923e013647": function() {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_self_c0d3a5923e013647"]();
/******/ 					},
/******/ 					"__wbg_window_7ee6c8be3432927d": function() {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_window_7ee6c8be3432927d"]();
/******/ 					},
/******/ 					"__wbg_globalThis_c6de1d938e089cf0": function() {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_globalThis_c6de1d938e089cf0"]();
/******/ 					},
/******/ 					"__wbg_global_c9a01ce4680907f8": function() {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbg_global_c9a01ce4680907f8"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_boolean_get": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_boolean_get"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["../pkg/wasm_game_of_life_bg.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"0":["../pkg/wasm_game_of_life_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"../pkg/wasm_game_of_life_bg.wasm":"a2b9635771eab728d7d5"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });