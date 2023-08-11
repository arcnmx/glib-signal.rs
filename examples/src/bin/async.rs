use {
	futures::StreamExt,
	glib::{g_critical, MainLoop},
	glib_signal::ObjectSignalExt,
	glib_signal_examples::TestObject,
	std::process::exit,
};

const DOMAIN: &'static str = "signal-async-example";

async fn main_async() {
	let obj = TestObject::new();
	let mut stream = obj.signal_stream(TestObject::SIGNAL_SOMETHING);

	let arg = "hello";
	obj.something(arg, false);

	let (signal_args,) = stream.next().await.unwrap();
	assert_eq!(signal_args, arg);
}

fn main() {
	let mainloop = MainLoop::new(None, false);
	ctrlc::set_handler({
		let mainloop = mainloop.clone();
		let mut counter = 0usize;
		move || {
			match counter {
				0 => mainloop.quit(),
				_ => exit(130),
			}
			counter = counter.saturating_add(1);
		}
	})
	.unwrap();
	let context = mainloop.context();
	context
		.with_thread_default(|| {
			let handle = context.spawn_local(main_async());

			let mainloop = mainloop.clone();
			context.spawn_local(async move {
				match handle.await {
					Ok(()) => mainloop.quit(),
					Err(e) => {
						g_critical!(DOMAIN, "main_async failed: {e}");
						exit(1);
					},
				}
			});
		})
		.unwrap();

	mainloop.run();
}
