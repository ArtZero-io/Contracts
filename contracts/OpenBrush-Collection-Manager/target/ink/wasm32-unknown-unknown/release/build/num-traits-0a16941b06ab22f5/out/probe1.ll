; ModuleID = 'probe1.ff4659d7-cgu.0'
source_filename = "probe1.ff4659d7-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

; probe1::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe15probe17h7a7de3a6e9271dc4E() unnamed_addr #0 {
start:
; call core::f64::<impl f64>::to_int_unchecked
  %_1 = call i32 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$16to_int_unchecked17hb05fe14b3c2d34c7E"(double 1.000000e+00) #3
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::f64::<impl f64>::to_int_unchecked
; Function Attrs: inlinehint nounwind
define hidden i32 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$16to_int_unchecked17hb05fe14b3c2d34c7E"(double %self) unnamed_addr #1 {
start:
; call <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
  %0 = call i32 @"_ZN65_$LT$f64$u20$as$u20$core..convert..num..FloatToInt$LT$i32$GT$$GT$16to_int_unchecked17h3fcecbe0a2f272d9E"(double %self) #3
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
; Function Attrs: inlinehint nounwind
define internal i32 @"_ZN65_$LT$f64$u20$as$u20$core..convert..num..FloatToInt$LT$i32$GT$$GT$16to_int_unchecked17h3fcecbe0a2f272d9E"(double %self) unnamed_addr #1 {
start:
  %0 = alloca i32, align 4
  %1 = call i32 @llvm.wasm.trunc.signed.i32.f64(double %self)
  store i32 %1, i32* %0, align 4
  %2 = load i32, i32* %0, align 4
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %2
}

; Function Attrs: nounwind readnone
declare i32 @llvm.wasm.trunc.signed.i32.f64(double) #2

attributes #0 = { nounwind "target-cpu"="generic" }
attributes #1 = { inlinehint nounwind "target-cpu"="generic" }
attributes #2 = { nounwind readnone }
attributes #3 = { nounwind }
