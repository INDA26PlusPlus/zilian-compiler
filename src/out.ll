; ModuleID = 'test'
source_filename = "test"

define i64 @main() {
main_block:
  %temp = alloca i64, align 8
  %loop_counter = alloca i64, align 8
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  %runTime = alloca i64, align 8
  store i64 6, ptr %runTime, align 4
  store i64 0, ptr %a, align 4
  store i64 1, ptr %b, align 4
  %runTime1 = load i64, ptr %runTime, align 4
  store i64 0, ptr %loop_counter, align 4
  br label %loop_cond

loop_cond:                                        ; preds = %loop_body, %main_block
  %counter = load i64, ptr %loop_counter, align 4
  %continue = icmp slt i64 %counter, %runTime1
  br i1 %continue, label %loop_body, label %loop_end

loop_body:                                        ; preds = %loop_cond
  %a2 = load i64, ptr %a, align 4
  store i64 %a2, ptr %temp, align 4
  %b3 = load i64, ptr %b, align 4
  store i64 %b3, ptr %a, align 4
  %a4 = load i64, ptr %a, align 4
  %temp5 = load i64, ptr %temp, align 4
  %add = add i64 %a4, %temp5
  store i64 %add, ptr %b, align 4
  %counter6 = load i64, ptr %loop_counter, align 4
  %next = add i64 %counter6, 1
  store i64 %next, ptr %loop_counter, align 4
  br label %loop_cond

loop_end:                                         ; preds = %loop_cond
  %return = load i64, ptr %a, align 4
  ret i64 %return
}
