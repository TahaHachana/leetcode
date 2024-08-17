// Reverse String
// https://leetcode.com/problems/reverse-string/description/

let reverseString (s: string[]) : unit =
    let mutable left = 0
    let mutable right = s.Length - 1

    while left < right do
        let l, r = s[left], s[right]
        s[left] <- r
        s[right] <- l
        left <- left + 1
        right <- right - 1
