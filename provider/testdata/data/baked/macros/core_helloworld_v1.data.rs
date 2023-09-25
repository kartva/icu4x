// @generated
# [doc = " Implement `DataProvider<HelloWorldV1Marker>` on the given struct using the data"] # [doc = r" hardcoded in this file. This allows the struct to be used with"] # [doc = r" `icu`'s `_unstable` constructors."] # [doc (hidden)] # [macro_export] macro_rules ! __impl_core_helloworld_v1 { ($ provider : ty) => { # [clippy :: msrv = "1.66"] const _ : () = < $ provider > :: MUST_USE_MAKE_PROVIDER_MACRO ; # [clippy :: msrv = "1.66"] impl icu_provider :: DataProvider < icu_provider :: hello_world :: HelloWorldV1Marker > for $ provider { fn load (& self , req : icu_provider :: DataRequest ,) -> Result < icu_provider :: DataResponse < icu_provider :: hello_world :: HelloWorldV1Marker > , icu_provider :: DataError > { static EN : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Hello World") , } ; static EN_GB : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Hello from 🇬🇧") , } ; static EN_002 : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Hello from 🌍") , } ; static EN_019 : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Hello from 🌎") , } ; static EN_142 : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Hello from 🌏") , } ; static EN_GB_U_SD_GBENG : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Hello from 🏴\u{e0067}\u{e0062}\u{e0065}\u{e006e}\u{e0067}\u{e007f}") , } ; static EN_001 : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Hello from 🗺\u{fe0f}") , } ; static SR_LATN : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Pozdrav svete") , } ; static SR : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Поздрав свете") , } ; static RU : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("Привет, мир") , } ; static BN : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("ওহে বিশ\u{9cd}ব") , } ; static JA : < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable = icu_provider :: hello_world :: HelloWorldV1 { message : alloc :: borrow :: Cow :: Borrowed ("こんにちは世界") , } ; static VALUES : [& < icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: DataMarker > :: Yokeable ; 13usize] = [& BN , & EN , & EN_001 , & EN_002 , & EN_019 , & EN_142 , & EN_GB , & EN_GB_U_SD_GBENG , & EN_001 , & JA , & RU , & SR , & SR_LATN] ; static KEYS : [& str ; 13usize] = ["bn" , "en" , "en-001" , "en-002" , "en-019" , "en-142" , "en-GB" , "en-GB-u-sd-gbeng" , "en-ZA" , "ja" , "ru" , "sr" , "sr-Latn"] ; if let Ok (payload) = KEYS . binary_search_by (| k | req . locale . strict_cmp (k . as_bytes ()) . reverse ()) . map (| i | * unsafe { VALUES . get_unchecked (i) }) { Ok (icu_provider :: DataResponse { payload : Some (icu_provider :: DataPayload :: from_static_ref (payload)) , metadata : Default :: default () , }) } else { Err (icu_provider :: DataErrorKind :: MissingLocale . with_req (< icu_provider :: hello_world :: HelloWorldV1Marker as icu_provider :: KeyedDataMarker > :: KEY , req)) } } } } }