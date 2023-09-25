// @generated
# [doc = " Implement `DataProvider<NarrowMinuteRelativeTimeFormatDataV1Marker>` on the given struct using the data"] # [doc = r" hardcoded in this file. This allows the struct to be used with"] # [doc = r" `icu`'s `_unstable` constructors."] # [doc (hidden)] # [macro_export] macro_rules ! __impl_relativetime_narrow_minute_v1 { ($ provider : ty) => { # [clippy :: msrv = "1.66"] const _ : () = < $ provider > :: MUST_USE_MAKE_PROVIDER_MACRO ; # [clippy :: msrv = "1.66"] impl icu_provider :: DataProvider < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker > for $ provider { fn load (& self , req : icu_provider :: DataRequest ,) -> Result < icu_provider :: DataResponse < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker > , icu_provider :: DataError > { static RU : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0\xD0\xB2 \xD1\x8D\xD1\x82\xD1\x83 \xD0\xBC\xD0\xB8\xD0\xBD\xD1\x83\xD1\x82\xD1\x83") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("- мин") , index : 1u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("- мин") , index : 1u8 , }) , many : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("- мин") , index : 1u8 , }) , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("- мин") , index : 1u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("+ мин") , index : 1u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("+ мин") , index : 1u8 , }) , many : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("+ мин") , index : 1u8 , }) , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("+ мин") , index : 1u8 , } , } , } ; static SR : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD0\xBC\xD0\xB8\xD0\xBD\xD1\x83\xD1\x82\xD0\xB0") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("пре  мин.") , index : 7u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("пре  мин.") , index : 7u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("пре  мин.") , index : 7u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("за  мин.") , index : 5u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("за  мин.") , index : 5u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("за  мин.") , index : 5u8 , } , } , } ; static SR_BA : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0\xD0\xBE\xD0\xB2\xD0\xBE\xD0\xB3 \xD0\xBC\xD0\xB8\xD0\xBD\xD1\x83\xD1\x82\xD0\xB0") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("прије  мин.") , index : 11u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("прије  мин.") , index : 11u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("прије  мин.") , index : 11u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("за  мин.") , index : 5u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("за  мин.") , index : 5u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("за  мин.") , index : 5u8 , } , } , } ; static AR : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0\xD9\x87\xD8\xB0\xD9\x87 \xD8\xA7\xD9\x84\xD8\xAF\xD9\x82\xD9\x8A\xD9\x82\xD8\xA9") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("قبل  دقيقة") , index : 7u8 , }) , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("قبل دقيقة واحدة") , index : 255u8 , }) , two : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("قبل دقيقتين") , index : 255u8 , }) , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("قبل  دقائق") , index : 7u8 , }) , many : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("قبل  دقيقة") , index : 7u8 , }) , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("قبل  دقيقة") , index : 7u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("خلال  دقيقة") , index : 9u8 , }) , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("خلال دقيقة واحدة") , index : 255u8 , }) , two : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("خلال دقيقتين") , index : 255u8 , }) , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("خلال  دقائق") , index : 9u8 , }) , many : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("خلال  دقيقة") , index : 9u8 , }) , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("خلال  دقيقة") , index : 9u8 , } , } , } ; static BN : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0\xE0\xA6\x8F\xE0\xA6\x87 \xE0\xA6\xAE\xE0\xA6\xBF\xE0\xA6\xA8\xE0\xA6\xBF\xE0\xA6\x9F") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" মিনিট আগে") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" মিনিট আগে") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" মিনিটে") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" মিনিটে") , index : 0u8 , } , } , } ; static TH : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0\xE0\xB8\x99\xE0\xB8\xB2\xE0\xB8\x97\xE0\xB8\xB5\xE0\xB8\x99\xE0\xB8\xB5\xE0\xB9\x89") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : None , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" นาท\u{e35}ท\u{e35}\u{e48}แล\u{e49}ว") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : None , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("ใน  นาท\u{e35}") , index : 7u8 , } , } , } ; static CCP : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0\xF0\x91\x84\x83\xF0\x91\x84\xB3\xF0\x91\x84\x86\xF0\x91\x84\xAC \xF0\x91\x84\x9F\xF0\x91\x84\xA8\xF0\x91\x84\x9A\xF0\x91\x84\xA8\xF0\x91\x84\x96\xF0\x91\x84\xB4") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" 𑄟\u{11128}𑄚\u{11128}𑄖\u{11134} 𑄃𑄉𑄬") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" 𑄟\u{11128}𑄚\u{11128}𑄖\u{11134} 𑄃𑄉𑄬") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" 𑄟\u{11128}𑄚\u{11128}𑄘𑄬") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" 𑄟\u{11128}𑄚\u{11128}𑄘𑄬") , index : 0u8 , } , } , } ; static TR : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0bu dakika") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" dk. önce") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" dk. önce") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" dk. sonra") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" dk. sonra") , index : 0u8 , } , } , } ; static FR : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0cette minute-ci") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("- min") , index : 1u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("- min") , index : 1u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("+ min") , index : 1u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("+ min") , index : 1u8 , } , } , } ; static ES : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0este minuto") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("hace  min") , index : 5u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("hace  min") , index : 5u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("dentro de  min") , index : 10u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("dentro de  min") , index : 10u8 , } , } , } ; static SR_LATN : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0ovog minuta") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("pre  min.") , index : 4u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("pre  min.") , index : 4u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("pre  min.") , index : 4u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("za  min.") , index : 3u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("za  min.") , index : 3u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("za  min.") , index : 3u8 , } , } , } ; static SR_LATN_BA : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0ovog minuta") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("prije  min.") , index : 6u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("prije  min.") , index : 6u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("prije  min.") , index : 6u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("za  min.") , index : 3u8 , }) , two : None , few : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("za  min.") , index : 3u8 , }) , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("za  min.") , index : 3u8 , } , } , } ; static FIL : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0sa minutong ito") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min. ang nakalipas") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min. ang nakalipas") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("sa  min.") , index : 3u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("sa  min.") , index : 3u8 , } , } , } ; static UND : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0this minute") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : None , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("- min") , index : 1u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : None , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("+ min") , index : 1u8 , } , } , } ; static EN_001 : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0this minute") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min ago") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min ago") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in  min") , index : 3u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in  min") , index : 3u8 , } , } , } ; static EN_CA : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0this minute") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min ago") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" mins ago") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in  min") , index : 3u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in  mins") , index : 3u8 , } , } , } ; static EN_AU : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0this minute") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min. ago") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" mins ago") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in  min.") , index : 3u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in  mins") , index : 3u8 , } , } , } ; static EN : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0this minute") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("m ago") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("m ago") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in m") , index : 3u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("in m") , index : 3u8 , } , } , } ; static HI_LATN : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\0yah minute") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min. pahle") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min. pahle") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : Some (icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min. mein") , index : 0u8 , }) , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed (" min. mein") , index : 0u8 , } , } , } ; static JA : < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_relativetime :: provider :: RelativeTimePatternDataV1 { relatives : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"\0") } , unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x01\0\0\0\0\x001 \xE5\x88\x86\xE4\xBB\xA5\xE5\x86\x85") }) } , past : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : None , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("分前") , index : 0u8 , } , } , future : icu_relativetime :: provider :: PluralRulesCategoryMapping { zero : None , one : None , two : None , few : None , many : None , other : icu_relativetime :: provider :: SingularSubPattern { pattern : alloc :: borrow :: Cow :: Borrowed ("分後") , index : 0u8 , } , } , } ; static VALUES : [& < icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: DataMarker > :: Yokeable ; 232usize] = [& AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & AR , & BN , & BN , & CCP , & CCP , & EN , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_AU , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_CA , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN , & EN_001 , & EN , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_001 , & EN , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & EN_001 , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & ES , & FIL , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & FR , & HI_LATN , & JA , & RU , & RU , & RU , & RU , & RU , & RU , & SR , & SR_BA , & SR_LATN , & SR_LATN_BA , & SR_LATN , & SR_LATN , & SR , & TH , & TR , & TR , & UND] ; static KEYS : [& str ; 232usize] = ["ar" , "ar-AE" , "ar-BH" , "ar-DJ" , "ar-DZ" , "ar-EG" , "ar-EH" , "ar-ER" , "ar-IL" , "ar-IQ" , "ar-JO" , "ar-KM" , "ar-KW" , "ar-LB" , "ar-LY" , "ar-MA" , "ar-MR" , "ar-OM" , "ar-PS" , "ar-QA" , "ar-SA" , "ar-SD" , "ar-SO" , "ar-SS" , "ar-SY" , "ar-TD" , "ar-TN" , "ar-YE" , "bn" , "bn-IN" , "ccp" , "ccp-IN" , "en" , "en-001" , "en-150" , "en-AE" , "en-AG" , "en-AI" , "en-AS" , "en-AT" , "en-AU" , "en-BB" , "en-BE" , "en-BI" , "en-BM" , "en-BS" , "en-BW" , "en-BZ" , "en-CA" , "en-CC" , "en-CH" , "en-CK" , "en-CM" , "en-CX" , "en-CY" , "en-DE" , "en-DG" , "en-DK" , "en-DM" , "en-ER" , "en-FI" , "en-FJ" , "en-FK" , "en-FM" , "en-GB" , "en-GD" , "en-GG" , "en-GH" , "en-GI" , "en-GM" , "en-GU" , "en-GY" , "en-HK" , "en-IE" , "en-IL" , "en-IM" , "en-IN" , "en-IO" , "en-JE" , "en-JM" , "en-KE" , "en-KI" , "en-KN" , "en-KY" , "en-LC" , "en-LR" , "en-LS" , "en-MG" , "en-MH" , "en-MO" , "en-MP" , "en-MS" , "en-MT" , "en-MU" , "en-MV" , "en-MW" , "en-MY" , "en-NA" , "en-NF" , "en-NG" , "en-NL" , "en-NR" , "en-NU" , "en-NZ" , "en-PG" , "en-PH" , "en-PK" , "en-PN" , "en-PR" , "en-PW" , "en-RW" , "en-SB" , "en-SC" , "en-SD" , "en-SE" , "en-SG" , "en-SH" , "en-SI" , "en-SL" , "en-SS" , "en-SX" , "en-SZ" , "en-TC" , "en-TK" , "en-TO" , "en-TT" , "en-TV" , "en-TZ" , "en-UG" , "en-UM" , "en-VC" , "en-VG" , "en-VI" , "en-VU" , "en-WS" , "en-ZA" , "en-ZM" , "en-ZW" , "es" , "es-419" , "es-AR" , "es-BO" , "es-BR" , "es-BZ" , "es-CL" , "es-CO" , "es-CR" , "es-CU" , "es-DO" , "es-EA" , "es-EC" , "es-GQ" , "es-GT" , "es-HN" , "es-IC" , "es-MX" , "es-NI" , "es-PA" , "es-PE" , "es-PH" , "es-PR" , "es-PY" , "es-SV" , "es-US" , "es-UY" , "es-VE" , "fil" , "fr" , "fr-BE" , "fr-BF" , "fr-BI" , "fr-BJ" , "fr-BL" , "fr-CA" , "fr-CD" , "fr-CF" , "fr-CG" , "fr-CH" , "fr-CI" , "fr-CM" , "fr-DJ" , "fr-DZ" , "fr-GA" , "fr-GF" , "fr-GN" , "fr-GP" , "fr-GQ" , "fr-HT" , "fr-KM" , "fr-LU" , "fr-MA" , "fr-MC" , "fr-MF" , "fr-MG" , "fr-ML" , "fr-MQ" , "fr-MR" , "fr-MU" , "fr-NC" , "fr-NE" , "fr-PF" , "fr-PM" , "fr-RE" , "fr-RW" , "fr-SC" , "fr-SN" , "fr-SY" , "fr-TD" , "fr-TG" , "fr-TN" , "fr-VU" , "fr-WF" , "fr-YT" , "hi-Latn" , "ja" , "ru" , "ru-BY" , "ru-KG" , "ru-KZ" , "ru-MD" , "ru-UA" , "sr" , "sr-BA" , "sr-Latn" , "sr-Latn-BA" , "sr-Latn-XK" , "sr-ME" , "sr-XK" , "th" , "tr" , "tr-CY" , "und"] ; if let Ok (payload) = KEYS . binary_search_by (| k | req . locale . strict_cmp (k . as_bytes ()) . reverse ()) . map (| i | * unsafe { VALUES . get_unchecked (i) }) { Ok (icu_provider :: DataResponse { payload : Some (icu_provider :: DataPayload :: from_static_ref (payload)) , metadata : Default :: default () , }) } else { Err (icu_provider :: DataErrorKind :: MissingLocale . with_req (< icu_relativetime :: provider :: NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider :: KeyedDataMarker > :: KEY , req)) } } } } }