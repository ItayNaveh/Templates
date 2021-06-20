fn main() {
	let mut html = std::fs::read_to_string("assets/index.html").unwrap();

	
	loop {
		let style_replacer = replacer(&html, ReplacerStuff {
			start_replacer:			r#"<link rel="stylesheet" href=""#.to_string(),
			to_replace_half1:		r#"<link rel="stylesheet" href=""#.to_string(),
			to_replace_half2:		r#"" />"#.to_string(),
			replacing_tag_half1:	"<style>".to_string(),
			replacing_tag_half2:	"</style>".to_string(),
		});

		match style_replacer {
			None => break,
			Some(thing) => html = thing,
		}
	}

	loop {
		let script_replacer = replacer(&html, ReplacerStuff {
			start_replacer:			r#"<script src=""#.to_string(),
			to_replace_half1:		r#"<script src=""#.to_string(),
			to_replace_half2:		r#""></script>"#.to_string(),
			replacing_tag_half1:	"<script>".to_string(),
			replacing_tag_half2:	"</script>".to_string(),
		});

		match script_replacer {
			None => break,
			Some(thing) => html = thing,
		}
	}

	std::fs::write("target/page.html", html).unwrap();
}


struct ReplacerStuff {
	start_replacer: String,
	to_replace_half1: String,
	to_replace_half2: String,
	replacing_tag_half1: String,
	replacing_tag_half2: String,
}

fn replacer(html: &String, replacer_stuff: ReplacerStuff) -> Option<String> {
	loop {
		let tag_pos = html.find(&replacer_stuff.start_replacer);

		if tag_pos == None {
			return None;
		}

		let tag_pos = tag_pos.unwrap();

		let file_name = html[tag_pos + replacer_stuff.start_replacer.len()..].split('"').nth(0).unwrap();
		// println!("name {}", name);

		let content = std::fs::read_to_string(format!("assets/{}", file_name)).unwrap();
		// println!("content {}", content);

		return Some(html.replace(
			&format!("{}{}{}", replacer_stuff.to_replace_half1, file_name, replacer_stuff.to_replace_half2),
			&format!("{}{}{}", replacer_stuff.replacing_tag_half1, content, replacer_stuff.replacing_tag_half2)
		));
		// println!("full html\n{}", html);
	}
}
