// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** See the [Rust documentation for `Strength`](https://docs.rs/icu/latest/icu/collator/enum.Strength.html) for more information.
*/
export class CollatorStrength {
    constructor(value : CollatorStrength | string);

    get value() : string;

    get ffiValue() : number;

    static Auto : CollatorStrength;

    static Primary : CollatorStrength;

    static Secondary : CollatorStrength;

    static Tertiary : CollatorStrength;

    static Quaternary : CollatorStrength;

    static Identical : CollatorStrength;


    

}