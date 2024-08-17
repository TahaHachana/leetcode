// Reverse String
// https://leetcode.com/problems/reverse-string/description/

fn reverse_string(mut s []string) {
	mut left := 0
	mut right := s.len - 1

	for left < right {
		s[left], s[right] = s[right], s[left]
		left += 1
		right -= 1
	}
}
