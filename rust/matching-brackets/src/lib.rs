pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brack_stack = vec![];

    string.chars().filter(|v| "()[]{}".contains(&format!("{}", v))).fold(true, |acc, c| {
        if "([{".contains(c) {
            brack_stack.push(c);

            return false;
        }

        if ")]}".contains(c) {
            return match brack_stack.pop() {
                Some(e) if c == ')' => e == '(',
                Some(e) if c == ']' => e == '[',
                Some(e) if c == '}' => e == '{',
                Some(_) | None => false
            };
        }

        acc
    })

}
