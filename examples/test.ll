; ModuleID = 'main'
source_filename = "main"

define i64 @main(i64 %0, i64 %1) {
entry:
  %a = alloca i64, align 8
  store i64 1, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 2, ptr %b, align 4
  %flag = alloca i64, align 8
  store i64 0, ptr %flag, align 4
  %c = alloca i64, align 8
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %greater_than = icmp sgt i64 %a1, %b2
  %greater_than_ext = zext i1 %greater_than to i64
  %ternary_cmp = icmp ne i64 %greater_than_ext, 0
  br i1 %ternary_cmp, label %then_block, label %else_block

then_block:                                       ; preds = %entry
  br label %merge_block

else_block:                                       ; preds = %entry
  %flag3 = load i64, ptr %flag, align 4
  %ternary_cmp4 = icmp ne i64 %flag3, 0
  br i1 %ternary_cmp4, label %then_block5, label %else_block6

merge_block:                                      ; preds = %merge_block7, %then_block
  %ternary_phi8 = phi i64 [ 5, %then_block ], [ %ternary_phi, %merge_block7 ]
  store i64 %ternary_phi8, ptr %c, align 4
  %c9 = load i64, ptr %c, align 4
  ret i64 %c9

then_block5:                                      ; preds = %else_block
  br label %merge_block7

else_block6:                                      ; preds = %else_block
  br label %merge_block7

merge_block7:                                     ; preds = %else_block6, %then_block5
  %ternary_phi = phi i64 [ 6, %then_block5 ], [ 7, %else_block6 ]
  br label %merge_block
}

