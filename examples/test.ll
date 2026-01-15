; ModuleID = 'main'
source_filename = "main"

define i64 @main(i64 %0, i64 %1) {
entry:
  %a = alloca i64, align 8
  store i64 1, ptr %a, align 4
  ret i64 0
}

