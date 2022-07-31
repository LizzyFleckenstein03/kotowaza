use unicode_width::UnicodeWidthChar;

pub trait Center {
	fn center(&self, term_width: usize) -> Self;
}

impl Center for String {
	fn center(&self, term_width: usize) -> Self {
		let mut result = String::new();
		let mut buffer = String::new();
		let mut buffer_width = 0;

		let max = self.chars().count() - 1;

		for (i, c) in self.chars().enumerate() {
			buffer_width += UnicodeWidthChar::width(c)
				.unwrap();

			buffer.push(c);

			if i == max || buffer_width >= term_width - 12 {
				result.push_str(&String::from(" ")
					.repeat((term_width - buffer_width) / 2));
				result.push_str(&buffer);

				if i != max {
					result.push('\n');
				}

				buffer.clear();
				buffer_width = 0;
			}
		}

		result
	}
}
