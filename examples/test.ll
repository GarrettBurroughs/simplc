; ModuleID = 'main'
source_filename = "main"

define i64 @main(i64 %0, i64 %1) {
entry:
  %x0 = alloca i64, align 8
  store i64 0, ptr %x0, align 4
  %b1 = alloca i64, align 8
  %x01 = load i64, ptr %x0, align 4
  %inc_tmp = load i64, ptr %x0, align 4
  %inc_add = add i64 %inc_tmp, 1
  store i64 %inc_add, ptr %x0, align 4
  store i64 %x01, ptr %b1, align 4
  ret i64 1
}

