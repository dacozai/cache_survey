/// # Example
/// ```
/// use mycache::mytest::min_swaps;
/// let s = "[[[]]]][][]][[]]][[[".to_string();
/// assert_eq!(2, min_swaps(s));
/// let s = "[]".to_string();
/// assert_eq!(0, min_swaps(s));
/// let s = "]]][[[".to_string();
/// assert_eq!(2, min_swaps(s));
/// let s = "][][".to_string();
/// assert_eq!(1, min_swaps(s));
/// ```
pub fn min_swaps(s: String) -> i32 {
    let mut imb = 0i32;
    let mut nc = 0i32;
    for c in s.chars() {
        if c == '[' {
            nc += 1;
        } else if nc > 0 {
            nc -= 1;
        } else {
            imb += 1;
        }
    }
    (imb + 1) / 2
}

#[allow(dead_code)]
fn min_swaps_stk(s: String) -> i32 {
    let mut stk = vec![];
    for c in s.chars() {
        if let Some(&pv) = stk.last() {
            if pv == '[' && c == ']' {
                stk.pop();
            } else {
                stk.push(c);
            }
        } else {
            stk.push(c);
        }
    }
    (stk.len() as i32 / 2 + 1) / 2
}

/// # Example
/// ```
/// use mycache::mytest::max_width_ramp;
/// let nums = vec![6, 0, 8, 2, 1, 5];
/// assert_eq!(4, max_width_ramp(nums));
/// let nums = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
/// assert_eq!(7, max_width_ramp(nums));
/// ```
pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let mut stk = vec![];
    let mut ans = 0i32;

    // first pass
    for (i, &v) in nums.iter().enumerate() {
        if stk.is_empty() || v < nums[*stk.last().unwrap()] {
            stk.push(i);
        }
    }

    // second pass
    for i in (0..nums.len()).rev() {
        while !stk.is_empty() && nums[*stk.last().unwrap()] <= nums[i] {
            ans = ans.max((i - stk.pop().unwrap()) as i32);
        }
    }

    ans
}


/// # Example
/// ```
/// use mycache::mytest::parse_bool_expr;
///
/// let s = "&(|(f))".to_string();
/// assert!(!parse_bool_expr(s));
/// let s = "|(f,f,f,t)".to_string();
/// assert!(parse_bool_expr(s));
/// let s = "!(&(f,t))".to_string();
/// assert!(parse_bool_expr(s));
/// let s = "|(&(t,f,t),t)".to_string();
/// assert!(parse_bool_expr(s));
/// ```
pub fn parse_bool_expr(expression: String) -> bool {
    let mut stk: Vec<u8> = vec![];
    let mut ops: Vec<u8> = vec![];
    let exp = expression.as_bytes();
    let mut cur = 0;
    while cur < exp.len() {
        match exp[cur] {
            b'!' | b'&' | b'|' => ops.push(exp[cur]),
            b',' => (),
            b')' => {
                if let Some(cchh) = ops.pop() {
                    let res = if cchh == b'!' {
                        let b = stk.pop().unwrap();
                        if b == b't' { b'f' } else { b't' }
                    } else if cchh == b'&' {
                        let mut b = b't';
                        while *stk.last().unwrap() != b'(' {
                            let bb = stk.pop().unwrap();
                            if bb == b'f' {
                                b = b'f';
                            }
                        }
                        b
                    } else {
                        let mut b = b'f';
                        while *stk.last().unwrap() != b'(' {
                            let bb = stk.pop().unwrap();
                            if bb == b't' {
                                b = b't';
                            }
                        }
                        b
                    };
                    stk.pop();
                    stk.push(res);
                }
            }
            _ => stk.push(exp[cur]),
        }
        cur += 1;
    }

    stk[0] == b't'
}
