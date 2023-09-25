// @generated
# [doc = " Implement `DataProvider<LongCompactDecimalFormatDataV1Marker>` on the given struct using the data"] # [doc = r" hardcoded in this file. This allows the struct to be used with"] # [doc = r" `icu`'s `_unstable` constructors."] # [doc (hidden)] # [macro_export] macro_rules ! __impl_compactdecimal_long_v1 { ($ provider : ty) => { # [clippy :: msrv = "1.66"] const _ : () = < $ provider > :: MUST_USE_MAKE_PROVIDER_MACRO ; # [clippy :: msrv = "1.66"] impl icu_provider :: DataProvider < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker > for $ provider { fn load (& self , req : icu_provider :: DataRequest ,) -> Result < icu_provider :: DataResponse < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker > , icu_provider :: DataError > { static TH : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x04\x05\x06\t\n\x0B\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x05\x05\x05\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x08\0\0\0\0\0\x0C\0\x1E\0*\09\0Q\0o\0\x87\0\x03\0 \xE0\xB8\x9E\xE0\xB8\xB1\xE0\xB8\x99\x04\0 \xE0\xB8\xAB\xE0\xB8\xA1\xE0\xB8\xB7\xE0\xB9\x88\xE0\xB8\x99\x05\0 \xE0\xB9\x81\xE0\xB8\xAA\xE0\xB8\x99\x06\0 \xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\t\0 \xE0\xB8\x9E\xE0\xB8\xB1\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\n\0 \xE0\xB8\xAB\xE0\xB8\xA1\xE0\xB8\xB7\xE0\xB9\x88\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\x0B\0 \xE0\xB9\x81\xE0\xB8\xAA\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\x0C\0 \xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99\xE0\xB8\xA5\xE0\xB9\x89\xE0\xB8\xB2\xE0\xB8\x99") }) } , } ; static FR : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x04\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0\n\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\x05\x06\x05\x01\x05\x01\x05\x01\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\n\0\0\0\0\0\n\0\x12\0\x19\0!\0+\x006\0A\0M\0W\0\x03\0 millier\x03\0 mille\x03\xFFmille\x03\0 mille\x06\0 million\x06\0 millions\t\0 milliard\t\0 milliards\x0C\0 billion\x0C\0 billions") }) } , } ; static AR : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x04\x06\x08\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x02\0\0\0\x03\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0\x08\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x05\x05\x03\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x08\0\0\0\0\0\x0B\0\x14\0\x1D\0,\09\0F\0S\0\x03\0 \xD8\xA2\xD9\x84\xD8\xA7\xD9\x81\x03\0 \xD8\xA3\xD9\x84\xD9\x81\x03\0 \xD8\xA3\xD9\x84\xD9\x81\x06\0 \xD9\x85\xD9\x84\xD8\xA7\xD9\x8A\xD9\x8A\xD9\x86\x06\0 \xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\x06\0 \xD9\x85\xD9\x84\xD9\x8A\xD9\x88\xD9\x86\t\0 \xD9\x85\xD9\x84\xD9\x8A\xD8\xA7\xD8\xB1\x0C\0 \xD8\xAA\xD8\xB1\xD9\x84\xD9\x8A\xD9\x88\xD9\x86") }) } , } ; static BN : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x05\x07\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x04\0\0\0\0\0\x12\0\x1E\0-\0\x03\0 \xE0\xA6\xB9\xE0\xA6\xBE\xE0\xA6\x9C\xE0\xA6\xBE\xE0\xA6\xB0\x05\0 \xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96\x07\0 \xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x9F\xE0\xA6\xBF\x0C\0 \xE0\xA6\xB2\xE0\xA6\xBE\xE0\xA6\x96 \xE0\xA6\x95\xE0\xA7\x8B\xE0\xA6\x9F\xE0\xA6\xBF") }) } , } ; static CCP : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x04\0\0\0\0\0\x03\0\x06\0\t\0\x03\0K\x06\0M\t\0G\x0C\0T") }) } , } ; static TR : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x04\0\0\0\0\0\x06\0\x0F\0\x18\0\x03\0 bin\x06\0 milyon\t\0 milyar\x0C\0 trilyon") }) } , } ; static EN : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x04\0\0\0\0\0\x0B\0\x15\0\x1F\0\x03\0 thousand\x06\0 million\t\0 billion\x0C\0 trillion") }) } , } ; static FR_CA : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x03\0\0\0\x05\0\0\0\x07\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x01\x05\x01\x05\x01\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x07\0\0\0\0\0\x08\0\x12\0\x1D\0(\x004\0>\0\x03\0 mille\x06\0 million\x06\0 millions\t\0 milliard\t\0 milliards\x0C\0 billion\x0C\0 billions") }) } , } ; static FIL : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x02\0\0\0\x04\0\0\0\x06\0\0\0\x08\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\x05\x01\x05\x01\x05\x01\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x08\0\0\0\0\0\x07\0\x11\0\x1A\0&\0/\0;\0E\0\x03\0 libo\x03\0 na libo\x06\0 milyon\x06\0 na milyon\t\0 bilyon\t\0 na bilyon\x0C\0 trilyon\x0C\0 na trilyon") }) } , } ; static SR_LATN : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x02\0\0\0\x04\0\0\0\x07\0\0\0\t\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x05\x01\x05\x01\x03\x05\x01\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\t\0\0\0\0\0\n\0\x14\0\x1D\0'\x003\0?\0K\0T\0\x03\0 hiljade\x03\0 hiljada\x06\0 milion\x06\0 miliona\t\0 milijarda\t\0 milijarde\t\0 milijardi\x0C\0 bilion\x0C\0 biliona") }) } , } ; static SR : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x02\0\0\0\x04\0\0\0\x07\0\0\0\t\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x05\x01\x05\x01\x03\x05\x01\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\t\0\0\0\0\0\x0F\0\x1E\0-\0>\0S\0h\0}\0\x8C\0\x03\0 \xD1\x85\xD0\xB8\xD1\x99\xD0\xB0\xD0\xB4\xD0\xB5\x03\0 \xD1\x85\xD0\xB8\xD1\x99\xD0\xB0\xD0\xB4\xD0\xB0\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB5\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xB8\xD1\x98\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB8\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD0\xB1\xD0\xB8\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0") }) } , } ; static RU : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\t\x0C") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\0\0\0\x06\0\0\0\t\0\0\0\x0C\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\x04\x05\x01\x04\x05\x01\x04\x05\x01\x04\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x0C\0\0\0\0\0\x0F\0\x1C\0+\0<\0Q\0d\0w\0\x8E\0\xA3\0\xB6\0\xCD\0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD0\xB0\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\x03\0 \xD1\x82\xD1\x8B\xD1\x81\xD1\x8F\xD1\x87\xD0\xB8\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB2\x06\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xBE\xD0\xB2\t\0 \xD0\xBC\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xB0\xD1\x80\xD0\xB4\xD0\xB0\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xBE\xD0\xB2\x0C\0 \xD1\x82\xD1\x80\xD0\xB8\xD0\xBB\xD0\xBB\xD0\xB8\xD0\xBE\xD0\xBD\xD0\xB0") }) } , } ; static ES_419 : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\x07\t\x0C\r") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x06\0\0\0\x07\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x01\x05\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x07\0\0\0\0\0\x06\0\x10\0\x1B\0&\x005\0?\0\x03\0 mil\x06\0 mill\xC3\xB3n\x06\0 millones\x06\0 millones\t\0 mil millones\x0C\0 bill\xC3\xB3n\x0C\0 billones") }) } , } ; static ES : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x03\x06\x07\t\x0C\r") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x03\0\0\0\x04\0\0\0\x05\0\0\0\x07\0\0\0\x08\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x01\x05\x05\x05\x01\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x08\0\0\0\0\0\x06\0\x10\0\x1B\0&\x005\0?\0J\0\x03\0 mil\x06\0 mill\xC3\xB3n\x06\0 millones\x06\0 millones\t\0 mil millones\x0C\0 bill\xC3\xB3n\x0C\0 billones\x0C\0 billones") }) } , } ; static JA : < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_compactdecimal :: provider :: CompactDecimalPatternDataV1 { patterns : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x04\x08\x0C\x10") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\x02\0\0\0\x03\0\0\0\x04\0\0\0") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\x05\x05\x05\x05") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x04\0\0\0\0\0\x05\0\n\0\x0F\0\x04\0\xE4\xB8\x87\x08\0\xE5\x84\x84\x0C\0\xE5\x85\x86\x10\0\xE4\xBA\xAC") }) } , } ; static VALUES : [& < icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable ; 265usize] = [& AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & BN , & BN , & BN , & BN , & CCP , & CCP , & CCP , & CCP , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & EN , & ES , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES , & ES_419 , & ES , & ES_419 , & ES_419 , & ES , & ES , & ES_419 , & ES_419 , & ES_419 , & ES , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & ES_419 , & FIL , & FR , & FR , & FR , & FR , & FR , & FR , & FR_CA , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & EN , & JA , & RU , & RU , & RU , & RU , & RU , & RU , & SR , & SR , & SR_LATN , & SR_LATN , & SR_LATN , & SR_LATN , & SR , & TH , & TH , & TR , & TR , & CCP] ; static KEYS : [& str ; 265usize] = ["ar" , "ar-AE" , "ar-AE-u-nu-arab" , "ar-BH" , "ar-BH-u-nu-latn" , "ar-DJ" , "ar-DJ-u-nu-latn" , "ar-DZ" , "ar-DZ-u-nu-arab" , "ar-EG" , "ar-EG-u-nu-latn" , "ar-EH" , "ar-EH-u-nu-arab" , "ar-ER" , "ar-ER-u-nu-latn" , "ar-IL" , "ar-IL-u-nu-latn" , "ar-IQ" , "ar-IQ-u-nu-latn" , "ar-JO" , "ar-JO-u-nu-latn" , "ar-KM" , "ar-KM-u-nu-latn" , "ar-KW" , "ar-KW-u-nu-latn" , "ar-LB" , "ar-LB-u-nu-latn" , "ar-LY" , "ar-LY-u-nu-arab" , "ar-MA" , "ar-MA-u-nu-arab" , "ar-MR" , "ar-MR-u-nu-latn" , "ar-OM" , "ar-OM-u-nu-latn" , "ar-PS" , "ar-PS-u-nu-latn" , "ar-QA" , "ar-QA-u-nu-latn" , "ar-SA" , "ar-SA-u-nu-latn" , "ar-SD" , "ar-SD-u-nu-latn" , "ar-SO" , "ar-SO-u-nu-latn" , "ar-SS" , "ar-SS-u-nu-latn" , "ar-SY" , "ar-SY-u-nu-latn" , "ar-TD" , "ar-TD-u-nu-latn" , "ar-TN" , "ar-TN-u-nu-arab" , "ar-YE" , "ar-YE-u-nu-latn" , "ar-u-nu-latn" , "bn" , "bn-IN" , "bn-IN-u-nu-latn" , "bn-u-nu-latn" , "ccp" , "ccp-IN" , "ccp-IN-u-nu-latn" , "ccp-u-nu-latn" , "en" , "en-001" , "en-150" , "en-AE" , "en-AG" , "en-AI" , "en-AS" , "en-AT" , "en-AU" , "en-BB" , "en-BE" , "en-BI" , "en-BM" , "en-BS" , "en-BW" , "en-BZ" , "en-CA" , "en-CC" , "en-CH" , "en-CK" , "en-CM" , "en-CX" , "en-CY" , "en-DE" , "en-DG" , "en-DK" , "en-DM" , "en-ER" , "en-FI" , "en-FJ" , "en-FK" , "en-FM" , "en-GB" , "en-GD" , "en-GG" , "en-GH" , "en-GI" , "en-GM" , "en-GU" , "en-GY" , "en-HK" , "en-IE" , "en-IL" , "en-IM" , "en-IN" , "en-IO" , "en-JE" , "en-JM" , "en-KE" , "en-KI" , "en-KN" , "en-KY" , "en-LC" , "en-LR" , "en-LS" , "en-MG" , "en-MH" , "en-MO" , "en-MP" , "en-MS" , "en-MT" , "en-MU" , "en-MV" , "en-MW" , "en-MY" , "en-NA" , "en-NF" , "en-NG" , "en-NL" , "en-NR" , "en-NU" , "en-NZ" , "en-PG" , "en-PH" , "en-PK" , "en-PN" , "en-PR" , "en-PW" , "en-RW" , "en-SB" , "en-SC" , "en-SD" , "en-SE" , "en-SG" , "en-SH" , "en-SI" , "en-SL" , "en-SS" , "en-SX" , "en-SZ" , "en-TC" , "en-TK" , "en-TO" , "en-TT" , "en-TV" , "en-TZ" , "en-UG" , "en-UM" , "en-VC" , "en-VG" , "en-VI" , "en-VU" , "en-WS" , "en-ZA" , "en-ZM" , "en-ZW" , "es" , "es-419" , "es-AR" , "es-BO" , "es-BR" , "es-BZ" , "es-CL" , "es-CO" , "es-CR" , "es-CU" , "es-DO" , "es-EA" , "es-EC" , "es-GQ" , "es-GT" , "es-HN" , "es-IC" , "es-MX" , "es-NI" , "es-PA" , "es-PE" , "es-PH" , "es-PR" , "es-PY" , "es-SV" , "es-US" , "es-UY" , "es-VE" , "fil" , "fr" , "fr-BE" , "fr-BF" , "fr-BI" , "fr-BJ" , "fr-BL" , "fr-CA" , "fr-CD" , "fr-CF" , "fr-CG" , "fr-CH" , "fr-CI" , "fr-CM" , "fr-DJ" , "fr-DZ" , "fr-GA" , "fr-GF" , "fr-GN" , "fr-GP" , "fr-GQ" , "fr-HT" , "fr-KM" , "fr-LU" , "fr-MA" , "fr-MC" , "fr-MF" , "fr-MG" , "fr-ML" , "fr-MQ" , "fr-MR" , "fr-MU" , "fr-NC" , "fr-NE" , "fr-PF" , "fr-PM" , "fr-RE" , "fr-RW" , "fr-SC" , "fr-SN" , "fr-SY" , "fr-TD" , "fr-TG" , "fr-TN" , "fr-VU" , "fr-WF" , "fr-YT" , "hi-Latn" , "ja" , "ru" , "ru-BY" , "ru-KG" , "ru-KZ" , "ru-MD" , "ru-UA" , "sr" , "sr-BA" , "sr-Latn" , "sr-Latn-BA" , "sr-Latn-XK" , "sr-ME" , "sr-XK" , "th" , "th-u-nu-thai" , "tr" , "tr-CY" , "und"] ; if let Ok (payload) = KEYS . binary_search_by (| k | req . locale . strict_cmp (k . as_bytes ()) . reverse ()) . map (| i | * unsafe { VALUES . get_unchecked (i) }) { Ok (icu_provider :: DataResponse { payload : Some (icu_provider :: DataPayload :: from_static_ref (payload)) , metadata : Default :: default () , }) } else { Err (icu_provider :: DataErrorKind :: MissingLocale . with_req (< icu_compactdecimal :: provider :: LongCompactDecimalFormatDataV1Marker as icu_provider :: KeyedDataMarker > :: KEY , req)) } } } } }