// @generated
# [doc = " Implement `DataProvider<ScriptValueToShortNameV1Marker>` on the given struct using the data"] # [doc = r" hardcoded in this file. This allows the struct to be used with"] # [doc = r" `icu`'s `_unstable` constructors."] # [doc (hidden)] # [macro_export] macro_rules ! __impl_propnames_to_short_linear4_sc_v1 { ($ provider : ty) => { # [clippy :: msrv = "1.66"] const _ : () = < $ provider > :: MUST_USE_MAKE_PROVIDER_MACRO ; # [clippy :: msrv = "1.66"] impl $ provider { # [doc (hidden)] pub const SINGLETON_PROPNAMES_TO_SHORT_LINEAR4_SC_V1 : & 'static < icu_properties :: provider :: ScriptValueToShortNameV1Marker as icu_provider :: DataMarker > :: Yokeable = & icu_properties :: provider :: names :: PropertyEnumToValueNameLinearTiny4MapV1 { map : unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (b"ZyyyZinhArabArmnBengBopoCherCoptCyrlDsrtDevaEthiGeorGothGrekGujrGuruHaniHangHebrHiraKndaKanaKhmrLaooLatnMlymMongMymrOgamItalOryaRunrSinhSyrcTamlTeluThaaThaiTibtCansYiiiTglgHanoBuhdTagbBraiCprtLimbLinbOsmaShawTaleUgarHrktBugiGlagKharSyloTaluTfngXpeoBaliBatkBlisBrahChamCirtCyrsEgydEgyhEgypGeokHansHantHmngHungIndsJavaKaliLatfLatgLepcLinaMandMayaMeroNkooOrkhPermPhagPhnxPlrdRoroSaraSyreSyrjSyrnTengVaiiVispXsuxZxxxZzzzCariJpanLanaLyciLydiOlckRjngSaurSgnwSundMoonMteiArmiAvstCakmKoreKthiManiPhliPhlpPhlvPrtiSamrTavtZmthZsymBamuLisuNkgbSarbBassDuplElbaGranKpelLomaMendMercNarbNbatPalmSindWaraAfakJurcMrooNshuShrdSoraTakrTangWoleHluwKhojTirhAghbMahjAhomHatrModiMultPaucSiddAdlmBhksMarcNewaOsgeHanbJamoZsyeGonmSoyoZanbDogrGongMakaMedfRohgSogdSogoElymHmnpNandWchoChrsDiakKitsYeziCpmnOugrTnsaTotoVithKawiNagm") } , } ; } # [clippy :: msrv = "1.66"] impl icu_provider :: DataProvider < icu_properties :: provider :: ScriptValueToShortNameV1Marker > for $ provider { fn load (& self , req : icu_provider :: DataRequest ,) -> Result < icu_provider :: DataResponse < icu_properties :: provider :: ScriptValueToShortNameV1Marker > , icu_provider :: DataError > { if req . locale . is_empty () { Ok (icu_provider :: DataResponse { payload : Some (icu_provider :: DataPayload :: from_static_ref (Self :: SINGLETON_PROPNAMES_TO_SHORT_LINEAR4_SC_V1)) , metadata : Default :: default () , }) } else { Err (icu_provider :: DataErrorKind :: ExtraneousLocale . with_req (< icu_properties :: provider :: ScriptValueToShortNameV1Marker as icu_provider :: KeyedDataMarker > :: KEY , req)) } } } } }