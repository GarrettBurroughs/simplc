; ModuleID = 'main'
source_filename = "main"

define i64 @main() {
entry:
  %main_return = alloca i64, align 8
  store i64 0, ptr %main_return, align 4
  %foo_0 = alloca i64, align 8
  store i64 0, ptr %main_return, align 4
  br label %exit

exit:                                             ; preds = %dead, %entry
  %fn_return = load i64, ptr %main_return, align 4
  ret i64 %fn_return

dead:                                             ; No predecessors!
  br label %exit
}

declare i64 @foo()

