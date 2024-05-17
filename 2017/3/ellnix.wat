(module
  (import "env" "input" (memory 2))

  (import "env" "debug_func"
    (func $debug(param i32)))

  (import "env" "print_memory_32"
    (func $print_mem(param i32 i32)))

  (func $abs (param $n i32) (result i32)
    (if (result i32)(i32.lt_s (local.get $n) (i32.const 0))
      (then (i32.mul (local.get $n) (i32.const -1)))
      (else (local.get $n))
    )
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

  (func $read_number (param $ptr i32)(param $len i32) (result i32)
    (local $n i32)
    (local $i i32)

    (local.set $n (i32.const 0))
    (local.set $i (i32.const 0))
    
    (loop $read_digits
      (local.set $len (i32.sub (local.get $len) (i32.const 1)))

      (local.set $n (i32.add (local.get $n) (i32.mul (i32.sub (i32.load8_u (local.get $len)) (i32.const 48)) (call $powow (i32.const 10) (local.get $i)))))

      (local.set $i (i32.add (local.get $i) (i32.const 1)))

      (if (i32.lt_u (local.get $ptr) (local.get $len))
        (then (br $read_digits))
      )
    )

    (local.get $n)
  )

  (func (export "part1") (param $ptr i32) (param $len i32) (result i32)
    (local $n i32)
    (local $spiral_i i32)
    (local $spiral_len i32)

    (local.set $spiral_i (i32.const 0))
    (local.set $spiral_len (i32.const 1))
    
    (local.set $n (call $read_number (local.get $ptr)(local.get $len)))

    (loop $normalize_on_spiral
      (local.set $n (i32.sub (local.get $n) (local.get $spiral_len)))

      (local.set $spiral_i (i32.add (local.get $spiral_i) (i32.const 1)))
      (local.set $spiral_len (i32.mul (local.get $spiral_i) (i32.const 8)))


      (if (i32.ge_s (local.get $n) (local.get $spiral_len))
        (then (br $normalize_on_spiral))
      )
    )

    (i32.add 
      (local.get $spiral_i) 
      (call $abs
        (i32.sub
          (i32.rem_u (local.get $n) (i32.div_u (local.get $spiral_len) (i32.const 4)))
          (local.get $spiral_i)
        )
      )
    )
  )

  (global $cur (mut i32) (i32.const 0))
  (global $cur_len (mut i32) (i32.const 0))
  (global $cur_cap (mut i32) (i32.const 0))

  (global $prev (mut i32) (i32.const 2000))
  (global $prev_cap (mut i32) (i32.const 0))

  (func $push_cur (param $n i32)
    (i32.store 
      (i32.add (global.get $cur) (i32.mul (global.get $cur_len) (i32.const 4)))
      (local.get $n)
    )
    (global.set $cur_len (i32.add (global.get $cur_len) (i32.const 1)))
  )

  (func $fetch_prev (param $q_i i32) (param $c_i i32) (result i32)
    (local $prev_side i32)
    (local $i i32)

    (local.set $prev_side (i32.div_u (global.get $prev_cap) (i32.const 4)))
    (if (result i32)(i32.gt_s (local.get $c_i) (local.get $prev_side))
      (then (i32.const 0))
      (else
        (local.set 
          $i 
          (i32.add
            (i32.mul (local.get $q_i) (local.get $prev_side))
            (local.get $c_i)
          )
        )
        (if (result i32)(i32.lt_s (local.get $i) (i32.const 0))
          (then (i32.const 0))
          (else
            (i32.load (i32.add (global.get $prev) (i32.mul (local.get $i) (i32.const 4))))
          )
        )
      )
    )
  )

  (func $fetch_cur (param $q_i i32) (param $c_i i32) (result i32)
    (local $i i32)
    (local $cur_side i32)

    (local.set $cur_side (i32.div_u (global.get $cur_cap) (i32.const 4)))

    (local.set 
      $i 
      (i32.add
        (i32.mul (local.get $q_i) (local.get $cur_side))
        (local.get $c_i)
      )
    )

    (if (result i32)(i32.ge_s (local.get $i) (i32.const 1))
      (then 
        (i32.load (i32.add (global.get $cur) (i32.mul (local.get $i) (i32.const 4))))
      )
      (else (i32.const 0)))
  )

  (func $fetch_start_if_necessary (param $q_i i32) (param $c_i i32) (result i32)
    (local $cur_side i32)

    (local.set $cur_side (i32.div_u (global.get $cur_cap) (i32.const 4)))

    (if (result i32)
      (i32.or 
        (i32.eq (local.get $q_i) (i32.const 4))
        (i32.and
          (i32.eq (local.get $q_i) (i32.const 3))
          (i32.eq (local.get $c_i) (i32.sub (local.get $cur_side) (i32.const 1)))
        )
      )
      (then (i32.load (i32.add (i32.const 4) (global.get $cur))))
      (else (i32.const 0))
    )
  )

  (func $calc_next (param $i i32) (result i32)
    (local $cur_side i32)
    (local $q_i i32)
    (local $c_i i32)

    (local.set $cur_side (i32.div_u (global.get $cur_cap) (i32.const 4)))
    (local.set $q_i (i32.div_u (local.get $i) (local.get $cur_side)))
    (local.set $c_i (i32.rem_u (local.get $i) (local.get $cur_side)))

    (if (result i32)(i32.eq (local.get $c_i) (i32.const 1))
      ;; post-corner case
      (then
        (i32.add
          (i32.add
            (call $fetch_prev (local.get $q_i) (local.get $c_i)) ;; with c_i
            (call $fetch_prev (local.get $q_i) (i32.sub (local.get $c_i) (i32.const 1))) ;; with c_i - 1
          )
          (i32.add
            (call $fetch_cur (local.get $q_i) (i32.sub (local.get $c_i) (i32.const 1))) ;; with c_i - 1
            (call $fetch_cur (local.get $q_i) (i32.sub (local.get $c_i) (i32.const 2))) ;; with c_i - 2
          )
        )
      )
      (else
        (if (result i32)(i32.eq (local.get $c_i) (i32.const 0))
          ;; corner case
          (then
            (i32.add
              (call $fetch_prev (local.get $q_i) (local.get $c_i)) ;; with c_i
              (i32.add
                (call $fetch_cur (local.get $q_i) (i32.sub (local.get $c_i) (i32.const 1))) ;; with c_i - 1
                (call $fetch_start_if_necessary (local.get $q_i) (local.get $c_i))
              )
            )
          )
          ;; general case
          (else
            (i32.add
              (i32.add
                (call $fetch_prev (local.get $q_i) (local.get $c_i)) ;; with c_i
                (call $fetch_prev (local.get $q_i) (i32.sub (local.get $c_i) (i32.const 1))) ;; with c_i - 1
              )
              (i32.add
                (call $fetch_prev (local.get $q_i) (i32.sub (local.get $c_i) (i32.const 2))) ;; with c_i - 2
                (i32.add
                  (call $fetch_start_if_necessary (local.get $q_i) (local.get $c_i))
                  (call $fetch_cur (local.get $q_i) (i32.sub (local.get $c_i) (i32.const 1))) ;; with c_i - 1
                )
              )
            )
          )
        )
      )
    )
  )

  (func $seed_initial
    (call $push_cur (i32.const 25))
    (call $push_cur (i32.const  1))
    (call $push_cur (i32.const  2))
    (call $push_cur (i32.const  4))
    (call $push_cur (i32.const  5))
    (call $push_cur (i32.const 10))
    (call $push_cur (i32.const 11))
    (call $push_cur (i32.const 23))
    (call $push_cur (i32.const 25))

    (global.set $cur_cap (i32.const 9))
    (call $swap_cur_next)
  )

  (func $swap_cur_next
    (local $tmp i32)

    (local.set $tmp (global.get $cur))
    (global.set $cur (global.get $prev))
    (global.set $prev (local.get $tmp))

    (global.set $cur_len (i32.const 0))

    (global.set $prev_cap (global.get $cur_cap))
    (global.set $cur_cap (i32.add (global.get $cur_cap) (i32.const 8)))
  )

  (func (export "part2") (param $ptr i32) (param $len i32) (result i32)
    (local $n i32)
    (local $i i32)
    (local $target i32)

    (local.set $target (call $read_number (local.get $ptr) (local.get $len)))

    (call $seed_initial)

    (block $found
      (loop $another_spiral
        (call $push_cur (i32.const 0))
        (local.set $i (i32.const 1))

        (loop $n_in_spiral
          (local.set $n (call $calc_next (local.get $i)))
          (if (i32.gt_u (local.get $n) (local.get $target))
            (then (br $found)))

          (call $push_cur (local.get $n))

          (local.set $i (i32.add (local.get $i) (i32.const 1)))
          (if (i32.lt_u (local.get $i) (global.get $cur_cap))
            (then (br $n_in_spiral)))
        )

        (i32.store (global.get $cur) (local.get $n))
        (call $swap_cur_next)
        (br $another_spiral)
      )
    )

    (local.get $n)
  )
)
