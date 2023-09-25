// @generated
# [doc = " Implement `DataProvider<LocaleFallbackParentsV1Marker>` on the given struct using the data"] # [doc = r" hardcoded in this file. This allows the struct to be used with"] # [doc = r" `icu`'s `_unstable` constructors."] # [doc (hidden)] # [macro_export] macro_rules ! __impl_fallback_parents_v1 { ($ provider : ty) => { # [clippy :: msrv = "1.66"] const _ : () = < $ provider > :: MUST_USE_MAKE_PROVIDER_MACRO ; # [clippy :: msrv = "1.66"] impl $ provider { # [doc (hidden)] pub const SINGLETON_FALLBACK_PARENTS_V1 : & 'static < icu_locid_transform :: provider :: LocaleFallbackParentsV1Marker as icu_provider :: DataMarker > :: Yokeable = & icu_locid_transform :: provider :: LocaleFallbackParentsV1 { parents : unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (unsafe { zerovec :: VarZeroVec :: from_bytes_unchecked (b"\x84\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1A\0\x1F\0$\0)\0.\x003\08\0=\0B\0G\0L\0Q\0V\0[\0`\0e\0j\0o\0t\0y\0~\0\x83\0\x88\0\x8D\0\x92\0\x97\0\x9C\0\xA1\0\xA6\0\xAB\0\xB0\0\xB5\0\xBA\0\xBF\0\xC4\0\xC9\0\xCE\0\xD3\0\xD8\0\xDD\0\xE2\0\xE7\0\xEC\0\xF1\0\xF6\0\xFB\0\0\x01\x05\x01\n\x01\x0F\x01\x14\x01\x19\x01\x1E\x01#\x01(\x01-\x012\x017\x01<\x01A\x01F\x01K\x01P\x01U\x01Z\x01_\x01d\x01i\x01n\x01s\x01x\x01}\x01\x82\x01\x87\x01\x8C\x01\x91\x01\x96\x01\x9B\x01\xA0\x01\xA5\x01\xAA\x01\xAF\x01\xB4\x01\xB9\x01\xBE\x01\xC3\x01\xC8\x01\xCD\x01\xD2\x01\xD7\x01\xDC\x01\xE1\x01\xE6\x01\xEB\x01\xF0\x01\xF5\x01\xFA\x01\xFF\x01\x04\x02\t\x02\x0E\x02\x13\x02\x18\x02\x1D\x02\"\x02'\x02,\x021\x026\x02;\x02@\x02G\x02I\x02K\x02M\x02R\x02W\x02\\\x02a\x02f\x02k\x02p\x02u\x02z\x02\x7F\x02\x84\x02\x89\x02en-150en-AGen-AIen-ATen-AUen-BBen-BEen-BMen-BSen-BWen-BZen-CCen-CHen-CKen-CMen-CXen-CYen-DEen-DGen-DKen-DMen-ERen-FIen-FJen-FKen-FMen-GBen-GDen-GGen-GHen-GIen-GMen-GYen-HKen-IEen-ILen-IMen-INen-IOen-JEen-JMen-KEen-KIen-KNen-KYen-LCen-LRen-LSen-MGen-MOen-MSen-MTen-MUen-MVen-MWen-MYen-NAen-NFen-NGen-NLen-NRen-NUen-NZen-PGen-PKen-PNen-PWen-RWen-SBen-SCen-SDen-SEen-SGen-SHen-SIen-SLen-SSen-SXen-SZen-TCen-TKen-TOen-TTen-TVen-TZen-UGen-VCen-VGen-VUen-WSen-ZAen-ZMen-ZWes-ARes-BOes-BRes-BZes-CLes-COes-CRes-CUes-DOes-ECes-GTes-HNes-MXes-NIes-PAes-PEes-PRes-PYes-SVes-USes-UYes-VEhi-Latnhtnbnnno-NOpt-AOpt-CHpt-CVpt-FRpt-GQpt-GWpt-LUpt-MOpt-MZpt-STpt-TLzh-Hant-MO") } , unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01150en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001en\0\0\0\0\0\0\x01001es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419es\0\0\0\0\0\0\x01419en\0\0\0\0\0\0\x01IN\0fr\0\0\0\0\0\0\x01HT\0no\0\0\0\0\0\0\0\0\0\0no\0\0\0\0\0\0\0\0\0\0no\0\0\0\0\0\0\0\0\0\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0pt\0\0\0\0\0\0\x01PT\0zh\0\x01Hant\x01HK\0") }) } , } ; } # [clippy :: msrv = "1.66"] impl icu_provider :: DataProvider < icu_locid_transform :: provider :: LocaleFallbackParentsV1Marker > for $ provider { fn load (& self , req : icu_provider :: DataRequest ,) -> Result < icu_provider :: DataResponse < icu_locid_transform :: provider :: LocaleFallbackParentsV1Marker > , icu_provider :: DataError > { if req . locale . is_empty () { Ok (icu_provider :: DataResponse { payload : Some (icu_provider :: DataPayload :: from_static_ref (Self :: SINGLETON_FALLBACK_PARENTS_V1)) , metadata : Default :: default () , }) } else { Err (icu_provider :: DataErrorKind :: ExtraneousLocale . with_req (< icu_locid_transform :: provider :: LocaleFallbackParentsV1Marker as icu_provider :: KeyedDataMarker > :: KEY , req)) } } } } }