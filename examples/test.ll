; ModuleID = 'main'
source_filename = "main"

define i64 @main(i64 %0, i64 %1) {
entry:
  %i = alloca i64, align 8
  %x = alloca i64, align 8
  store i64 1, ptr %x, align 4
  br label %post_declaration

post_declaration:                                 ; preds = %dead, %entry
  store i64 5, ptr %i, align 4
  %x1 = load i64, ptr %x, align 4
  %equals = icmp eq i64 %x1, 1
  %equals_ext = zext i1 %equals to i64
  %lhs_cmp = icmp ne i64 %equals_ext, 0
  br i1 %lhs_cmp, label %rhs_block, label %merge_block

dead:                                             ; No predecessors!
  store i64 0, ptr %x, align 4
  store i64 0, ptr %i, align 4
  br label %post_declaration

rhs_block:                                        ; preds = %post_declaration
  %i2 = load i64, ptr %i, align 4
  %equals3 = icmp eq i64 %i2, 5
  %equals_ext4 = zext i1 %equals3 to i64
  %lbool = icmp ne i64 %equals_ext4, 0
  %ExtendedResult = zext i1 %lbool to i64
  br label %merge_block

merge_block:                                      ; preds = %rhs_block, %post_declaration
  %logic_result = phi i64 [ 0, %post_declaration ], [ %ExtendedResult, %rhs_block ]
  ret i64 %logic_result
}

