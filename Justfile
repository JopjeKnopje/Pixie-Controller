install:
	cp target/release/service ~/.local/bin/pixie-controller -fv
	cp target/release/cli ~/.local/bin/pixie-cli -fv
	chmod +x ~/.local/bin/pixie-controller
	chmod +x ~/.local/bin/pixie

service:
	mkdir -p ~/.local/share/systemd/user/
	cp systemd/psu-controller.service ~/.local/share/systemd/user/
	systemctl --user daemon-reload


monitor:
	journalctl --user -u pixie-controller -f
