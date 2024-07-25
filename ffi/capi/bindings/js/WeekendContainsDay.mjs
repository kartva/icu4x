// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** Documents which days of the week are considered to be a part of the weekend
*
*See the [Rust documentation for `weekend`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#method.weekend) for more information.
*/
export class WeekendContainsDay {
    #monday;
    get monday()  {
        return this.#monday;
    }
    set monday(value) {
        this.#monday = value;
    }
    #tuesday;
    get tuesday()  {
        return this.#tuesday;
    }
    set tuesday(value) {
        this.#tuesday = value;
    }
    #wednesday;
    get wednesday()  {
        return this.#wednesday;
    }
    set wednesday(value) {
        this.#wednesday = value;
    }
    #thursday;
    get thursday()  {
        return this.#thursday;
    }
    set thursday(value) {
        this.#thursday = value;
    }
    #friday;
    get friday()  {
        return this.#friday;
    }
    set friday(value) {
        this.#friday = value;
    }
    #saturday;
    get saturday()  {
        return this.#saturday;
    }
    set saturday(value) {
        this.#saturday = value;
    }
    #sunday;
    get sunday()  {
        return this.#sunday;
    }
    set sunday(value) {
        this.#sunday = value;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        slice_cleanup_callbacks,
        appendArrayMap
    ) {
        return [this.#monday, this.#tuesday, this.#wednesday, this.#thursday, this.#friday, this.#saturday, this.#sunday]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    _fromFFI(ptr) {
        const mondayDeref = (new Uint8Array(wasm.memory.buffer, ptr, 1))[0] == 1;
        this.#monday = mondayDeref;
        const tuesdayDeref = (new Uint8Array(wasm.memory.buffer, ptr + 1, 1))[0] == 1;
        this.#tuesday = tuesdayDeref;
        const wednesdayDeref = (new Uint8Array(wasm.memory.buffer, ptr + 2, 1))[0] == 1;
        this.#wednesday = wednesdayDeref;
        const thursdayDeref = (new Uint8Array(wasm.memory.buffer, ptr + 3, 1))[0] == 1;
        this.#thursday = thursdayDeref;
        const fridayDeref = (new Uint8Array(wasm.memory.buffer, ptr + 4, 1))[0] == 1;
        this.#friday = fridayDeref;
        const saturdayDeref = (new Uint8Array(wasm.memory.buffer, ptr + 5, 1))[0] == 1;
        this.#saturday = saturdayDeref;
        const sundayDeref = (new Uint8Array(wasm.memory.buffer, ptr + 6, 1))[0] == 1;
        this.#sunday = sundayDeref;

        return this;
    }
    

}