install:
	cp target/release/controller ~/.local/bin/pixie-controller -fv
	cp target/release/cli ~/.local/bin/pixie-cli -fv
	chmod +x ~/.local/bin/pixie-controller
	chmod +x ~/.local/bin/pixie-cli

service:
	mkdir -p ~/.local/share/systemd/user/
	cp systemd/pixie-controller.service ~/.local/share/systemd/user/
	systemctl --user daemon-reload


monitor:
	journalctl --user -u pixie-controller -f
