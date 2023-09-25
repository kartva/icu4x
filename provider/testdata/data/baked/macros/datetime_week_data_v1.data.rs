// @generated
# [doc = " Implement `DataProvider<WeekDataV1Marker>` on the given struct using the data"] # [doc = r" hardcoded in this file. This allows the struct to be used with"] # [doc = r" `icu`'s `_unstable` constructors."] # [doc (hidden)] # [macro_export] macro_rules ! __impl_datetime_week_data_v1 { ($ provider : ty) => { # [clippy :: msrv = "1.66"] const _ : () = < $ provider > :: MUST_USE_MAKE_PROVIDER_MACRO ; # [clippy :: msrv = "1.66"] impl icu_provider :: DataProvider < icu_calendar :: provider :: WeekDataV1Marker > for $ provider { fn load (& self , req : icu_provider :: DataRequest ,) -> Result < icu_provider :: DataResponse < icu_calendar :: provider :: WeekDataV1Marker > , icu_provider :: DataError > { static UND_MV : < icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_calendar :: provider :: WeekDataV1 { first_weekday : icu_calendar :: types :: IsoWeekday :: Friday , min_week_days : 1u8 , } ; static CCP : < icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_calendar :: provider :: WeekDataV1 { first_weekday : icu_calendar :: types :: IsoWeekday :: Monday , min_week_days : 1u8 , } ; static ES : < icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_calendar :: provider :: WeekDataV1 { first_weekday : icu_calendar :: types :: IsoWeekday :: Monday , min_week_days : 4u8 , } ; static AR : < icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_calendar :: provider :: WeekDataV1 { first_weekday : icu_calendar :: types :: IsoWeekday :: Saturday , min_week_days : 1u8 , } ; static BN : < icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_calendar :: provider :: WeekDataV1 { first_weekday : icu_calendar :: types :: IsoWeekday :: Sunday , min_week_days : 1u8 , } ; static UND_PT : < icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_calendar :: provider :: WeekDataV1 { first_weekday : icu_calendar :: types :: IsoWeekday :: Sunday , min_week_days : 4u8 , } ; static VALUES : [& < icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: DataMarker > :: Yokeable ; 172usize] = [& AR , & AR , & BN , & CCP , & BN , & CCP , & BN , & ES , & CCP , & BN , & ES , & BN , & ES , & CCP , & CCP , & BN , & CCP , & CCP , & ES , & AR , & AR , & BN , & CCP , & CCP , & CCP , & ES , & CCP , & BN , & ES , & CCP , & ES , & CCP , & CCP , & BN , & ES , & ES , & AR , & CCP , & CCP , & BN , & BN , & BN , & BN , & CCP , & BN , & BN , & ES , & CCP , & CCP , & CCP , & BN , & CCP , & CCP , & ES , & ES , & AR , & ES , & BN , & BN , & AR , & CCP , & ES , & AR , & ES , & BN , & ES , & ES , & ES , & ES , & ES , & CCP , & ES , & ES , & ES , & ES , & ES , & BN , & BN , & BN , & BN , & CCP , & ES , & BN , & ES , & BN , & ES , & BN , & AR , & AR , & ES , & ES , & ES , & BN , & AR , & BN , & BN , & CCP , & BN , & BN , & AR , & CCP , & BN , & CCP , & ES , & CCP , & ES , & ES , & CCP , & AR , & ES , & CCP , & CCP , & BN , & CCP , & BN , & CCP , & BN , & ES , & BN , & UND_MV , & BN , & CCP , & BN , & BN , & ES , & ES , & BN , & CCP , & AR , & BN , & BN , & BN , & BN , & ES , & BN , & UND_PT , & BN , & AR , & ES , & CCP , & CCP , & ES , & BN , & AR , & ES , & BN , & CCP , & ES , & ES , & ES , & BN , & AR , & BN , & CCP , & CCP , & CCP , & BN , & BN , & CCP , & BN , & BN , & CCP , & CCP , & ES , & BN , & BN , & CCP , & BN , & CCP , & BN , & BN , & BN] ; static KEYS : [& str ; 172usize] = ["ar" , "ar-EG" , "bn" , "ccp" , "en" , "en-001" , "en-ZA" , "es" , "es-AR" , "fil" , "fr" , "ja" , "ru" , "sr" , "sr-Latn" , "th" , "tr" , "und" , "und-AD" , "und-AE" , "und-AF" , "und-AG" , "und-AI" , "und-AL" , "und-AM" , "und-AN" , "und-AR" , "und-AS" , "und-AT" , "und-AU" , "und-AX" , "und-AZ" , "und-BA" , "und-BD" , "und-BE" , "und-BG" , "und-BH" , "und-BM" , "und-BN" , "und-BR" , "und-BS" , "und-BT" , "und-BW" , "und-BY" , "und-BZ" , "und-CA" , "und-CH" , "und-CL" , "und-CM" , "und-CN" , "und-CO" , "und-CR" , "und-CY" , "und-CZ" , "und-DE" , "und-DJ" , "und-DK" , "und-DM" , "und-DO" , "und-DZ" , "und-EC" , "und-EE" , "und-EG" , "und-ES" , "und-ET" , "und-FI" , "und-FJ" , "und-FO" , "und-FR" , "und-GB" , "und-GE" , "und-GF" , "und-GG" , "und-GI" , "und-GP" , "und-GR" , "und-GT" , "und-GU" , "und-HK" , "und-HN" , "und-HR" , "und-HU" , "und-ID" , "und-IE" , "und-IL" , "und-IM" , "und-IN" , "und-IQ" , "und-IR" , "und-IS" , "und-IT" , "und-JE" , "und-JM" , "und-JO" , "und-JP" , "und-KE" , "und-KG" , "und-KH" , "und-KR" , "und-KW" , "und-KZ" , "und-LA" , "und-LB" , "und-LI" , "und-LK" , "und-LT" , "und-LU" , "und-LV" , "und-LY" , "und-MC" , "und-MD" , "und-ME" , "und-MH" , "und-MK" , "und-MM" , "und-MN" , "und-MO" , "und-MQ" , "und-MT" , "und-MV" , "und-MX" , "und-MY" , "und-MZ" , "und-NI" , "und-NL" , "und-NO" , "und-NP" , "und-NZ" , "und-OM" , "und-PA" , "und-PE" , "und-PH" , "und-PK" , "und-PL" , "und-PR" , "und-PT" , "und-PY" , "und-QA" , "und-RE" , "und-RO" , "und-RS" , "und-RU" , "und-SA" , "und-SD" , "und-SE" , "und-SG" , "und-SI" , "und-SJ" , "und-SK" , "und-SM" , "und-SV" , "und-SY" , "und-TH" , "und-TJ" , "und-TM" , "und-TR" , "und-TT" , "und-TW" , "und-UA" , "und-UM" , "und-US" , "und-UY" , "und-UZ" , "und-VA" , "und-VE" , "und-VI" , "und-VN" , "und-WS" , "und-XK" , "und-YE" , "und-ZA" , "und-ZW"] ; if let Ok (payload) = KEYS . binary_search_by (| k | req . locale . strict_cmp (k . as_bytes ()) . reverse ()) . map (| i | * unsafe { VALUES . get_unchecked (i) }) { Ok (icu_provider :: DataResponse { payload : Some (icu_provider :: DataPayload :: from_static_ref (payload)) , metadata : Default :: default () , }) } else { Err (icu_provider :: DataErrorKind :: MissingLocale . with_req (< icu_calendar :: provider :: WeekDataV1Marker as icu_provider :: KeyedDataMarker > :: KEY , req)) } } } } }