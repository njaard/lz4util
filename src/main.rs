extern crate clap;
extern crate lz4;

use std::io::{Write,BufReader,BufWriter};

fn main()
{
	use clap::Arg;
	let matches
		= clap::App::new("lz4")
		.author("Kalle Samuels <ks@ks.ax>")
		.about("A tool to compress and decompress lz4 files")
		.arg(
			Arg::with_name("stdout")
				.long("to-stdout")
				.long("stdout")
				.short("c")
				.help("write to stdout, keep original file")
		)
		.arg(
			Arg::with_name("decompress")
				.long("decompress")
				.long("uncompress")
				.short("d")
				.help("decompress, instead of the default, compression")
		)
		.arg(
			Arg::with_name("file")
				.takes_value(true)
				.multiple(true)
				.help("filenames to compress (or decompress)")
		)
		.arg(
			Arg::with_name("force")
				.long("force")
				.short("f")
				.help("do not ask to overwrite destination files (and do so)")
		)
		.arg(
			Arg::with_name("keep")
				.long("keep")
				.short("k")
				.help("do not delete original file")
		)
		.get_matches();

	let decompressing = matches.is_present("decompress");

	let delete_original =
		!matches.is_present("keep") && !matches.is_present("stdout");

	let mut failure = false;

	let files : Vec<&str> =
		match matches.values_of("file")
		{
			Some(a) => a.collect(),
			None => vec!["-"],
		};

	let mut ask_overwrite = AskOverwrite::new(matches.is_present("force"));

	if decompressing
	{
		for name in files
		{
			if name == "-"
			{
				let file = std::io::stdin();
				let mut m = lz4::Decoder::new(file.lock())
					.expect(&format!("failed to decompress stdin: {}", name));

				let o = std::io::stdout();
				std::io::copy(&mut m, &mut o.lock())
					.expect("failed to decode lz4 stream");
			}
			else
			{
				{
					if !name.ends_with(".lz4")
					{
						eprintln!("{} ends with unknown suffix, skipping", name);
						failure = true;
						continue;
					}

					let new_name = &name[0..name.len()-".lz4".len()];
					if !matches.is_present("stdout") && !ask_overwrite.ask(new_name)
						{ continue; }

					let file = BufReader::new(std::fs::File::open(name)
						.expect(&format!("failed to open '{}'", name)));
					let mut output : Box<Write> =
						if matches.is_present("stdout")
						{
							Box::new(std::io::stdout())
						}
						else
						{
							Box::new(
								BufWriter::new(std::fs::File::create(new_name)
									.expect(&format!("failed to create '{}'", name)))
							)
						};

					let mut m = lz4::Decoder::new(file)
						.expect(&format!("failed to decompress {}", name));

					std::io::copy(&mut m, &mut *output)
						.expect("failed to decode lz4 stream");
				}

				if delete_original
				{
					std::fs::remove_file(name)
						.expect(&format!("failed to erase {}", name));
				}
			}
		}
	}
	else
	{ // compress
		for name in files
		{
			if name == "-"
			{
				let file = std::io::stdin();
				let o = std::io::stdout();

				let mut m = lz4::EncoderBuilder::new()
					.build(o.lock())
					.unwrap();

				std::io::copy(&mut file.lock(), &mut m)
					.expect("failed to encode lz4 stream");
				m.finish()
					.1.expect("failed to finish encoding lz4 stream");
			}
			else
			{
				{
					if name.ends_with(".lz4")
					{
						eprintln!("{} ends with suffix, skipping", name);
						failure = true;
						continue;
					}

					let new_name = &format!("{}.lz4", name);

					if !matches.is_present("stdout") && !ask_overwrite.ask(new_name)
						{ continue; }

					let mut file = BufReader::new(std::fs::File::open(name)
						.expect(&format!("failed to open {}", name)));
					let mut output : Box<Write> =
						if matches.is_present("stdout")
						{
							Box::new(std::io::stdout())
						}
						else
						{
							Box::new(
								BufWriter::new(std::fs::File::create(new_name)
									.expect(&format!("failed to create {}", name)))
							)
						};

					let mut m = lz4::EncoderBuilder::new()
						.build(output)
						.unwrap();
					std::io::copy(&mut file, &mut m)
						.expect("failed to encode lz4 stream");
					m.finish()
						.1.expect("failed to finish encoding lz4 stream");
				}

				if delete_original
				{
					std::fs::remove_file(name)
						.expect(&format!("failed to erase {}", name));
				}
			}
		}
	}

	if failure
	{
		std::process::exit(1);
	}
}

struct AskOverwrite
{
	force: bool,
}

impl AskOverwrite
{
	fn new(force : bool) -> AskOverwrite
	{
		AskOverwrite
		{
			force: force,
		}
	}

	fn ask(&mut self, new_name: &str) -> bool
	{
		let path = std::path::Path::new(new_name);
		if !path.exists()
		{
			true
		}
		else if path.exists() && self.force
		{
			std::fs::remove_file(path)
				.expect("failed to erase file");
			true
		}
		else if path.exists()
		{
			print!("lz4: {} already exists; do you wish to overwrite (y or n)? ", new_name);
			std::io::stdout().flush().unwrap();
			let mut line = String::new();
			// failure to read stdin means "no"
			let _ = std::io::stdin().read_line(&mut line);
			if line.starts_with("y")
			{
				std::fs::remove_file(path)
					.expect(&format!("failed to erase '{}'", new_name));
				true
			}
			else
			{
				println!("not overwritten");
				false
			}
		}
		else
		{
			println!("lz4: {} already exists; not overwritten", new_name);
			false
		}

	}
}


