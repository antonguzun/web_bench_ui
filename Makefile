local:
	trunk serve --proxy-backend=https://app.guzun.dev/web_benchmark

prod:
	trunk build --release