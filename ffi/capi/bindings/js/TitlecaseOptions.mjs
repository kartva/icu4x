// generated by diplomat-tool
import { LeadingAdjustment } from "./LeadingAdjustment.mjs"
import { TrailingCase } from "./TrailingCase.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `TitlecaseOptions`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html) for more information.
*/
export class TitlecaseOptions {
    #leadingAdjustment;
    get leadingAdjustment()  {
        return this.#leadingAdjustment;
    }
    set leadingAdjustment(value) {
        this.#leadingAdjustment = value;
    }
    #trailingCase;
    get trailingCase()  {
        return this.#trailingCase;
    }
    set trailingCase(value) {
        this.#trailingCase = value;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        slice_cleanup_callbacks,
        appendArrayMap
    ) {
        return [this.#leadingAdjustment.ffiValue, this.#trailingCase.ffiValue]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    _fromFFI(ptr) {
        const leadingAdjustmentDeref = diplomatRuntime.enumDiscriminant(wasm, ptr);
        this.#leadingAdjustment = LeadingAdjustment[Array.from(LeadingAdjustment.values.keys())[leadingAdjustmentDeref]];
        const trailingCaseDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 4);
        this.#trailingCase = TrailingCase[Array.from(TrailingCase.values.keys())[trailingCaseDeref]];

        return this;
    }
    static defaultOptions() {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
        const result = wasm.icu4x_TitlecaseOptionsV1_default_mv1(diplomat_receive_buffer);
    
        try {
    
            return new TitlecaseOptions()._fromFFI(diplomat_receive_buffer);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
        
        }
    }

    

}