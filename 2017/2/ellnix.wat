(module
  (import "env" "input" (memory 1))

  (import "env" "debug_func"
    (func $debug(param i32)))

  (func $update_min (param $min i32)(param $n i32)(result i32)
    (if (result i32)(i32.or (i32.lt_s (local.get $min) (i32.const 0)) (i32.gt_s (local.get $min) (local.get $n)))
    (then
      (local.get $n)
    )
    (else
      (local.get $min)
    ))
  )

  (func $update_max (param $max i32)(param $n i32)(result i32)
    (if (result i32)(i32.or (i32.lt_s (local.get $max) (i32.const 0)) (i32.gt_s (local.get $n) (local.get $max)))
    (then
      (local.get $n)
    )
    (else
      (local.get $max)
    ))
  )

  (func $powow (param $n i32)(param $exp i32)(result i32)
    (local $result i32)
    (local.set $result (i32.const 1))

    (block $exp
      (loop $mul_loop
        (local.set $exp (i32.sub (local.get $exp) (i32.const 1)))
        (if (i32.lt_s (local.get $exp) (i32.const 0))
          (then (br $exp)))
          (local.set $result
            (i32.mul (local.get $result) (local.get $n))
          )
        (br $mul_loop)
      )
    )

    (local.get $result)
  )

  (func (export "part1")(param $ptr i32)(param $len i32)(result i32)
    (local $min i32)
    (local $max i32)
    (local $checksum i32)
    (local $current_byte i32)
    (local $n i32)
    (local $buffer_n i32)
    (local $digit_n i32)

    (local.set $checksum (i32.const 0))
    (local.set $buffer_n (i32.const 0))
    (local.set $digit_n (i32.const 0))

    (loop $byte_loop
      (local.set $current_byte (i32.load8_u (local.get $len)))

      ;; A tab is 9 in ASCII, a newline 10
      (if (i32.or (i32.eq (local.get $current_byte) (i32.const 9)) (i32.eq (local.get $current_byte) (i32.const 10)))
        (then
          (local.set $min (call $update_min (local.get $min) (local.get $buffer_n)))
          (local.set $max (call $update_max (local.get $max) (local.get $buffer_n)))

          (local.set $buffer_n (i32.const 0))
          (local.set $digit_n (i32.const 0))
        )
        (else 
          (local.set $n (i32.sub (local.get $current_byte) (i32.const 48)))
          (local.set $buffer_n
            (i32.add (local.get $buffer_n)
              (i32.mul (local.get $n) (call $powow (i32.const 10) (local.get $digit_n)))
            )
          )
          (local.set $digit_n (i32.add (local.get $digit_n) (i32.const 1)))
        ))

      ;; Newline is 10 in ASCII
      (if (i32.eq (local.get $current_byte) (i32.const 10))
        (then 
          (local.set $checksum
            (i32.add (local.get $checksum) (i32.sub (local.get $max) (local.get $min)))
          )
          (local.set $max (i32.const -1))
          (local.set $min (i32.const -1))
        ))

      (local.set $len (i32.sub (local.get $len) (i32.const 1)))
      (if (i32.ge_s (local.get $len) (local.get $ptr))
        (br $byte_loop))
    )

    (local.set $checksum
      (i32.add (local.get $checksum) (i32.sub (local.get $max) (local.get $min)))
    )

    (local.get $checksum)
  )

  (func $push_to_arr (param $n i32) (param $arr i32) (param $len i32)
    (i32.store 
      ;; memory is addressed in bytes, 32 bits = 4 bytes
      (i32.add (local.get $arr) (i32.mul (local.get $len) (i32.const 4)))
      (local.get $n)
    )
  )

  (func $divide_divisible_in_arr (param $arr i32) (param $len i32)(result i32)
    (local $idx i32)
    (local $idx2 i32)
    (local $a i32)
    (local $b i32)
    (local $n i32)

    (loop $each_n
      (local.set $a (i32.load (i32.add (local.get $arr) (i32.mul (local.get $idx) (i32.const 4)))))
      (local.set $idx2 (i32.const 0))
      (loop $other_ns
        (block $skip_self
          (if (i32.eq (local.get $idx) (local.get $idx2))
          (then (br $skip_self))
          (else
            (local.set $b (i32.load (i32.add (local.get $arr) (i32.mul (local.get $idx2) (i32.const 4)))))
            (if (i32.eq (i32.rem_u (local.get $a) (local.get $b)) (i32.const 0))
              (then (return (i32.div_u (local.get $a) (local.get $b))))
          ))))

        (local.set $idx2 (i32.add (local.get $idx2) (i32.const 1)))
        (if (i32.lt_u (local.get $idx2) (local.get $len))
          (then (br $other_ns)))
      )

      (local.set $idx (i32.add (local.get $idx) (i32.const 1)))
      (if (i32.lt_u (local.get $idx) (local.get $len))
        (then (br $each_n)))
    )

    (i32.const 0)
  )

  (func (export "part2")(param $ptr i32)(param $len i32)(result i32)
    (local $current_byte i32)
    (local $n i32)
    (local $arr_ptr i32)
    (local $arr_len i32)
    (local $buffer_n i32)
    (local $digit_n i32)
    (local $checksum i32)

    (local.set $arr_ptr (i32.add (local.get $len) (i32.const 1)))

    (loop $byte_loop
      (local.set $current_byte (i32.load8_u (local.get $len)))

      ;; A tab is 9 in ASCII, a newline 10
      (if (i32.or (i32.eq (local.get $current_byte) (i32.const 9)) (i32.eq (local.get $current_byte) (i32.const 10)))
        (then
          ;; save number in memory
          (call $push_to_arr (local.get $buffer_n) (local.get $arr_ptr) (local.get $arr_len))

          (local.set $arr_len (i32.add (local.get $arr_len) (i32.const 1)))

          (local.set $buffer_n (i32.const 0))
          (local.set $digit_n (i32.const 0))
        )
        (else 
          (local.set $n (i32.sub (local.get $current_byte) (i32.const 48)))
          (local.set $buffer_n
            (i32.add (local.get $buffer_n)
              (i32.mul (local.get $n) (call $powow (i32.const 10) (local.get $digit_n)))
            )
          )
          (local.set $digit_n (i32.add (local.get $digit_n) (i32.const 1)))
        ))

      ;; Newline is 10 in ASCII
      (if (i32.eq (local.get $current_byte) (i32.const 10))
        (then 
          (local.set $checksum
            (i32.add (local.get $checksum) (call $divide_divisible_in_arr (local.get $arr_ptr) (local.get $arr_len))
          )

          (local.set $arr_len (i32.const 0))
        )))

      (local.set $len (i32.sub (local.get $len) (i32.const 1)))
      (if (i32.ge_s (local.get $len) (local.get $ptr))
        (br $byte_loop))
    )

    (local.set $checksum
      (i32.add (local.get $checksum) (call $divide_divisible_in_arr (local.get $arr_ptr) (local.get $arr_len)))
    )

    (local.get $checksum)
  )
)
